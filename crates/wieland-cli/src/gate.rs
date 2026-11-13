use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use wieland_core::{Corpus, DenyLint, Language, deny_set};

use crate::gate_typescript;

pub enum Scope {
    All,
    Staged,
    Changed,
    Files(Vec<String>),
}

pub struct Violation {
    pub file: String,
    pub line: u64,
    pub rule_id: String,
    pub title: String,
    pub statement: String,
}

#[derive(Default)]
pub struct Report {
    pub blocking: Vec<Violation>,
    pub advice: Vec<Violation>,
    pub checked: usize,
    pub language: Option<Language>,
}

pub fn project_language(start: &Path) -> Option<(Language, PathBuf)> {
    let mut dir = Some(start);
    while let Some(current) = dir {
        if current.join("Cargo.toml").is_file() {
            return Some((Language::Rust, current.to_path_buf()));
        }
        if current.join("tsconfig.json").is_file() {
            return Some((Language::Typescript, current.to_path_buf()));
        }
        if current.join("go.mod").is_file() {
            return Some((Language::Go, current.to_path_buf()));
        }
        dir = current.parent();
    }
    None
}

pub fn run(rules_dir: &str, args: &[String]) -> ExitCode {
    let mut scope = Scope::All;
    let mut files: Vec<String> = Vec::new();
    let mut collecting_files = false;
    for arg in args {
        if collecting_files && !arg.starts_with("--") {
            files.push(arg.clone());
            continue;
        }
        collecting_files = false;
        match arg.as_str() {
            "--all" => scope = Scope::All,
            "--staged" => scope = Scope::Staged,
            "--changed" => scope = Scope::Changed,
            "--file" => collecting_files = true,
            other => {
                eprintln!(
                    "gate: unknown flag `{other}` (expected --all|--staged|--changed|--file <paths>)"
                );
                return ExitCode::FAILURE;
            }
        }
    }
    if !files.is_empty() {
        scope = Scope::Files(files);
    }

    if matches!(scope, Scope::Staged | Scope::Changed) {
        return run_git_scope(rules_dir, &scope);
    }
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            eprintln!("gate: current directory: {err}");
            return ExitCode::FAILURE;
        }
    };
    match check_project(rules_dir, &cwd, &scope) {
        Ok(mut report) => {
            match crate::project_checks::for_project(&cwd) {
                Ok(found) => report.blocking.extend(found),
                Err(err) => {
                    eprintln!("gate: {err}");
                    return ExitCode::FAILURE;
                }
            }
            print_report(&report)
        }
        Err(err) => {
            eprintln!("gate: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run_git_scope(rules_dir: &str, scope: &Scope) -> ExitCode {
    let top = git_files(&["rev-parse", "--show-toplevel"])
        .into_iter()
        .next()
        .map(PathBuf::from);
    let Some(top) = top else {
        eprintln!("gate: not inside a git repository");
        return ExitCode::FAILURE;
    };
    let sources: Vec<(PathBuf, Language)> = scope_files(scope)
        .unwrap_or_default()
        .into_iter()
        .map(|f| top.join(f))
        .filter(|p| p.is_file())
        .filter_map(|p| {
            let language = crate::hook::language_of(&p.to_string_lossy())?;
            Some((p, language))
        })
        .collect();
    if sources.is_empty() {
        println!("gate: no Rust, TypeScript or Go files in scope");
        return ExitCode::SUCCESS;
    }
    let staged: Vec<PathBuf> = sources.iter().map(|(p, _)| p.clone()).collect();
    let project_blocks = match crate::project_checks::for_paths(&staged) {
        Ok(found) => found,
        Err(err) => {
            eprintln!("gate: {err}");
            return ExitCode::FAILURE;
        }
    };
    match crate::hook::check_sources(rules_dir, &sources) {
        Ok((mut blocking, advice)) => {
            blocking.extend(project_blocks);
            if !advice.is_empty() {
                eprint!("{}", format_advice(&advice));
            }
            if blocking.is_empty() {
                println!("gate: clean ({} file(s) checked)", sources.len());
                ExitCode::SUCCESS
            } else {
                eprint!("{}", format_violations(&blocking));
                ExitCode::FAILURE
            }
        }
        Err(err) => {
            eprintln!("gate: {err}");
            ExitCode::FAILURE
        }
    }
}

pub fn check_project(rules_dir: &str, start: &Path, scope: &Scope) -> Result<Report, String> {
    let Some((language, root)) = project_language(start) else {
        return Err(format!(
            "no Cargo.toml, tsconfig.json or go.mod at or above {}",
            start.display()
        ));
    };
    let mut report = match language {
        Language::Rust => {
            let (blocking, checked) = check(rules_dir, &root, scope)?;
            Report {
                blocking,
                advice: Vec::new(),
                checked,
                language: None,
            }
        }
        Language::Typescript => gate_typescript::check(Path::new(rules_dir), &root, scope)?,
        Language::Go => crate::gate_go::check(Path::new(rules_dir), &root, scope)?,
    };
    report.language = Some(language);
    Ok(report)
}

pub fn print_report(report: &Report) -> ExitCode {
    if !report.advice.is_empty() {
        eprint!("{}", format_advice(&report.advice));
    }
    if report.blocking.is_empty() {
        match report.language {
            Some(Language::Rust) | None => {
                println!("gate: clean ({} deny-set lint(s) checked)", report.checked)
            }
            Some(language) => println!(
                "gate: clean ({} {} detector(s) checked, {} advice)",
                report.checked,
                language.name(),
                report.advice.len()
            ),
        }
        ExitCode::SUCCESS
    } else {
        eprint!("{}", format_violations(&report.blocking));
        ExitCode::FAILURE
    }
}

pub fn format_advice(advice: &[Violation]) -> String {
    let mut out = String::new();
    for v in advice {
        out.push_str(&format!(
            "{}:{}: advice [{}] {}\n",
            v.file, v.line, v.rule_id, v.title
        ));
    }
    out.push_str(&format!("gate: {} advice (does not block)\n", advice.len()));
    out
}

pub fn format_violations(violations: &[Violation]) -> String {
    let mut out = String::new();
    for v in violations {
        out.push_str(&format!(
            "{}:{}: [{}] {}\n",
            v.file, v.line, v.rule_id, v.title
        ));
        if !v.statement.is_empty() {
            out.push_str(&format!("    fix: {}\n", v.statement));
        }
    }
    out.push_str(&format!("gate: {} violation(s)\n", violations.len()));
    out
}

pub fn check(
    rules_dir: &str,
    project: &Path,
    scope: &Scope,
) -> Result<(Vec<Violation>, usize), String> {
    let corpus =
        Corpus::load(rules_dir).map_err(|err| format!("loading corpus at {rules_dir}: {err}"))?;
    let lints = deny_set(&corpus);
    if lints.is_empty() {
        return Ok((Vec::new(), 0));
    }
    let by_code: HashMap<String, &DenyLint> = lints
        .iter()
        .map(|d| (format!("clippy::{}", d.lint), d))
        .collect();

    let mut cmd = Command::new("cargo");
    cmd.current_dir(project);
    cmd.args(["clippy", "--message-format=json", "--quiet"]);
    cmd.arg("--");
    for entry in &lints {
        cmd.arg("-D").arg(format!("clippy::{}", entry.lint));
    }
    let output = cmd
        .output()
        .map_err(|err| format!("failed to run cargo clippy: {err}"))?;

    let scope_files = scope_files(scope);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut violations: Vec<Violation> = Vec::new();
    for line in stdout.lines() {
        let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if value.get("reason").and_then(|r| r.as_str()) != Some("compiler-message") {
            continue;
        }
        let message = &value["message"];
        let Some(code) = message
            .get("code")
            .and_then(|c| c.get("code"))
            .and_then(|c| c.as_str())
        else {
            continue;
        };
        let Some(deny) = by_code.get(code) else {
            continue;
        };
        let (file, line_no) = primary_span(&message["spans"]);
        if let Some(files) = &scope_files {
            if !files.iter().any(|f| path_matches(f, &file)) {
                continue;
            }
        }
        violations.push(Violation {
            file,
            line: line_no,
            rule_id: deny.rule_id.clone(),
            title: deny.title.clone(),
            statement: deny.statement.clone(),
        });
    }
    Ok((violations, lints.len()))
}

fn primary_span(spans: &serde_json::Value) -> (String, u64) {
    let Some(array) = spans.as_array() else {
        return ("<unknown>".to_string(), 0);
    };
    let chosen = array
        .iter()
        .find(|s| s.get("is_primary").and_then(|p| p.as_bool()) == Some(true))
        .or_else(|| array.first());
    match chosen {
        Some(span) => (
            span.get("file_name")
                .and_then(|f| f.as_str())
                .unwrap_or("<unknown>")
                .to_string(),
            span.get("line_start").and_then(|l| l.as_u64()).unwrap_or(0),
        ),
        None => ("<unknown>".to_string(), 0),
    }
}

pub fn scope_files(scope: &Scope) -> Option<Vec<String>> {
    match scope {
        Scope::All => None,
        Scope::Staged => Some(git_files(&["diff", "--cached", "--name-only"])),
        Scope::Changed => Some(git_files(&["diff", "--name-only", "HEAD"])),
        Scope::Files(files) => Some(files.clone()),
    }
}

fn git_files(args: &[&str]) -> Vec<String> {
    let Ok(output) = Command::new("git").args(args).output() else {
        return Vec::new();
    };
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

pub fn path_matches(changed: &str, violation: &str) -> bool {
    changed.ends_with(violation) || violation.ends_with(changed)
}
