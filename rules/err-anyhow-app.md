---
id: err-anyhow-app
title: Use anyhow for application error handling
category: errors
severity: info
tier: 1
enforceability: judge
tags: [error-handling, anyhow, applications]
applies_to:
  - "binary crate main and application code"
  - "Result<T, Box<dyn std::error::Error>> in an app"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#err-anyhow-app
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
In application and CLI code, use `anyhow::Result` with `.context()` for error handling instead of hand-rolled typed errors or `Box<dyn Error>`.

## Why
Applications usually do not pattern-match on error variants — they report what went wrong with enough context to debug it. `anyhow` gives you a single error type that converts from any `std::error::Error`, chains context, and captures backtraces, removing the boilerplate of managing per-source enums. Reserve typed errors (`thiserror`) for library public APIs where callers must match on failures.

## Do
```rust
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    let path = find_config().context("failed to locate config file")?;
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read config from {}", path.display()))?;
    let config: Config = toml::from_str(&content).context("failed to parse config as TOML")?;
    validate(&config).context("config validation failed")?;
    Ok(config)
}
```

## Don't
```rust
fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = find_config()?;
    let content = std::fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&content)?;
    validate(&config)?;
    Ok(config)
}
```

## Notes
Use `thiserror` instead when the code is a library whose callers need to match on specific error variants. You can combine both: define typed errors in the library, handle them with `anyhow` in the application, and `downcast_ref` when you occasionally need a concrete variant.
