---
id: api-must-use
title: Add #[must_use] to types and functions whose result must not be ignored
category: api-design
severity: warn
tier: 2
enforceability: gate
tags: [must-use, correctness, api]
applies_to:
  - "fn returning a pure computed value"
  - "RAII guard, future, or iterator type"
rust_edition: "2024"
msrv: null
detect:
  clippy: [must_use_candidate]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-must-use
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Mark a function or type `#[must_use]` when discarding its return value is almost always a bug — pure computations, RAII guards, futures, iterators, and fallible results.

## Why
Some values exist only to be used: a pure function's output, a lock guard whose drop releases the lock, a future that does nothing until awaited, a lazy iterator adaptor. Ignoring them is silent and hard to spot in review. `#[must_use]` makes the compiler warn on the unused value, and a custom message tells the reader what to do instead. Clippy's `must_use_candidate` suggests functions that should carry it.

## Do
```rust
#[must_use = "this returns a new value and does not modify the input"]
fn checksum(data: &[u8]) -> u32 {
    data.iter().map(|&b| u32::from(b)).sum()
}

let data = [1u8, 2, 3];
let c = checksum(&data);
assert_eq!(c, 6);
```

## Don't
```rust
fn checksum(data: &[u8]) -> u32 {
    data.iter().map(|&b| u32::from(b)).sum()
}

let data = [1u8, 2, 3];
checksum(&data);
```

## Notes
Do not apply `#[must_use]` to functions run for their side effects where ignoring the result is legitimate, such as a `log` call returning `Result<(), io::Error>` that callers may reasonably drop.
