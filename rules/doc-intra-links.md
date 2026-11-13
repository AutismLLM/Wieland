---
id: doc-intra-links
title: Reference types and items with intra-doc links, not plain text
category: docs
severity: info
tier: 1
enforceability: judge
tags: [rustdoc, links, navigation]
applies_to:
  - "doc comment mentioning another type, method, or module"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#doc-intra-links
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Write references to other items as intra-doc links (`[`Type`]`, `[`Self::method`]`) instead of plain text.

## Why
Intra-doc links render as clickable navigation and are verified at doc-build time, so a rename or removal turns into a warning instead of a dead reference. Plain-text mentions become stale and unclickable. Building docs with `RUSTDOCFLAGS="-D warnings"` then fails CI on any broken link.

## Do
```rust
/// Returns the length of the buffer.
///
/// See also [`capacity()`](Self::capacity) for the allocated size,
/// and [`Buffer`] for more details.
pub fn len(&self) -> usize {
    self.data.len()
}
```

## Don't
```rust
/// Returns the length of the buffer.
///
/// See also `capacity()` for the allocated size, and the
/// `Buffer` struct for more details.
pub fn len(&self) -> usize {
    self.data.len()
}
```

## Notes
When a name is ambiguous, disambiguate with a prefix: `[`Error`](struct@Error)` vs `[`Error`](trait@Error)`.
