---
id: name-iter-convention
title: Name iterator methods iter, iter_mut, and into_iter
category: naming
severity: info
tier: 1
enforceability: judge
tags: [naming, iterators, api]
applies_to:
  - "collection types that expose iteration"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-iter-convention
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Expose iteration through the three standard names: `iter` (borrows `&T`), `iter_mut` (borrows `&mut T`), and `into_iter` (consumes, yields `T`).

## Why
These names encode ownership semantics and are what every Rust reader and every `for` loop expects. Inventing names like `elements` or `get_items` breaks recognition and the `for item in &collection` desugaring that relies on `IntoIterator`.

## Do
```rust
struct MyCollection<T> {
    items: Vec<T>,
}

impl<T> MyCollection<T> {
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut()
    }
}
```

## Don't
```rust
struct MyCollection<T> {
    items: Vec<T>,
}

impl<T> MyCollection<T> {
    fn elements(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    fn get_items(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}
```

## Notes
Domain-specific iterators follow the same shape: `keys`, `values`, `values_mut`, `drain`.
