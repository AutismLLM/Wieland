use std::collections::{BTreeSet, HashMap};
use std::path::Path;

use wieland_core::{Detector, Language, Level, golangci_linters, golangci_yml, load_pack};

use crate::gate::{Report, Scope, Violation, path_matches, scope_files};
use crate::tools;

const ALWAYS_ON: [&str; 1] = ["typecheck"];

pub struct Lint {
    pub issues: Vec<serde_json::Value>,
    pub enabled: BTreeSet<String>,
}

pub fn run_golangci(project: &Path, linters: &[Detector]) -> Result<Lint, String> {
    let bin = tools::go_bin("golangci-lint")?;
    let dir = tools::scratch_dir("gate-go")?;
    let config = dir.join("golangci.yml");
    tools::write(&config, &golangci_yml(linters))?;
    let output = std::process::Command::new(&bin)
        .arg("run")
        .arg("-c")
        .arg(&config)
        .args([
            "--output.json.path=stdout",
            "--max-issues-per-linter=0",
            "--max-same-issues=0",
            "--allow-parallel-runners",
            "./...",
        ])
        .current_dir(project)
        .output()
        .map_err(|e| format!("running {}: {e}", bin.display()))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_line = stdout
        .lines()
        .find(|l| l.trim_start().starts_with('{'))
        .unwrap_or("");
    let report: serde_json::Value = serde_json::from_str(json_line).map_err(|_| {
        format!(
            "golangci-lint did not print JSON (broken config or install): {}{}",
            stdout.trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        )
    })?;
    let issues = report
        .get("Issues")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let enabled = report
        .get("Report")
        .and_then(|r| r.get("Linters"))
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .filter(|l| l.get("Enabled").and_then(serde_json::Value::as_bool) == Some(true))
        .filter_map(|l| l.get("Name").and_then(serde_json::Value::as_str))
        .filter(|name| !ALWAYS_ON.contains(name))
        .map(str::to_string)
        .collect();
    Ok(Lint { issues, enabled })
}

pub fn check(rules_root: &Path, project: &Path, scope: &Scope) -> Result<Report, String> {
    let corpus = load_pack(rules_root, Language::Go)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("no Go rules under {}", rules_root.display()))?;
    let linters = golangci_linters(&corpus);
    let files = scope_files(scope).map(|list| {
        list.into_iter()
            .filter(|f| f.ends_with(".go"))
            .collect::<Vec<_>>()
    });
    let mut report = Report {
        checked: linters.len(),
        ..Report::default()
    };
    if files.as_ref().is_some_and(Vec::is_empty) {
        return Ok(report);
    }
    let lint = run_golangci(project, &linters)?;
    let by_name: HashMap<&str, &Detector> = linters.iter().map(|d| (d.name.as_str(), d)).collect();
    for issue in &lint.issues {
        let linter = issue
            .get("FromLinter")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let pos = issue.get("Pos");
        let file = pos
            .and_then(|p| p.get("Filename"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or("<unknown>")
            .to_string();
        let line = pos
            .and_then(|p| p.get("Line"))
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0);
        if let Some(list) = &files {
            if !list.iter().any(|f| path_matches(f, &file)) {
                continue;
            }
        }
        let text = issue
            .get("Text")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string();
        match by_name.get(linter) {
            Some(detector) => {
                let violation = Violation {
                    file,
                    line,
                    rule_id: detector.rule_id.clone(),
                    title: format!("{} ({linter}: {text})", detector.title),
                    statement: detector.statement.clone(),
                };
                match detector.level {
                    Level::Error => report.blocking.push(violation),
                    Level::Warn => report.advice.push(violation),
                }
            }
            None => report.blocking.push(Violation {
                file,
                line,
                rule_id: format!("go {linter}"),
                title: text,
                statement: String::new(),
            }),
        }
    }
    for list in [&mut report.blocking, &mut report.advice] {
        list.sort_by(|a, b| a.file.cmp(&b.file).then(a.line.cmp(&b.line)));
    }
    Ok(report)
}
