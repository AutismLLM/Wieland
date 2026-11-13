---
id: anti-expect-lazy
title: Reserve expect for bugs, not for recoverable I/O or input errors
category: anti-patterns
severity: warn
tier: 2
enforceability: check
tags: [error-handling, panic]
applies_to:
  - ".expect() on network, file, database, or parse results"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['\.expect\(']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#anti-expect-lazy
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `?` to return an error for conditions that can occur in production; reserve `.expect()` for invariants whose violation is a bug.

## Why
`.expect()` is still a panic — it just carries a custom message. Using it for expected failures (network timeouts, missing files, invalid user input) crashes the program on conditions that should be handled gracefully, forcing callers to either catch the unwind or die with it.

## Do
```rust
let config = fs::read_to_string("config.toml")
    .context("failed to read config file")?;

let age: u32 = input.parse()
    .map_err(|_| Error::InvalidInput("age must be a number"))?;
```

## Don't
```rust
let config = fs::read_to_string("config.toml").expect("config not found");

let age: u32 = input.parse().expect("invalid age");
```

## Notes
`.expect()` is appropriate for programming errors that indicate a bug: a poisoned mutex, a regex known valid at compile time, or embedded static data that must parse. There the message documents the invariant you are asserting.
