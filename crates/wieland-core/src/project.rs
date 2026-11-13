use std::path::{Path, PathBuf};

use serde::Deserialize;

pub const PROJECT_FILE: &str = "wieland.yaml";

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProjectCheck {
    pub name: String,
    pub run: String,
    #[serde(default)]
    pub extensions: Vec<String>,
    #[serde(default)]
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProjectFile {
    #[serde(default)]
    pub checks: Vec<ProjectCheck>,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub root: PathBuf,
    pub file: ProjectFile,
}

impl ProjectCheck {
    pub fn applies_to(&self, relative: &Path) -> bool {
        let text = relative.to_string_lossy();
        let by_extension = self.extensions.is_empty()
            || self.extensions.iter().any(|e| text.ends_with(e.as_str()));
        let by_path =
            self.paths.is_empty() || self.paths.iter().any(|p| text.starts_with(p.as_str()));
        by_extension && by_path
    }
}

pub fn parse_project(text: &str) -> Result<ProjectFile, String> {
    serde_yaml_ng::from_str(text).map_err(|e| format!("{PROJECT_FILE}: {e}"))
}

pub fn find_project(start: &Path) -> Result<Option<Project>, String> {
    let mut dir = if start.is_dir() {
        Some(start)
    } else {
        start.parent()
    };
    while let Some(current) = dir {
        let candidate = current.join(PROJECT_FILE);
        if candidate.is_file() {
            let text = std::fs::read_to_string(&candidate)
                .map_err(|e| format!("{}: {e}", candidate.display()))?;
            let file = parse_project(&text)?;
            return Ok(Some(Project {
                root: current.to_path_buf(),
                file,
            }));
        }
        dir = current.parent();
    }
    Ok(None)
}
