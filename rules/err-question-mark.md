---
id: err-question-mark
title: Use the ? operator for error propagation instead of manual match
category: errors
severity: warn
tier: 0
enforceability: gate
tags: [error-handling, propagation, idiom]
applies_to:
  - "match on a Result that returns Err in one arm"
  - "if let Some/Ok that early-returns the other case"
rust_edition: "2024"
msrv: null
detect:
  clippy: [question_mark]
  grep: []
  ast_query: null
autofix: true
sources:
  - leonardomso/rust-skills#err-question-mark
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Propagate errors with the `?` operator rather than a verbose `match`/`if let` that returns `Err` in one arm.

## Why
`?` is Rust's idiomatic propagation: it unwraps the `Ok`/`Some` value, early-returns the error after applying `From::from`, and collapses a five-line `match` into one expression. The manual form is longer, easy to get subtly wrong, and hides the happy path. `clippy::question_mark` flags the mechanical cases and can rewrite them automatically.

## Do
```rust
fn load_config() -> Result<Config, Error> {
    let content = std::fs::read_to_string("config.toml")?;
    let config = toml::from_str(&content)?;
    Ok(config)
}
```

## Don't
```rust
fn load_config() -> Result<Config, Error> {
    let content = match std::fs::read_to_string("config.toml") {
        Ok(c) => c,
        Err(e) => return Err(Error::Io(e)),
    };
    let config = match toml::from_str(&content) {
        Ok(c) => c,
        Err(e) => return Err(Error::Parse(e)),
    };
    Ok(config)
}
```

## Notes
`?` works on `Option` too (`text.lines().next()?`) and converts `Option` to `Result` via `.ok_or_else(...)`. Keep the explicit `match` only when the arms do more than propagate — for example, logging, retrying, or mapping to a different value.
