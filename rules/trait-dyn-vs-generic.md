---
id: trait-dyn-vs-generic
title: Choose static dispatch vs dynamic dispatch (dyn Trait) deliberately
category: traits
severity: info
tier: 1
enforceability: judge
tags: [api-design, dispatch, performance]
applies_to:
  - "Box<dyn Trait> used for a single known concrete type"
  - "generic bound where heterogeneous storage is needed"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#trait-dyn-vs-generic
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Choose static dispatch (generics or `impl Trait`) versus dynamic dispatch (`dyn Trait`) deliberately: generics for hot, homogeneous code; `dyn` when you need heterogeneous storage, runtime plug-ins, or smaller binaries.

## Why
Generic bounds and `impl Trait` monomorphize: each concrete type gets a specialised, inlinable copy, at the cost of binary size. `dyn Trait` is a fat pointer dispatched through a vtable — a single code path with one indirection per call, and the only way to store heterogeneous values or return erased types. The wrong choice either leaves performance on the table or blocks heterogeneous collections entirely.

## Do
```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
struct Rect { w: f64, h: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}
impl Shape for Rect {
    fn area(&self) -> f64 { self.w * self.h }
}

fn total_area_generic<S: Shape>(shapes: &[S]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn total_area_dyn(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn demo() {
    let circles = [Circle { radius: 1.0 }, Circle { radius: 2.0 }];
    let _ = total_area_generic(&circles);

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rect { w: 3.0, h: 4.0 }),
    ];
    let _ = total_area_dyn(&shapes);
}
```

## Don't
```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn demo() {
    let circles: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Circle { radius: 2.0 }),
    ];
    let _ = total_area(&circles);
}
```

## Notes
Default to generics for a single known type and hot paths. Reach for `dyn` when you store heterogeneous values, register callbacks or plug-ins at runtime, or when many monomorphisations would bloat the binary. Keep the trait dyn-compatible if you want the `dyn` option.
