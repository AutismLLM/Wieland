---
id: api-dont-leak-types
title: Prefer std types over third-party crate types in your public API
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [api, dependencies, stability, semver]
applies_to:
  - "public fn signature taking or returning a third-party crate's type"
  - "public struct field typed with a dependency's type"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-DONT-LEAK-TYPES
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Where you can avoid it, do not expose a third-party crate's type in your own public API. Any type that appears in a public signature becomes part of your stability contract, and only `std`/`core` ship the permanent stability guarantee needed to back that contract.

## Why
A type from `some_dep` in your public signature forces every downstream user to depend on `some_dep` too, and pins the exact version range they must accept. If you later swap `some_dep` for an alternative, or its own public types change shape, that break ripples straight into your users' code even though your own logic did not change. `std` types carry no such risk: they ship with the compiler and their stability is guaranteed for as long as Rust exists.

## Do
```rust
use std::time::Duration;

pub struct RetryConfig {
    pub backoff: Duration,
    pub max_attempts: u32,
}

pub fn connect(timeout: Duration) -> std::io::Result<Connection> {
    // internally may use a third-party client, but the public
    // signature only exposes std types.
    todo!()
}
pub struct Connection;
```

## Don't
```rust
// Forces every caller to add `other_crate` as a direct dependency
// and track its version in lockstep with yours.
pub fn connect(timeout: other_crate::Duration) -> other_crate::Result<Connection> {
    todo!()
}
pub struct Connection;
```

## Notes
Some leakage is unavoidable or even beneficial: umbrella crates may freely leak types from their own sibling crates, a type may be leaked behind a clearly named feature flag (e.g. `serde`), and leaking without a feature flag can be justified when it gives a substantial benefit such as interoperability with a de facto standard crate in the ecosystem. Treat every other case as a deliberate, reviewed exception rather than a default.
