---
id: anti-stringly-typed
title: Use enums and newtypes instead of stringly-typed values
category: anti-patterns
severity: info
tier: 1
enforceability: judge
tags: [type-safety, api-design, correctness]
applies_to:
  - "a &str parameter with a fixed set of valid values"
  - "String fields that carry semantic meaning like ids or emails"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#anti-stringly-typed
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Model a fixed set of values as an enum and a validated semantic value as a newtype, instead of passing raw `String`/`&str`.

## Why
Strings accept any value, validate nothing, and give no IDE support — "pending" vs "Pending" vs a typo all compile. Enums make the match exhaustive and let the compiler catch missing or swapped cases; newtypes make illegal values unrepresentable and stop one string id being confused for another. Mistakes move from runtime panics to compile errors.

## Do
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrderStatus {
    Pending,
    Processing,
    Completed,
    Cancelled,
}

fn process_order(status: OrderStatus) {
    match status {
        OrderStatus::Pending => {}
        OrderStatus::Processing => {}
        OrderStatus::Completed => {}
        OrderStatus::Cancelled => {}
    }
}
```

## Don't
```rust
fn process_order(status: &str) {
    match status {
        "pending" => {}
        "completed" => {}
        _ => panic!("unknown status"),
    }
}

process_order("complete");
```

## Notes
Parse strings into these types once at the boundary — implement `FromStr` or use serde's `rename_all` — and work with the typed values everywhere inside. Add a `Display` impl when you need to render them back to text.
