---
id: type-no-stringly
title: Replace stringly-typed APIs with enums and validated newtypes
category: types
severity: warn
tier: 1
enforceability: judge
tags: [enums, newtype, api-design]
applies_to:
  - "function taking a &str that only accepts a fixed set of values"
  - "match on a string with a catch-all panic arm"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#type-no-stringly
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Model a fixed set of values as an enum (or a validated newtype), not a `&str`, so typos and wrong formats fail at compile time instead of at runtime.

## Why
Strings accept any value, so `"Pending"`, `"aktive"`, or `"done"` all compile and only blow up when a `match` hits its catch-all panic arm. Enums make invalid inputs unrepresentable, give exhaustive `match` coverage, and bring IDE autocomplete and documentation along for free. Parse strings into typed values once, at the boundary.

## Do
```rust
enum Status {
    Pending,
    Active,
    Completed,
}

fn set_status(status: Status) {
    match status {
        Status::Pending => start_pending(),
        Status::Active => start_active(),
        Status::Completed => finish(),
    }
}

set_status(Status::Pending);
```

## Don't
```rust
fn set_status(status: &str) {
    match status {
        "pending" => start_pending(),
        "active" => start_active(),
        "completed" => finish(),
        _ => panic!("Unknown status"),
    }
}

set_status("Pending");
```

## Notes
Implement `FromStr` to parse external input into the enum at the boundary, then work with the typed value everywhere inside. `#[serde(rename_all = "snake_case")]` gives clean wire representations that are validated during deserialization.
