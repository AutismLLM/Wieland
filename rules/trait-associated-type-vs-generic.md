---
id: trait-associated-type-vs-generic
title: Use an associated type for one output per impl, a generic param for many
category: traits
severity: info
tier: 1
enforceability: judge
tags: [api-design, generics, associated-types]
applies_to:
  - "trait with a single natural output type per implementor"
  - "trait implemented for many input types on one type"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#trait-associated-type-vs-generic
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use an associated type when each implementor has exactly one output type; use a generic parameter when a single type can implement the trait for many input types.

## Why
An associated type is part of the implementing type's identity: there is one valid binding per impl, so callers never need turbofish. A generic parameter allows multiple simultaneous impls on the same type — `impl Add<f64>` and `impl Add<Vec2>` can coexist, while an `Iterator` can only define one `Item`. Picking an associated type when several impls are needed makes them impossible; picking a generic when there is one output forces noisy annotations at every call site.

## Do
```rust
trait Parser {
    type Output;
    fn parse(&self, input: &str) -> Option<Self::Output>;
}

struct NumberParser;

impl Parser for NumberParser {
    type Output = f64;
    fn parse(&self, input: &str) -> Option<f64> {
        input.trim().parse().ok()
    }
}

fn run<P: Parser>(p: &P, s: &str) -> Option<P::Output> {
    p.parse(s)
}

impl std::ops::Add<f64> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: f64) -> Vec2 { Vec2 { x: self.x + rhs, y: self.y + rhs } }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 { Vec2 { x: self.x + rhs.x, y: self.y + rhs.y } }
}

#[derive(Clone, Copy)]
struct Vec2 { x: f64, y: f64 }
```

## Don't
```rust
trait Parser<Output> {
    fn parse(&self, input: &str) -> Option<Output>;
}

struct JsonParser;

impl Parser<String> for JsonParser {
    fn parse(&self, input: &str) -> Option<String> {
        Some(input.to_owned())
    }
}

fn run<P: Parser<String>>(p: &P, s: &str) -> Option<String> {
    p.parse(s)
}
```

## Notes
When you must constrain the associated type in a bound, write `P: Parser<Output = JsonValue>` — still less noisy than a free generic parameter. `Iterator`, `Future`, and `Deref` use associated types; `Add<Rhs>`, `From<T>`, and `Into<T>` use generic parameters.
