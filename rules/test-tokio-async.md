---
id: test-tokio-async
title: Drive async tests with #[tokio::test] instead of a manual runtime
category: testing
severity: info
tier: 1
enforceability: judge
tags: [async, tokio, runtime]
applies_to:
  - "async test functions"
  - "tests that build a tokio Runtime by hand to block_on"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-tokio-async
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Annotate async tests with `#[tokio::test]`, which provides the runtime, instead of constructing a `Runtime` and calling `block_on` yourself.

## Why
An async fn needs a runtime to be driven, and a plain `#[test] async fn` does not compile. `#[tokio::test]` sets up and tears down a runtime per test, keeping the body flat and readable. Building a `Runtime` by hand in every test is boilerplate that is easy to get subtly wrong, and it obscures the actual test logic.

## Do
```rust
#[tokio::test]
async fn fetch_data_succeeds() {
    let result = fetch_data().await;
    assert!(result.is_ok());
}
```

## Don't
```rust
#[test]
fn fetch_data_succeeds() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let result = fetch_data().await;
        assert!(result.is_ok());
    });
}
```

## Notes
Configure the flavor when a test needs it: `#[tokio::test(flavor = "current_thread")]` for deterministic single-threaded runs, or `#[tokio::test(start_paused = true)]` to control time with `tokio::time::advance`.
