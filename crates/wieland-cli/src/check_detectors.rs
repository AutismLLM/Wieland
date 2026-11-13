use std::path::Path;
use std::process::{Command, ExitCode};

use std::collections::BTreeSet;

use wieland_core::{Detector, Language, golangci_linters, load_pack, oxlint_rules, tsconfig_flags};

use crate::tools;

pub fn run(rules_root: &str) -> ExitCode {
    let root = Path::new(rules_root);
    let mut failures: Vec<String> = Vec::new();
    let mut checked = 0;
    match load_pack(root, Language::Typescript) {
        Ok(Some(corpus)) => {
            let rules = oxlint_rules(&corpus);
            let flags = tsconfig_flags(&corpus);
            match check_typescript(&rules, &flags) {
                Ok(()) => {
                    checked += rules.len() + flags.len();
                    println!(
                        "check-detectors: typescript ok ({} oxlint rule(s), {} tsconfig option(s))",
                        rules.len(),
                        flags.len()
                    );
                }
                Err(err) => failures.push(format!("typescript: {err}")),
            }
        }
        Ok(None) => {}
        Err(err) => failures.push(err.to_string()),
    }
    match load_pack(root, Language::Go) {
        Ok(Some(corpus)) => {
            let linters = golangci_linters(&corpus);
            match check_go(&linters) {
                Ok(()) => {
                    checked += linters.len();
                    println!(
                        "check-detectors: go ok ({} golangci-lint linter(s))",
                        linters.len()
                    );
                }
                Err(err) => failures.push(format!("go: {err}")),
            }
        }
        Ok(None) => {}
        Err(err) => failures.push(err.to_string()),
    }
    if failures.is_empty() {
        println!("check-detectors: {checked} detector(s) exist in the pinned tools");
        ExitCode::SUCCESS
    } else {
        for failure in &failures {
            eprintln!("check-detectors: {failure}");
        }
        ExitCode::FAILURE
    }
}

fn check_typescript(rules: &[Detector], flags: &[Detector]) -> Result<(), String> {
    let tsc = tools::typescript_bin("tsc")?;
    let dir = tools::scratch_dir("detectors-ts")?;
    tools::write(&dir.join("empty.ts"), "export {};\n")?;
    tools::write(&dir.join("tsconfig.json"), &probe_tsconfig(flags))?;
    tools::write(
        &dir.join("oxlintrc.json"),
        &wieland_core::compile_oxlintrc(rules),
    )?;

    let output = Command::new(&tsc)
        .args(["-p", "tsconfig.json", "--noEmit"])
        .current_dir(&dir)
        .output()
        .map_err(|e| format!("running {}: {e}", tsc.display()))?;
    if !output.status.success() {
        return Err(format!(
            "tsc rejected the required compiler options:\n{}",
            String::from_utf8_lossy(&output.stdout).trim()
        ));
    }

    let output = tools::oxlint_command()?
        .args([
            "--type-aware",
            "-c",
            "oxlintrc.json",
            "--format",
            "json",
            "empty.ts",
        ])
        .current_dir(&dir)
        .output()
        .map_err(|e| format!("running oxlint: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let Ok(report) = serde_json::from_str::<serde_json::Value>(&stdout) else {
        return Err(format!(
            "oxlint rejected the compiled config:\n{}{}",
            stdout.trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    };
    let ran = report
        .get("number_of_rules")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    if ran != rules.len() as u64 {
        return Err(format!(
            "oxlint ran {ran} rule(s) but the corpus names {}; default rules are leaking in or a rule is ignored",
            rules.len()
        ));
    }
    Ok(())
}

fn probe_tsconfig(flags: &[Detector]) -> String {
    let mut options: Vec<String> = vec![
        "\"module\": \"esnext\"".to_string(),
        "\"moduleResolution\": \"bundler\"".to_string(),
    ];
    for flag in flags {
        options.push(format!("\"{}\": true", flag.name));
    }
    format!(
        "{{\n  \"compilerOptions\": {{\n    {}\n  }},\n  \"files\": [\"empty.ts\"]\n}}\n",
        options.join(",\n    ")
    )
}

fn check_go(linters: &[Detector]) -> Result<(), String> {
    let dir = tools::scratch_dir("detectors-go")?;
    tools::write(&dir.join("go.mod"), "module probe\n\ngo 1.26\n")?;
    tools::write(&dir.join("probe.go"), "package probe\n")?;
    let lint = crate::gate_go::run_golangci(&dir, linters)?;
    let wanted: BTreeSet<String> = linters.iter().map(|d| d.name.clone()).collect();
    if lint.enabled != wanted {
        let extra: Vec<&String> = lint.enabled.difference(&wanted).collect();
        let missing: Vec<&String> = wanted.difference(&lint.enabled).collect();
        return Err(format!(
            "golangci-lint enabled {:?} beyond the corpus and lacks {:?}",
            extra, missing
        ));
    }
    Ok(())
}
