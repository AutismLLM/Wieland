---
id: type-deref-coercion
title: Implement Deref/DerefMut only for smart pointers and transparent wrappers
category: types
severity: info
tier: 1
enforceability: judge
tags: [traits, deref, api-design]
applies_to:
  - "impl Deref for a domain type"
  - "newtype wrapping a struct"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#type-deref-coercion
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Implement `Deref`/`DerefMut` only when your type is a smart pointer or a transparent wrapper for its target, never as an OOP-style inheritance mechanism.

## Why
`Deref` coercions make `Box<T>`, `Arc<T>`, `String`, and `Vec<T>` ergonomic by surfacing the inner type's methods transparently. Using `Deref` to "inherit" the API of a wrapped domain type pollutes method resolution and hides what the wrapper owns versus borrows. Adding a method to the target then silently changes every wrapper that derefs to it, making refactors hazardous.

## Do
```rust
struct MyBox<T>(T);

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
```

## Don't
```rust
struct User {
    name: String,
    email: String,
}

struct AdminUser(User);

impl std::ops::Deref for AdminUser {
    type Target = User;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn greet(admin: &AdminUser) {
    println!("hello, {}", admin.name);
}
```

## Notes
Legitimate targets: pointer indirection (`Box`/`Rc`/`Arc`), owned-to-borrowed containers (`String` -> `str`, `Vec<T>` -> `[T]`), RAII guards (`MutexGuard<T>` -> `T`), and newtypes whose entire purpose is "a `T` with extra invariants". For domain types, expose only the methods you intend, explicitly.
