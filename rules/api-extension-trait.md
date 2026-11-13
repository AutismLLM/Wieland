---
id: api-extension-trait
title: Use extension traits to add methods to types you do not own
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [traits, orphan-rules, extension]
applies_to:
  - "wanting a method on an external type"
  - "inherent impl on a foreign type"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-extension-trait
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
To attach new methods to a type defined in another crate, declare an extension trait (conventionally named `TypeExt`) and implement it for that type, rather than reaching for an inherent impl the orphan rules forbid.

## Why
Rust's coherence rules bar an inherent impl on a foreign type and bar implementing a foreign trait for a foreign type. An extension trait is the sanctioned escape hatch: a local trait carrying your methods, implemented for the external type, brought into scope with a `use`. It is the pattern behind `itertools::Itertools`, `futures::StreamExt`, and `anyhow::Context`, and it keeps the added methods opt-in and discoverable.

## Do
```rust
pub trait ByteSliceExt {
    fn as_hex(&self) -> String;
}

impl ByteSliceExt for [u8] {
    fn as_hex(&self) -> String {
        self.iter().map(|b| format!("{b:02x}")).collect()
    }
}

let data: &[u8] = b"hi";
assert_eq!(data.as_hex(), "6869");
```

## Don't
```rust
impl [u8] {
    fn as_hex(&self) -> String {
        self.iter().map(|b| format!("{b:02x}")).collect()
    }
}
```
