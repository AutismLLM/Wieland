---
id: api-no-visible-nesting
title: Do not let generic type parameters visibly nest more than one level deep
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [api-design, generics, ergonomics]
applies_to:
  - "service-like struct field typed Service<Backend<Store>>"
  - "a type users must name that carries two or more levels of type parameters"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-SIMPLE-ABSTRACTIONS
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For service-like types users are expected to name, keep generic parameters from nesting more than one level deep — `Service<Backend>` is fine, `Service<Backend<Store>>` is not. Reserve deeper nesting for container or utility types users rarely write out by hand.

## Why
A type parameter that itself takes a type parameter compounds cognitive load: the user must resolve every layer's bounds just to write the type down, and error messages grow to match. Containers and iterator adapters earn that cost because they are generic by nature (`List<T>`, `Chain<A, B>`); a service type instantiated once per application does not, and nesting on top of nesting there is a sign the abstraction needs a concrete wrapper instead of another type parameter.

## Do
```rust
struct Backend;

struct Service<B = Backend> {
    backend: B,
}
```

## Don't
```rust
struct Store;
struct Backend<S> { store: S }

// Users of Service now have to write and reason about
// Service<Backend<Store>> just to name the type.
struct Service<B> {
    backend: B,
}

type MyService = Service<Backend<Store>>;
```

## Notes
Containers and smart pointers (`List<T>`, `Matrix4x4<f32>`) are expected exceptions since users must name their element type anyway. If a service type's users bring their own nested type as a single parameter — `Service<T>` where the caller chooses `T = Backend<Store>` — that is fine; the guideline targets nesting the service type itself introduces.
