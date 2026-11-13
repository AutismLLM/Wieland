---
id: doc-inline-reexport
title: Mark `pub use` re-exports of your own types with `#[doc(inline)]`
category: docs
severity: info
tier: 1
enforceability: judge
tags: [rustdoc, re-export, navigation]
applies_to:
  - "pub use of an internal module's item at the crate root"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-DOC-INLINE
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Annotate `pub use` re-exports of your crate's own items with `#[doc(inline)]` so they render as full entries alongside their siblings instead of an opaque "Re-exports" block.

## Why
Without `#[doc(inline)]`, `cargo doc` puts re-exported items under a collapsed "Re-exports" heading that just links elsewhere, hiding their docs, fields, and methods from the module's landing page. `#[doc(inline)]` tells rustdoc to inline the full item where the `pub use` is written, so users browsing the flat public API see it in place, like any other item.

## Do
```rust
mod client;

#[doc(inline)]
pub use client::HttpClient;

#[doc(inline)]
pub use client::config::*;
```

## Don't
```rust
mod client;

pub use client::HttpClient;
pub use client::config::*;
```

## Notes
Reserve this for re-exports of your own crate's items. Re-export `std` or third-party types without `#[doc(inline)]` so it stays visible to readers that the type is external, not part of your crate's own module tree.
