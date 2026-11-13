---
id: pat-let-else
title: Use let-else for early-return pattern extraction
category: patterns
severity: warn
tier: 2
enforceability: gate
tags: [readability, control-flow, ergonomics]
applies_to:
  - "nested if let with else return"
  - "rightward drift from guard clauses"
rust_edition: "2024"
msrv: null
detect:
  clippy: [manual_let_else]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#pat-let-else
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `let ... else { ... }` to bind a pattern on the happy path and diverge in the `else` branch, instead of nesting `if let` blocks.

## Why
`let ... else` (stable since 1.65) binds the pattern in the success path and requires the `else` block to diverge via `return`, `continue`, `break`, or a macro like `panic!`/`bail!`. This keeps the happy path at the top indentation level and eliminates the rightward drift that accumulates when several `if let` guards nest inside one another. The bound variable stays in scope for the rest of the function.

## Do
```rust
fn process(input: Option<String>) -> Option<u32> {
    let Some(s) = input else { return None; };
    let Ok(n) = s.trim().parse::<u32>() else { return None; };
    if n == 0 {
        return None;
    }
    Some(n * 2)
}
```

## Don't
```rust
fn process(input: Option<String>) -> Option<u32> {
    if let Some(s) = input {
        if let Ok(n) = s.trim().parse::<u32>() {
            if n > 0 {
                return Some(n * 2);
            } else {
                return None;
            }
        } else {
            return None;
        }
    } else {
        return None;
    }
}
```

## Notes
The bound variable is in scope after the `let`, not inside the `else` block. Prefer `?` when the `else` branch would merely propagate an error; `let ... else` shines when the divergence is a `return`, `continue`, or `break`. The `else` block accepts any diverging expression, including `bail!("...")`.
