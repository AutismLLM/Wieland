---
id: doc-crate-readme
title: Make README.md the single source for crate-root docs
category: docs
severity: info
tier: 1
enforceability: judge
tags: [rustdoc, readme, crates-io]
applies_to:
  - "crate-level documentation in lib.rs"
  - "README.md of a published crate"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#doc-crate-readme
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Pull the crate-root documentation from `README.md` with `#![doc = include_str!("../README.md")]` instead of maintaining a separate `//!` copy.

## Why
A hand-written crate-level doc comment and a `README.md` inevitably drift: one gets updated for GitHub readers, the other for docs.rs. `include_str!` makes the README the single source for GitHub, crates.io, and docs.rs at once. Set `readme = "README.md"` in `Cargo.toml` so all three surfaces render the same file.

## Do
```rust
#![doc = include_str!("../README.md")]

pub fn do_thing() {}
```

## Don't
```rust
//! # my-crate
//!
//! A library for doing things. (duplicate, will get out of date)
//!
//! ## Usage
//! ...

pub fn do_thing() {}
```

## Notes
If the README has non-Rust code blocks, tag them (` ```bash `, ` ```text `, ` ```rust,no_run `) so rustdoc does not try to compile them as doc-tests.
