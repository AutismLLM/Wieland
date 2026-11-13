---
id: test-snapshot-testing
title: Snapshot-test large or serialized output with insta
category: testing
severity: info
tier: 1
enforceability: judge
tags: [snapshot-testing, insta, serialization]
applies_to:
  - "tests asserting on multi-line or structured output"
  - "rendered errors, JSON/YAML serialization, CLI output"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-snapshot-testing
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For large structured output — pretty-printed structs, serialized JSON/YAML, rendered errors, CLI text — assert with `insta` snapshots instead of hand-written `assert_eq!` on a literal.

## Why
Inline expected strings for multi-line output are verbose, brittle, and painful to update: one intentional format change means editing an escaped literal by hand. `insta` records an approved snapshot, diffs against it on later runs, and lets you accept legitimate changes with `cargo insta review`. Snapshots are committed and reviewed in the PR, so output changes become visible and deliberate.

## Do
```rust
use insta::{assert_debug_snapshot, assert_json_snapshot};

#[test]
fn renders_not_found_error() {
    let err = AppError::NotFound { id: 42 };
    assert_debug_snapshot!(err);
}

#[test]
fn serializes_default_config() {
    let config = Config::default();
    assert_json_snapshot!(config);
}
```

## Don't
```rust
#[test]
fn serializes_default_config() {
    let config = Config::default();
    let json = serde_json::to_string_pretty(&config).unwrap();
    assert_eq!(json, "{\n  \"timeout\": 30,\n  \"retries\": 3\n}");
}
```

## Notes
Keep `assert_eq!` for short, simple values (`true`, `42`, `"ok"`); snapshots earn their keep only once the output is large or structured.
