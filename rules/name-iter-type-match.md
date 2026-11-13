---
id: name-iter-type-match
title: Name iterator types after the method that returns them
category: naming
severity: info
tier: 1
enforceability: judge
tags: [naming, iterators, types]
applies_to:
  - "named iterator structs returned from iter/keys/values/drain"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-iter-type-match
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Name a returned iterator type after its constructing method: `iter` returns `Iter`, `into_iter` returns `IntoIter`, `keys` returns `Keys`.

## Why
This pattern is set by the standard library, so `Iter`, `IterMut`, `Keys`, and `Drain` are instantly recognizable and let a reader map a method to its type without hunting. A name like `MyCollectionIterator` adds noise, and a bare `Iterator` collides with the trait.

## Do
```rust
pub struct MyCollection<T> {
    items: Vec<T>,
}

pub struct Iter<'a, T> {
    inner: std::slice::Iter<'a, T>,
}

impl<T> MyCollection<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { inner: self.items.iter() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
```

## Don't
```rust
pub struct MyCollection<T> {
    items: Vec<T>,
}

pub struct MyCollectionIterator<'a, T> {
    inner: std::slice::Iter<'a, T>,
}

impl<T> MyCollection<T> {
    pub fn iter(&self) -> MyCollectionIterator<'_, T> {
        MyCollectionIterator { inner: self.items.iter() }
    }
}
```

## Notes
Custom traversals extend the pattern: `nodes` returns `Nodes`, `edges` returns `Edges`.
