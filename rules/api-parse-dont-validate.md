---
id: api-parse-dont-validate
title: Parse input into validated types at boundaries
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [api, correctness, types, newtype]
applies_to:
  - "repeated validation of the same raw value"
  - "functions taking &str they must re-check"
  - "system boundary parsing"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-parse-dont-validate
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
At each boundary, parse raw input into a type that can only hold valid data, then pass that type around instead of re-validating a primitive.

## Why
Validating-and-hoping scatters the same checks across the codebase and relies on every call site remembering to run them — miss one and invalid data flows through silently. Parsing into a newtype whose constructor is the only way in makes invalid states unrepresentable: once you hold an `Email`, the type system guarantees it is valid, so downstream functions need no checks at all. Validation happens once, at the edge.

## Do
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(s: impl Into<String>) -> Result<Self, EmailError> {
        let s = s.into();
        if s.contains('@') && s.len() > 3 {
            Ok(Email(s))
        } else {
            Err(EmailError::Invalid)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn send_email(email: &Email) -> Result<(), Error> {
    smtp_send(email.as_str())
}
```

## Don't
```rust
fn send_email(email: &str) -> Result<(), Error> {
    if !is_valid_email(email) {
        return Err(Error::InvalidEmail);
    }
    smtp_send(email)
}

fn add_to_mailing_list(email: &str) -> Result<(), Error> {
    if !is_valid_email(email) {
        return Err(Error::InvalidEmail);
    }
    list_store(email)
}
```

## Notes
Parse all inputs upfront at the system boundary (HTTP handler, CLI, config load), then work with the validated types internally. The same pattern gives you `Port(u16)`, `NonEmptyString`, `Percentage(u8)`, and other constrained newtypes. `TryFrom`/`FromStr` are idiomatic entry points, and `#[serde(try_from = "...")]` moves validation to the deserialization boundary.
