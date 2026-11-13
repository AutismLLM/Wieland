---
id: test-proptest-properties
title: Test invariants across generated inputs with proptest
category: testing
severity: info
tier: 1
enforceability: judge
tags: [property-testing, proptest, invariants]
applies_to:
  - "functions with algebraic properties (roundtrip, idempotence, invariants)"
  - "parsers, encoders, and pure transformations"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#test-proptest-properties
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `proptest` to assert properties — roundtrips, idempotence, invariants — over randomly generated inputs rather than only a handful of hand-picked examples.

## Why
Example-based tests only cover the cases you thought of. Property tests generate many inputs and surface the edge cases you didn't — empty strings, boundary integers, unusual Unicode — and on failure proptest shrinks the input to a minimal reproducing case, so debugging starts from the smallest counterexample.

## Do
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn double_reverse_is_identity(s in ".*") {
        let reversed: String = s.chars().rev().collect();
        let restored: String = reversed.chars().rev().collect();
        prop_assert_eq!(s, restored);
    }

    #[test]
    fn sort_is_idempotent(mut v in prop::collection::vec(any::<i32>(), 0..100)) {
        v.sort();
        let once = v.clone();
        v.sort();
        prop_assert_eq!(v, once);
    }
}
```

## Don't
```rust
#[test]
fn double_reverse_is_identity() {
    let s = "hello".to_string();
    let reversed: String = s.chars().rev().collect();
    let restored: String = reversed.chars().rev().collect();
    assert_eq!(s, restored);
}
```
