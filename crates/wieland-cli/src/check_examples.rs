use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use wieland_core::Corpus;

const PROJECT_DIR: &str = "target/check-examples";

const HEADER: &str = "#![allow(unused, dead_code, unreachable_code, unused_imports, unused_variables, unused_mut, unused_assignments, unused_macros, non_local_definitions)]\n";

const MANIFEST: &str = r#"[package]
name = "wieland-check-examples"
version = "0.0.0"
edition = "2024"
publish = false

[workspace]

[lib]
path = "src/lib.rs"

[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-util = { version = "0.7", features = ["full"] }
futures = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
thiserror = "2"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
log = "0.4"
smallvec = "1"
arrayvec = "0.7"
thin-vec = "0.2"
bytes = "1"
compact_str = "0.9"
smartstring = "1"
once_cell = "1"
parking_lot = "0.12"
rayon = "1"
rustc-hash = "2"
ahash = "0.8"
secrecy = "0.10"
memchr = "2"
static_assertions = "1"
bumpalo = "3"
indexmap = "2"
tempfile = "3"
mockall = "0.13"
proptest = "1"
insta = "1"
criterion = "0.5"
loom = "0.7"
syn = { version = "2", features = ["full", "extra-traits", "derive"] }
quote = "1"
proc-macro2 = "1"
"#;

const PLACEHOLDER_CRATES: &[&str] = &[
    "my_crate",
    "mycrate",
    "mylib",
    "my_app",
    "my_project",
    "my_lib",
    "mycrate_derive",
];
const PLACEHOLDER_USES: &[&str] = &["model", "transport", "service", "internal", "app", "domain"];

const RES_CODES: &[&str] = &[
    "E0432", "E0433", "E0412", "E0425", "E0405", "E0531", "E0422", "E0423", "E0573", "E0463",
    "E0583", "E0561", "E0658",
];
const RES_PREFIXES: &[&str] = &[
    "cannot find",
    "unresolved import",
    "failed to resolve",
    "use of undeclared",
    "cannot determine",
    "can't find crate",
    "maybe a missing crate",
    "unresolved module",
];
const LOW_CODES: &[&str] = &["E0282", "E0283"];

struct Block {
    section: String,
    line: usize,
    code: String,
}

struct Example {
    name: String,
    rule_id: String,
    section: String,
    line: usize,
}

pub fn run(rules_dir: &str) -> ExitCode {
    let corpus = match Corpus::load(rules_dir) {
        Ok(corpus) => corpus,
        Err(err) => {
            eprintln!("error: loading corpus at {rules_dir}: {err}");
            return ExitCode::FAILURE;
        }
    };

    let project = PathBuf::from(PROJECT_DIR);
    let examples_dir = project.join("examples");
    if let Err(err) = prepare_project(&project, &examples_dir) {
        eprintln!("check-examples: {err}");
        return ExitCode::FAILURE;
    }

    let mut examples = Vec::new();
    let mut idx = 0usize;
    for rule in &corpus.rules {
        for block in rust_blocks(&rule.body) {
            if is_candidate(&block.code, &block.section) {
                let name = format!("{}__{}", rule.id.replace('-', "_"), idx);
                let content = wrap(&block.code);
                let path = examples_dir.join(format!("{name}.rs"));
                if let Err(err) = fs::write(&path, content) {
                    eprintln!("check-examples: writing {}: {err}", path.display());
                    return ExitCode::FAILURE;
                }
                examples.push(Example {
                    name,
                    rule_id: rule.id.clone(),
                    section: block.section.clone(),
                    line: block.line,
                });
            }
            idx += 1;
        }
    }

    println!(
        "check-examples: generated {} example(s) from {} rust block(s); running cargo check (first run compiles ~40 crates)...",
        examples.len(),
        idx
    );

    let manifest = project.join("Cargo.toml");
    let output = match Command::new("cargo")
        .args([
            "check",
            "--examples",
            "--message-format=json",
            "--quiet",
            "--manifest-path",
        ])
        .arg(&manifest)
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            eprintln!("check-examples: failed to run cargo check: {err}");
            return ExitCode::FAILURE;
        }
    };

    let diagnostics = parse_diagnostics(&String::from_utf8_lossy(&output.stdout));
    report(&examples, &diagnostics)
}

fn prepare_project(project: &Path, examples_dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(project.join("src"))?;
    fs::write(project.join("Cargo.toml"), MANIFEST)?;
    fs::write(project.join("src/lib.rs"), "")?;
    if examples_dir.exists() {
        for entry in fs::read_dir(examples_dir)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                fs::remove_file(path)?;
            }
        }
    } else {
        fs::create_dir_all(examples_dir)?;
    }
    Ok(())
}

fn rust_blocks(body: &str) -> Vec<Block> {
    let lines: Vec<&str> = body.lines().collect();
    let mut blocks = Vec::new();
    let mut section = String::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("##") {
            section = rest.trim_start_matches('#').trim().to_string();
        }
        if line.trim() == "```rust" {
            let start = i + 1;
            let mut j = start;
            while j < lines.len() && lines[j].trim() != "```" {
                j += 1;
            }
            blocks.push(Block {
                section: section.clone(),
                line: start + 1,
                code: lines[start..j].join("\n"),
            });
            i = j;
        }
        i += 1;
    }
    blocks
}

fn is_candidate(code: &str, section: &str) -> bool {
    let s = section.trim().to_lowercase();
    if s == "bad" || s == "don't" || s == "don’t" {
        return false;
    }
    if code.contains("#![feature") || code.contains("proc_macro") {
        return false;
    }
    for name in PLACEHOLDER_CRATES {
        if code.contains(name) {
            return false;
        }
    }
    for line in code.lines() {
        let t = line.trim();
        if t == "..." {
            return false;
        }
        if let Some(rest) = t.strip_prefix("use ") {
            let token: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if PLACEHOLDER_USES.contains(&token.as_str()) {
                return false;
            }
        }
    }
    true
}

fn wrap(block: &str) -> String {
    if has_fn_main(block) {
        format!("{HEADER}{block}\n")
    } else if block.contains("#![") || has_mod(block) {
        format!("{HEADER}{block}\nfn main() {{}}\n")
    } else {
        format!(
            "{HEADER}async fn __ex() -> Result<(), Box<dyn std::error::Error>> {{\n{block}\n;\nOk(())\n}}\nfn main() {{}}\n"
        )
    }
}

fn has_fn_main(block: &str) -> bool {
    let mut rest = block;
    while let Some(pos) = rest.find("fn main") {
        let after = rest[pos + "fn main".len()..].trim_start();
        if after.starts_with('(') {
            return true;
        }
        rest = &rest[pos + "fn main".len()..];
    }
    false
}

fn has_mod(block: &str) -> bool {
    for line in block.lines() {
        let mut t = line.trim_start();
        if let Some(after) = t.strip_prefix("pub") {
            t = after.trim_start();
            if t.starts_with('(') {
                if let Some(close) = t.find(')') {
                    t = t[close + 1..].trim_start();
                }
            }
        }
        if t.starts_with("mod ") {
            return true;
        }
    }
    false
}

fn parse_diagnostics(stdout: &str) -> BTreeMap<String, Vec<(String, String)>> {
    let mut map: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for line in stdout.lines() {
        let line = line.trim();
        if !line.starts_with('{') {
            continue;
        }
        let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if value.get("reason").and_then(|r| r.as_str()) != Some("compiler-message") {
            continue;
        }
        let message = &value["message"];
        if message.get("level").and_then(|l| l.as_str()) != Some("error") {
            continue;
        }
        let Some(target) = value
            .get("target")
            .and_then(|t| t.get("name"))
            .and_then(|n| n.as_str())
        else {
            continue;
        };
        let code = message
            .get("code")
            .and_then(|c| c.get("code"))
            .and_then(|c| c.as_str())
            .unwrap_or("")
            .to_string();
        let text = message
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("")
            .to_string();
        map.entry(target.to_string())
            .or_default()
            .push((code, text));
    }
    map
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Cat {
    Fragment,
    Artifact,
    Low,
    Suspect,
}

fn is_resolution(code: &str, msg: &str) -> bool {
    RES_CODES.contains(&code) || RES_PREFIXES.iter().any(|p| msg.starts_with(p))
}

fn is_artifact(code: &str, msg: &str) -> bool {
    msg.contains("parameter is only allowed in associated functions")
        || code == "E0586"
        || code == "E0585"
        || msg.contains("`...`")
        || msg.contains("missing documentation")
        || msg.contains("await is only allowed inside")
}

fn diag_cat(code: &str, msg: &str) -> Cat {
    if is_resolution(code, msg) {
        Cat::Fragment
    } else if is_artifact(code, msg) {
        Cat::Artifact
    } else if LOW_CODES.contains(&code) {
        Cat::Low
    } else {
        Cat::Suspect
    }
}

fn report(examples: &[Example], diagnostics: &BTreeMap<String, Vec<(String, String)>>) -> ExitCode {
    let mut clean = 0;
    let mut fragments = 0;
    let mut artifacts = 0;
    let mut low = 0;
    let mut suspects: Vec<&Example> = Vec::new();

    for example in examples {
        match diagnostics.get(&example.name) {
            None => clean += 1,
            Some(diags) => {
                let bucket = diags
                    .iter()
                    .map(|(code, msg)| diag_cat(code, msg))
                    .max()
                    .unwrap_or(Cat::Fragment);
                match bucket {
                    Cat::Fragment => fragments += 1,
                    Cat::Artifact => artifacts += 1,
                    Cat::Low => low += 1,
                    Cat::Suspect => suspects.push(example),
                }
            }
        }
    }

    println!("== compile-check summary ==");
    println!("examples checked            : {}", examples.len());
    println!("compiled clean              : {clean}");
    println!("fragments (undefined syms)  : {fragments}");
    println!("wrapper/pseudocode artifacts: {artifacts}");
    println!("low-signal (needs type ann) : {low}");
    println!("SUSPECT (review these)      : {}", suspects.len());

    if suspects.is_empty() {
        ExitCode::SUCCESS
    } else {
        println!();
        for example in &suspects {
            let diags = &diagnostics[&example.name];
            let codes: Vec<&str> = diags
                .iter()
                .filter(|(code, msg)| diag_cat(code, msg) == Cat::Suspect)
                .map(|(code, msg)| {
                    if code.is_empty() {
                        msg.as_str()
                    } else {
                        code.as_str()
                    }
                })
                .collect();
            eprintln!(
                "{}.md:{} [{}] {}",
                example.rule_id,
                example.line,
                example.section,
                codes.join(", ")
            );
            if let Some((_, msg)) = diags
                .iter()
                .find(|(code, msg)| diag_cat(code, msg) == Cat::Suspect)
            {
                eprintln!("    {msg}");
            }
        }
        eprintln!("check-examples: {} suspect block(s)", suspects.len());
        ExitCode::FAILURE
    }
}
