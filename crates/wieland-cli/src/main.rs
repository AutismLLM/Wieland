use std::path::Path;
use std::process::ExitCode;

use wieland_core::{
    Corpus, Language, Target, check_drift, corpus_sha, load_pack, render_packs, validate_corpus,
    write_artifacts,
};

mod check_detectors;
mod check_examples;
mod check_examples_go;
mod check_examples_ts;
mod gate;
mod gate_go;
mod gate_typescript;
mod hook;
mod install;
mod tools;

const RULES_DIR: &str = "rules";
const DIST_DIR: &str = "dist";

fn rules_dir() -> String {
    if let Ok(dir) = std::env::var("WIELAND_RULES") {
        return dir;
    }
    if Path::new(RULES_DIR).is_dir() {
        return RULES_DIR.to_string();
    }
    if let Some(installed) = hook::installed_rules_dir() {
        if installed.is_dir() {
            return installed.to_string_lossy().into_owned();
        }
    }
    RULES_DIR.to_string()
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("help");
    match cmd {
        "validate" => cmd_validate(args.get(2).map(String::as_str).unwrap_or(RULES_DIR)),
        "compile" => cmd_compile(&args[2..]),
        "gate" => gate::run(&rules_dir(), &args[2..]),
        "hook" => hook::run(&args[2..]),
        "install" => install::run(args.get(2).map(String::as_str).unwrap_or(RULES_DIR)),
        "check-examples" => cmd_check_examples(&args[2..]),
        "check-detectors" => {
            check_detectors::run(args.get(2).map(String::as_str).unwrap_or(RULES_DIR))
        }
        _ => {
            print_help();
            ExitCode::SUCCESS
        }
    }
}

fn cmd_check_examples(args: &[String]) -> ExitCode {
    let pack = args
        .iter()
        .position(|a| a == "--pack")
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
        .unwrap_or("all");
    let root = rules_dir();
    let mut ok = true;
    if pack == "all" || pack == "rust" {
        ok &= check_examples::run(&root) == ExitCode::SUCCESS;
    }
    if pack == "all" || pack == "typescript" {
        match check_examples_ts::run(Path::new(&root)) {
            Ok(clean) => ok &= clean,
            Err(err) => {
                eprintln!("check-examples: {err}");
                ok = false;
            }
        }
    }
    if pack == "all" || pack == "go" {
        match check_examples_go::run(Path::new(&root)) {
            Ok(clean) => ok &= clean,
            Err(err) => {
                eprintln!("check-examples: {err}");
                ok = false;
            }
        }
    }
    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn cmd_validate(root: &str) -> ExitCode {
    let corpus = match Corpus::load(root) {
        Ok(corpus) => corpus,
        Err(err) => {
            eprintln!("error: loading corpus at {root}: {err}");
            return ExitCode::FAILURE;
        }
    };
    let findings = validate_corpus(&corpus);
    if findings.is_empty() {
        println!("validate: {} rules OK", corpus.len());
        ExitCode::SUCCESS
    } else {
        for finding in &findings {
            eprintln!("{}: {}", finding.rule_id, finding.message);
        }
        eprintln!("validate: {} finding(s)", findings.len());
        ExitCode::FAILURE
    }
}

fn cmd_compile(args: &[String]) -> ExitCode {
    let mut all = false;
    let mut check = false;
    let mut target_name: Option<&str> = None;
    for arg in args {
        match arg.as_str() {
            "--all" => all = true,
            "--check" => check = true,
            other if other.starts_with("--") => {
                eprintln!("compile: unknown flag `{other}`");
                return ExitCode::FAILURE;
            }
            other => target_name = Some(other),
        }
    }

    let rules_root = Path::new(RULES_DIR);
    let artifacts = if all {
        match render_packs(rules_root) {
            Ok(artifacts) => artifacts,
            Err(err) => {
                eprintln!("compile: {err}");
                return ExitCode::FAILURE;
            }
        }
    } else if let Some(name) = target_name {
        let Some(target) = Target::from_name(name) else {
            eprintln!("compile: unknown target `{name}` (expected bundle|claude-code|deny-set)");
            return ExitCode::FAILURE;
        };
        let mut artifacts = Vec::new();
        for language in Language::ALL {
            let corpus = match load_pack(rules_root, language) {
                Ok(Some(corpus)) => corpus,
                Ok(None) => continue,
                Err(err) => {
                    eprintln!("compile: {err}");
                    return ExitCode::FAILURE;
                }
            };
            let sha = corpus_sha(&corpus);
            match target.render(&corpus, &sha) {
                Ok(rendered) => artifacts.extend(rendered),
                Err(err) => {
                    eprintln!("compile: {err}");
                    return ExitCode::FAILURE;
                }
            }
        }
        artifacts
    } else {
        eprintln!("compile: specify a target (bundle|claude-code|deny-set) or --all");
        return ExitCode::FAILURE;
    };

    let dist = Path::new(DIST_DIR);
    if check {
        match check_drift(dist, &artifacts, all) {
            Ok(()) => {
                println!("compile: {} artifact(s) up to date", artifacts.len());
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("{err}");
                ExitCode::FAILURE
            }
        }
    } else {
        match write_artifacts(dist, &artifacts) {
            Ok(()) => {
                println!(
                    "compile: wrote {} artifact(s) to {DIST_DIR}/",
                    artifacts.len()
                );
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("compile: {err}");
                ExitCode::FAILURE
            }
        }
    }
}

fn print_help() {
    println!("wieland <command>");
    println!("  validate [rules-dir]        schema-check the corpus (default: rules)");
    println!(
        "  compile <target>            write one target for every pack (bundle|claude-code|deny-set)"
    );
    println!(
        "  compile --all               write all targets for every pack (rules/, rules/typescript/, rules/go/)"
    );
    println!("  compile --all --check       drift gate: fail if dist/ is stale");
    println!("  compile deny-set            emit dist/deny-set/ (clippy.toml + [lints] table)");
    println!(
        "  gate [--all|--staged|--changed|--file <paths>]  run clippy deny-set, map lint->rule id"
    );
    println!(
        "  hook [--agent claude|codex|cursor|gemini|pi] [--path <file>]  gate the edited file(s); hook JSON on stdin"
    );
    println!("  install [rules-dir]         copy the corpus to ~/.wieland/rules for global use");
    println!(
        "  check-examples [--pack rust|typescript|go]  compile-check the Do blocks of every pack"
    );
    println!(
        "  check-detectors [rules-dir] fail if a corpus detector is unknown to the pinned tools"
    );
}
