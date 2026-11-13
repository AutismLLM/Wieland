use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use wieland_core::{
    Detector, Language, Level, compile_oxlintrc, load_pack, oxlint_rules, tsconfig_flags,
};

use crate::gate::{Report, Scope, Violation, path_matches, scope_files};
use crate::tools;

const DEFAULT_TRUE: [&str; 1] = ["strict"];
const TS_EXTENSIONS: [&str; 4] = [".ts", ".tsx", ".mts", ".cts"];
const TSC_CODE_RULES: [(&str, &str); 1] = [("TS1294", "syn-erasable-only")];

pub fn check(rules_root: &Path, project: &Path, scope: &Scope) -> Result<Report, String> {
    let corpus = load_pack(rules_root, Language::Typescript)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("no TypeScript rules under {}", rules_root.display()))?;
    let rules = oxlint_rules(&corpus);
    let flags = tsconfig_flags(&corpus);
    let titles: HashMap<String, (String, String)> = corpus
        .rules
        .iter()
        .map(|r| {
            (
                r.id.clone(),
                (
                    r.title.clone(),
                    wieland_core::frontmatter::section_body(&r.body, "Rule").unwrap_or_default(),
                ),
            )
        })
        .collect();
    let files = scope_files(scope).map(|list| {
        list.into_iter()
            .filter(|f| is_typescript(f))
            .collect::<Vec<_>>()
    });

    let mut report = Report {
        checked: rules.len() + flags.len(),
        ..Report::default()
    };
    let tsc = tools::project_tsc(project)?;
    for config in tsconfigs(project)? {
        let rel = config
            .strip_prefix(project)
            .unwrap_or(&config)
            .display()
            .to_string();
        let resolved = show_config(&tsc, project, &rel)?;
        for flag in &flags {
            if !flag_is_true(&resolved, &flag.name) {
                push(
                    &mut report,
                    flag.level,
                    Violation {
                        file: rel.clone(),
                        line: 1,
                        rule_id: flag.rule_id.clone(),
                        title: format!(
                            "{} (compilerOptions.{} is not true)",
                            flag.title, flag.name
                        ),
                        statement: flag.statement.clone(),
                    },
                );
            }
        }
        for error in type_errors(&tsc, project, &rel)? {
            if let Some(list) = &files {
                if !list.iter().any(|f| path_matches(f, &error.file)) {
                    continue;
                }
            }
            report.blocking.push(map_tsc_error(error, &titles));
        }
    }

    if files.as_ref().is_none_or(|list| !list.is_empty()) {
        for (level, violation) in lint(project, &rules, files.as_deref())? {
            push(&mut report, level, violation);
        }
    }
    for list in [&mut report.blocking, &mut report.advice] {
        list.sort_by(|a, b| a.file.cmp(&b.file).then(a.line.cmp(&b.line)));
    }
    Ok(report)
}

fn push(report: &mut Report, level: Level, violation: Violation) {
    match level {
        Level::Error => report.blocking.push(violation),
        Level::Warn => report.advice.push(violation),
    }
}

pub fn is_typescript(path: &str) -> bool {
    !path.ends_with(".d.ts") && TS_EXTENSIONS.iter().any(|ext| path.ends_with(ext))
}

fn tsconfigs(project: &Path) -> Result<Vec<PathBuf>, String> {
    let entries =
        std::fs::read_dir(project).map_err(|e| format!("reading {}: {e}", project.display()))?;
    let mut out: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("tsconfig") && n.ends_with(".json"))
        })
        .collect();
    out.sort();
    Ok(out)
}

fn show_config(tsc: &Path, project: &Path, config: &str) -> Result<serde_json::Value, String> {
    let output = Command::new(tsc)
        .args(["-p", config, "--showConfig"])
        .current_dir(project)
        .output()
        .map_err(|e| format!("running {}: {e}", tsc.display()))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).map_err(|_| {
        format!(
            "tsc --showConfig {config} did not print JSON: {}",
            stdout.trim()
        )
    })
}

fn flag_is_true(resolved: &serde_json::Value, flag: &str) -> bool {
    match resolved
        .get("compilerOptions")
        .and_then(|o| o.get(flag))
        .and_then(serde_json::Value::as_bool)
    {
        Some(value) => value,
        None => DEFAULT_TRUE.contains(&flag),
    }
}

pub struct TscError {
    pub file: String,
    pub line: u64,
    pub code: String,
    pub message: String,
}

fn type_errors(tsc: &Path, project: &Path, config: &str) -> Result<Vec<TscError>, String> {
    let output = Command::new(tsc)
        .args(["-p", config, "--noEmit", "--pretty", "false"])
        .current_dir(project)
        .output()
        .map_err(|e| format!("running {}: {e}", tsc.display()))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let errors: Vec<TscError> = stdout.lines().filter_map(parse_tsc_line).collect();
    if !output.status.success() && errors.is_empty() {
        return Err(format!("tsc -p {config} failed: {}", stdout.trim()));
    }
    Ok(errors)
}

pub fn parse_tsc_line(line: &str) -> Option<TscError> {
    let (location, rest) = line.split_once("): error ")?;
    let (file, position) = location.rsplit_once('(')?;
    let line_no = position.split(',').next()?.parse().ok()?;
    let (code, message) = rest.split_once(": ")?;
    Some(TscError {
        file: file.to_string(),
        line: line_no,
        code: code.to_string(),
        message: message.to_string(),
    })
}

fn map_tsc_error(error: TscError, titles: &HashMap<String, (String, String)>) -> Violation {
    let mapped = TSC_CODE_RULES
        .iter()
        .find(|(code, _)| *code == error.code)
        .and_then(|(_, rule)| titles.get(*rule).map(|t| (rule.to_string(), t)));
    match mapped {
        Some((rule_id, (title, statement))) => Violation {
            file: error.file,
            line: error.line,
            rule_id,
            title: format!("{title} ({}: {})", error.code, error.message),
            statement: statement.clone(),
        },
        None => Violation {
            file: error.file,
            line: error.line,
            rule_id: format!("tsc {}", error.code),
            title: error.message,
            statement: String::new(),
        },
    }
}

fn lint(
    project: &Path,
    rules: &[Detector],
    files: Option<&[String]>,
) -> Result<Vec<(Level, Violation)>, String> {
    if rules.is_empty() {
        return Ok(Vec::new());
    }
    let dir = tools::scratch_dir("gate-ts")?;
    let config = dir.join("oxlintrc.json");
    tools::write(&config, &compile_oxlintrc(rules))?;
    let mut cmd = tools::oxlint_command()?;
    cmd.arg("--type-aware")
        .arg("-c")
        .arg(&config)
        .args(["--format", "json"])
        .current_dir(project);
    match files {
        Some(list) => {
            cmd.args(list);
        }
        None => {
            cmd.arg(".");
        }
    }
    let output = cmd.output().map_err(|e| format!("running oxlint: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let report: serde_json::Value = serde_json::from_str(&stdout).map_err(|_| {
        format!(
            "oxlint did not print JSON (broken config or install): {}{}",
            stdout.trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        )
    })?;
    let by_name: HashMap<&str, &Detector> = rules.iter().map(|d| (d.name.as_str(), d)).collect();
    let mut out = Vec::new();
    for diagnostic in report
        .get("diagnostics")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
    {
        let Some(name) = diagnostic
            .get("code")
            .and_then(serde_json::Value::as_str)
            .and_then(oxlint_rule_name)
        else {
            continue;
        };
        let Some(detector) = by_name.get(name.as_str()) else {
            continue;
        };
        let file = diagnostic
            .get("filename")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("<unknown>")
            .to_string();
        let line = diagnostic
            .get("labels")
            .and_then(|l| l.get(0))
            .and_then(|l| l.get("span"))
            .and_then(|s| s.get("line"))
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0);
        out.push((
            detector.level,
            Violation {
                file,
                line,
                rule_id: detector.rule_id.clone(),
                title: detector.title.clone(),
                statement: detector.statement.clone(),
            },
        ));
    }
    Ok(out)
}

pub fn oxlint_rule_name(code: &str) -> Option<String> {
    let (plugin, rest) = code.split_once('(')?;
    let rule = rest.strip_suffix(')')?;
    Some(format!("{plugin}/{rule}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_oxlint_code_to_rule_name() {
        assert_eq!(
            oxlint_rule_name("typescript(no-floating-promises)"),
            Some("typescript/no-floating-promises".to_string())
        );
        assert_eq!(oxlint_rule_name("garbage"), None);
    }

    #[test]
    fn parses_tsc_error_line() {
        let error = parse_tsc_line(
            "src/a.ts(3,7): error TS2322: Type 'string' is not assignable to type 'number'.",
        )
        .expect("parses");
        assert_eq!(error.file, "src/a.ts");
        assert_eq!(error.line, 3);
        assert_eq!(error.code, "TS2322");
        assert!(error.message.starts_with("Type 'string'"));
    }

    #[test]
    fn typescript_extensions() {
        assert!(is_typescript("a/b.ts"));
        assert!(is_typescript("a/b.tsx"));
        assert!(is_typescript("a/b.mts"));
        assert!(is_typescript("a/b.cts"));
        assert!(!is_typescript("a/b.d.ts"));
        assert!(!is_typescript("a/b.js"));
    }

    #[test]
    fn strict_defaults_to_true() {
        let resolved = serde_json::json!({"compilerOptions": {}});
        assert!(flag_is_true(&resolved, "strict"));
        assert!(!flag_is_true(&resolved, "noUncheckedIndexedAccess"));
        let off = serde_json::json!({"compilerOptions": {"strict": false}});
        assert!(!flag_is_true(&off, "strict"));
    }
}
