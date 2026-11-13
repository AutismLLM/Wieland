---
id: serde-rename-all
title: Match the external naming convention with serde(rename_all)
category: serde
severity: warn
tier: 2
enforceability: check
tags: [serde, naming, wire-format]
applies_to:
  - "a struct or enum with repeated per-field #[serde(rename = ...)]"
  - "Rust snake_case fields serialized to camelCase/kebab-case APIs"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['#\[serde\(rename = "']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#serde-rename-all
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Set the wire naming convention once with a container-level `#[serde(rename_all = "...")]` instead of renaming each field individually.

## Why
Rust fields are `snake_case` by convention, but JSON APIs, GraphQL, and config formats often use `camelCase`, `kebab-case`, or `SCREAMING_SNAKE_CASE`. Renaming every field with its own `#[serde(rename = "...")]` is noisy and error-prone. A single `#[serde(rename_all = "camelCase")]` on the container keeps Rust idiomatic and the wire format correct in one declaration, with per-field `rename` reserved for genuine exceptions.

## Do
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserProfile {
    first_name: String,
    last_name: String,
    email_address: String,
    is_active: bool,
    #[serde(rename = "type")]
    user_type: String,
}
```

## Don't
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct UserProfile {
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "emailAddress")]
    email_address: String,
    #[serde(rename = "isActive")]
    is_active: bool,
}
```

## Notes
`rename_all` applies to both directions and, on enums, to variant names. A field-level `#[serde(rename = "...")]` always wins over the container setting, so keep it for reserved words like `type` or one-off mismatches — the grep heuristic flags clusters of per-field renames, not the legitimate lone exception.
