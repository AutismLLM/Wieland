---
id: api-impl-fromiterator
title: Implement FromIterator, Extend, and IntoIterator on your collection types
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [iterator, collections, traits]
applies_to:
  - "custom collection type"
  - "type wrapping Vec or HashMap"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-impl-fromiterator
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Give collection types the full iteration contract: `FromIterator<T>` for `collect`, `Extend<T>` for batch insertion, and `IntoIterator` for the owned, `&`, and `&mut` forms.

## Why
These traits are what makes a collection feel native. `FromIterator` enables `iter.collect::<MyCollection>()`; `Extend` enables `.extend(iter)` and is what `collect` calls internally to reuse an allocation; the three `IntoIterator` impls let the type drive `for` loops and adapter chains by value, by shared reference, and by mutable reference. Omitting them forces callers into manual push loops and breaks generic code that expects the standard collection shape.

## Do
```rust
struct Bag<T>(Vec<T>);

impl<T> FromIterator<T> for Bag<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Bag(iter.into_iter().collect())
    }
}

impl<T> Extend<T> for Bag<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl<T> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Bag<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

let b: Bag<i32> = [1, 2, 3].into_iter().collect();
for _x in &b {}
```

## Don't
```rust
struct Bag<T>(Vec<T>);

impl<T> Bag<T> {
    fn new() -> Self { Bag(Vec::new()) }
    fn push(&mut self, item: T) { self.0.push(item); }
}

let mut b = Bag::new();
for x in [1, 2, 3] {
    b.push(x);
}
```

## Notes
When your collection wraps a standard container, delegate `from_iter` and `extend` to the inner container's implementations so `collect` can reuse its allocation.
