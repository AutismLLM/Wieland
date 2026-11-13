---
id: type-newtype-ids
title: Wrap identifiers in distinct newtypes instead of raw integers
category: types
severity: info
tier: 1
enforceability: judge
tags: [newtype, ids, type-safety]
applies_to:
  - "function taking multiple id parameters of the same primitive"
  - "raw u64/String used as an identifier"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#type-newtype-ids
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Wrap each kind of identifier in its own newtype (e.g. `UserId(u64)`, `PostId(u64)`) so mixing them up is a compile error rather than a runtime bug.

## Why
Raw integers for IDs are interchangeable to the compiler, so passing a `post_id` where a `user_id` is expected compiles cleanly and fails at runtime. Distinct newtypes make each ID a separate type, catch swapped arguments during type-checking, and give the value a name and a place to hang helper methods.

## Do
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PostId(pub u64);

fn get_user_posts(user_id: UserId, post_id: PostId) -> Vec<Post> {
    lookup(user_id, post_id)
}

let posts = get_user_posts(UserId(1), PostId(42));
```

## Don't
```rust
fn get_user_posts(user_id: u64, post_id: u64) -> Vec<Post> {
    lookup(user_id, post_id)
}

let posts = get_user_posts(post_id, user_id);
```

## Notes
Derive the traits your keys need (`PartialEq`, `Eq`, `Hash` for map keys; `Ord` for `BTreeMap`). Use `#[serde(transparent)]` so the ID serializes as its inner value, and a small `define_id!` macro keeps a family of ID types consistent.
