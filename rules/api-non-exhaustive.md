---
id: api-non-exhaustive
title: Mark evolvable public enums and structs #[non_exhaustive] for forward compatibility
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [semver, forward-compat, public-api]
applies_to:
  - "pub enum likely to gain variants"
  - "pub struct likely to gain fields"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-non-exhaustive
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Annotate public enums and structs that you expect to grow with `#[non_exhaustive]`, so adding a variant or field later is not a breaking change.

## Why
Adding a variant to a public enum breaks any downstream exhaustive `match`, and adding a public field breaks any downstream struct-literal construction — both are semver-major changes. `#[non_exhaustive]` requires external code to include a wildcard arm and to build the type through a constructor, which reserves the right to extend it in a minor release. This matters most for error enums and options structs that accrete cases over time.

## Do
```rust
#[non_exhaustive]
pub enum ErrorKind {
    NotFound,
    PermissionDenied,
    TimedOut,
}

fn describe(kind: &ErrorKind) -> &'static str {
    match kind {
        ErrorKind::NotFound => "not found",
        ErrorKind::PermissionDenied => "denied",
        ErrorKind::TimedOut => "timed out",
        _ => "unknown",
    }
}
```

## Don't
```rust
pub enum ErrorKind {
    NotFound,
    PermissionDenied,
    TimedOut,
}

fn describe(kind: &ErrorKind) -> &'static str {
    match kind {
        ErrorKind::NotFound => "not found",
        ErrorKind::PermissionDenied => "denied",
        ErrorKind::TimedOut => "timed out",
    }
}
```

## Notes
Do not apply it to non-public types (they have no external callers to protect) or to types that are complete by definition, such as an ordering enum with exactly `Less`, `Equal`, `Greater`.
