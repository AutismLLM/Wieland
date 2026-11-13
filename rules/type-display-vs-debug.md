---
id: type-display-vs-debug
title: Use Display for user-facing output and Debug for diagnostics
category: types
severity: warn
tier: 2
enforceability: check
tags: [traits, formatting, errors]
applies_to:
  - "impl Display for a type"
  - "user-facing error messages"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['write!\(f, "\{:\?\}", self\)']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#type-display-vs-debug
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Derive `Debug` for diagnostics and hand-write `Display` for human-readable output; never route `Debug` to users or implement `Display` by delegating to `Debug`.

## Why
`Debug` (`{:?}`) targets developers: logs, panic messages, and test assertions. `Display` (`{}`) targets end users and is required by `std::error::Error` so error chains read naturally. Sending `Debug` output to users leaks internal field names, and a `Display` impl that just forwards to `Debug` throws away the point of the trait.

## Do
```rust
use std::fmt;

#[derive(Debug)]
struct ParseError {
    input: String,
    line: u32,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error on line {}: {:?}", self.line, self.input)
    }
}

impl std::error::Error for ParseError {}
```

## Don't
```rust
use std::fmt;

#[derive(Debug)]
struct ParseError {
    input: String,
    line: u32,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
```

## Notes
Derive `Debug` on every public type (API Guidelines C-DEBUG). The `{:#?}` pretty-print form is still `Debug` — use it in tests for readable assertions, not in user-facing code. `thiserror`'s `#[error("...")]` generates a correct `Display` for you.
