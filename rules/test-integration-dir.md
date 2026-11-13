---
id: test-integration-dir
title: Put integration tests in the tests/ directory
category: testing
severity: info
tier: 1
enforceability: judge
tags: [structure, integration-tests, public-api]
applies_to:
  - "tests exercising the crate's public API end to end"
  - "tests/ directory layout"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-integration-dir
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Put integration tests in files under `tests/` at the crate root, where each file is compiled as its own crate and can only see the public API.

## Why
A `tests/` file links against your crate exactly as an external user would, so it verifies the real public interface rather than implementation details. Whole-workflow tests wedged into `src/` under `#[cfg(test)]` can reach private items, which lets them lean on internals that a downstream consumer cannot use.

## Do
```rust
// tests/integration_test.rs
use my_crate::{Client, Config};

#[test]
fn full_workflow_succeeds() {
    let config = Config::default();
    let client = Client::new(config);

    let result = client.process("input");

    assert!(result.is_ok());
}
```

## Don't
```rust
// src/lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn integration_test_full_workflow() {
        let client = crate::internal::RawClient::new();
        assert!(client.process("input").is_ok());
    }
}
```

## Notes
Share helpers across integration files via a `tests/common/mod.rs` module; a bare `tests/common.rs` would itself be compiled as a test binary.
