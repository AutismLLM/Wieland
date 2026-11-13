---
id: type-option-nullable
title: Represent maybe-absent values with Option, not sentinels
category: types
severity: warn
tier: 0
enforceability: judge
tags: [option, absence, null-safety]
applies_to:
  - "function returning an empty/default value on 'not found'"
  - "sentinel or nullable pointer for missing data"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#type-option-nullable
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `Option<T>` to model "value or nothing" so the compiler forces the absent case to be handled, instead of returning an empty default or a sentinel.

## Why
Sentinel values and empty defaults let a caller silently use a missing value — printing an empty user, dereferencing a null pointer — with no compiler help. `Option<T>` makes absence explicit in the type: you cannot reach the inner value without acknowledging `None`, which eliminates a whole class of null-pointer bugs at compile time.

## Do
```rust
fn find_user(id: u64) -> Option<User> {
    users().get(&id).cloned()
}

match find_user(42) {
    Some(u) => println!("{}", u.name),
    None => println!("User not found"),
}
```

## Don't
```rust
fn find_user(id: u64) -> User {
    users().get(&id).cloned().unwrap_or(User::empty())
}

let user = find_user(42);
println!("{}", user.name);
```

## Notes
Combinators keep `Option` ergonomic: `map`, `and_then`, `unwrap_or_else`, `as_deref`, and the `?` operator for propagation. Reach for `Result<T, E>` instead when the absence carries an error reason; convert with `ok_or(...)` or `.ok()`.
