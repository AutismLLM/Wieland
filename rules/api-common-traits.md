---
id: api-common-traits
title: Derive the standard traits (Debug, Clone, PartialEq) on public types
category: api-design
severity: warn
tier: 2
enforceability: check
tags: [traits, derive, interoperability]
applies_to:
  - "pub struct"
  - "pub enum"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['pub struct \w', 'pub enum \w']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-common-traits
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Derive the standard traits — at minimum `Debug`, and `Clone`/`PartialEq` where meaningful — on public types so they interoperate with the rest of the ecosystem.

## Why
Standard traits are the shared vocabulary of the Rust ecosystem. `Debug` enables `{:?}` formatting, assertion messages, and logging; `Clone` allows explicit duplication; `PartialEq` enables `==` and `assert_eq!`; `Eq + Hash` lets a type be a map key. Omitting them forces downstream users into workarounds and blocks common patterns like testing and collections.

## Do
```rust
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(u64);

let mut users: HashMap<UserId, Point> = HashMap::new();
users.insert(UserId(1), Point { x: 0.0, y: 0.0 });
```

## Don't
```rust
pub struct Point {
    pub x: f64,
    pub y: f64,
}

let a = Point { x: 1.0, y: 2.0 };
println!("{a:?}");
```

## Notes
Skip a derive when it is genuinely wrong: no `Eq`/`Ord` on floating-point fields, no `Copy` on types that own heap data or implement `Drop`, and a hand-written `Debug` for types holding secrets so they are not logged in plaintext.
