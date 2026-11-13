---
id: anti-type-erasure
title: Return impl Trait instead of Box<dyn Trait> when the type is known
category: anti-patterns
severity: warn
tier: 2
enforceability: check
tags: [performance, dispatch, generics]
applies_to:
  - "Box<dyn Trait> return type wrapping a single concrete type"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['Box<dyn ']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#anti-type-erasure
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `impl Trait` when a single concrete type is returned; reserve `Box<dyn Trait>` for genuinely heterogeneous or runtime-chosen types.

## Why
`Box<dyn Trait>` erases the type behind a heap allocation and dynamic dispatch. When only one concrete type flows out, `impl Trait` gives the same caller-facing flexibility with zero overhead: the value stays on the stack and calls are statically dispatched and inlinable.

## Do
```rust
fn get_iterator() -> impl Iterator<Item = i32> {
    (0..10).map(|x| x * 2)
}
```

## Don't
```rust
fn get_iterator() -> Box<dyn Iterator<Item = i32>> {
    Box::new((0..10).map(|x| x * 2))
}
```

## Notes
`Box<dyn Trait>` is the right tool for a heterogeneous collection of different concrete types, a type chosen at runtime from config, or a recursive structure. When you have a small closed set of variants, an `enum` with a method dispatched by `match` avoids both the box and the trait object.
