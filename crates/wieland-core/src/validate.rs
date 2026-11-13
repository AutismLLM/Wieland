use std::collections::HashMap;
use std::collections::HashSet;

use crate::corpus::Corpus;
use crate::frontmatter::section_headings;
use crate::rule::{Enforceability, Language, Rule};
use crate::taxonomy;

#[derive(Debug, Clone)]
pub struct Finding {
    pub rule_id: String,
    pub message: String,
}

const REQUIRED_SECTIONS: [&str; 4] = ["Rule", "Why", "Do", "Don't"];
const EDITIONS: [&str; 4] = ["2015", "2018", "2021", "2024"];
const STATUSES: [&str; 3] = ["draft", "stable", "deprecated"];

pub fn validate_corpus(corpus: &Corpus) -> Vec<Finding> {
    let mut findings = Vec::new();
    let prefixes: HashMap<Language, HashMap<String, String>> = Language::ALL
        .into_iter()
        .map(|language| {
            let map = taxonomy::categories_for(language)
                .into_iter()
                .map(|c| (c.key, c.prefix))
                .collect();
            (language, map)
        })
        .collect();

    let mut seen: HashMap<&str, usize> = HashMap::new();
    for rule in &corpus.rules {
        *seen.entry(rule.id.as_str()).or_insert(0) += 1;
    }

    for rule in &corpus.rules {
        let mut push = |message: String| {
            findings.push(Finding {
                rule_id: rule.id.clone(),
                message,
            });
        };

        if seen.get(rule.id.as_str()).copied().unwrap_or(0) > 1 {
            push(format!("duplicate id `{}`", rule.id));
        }

        if !is_valid_id(&rule.id) {
            push(format!("id `{}` must match ^[a-z]+-[a-z0-9-]+$", rule.id));
        }

        if rule.title.trim().is_empty() {
            push("title is empty".to_string());
        }
        if rule.title.chars().count() > 90 {
            push(format!(
                "title exceeds 90 chars ({})",
                rule.title.chars().count()
            ));
        }

        match prefixes
            .get(&rule.language)
            .and_then(|map| map.get(&rule.category))
        {
            None => push(format!(
                "unknown category `{}` (not in {})",
                rule.category,
                taxonomy::taxonomy_file(rule.language)
            )),
            Some(prefix) => {
                if !rule.id.starts_with(&format!("{prefix}-")) {
                    push(format!(
                        "id `{}` must start with `{prefix}-` for category `{}`",
                        rule.id, rule.category
                    ));
                }
            }
        }

        if rule.sources.is_empty() {
            push("sources must list at least one provenance entry".to_string());
        }

        if !STATUSES.contains(&rule.status.as_str()) {
            push(format!(
                "status `{}` must be one of draft|stable|deprecated",
                rule.status
            ));
        }

        match rule.language {
            Language::Rust => {
                if !EDITIONS.contains(&rule.rust_edition.as_str()) {
                    push(format!(
                        "rust_edition `{}` must be one of 2015|2018|2021|2024",
                        rule.rust_edition
                    ));
                }
                if rule.ts_version.is_some() || rule.go_version.is_some() {
                    push("a Rust rule may not carry ts_version or go_version".to_string());
                }
            }
            Language::Typescript => {
                check_version(rule.ts_version.as_deref(), "ts_version", &mut push);
                if rule.msrv.is_some() || rule.go_version.is_some() {
                    push("a TypeScript rule may not carry msrv or go_version".to_string());
                }
            }
            Language::Go => {
                check_version(rule.go_version.as_deref(), "go_version", &mut push);
                if rule.msrv.is_some() || rule.ts_version.is_some() {
                    push("a Go rule may not carry msrv or ts_version".to_string());
                }
            }
        }

        for foreign in rule.detect.foreign_detectors(rule.language) {
            push(format!(
                "a {} rule may not carry detect.{foreign}",
                rule.language.name()
            ));
        }

        if rule.enforceability == Enforceability::Gate
            && !rule.detect.has_gate_detector_for(rule.language)
        {
            if rule.language == Language::Rust {
                push("enforceability=gate requires a detect.clippy lint; ast_query is not run by the gate".to_string());
            } else {
                push(format!(
                    "enforceability=gate requires {} for a {} rule; grep and ast_query are not run by the gate",
                    rule.language.gate_detector_names(),
                    rule.language.name()
                ));
            }
        }

        if rule.tier.needs_detector() && rule.detect.is_empty() {
            push("tier 2/3 requires a non-empty detect{}".to_string());
        }

        for section in missing_sections(rule) {
            push(format!("body is missing required `## {section}` section"));
        }
    }

    findings
}

fn check_version(value: Option<&str>, field: &str, push: &mut impl FnMut(String)) {
    match value {
        None => push(format!("{field} is required for this language")),
        Some(v) if !is_major_minor(v) => push(format!(
            "{field} `{v}` must look like MAJOR.MINOR, e.g. \"7.0\""
        )),
        Some(_) => {}
    }
}

fn is_major_minor(value: &str) -> bool {
    let Some((major, minor)) = value.split_once('.') else {
        return false;
    };
    !major.is_empty()
        && !minor.is_empty()
        && major.chars().all(|c| c.is_ascii_digit())
        && minor.chars().all(|c| c.is_ascii_digit())
}

fn missing_sections(rule: &Rule) -> Vec<&'static str> {
    let present: HashSet<String> = section_headings(&rule.body).into_iter().collect();
    REQUIRED_SECTIONS
        .into_iter()
        .filter(|s| !present.contains(*s))
        .collect()
}

fn is_valid_id(id: &str) -> bool {
    let Some((prefix, rest)) = id.split_once('-') else {
        return false;
    };
    if prefix.is_empty() || rest.is_empty() {
        return false;
    }
    if !prefix.chars().all(|c| c.is_ascii_lowercase()) {
        return false;
    }
    rest.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}
