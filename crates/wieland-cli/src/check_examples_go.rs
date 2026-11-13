use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use wieland_core::frontmatter::section_body;
use wieland_core::{Language, load_pack};

use crate::tools;

const PROJECT_DIR: &str = "target/check-examples-go";

pub fn go_blocks(section: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut current: Option<Vec<&str>> = None;
    for line in section.lines() {
        match current.as_mut() {
            None => {
                if line.trim_start() == "```go" {
                    current = Some(Vec::new());
                }
            }
            Some(lines) => {
                if line.trim_start() == "```" {
                    blocks.push(lines.join("\n"));
                    current = None;
                } else {
                    lines.push(line);
                }
            }
        }
    }
    blocks
}

pub fn parse_go_diagnostic(line: &str) -> Option<(String, u64, String)> {
    let line = line.strip_prefix("vet: ").unwrap_or(line);
    let (file, rest) = line.split_once(".go:")?;
    let (line_no, rest) = rest.split_once(':')?;
    let line_no = line_no.parse().ok()?;
    let message = rest
        .split_once(": ")
        .map(|(_, m)| m)
        .unwrap_or(rest)
        .trim()
        .to_string();
    Some((format!("{file}.go"), line_no, message))
}

pub fn run(rules_root: &Path) -> Result<bool, String> {
    let Some(corpus) = load_pack(rules_root, Language::Go).map_err(|e| e.to_string())? else {
        return Ok(true);
    };
    let project = Path::new(PROJECT_DIR);
    std::fs::create_dir_all(project).map_err(|e| format!("creating {}: {e}", project.display()))?;
    tools::write(&project.join("go.mod"), "module examples\n\ngo 1.26\n")?;

    let mut packages: BTreeMap<String, String> = BTreeMap::new();
    for rule in &corpus.rules {
        let Some(section) = section_body(&rule.body, "Do") else {
            continue;
        };
        for (n, block) in go_blocks(&section).into_iter().enumerate() {
            let package = format!("{}_{n}", rule.id.replace('-', "_"));
            let dir = project.join(&package);
            std::fs::create_dir_all(&dir)
                .map_err(|e| format!("creating {}: {e}", dir.display()))?;
            tools::write(
                &dir.join("example.go"),
                &format!("package {package}\n\n{block}\n"),
            )?;
            packages.insert(package, rule.id.clone());
        }
    }

    let list: Vec<String> = packages.keys().map(|p| format!("./{p}")).collect();
    println!(
        "check-examples: {} Go Do block(s) from {} rule(s); running go vet...",
        packages.len(),
        corpus.len()
    );
    let output = Command::new("go")
        .arg("vet")
        .args(&list)
        .current_dir(project)
        .output()
        .map_err(|e| format!("running go vet: {e}"))?;
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let mut broken = 0;
    for line in text.lines() {
        let Some((file, line_no, message)) = parse_go_diagnostic(line) else {
            continue;
        };
        let package = file
            .trim_start_matches("./")
            .split('/')
            .next()
            .unwrap_or("");
        let rule = packages.get(package).map(String::as_str).unwrap_or("?");
        eprintln!("check-examples: [{rule}] {file}:{line_no}: {message}");
        broken += 1;
    }
    if broken == 0 && !output.status.success() {
        return Err(format!("go vet failed: {}", text.trim()));
    }
    if broken == 0 {
        println!("check-examples: Go {} clean / 0 broken", packages.len());
        Ok(true)
    } else {
        eprintln!("check-examples: Go {broken} error(s)");
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_vet_line() {
        assert_eq!(
            parse_go_diagnostic("err_x_0/example.go:7:2: declared and not used: n"),
            Some((
                "err_x_0/example.go".to_string(),
                7,
                "declared and not used: n".to_string()
            ))
        );
        assert_eq!(
            parse_go_diagnostic(
                "vet: conc_x_0/example.go:12:3: copylocks: call of f copies lock value"
            ),
            Some((
                "conc_x_0/example.go".to_string(),
                12,
                "copylocks: call of f copies lock value".to_string()
            ))
        );
        assert_eq!(parse_go_diagnostic("# examples/err_x_0"), None);
    }
}
