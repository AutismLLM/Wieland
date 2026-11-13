---
id: proj-flat-small
title: Keep small projects flat and add structure only when complexity demands it
category: project-layout
severity: info
tier: 1
enforceability: judge
tags: [layout, modules, simplicity]
applies_to:
  - "a project with fewer than ~10 source files"
  - "nested module folders that only re-export"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-flat-small
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Keep small projects flat: put files directly in `src/` and introduce nested module folders only once file count or file size actually forces the issue.

## Why
Over-organizing a handful of files adds navigation overhead with no benefit — deep folders, `mod.rs` files that only re-export, and empty layers all cost reading time while conveying nothing. Start flat and let structure grow with real complexity: group by feature around 10-20 files, add submodule folders past 20.

## Do
```rust
// src/lib.rs — a flat, ~6-file project
mod config;
mod database;
mod user;
mod error;

pub use config::Config;
pub use error::Error;
```

## Don't
```rust
// src/lib.rs — deep layers for a project this small
pub mod domain {
    pub mod models {
        pub mod user;
    }
}
pub mod infrastructure {
    pub mod database {
        pub mod connection;
    }
}
```

## Notes
Signs you have earned more structure: files exceeding 300-500 lines, `_`-prefixed grouping in file names (`user_model.rs`, `user_service.rs`), or related files that are hard to find. Until then, flat wins.
