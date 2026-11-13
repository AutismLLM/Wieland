---
id: lint-cargo-metadata
title: Enable clippy::cargo so published crates carry complete package metadata
category: lints
severity: warn
tier: 2
enforceability: gate
tags: [clippy, cargo, publishing, metadata]
applies_to:
  - "Cargo.toml of a crate published to crates.io"
  - "[package] table missing description/license/repository"
  - "wildcard dependency versions"
rust_edition: "2024"
msrv: null
detect:
  clippy: [cargo_common_metadata, wildcard_dependencies, multiple_crate_versions]
  grep: ['version\s*=\s*"\*"']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#lint-cargo-metadata
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For any crate you publish to crates.io, enable the `clippy::cargo` lint group and give the package a description, a license, and a repository.

## Why
`clippy::cargo` reads `Cargo.toml` and flags the things that make a package unpublishable or fragile: missing `description`/`license`/`repository`, wildcard (`"*"`) dependency versions that break reproducibility, and the same crate resolving to multiple versions in the tree. Fixing these before you publish saves a failed `cargo publish` and gives downstream users a package that actually documents itself.

## Do
```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2024"
description = "A short description of what this crate does"
license = "MIT OR Apache-2.0"
repository = "https://github.com/user/my-crate"

[dependencies]
serde = "1.0"

[lints.clippy]
cargo = "warn"
```

## Don't
```toml
[package]
name = "my-crate"
version = "0.1.0"

[dependencies]
serde = "*"
```

## Notes
For internal or unpublished crates the metadata is noise; set `cargo = "allow"`, or keep the group on and selectively `allow` `multiple_crate_versions` when a duplicate version is genuinely unavoidable.
