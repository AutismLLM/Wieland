---
id: serde-deny-unknown-fields
title: Reject unexpected keys with #[serde(deny_unknown_fields)]
category: serde
severity: warn
tier: 1
enforceability: check
tags: [correctness, serde, config, api]
applies_to:
  - "#[derive(Deserialize)] on a config or DTO struct"
  - "deserialized public API contract"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: "struct with #[derive(Deserialize)] lacking #[serde(deny_unknown_fields)]"
autofix: false
sources:
  - leonardomso/rust-skills#serde-deny-unknown-fields
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Add `#[serde(deny_unknown_fields)]` to config-file structs and API DTOs so unrecognized keys become hard errors.

## Why
By default serde silently discards any input key that does not match a struct field. For config files and strict API contracts that is dangerous: a typo like `"timout_secs"` deserializes without error while the intended field stays at its default, producing a silent misconfiguration. `deny_unknown_fields` turns the unexpected key into an error that names the offending field, surfacing the mistake immediately at the boundary.

## Do
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct ServerConfig {
    host: String,
    port: u16,
    timeout_secs: u64,
}

fn parse_config(json: &str) -> Result<ServerConfig, serde_json::Error> {
    serde_json::from_str(json)
}
```

## Don't
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    timeout_secs: u64,
}
```

## Notes
Skip it on deliberately extensible structs where callers pass extra metadata; use `#[serde(flatten)]` with a `HashMap` catch-all instead. `deny_unknown_fields` is incompatible with `#[serde(flatten)]` on the same struct — `flatten` needs to forward unmatched keys, which `deny_unknown_fields` intercepts. If you need both, split the struct or deserialize into a `serde_json::Value` first.
