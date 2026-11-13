---
id: proj-prelude-module
title: Offer a conservative prelude module for the crate's common imports
category: project-layout
severity: info
tier: 1
enforceability: judge
tags: [api, re-export, ergonomics]
applies_to:
  - "a library whose users import many items individually"
  - "the crate's public API surface"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-prelude-module
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Provide a `prelude` module that glob-re-exports the handful of types and traits users almost always need, following the `std::prelude` pattern.

## Why
When the common workflow needs many imports, a prelude lets users write `use my_crate::prelude::*` once instead of listing every type. Kept conservative — core types, key traits, the error type — it stays ergonomic without polluting the namespace; kept stable, it avoids breaking users, since removing an item is a breaking change.

## Do
```rust
// src/lib.rs
pub mod prelude {
    pub use crate::{Client, Config, Error, Request, Response};
    pub use crate::traits::{Handler, Middleware};
    pub use crate::types::Method;
}

// downstream user:
use my_crate::prelude::*;
```

## Don't
```rust
// forcing every user to hand-assemble the same import block
use my_crate::Client;
use my_crate::Config;
use my_crate::Error;
use my_crate::Request;
use my_crate::Response;
use my_crate::traits::{Handler, Middleware};
use my_crate::types::Method;
```

## Notes
Be conservative: include only truly common items, avoid names that clash on glob import, and document the contents in module docs. Feature-gated preludes and a small core plus a `full_prelude` for power users are both fine patterns.
