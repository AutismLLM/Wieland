---
id: name-to-expensive
title: Use the to_ prefix for expensive, allocating conversions
category: naming
severity: info
tier: 1
enforceability: judge
tags: [naming, conversion, cost]
applies_to:
  - "methods that convert by allocating, cloning, or computing"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-to-expensive
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Prefix a conversion that allocates, clones, or computes with `to_`, distinguishing it from the free `as_` and the consuming `into_`.

## Why
The prefix is a cost signal: seeing `to_string` or `to_vec`, a caller knows the call allocates and may want to hoist or cache it. Hiding an allocating conversion behind `as_` misleads readers into treating it as free and invites needless repeated work in hot paths.

## Do
```rust
struct Email(String);

impl Email {
    fn as_str(&self) -> &str {
        &self.0
    }

    fn to_lowercase(&self) -> Email {
        Email(self.0.to_lowercase())
    }
}
```

## Don't
```rust
struct Email(String);

impl Email {
    fn as_lowercase(&self) -> Email {
        Email(self.0.to_lowercase())
    }
}
```

## Notes
`to_owned` is the canonical `to_` method for turning borrowed data into an owned copy (`&str` to `String`, `&[T]` to `Vec<T>`).
