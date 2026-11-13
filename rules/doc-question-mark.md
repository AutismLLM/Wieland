---
id: doc-question-mark
title: Propagate errors with `?` in doc examples, not `.unwrap()`
category: docs
severity: info
tier: 2
enforceability: check
tags: [rustdoc, examples, error-handling]
applies_to:
  - "# Examples block calling a fallible function"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['///.*\.unwrap\(\)']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#doc-question-mark
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `?` to propagate errors in doc examples instead of `.unwrap()`.

## Why
Doc examples are copied verbatim; `.unwrap()` teaches users to ignore errors, while `?` models proper propagation. It also makes the doc-test itself stricter: an error fails the test instead of panicking with a generic message, so bugs in the example surface. Rust wraps the example in a function returning `Result`, so `?` works with a trailing `# Ok::<(), Error>(())`.

## Do
```rust
/// Reads a configuration file.
///
/// # Examples
///
/// ```
/// # use my_crate::{Config, Error};
/// let config = Config::from_file("config.toml")?;
/// println!("{:?}", config.database_url);
/// # Ok::<(), Error>(())
/// ```
pub fn from_file(path: &str) -> Result<Config, Error> {
    todo!()
}
```

## Don't
```rust
/// Reads a configuration file.
///
/// # Examples
///
/// ```
/// let config = Config::from_file("config.toml").unwrap();
/// println!("{:?}", config.database_url);
/// ```
pub fn from_file(path: &str) -> Result<Config, Error> {
    todo!()
}
```

## Notes
`.unwrap()` is acceptable for values known-good at compile time, such as a literal regex or `"42".parse::<i32>().unwrap()`.
