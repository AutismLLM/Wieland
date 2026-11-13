---
id: err-source-chain
title: Preserve error chains with a #[source] field or source() method
category: errors
severity: warn
tier: 1
enforceability: judge
tags: [error-handling, thiserror, source-chain]
applies_to:
  - ".map_err(|e| MyError::Variant(e.to_string()))"
  - "a custom error type that wraps an underlying cause"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#err-source-chain
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When wrapping an underlying error, keep the original as a `#[source]` field (or implement `source()`) instead of flattening it into a string.

## Why
`std::error::Error::source()` is how logging frameworks and error reporters walk the full cause chain: "config parse failed → JSON syntax error at line 5 → unexpected token". Collapsing the cause into `e.to_string()` discards that chain and the underlying error's structured data, leaving you with a single lossy message and no way to inspect or downcast the root cause.

## Do
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("failed to read config file '{path}'")]
    ReadFailed {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse config file '{path}'")]
    ParseFailed {
        path: String,
        #[source]
        source: serde_json::Error,
    },
}

fn load_config(path: &str) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path).map_err(|source| ConfigError::ReadFailed {
        path: path.to_string(),
        source,
    })?;
    serde_json::from_str(&content).map_err(|source| ConfigError::ParseFailed {
        path: path.to_string(),
        source,
    })
}
```

## Don't
```rust
#[derive(Debug)]
enum ConfigError {
    ParseFailed(String),
}

fn load_config(path: &str) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| ConfigError::ParseFailed(e.to_string()))?;
    serde_json::from_str(&content).map_err(|e| ConfigError::ParseFailed(e.to_string()))
}
```

## Notes
`#[from]` implies `#[source]` and also generates the `From` impl, so use it when no extra context is needed; use a bare `#[source]` field when the variant also carries context like a path. With `anyhow`, `.context()` builds the same chain, viewable via `{:?}` or `err.chain()`.
