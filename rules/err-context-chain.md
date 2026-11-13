---
id: err-context-chain
title: Add context to propagated errors with context or with_context
category: errors
severity: info
tier: 1
enforceability: judge
tags: [error-handling, anyhow, context]
applies_to:
  - "the ? operator on an io or parse error"
  - "std::fs and serde calls that can fail"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#err-context-chain
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Attach a describing message to errors as you propagate them with `.context()` or `.with_context()`, so the final error tells what operation failed and why.

## Why
A raw `std::io::Error` reads as "No such file or directory (os error 2)" with no hint about which file or what you were doing. Wrapping each fallible step in context builds a cause chain — "failed to parse user 42 JSON: expected ':' at line 5" — that turns an opaque failure into a diagnosis. Use `with_context` (lazy closure) when the message interpolates runtime data so the allocation only happens on the error path.

## Do
```rust
use anyhow::{Context, Result};

fn load_user(id: u64) -> Result<User> {
    let path = format!("users/{id}.json");
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read user file: {path}"))?;
    let user: User = serde_json::from_str(&content)
        .with_context(|| format!("failed to parse user {id} JSON"))?;
    Ok(user)
}
```

## Don't
```rust
fn load_user(id: u64) -> Result<User, anyhow::Error> {
    let path = format!("users/{id}.json");
    let content = std::fs::read_to_string(&path)?;
    let user: User = serde_json::from_str(&content)?;
    Ok(user)
}
```

## Notes
Prefer `context()` for static strings and `with_context(|| ...)` when the message includes formatted runtime data or is expensive to build. In typed-error code, the equivalent is a `#[source]` field carrying the underlying cause.
