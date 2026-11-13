---
id: err-thiserror-lib
title: Use thiserror to define typed error enums in libraries
category: errors
severity: info
tier: 1
enforceability: judge
tags: [errors, api, ergonomics]
applies_to:
  - "library returning Result"
  - "error type returned by public API"
  - "manual std::error::Error impl"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#err-thiserror-lib
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Define library error types as `thiserror`-derived enums so callers get typed, matchable errors instead of strings or `Box<dyn Error>`.

## Why
A library's callers need to handle specific failure modes — retry a timeout, report a syntax error's line, propagate the rest. `String` and `Box<dyn Error>` erase that structure, forcing callers to match on message text or give up on discrimination. `thiserror` generates the `Display` and `Error` (including `source()`) impls from attributes, giving you an ergonomic, matchable enum with `#[from]` conversions for free and none of the hand-written boilerplate.

## Do
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid syntax at line {line}: {message}")]
    Syntax { line: usize, message: String },
    #[error("unexpected end of file")]
    UnexpectedEof,
    #[error("io error reading input")]
    Io(#[from] std::io::Error),
}

fn parse(input: &str) -> Result<Ast, ParseError> {
    if input.is_empty() {
        return Err(ParseError::UnexpectedEof);
    }
    todo!()
}
```

## Don't
```rust
fn parse(input: &str) -> Result<Ast, String> {
    Err("parse error".to_string())
}

fn load(path: &Path) -> Result<Data, Box<dyn std::error::Error>> {
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "file not found",
    )))
}
```

## Notes
Use `thiserror` for the public API of libraries where callers match on variants; reach for `anyhow` in application code and internal glue where an opaque, context-carrying error is more ergonomic. `#[error(transparent)]` and `#[source]` let you preserve the underlying error chain.
