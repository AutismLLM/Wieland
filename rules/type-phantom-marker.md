---
id: type-phantom-marker
title: Express type relationships with PhantomData at zero runtime cost
category: types
severity: info
tier: 1
enforceability: judge
tags: [phantomdata, generics, variance]
applies_to:
  - "generic type whose parameter appears in no field"
  - "typestate or FFI handle parameterized by a marker type"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#type-phantom-marker
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a type must be parameterized by a type it does not store, add a `PhantomData<T>` field to record the association without any runtime cost.

## Why
A generic parameter that appears in no field is a compile error, and faking a real field wastes memory and adds trait requirements. `PhantomData<T>` is zero-sized: it tells the compiler the type is "associated with" `T` for the purposes of variance, drop-checking, and semantic distinctness, so `Handle<User>` and `Handle<Order>` stay incompatible while occupying no extra space.

## Do
```rust
use std::marker::PhantomData;

struct Handle<T> {
    id: u64,
    _marker: PhantomData<T>,
}

impl<T> Handle<T> {
    fn new(id: u64) -> Self {
        Handle { id, _marker: PhantomData }
    }
}

struct User;
struct Order;

fn process_user(_h: Handle<User>) {}

let user_handle = Handle::<User>::new(1);
process_user(user_handle);
```

## Don't
```rust
struct Handle<T> {
    id: u64,
    _type: Option<T>,
}
```

## Notes
The chosen `PhantomData` form encodes variance: `PhantomData<T>` is covariant, `PhantomData<fn(T)>` contravariant, and `PhantomData<fn(T) -> T>` invariant. It also powers typestate machines (`Door<Locked>` vs `Door<Unlocked>`) and type-safe FFI handles.
