---
id: api-default-impl
title: Implement Default for types that have a sensible default value
category: api-design
severity: warn
tier: 2
enforceability: gate
tags: [default, traits, ergonomics]
applies_to:
  - "fn new() -> Self with no arguments"
  - "type with a canonical empty/zero value"
rust_edition: "2024"
msrv: null
detect:
  clippy: [new_without_default]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-default-impl
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a type has a canonical default value, implement `Default` (derive it when every field's default is right, hand-write it otherwise) rather than exposing only a bespoke `new()`.

## Why
`Default` is the ecosystem's canonical "empty instance" hook. It unlocks `unwrap_or_default()`, `..Default::default()` struct-update syntax, generic bounds `T: Default`, and `#[derive(Default)]` on containing types. A type with an argument-free `new()` but no `Default` cannot participate in any of those patterns; clippy's `new_without_default` flags exactly this gap.

## Do
```rust
use std::time::Duration;

pub struct Config {
    timeout: Duration,
    retries: u32,
    verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            timeout: Duration::from_secs(30),
            retries: 3,
            verbose: false,
        }
    }
}

let config = Config { retries: 5, ..Config::default() };
```

## Don't
```rust
use std::time::Duration;

pub struct Config {
    timeout: Duration,
    retries: u32,
    verbose: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            timeout: Duration::from_secs(30),
            retries: 3,
            verbose: false,
        }
    }
}
```

## Notes
Do not implement `Default` when some field has no meaningful default (for example a required `UserId`). In that case keep a constructor that takes the required values, or use a builder.
