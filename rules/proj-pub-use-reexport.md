---
id: proj-pub-use-reexport
title: Re-export with pub use to give a flat public API over nested modules
category: project-layout
severity: info
tier: 1
enforceability: judge
tags: [api, re-export, ergonomics]
applies_to:
  - "a library exposing deep module paths to users"
  - "public modules that force long import paths"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-pub-use-reexport
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Keep internal modules private and surface the public API with `pub use` re-exports at the crate root, so users import from one flat namespace.

## Why
`pub use` lifts items from submodules to the current level, giving a flat, ergonomic API while your internal organization stays free to change. Exposing deep module paths directly forces long imports like `my_crate::client::http::HttpClient` and pins your internal layout into the public contract, so any reorganization breaks downstream code.

## Do
```rust
// src/lib.rs — flat public API over private internals
mod error;
mod config;
mod client;

pub use client::http::HttpClient;
pub use config::Config;
pub use error::MyError;

// downstream user:
use my_crate::{Config, HttpClient, MyError};
```

## Don't
```rust
// src/lib.rs — deep paths leak the internal layout
pub mod error;
pub mod config;
pub mod client;

// downstream user is forced to write:
use my_crate::client::http::HttpClient;
use my_crate::config::Config;
use my_crate::error::MyError;
```

## Notes
Re-export selectively so implementation details stay hidden, and use `pub use path::Item as Alias` to rename on the way out (for example a legacy client). Re-exporting external types users need (`pub use bytes::Bytes;`) spares them a direct dependency; glob re-exports of foreign crates are usually too broad.
