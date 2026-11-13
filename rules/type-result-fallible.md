---
id: type-result-fallible
title: Return Result for operations that can fail
category: types
severity: warn
tier: 1
enforceability: judge
tags: [result, errors, fallibility]
applies_to:
  - "function that can fail but returns Option or a sentinel"
  - "function that unwraps or panics on recoverable errors"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#type-result-fallible
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Return `Result<T, E>` from operations that can fail, so callers must acknowledge the error and cannot silently ignore it.

## Why
`Result<T, E>` makes failure explicit and carries context about what went wrong, unlike `Option` (which discards the cause), a panic (which crashes the caller), or a magic sentinel like `-1` (which is easy to miss). The `?` operator keeps propagation ergonomic while every failure stays visible in the type.

## Do
```rust
fn read_config(path: &str) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)
        .map_err(ConfigError::Io)?;
    toml::from_str(&content)
        .map_err(ConfigError::Parse)
}

fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    Ok(a / b)
}
```

## Don't
```rust
fn read_config(path: &str) -> Option<Config> {
    let content = std::fs::read_to_string(path).ok()?;
    toml::from_str(&content).ok()
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        return -1;
    }
    a / b
}
```

## Notes
Define error enums with `thiserror` and `#[from]` so the `?` operator converts source errors automatically. Combinators (`map`, `map_err`, `and_then`, `unwrap_or`) cover the cases where a full `match` is overkill.
