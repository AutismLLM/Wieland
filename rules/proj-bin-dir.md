---
id: proj-bin-dir
title: Put multiple binaries in src/bin/ instead of hand-written [[bin]] targets
category: project-layout
severity: info
tier: 1
enforceability: judge
tags: [layout, cargo, binaries]
applies_to:
  - "a crate that ships more than one binary"
  - "manual [[bin]] sections in Cargo.toml"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-bin-dir
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a crate produces multiple binaries, place each entry point in `src/bin/` so Cargo discovers it automatically instead of declaring `[[bin]]` targets by hand.

## Why
Every file in `src/bin/` becomes a binary target named after the file with no `Cargo.toml` bookkeeping. Hand-written `[[bin]]` blocks are verbose, drift out of sync with the files on disk, and make it unclear whether a top-level `src/foo.rs` is a module or an entry point. Auto-discovery keeps shared code in `lib.rs` and each binary thin.

## Do
```rust
// src/lib.rs — shared code, reused by every binary
pub mod config;
pub mod database;

// src/bin/server.rs — auto-discovered as the `server` binary
use my_project::{config, database};

fn main() {
    let cfg = config::load();
    let _db = database::connect(&cfg);
}
```

## Don't
```toml
# Cargo.toml — verbose, drifts from the files on disk
[[bin]]
name = "server"
path = "src/server.rs"

[[bin]]
name = "cli"
path = "src/cli.rs"
```

## Notes
Keep explicit `[[bin]]` only when you need custom settings such as `required-features` or a name that differs from the file, or `default-run` to pick the default among several binaries.
