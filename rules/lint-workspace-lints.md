---
id: lint-workspace-lints
title: Configure lints once at the workspace root and inherit them in members
category: lints
severity: info
tier: 1
enforceability: judge
tags: [workspace, clippy, configuration, consistency]
applies_to:
  - "multi-crate workspace Cargo.toml"
  - "member crates with divergent [lints] tables"
rust_edition: "2024"
msrv: "1.74"
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#lint-workspace-lints
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Define your lint policy once in `[workspace.lints]` at the root and have every member crate inherit it with `[lints] workspace = true`, instead of letting each crate set its own standards.

## Why
Without a central policy each crate drifts to its own rules or none, so a lint denied in one crate slips through in another and CI enforcement becomes a lottery. Workspace lints (stable since 1.74) put one source of truth at the root; members opt in with two lines and get identical enforcement. Individual crates can still override a single lint where the context genuinely differs, but they start from the shared baseline.

## Do
```toml
[workspace.lints.clippy]
correctness = { level = "deny", priority = -1 }
suspicious = { level = "deny", priority = -1 }
unwrap_used = "deny"

[workspace.lints.rustdoc]
broken_intra_doc_links = "deny"
```
```toml
[lints]
workspace = true
```

## Don't
```toml
[lints.clippy]
unwrap_used = "deny"
```
```toml
[lints.clippy]
unwrap_used = "warn"
```

## Notes
Category-level entries need `priority = -1` so specific per-lint overrides win. A member can still relax one lint locally — a binary crate can `allow` `unwrap_used`, a test-utils crate can `allow` `print_stdout` — while inheriting everything else.
