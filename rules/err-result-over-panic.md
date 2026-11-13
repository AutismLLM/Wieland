---
id: err-result-over-panic
title: Return Result for recoverable errors instead of panicking
category: errors
severity: warn
tier: 0
enforceability: judge
tags: [correctness, api, errors]
applies_to:
  - "panic! on recoverable input"
  - "fn returning a bare value that can fail"
  - "library public API"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#err-result-over-panic
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For any error a caller could reasonably recover from, return `Result<T, E>` rather than panicking.

## Why
A panic unwinds the stack and crashes the thread or process; from the caller's side it is unrecoverable. Returning `Result<T, E>` hands the caller the choice — retry, fall back, propagate, or log. Libraries should almost never panic on their inputs, and applications should reserve panics for truly unrecoverable states such as violated internal invariants.

## Do
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid config format: {0}")]
    Parse(#[from] serde_json::Error),
}

fn parse_config(path: &str) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
}
```

## Don't
```rust
fn parse_config(path: &str) -> Config {
    let content = std::fs::read_to_string(path)
        .expect("Failed to read config");
    serde_json::from_str(&content)
        .expect("Invalid config format")
}
```

## Notes
Panicking is appropriate for genuine bugs: a violated internal invariant, an unreachable state (`unreachable!()`), an unimplemented path (`unimplemented!()`), or setup in `main` that cannot meaningfully continue. Tests and quick prototypes may `unwrap` freely.
