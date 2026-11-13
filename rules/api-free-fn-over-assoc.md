---
id: api-free-fn-over-assoc
title: Prefer free functions over associated functions for logic unrelated to construction
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [api, functions, readability]
applies_to:
  - "impl blocks containing a fn with no meaningful receiver relationship"
  - "helper logic that does not read or produce Self"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-REGULAR-FN
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Reserve `impl` blocks for constructors and methods that take `self`/`&self`/`&mut self`; move logic that neither reads nor produces `Self` out to a plain free function.

## Why
Unlike some OO languages, Rust functions are first-class without a hosting class. Parking unrelated logic under `impl Type` as an associated function only adds required qualification (`Type::helper(x)` vs `helper(x)`) without adding meaning, and it misleads readers into thinking the function operates on `Type`.

## Do
```rust
struct Database;

impl Database {
    fn new() -> Self {
        Database
    }

    fn query(&self, sql: &str) -> Vec<String> {
        vec![sql.to_string()]
    }
}

fn check_parameters(p: &str) -> bool {
    !p.is_empty()
}
```

## Don't
```rust
struct Database;

impl Database {
    fn new() -> Self {
        Database
    }

    fn query(&self, sql: &str) -> Vec<String> {
        vec![sql.to_string()]
    }

    fn check_parameters(p: &str) -> bool {
        !p.is_empty()
    }
}
```

## Notes
Associated functions required by a trait (`Default::default`, `FromStr::from_str`) are idiomatic even without a `self` receiver — the exception is trait conformance, not general-purpose computation.
