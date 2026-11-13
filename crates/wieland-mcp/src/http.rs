use std::io::Cursor;
use std::sync::Arc;

use tiny_http::{Header, Method, Response, Server};

use wieland_core::{Corpus, Target, corpus_sha};

pub fn serve(
    addr: &str,
    corpus: Arc<Corpus>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server = Server::http(addr)?;
    let sha = corpus_sha(&corpus);
    eprintln!(
        "wieland-mcp: HTTP serving {} rules on http://{addr}",
        corpus.len()
    );
    for request in server.incoming_requests() {
        if *request.method() != Method::Get {
            let _ = request.respond(response(
                405,
                "text/plain",
                "method not allowed".to_string(),
            ));
            continue;
        }
        let path = request.url().split('?').next().unwrap_or("").to_string();
        let (status, ctype, body) = route(&path, &corpus, &sha);
        let _ = request.respond(response(status, ctype, body));
    }
    Ok(())
}

fn route(path: &str, corpus: &Corpus, sha: &str) -> (u16, &'static str, String) {
    match path {
        "/version" => (200, "application/json", version_json(corpus, sha)),
        "/categories.json" => (200, "application/json", categories_json(corpus)),
        "/bundle/core.txt" => match core_txt(corpus, sha) {
            Some(txt) => (200, "text/plain; charset=utf-8", txt),
            None => (500, "text/plain", "core.txt render failed".to_string()),
        },
        _ => {
            if let Some(id) = path.strip_prefix("/rules/") {
                match corpus.rules.iter().find(|r| r.id == id) {
                    Some(rule) => (200, "text/markdown; charset=utf-8", rule.raw.clone()),
                    None => (404, "text/plain", format!("no rule `{id}`\n")),
                }
            } else {
                (
                    404,
                    "text/plain",
                    format!(
                        "not found: {path}\nroutes: /version /categories.json /bundle/core.txt /rules/<id>\n"
                    ),
                )
            }
        }
    }
}

fn version_json(corpus: &Corpus, sha: &str) -> String {
    let tag = format!("wieland-corpus-{}", &sha[..sha.len().min(12)]);
    serde_json::json!({
        "server": "wieland-forge",
        "version": env!("CARGO_PKG_VERSION"),
        "corpus_sha": sha,
        "corpus_tag": tag,
        "rule_count": corpus.len(),
    })
    .to_string()
}

fn categories_json(corpus: &Corpus) -> String {
    let cats: Vec<serde_json::Value> = wieland_core::taxonomy::categories()
        .into_iter()
        .map(|c| {
            let count = corpus.rules.iter().filter(|r| r.category == c.key).count();
            serde_json::json!({ "key": c.key, "prefix": c.prefix, "count": count })
        })
        .collect();
    serde_json::Value::Array(cats).to_string()
}

fn core_txt(corpus: &Corpus, sha: &str) -> Option<String> {
    let artifacts = Target::Bundle.render(corpus, sha).ok()?;
    artifacts
        .into_iter()
        .find(|a| a.rel_path.to_str() == Some("core.txt"))
        .map(|a| a.contents)
}

fn response(status: u16, content_type: &str, body: String) -> Response<Cursor<Vec<u8>>> {
    let base = Response::from_string(body).with_status_code(status);
    match Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()) {
        Ok(header) => base.with_header(header),
        Err(()) => base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use wieland_core::parse_rule;

    fn corpus() -> Corpus {
        let text = "---\nid: own-x\ntitle: title\ncategory: ownership\nseverity: warn\ntier: 0\nenforceability: judge\nrust_edition: \"2024\"\nmsrv: null\ndetect:\n  clippy: []\n  grep: []\n  ast_query: null\nautofix: false\nsources:\n  - test\nsupersedes: []\nstatus: stable\n---\n\n## Rule\nr\n\n## Why\nw\n\n## Do\n```rust\nfn a() {}\n```\n\n## Don't\n```rust\nfn b() {}\n```\n";
        let rule = parse_rule(Path::new("own-x.md"), text).expect("test rule parses");
        Corpus {
            root: "rules".into(),
            rules: vec![rule],
        }
    }

    #[test]
    fn version_route_reports_hash_and_tag() {
        let c = corpus();
        let sha = corpus_sha(&c);
        let (status, ctype, body) = route("/version", &c, &sha);
        assert_eq!(status, 200);
        assert_eq!(ctype, "application/json");
        assert!(body.contains(&sha));
        assert!(body.contains("wieland-corpus-"));
    }

    #[test]
    fn rules_route_returns_markdown_then_404() {
        let c = corpus();
        let sha = corpus_sha(&c);
        let (status, ctype, body) = route("/rules/own-x", &c, &sha);
        assert_eq!(status, 200);
        assert_eq!(ctype, "text/markdown; charset=utf-8");
        assert!(body.contains("id: own-x"));
        let (missing, _, _) = route("/rules/nope", &c, &sha);
        assert_eq!(missing, 404);
    }

    #[test]
    fn core_and_categories_routes() {
        let c = corpus();
        let sha = corpus_sha(&c);
        let (status, _, body) = route("/bundle/core.txt", &c, &sha);
        assert_eq!(status, 200);
        assert!(body.contains("own-x"));
        let (cat_status, cat_ctype, cat_body) = route("/categories.json", &c, &sha);
        assert_eq!(cat_status, 200);
        assert_eq!(cat_ctype, "application/json");
        assert!(cat_body.contains("ownership"));
    }

    #[test]
    fn unknown_route_is_404() {
        let c = corpus();
        let sha = corpus_sha(&c);
        let (status, _, _) = route("/nope", &c, &sha);
        assert_eq!(status, 404);
    }
}
