use std::path::{Path, PathBuf};
use std::process::Command;

pub fn tools_root() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("WIELAND_TOOLS") {
        return Some(PathBuf::from(dir));
    }
    let home = std::env::var("HOME").ok()?;
    Some(Path::new(&home).join(".wieland").join("tools"))
}

pub fn typescript_bin(name: &str) -> Result<PathBuf, String> {
    let root = tools_root().ok_or("HOME is not set")?;
    let path = root
        .join("typescript")
        .join("node_modules")
        .join(".bin")
        .join(name);
    if path.is_file() {
        Ok(path)
    } else {
        Err(format!(
            "{} not found (run ci/install.sh to install the pinned TypeScript tools)",
            path.display()
        ))
    }
}

pub fn project_tsc(project: &Path) -> Result<PathBuf, String> {
    let local = project.join("node_modules").join(".bin").join("tsc");
    if local.is_file() {
        return Ok(local);
    }
    typescript_bin("tsc")
}

pub fn oxlint_command() -> Result<Command, String> {
    let oxlint = typescript_bin("oxlint")?;
    let tsgolint = typescript_bin("tsgolint")?;
    let mut cmd = Command::new(oxlint);
    cmd.env("OXLINT_TSGOLINT_PATH", tsgolint);
    Ok(cmd)
}

pub fn go_bin(name: &str) -> Result<PathBuf, String> {
    let root = tools_root().ok_or("HOME is not set")?;
    let path = root.join("go").join("bin").join(name);
    if path.is_file() {
        Ok(path)
    } else {
        Err(format!(
            "{} not found (run ci/install.sh to install the pinned Go tools)",
            path.display()
        ))
    }
}

pub fn scratch_dir(name: &str) -> Result<PathBuf, String> {
    let base = std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir());
    let dir = base.join(format!("wieland-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).map_err(|e| format!("creating {}: {e}", dir.display()))?;
    Ok(dir)
}

pub fn write(path: &Path, contents: &str) -> Result<(), String> {
    std::fs::write(path, contents).map_err(|e| format!("writing {}: {e}", path.display()))
}
