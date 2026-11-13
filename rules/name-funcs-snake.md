---
id: name-funcs-snake
title: Use snake_case for functions, methods, variables, and modules
category: naming
severity: warn
tier: 2
enforceability: check
tags: [naming, functions, casing]
applies_to:
  - "function, method, local variable, and module declarations"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['fn [a-z][a-zA-Z0-9]*[A-Z]']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-funcs-snake
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Name every function, method, local variable, and module in `snake_case`.

## Why
Rust reserves `snake_case` for value-level names and `UpperCamelCase` for types; the split lets a reader tell at a glance whether `HttpClient` is a type and `send_request` a function. The compiler warns on `camelCase` names, so any other casing is both non-idiomatic and noisy.

## Do
```rust
struct User {
    name: String,
}

impl User {
    fn full_name(&self) -> String {
        self.name.clone()
    }
}

fn calculate_total(items: &[f64]) -> f64 {
    items.iter().sum()
}
```

## Don't
```rust
struct User {
    name: String,
}

impl User {
    fn fullName(&self) -> String {
        self.name.clone()
    }
}

fn calculateTotal(items: &[f64]) -> f64 {
    items.iter().sum()
}
```

## Notes
Acronyms become lowercase inside `snake_case`: `parse_json`, `connect_tcp`, `generate_uuid`.
