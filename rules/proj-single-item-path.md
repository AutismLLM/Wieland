---
id: proj-single-item-path
title: Expose each public item through exactly one path
category: project-layout
severity: warn
tier: 1
enforceability: judge
tags: [api, re-export, refactoring, modules]
applies_to:
  - "a public item reachable both via its defining module and a crate-root re-export"
  - "leftover re-exports from a previous module layout"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-SINGLE-ITEM-PATH
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
A public item should be reachable through exactly one path. If you re-export it at a shorter path, make the defining module private (or `pub(crate)`) so the longer path stops being part of the public API.

## Why
When both `crate::db::Connection` and `crate::Connection` resolve to the same type, users, docs, and search results end up with two "correct" answers and no signal for which one is canonical. This duplication is often left behind after an iterative refactor — a re-export gets added for ergonomics, but the original module stays `pub` instead of being tightened, so old and new import paths both keep working indefinitely and rustdoc lists the type twice.

## Do
```rust
mod db {
    pub struct Connection;
}

pub use db::Connection;
```

## Don't
```rust
pub mod db {
    pub struct Connection;
}

pub use db::Connection;
```

## Notes
This targets user-facing duplication only. Multiple internal references to the same item while building an export tree (`pub(crate) mod db`) are fine and often unavoidable. Re-exports of foreign crate items and `_private` modules required by macro hygiene are also exempt — the rule is about not giving your own users two different public addresses for the same item.
