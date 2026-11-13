---
id: own-cow-conditional
title: Use Cow<'a, T> when you usually borrow but sometimes must own
category: ownership
severity: info
tier: 1
enforceability: judge
tags: [cow, allocation, conditional-ownership]
applies_to:
  - "function that returns input unchanged most of the time, modified rarely"
  - "returning either a static string or a formatted one"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#own-cow-conditional
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Return or accept `Cow<'a, T>` when a value is usually borrowed and only occasionally needs to be owned, so you allocate only on the path that actually mutates.

## Why
`Cow` holds either a `Borrowed` reference or an `Owned` value and clones only when ownership is forced. Returning a plain `String` from a "maybe transform" function allocates even on the common pass-through path. `Cow` keeps that path zero-cost while still letting the rare case produce an owned value.

## Do
```rust
use std::borrow::Cow;

fn normalize_path(path: &str) -> Cow<'_, str> {
    if path.contains("//") {
        Cow::Owned(path.replace("//", "/"))
    } else {
        Cow::Borrowed(path)
    }
}
```

## Don't
```rust
fn normalize_path(path: &str) -> String {
    if path.contains("//") {
        path.replace("//", "/")
    } else {
        path.to_string()
    }
}
```

## Notes
Skip `Cow` when you always need an owned value (just return the owned type) or always borrow (return a reference). Its payoff is the conditional case on a hot path.
