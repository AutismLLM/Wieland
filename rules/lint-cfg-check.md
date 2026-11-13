---
id: lint-cfg-check
title: Declare custom cfgs so unexpected_cfgs catches feature-gate typos
category: lints
severity: warn
tier: 2
enforceability: check
tags: [cfg, features, compile-time, typo]
applies_to:
  - "#[cfg(feature = ...)] attributes"
  - "custom cfg names like tokio_unstable"
  - "conditionally compiled impls and functions"
rust_edition: "2024"
msrv: "1.80"
detect:
  clippy: []
  grep: ['#\[cfg\(feature\s*=', '#\[cfg\([a-z_]+\)\]']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#lint-cfg-check
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Keep `unexpected_cfgs` enabled and declare every custom cfg via `check-cfg` so a mistyped or undeclared cfg becomes a compile-time warning instead of silently dead code.

## Why
A typo like `#[cfg(feature = "serde_")]` or a custom `#[cfg(tokio_unstable)]` with no declaration compiles with no error and produces a block that is silently never enabled. The `unexpected_cfgs` lint (stable since 1.80) flags any cfg name or value the compiler does not know about. Cargo registers your feature names automatically; custom cfgs must be listed in `check-cfg`, and that one line converts an invisible bug into a warning.

## Do
```rust
#[cfg(feature = "serde")]
impl serde::Serialize for MyType {}

#[cfg(tokio_unstable)]
pub fn experimental() {}
```
```toml
[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = [
    'cfg(tokio_unstable)',
] }
```

## Don't
```rust
#[cfg(feature = "serde_")]
impl serde::Serialize for MyType {}

#[cfg(tokio_unstable)]
pub fn experimental() {}
```

## Notes
Feature names (`feature = "serde"`) are registered by Cargo automatically and never need a `check-cfg` entry. `check-cfg` values must be quoted cfg expressions: `'cfg(name)'` or `'cfg(name, values("v1", "v2"))'`. Promote the lint to `deny` in CI if you want hard failures.
