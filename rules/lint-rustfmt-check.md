---
id: lint-rustfmt-check
title: Run cargo fmt --check in CI to keep formatting off the review table
category: lints
severity: info
tier: 1
enforceability: judge
tags: [rustfmt, ci, formatting, process]
applies_to:
  - "CI pipeline configuration"
  - "pre-commit hooks"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: true
sources:
  - leonardomso/rust-skills#lint-rustfmt-check
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Gate CI on `cargo fmt --all --check` so unformatted code fails before merge, not after.

## Why
Consistent formatting removes an entire class of review comments and keeps diffs about behavior, not whitespace. `cargo fmt --all --check` fails without modifying files, so it is safe to run in CI and turns "please run rustfmt" into an automatic gate. A shared `rustfmt.toml` makes the format a project decision rather than an editor setting.

## Do
```bash
cargo fmt --all --check
```
```toml
edition = "2024"
max_width = 100
group_imports = "StdExternalCrate"
imports_granularity = "Module"
```

## Don't
```bash
cargo build --release
cargo test
```

## Notes
Locally, `cargo fmt --all` applies the fixes; the `--check` form is for CI and pre-commit hooks. Mark genuinely hand-aligned code (lookup tables, matrices) with `#[rustfmt::skip]` rather than disabling the gate.
