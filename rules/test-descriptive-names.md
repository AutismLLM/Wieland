---
id: test-descriptive-names
title: Name tests for the behavior they verify
category: testing
severity: info
tier: 1
enforceability: judge
tags: [testing, readability, docs]
applies_to:
  - "#[test] function named test1 or it_works"
  - "test function name"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-descriptive-names
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Name each test after the behavior it verifies, so the name alone tells you what broke when it fails.

## Why
Test names appear verbatim in the failure output and act as executable documentation of the behavior under test. `test test1 ... FAILED` tells you nothing; `test parse_returns_error_for_empty_input ... FAILED` tells you exactly what regressed without opening the file. A descriptive `function_condition_expected` (or `scenario_expectation`) name turns the test list into a readable spec of the module's contract.

## Do
```rust
#[test]
fn parse_returns_error_for_empty_input() { /* ... */ }

#[test]
fn parse_handles_unicode_characters() { /* ... */ }

#[test]
fn expired_token_is_rejected() { /* ... */ }

#[test]
fn adding_item_increases_cart_total() { /* ... */ }
```

## Don't
```rust
#[test]
fn test1() { /* ... */ }

#[test]
fn test_parse() { /* ... */ }

#[test]
fn it_works() { /* ... */ }
```

## Notes
Common patterns are `function_condition_expected_result`, `scenario_expectation`, and BDD-style `when_x_then_y`. Group related tests into nested `mod` blocks (`tests::parsing::rejects_trailing_comma`) so the module path adds context in the output.
