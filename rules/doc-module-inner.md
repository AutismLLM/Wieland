---
id: doc-module-inner
title: Document modules and crates with inner `//!` comments
category: docs
severity: info
tier: 1
enforceability: judge
tags: [rustdoc, module, crate-root]
applies_to:
  - "top of lib.rs, mod.rs, or a single-file module"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#doc-module-inner
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use inner `//!` doc comments at the top of a module or crate root to describe the module itself, its contents, and how to use it.

## Why
`//!` documents the enclosing module rather than the next item, so it lands on the module's `cargo doc` landing page — the first thing users see when they navigate there. Plain `//` comments render nowhere, and a `///` above the first `use` documents that import, not the module. Inner docs orient the reader before they dig into individual items.

## Do
```rust
//! Authentication and authorization utilities.
//!
//! This module provides multiple authentication strategies:
//!
//! - [`JwtAuth`] — JSON Web Token based authentication
//! - [`SessionAuth`] — cookie-based session authentication
//!
//! # Examples
//!
//! ```
//! # use my_crate::auth::JwtAuth;
//! let auth = JwtAuth::new("secret-key");
//! ```

use std::collections::HashMap;

pub struct Session {}
```

## Don't
```rust
// This module handles authentication
// It provides JWT and session-based auth

use std::collections::HashMap;

pub struct Session {}
```
