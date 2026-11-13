---
id: name-no-get-prefix
title: Omit the get_ prefix on simple getters
category: naming
severity: warn
tier: 2
enforceability: check
tags: [naming, getters, api]
applies_to:
  - "accessor methods that return a field"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['fn get_[a-z_]+\(&self\)']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-no-get-prefix
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Name a simple field accessor after the field itself (`name()`, `len()`), not with a `get_` prefix.

## Why
Rust convention keeps the common case concise: `user.name()` reads better than `user.get_name()`, and the standard library follows it everywhere (`String::len`, `Path::file_name`). The `get` name is reserved for fallible or computed access that returns an `Option`, so a bare `get_` on a plain field is both verbose and slightly misleading.

## Do
```rust
struct User {
    name: String,
    age: u32,
}

impl User {
    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }
}
```

## Don't
```rust
struct User {
    name: String,
    age: u32,
}

impl User {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}
```

## Notes
`get` is right when the method does more than field access, like `HashMap::get` and `Vec::get`, which return `Option<&T>` after a bounds-checked lookup.
