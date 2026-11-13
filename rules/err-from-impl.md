---
id: err-from-impl
title: Implement From for error conversions to enable the ? operator
category: errors
severity: info
tier: 1
enforceability: judge
tags: [error-handling, thiserror, conversions]
applies_to:
  - "repeated .map_err() wrapping a source error into a variant"
  - "a custom error enum wrapping other error types"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#err-from-impl
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Give your error type a `From<SourceError>` impl (via `#[from]` on the variant) so `?` converts source errors automatically instead of hand-written `.map_err()` at every call.

## Why
The `?` operator calls `From::from` on the error before returning it. Implementing `From<SourceError> for YourError` lets propagation read as a bare `?`, removing the repeated `.map_err(|e| AppError::Io(e))` boilerplate and keeping wrapping consistent across the codebase. `thiserror`'s `#[from]` generates both the `From` impl and the `source()` link for you.

## Do
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

fn load_config(path: &str) -> Result<Config, AppError> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}
```

## Don't
```rust
fn load_config(path: &str) -> Result<Config, AppError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| AppError::Io(e))?;
    let config: Config = serde_json::from_str(&content)
        .map_err(|e| AppError::Parse(e))?;
    Ok(config)
}
```

## Notes
When conversion needs extra context (the path, an id), you cannot use `#[from]` — keep an explicit `.map_err()` that fills a struct variant with a `#[source]` field, or add `anyhow` context. Avoid a blanket `impl<E: Error> From<E>`; it conflicts with other `From` impls and erases variant structure.
