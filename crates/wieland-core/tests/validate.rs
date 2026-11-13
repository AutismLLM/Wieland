use std::path::{Path, PathBuf};

use wieland_core::corpus::Corpus;
use wieland_core::{parse_rule, validate_corpus};

const VALID: &str = r#"---
id: own-example
title: Borrow instead of cloning when a read-only reference is enough
category: ownership
severity: warn
tier: 0
enforceability: judge
tags: [ownership]
applies_to:
  - ".clone()"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#own-borrow-over-clone
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Borrow when you only need to read.

## Why
Cloning copies the whole value; a reference does not.

## Do
```rust
fn total(items: &[u32]) -> u32 {
    items.iter().sum()
}
```

## Don't
```rust
fn total(items: Vec<u32>) -> u32 {
    items.iter().sum()
}
```
"#;

fn rule_from(text: &str) -> wieland_core::Rule {
    parse_rule(Path::new("fixture.md"), text).expect("fixture frontmatter parses")
}

fn findings_for(text: &str) -> Vec<String> {
    let corpus = Corpus {
        root: PathBuf::from("fixture"),
        rules: vec![rule_from(text)],
    };
    validate_corpus(&corpus)
        .into_iter()
        .map(|f| f.message)
        .collect()
}

fn has(messages: &[String], needle: &str) -> bool {
    messages.iter().any(|m| m.contains(needle))
}

#[test]
fn valid_rule_has_no_findings() {
    assert!(findings_for(VALID).is_empty(), "{:?}", findings_for(VALID));
}

#[test]
fn missing_do_section_is_a_finding() {
    let text = VALID.replace(
        "## Do\n```rust\nfn total(items: &[u32]) -> u32 {\n    items.iter().sum()\n}\n```\n\n",
        "",
    );
    assert!(has(&findings_for(&text), "missing required `## Do`"));
}

#[test]
fn unknown_category_is_a_finding() {
    let text = VALID.replace("category: ownership", "category: nonsense");
    assert!(has(&findings_for(&text), "unknown category"));
}

#[test]
fn id_prefix_must_match_category() {
    let text = VALID.replace("id: own-example", "id: mem-example");
    assert!(has(&findings_for(&text), "must start with `own-`"));
}

#[test]
fn gate_without_detector_is_a_finding() {
    let text = VALID.replace("enforceability: judge", "enforceability: gate");
    assert!(has(
        &findings_for(&text),
        "enforceability=gate requires a detect.clippy lint"
    ));
}

#[test]
fn tier_two_without_detector_is_a_finding() {
    let text = VALID
        .replace("tier: 0", "tier: 2")
        .replace("enforceability: judge", "enforceability: check");
    assert!(has(
        &findings_for(&text),
        "tier 2/3 requires a non-empty detect"
    ));
}

#[test]
fn bad_status_is_a_finding() {
    let text = VALID.replace("status: stable", "status: shipped");
    assert!(has(&findings_for(&text), "status `shipped`"));
}

#[test]
fn bad_edition_is_a_finding() {
    let text = VALID.replace(r#"rust_edition: "2024""#, r#"rust_edition: "2020""#);
    assert!(has(&findings_for(&text), "rust_edition `2020`"));
}

#[test]
fn empty_sources_is_a_finding() {
    let text = VALID.replace(
        "sources:\n  - leonardomso/rust-skills#own-borrow-over-clone",
        "sources: []",
    );
    assert!(has(&findings_for(&text), "at least one provenance"));
}

#[test]
fn overlong_title_is_a_finding() {
    let long = "x".repeat(120);
    let text = VALID.replace(
        "title: Borrow instead of cloning when a read-only reference is enough",
        &format!("title: {long}"),
    );
    assert!(has(&findings_for(&text), "title exceeds 90 chars"));
}

#[test]
fn uppercase_id_is_a_finding() {
    let text = VALID.replace("id: own-example", "id: own-Example");
    assert!(has(&findings_for(&text), "must match"));
}

#[test]
fn duplicate_id_is_a_finding() {
    let corpus = Corpus {
        root: PathBuf::from("fixture"),
        rules: vec![rule_from(VALID), rule_from(VALID)],
    };
    let messages: Vec<String> = validate_corpus(&corpus)
        .into_iter()
        .map(|f| f.message)
        .collect();
    assert!(has(&messages, "duplicate id"));
}

#[test]
fn missing_frontmatter_fence_is_a_parse_error() {
    assert!(parse_rule(Path::new("f.md"), "## Rule\nno frontmatter here\n").is_err());
}

#[test]
fn unterminated_frontmatter_is_a_parse_error() {
    assert!(parse_rule(Path::new("f.md"), "---\nid: own-x\ntitle: hi\n").is_err());
}

#[test]
fn unknown_frontmatter_field_is_a_parse_error() {
    let text = VALID.replace("status: stable", "status: stable\nbogus_field: 1");
    assert!(parse_rule(Path::new("f.md"), &text).is_err());
}

#[test]
fn bad_severity_is_a_parse_error() {
    let text = VALID.replace("severity: warn", "severity: critical");
    assert!(parse_rule(Path::new("f.md"), &text).is_err());
}

#[test]
fn out_of_range_tier_is_a_parse_error() {
    let text = VALID.replace("tier: 0", "tier: 7");
    assert!(parse_rule(Path::new("f.md"), &text).is_err());
}

#[test]
fn missing_required_field_is_a_parse_error() {
    let text = VALID.replace(
        "title: Borrow instead of cloning when a read-only reference is enough\n",
        "",
    );
    assert!(parse_rule(Path::new("f.md"), &text).is_err());
}

const VALID_TS: &str = r#"---
id: async-example
title: Await or explicitly handle every promise
category: async
language: typescript
ts_version: "7.0"
severity: deny
tier: 3
enforceability: gate
detect:
  oxlint: [typescript/no-floating-promises]
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Await every promise.

## Why
A floating promise drops its rejection.

## Do
```ts
export async function run(save: () => Promise<void>): Promise<void> {
  await save();
}
```

## Don't
```ts
export function run(save: () => Promise<void>): void {
  save();
}
```
"#;

const VALID_GO: &str = r#"---
id: err-example
title: Check every returned error
category: errors
language: go
go_version: "1.26"
severity: deny
tier: 3
enforceability: gate
detect:
  golangci: [errcheck]
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Check every error.

## Why
A dropped error hides a failure.

## Do
```go
if err := f.Close(); err != nil {
	return err
}
```

## Don't
```go
f.Close()
```
"#;

#[test]
fn valid_typescript_rule_has_no_findings() {
    assert!(
        findings_for(VALID_TS).is_empty(),
        "{:?}",
        findings_for(VALID_TS)
    );
}

#[test]
fn valid_go_rule_has_no_findings() {
    assert!(
        findings_for(VALID_GO).is_empty(),
        "{:?}",
        findings_for(VALID_GO)
    );
}

#[test]
fn typescript_rule_without_ts_version_is_a_finding() {
    let text = VALID_TS.replace("ts_version: \"7.0\"\n", "");
    assert!(has(&findings_for(&text), "ts_version is required"));
}

#[test]
fn malformed_ts_version_is_a_finding() {
    let text = VALID_TS.replace("ts_version: \"7.0\"", "ts_version: \"seven\"");
    assert!(has(&findings_for(&text), "must look like MAJOR.MINOR"));
}

#[test]
fn go_rule_without_go_version_is_a_finding() {
    let text = VALID_GO.replace("go_version: \"1.26\"\n", "");
    assert!(has(&findings_for(&text), "go_version is required"));
}

#[test]
fn typescript_gate_without_its_detector_is_a_finding() {
    let text = VALID_TS.replace(
        "  oxlint: [typescript/no-floating-promises]\n",
        "  grep: [x]\n",
    );
    assert!(has(
        &findings_for(&text),
        "requires detect.oxlint or detect.tsconfig for a typescript rule"
    ));
}

#[test]
fn typescript_gate_on_tsconfig_flag_is_valid() {
    let text = VALID_TS.replace(
        "  oxlint: [typescript/no-floating-promises]",
        "  tsconfig: [strict]",
    );
    assert!(findings_for(&text).is_empty(), "{:?}", findings_for(&text));
}

#[test]
fn typescript_rule_with_clippy_is_a_finding() {
    let text = VALID_TS.replace(
        "  oxlint: [typescript/no-floating-promises]",
        "  oxlint: [typescript/no-floating-promises]\n  clippy: [unwrap_used]",
    );
    assert!(has(
        &findings_for(&text),
        "a typescript rule may not carry detect.clippy"
    ));
}

#[test]
fn rust_rule_with_oxlint_is_a_finding() {
    let text = VALID.replace(
        "  clippy: []",
        "  clippy: []\n  oxlint: [typescript/no-explicit-any]",
    );
    assert!(has(
        &findings_for(&text),
        "a rust rule may not carry detect.oxlint"
    ));
}

#[test]
fn go_gate_without_golangci_is_a_finding() {
    let text = VALID_GO.replace("  golangci: [errcheck]", "  grep: [x]");
    assert!(has(
        &findings_for(&text),
        "requires detect.golangci for a go rule"
    ));
}

#[test]
fn rust_category_on_typescript_rule_is_a_finding() {
    let text = VALID_TS
        .replace("category: async", "category: ownership")
        .replace("id: async-example", "id: own-example");
    assert!(has(
        &findings_for(&text),
        "unknown category `ownership` (not in taxonomy/categories-typescript.yaml)"
    ));
}

#[test]
fn rust_rule_with_ts_version_is_a_finding() {
    let text = VALID.replace("status: stable", "status: stable\nts_version: \"7.0\"");
    assert!(has(
        &findings_for(&text),
        "a Rust rule may not carry ts_version"
    ));
}

#[test]
fn unknown_language_is_a_parse_error() {
    let text = VALID_TS.replace("language: typescript", "language: python");
    assert!(parse_rule(Path::new("f.md"), &text).is_err());
}
