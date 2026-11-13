use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use wieland_core::frontmatter::section_body;
use wieland_core::{Language, load_pack, tsconfig_flags};

use crate::gate_typescript::parse_tsc_line;
use crate::tools;

const PROJECT_DIR: &str = "target/check-examples-ts";

pub struct TsExample {
    pub file: String,
    pub rule_id: String,
}

pub fn ts_blocks(section: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut current: Option<Vec<&str>> = None;
    for line in section.lines() {
        match current.as_mut() {
            None => {
                if line.trim_start() == "```ts" || line.trim_start() == "```typescript" {
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

pub fn run(rules_root: &Path) -> Result<bool, String> {
    let Some(corpus) = load_pack(rules_root, Language::Typescript).map_err(|e| e.to_string())?
    else {
        return Ok(true);
    };
    let tsc = tools::typescript_bin("tsc")?;
    let types_root = tools::tools_root()
        .ok_or("HOME is not set")?
        .join("typescript")
        .join("node_modules")
        .join("@types");
    let project = Path::new(PROJECT_DIR);
    let examples_dir = project.join("examples");
    std::fs::create_dir_all(&examples_dir)
        .map_err(|e| format!("creating {}: {e}", examples_dir.display()))?;

    let mut examples: Vec<TsExample> = Vec::new();
    for rule in &corpus.rules {
        let Some(section) = section_body(&rule.body, "Do") else {
            continue;
        };
        for (n, block) in ts_blocks(&section).into_iter().enumerate() {
            let file = format!("examples/{}__{n}.ts", rule.id.replace('-', "_"));
            tools::write(&project.join(&file), &format!("{block}\n"))?;
            examples.push(TsExample {
                file,
                rule_id: rule.id.clone(),
            });
        }
    }

    let mut options: Vec<String> = vec![
        "\"target\": \"es2024\"".to_string(),
        "\"module\": \"esnext\"".to_string(),
        "\"moduleResolution\": \"bundler\"".to_string(),
        "\"lib\": [\"es2024\", \"dom\"]".to_string(),
        "\"types\": [\"node\"]".to_string(),
        format!("\"typeRoots\": [\"{}\"]", types_root.display()),
        "\"noEmit\": true".to_string(),
    ];
    for flag in tsconfig_flags(&corpus) {
        options.push(format!("\"{}\": true", flag.name));
    }
    let files: Vec<String> = examples.iter().map(|e| format!("\"{}\"", e.file)).collect();
    tools::write(
        &project.join("tsconfig.json"),
        &format!(
            "{{\n  \"compilerOptions\": {{\n    {}\n  }},\n  \"files\": [{}]\n}}\n",
            options.join(",\n    "),
            files.join(", ")
        ),
    )?;

    println!(
        "check-examples: {} TypeScript Do block(s) from {} rule(s); running tsc...",
        examples.len(),
        corpus.len()
    );
    let output = Command::new(&tsc)
        .args(["-p", "tsconfig.json", "--pretty", "false"])
        .current_dir(project)
        .output()
        .map_err(|e| format!("running {}: {e}", tsc.display()))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let by_file: BTreeMap<&str, &str> = examples
        .iter()
        .map(|e| (e.file.as_str(), e.rule_id.as_str()))
        .collect();
    let mut broken = 0;
    for line in stdout.lines() {
        let Some(error) = parse_tsc_line(line) else {
            continue;
        };
        let rule = by_file.get(error.file.as_str()).copied().unwrap_or("?");
        eprintln!(
            "check-examples: [{rule}] {}:{}: {} {}",
            error.file, error.line, error.code, error.message
        );
        broken += 1;
    }
    if broken == 0 && !output.status.success() {
        return Err(format!("tsc failed: {}", stdout.trim()));
    }
    if broken == 0 {
        println!(
            "check-examples: TypeScript {} clean / 0 broken",
            examples.len()
        );
        Ok(true)
    } else {
        eprintln!("check-examples: TypeScript {broken} error(s)");
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_ts_blocks_only() {
        let section =
            "text\n```ts\nconst a = 1;\n```\n```json\n{}\n```\n```ts\nconst b = 2;\n```\n";
        assert_eq!(ts_blocks(section), vec!["const a = 1;", "const b = 2;"]);
    }
}
