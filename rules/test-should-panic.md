---
id: test-should-panic
title: Assert expected panics with #[should_panic], not manual catch_unwind
category: testing
severity: info
tier: 2
enforceability: check
tags: [panics, should-panic, assertions]
applies_to:
  - "tests verifying that invalid input triggers a panic"
  - "std::panic::catch_unwind in test code"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['catch_unwind']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-should-panic
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `#[should_panic(expected = "...")]` to assert that a test's body panics, matching on a substring of the panic message rather than reaching for `catch_unwind`.

## Why
`#[should_panic]` states the expectation declaratively and the `expected` string pins it to the right panic, so an unrelated panic still fails the test. Hand-rolled `std::panic::catch_unwind` is verbose, prints scary backtraces during a passing run, and gives no easy way to check which panic occurred.

## Do
```rust
#[test]
#[should_panic(expected = "division by zero")]
fn divide_by_zero_panics() {
    divide(1, 0);
}
```

## Don't
```rust
#[test]
fn divide_by_zero_panics() {
    let result = std::panic::catch_unwind(|| divide(1, 0));
    assert!(result.is_err());
}
```

## Notes
Reserve `#[should_panic]` for genuine panics (invariant violations, programmer bugs). For recoverable failures the function should return `Result` and the test should assert `result.is_err()` instead.
