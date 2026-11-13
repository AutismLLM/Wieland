---
id: proj-crates-flat-folder
title: Keep all workspace crates as siblings in one flat folder
category: project-layout
severity: warn
tier: 1
enforceability: judge
tags: [cargo, workspace, layout, crates]
applies_to:
  - "a repo with a workspace Cargo.toml"
  - "adding a new crate to an existing workspace"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-CRATES-FLAT-FOLDER
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Put every crate the workspace produces as a direct sibling under one top-level directory (e.g. `crates/`). Never nest one crate's `Cargo.toml` inside another crate's folder or its `src/`; express a relationship between crates with a shared name prefix instead.

## Why
A flat sibling layout means every crate is one hop from the workspace root and `cargo new crates/foo` always works the same way — there is nothing to search for. Nesting a crate inside another crate's `src/` breaks that: it confuses `src/` (module tree) with the crate boundary (compilation unit), makes path dependencies fragile, and forces readers to hunt through implementation folders to find out how many crates actually exist.

## Do
```bash
Cargo.toml
crates/
  foo/Cargo.toml
  foo_core/Cargo.toml
  foo_proc/Cargo.toml
  bar/Cargo.toml
```

## Don't
```bash
Cargo.toml
crates/
  foo/Cargo.toml
    src/lib.rs
       deps/bar/Cargo.toml   # a crate hiding inside another crate's src/
```

## Notes
Past a dozen or two crates, add one layer of grouping folders (`server/`, `client/`, `common/`) rather than deepening the tree further. A crate embedding a dummy UI-test crate for its own tooling is the rare acceptable exception to "never nested".
