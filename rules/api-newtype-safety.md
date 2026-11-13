---
id: api-newtype-safety
title: Wrap semantically distinct values in newtypes so the compiler catches mix-ups
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [newtype, type-safety, domain-modeling]
applies_to:
  - "function taking two arguments of the same primitive type"
  - "IDs, units, or validated strings held as raw primitives"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-newtype-safety
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Wrap values that share a primitive representation but differ in meaning — IDs, units, validated strings — in distinct newtypes so the type system refuses to interchange them.

## Why
Raw primitives carry no domain meaning, so a function taking `(u64, u64)` happily accepts its arguments in the wrong order and a timestamp passed where an ID was expected compiles cleanly and fails at runtime. Newtypes give each concept its own type, turning those swaps into compile errors, and they are zero-cost: a single-field wrapper has the same size and layout as the value it holds.

## Do
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GroupId(u64);

fn add_user_to_group(user: UserId, group: GroupId) {
    let _ = (user, group);
}

let user = UserId(100);
let group = GroupId(5);
add_user_to_group(user, group);
```

## Don't
```rust
fn add_user_to_group(user_id: u64, group_id: u64) {
    let _ = (user_id, group_id);
}

let user_id = 100u64;
let group_id = 5u64;
add_user_to_group(group_id, user_id);
```

## Notes
A newtype earns its keep only where confusion is possible. A one-off `struct X(i32)` used in a single place adds ceremony without preventing any real mistake — reach for it when values could be transposed, when units must not mix, or when a value needs validation on construction.
