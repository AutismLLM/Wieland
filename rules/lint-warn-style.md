---
id: lint-warn-style
title: Enable clippy::style to keep code on the idiomatic path
category: lints
severity: warn
tier: 2
enforceability: gate
tags: [clippy, style, idiomatic, readability]
applies_to:
  - "len() == 0 instead of is_empty()"
  - "explicit return at end of function"
  - "match where if let or ? would read better"
rust_edition: "2024"
msrv: null
detect:
  clippy: [len_zero, redundant_field_names, needless_return, question_mark, single_match, match_like_matches_macro, collapsible_if]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#lint-warn-style
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Enable the `clippy::style` group so code follows idiomatic Rust patterns that reviewers and tooling expect.

## Why
Style violations are not bugs, but they slow every reader who has to translate `len() == 0` into "empty" or trace a `match` that a `?` would express in one character. The style group nudges code onto the shared idiom — `is_empty()`, `if let`, `matches!`, no trailing `return` — so the whole codebase reads the same way and reviews stay about substance.

## Do
```rust
fn process(data: &[i32]) -> Option<i32> {
    if data.is_empty() {
        return None;
    }
    let first = data.first()?;
    Some(*first)
}
```

## Don't
```rust
fn process(data: Vec<i32>) -> Option<i32> {
    if data.len() == 0 {
        return None;
    }
    let first = match data.first() {
        Some(x) => x,
        None => return None,
    };
    return Some(*first);
}
```

## Notes
A few style lints are genuine team preferences — if you deliberately use explicit `return` for early-exit clarity, `#[allow(clippy::needless_return)]` at the site rather than disabling the whole group.
