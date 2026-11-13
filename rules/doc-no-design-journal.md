---
id: doc-no-design-journal
title: Keep crate and module docs to end-state behavior, not the design journey
category: docs
severity: warn
tier: 1
enforceability: judge
tags: [rustdoc, docs, agents, maintenance]
applies_to:
  - "crate-level and module-level doc comments"
  - "agent-generated docs that narrate 'why we picked X over Y'"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-NO-META-DESIGN-DOCUMENTATION
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Document what a crate or module does today, for the reader who arrives cold. Do not narrate how it got there, which alternatives were rejected, or which internal guidelines a change followed.

## Why
Design narratives and self-report tables ("Applied M-SHORT-NAMES: yes, in the HTTP handlers") describe process, not behavior. They are stale the moment the next change lands, they mean nothing to a user who just wants to know what the crate does, and they bloat the doc a reader has to scan before finding the actual API. Enduring, user-relevant architectural facts (allocation-free, `#[no_std]`, an OSI-layered design) belong in a short "Design Principles" section of the README — everything about *why this PR chose X over Y* belongs in the commit message or PR description, not in `///` or `//!` comments.

## Do
```rust
//! HTTP client with automatic retry and connection pooling.
//!
//! This crate is allocation-free on the hot path and works under `#![no_std]`
//! with the `alloc` feature.

pub struct Client {}
```

## Don't
```rust
//! HTTP client with automatic retry and connection pooling.
//!
//! ## Design journal
//! We initially considered a callback-based API but switched to `async fn`
//! because it composed better with `?`. See discussion in PR #42.
//!
//! | Rule | Applied | Where |
//! | --- | :---: | --- |
//! | M-ASYNC-FN | done | migrated all I/O to async fn |

pub struct Client {}
```

## Notes
This applies to doc comments and README content that ships to users, not to CHANGELOG.md, ADRs, or PR descriptions, which are the correct home for rationale and process history.
