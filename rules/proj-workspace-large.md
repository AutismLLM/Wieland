---
id: proj-workspace-large
title: Use a Cargo workspace for large multi-crate projects
category: project-layout
severity: info
tier: 1
enforceability: judge
tags: [cargo, workspace, layout]
applies_to:
  - "several related crates split across separate repositories"
  - "a growing project with multiple internal libraries"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-workspace-large
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Group multiple related crates under one Cargo workspace instead of separate repositories, sharing a single lock file, build cache, and dependency set.

## Why
A workspace gives related crates one `Cargo.lock`, a shared build cache, and synchronized dependency versions, so builds are faster and incremental and cross-crate changes happen in one place. Splitting the same crates across repositories duplicates lock files, lets versions drift, and makes coordinated development painful.

## Do
```toml
# Root Cargo.toml — virtual workspace
[workspace]
resolver = "3"  # default for edition 2024; use "2" for 2021
members = ["crates/core", "crates/cli", "crates/server", "crates/common"]

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }

[workspace.lints.rust]
unsafe_code = "forbid"
```

## Don't
```toml
# four separate repos, each with its own Cargo.lock and drifting versions:
#   my-app-core/Cargo.toml
#   my-app-cli/Cargo.toml
#   my-app-server/Cargo.toml
#   my-app-common/Cargo.toml
[package]
name = "my-app-core"
version = "0.1.0"
edition = "2024"
```

## Notes
A single binary or library needs no workspace. Reach for one when you have multiple related crates, shared internal libraries, a mono-repo of services, or a plugin architecture. Member crates inherit shared settings with `version.workspace = true` and `[lints] workspace = true`.
