use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use wieland_core::Language;

use crate::gate::{self, Scope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Agent {
    Claude,
    Codex,
    Cursor,
    Gemini,
    Plain,
}

impl Agent {
    pub fn from_name(name: &str) -> Option<Agent> {
        match name {
            "claude" | "claude-code" => Some(Agent::Claude),
            "codex" => Some(Agent::Codex),
            "cursor" => Some(Agent::Cursor),
            "gemini" => Some(Agent::Gemini),
            "pi" | "plain" => Some(Agent::Plain),
            _ => None,
        }
    }
}

pub fn run(args: &[String]) -> ExitCode {
    let agent = args
        .iter()
        .position(|a| a == "--agent")
        .and_then(|i| args.get(i + 1))
        .and_then(|n| Agent::from_name(n))
        .unwrap_or(Agent::Claude);
    match gate_and_respond(args) {
        Some(code) => code,
        None => {
            if matches!(agent, Agent::Cursor | Agent::Gemini) {
                println!("{{}}");
            }
            ExitCode::SUCCESS
        }
    }
}

fn gate_and_respond(args: &[String]) -> Option<ExitCode> {
    let mut agent = Agent::Claude;
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--agent" => {
                let Some(found) = iter.next().and_then(|n| Agent::from_name(n)) else {
                    eprintln!("wieland hook: --agent expects claude|codex|cursor|gemini|pi");
                    return None;
                };
                agent = found;
            }
            "--path" => {
                if let Some(path) = iter.next() {
                    paths.push(absolute(Path::new(path)));
                }
            }
            other => {
                eprintln!("wieland hook: unknown argument `{other}`; skipping");
                return None;
            }
        }
    }
    if paths.is_empty() {
        let mut input = String::new();
        if std::io::stdin().read_to_string(&mut input).is_err() {
            return None;
        }
        paths = edited_paths(&input);
    }
    let sources: Vec<(PathBuf, Language)> = paths
        .into_iter()
        .filter_map(|p| {
            let language = language_of(&p.to_string_lossy())?;
            Some((p, language))
        })
        .collect();
    if sources.is_empty() {
        return None;
    }
    let Some(rules_dir) = resolve_rules_dir() else {
        eprintln!(
            "wieland hook: no corpus found (set WIELAND_RULES or run `wieland install`); skipping"
        );
        return None;
    };
    match check_sources(&rules_dir.to_string_lossy(), &sources) {
        Ok((blocking, advice)) => respond(agent, &blocking, &advice),
        Err(err) => {
            eprintln!("wieland hook: {err}; skipping");
            None
        }
    }
}

fn absolute(path: &Path) -> PathBuf {
    if path.is_absolute() {
        return path.to_path_buf();
    }
    std::env::current_dir()
        .map(|cwd| cwd.join(path))
        .unwrap_or_else(|_| path.to_path_buf())
}

pub fn check_sources(
    rules_dir: &str,
    sources: &[(PathBuf, Language)],
) -> Result<(Vec<gate::Violation>, Vec<gate::Violation>), String> {
    let mut groups: Vec<(PathBuf, Vec<String>)> = Vec::new();
    for (file, language) in sources {
        let Some(root) = enclosing_root(file, *language) else {
            continue;
        };
        let text = file.to_string_lossy().into_owned();
        match groups.iter_mut().find(|(r, _)| *r == root) {
            Some((_, files)) => files.push(text),
            None => groups.push((root, vec![text])),
        }
    }
    let mut blocking = Vec::new();
    let mut advice = Vec::new();
    for (root, files) in groups {
        let report = gate::check_project(rules_dir, &root, &Scope::Files(files.clone()))?;
        for violation in report.blocking {
            let mut violation = violation;
            violation.file = display_path(&root, &violation.file);
            if files.iter().any(|f| gate::path_matches(f, &violation.file)) {
                blocking.push(violation);
            } else {
                advice.push(violation);
            }
        }
        for violation in report.advice {
            let mut violation = violation;
            violation.file = display_path(&root, &violation.file);
            advice.push(violation);
        }
    }
    Ok((blocking, advice))
}

fn display_path(root: &Path, file: &str) -> String {
    let path = Path::new(file);
    if path.is_absolute() {
        file.to_string()
    } else {
        root.join(path).to_string_lossy().into_owned()
    }
}

pub fn feedback_text(blocking: &[gate::Violation], advice: &[gate::Violation]) -> String {
    let mut out = String::new();
    if !blocking.is_empty() {
        out.push_str("Wieland blocked this edit. Fix these before continuing:\n");
        out.push_str(&gate::format_violations(blocking));
    }
    if !advice.is_empty() {
        out.push_str(&gate::format_advice(advice));
    }
    out
}

fn respond(
    agent: Agent,
    blocking: &[gate::Violation],
    advice: &[gate::Violation],
) -> Option<ExitCode> {
    if blocking.is_empty() && advice.is_empty() {
        return None;
    }
    let text = feedback_text(blocking, advice);
    let code = if blocking.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(2)
    };
    Some(match agent {
        Agent::Claude | Agent::Codex => {
            eprint!("{text}");
            code
        }
        Agent::Cursor => {
            println!("{}", serde_json::json!({ "additional_context": text }));
            ExitCode::SUCCESS
        }
        Agent::Gemini => {
            println!(
                "{}",
                serde_json::json!({
                    "hookSpecificOutput": {
                        "hookEventName": "AfterTool",
                        "additionalContext": text
                    }
                })
            );
            ExitCode::SUCCESS
        }
        Agent::Plain => {
            print!("{text}");
            code
        }
    })
}

pub fn language_of(path: &str) -> Option<Language> {
    if path.ends_with(".rs") {
        return Some(Language::Rust);
    }
    if crate::gate_typescript::is_typescript(path) {
        return Some(Language::Typescript);
    }
    if path.ends_with(".go") {
        return Some(Language::Go);
    }
    None
}

pub fn edited_paths(hook_json: &str) -> Vec<PathBuf> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(hook_json) else {
        return Vec::new();
    };
    let cwd = value
        .get("cwd")
        .and_then(|c| c.as_str())
        .map(PathBuf::from)
        .or_else(|| {
            value
                .get("workspace_roots")
                .and_then(|r| r.get(0))
                .and_then(|r| r.as_str())
                .map(PathBuf::from)
        });
    let input = match value.get("tool_input") {
        Some(serde_json::Value::String(text)) => serde_json::from_str(text).unwrap_or_default(),
        Some(other) => other.clone(),
        None => serde_json::Value::Null,
    };
    let mut raw: Vec<String> = Vec::new();
    for key in ["file_path", "path", "absolute_path"] {
        if let Some(found) = input.get(key).and_then(|v| v.as_str()) {
            raw.push(found.to_string());
            break;
        }
    }
    if raw.is_empty() {
        if let Some(found) = value.get("file_path").and_then(|v| v.as_str()) {
            raw.push(found.to_string());
        }
    }
    if raw.is_empty() {
        let patch = input
            .get("command")
            .or_else(|| input.get("patch"))
            .or_else(|| input.get("input"))
            .and_then(|v| v.as_str())
            .or_else(|| input.as_str());
        if let Some(patch) = patch {
            raw.extend(patch_paths(patch));
        }
    }
    raw.into_iter()
        .map(|file| {
            let path = PathBuf::from(&file);
            match (&cwd, path.is_absolute()) {
                (Some(dir), false) => dir.join(path),
                _ => path,
            }
        })
        .collect()
}

pub fn patch_paths(patch: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in patch.lines() {
        for prefix in ["*** Add File: ", "*** Update File: ", "*** Move to: "] {
            if let Some(path) = line.strip_prefix(prefix) {
                let path = path.trim().to_string();
                if !out.contains(&path) {
                    out.push(path);
                }
            }
        }
    }
    out
}

pub fn manifest_for(language: Language) -> &'static str {
    match language {
        Language::Rust => "Cargo.toml",
        Language::Typescript => "tsconfig.json",
        Language::Go => "go.mod",
    }
}

pub fn enclosing_root(file: &Path, language: Language) -> Option<PathBuf> {
    let manifest = manifest_for(language);
    let mut dir = file.parent()?;
    loop {
        if dir.join(manifest).is_file() {
            return Some(dir.to_path_buf());
        }
        dir = dir.parent()?;
    }
}

pub fn resolve_rules_dir() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("WIELAND_RULES") {
        let path = PathBuf::from(dir);
        if path.is_dir() {
            return Some(path);
        }
    }
    let installed = installed_rules_dir()?;
    if installed.is_dir() {
        Some(installed)
    } else {
        None
    }
}

pub fn installed_rules_dir() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    Some(Path::new(&home).join(".wieland").join("rules"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edited_source_file(hook_json: &str) -> Option<(PathBuf, Language)> {
        edited_paths(hook_json).into_iter().find_map(|p| {
            let language = language_of(&p.to_string_lossy())?;
            Some((p, language))
        })
    }

    #[test]
    fn extracts_absolute_rust_path() {
        let json = r#"{"tool_name":"Edit","tool_input":{"file_path":"/tmp/a/src/lib.rs"}}"#;
        assert_eq!(
            edited_source_file(json),
            Some((PathBuf::from("/tmp/a/src/lib.rs"), Language::Rust))
        );
    }

    #[test]
    fn joins_relative_path_with_cwd() {
        let json = r#"{"cwd":"/tmp/a","tool_input":{"file_path":"src/lib.rs"}}"#;
        assert_eq!(
            edited_source_file(json),
            Some((PathBuf::from("/tmp/a/src/lib.rs"), Language::Rust))
        );
    }

    #[test]
    fn ignores_non_rust_files() {
        let json = r#"{"tool_input":{"file_path":"/tmp/a/README.md"}}"#;
        assert_eq!(edited_source_file(json), None);
    }

    #[test]
    fn ignores_malformed_input() {
        assert_eq!(edited_source_file("not json"), None);
        assert_eq!(edited_source_file("{}"), None);
    }

    #[test]
    fn finds_crate_root_above_file() {
        let dir = std::env::temp_dir().join(format!("wieland-hook-test-{}", std::process::id()));
        let src = dir.join("src");
        std::fs::create_dir_all(&src).ok();
        std::fs::write(dir.join("Cargo.toml"), "[package]").ok();
        let file = src.join("main.rs");
        assert_eq!(enclosing_root(&file, Language::Rust), Some(dir.clone()));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn extracts_typescript_file_with_language() {
        let json = r#"{"tool_input":{"file_path":"/tmp/a/src/app.tsx"}}"#;
        assert_eq!(
            edited_source_file(json),
            Some((PathBuf::from("/tmp/a/src/app.tsx"), Language::Typescript))
        );
    }

    #[test]
    fn skips_declaration_files() {
        let json = r#"{"tool_input":{"file_path":"/tmp/a/src/env.d.ts"}}"#;
        assert_eq!(edited_source_file(json), None);
    }

    #[test]
    fn accepts_mts_and_cts() {
        for name in ["a.mts", "a.cts", "a.ts"] {
            let json = format!(r#"{{"tool_input":{{"file_path":"/tmp/{name}"}}}}"#);
            assert!(edited_source_file(&json).is_some(), "{name}");
        }
    }

    #[test]
    fn finds_tsconfig_root_above_file() {
        let dir = std::env::temp_dir().join(format!("wieland-hook-ts-{}", std::process::id()));
        let src = dir.join("src").join("deep");
        std::fs::create_dir_all(&src).ok();
        std::fs::write(dir.join("tsconfig.json"), "{}").ok();
        let file = src.join("a.ts");
        assert_eq!(
            enclosing_root(&file, Language::Typescript),
            Some(dir.clone())
        );
        assert_eq!(enclosing_root(&file, Language::Rust), None);
    }

    #[test]
    fn finds_go_mod_root_for_go_file() {
        let json = r#"{"tool_input":{"file_path":"/tmp/m/pkg/a.go"}}"#;
        assert_eq!(
            edited_source_file(json),
            Some((PathBuf::from("/tmp/m/pkg/a.go"), Language::Go))
        );
        let dir = std::env::temp_dir().join(format!("wieland-hook-go-{}", std::process::id()));
        let pkg = dir.join("pkg");
        std::fs::create_dir_all(&pkg).ok();
        std::fs::write(dir.join("go.mod"), "module m\n").ok();
        assert_eq!(
            enclosing_root(&pkg.join("a.go"), Language::Go),
            Some(dir.clone())
        );
    }

    #[test]
    fn reads_cursor_string_tool_input() {
        let json = r#"{"tool_name":"Write","tool_input":"{\"file_path\":\"/w/src/a.ts\"}"}"#;
        assert_eq!(edited_paths(json), vec![PathBuf::from("/w/src/a.ts")]);
    }

    #[test]
    fn reads_cursor_after_file_edit_top_level_path() {
        let json = r#"{"hook_event_name":"afterFileEdit","file_path":"/w/main.go","edits":[]}"#;
        assert_eq!(edited_paths(json), vec![PathBuf::from("/w/main.go")]);
    }

    #[test]
    fn reads_pi_style_path_key() {
        let json = r#"{"tool_name":"edit","tool_input":{"path":"src/lib.rs"},"cwd":"/w"}"#;
        assert_eq!(edited_paths(json), vec![PathBuf::from("/w/src/lib.rs")]);
    }

    #[test]
    fn reads_codex_apply_patch_paths() {
        let patch = "*** Begin Patch\n*** Update File: src/a.ts\n@@\n-x\n+y\n*** Add File: pkg/b.go\n+package b\n*** End Patch";
        let json = serde_json::json!({
            "tool_name": "apply_patch",
            "cwd": "/w",
            "tool_input": { "command": patch }
        })
        .to_string();
        assert_eq!(
            edited_paths(&json),
            vec![PathBuf::from("/w/src/a.ts"), PathBuf::from("/w/pkg/b.go")]
        );
    }

    #[test]
    fn agent_names() {
        assert_eq!(Agent::from_name("cursor"), Some(Agent::Cursor));
        assert_eq!(Agent::from_name("pi"), Some(Agent::Plain));
        assert_eq!(Agent::from_name("vim"), None);
    }

    #[test]
    fn no_crate_root_returns_none() {
        assert_eq!(
            enclosing_root(Path::new("/nonexistent/x/y.rs"), Language::Rust),
            None
        );
    }
}
