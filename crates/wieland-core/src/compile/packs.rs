use std::path::{Path, PathBuf};

use super::{Artifact, CompileError, render_all};
use crate::corpus::Corpus;
use crate::rule::Language;

pub fn pack_root(rules_root: &Path, language: Language) -> PathBuf {
    match language.pack_dir() {
        Some(dir) => rules_root.join(dir),
        None => rules_root.to_path_buf(),
    }
}

pub fn load_pack(rules_root: &Path, language: Language) -> Result<Option<Corpus>, CompileError> {
    let root = pack_root(rules_root, language);
    if !root.is_dir() {
        return Ok(None);
    }
    let corpus = Corpus::load(&root).map_err(|err| CompileError::Pack {
        message: format!("loading {} pack: {err}", language.name()),
    })?;
    if let Some(stray) = corpus.rules.iter().find(|r| r.language != language) {
        return Err(CompileError::Pack {
            message: format!(
                "{}: rule `{}` says language {} but sits in the {} pack",
                stray.path.display(),
                stray.id,
                stray.language.name(),
                language.name()
            ),
        });
    }
    if corpus.is_empty() {
        return Ok(None);
    }
    Ok(Some(corpus))
}

pub fn render_packs(rules_root: &Path) -> Result<Vec<Artifact>, CompileError> {
    let mut artifacts = Vec::new();
    for language in Language::ALL {
        if let Some(corpus) = load_pack(rules_root, language)? {
            artifacts.extend(render_all(&corpus)?);
        }
    }
    Ok(artifacts)
}
