---
id: type-newtype-validated
title: Enforce validation at construction time with newtypes
category: types
severity: info
tier: 0
enforceability: judge
tags: [newtype, validation, parse-dont-validate]
applies_to:
  - "value validated repeatedly across call sites"
  - "domain value with an invariant (email, url, range)"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#type-newtype-validated
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Validate a value once inside a newtype's fallible constructor, so that holding the type is proof the invariant holds and no downstream code re-checks it.

## Why
Scattering `is_valid(...)` checks across every function that touches a raw `String` is error-prone and easy to forget. A validated newtype ("parse, don't validate") moves the check to the boundary: once you have an `Email`, it is guaranteed valid, so callers accept `Email` instead of `&str` and never re-validate.

## Do
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(s: &str) -> Result<Self, EmailError> {
        if is_valid_email(s) {
            Ok(Email(s.to_string()))
        } else {
            Err(EmailError::Invalid(s.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn send_email(to: &Email, body: &str) -> Result<(), Error> {
    send_to_address(to.as_str(), body)
}
```

## Don't
```rust
fn send_email(to: &str, body: &str) -> Result<(), Error> {
    if !is_valid_email(to) {
        return Err(Error::InvalidEmail);
    }
    send_to_address(to, body)
}

fn add_recipient(list: &mut Vec<String>, email: &str) -> Result<(), Error> {
    if !is_valid_email(email) {
        return Err(Error::InvalidEmail);
    }
    list.push(email.to_string());
    Ok(())
}
```

## Notes
Implement `Deserialize` by hand (or via `#[serde(try_from = "String")]`) so serde runs the same validation on the wire. The same pattern covers URLs, non-empty strings, positive numbers, and bounded ranges like a `Percentage(f64)`.
