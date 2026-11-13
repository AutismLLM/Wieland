---
id: err-backtrace-predicates
title: Capture a Backtrace in error types and expose kinds via is_xxx predicates
category: errors
severity: info
tier: 1
enforceability: judge
tags: [error-handling, backtrace, api-design]
applies_to:
  - "error struct wrapping several internal failure modes"
  - "pub enum ErrorKind exposed directly on a public error type"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-ERRORS-CANONICAL-STRUCTS
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When one error type covers several internal failure modes, keep the enum private as an `ErrorKind` field on a public struct, expose `is_xxx()` predicate methods instead of the enum itself, and capture a `std::backtrace::Backtrace` when the error is constructed.

## Why
Exposing a public failure enum locks you to that exact variant set forever; every new variant becomes a breaking change for any caller who wrote an exhaustive `match`. A struct wrapping a private `ErrorKind`, with `is_xxx()` accessors, lets you add internal cases freely while still giving callers the discrimination they need. `Backtrace::capture()` is nearly free when `RUST_BACKTRACE` is unset — the runtime reads the env var and skips the walk — so capturing it by default costs little and pays off once an error needs tracing back through a deep call stack.

## Do
```rust
use std::backtrace::Backtrace;

#[derive(Debug)]
enum ErrorKind {
    Io(std::io::Error),
    Protocol,
}

#[derive(Debug)]
pub struct HttpError {
    kind: ErrorKind,
    backtrace: Backtrace,
}

impl HttpError {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind, backtrace: Backtrace::capture() }
    }

    pub fn is_io(&self) -> bool {
        matches!(self.kind, ErrorKind::Io(_))
    }

    pub fn is_protocol(&self) -> bool {
        matches!(self.kind, ErrorKind::Protocol)
    }
}
```

## Don't
```rust
#[derive(Debug)]
pub enum HttpError {
    Io(std::io::Error),
    Protocol,
}
```

## Notes
This targets error types spanning several unrelated failure modes; a small, focused error type with a handful of genuinely stable variants can still expose a plain `#[non_exhaustive]` enum. Reserve `Backtrace::capture()` for error construction sites, not every intermediate `?`.
