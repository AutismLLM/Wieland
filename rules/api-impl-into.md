---
id: api-impl-into
title: Accept impl Into<T> for ergonomic construction, implement From for the conversion
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [conversion, into, ergonomics]
applies_to:
  - "fn parameter that will be stored as an owned value"
  - "constructor or builder setter"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-impl-into
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a function needs to own its argument, take `impl Into<T>` so callers pass either the target type or anything convertible to it, and supply the conversion by implementing `From`.

## Why
`impl Into<String>` lets a caller pass `&str`, `String`, or a `format!` result with no explicit `.into()`, removing conversion boilerplate at every call site while the function converts once internally. Because `From` implies `Into` via the standard blanket impl, you implement `From<T>` and get the flexible parameter for free. Prefer `AsRef<T>` instead when the function only borrows rather than stores the value.

## Do
```rust
struct Config {
    name: String,
}

impl Config {
    fn new(name: impl Into<String>) -> Self {
        Config { name: name.into() }
    }
}

let _ = Config::new("myapp");
let _ = Config::new(String::from("myapp"));
let _ = Config::new(format!("app-{}", 7));
```

## Don't
```rust
struct Config {
    name: String,
}

impl Config {
    fn new(name: String) -> Self {
        Config { name }
    }
}

let _ = Config::new("myapp".to_string());
```

## Notes
Avoid `impl Into<T>` where it does not fit: in struct fields and recursive types (`impl Trait` is not allowed there), in return position when callers need to name the type, and on hot paths where you would rather take the concrete type directly.
