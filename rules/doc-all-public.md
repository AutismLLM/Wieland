---
id: doc-all-public
title: Document every public item with a `///` doc comment
category: docs
severity: warn
tier: 2
enforceability: gate
tags: [rustdoc, api, public]
applies_to:
  - "pub fn"
  - "pub struct"
  - "pub enum"
  - "pub trait"
rust_edition: "2024"
msrv: null
detect:
  clippy: [missing_docs]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#doc-all-public
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Give every public item a `///` doc comment describing what it is and how it is used.

## Why
Public items are your crate's API contract. Undocumented, they force users to read source to learn behavior, raising support burden and slowing adoption. `cargo doc` renders these comments into the primary reference — but only for items you actually document. Enabling `missing_docs` turns the gap into a compiler warning.

## Do
```rust
#![warn(missing_docs)]

/// Configuration for establishing a connection to the service.
pub struct Config {
    /// Maximum time to wait for a response before timing out.
    pub timeout: Duration,
    /// Number of retry attempts for failed requests.
    pub retries: u32,
}

/// Establishes a connection using the provided configuration.
///
/// # Errors
///
/// Returns an error if the connection cannot be established.
pub fn connect(config: Config) -> Result<Connection, Error> {
    todo!()
}
```

## Don't
```rust
pub struct Config {
    pub timeout: Duration,
    pub retries: u32,
}

pub fn connect(config: Config) -> Result<Connection, Error> {
    todo!()
}
```
