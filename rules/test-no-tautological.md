---
id: test-no-tautological
title: Tests must verify behavior, not restate the value under test
category: testing
severity: warn
tier: 1
enforceability: judge
tags: [testing, correctness, signal]
applies_to:
  - "#[test] functions that assert a constant equals its own literal"
  - "tests that mirror the implementation's branches instead of its contract"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-TAUTOLOGICAL-TESTS
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
A test must check a property of the code under test, not simply repeat the same value or logic the implementation already encodes.

## Why
A test that asserts `CHECKPOINTS == [0, 90, 180, 270]` right next to the definition `const CHECKPOINTS: [u32; 4] = [0, 90, 180, 270]` passes by construction and can never catch a regression — it only re-types the literal. Such tests pass mutation testing trivially, inflate coverage numbers, and add churn (every legitimate edit to the constant forces an edit to the "test") without ever proving the code behaves correctly. A test earns its keep only when it can fail for a reason that matters.

## Do
```rust
const CHECKPOINTS: [u32; 4] = [0, 90, 180, 270];

#[test]
fn checkpoints_are_evenly_spaced_and_increasing() {
    let gaps: Vec<u32> = CHECKPOINTS.windows(2).map(|w| w[1] - w[0]).collect();
    assert!(gaps.iter().all(|&g| g == gaps[0]), "checkpoints must be evenly spaced");
    assert!(CHECKPOINTS.windows(2).all(|w| w[0] < w[1]), "checkpoints must be increasing");
}
```

## Don't
```rust
const CHECKPOINTS: [u32; 4] = [0, 90, 180, 270];

#[test]
fn checkpoints_are_correct() {
    assert_eq!(CHECKPOINTS, [0, 90, 180, 270]);
}
```

## Notes
If a mutation-testing tool flags a mutant that only a tautological assertion would kill, skip that mutant explicitly instead of writing a test that adds no signal. Prefer asserting an invariant (ordering, spacing, range, relationship between fields) that would actually break if the implementation regressed.
