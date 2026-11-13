---
id: lint-warn-complexity
title: Enable clippy::complexity to flag needlessly convoluted code
category: lints
severity: warn
tier: 2
enforceability: gate
tags: [clippy, complexity, readability]
applies_to:
  - "manual Option::map / and_then chains"
  - "match arms that just rewrap Ok/Err"
  - "redundant casts and conversions"
rust_edition: "2024"
msrv: null
detect:
  clippy: [bind_instead_of_map, bool_comparison, clone_on_copy, filter_next, unnecessary_cast, useless_conversion]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#lint-warn-complexity
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Enable the `clippy::complexity` group so unnecessarily convoluted code is flagged and pointed at its simpler equivalent.

## Why
Complex code is harder to read, harder to maintain, and hides bugs in its folds. The complexity group spots hand-rolled versions of standard combinators — a `match` that just rewraps `Ok`/`Err`, `and_then(|x| Some(...))` instead of `map`, `.clone()` on a `Copy` type — and names the shorter form. Adopting the suggestions lowers the cognitive load of every future reader.

## Do
```rust
fn find_positive(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().find(|&x| x > 0)
}
```

## Don't
```rust
fn find_positive(nums: &[i32]) -> Option<i32> {
    let filtered: Vec<_> = nums.iter().cloned().filter(|x| *x > 0).collect();
    if filtered.len() == 0 {
        None
    } else {
        Some(filtered[0])
    }
}
```

## Notes
Complexity suggestions are almost always improvements, but a few (like collapsing a match that aids readability of an exhaustive enum) can be judgment calls; `#[allow]` the specific lint at the specific site rather than turning the group off.
