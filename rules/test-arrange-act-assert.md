---
id: test-arrange-act-assert
title: Structure tests with clear Arrange, Act, Assert sections
category: testing
severity: info
tier: 1
enforceability: judge
tags: [readability, structure, unit-tests]
applies_to:
  - "#[test] functions"
  - "test bodies that mix setup, execution, and assertions"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-arrange-act-assert
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Structure each test in three clear phases — arrange the inputs, act by calling the code under test, then assert on the result — and test one behavior per function.

## Why
The AAA pattern makes a test's intent obvious: what is set up, what is exercised, what is verified. A test that interleaves setup, calls, and assertions for several unrelated concerns is hard to read and, when it fails, hard to localize. One behavior per test means a failure names the exact broken behavior.

## Do
```rust
#[test]
fn user_creation_fails_with_empty_name() {
    let name = "";
    let email = "email@example.com";

    let result = User::new(name, email);

    assert!(matches!(result, Err(UserError::EmptyName)));
}
```

## Don't
```rust
#[test]
fn test_user() {
    assert_eq!(User::new("alice", "alice@example.com").unwrap().name(), "alice");
    assert!(User::new("", "email@example.com").is_err());
    let u = User::new("bob", "bob@example.com").unwrap();
    assert!(u.validate());
    assert_eq!(u.email(), "bob@example.com");
}
```
