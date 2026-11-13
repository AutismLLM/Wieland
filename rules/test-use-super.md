---
id: test-use-super
title: Import the module under test with use super::* in the tests module
category: testing
severity: info
tier: 1
enforceability: judge
tags: [structure, imports, unit-tests]
applies_to:
  - "#[cfg(test)] mod tests bodies"
  - "verbose crate:: imports inside a test module"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-use-super
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Start a `#[cfg(test)] mod tests` with `use super::*;` to pull in the parent module's items, including private ones, rather than re-importing each name by absolute path.

## Why
The tests module is a child of the module it tests, so `use super::*` gives it every item from the parent — public and private — in one line. Listing `crate::my_module::Thing` imports one by one is verbose and cannot reach private helpers, forcing you to widen visibility just to test internals.

## Do
```rust
pub fn public_function() -> i32 {
    private_helper() + 1
}

fn private_helper() -> i32 {
    41
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_helper_is_answerish() {
        assert_eq!(private_helper(), 41);
    }
}
```

## Don't
```rust
#[cfg(test)]
mod tests {
    use crate::my_module::public_function;

    #[test]
    fn public_function_adds_one() {
        assert_eq!(public_function(), 42);
    }
}
```

## Notes
In deeply nested modules reach a grandparent with `use super::super::*`; add test-only crates (`use proptest::prelude::*`) alongside the `use super::*` line.
