---
id: pat-if-let-chains
title: Use if let chains to combine pattern bindings and conditions
category: patterns
severity: info
tier: 1
enforceability: judge
tags: [pattern-matching, readability, edition-2024]
applies_to:
  - "nested if let where the inner body is the happy path"
  - "multiple optional bindings guarding a single action"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#pat-if-let-chains
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Combine multiple pattern bindings and boolean conditions into one `if let` chain with `&&` instead of nesting a separate `if let` for each.

## Why
If-let chains (stabilized in Rust 1.88 under the 2024 edition) let a single `if` header bind multiple patterns and test arbitrary boolean conditions, all joined by `&&`. Without chains, each additional binding adds a nesting level, pushing the happy-path body further right and forcing the reader to track multiple scopes. Short-circuit semantics still hold, so later clauses run only if earlier ones succeed.

## Do
```rust
fn handle(input: Option<String>, limit: Option<u32>) -> Option<String> {
    if let Some(s) = input
        && let Ok(n) = s.trim().parse::<u32>()
        && let Some(max) = limit
        && n <= max
    {
        return Some(format!("valid: {n}"));
    }
    None
}
```

## Don't
```rust
fn handle(input: Option<String>, limit: Option<u32>) -> Option<String> {
    if let Some(s) = input {
        if let Ok(n) = s.trim().parse::<u32>() {
            if let Some(max) = limit {
                if n <= max {
                    return Some(format!("valid: {n}"));
                }
            }
        }
    }
    None
}
```

## Notes
Requires the 2024 edition. Prefer `let ... else` when the goal is an early return on failure; reach for a chain when a richer condition combines several optional bindings before running a positive body.
