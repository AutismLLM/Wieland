---
id: test-cfg-test-module
title: Put unit tests in a #[cfg(test)] mod tests within each module
category: testing
severity: info
tier: 2
enforceability: check
tags: [structure, unit-tests, cfg]
applies_to:
  - "mod tests declarations"
  - "test functions living in src/"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['^\s*mod tests\s*\{']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-cfg-test-module
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Place unit tests in a `#[cfg(test)] mod tests { use super::*; }` inside the same file as the code they test.

## Why
`#[cfg(test)]` compiles the test module only under `cargo test`, keeping it out of release binaries. Keeping tests in a `tests` submodule of the same file puts them next to the code they exercise while giving them access to private items via `use super::*`. A bare `mod tests` without the attribute compiles test code into the shipped binary.

## Do
```rust
fn public_api() -> i32 {
    private_helper() * 2
}

fn private_helper() -> i32 {
    21
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_api_doubles_helper() {
        assert_eq!(public_api(), 42);
    }
}
```

## Don't
```rust
mod tests {
    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}
```
