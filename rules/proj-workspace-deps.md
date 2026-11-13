---
id: proj-workspace-deps
title: Inherit shared dependencies from [workspace.dependencies] to stop drift
category: project-layout
severity: info
tier: 1
enforceability: judge
tags: [cargo, workspace, dependencies]
applies_to:
  - "member crates pinning the same dependency to different versions"
  - "a multi-crate workspace"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-workspace-deps
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Declare shared dependencies once under `[workspace.dependencies]` and inherit them in member crates with `dep.workspace = true`.

## Why
Without inheritance, member crates drift onto different versions of the same dependency, which bloats the binary with duplicate copies, lengthens compile times, and can introduce subtle behavior differences. Declaring each dependency once at the workspace root (Rust 1.64+) and inheriting it keeps every crate on one version.

## Do
```toml
# Root Cargo.toml
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.32", features = ["full"] }

# crates/core/Cargo.toml
[dependencies]
serde.workspace = true
tokio = { workspace = true, features = ["net"] }  # adds to workspace features
```

## Don't
```toml
# crate-a/Cargo.toml
[dependencies]
serde = "1.0.150"
tokio = "1.25"

# crate-b/Cargo.toml — silently drifted to different versions
[dependencies]
serde = "1.0.188"
tokio = "1.32"
```

## Notes
A member can add features on top of the inherited set with `{ workspace = true, features = [...] }`. Optional must be set on the member (`{ workspace = true, optional = true }`), not in `[workspace.dependencies]`. The same inheritance works for `[dev-dependencies]`, `[build-dependencies]`, and internal path crates.
