---
id: doc-examples-section
title: Give public APIs a `# Examples` section with runnable code
category: docs
severity: info
tier: 1
enforceability: judge
tags: [rustdoc, examples, doctest]
applies_to:
  - "pub fn"
  - "pub struct"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#doc-examples-section
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Add a `# Examples` section with runnable code to the public items users are most likely to call.

## Why
Examples are the highest-value part of documentation: they show exactly how to use an API, so users copy rather than guess. Because Rust compiles and runs `# Examples` blocks as doc-tests, they also stay correct as the code evolves — a stale example fails `cargo test --doc`.

## Do
```rust
/// Parses a string into a `Foo`.
///
/// # Examples
///
/// ```
/// use my_crate::parse;
///
/// let foo = parse("hello").unwrap();
/// assert_eq!(foo.name(), "hello");
/// ```
pub fn parse(s: &str) -> Result<Foo, Error> {
    todo!()
}
```

## Don't
```rust
/// Parses a string into a Foo.
pub fn parse(s: &str) -> Result<Foo, Error> {
    todo!()
}
```

## Notes
Prefer `?` over `.unwrap()` in examples, and hide setup lines with a `# ` prefix so the example stays focused.
