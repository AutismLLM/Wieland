---
id: err-custom-type
title: Define custom error types for domain-specific failures
category: errors
severity: info
tier: 1
enforceability: judge
tags: [error-handling, thiserror, api-design]
applies_to:
  - "Result<T, String> returned from a library function"
  - "a function with several distinct failure modes"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#err-custom-type
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Model domain-specific failures as a custom error enum whose variants name each failure mode, instead of returning `String` or a catch-all error.

## Why
`Result<(), String>` forces callers into fragile string matching and documents nothing about what can go wrong. A typed enum encodes each failure mode in the type system, lets callers `match` on the exact case to recover, and carries structured data (the offending value, limits) for display and handling. It makes the API self-documenting and the failure set exhaustive.

## Do
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("name cannot be empty")]
    EmptyName,
    #[error("name exceeds maximum length of {max} characters")]
    NameTooLong { max: usize, actual: usize },
    #[error("invalid age {0}: must be between 0 and 150")]
    InvalidAge(u8),
}

fn validate_user(user: &User) -> Result<(), ValidationError> {
    if user.name.is_empty() {
        return Err(ValidationError::EmptyName);
    }
    if user.name.len() > 100 {
        return Err(ValidationError::NameTooLong { max: 100, actual: user.name.len() });
    }
    if user.age > 150 {
        return Err(ValidationError::InvalidAge(user.age));
    }
    Ok(())
}
```

## Don't
```rust
fn validate_user(user: &User) -> Result<(), String> {
    if user.name.is_empty() {
        return Err("Name is empty".to_string());
    }
    if user.age > 150 {
        return Err("Age is invalid".to_string());
    }
    Ok(())
}
```

## Notes
For applications and prototypes that only report errors, `anyhow::Error` is fine — reach for a custom type when callers must distinguish failures. Add `#[non_exhaustive]` to public error enums so new variants are not a breaking change.
