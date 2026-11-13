---
id: serde-enum-representation
title: Choose enum tagging deliberately across external, internal, adjacent, untagged
category: serde
severity: info
tier: 1
enforceability: judge
tags: [serde, enum, wire-format]
applies_to:
  - "an enum that (de)serializes to an external API or config schema"
  - "an enum with a discriminator field on the wire"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#serde-enum-representation
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Pick an enum tagging strategy — externally (default), internally, adjacently tagged, or untagged — to match the wire schema you must produce, rather than accepting serde's default by accident.

## Why
Serde's default externally tagged form wraps every variant in an object keyed by variant name, which clashes with APIs, event systems, or config schemas that expect a discriminator field. The wrong tagging strategy silently mismatches your wire format against the expected schema, causing parse failures or round-trip data loss.

## Do
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ShapeInternal {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
enum ShapeAdjacent {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Count(u32),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
}
```

## Don't
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}
```

## Notes
Untagged deserializes by trying each variant in declaration order — slower, can silently pick the wrong variant, and gives generic errors; reserve it for small, structurally distinct sets. Internally tagged cannot represent tuple or primitive-wrapping newtype variants (all variants must serialize as maps); use adjacently tagged for those.
