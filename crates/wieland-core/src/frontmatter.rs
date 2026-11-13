use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::rule::{Detect, Enforceability, Language, Rule, Severity, Tier};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FrontMatter {
    id: String,
    title: String,
    category: String,
    severity: Severity,
    tier: Tier,
    enforceability: Enforceability,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    applies_to: Vec<String>,
    #[serde(default)]
    language: Language,
    #[serde(default)]
    ts_version: Option<String>,
    #[serde(default)]
    go_version: Option<String>,
    #[serde(default = "default_edition")]
    rust_edition: String,
    #[serde(default)]
    msrv: Option<String>,
    #[serde(default)]
    reviewed: Option<String>,
    #[serde(default)]
    detect: Detect,
    #[serde(default)]
    autofix: bool,
    sources: Vec<String>,
    #[serde(default)]
    supersedes: Vec<String>,
    status: String,
}

fn default_edition() -> String {
    "2024".to_string()
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

fn split(text: &str) -> Result<(&str, &str), ParseError> {
    let rest = text.strip_prefix("---\n").ok_or(ParseError {
        message: "file must begin with a `---` frontmatter fence".to_string(),
    })?;
    if let Some(i) = rest.find("\n---\n") {
        return Ok((&rest[..i], rest[i + 5..].trim_start_matches('\n')));
    }
    if let Some(yaml) = rest.strip_suffix("\n---") {
        return Ok((yaml, ""));
    }
    Err(ParseError {
        message: "unterminated frontmatter: missing closing `---` fence".to_string(),
    })
}

pub fn parse_rule(path: &Path, text: &str) -> Result<Rule, ParseError> {
    let (yaml, body) = split(text)?;
    let fm: FrontMatter = serde_yaml_ng::from_str(yaml).map_err(|err| ParseError {
        message: format!("frontmatter: {err}"),
    })?;
    Ok(Rule {
        id: fm.id,
        title: fm.title,
        category: fm.category,
        severity: fm.severity,
        tier: fm.tier,
        enforceability: fm.enforceability,
        tags: fm.tags,
        applies_to: fm.applies_to,
        language: fm.language,
        ts_version: fm.ts_version,
        go_version: fm.go_version,
        rust_edition: fm.rust_edition,
        msrv: fm.msrv,
        reviewed: fm.reviewed,
        detect: fm.detect,
        autofix: fm.autofix,
        sources: fm.sources,
        supersedes: fm.supersedes,
        status: fm.status,
        body: body.to_string(),
        raw: text.to_string(),
        path: PathBuf::from(path),
    })
}

pub fn section_headings(body: &str) -> Vec<String> {
    body.lines()
        .filter_map(|line| line.strip_prefix("## "))
        .map(|name| name.trim().to_string())
        .collect()
}

pub fn section_body(body: &str, name: &str) -> Option<String> {
    let mut collecting = false;
    let mut found = false;
    let mut out: Vec<&str> = Vec::new();
    for line in body.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if collecting {
                break;
            }
            if heading.trim() == name {
                collecting = true;
                found = true;
            }
            continue;
        }
        if collecting {
            out.push(line);
        }
    }
    if found {
        Some(out.join("\n").trim().to_string())
    } else {
        None
    }
}
