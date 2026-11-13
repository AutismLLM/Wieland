use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::transport::stdio;
use rmcp::{ServerHandler, ServiceExt, schemars, tool, tool_handler, tool_router};

use wieland_core::compile::{ordered, tier0};
use wieland_core::rule::{Enforceability, Rule, Severity};
use wieland_core::{Corpus, DenyLint, corpus_sha, deny_set, taxonomy};

mod http;

#[derive(serde::Deserialize, schemars::JsonSchema)]
struct LookupArgs {
    #[schemars(description = "substring matched against rule id, title, tags, and category")]
    query: String,
    #[schemars(description = "optional category key to restrict the search (e.g. errors)")]
    #[serde(default)]
    category: Option<String>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
struct ExplainArgs {
    #[schemars(description = "a rule id (e.g. err-no-unwrap-prod) OR a clippy lint name")]
    target: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
struct ReviewArgs {
    #[schemars(
        description = "path to a crate directory (with Cargo.toml) to run the clippy deny-set against"
    )]
    #[serde(default)]
    path: Option<String>,
    #[schemars(
        description = "a Rust snippet; checked with grep heuristics only, clippy is skipped"
    )]
    #[serde(default)]
    code: Option<String>,
}

pub struct Wieland {
    corpus: Arc<Corpus>,
    sha: String,
    denies: Vec<DenyLint>,
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

impl Wieland {
    fn new(corpus: Arc<Corpus>) -> Self {
        let sha = corpus_sha(&corpus);
        let denies = deny_set(&corpus);
        Self {
            corpus,
            sha,
            denies,
            tool_router: Self::tool_router(),
        }
    }

    fn grep_review(&self, code: &str) -> String {
        let mut hits = Vec::new();
        for rule in &self.corpus.rules {
            for pattern in &rule.detect.grep {
                if let Ok(re) = regex::Regex::new(pattern) {
                    if re.is_match(code) {
                        hits.push(format!("[{}] {} (grep /{}/)", rule.id, rule.title, pattern));
                        break;
                    }
                }
            }
        }
        let mut out = String::from(
            "note: clippy skipped — a snippet has no crate context; grep heuristics only (advisory, may false-positive).\n\n",
        );
        if hits.is_empty() {
            out.push_str("no grep heuristics matched.");
        } else {
            out.push_str(&hits.join("\n"));
        }
        out
    }

    fn clippy_review(&self, dir: &Path) -> String {
        if !dir.join("Cargo.toml").exists() {
            return format!("{}: no Cargo.toml — not a crate directory", dir.display());
        }
        let mut cmd = Command::new("cargo");
        cmd.current_dir(dir)
            .args(["clippy", "--message-format=json", "--quiet"])
            .arg("--");
        for entry in &self.denies {
            cmd.arg("-D").arg(format!("clippy::{}", entry.lint));
        }
        let output = match cmd.output() {
            Ok(output) => output,
            Err(err) => return format!("failed to run cargo clippy: {err}"),
        };
        let by_code: HashMap<String, &DenyLint> = self
            .denies
            .iter()
            .map(|d| (format!("clippy::{}", d.lint), d))
            .collect();
        let mut findings = Vec::new();
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
                continue;
            };
            if value.get("reason").and_then(|r| r.as_str()) != Some("compiler-message") {
                continue;
            }
            let message = &value["message"];
            let Some(code) = message
                .get("code")
                .and_then(|c| c.get("code"))
                .and_then(|c| c.as_str())
            else {
                continue;
            };
            let Some(deny) = by_code.get(code) else {
                continue;
            };
            let (file, line_no) = primary_span(&message["spans"]);
            findings.push(format!(
                "{file}:{line_no}: [{}] {}",
                deny.rule_id, deny.title
            ));
        }
        if findings.is_empty() {
            format!(
                "clippy deny-set clean ({} lint(s) checked) in {}",
                self.denies.len(),
                dir.display()
            )
        } else {
            findings.join("\n")
        }
    }
}

#[tool_router]
impl Wieland {
    #[tool(description = "Report the Wieland server + corpus version and content hash")]
    fn version(&self) -> String {
        serde_json::json!({
            "server": "wieland-forge",
            "version": env!("CARGO_PKG_VERSION"),
            "corpus_sha": self.sha,
            "rule_count": self.corpus.len(),
        })
        .to_string()
    }

    #[tool(description = "List the rule categories with their id prefix and rule count")]
    fn categories(&self) -> String {
        let mut out = String::new();
        for category in taxonomy::categories() {
            let count = self
                .corpus
                .rules
                .iter()
                .filter(|r| r.category == category.key)
                .count();
            out.push_str(&format!(
                "{} (prefix {}-) — {} rule(s)\n",
                category.key, category.prefix, count
            ));
        }
        out
    }

    #[tool(description = "Return the always-on L0 core rules (tier 0) in full")]
    fn core(&self) -> String {
        tier0(&self.corpus)
            .iter()
            .map(|r| render_rule(r))
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    #[tool(description = "Search rules by substring across id, title, tags, and category")]
    fn lookup(&self, Parameters(args): Parameters<LookupArgs>) -> String {
        let needle = args.query.to_lowercase();
        let mut hits = Vec::new();
        for rule in ordered(&self.corpus) {
            if let Some(cat) = &args.category {
                if &rule.category != cat {
                    continue;
                }
            }
            let hay = format!(
                "{} {} {} {}",
                rule.id,
                rule.title.to_lowercase(),
                rule.category,
                rule.tags.join(" ").to_lowercase()
            );
            if hay.to_lowercase().contains(&needle) {
                hits.push(format!(
                    "{} — {} (tier {} · {})",
                    rule.id,
                    rule.title,
                    rule.tier.level(),
                    enforceability_word(rule.enforceability)
                ));
            }
        }
        if hits.is_empty() {
            format!("no rules match `{}`", args.query)
        } else {
            format!("{} match(es):\n{}", hits.len(), hits.join("\n"))
        }
    }

    #[tool(description = "Explain a rule by id, or the rule(s) that own a clippy lint")]
    fn explain(&self, Parameters(args): Parameters<ExplainArgs>) -> String {
        if let Some(rule) = self.corpus.rules.iter().find(|r| r.id == args.target) {
            return render_rule(rule);
        }
        let lint = args.target.strip_prefix("clippy::").unwrap_or(&args.target);
        let owners: Vec<&Rule> = ordered(&self.corpus)
            .into_iter()
            .filter(|r| r.detect.clippy.iter().any(|l| l == lint))
            .collect();
        if owners.is_empty() {
            format!(
                "no rule with id `{}` and no rule owns clippy::{lint}",
                args.target
            )
        } else {
            owners
                .iter()
                .map(|r| render_rule(r))
                .collect::<Vec<_>>()
                .join("\n\n")
        }
    }

    #[tool(
        description = "Review Rust: `path` (crate dir) runs the clippy deny-set; `code` (snippet) gets grep heuristics and states clippy was skipped"
    )]
    fn review(&self, Parameters(args): Parameters<ReviewArgs>) -> String {
        if let Some(path) = args.path {
            self.clippy_review(Path::new(&path))
        } else if let Some(code) = args.code {
            self.grep_review(&code)
        } else {
            "provide either `path` (a crate directory) or `code` (a Rust snippet)".to_string()
        }
    }
}

#[tool_handler]
impl ServerHandler for Wieland {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_instructions(
            "Wieland — canonical Rust coding rules. Tools: version, categories, core, lookup, explain, review. The corpus is the single source of truth; version reports its content hash.",
        )
    }
}

fn render_rule(rule: &Rule) -> String {
    let ast = rule.detect.ast_query.as_deref().unwrap_or("none");
    format!(
        "# {} — {}\ntier {} · {} · {} · category:{}\ndetect: clippy={:?} grep={:?} ast={}\n\n{}",
        rule.id,
        rule.title,
        rule.tier.level(),
        enforceability_word(rule.enforceability),
        severity_word(rule.severity),
        rule.category,
        rule.detect.clippy,
        rule.detect.grep,
        ast,
        rule.body.trim_end()
    )
}

fn enforceability_word(enforceability: Enforceability) -> &'static str {
    match enforceability {
        Enforceability::Compiler => "compiler",
        Enforceability::Gate => "gate",
        Enforceability::Check => "check",
        Enforceability::Judge => "judge",
    }
}

fn severity_word(severity: Severity) -> &'static str {
    match severity {
        Severity::Info => "info",
        Severity::Warn => "warn",
        Severity::Deny => "deny",
    }
}

fn primary_span(spans: &serde_json::Value) -> (String, u64) {
    let Some(array) = spans.as_array() else {
        return ("<unknown>".to_string(), 0);
    };
    let chosen = array
        .iter()
        .find(|s| s.get("is_primary").and_then(|p| p.as_bool()) == Some(true))
        .or_else(|| array.first());
    match chosen {
        Some(span) => (
            span.get("file_name")
                .and_then(|f| f.as_str())
                .unwrap_or("<unknown>")
                .to_string(),
            span.get("line_start").and_then(|l| l.as_u64()).unwrap_or(0),
        ),
        None => ("<unknown>".to_string(), 0),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = std::env::args().collect();
    let rules = std::env::var("WIELAND_RULES").unwrap_or_else(|_| "rules".to_string());
    let corpus =
        Arc::new(Corpus::load(&rules).map_err(|e| format!("loading corpus at {rules}: {e}"))?);

    if args.get(1).map(String::as_str) == Some("http") {
        let addr = args
            .get(2)
            .cloned()
            .or_else(|| std::env::var("WIELAND_HTTP").ok())
            .unwrap_or_else(|| "127.0.0.1:8787".to_string());
        return tokio::task::spawn_blocking(move || http::serve(&addr, corpus)).await?;
    }

    if let Ok(addr) = std::env::var("WIELAND_HTTP") {
        let http_corpus = Arc::clone(&corpus);
        std::thread::spawn(move || {
            if let Err(err) = http::serve(&addr, http_corpus) {
                eprintln!("wieland-mcp: HTTP server error: {err}");
            }
        });
    }

    let service = Wieland::new(corpus).serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
