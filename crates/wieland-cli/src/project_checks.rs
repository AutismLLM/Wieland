use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use wieland_core::{Project, find_project};

use crate::gate::Violation;

const OUTPUT_LINES: usize = 20;

pub fn for_paths(paths: &[PathBuf]) -> Result<Vec<Violation>, String> {
    let mut seen: BTreeSet<(PathBuf, String)> = BTreeSet::new();
    let mut out = Vec::new();
    for path in paths {
        let Some(project) = find_project(path)? else {
            continue;
        };
        let relative = path
            .strip_prefix(&project.root)
            .unwrap_or(path)
            .to_path_buf();
        for check in &project.file.checks {
            if !check.applies_to(&relative) {
                continue;
            }
            if !seen.insert((project.root.clone(), check.name.clone())) {
                continue;
            }
            if let Some(violation) = run_check(&project, &check.name, &check.run, path)? {
                out.push(violation);
            }
        }
    }
    Ok(out)
}

pub fn for_project(start: &Path) -> Result<Vec<Violation>, String> {
    let Some(project) = find_project(start)? else {
        return Ok(Vec::new());
    };
    let mut out = Vec::new();
    for check in &project.file.checks {
        if let Some(violation) = run_check(&project, &check.name, &check.run, &project.root)? {
            out.push(violation);
        }
    }
    Ok(out)
}

fn run_check(
    project: &Project,
    name: &str,
    run: &str,
    subject: &Path,
) -> Result<Option<Violation>, String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(run)
        .current_dir(&project.root)
        .output()
        .map_err(|e| format!("project check `{name}`: {e}"))?;
    if output.status.success() {
        return Ok(None);
    }
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    let tail = lines[lines.len().saturating_sub(OUTPUT_LINES)..].join("\n    ");
    Ok(Some(Violation {
        file: subject.to_string_lossy().into_owned(),
        line: 0,
        rule_id: format!("project:{name}"),
        title: format!("`{run}` failed"),
        statement: tail,
    }))
}
