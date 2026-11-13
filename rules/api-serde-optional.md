---
id: api-serde-optional
title: Gate serde support behind a feature flag rather than a hard dependency
category: api-design
severity: warn
tier: 2
enforceability: check
tags: [serde, features, dependencies]
applies_to:
  - "library crate Cargo.toml"
  - "derive(Serialize, Deserialize) on public types"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['^serde\s*=\s*"', 'serde\s*=\s*\{[^}]*\}\s*$']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-serde-optional
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
In a general-purpose library, make `serde` an optional dependency behind a feature and gate the derives with `cfg_attr`, so users who do not serialize pay nothing.

## Why
A required `serde` dependency forces its compile time and code size onto every downstream user, including those who never serialize a single type. Marking it `optional = true` and exposing a `serde` feature lets consumers opt in only when they need it, which follows Rust's minimal-dependency philosophy. The `#[cfg_attr(feature = "serde", derive(...))]` form applies the derives only when the feature is active.

## Do
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Config {
    pub name: String,
    pub value: i32,
}
```

## Don't
```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub value: i32,
}
```

## Notes
Make serde a required dependency when serialization is the crate's whole purpose — a config-file parser, a data-format library, or an API client — where every user needs it anyway.
