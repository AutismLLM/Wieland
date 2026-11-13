---
id: pat-matches-macro
title: Use matches!() for boolean pattern tests
category: patterns
severity: warn
tier: 2
enforceability: gate
tags: [pattern-matching, readability, clippy]
applies_to:
  - "match arm returning true/false with a _ => false fallback"
  - "is_/has_/can_ predicate implemented as a verbose match"
rust_edition: "2024"
msrv: null
detect:
  clippy: [match_like_matches_macro]
  grep: []
  ast_query: null
autofix: true
sources:
  - leonardomso/rust-skills#pat-matches-macro
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `matches!(value, Pattern)` for a boolean pattern test instead of a `match` that returns `true`/`false`.

## Why
`matches!(value, Pattern)` is the idiomatic way to ask "does this value match a pattern?" in a boolean context. Spelling it out as a `match` returning `true`/`false` is noise that hides the intent, and Clippy flags it with `clippy::match_like_matches_macro`. The macro accepts optional guards and `|` alternation, so range checks, filtered `Option` tests, and multi-variant checks all stay on one line.

## Do
```rust
enum Status {
    Active,
    Pending,
    Closed,
}

fn is_active(s: &Status) -> bool {
    matches!(s, Status::Active)
}

fn is_terminal(s: &Status) -> bool {
    matches!(s, Status::Closed | Status::Pending)
}

fn is_positive(opt: Option<i32>) -> bool {
    matches!(opt, Some(v) if v > 0)
}
```

## Don't
```rust
enum Status {
    Active,
    Pending,
    Closed,
}

fn is_active(s: &Status) -> bool {
    match s {
        Status::Active => true,
        _ => false,
    }
}

fn is_positive(opt: Option<i32>) -> bool {
    match opt {
        Some(v) if v > 0 => true,
        _ => false,
    }
}
```

## Notes
When you need to bind the matched value or branch on it further, a real `match` or `if let` is still the right tool. In tests, prefer `assert_matches!` over `assert!(matches!(...))` for a better failure message.
