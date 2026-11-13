---
id: test-doctest-examples
title: Keep documentation examples as executable doctests
category: testing
severity: info
tier: 1
enforceability: judge
tags: [documentation, doctests, examples]
applies_to:
  - "/// doc comments containing example code"
  - "public API documentation"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-doctest-examples
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Write documentation examples inside fenced ```` ```rust ```` code blocks so `cargo test` compiles and runs them as doctests.

## Why
Doctests both show readers how to use an API and prove the shown code still compiles and runs. Examples pasted as plain text, or written with the wrong number of arguments, drift out of sync with the API silently — a real doctest turns that drift into a failing build.

## Do
```rust
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// use my_crate::add;
///
/// let sum = add(1, 2);
/// assert_eq!(sum, 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Don't
```rust
/// Adds two numbers.
///
/// Example:
/// let sum = add(1, 2, 3);
/// assert_eq!(sum, 6);
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Notes
Use ```` ```no_run ```` for examples that compile but should not execute (servers, blocking loops), and ```` ```compile_fail ```` to assert that a snippet is rejected by the compiler.
