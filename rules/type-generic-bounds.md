---
id: type-generic-bounds
title: Put trait bounds only where needed and prefer where clauses
category: types
severity: warn
tier: 2
enforceability: gate
tags: [generics, bounds, readability]
applies_to:
  - "generic struct definition"
  - "generic function signature"
rust_edition: "2024"
msrv: null
detect:
  clippy: [trait_duplication_in_bounds, type_repetition_in_bounds]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#type-generic-bounds
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Add trait bounds only where the functionality needs them — on the impls, not the type definition — and move multi-bound signatures into a `where` clause.

## Why
Bounds on a struct definition constrain every use of the type, even plain storage that does not need them. Redundant or inline-crammed bounds hurt readability and flexibility. Placing each bound on the impl that uses it keeps the type maximally usable, and a `where` clause keeps complex signatures scannable.

## Do
```rust
struct Container<T> {
    items: Vec<T>,
}

impl<T: Clone> Container<T> {
    fn duplicate(&self) -> Self {
        Container { items: self.items.clone() }
    }
}

fn process<T, E>(value: T) -> Result<T, E>
where
    T: Clone + std::fmt::Debug + Send + Sync + 'static,
    E: std::error::Error + Send + Clone,
{
    Ok(value)
}
```

## Don't
```rust
struct Container<T: Clone + std::fmt::Debug> {
    items: Vec<T>,
}

fn print_twice<T: Clone + std::fmt::Debug>(value: T)
where
    T: Clone,
{
    println!("{value:?} {value:?}");
}
```

## Notes
Supertrait bounds are implied (`T: Foo` where `trait Foo: Clone` gives you `T: Clone` for free), so do not restate them. A `where` clause is the only place you can bound associated types or expressions like `Vec<U>: Debug`.
