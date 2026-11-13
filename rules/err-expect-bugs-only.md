---
id: err-expect-bugs-only
title: Use expect only for invariants that indicate bugs, not recoverable failures
category: errors
severity: warn
tier: 1
enforceability: judge
tags: [error-handling, panic, expect]
applies_to:
  - ".expect() on user input, IO, or network results"
  - ".expect() on a parse of external data"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#err-expect-bugs-only
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Reserve `.expect()` for violated invariants that can only fail because of a bug, and write a message explaining why the invariant holds; return a `Result` for user, IO, or network failures.

## Why
`.expect()` still panics — it only differs from `.unwrap()` by carrying a message. Using it on things that legitimately fail (bad input, a missing file, a refused connection) turns recoverable conditions into crashes. Restricting it to true invariants means every `.expect()` in the codebase marks a "this cannot happen unless our code is wrong" point, and its message tells the next developer how to fix the bug.

## Do
```rust
fn cache_and_get(&mut self, key: String, value: Value) -> &Value {
    self.cache.insert(key.clone(), value);
    self.cache
        .get(&key)
        .expect("BUG: key must exist immediately after insert")
}
```

## Don't
```rust
fn parse_user_input(input: &str) -> Config {
    serde_json::from_str(input).expect("Invalid JSON")
}
```

## Notes
When failure is expected, return a `Result` (`input.parse().map_err(|_| ConfigError::InvalidPort)?`), supply a default (`unwrap_or(8080)`), or handle it explicitly. The "validate once, `expect` after" pattern — building a validated newtype whose invariant later `expect`s hold — is a legitimate use.
