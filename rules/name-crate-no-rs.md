---
id: name-crate-no-rs
title: Don't suffix crate names with -rs or -rust
category: naming
severity: info
tier: 1
enforceability: judge
tags: [naming, crates, cargo]
applies_to:
  - "package name in Cargo.toml"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-crate-no-rs
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Name a crate for what it does, not the language it is written in — drop redundant `-rs`, `-rust`, and `rust-` affixes.

## Why
Everything on crates.io is Rust, so the suffix carries no information; it only lengthens the name and clutters the namespace. The ecosystem's most-used crates (`serde`, `tokio`, `reqwest`, `clap`) all omit it, and following suit keeps names short and typable.

## Do
```toml
[package]
name = "json-parser"
version = "0.1.0"
edition = "2024"
```

## Don't
```toml
[package]
name = "json-parser-rs"
version = "0.1.0"
edition = "2024"
```

## Notes
A language token is fine when it names the target rather than the crate's language, e.g. `windows-sys` (platform) or `python-ast` (a port's domain).
