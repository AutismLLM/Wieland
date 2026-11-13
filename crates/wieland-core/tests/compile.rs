use std::path::Path;

use wieland_core::compile::{CompileError, Target};
use wieland_core::corpus::Corpus;
use wieland_core::{L0_TOKEN_BUDGET, Rule, corpus_sha, deny_set, parse_rule, render_all};

fn make_deny_rule(id: &str, lint: &str) -> Rule {
    let text = format!(
        "---\n\
id: {id}\n\
title: title for {id}\n\
category: anti-patterns\n\
severity: deny\n\
tier: 3\n\
enforceability: gate\n\
rust_edition: \"2024\"\n\
msrv: null\n\
detect:\n  clippy: [{lint}]\n  grep: []\n  ast_query: null\n\
autofix: false\n\
sources:\n  - test\n\
supersedes: []\n\
status: stable\n\
---\n\n## Rule\navoid {lint}\n\n## Why\nreason\n\n## Do\n```rust\nfn a() {{}}\n```\n\n## Don't\n```rust\nfn b() {{}}\n```\n"
    );
    parse_rule(Path::new("x.md"), &text).expect("test rule parses")
}

fn make_rule(id: &str, tier: u8, filler: &str) -> Rule {
    let text = format!(
        "---\n\
id: {id}\n\
title: title for {id}\n\
category: ownership\n\
severity: warn\n\
tier: {tier}\n\
enforceability: judge\n\
rust_edition: \"2024\"\n\
msrv: null\n\
detect:\n  clippy: []\n  grep: []\n  ast_query: null\n\
autofix: false\n\
sources:\n  - test\n\
supersedes: []\n\
status: stable\n\
---\n\n## Rule\n{filler}\n\n## Why\nreason\n\n## Do\n```rust\nfn a() {{}}\n```\n\n## Don't\n```rust\nfn b() {{}}\n```\n"
    );
    parse_rule(Path::new("x.md"), &text).expect("test rule parses")
}

fn corpus_of(rules: Vec<Rule>) -> Corpus {
    Corpus {
        root: "rules".into(),
        rules,
    }
}

#[test]
fn render_is_deterministic() {
    let corpus = corpus_of(vec![
        make_rule("own-a", 0, "small"),
        make_rule("own-b", 1, "small"),
        make_rule("mem-c", 2, "small"),
    ]);
    let first = render_all(&corpus).expect("render");
    let second = render_all(&corpus).expect("render");
    assert_eq!(first.len(), second.len());
    for (a, b) in first.iter().zip(second.iter()) {
        assert_eq!(a.rel_path, b.rel_path);
        assert_eq!(a.contents, b.contents);
    }
}

#[test]
fn over_budget_l0_fails_the_build() {
    let big = "x".repeat(L0_TOKEN_BUDGET * 4 + 4000);
    let corpus = corpus_of(vec![make_rule("own-big", 0, &big)]);
    match render_all(&corpus) {
        Err(CompileError::Budget {
            artifact, budget, ..
        }) => {
            assert_eq!(artifact, "core.txt");
            assert_eq!(budget, L0_TOKEN_BUDGET);
        }
        other => panic!("expected Budget error, got {other:?}"),
    }
}

#[test]
fn small_l0_is_under_budget() {
    let corpus = corpus_of(vec![
        make_rule("own-a", 0, "borrow instead of cloning"),
        make_rule("err-b", 0, "return Result not panic"),
    ]);
    assert!(render_all(&corpus).is_ok());
}

#[test]
fn corpus_sha_is_stable_and_content_sensitive() {
    let base = corpus_of(vec![make_rule("own-a", 0, "one")]);
    assert_eq!(corpus_sha(&base), corpus_sha(&base));

    let changed = corpus_of(vec![make_rule("own-a", 0, "two")]);
    assert_ne!(corpus_sha(&base), corpus_sha(&changed));
}

#[test]
fn deny_set_selects_only_deny_gate_clippy_rules() {
    let corpus = corpus_of(vec![
        make_deny_rule("anti-x", "unwrap_used"),
        make_rule("own-judge", 1, "a judge rule with no detector"),
    ]);
    let ds = deny_set(&corpus);
    assert_eq!(ds.len(), 1);
    assert_eq!(ds[0].lint, "unwrap_used");
    assert_eq!(ds[0].rule_id, "anti-x");
    assert!(ds[0].statement.contains("avoid unwrap_used"));
}

#[test]
fn deny_set_dedupes_shared_lints() {
    let corpus = corpus_of(vec![
        make_deny_rule("anti-a", "unwrap_used"),
        make_deny_rule("anti-b", "unwrap_used"),
    ]);
    let ds = deny_set(&corpus);
    assert_eq!(ds.len(), 1);
    assert_eq!(ds[0].rule_id, "anti-a");
}

#[test]
fn deny_set_target_emits_lints_table() {
    let corpus = corpus_of(vec![make_deny_rule("anti-x", "unwrap_used")]);
    let sha = corpus_sha(&corpus);
    let artifacts = Target::DenySet.render(&corpus, &sha).expect("deny-set");
    let lints = artifacts
        .iter()
        .find(|a| a.rel_path.display().to_string() == "deny-set/lints.toml")
        .expect("lints.toml emitted");
    assert!(lints.contents.contains("[lints.clippy]"));
    assert!(lints.contents.contains("unwrap_used = \"deny\""));
}

#[test]
fn bundle_targets_both_txt_files() {
    let corpus = corpus_of(vec![make_rule("own-a", 0, "small")]);
    let sha = corpus_sha(&corpus);
    let artifacts = Target::Bundle.render(&corpus, &sha).expect("bundle");
    let names: Vec<String> = artifacts
        .iter()
        .map(|a| a.rel_path.display().to_string())
        .collect();
    assert!(names.contains(&"all.txt".to_string()));
    assert!(names.contains(&"core.txt".to_string()));
    assert!(artifacts[0].contents.contains(&sha));
}

#[test]
fn typescript_pack_renders_under_its_own_dir() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules");
    let corpus = wieland_core::load_pack(&root, wieland_core::Language::Typescript)
        .expect("typescript pack loads")
        .expect("typescript pack is not empty");
    let artifacts = wieland_core::render_all(&corpus).expect("typescript pack renders");
    assert!(
        artifacts
            .iter()
            .all(|a| a.rel_path.starts_with("typescript"))
    );
    let config = artifacts
        .iter()
        .find(|a| a.rel_path.ends_with("deny-set/oxlintrc.json"))
        .expect("oxlintrc.json emitted");
    assert!(config.contents.contains("\"correctness\": \"off\""));
    assert!(
        config
            .contents
            .contains("\"typescript/no-floating-promises\": \"error\"")
    );
    assert!(
        config
            .contents
            .contains("\"typescript/no-unsafe-type-assertion\": \"warn\"")
    );
    let skill = artifacts
        .iter()
        .find(|a| a.rel_path.ends_with("claude-code/SKILL.md"))
        .expect("skill emitted");
    assert!(skill.contents.contains("name: wieland-typescript"));
}
