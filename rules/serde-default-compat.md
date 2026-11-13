---
id: serde-default-compat
title: Use serde(default) for optional and backward-compatible fields
category: serde
severity: info
tier: 1
enforceability: judge
tags: [serde, compatibility, defaults]
applies_to:
  - "a newly added struct field that older payloads lack"
  - "an all-optional config struct"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#serde-default-compat
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Mark fields (or the whole container) with `#[serde(default)]` so payloads missing those keys fill from `Default` instead of failing deserialization.

## Why
Without `#[serde(default)]`, any field missing from an incoming payload causes deserialization to fail with a "missing field" error. When you add fields to a struct over time, older payloads that predate them break. `#[serde(default)]` fills missing keys from the type's `Default` (or a named function), enabling graceful forward compatibility.

## Do
```rust
use serde::{Serialize, Deserialize};

fn default_retries() -> u32 { 3 }

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    host: String,
    port: u16,
    #[serde(default)]
    timeout_secs: u64,
    #[serde(default = "default_retries")]
    retries: u32,
    #[serde(default)]
    tls_cert_path: Option<String>,
}
```

## Don't
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    host: String,
    port: u16,
    timeout_secs: u64,
    retries: u32,
}
```

## Notes
`#[serde(default = "path")]` calls a `fn() -> T` when `Default` would give the wrong value (e.g. `30` not `0`). Container-level `#[serde(default)]` requires the struct to impl `Default` and makes every field optional — which can hide field-name typos, so prefer field-level annotation when only some fields are backward-compatible additions. `default` affects deserialization only; pair with `skip_serializing_if` to omit defaults on the way out.
