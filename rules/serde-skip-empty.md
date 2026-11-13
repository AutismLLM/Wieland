---
id: serde-skip-empty
title: Omit empty fields with serde(skip_serializing_if)
category: serde
severity: info
tier: 1
enforceability: judge
tags: [serde, wire-format, payload-size]
applies_to:
  - "Option or collection fields that serialize as null or []"
  - "an internal field that should never appear on the wire"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#serde-skip-empty
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Drop empty or absent fields from output with `#[serde(skip_serializing_if = "...")]`, and use `#[serde(skip)]` for fields that should never touch the wire in either direction.

## Why
Serializing `None` as `null` and empty collections as `[]` bloats payloads, clutters logs, and can confuse clients that distinguish a missing key from an explicit null. `skip_serializing_if` conditionally drops a field when its predicate returns true, keeping the wire format lean; `skip` excludes an internal field from both serialization and deserialization entirely.

## Do
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApiResponse {
    id: u64,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip)]
    cache_key: Option<String>,
}
```

## Don't
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    id: u64,
    name: String,
    description: Option<String>,
    tags: Vec<String>,
    error: Option<String>,
}
```

## Notes
`skip_serializing_if` takes any path resolving to `fn(&T) -> bool` — `Option::is_none`, `Vec::is_empty`, `String::is_empty`, or a custom predicate. A field marked `#[serde(skip)]` must have the type impl `Default` so deserialization can still construct the struct. Pair `skip_serializing_if` with `#[serde(default)]` so a missing key on the way in also yields the empty value rather than an error.
