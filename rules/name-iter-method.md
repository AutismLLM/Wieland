---
id: name-iter-method
title: Name iterator methods by convention so for loops work
category: naming
severity: info
tier: 1
enforceability: judge
tags: [naming, iterators, intoiterator]
applies_to:
  - "types implementing IntoIterator or exposing an owning iterator"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-iter-method
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Provide owning iteration through an `IntoIterator` impl whose `into_iter` yields `T`, not an ad-hoc method like `to_iter`.

## Why
`for item in collection` desugars to `collection.into_iter()`, so a type only participates in the loop syntax and iterator adapters when it implements `IntoIterator` under the standard names. A hand-rolled `to_iter` compiles but never plugs into that machinery, surprising callers.

## Do
```rust
struct Collection<T> {
    items: Vec<T>,
}

impl<T> IntoIterator for Collection<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
```

## Don't
```rust
struct Collection<T> {
    items: Vec<T>,
}

impl<T> Collection<T> {
    fn to_iter(self) -> impl Iterator<Item = T> {
        self.items.into_iter()
    }
}
```

## Notes
Implement `IntoIterator` for `&T` and `&mut T` too if borrowing iteration in a `for` loop should work.
