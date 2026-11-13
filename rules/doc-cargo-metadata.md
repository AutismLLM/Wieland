---
id: doc-cargo-metadata
title: Fill Cargo.toml metadata before publishing a crate
category: docs
severity: warn
tier: 2
enforceability: gate
tags: [cargo, crates-io, metadata]
applies_to:
  - "[package] section of a published crate's Cargo.toml"
rust_edition: "2024"
msrv: null
detect:
  clippy: [cargo_common_metadata]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#doc-cargo-metadata
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Populate `description`, `license`, `repository`, and the other common `[package]` fields before publishing to crates.io.

## Why
This metadata is what crates.io shows in search results and on the crate page; users judge trust and relevance from it. A bare package looks abandoned and is hard to find. `clippy::cargo_common_metadata` flags the missing fields so you catch them before `cargo publish`.

## Do
```toml
[package]
name = "my-awesome-crate"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
description = "A fast, ergonomic HTTP client for Rust"
license = "MIT OR Apache-2.0"
repository = "https://github.com/username/my-awesome-crate"
documentation = "https://docs.rs/my-awesome-crate"
readme = "README.md"
keywords = ["http", "client", "async", "networking"]
categories = ["network-programming", "web-programming::http-client"]
```

## Don't
```toml
[package]
name = "my-awesome-crate"
version = "0.1.0"
edition = "2024"
```

## Notes
Keep `keywords` to five specific, searchable terms; avoid filler like `rust`, `library`, or `fast`. Use `cargo publish --dry-run` and `cargo package --list` to verify what ships.
