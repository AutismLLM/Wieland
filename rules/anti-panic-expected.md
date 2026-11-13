---
id: anti-panic-expected
title: Return Result for recoverable errors instead of panicking
category: anti-patterns
severity: warn
tier: 2
enforceability: check
tags: [error-handling, panic]
applies_to:
  - "panic!/expect on network, file, or input failures"
  - "panic! used for input validation"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['panic!\(', '\.expect\(']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#anti-panic-expected
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Represent recoverable failures — bad input, missing files, network errors — as a returned `Result`, not a `panic!` or `.expect()`.

## Why
Panics crash the program and are meant for unrecoverable situations: bugs, corrupted state, violated invariants. Using them for expected conditions makes software fragile and pushes callers into catching unwinds or dying. `Result` lets each caller decide how to respond.

## Do
```rust
fn parse_config(input: &str) -> Result<Config, ConfigError> {
    toml::from_str(input).map_err(ConfigError::Parse)
}

fn process_age(age: i32) -> Result<(), ValidationError> {
    if age < 0 {
        return Err(ValidationError::NegativeAge);
    }
    Ok(())
}
```

## Don't
```rust
fn parse_config(input: &str) -> Config {
    toml::from_str(input).expect("invalid config")
}

fn process_age(age: i32) {
    if age < 0 {
        panic!("age cannot be negative");
    }
}
```

## Notes
`panic!`, `unreachable!`, and `assert!` are the right tools for genuine bugs and impossible states — an invariant you control being violated, or an unrecoverable initialization failure. Do not use panic for ordinary control flow.
