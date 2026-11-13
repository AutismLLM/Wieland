---
id: own-copy-small
title: Derive Copy for small, plain-data types
category: ownership
severity: info
tier: 1
enforceability: judge
tags: [copy, ergonomics, value-types]
applies_to:
  - "small struct of Copy fields (no heap data, no Drop)"
  - "newtype wrapping an integer or float"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#own-copy-small
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Derive `Copy` (alongside `Clone`) for small value types whose fields are all `Copy` and that hold no heap data or `Drop` impl.

## Why
A `Copy` type is duplicated implicitly on assignment and pass-by-value, so callers never write `.clone()` and values stay usable after being passed. For types up to roughly two machine words, copying the bytes is as cheap as moving a pointer, so the ergonomic win is free. A type qualifies only if every field is `Copy` and it has no custom `Drop`.

## Do
```rust
#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn distance(a: Point, b: Point) -> f64 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt()
}

let origin = Point { x: 0.0, y: 0.0 };
let d = distance(origin, Point { x: 3.0, y: 4.0 });
let _still_valid = origin;
```

## Don't
```rust
#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn distance(a: Point, b: Point) -> f64 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt()
}

let origin = Point { x: 0.0, y: 0.0 };
let d = distance(origin.clone(), Point { x: 3.0, y: 4.0 });
```

## Notes
Do not force `Copy` onto larger types (roughly over 64 bytes) just for convenience — silent bytewise copies at every use can cost more than an explicit move. A type with heap data (`String`, `Vec`, `Box`) cannot be `Copy` at all.
