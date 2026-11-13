---
id: serde-try-from-validate
title: Validate while deserializing with serde(try_from)
category: serde
severity: warn
tier: 1
enforceability: judge
tags: [serde, validation, parse-dont-validate]
applies_to:
  - "a newtype whose invariant is checked after deserialization"
  - "domain types that must never hold an invalid value"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#serde-try-from-validate
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Route deserialization through `#[serde(try_from = "Raw")]` so validation runs during parsing and an invalid value is never constructed, instead of validating after the fact.

## Why
Validating after deserialization means an invalid value can exist in memory, even briefly, and the check is easy to forget. Parse-don't-validate says to make invalid states unrepresentable. `#[serde(try_from = "Raw")]` wires the deserializer through a `TryFrom` conversion: serde reads the raw type, then your conversion either produces the validated value or returns a deserialization error.

## Do
```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
struct ValidatedEmail(String);

impl TryFrom<String> for ValidatedEmail {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.contains('@') && !s.starts_with('@') && !s.ends_with('@') {
            Ok(ValidatedEmail(s))
        } else {
            Err(format!("invalid email address: {s}"))
        }
    }
}

impl From<ValidatedEmail> for String {
    fn from(e: ValidatedEmail) -> String {
        e.0
    }
}
```

## Don't
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Email(String);

fn process(json: &str) -> Result<(), Box<dyn std::error::Error>> {
    let email: Email = serde_json::from_str(json)?;
    println!("{email:?}");
    Ok(())
}
```

## Notes
`#[serde(into = "Raw")]` handles serialization by calling `Raw::from(value)`, and requires the type to impl `Clone` since serde may clone before converting. The `try_from`/`into` attributes replace the field-by-field derived impl entirely. Define the `TryFrom` logic once and reuse it across serde and non-serde paths such as CLI parsing and form validation.
