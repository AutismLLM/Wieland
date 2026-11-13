---
id: proj-msrv-declare
title: Declare rust-version (MSRV) in Cargo.toml and gate it in CI
category: project-layout
severity: info
tier: 1
enforceability: judge
tags: [cargo, msrv, ci]
applies_to:
  - "a published library without package.rust-version"
  - "Cargo.toml [package] section"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-msrv-declare
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Declare `package.rust-version` (your MSRV) in `Cargo.toml` and add a CI job that builds on exactly that toolchain.

## Why
A declared MSRV makes Cargo emit a clear "requires rustc X" error on too-old toolchains instead of a cryptic failure deep in your code, and the edition-2024 resolver (`resolver = "3"`) uses it to avoid dependency versions whose own `rust-version` exceeds yours — preventing a transitive bump from silently raising your floor. Without one you have no contract with downstream users and no gate to catch regressions.

## Do
```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2024"
rust-version = "1.80"  # oldest toolchain you commit to supporting

[workspace]
resolver = "3"  # default for edition 2024; enables MSRV-aware resolution
```

## Don't
```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"
# no rust-version: users get cryptic errors on old toolchains,
# and a dep bump can silently raise the floor
```

## Notes
Pick the oldest stable toolchain your users realistically run, and do not go lower than needed — a lower MSRV forces you onto older dependency versions. Treat an MSRV bump as a semver-minor change for libraries; `cargo msrv` finds the real floor automatically.
