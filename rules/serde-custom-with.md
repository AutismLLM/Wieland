---
id: serde-custom-with
title: Customize field (de)serialization with with / serialize_with / deserialize_with
category: serde
severity: info
tier: 1
enforceability: judge
tags: [serde, wire-format, conversion]
applies_to:
  - "a field whose Rust type differs from its wire representation"
  - "Duration/timestamp/bytes serialized as seconds/string/base64"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#serde-custom-with
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a field's natural Rust type differs from its wire form, point serde at conversion functions with `#[serde(with = "module")]` (or `serialize_with`/`deserialize_with`) instead of changing the field type.

## Why
Some types have a natural Rust representation that differs from what the wire format expects: a `Duration` stored as whole seconds, raw bytes as base64, a timestamp as an ISO-8601 string. Changing the field type just to satisfy serde pollutes the domain model. The `with` attribute keeps the field as `Duration` while custom functions handle the encoding, and one-sided variants let you override only the direction you need.

## Do
```rust
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::time::Duration;

mod duration_secs {
    use super::*;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    #[serde(with = "duration_secs")]
    timeout: Duration,
}
```

## Don't
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    timeout_secs: u64,
}
```

## Notes
The `with` module functions must match serde's exact signatures — `serialize` receives `&T`, not `T`. Crates like `time`, `chrono`, and `uuid` ship ready-made `with` modules. For a representation used everywhere, a newtype with its own `Serialize`/`Deserialize` impl is often cleaner than repeating `#[serde(with = "...")]`.
