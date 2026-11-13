---
id: api-operator-overload
title: Overload operators only when their meaning is natural and side-effect-free
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [operators, traits, least-surprise]
applies_to:
  - "impl Add/Sub/Mul/Index for a custom type"
  - "std::ops trait implementation"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-operator-overload
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Implement a `std::ops` operator only when its meaning is immediately obvious and free of side effects; otherwise expose a named method.

## Why
Operators carry strong reader expectations: `+` combines, `*` scales, `Index` looks up — cheaply and without surprises. Overloads that fit that intuition (numeric newtypes, vector and matrix math, set union with `|`) make code read naturally. Overloads that hide side effects — a `+` that mutates state, logs, or performs IO — violate the principle of least surprise and mislead every reader. When the meaning is not self-evident, a named method is clearer. Also implement the operator for `&` forms so callers are not forced to clone.

## Do
```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2 {
    x: f64,
    y: f64,
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Add for &Vector2 {
    type Output = Vector2;
    fn add(self, rhs: &Vector2) -> Vector2 {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

let a = Vector2 { x: 1.0, y: 2.0 };
let b = Vector2 { x: 3.0, y: 4.0 };
assert_eq!(a + b, Vector2 { x: 4.0, y: 6.0 });
```

## Don't
```rust
use std::ops::Add;

struct Logger(Vec<String>);

impl Add<String> for Logger {
    type Output = Logger;
    fn add(mut self, msg: String) -> Logger {
        println!("logged: {msg}");
        self.0.push(msg);
        self
    }
}
```

## Notes
When you implement a binary operator, implement its assignment form too (`AddAssign` alongside `Add`), and consider the `&` and scalar right-hand-side variants real numeric types tend to need.
