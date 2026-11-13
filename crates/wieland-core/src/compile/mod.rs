use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::corpus::Corpus;
use crate::rule::{Language, Rule};

mod bundle;
mod claude_code;
mod deny_set;
mod packs;

pub use deny_set::{
    DenyLint, Detector, Level, deny_set, golangci_linters, golangci_yml, oxlint_rules,
    oxlintrc_json, tsconfig_flags,
};
pub use packs::{load_pack, pack_root, render_packs};

pub const L0_TOKEN_BUDGET: usize = 5000;

#[derive(Debug)]
pub struct Artifact {
    pub rel_path: PathBuf,
    pub contents: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Bundle,
    ClaudeCode,
    DenySet,
}

#[derive(Debug)]
pub enum CompileError {
    Budget {
        artifact: String,
        tokens: usize,
        budget: usize,
    },
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Drift {
        differences: Vec<String>,
    },
    Pack {
        message: String,
    },
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Budget {
                artifact,
                tokens,
                budget,
            } => write!(
                f,
                "{artifact} is {tokens} tokens, over the L0 budget of {budget}"
            ),
            CompileError::Io { path, source } => write!(f, "{}: {source}", path.display()),
            CompileError::Drift { differences } => {
                writeln!(f, "dist/ is stale ({} difference(s)):", differences.len())?;
                for d in differences {
                    writeln!(f, "  {d}")?;
                }
                write!(f, "run `wieland compile --all` and commit dist/")
            }
            CompileError::Pack { message } => f.write_str(message),
        }
    }
}

impl std::error::Error for CompileError {}

impl Target {
    pub const ALL: [Target; 3] = [Target::Bundle, Target::ClaudeCode, Target::DenySet];

    pub fn from_name(name: &str) -> Option<Target> {
        match name {
            "bundle" => Some(Target::Bundle),
            "claude-code" => Some(Target::ClaudeCode),
            "deny-set" => Some(Target::DenySet),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Target::Bundle => "bundle",
            Target::ClaudeCode => "claude-code",
            Target::DenySet => "deny-set",
        }
    }

    pub fn render(self, corpus: &Corpus, sha: &str) -> Result<Vec<Artifact>, CompileError> {
        match self {
            Target::Bundle => bundle::render(corpus, sha),
            Target::ClaudeCode => claude_code::render(corpus, sha),
            Target::DenySet => deny_set::render(corpus, sha),
        }
    }
}

pub fn corpus_sha(corpus: &Corpus) -> String {
    let mut rules: Vec<&Rule> = corpus.rules.iter().collect();
    rules.sort_by(|a, b| a.id.cmp(&b.id));
    let mut hasher = Sha256::new();
    for rule in rules {
        hasher.update(rule.id.as_bytes());
        hasher.update([0u8]);
        hasher.update(rule.raw.as_bytes());
        hasher.update([0u8]);
    }
    let digest = hasher.finalize();
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

pub fn pack_language(corpus: &Corpus) -> Language {
    corpus.rules.first().map(|r| r.language).unwrap_or_default()
}

pub(crate) fn in_pack(language: Language, rel: &str) -> PathBuf {
    match language.pack_dir() {
        Some(dir) => Path::new(dir).join(rel),
        None => PathBuf::from(rel),
    }
}

pub(crate) fn pack_title(language: Language) -> &'static str {
    match language {
        Language::Rust => "Wieland",
        Language::Typescript => "Wieland TypeScript",
        Language::Go => "Wieland Go",
    }
}

pub fn approx_tokens(text: &str) -> usize {
    text.len().div_ceil(4)
}

pub fn ordered(corpus: &Corpus) -> Vec<&Rule> {
    let mut rules: Vec<&Rule> = corpus.rules.iter().collect();
    rules.sort_by(|a, b| a.category.cmp(&b.category).then_with(|| a.id.cmp(&b.id)));
    rules
}

pub fn tier0(corpus: &Corpus) -> Vec<&Rule> {
    let mut rules: Vec<&Rule> = corpus
        .rules
        .iter()
        .filter(|r| r.tier.level() == 0)
        .collect();
    rules.sort_by(|a, b| a.id.cmp(&b.id));
    rules
}

pub fn render_all(corpus: &Corpus) -> Result<Vec<Artifact>, CompileError> {
    let sha = corpus_sha(corpus);
    let mut artifacts = Vec::new();
    for target in Target::ALL {
        artifacts.extend(target.render(corpus, &sha)?);
    }
    Ok(artifacts)
}

pub fn write_artifacts(dist: &Path, artifacts: &[Artifact]) -> Result<(), CompileError> {
    for artifact in artifacts {
        let path = dist.join(&artifact.rel_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| CompileError::Io {
                path: parent.to_path_buf(),
                source,
            })?;
        }
        std::fs::write(&path, &artifact.contents).map_err(|source| CompileError::Io {
            path: path.clone(),
            source,
        })?;
    }
    Ok(())
}

pub fn check_drift(
    dist: &Path,
    artifacts: &[Artifact],
    detect_stale: bool,
) -> Result<(), CompileError> {
    let mut differences = Vec::new();
    let mut produced = std::collections::BTreeSet::new();

    for artifact in artifacts {
        produced.insert(artifact.rel_path.clone());
        let path = dist.join(&artifact.rel_path);
        match std::fs::read_to_string(&path) {
            Ok(on_disk) if on_disk == artifact.contents => {}
            Ok(_) => differences.push(format!("changed: {}", artifact.rel_path.display())),
            Err(_) => differences.push(format!("missing: {}", artifact.rel_path.display())),
        }
    }

    if detect_stale {
        for existing in walk(dist) {
            if let Ok(rel) = existing.strip_prefix(dist) {
                if !produced.contains(rel) {
                    differences.push(format!("stale: {}", rel.display()));
                }
            }
        }
    }

    if differences.is_empty() {
        Ok(())
    } else {
        differences.sort();
        Err(CompileError::Drift { differences })
    }
}

fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(walk(&path));
        } else {
            out.push(path);
        }
    }
    out
}

pub(crate) fn meta_line(rule: &Rule) -> String {
    format!(
        "tier {} · {} · {} · category:{}",
        rule.tier.level(),
        enforceability_name(rule),
        severity_name(rule),
        rule.category
    )
}

pub(crate) fn detect_line(rule: &Rule) -> String {
    let ast = rule.detect.ast_query.as_deref().unwrap_or("none");
    match rule.language {
        Language::Rust => format!(
            "detect: clippy={:?} grep={:?} ast={}",
            rule.detect.clippy, rule.detect.grep, ast
        ),
        Language::Typescript => format!(
            "detect: oxlint={:?} tsconfig={:?}",
            rule.detect.oxlint, rule.detect.tsconfig
        ),
        Language::Go => format!("detect: golangci={:?}", rule.detect.golangci),
    }
}

fn enforceability_name(rule: &Rule) -> &'static str {
    use crate::rule::Enforceability::*;
    match rule.enforceability {
        Compiler => "compiler",
        Gate => "gate",
        Check => "check",
        Judge => "judge",
    }
}

fn severity_name(rule: &Rule) -> &'static str {
    use crate::rule::Severity::*;
    match rule.severity {
        Info => "info",
        Warn => "warn",
        Deny => "deny",
    }
}
