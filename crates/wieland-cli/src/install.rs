use std::path::Path;
use std::process::ExitCode;

use wieland_core::{Corpus, Language};

use crate::hook;

pub fn run(rules_dir: &str) -> ExitCode {
    if let Err(err) = Corpus::load(rules_dir) {
        eprintln!("install: loading corpus at {rules_dir}: {err}");
        return ExitCode::FAILURE;
    }
    let Some(target) = hook::installed_rules_dir() else {
        eprintln!("install: HOME is not set");
        return ExitCode::FAILURE;
    };
    let source = Path::new(rules_dir);
    let packs = match copy_rules(source, &target) {
        Ok(count) => {
            println!("install: {count} Rust rules -> {}", target.display());
            copy_packs(source, &target)
        }
        Err(err) => Err(err),
    };
    match packs {
        Ok(()) => {
            println!();
            println!("hook it into Claude Code (~/.claude/settings.json):");
            println!(
                r#"  {{"hooks": {{"PostToolUse": [{{"matcher": "Edit|Write",
    "hooks": [{{"type": "command", "command": "wieland hook"}}]}}]}}}}"#
            );
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("install: {err}");
            ExitCode::FAILURE
        }
    }
}

fn copy_packs(source: &Path, target: &Path) -> Result<(), String> {
    for language in [Language::Typescript, Language::Go] {
        let Some(dir) = language.pack_dir() else {
            continue;
        };
        let from = source.join(dir);
        if !from.is_dir() {
            continue;
        }
        let to = target.join(dir);
        let count = copy_markdown(&from, &to)?;
        println!(
            "install: {count} {} rules -> {}",
            language.name(),
            to.display()
        );
    }
    Ok(())
}

fn copy_markdown(source: &Path, target: &Path) -> Result<usize, String> {
    std::fs::create_dir_all(target).map_err(|e| format!("creating {}: {e}", target.display()))?;
    let entries =
        std::fs::read_dir(source).map_err(|e| format!("reading {}: {e}", source.display()))?;
    let mut count = 0;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        std::fs::copy(&path, target.join(entry.file_name()))
            .map_err(|e| format!("copying {}: {e}", path.display()))?;
        count += 1;
    }
    Ok(count)
}

fn copy_rules(source: &Path, target: &Path) -> Result<usize, String> {
    if target.exists() {
        std::fs::remove_dir_all(target)
            .map_err(|e| format!("clearing {}: {e}", target.display()))?;
    }
    std::fs::create_dir_all(target).map_err(|e| format!("creating {}: {e}", target.display()))?;
    let entries =
        std::fs::read_dir(source).map_err(|e| format!("reading {}: {e}", source.display()))?;
    let mut count = 0;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let name = entry.file_name();
        std::fs::copy(&path, target.join(&name))
            .map_err(|e| format!("copying {}: {e}", path.display()))?;
        count += 1;
    }
    if count == 0 {
        return Err(format!("no rule files found in {}", source.display()));
    }
    Ok(count)
}
