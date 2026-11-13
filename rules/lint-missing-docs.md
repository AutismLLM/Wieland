---
id: lint-missing-docs
title: Warn on missing_docs so every public API item is documented
category: lints
severity: info
tier: 2
enforceability: check
tags: [docs, public-api, libraries]
applies_to:
  - "pub struct / pub fn / pub trait / pub mod in a library crate"
  - "lib.rs crate attributes"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['pub\s+(struct|enum|trait|fn)\s', 'pub\s+mod\s']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#lint-missing-docs
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Enable the `missing_docs` lint so every public item in a library carries a doc comment.

## Why
For a library, the documentation is the user interface: an undocumented `pub` item forces users to read your source to learn what it does. `missing_docs` fires only on public items, so it maps exactly onto your API surface and turns "we should document this" into a warning you cannot forget.

## Do
```rust
#![warn(missing_docs)]

//! User management module.

/// A registered user in the system.
pub struct User {
    /// The user's display name.
    pub name: String,
    /// The user's age in years.
    pub age: u32,
}

/// Processes pending user requests.
pub fn process() {}
```

## Don't
```rust
#![warn(missing_docs)]

pub struct User {
    pub name: String,
    pub age: u32,
}

pub fn process() {}
```

## Notes
`missing_docs` ignores private items, so it never nags about internals. For existing codebases start at `warn` and fix incrementally, then promote to `deny` once clean. A genuinely internal-but-public module can carry `#[allow(missing_docs)]`.
