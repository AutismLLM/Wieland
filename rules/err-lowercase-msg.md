---
id: err-lowercase-msg
title: Start error messages lowercase with no trailing punctuation
category: errors
severity: info
tier: 2
enforceability: check
tags: [error-handling, style, messages]
applies_to:
  - "#[error(\"...\")] attributes on error variants"
  - "context strings passed to .context()"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['#\[error\("[A-Z][a-z]', '#\[error\("[^"]*[.!]"\)']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#err-lowercase-msg
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Write error messages starting lowercase and without trailing punctuation, so they compose cleanly when chained or logged.

## Why
Error messages are routinely concatenated into cause chains and log lines. Lowercase, punctuation-free fragments join naturally — "failed to load config: invalid JSON: unexpected token" — while capitalized, period-terminated strings produce "Failed to load config.: Invalid JSON.: Unexpected token." The standard library follows this convention ("entity not found", "invalid digit found in string"), so matching it keeps your errors consistent with `std`.

## Do
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("failed to read config file")]
    ReadFailed(#[from] std::io::Error),
    #[error("invalid JSON format")]
    ParseFailed(#[from] serde_json::Error),
    #[error("key not found: {0}")]
    KeyNotFound(String),
}
```

## Don't
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("Failed to read config file.")]
    ReadFailed(#[from] std::io::Error),
    #[error("Invalid JSON format!")]
    ParseFailed(#[from] serde_json::Error),
    #[error("The requested key was not found")]
    KeyNotFound(String),
}
```

## Notes
Proper nouns and acronyms keep their case — `"invalid JSON syntax"`, `"OAuth token expired"`, `"HTTP request failed"`. Error codes may be uppercase (`"error code E0001: invalid input"`). The rule is about the leading word and trailing character, not interior casing.
