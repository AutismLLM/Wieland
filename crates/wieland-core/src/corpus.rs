use std::path::{Path, PathBuf};

use crate::frontmatter::parse_rule;
use crate::rule::Rule;

#[derive(Debug)]
pub enum CorpusError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Parse {
        path: PathBuf,
        message: String,
    },
}

impl std::fmt::Display for CorpusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CorpusError::Io { path, source } => {
                write!(f, "{}: {source}", path.display())
            }
            CorpusError::Parse { path, message } => {
                write!(f, "{}: {message}", path.display())
            }
        }
    }
}

impl std::error::Error for CorpusError {}

#[derive(Debug, Default)]
pub struct Corpus {
    pub root: PathBuf,
    pub rules: Vec<Rule>,
}

impl Corpus {
    pub fn load(root: impl AsRef<Path>) -> Result<Self, CorpusError> {
        let root = root.as_ref().to_path_buf();
        let mut paths = Vec::new();
        let entries = std::fs::read_dir(&root).map_err(|source| CorpusError::Io {
            path: root.clone(),
            source,
        })?;
        for entry in entries {
            let entry = entry.map_err(|source| CorpusError::Io {
                path: root.clone(),
                source,
            })?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                paths.push(path);
            }
        }
        paths.sort();

        let mut rules = Vec::with_capacity(paths.len());
        for path in paths {
            let text = std::fs::read_to_string(&path).map_err(|source| CorpusError::Io {
                path: path.clone(),
                source,
            })?;
            let rule = parse_rule(&path, &text).map_err(|err| CorpusError::Parse {
                path: path.clone(),
                message: err.message,
            })?;
            rules.push(rule);
        }

        Ok(Self { root, rules })
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}
