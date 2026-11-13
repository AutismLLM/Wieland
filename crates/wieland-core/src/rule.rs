use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "u8")]
pub enum Tier {
    Core,
    OnDemand,
    Checked,
    Enforced,
}

impl Tier {
    pub fn level(self) -> u8 {
        match self {
            Tier::Core => 0,
            Tier::OnDemand => 1,
            Tier::Checked => 2,
            Tier::Enforced => 3,
        }
    }

    pub fn needs_detector(self) -> bool {
        matches!(self, Tier::Checked | Tier::Enforced)
    }
}

impl TryFrom<u8> for Tier {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tier::Core),
            1 => Ok(Tier::OnDemand),
            2 => Ok(Tier::Checked),
            3 => Ok(Tier::Enforced),
            other => Err(format!("tier must be one of 0,1,2,3 (got {other})")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warn,
    Deny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Enforceability {
    Compiler,
    Gate,
    Check,
    Judge,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    Rust,
    Typescript,
    Go,
}

impl Language {
    pub const ALL: [Language; 3] = [Language::Rust, Language::Typescript, Language::Go];

    pub fn name(self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Typescript => "typescript",
            Language::Go => "go",
        }
    }

    pub fn from_name(name: &str) -> Option<Language> {
        Language::ALL.into_iter().find(|l| l.name() == name)
    }

    pub fn pack_dir(self) -> Option<&'static str> {
        match self {
            Language::Rust => None,
            Language::Typescript => Some("typescript"),
            Language::Go => Some("go"),
        }
    }

    pub fn gate_detector_names(self) -> &'static str {
        match self {
            Language::Rust => "detect.clippy",
            Language::Typescript => "detect.oxlint or detect.tsconfig",
            Language::Go => "detect.golangci",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Detect {
    #[serde(default)]
    pub clippy: Vec<String>,
    #[serde(default)]
    pub oxlint: Vec<String>,
    #[serde(default)]
    pub tsconfig: Vec<String>,
    #[serde(default)]
    pub golangci: Vec<String>,
    #[serde(default)]
    pub grep: Vec<String>,
    #[serde(default)]
    pub ast_query: Option<String>,
}

impl Detect {
    pub fn is_empty(&self) -> bool {
        self.clippy.is_empty()
            && self.oxlint.is_empty()
            && self.tsconfig.is_empty()
            && self.golangci.is_empty()
            && self.grep.is_empty()
            && self.ast_query.is_none()
    }

    pub fn has_gate_detector(&self) -> bool {
        self.has_gate_detector_for(Language::Rust)
    }

    pub fn has_gate_detector_for(&self, language: Language) -> bool {
        match language {
            Language::Rust => !self.clippy.is_empty(),
            Language::Typescript => !self.oxlint.is_empty() || !self.tsconfig.is_empty(),
            Language::Go => !self.golangci.is_empty(),
        }
    }

    pub fn foreign_detectors(&self, language: Language) -> Vec<&'static str> {
        let mut out = Vec::new();
        if language != Language::Rust && !self.clippy.is_empty() {
            out.push("clippy");
        }
        if language != Language::Typescript && !self.oxlint.is_empty() {
            out.push("oxlint");
        }
        if language != Language::Typescript && !self.tsconfig.is_empty() {
            out.push("tsconfig");
        }
        if language != Language::Go && !self.golangci.is_empty() {
            out.push("golangci");
        }
        out
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub title: String,
    pub category: String,
    pub severity: Severity,
    pub tier: Tier,
    pub enforceability: Enforceability,
    pub tags: Vec<String>,
    pub applies_to: Vec<String>,
    pub language: Language,
    pub ts_version: Option<String>,
    pub go_version: Option<String>,
    pub rust_edition: String,
    pub msrv: Option<String>,
    pub reviewed: Option<String>,
    pub detect: Detect,
    pub autofix: bool,
    pub sources: Vec<String>,
    pub supersedes: Vec<String>,
    pub status: String,
    pub body: String,
    pub raw: String,
    pub path: PathBuf,
}
