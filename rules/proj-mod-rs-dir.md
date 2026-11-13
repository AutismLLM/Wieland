---
id: proj-mod-rs-dir
title: Pick one multi-file module style and apply it consistently
category: project-layout
severity: info
tier: 1
enforceability: judge
tags: [layout, modules, consistency]
applies_to:
  - "a crate mixing user.rs and user/mod.rs module styles"
  - "multi-file modules"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-mod-rs-dir
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Choose one style for multi-file modules — `foo/mod.rs` or an adjacent `foo.rs` beside a `foo/` folder — and use it consistently across the crate; prefer the `mod.rs` style for larger module trees.

## Why
Rust supports both `user/mod.rs` and `user.rs` + `user/` for the same module, and mixing them within one crate makes the layout harder to scan. The `mod.rs` form keeps a module's root inside its own folder, which reads clearly for larger modules and is common in big codebases; the adjacent form keeps the interface visible from outside the folder and suits small modules.

## Do
```rust
// src/database/mod.rs — mod.rs style, module root lives with its files
mod connection;
mod migrations;
mod queries;
mod error;

pub use connection::Pool;
pub use error::DatabaseError;
pub use queries::{UserQueries, OrderQueries};
```

## Don't
```rust
// src/lib.rs — same crate, two styles fighting each other
mod user;      // backed by src/user.rs + src/user/ (adjacent style)
mod database;  // backed by src/database/mod.rs (mod.rs style)
```

## Notes
You can pin a choice with a Cargo lint: `clippy::mod_module_files` enforces the `mod.rs` style, `clippy::self_named_module_files` enforces the adjacent style.
