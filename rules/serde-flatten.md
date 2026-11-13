---
id: serde-flatten
title: Inline nested structs or capture extra keys with serde(flatten)
category: serde
severity: info
tier: 1
enforceability: judge
tags: [serde, composition, wire-format]
applies_to:
  - "structs duplicating a shared set of fields (pagination, audit, envelope)"
  - "a catch-all bucket for unknown or dynamic keys"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#serde-flatten
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `#[serde(flatten)]` to merge a shared nested struct into the parent's flat wire form, or to collect unmatched keys into a map, instead of copy-pasting the same fields into every struct.

## Why
APIs frequently share a common set of fields across message types (pagination metadata, audit timestamps, error envelopes). Duplicating those fields in every struct is fragile. `#[serde(flatten)]` merges a nested struct's fields into the parent's wire representation, keeping the JSON flat while the Rust stays modular; a flattened `HashMap` field additionally captures all unknown keys.

## Do
```rust
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Pagination {
    page: u32,
    per_page: u32,
    total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserListResponse {
    users: Vec<String>,
    #[serde(flatten)]
    pagination: Pagination,
}

#[derive(Serialize, Deserialize, Debug)]
struct FlexibleConfig {
    name: String,
    version: u32,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}
```

## Don't
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct UserListResponse {
    users: Vec<String>,
    page: u32,
    per_page: u32,
    total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostListResponse {
    posts: Vec<String>,
    page: u32,
    per_page: u32,
    total: u64,
}
```

## Notes
`#[serde(flatten)]` is incompatible with `#[serde(deny_unknown_fields)]` on the same struct, since flatten must forward unrecognized keys. It uses a slower content-buffering path — avoid it on extremely hot deserialization paths — and does not work with non-self-describing formats like `bincode`.
