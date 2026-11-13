---
id: own-lifetime-elision
title: Rely on lifetime elision; write explicit lifetimes only when required
category: ownership
severity: warn
tier: 2
enforceability: gate
tags: [lifetimes, elision, readability]
applies_to:
  - "fn with a single reference parameter and a reference return"
  - "impl block or method with elidable lifetime annotations"
rust_edition: "2024"
msrv: null
detect:
  clippy: [needless_lifetimes]
  grep: []
  ast_query: null
autofix: true
sources:
  - leonardomso/rust-skills#own-lifetime-elision
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Let the lifetime elision rules infer lifetimes; add explicit named lifetimes only where elision genuinely cannot resolve the relationship.

## Why
The three elision rules cover the overwhelmingly common cases — one input reference, or a method taking `&self` — so annotating them adds noise without adding meaning. `clippy::needless_lifetimes` flags the redundant annotations and `clippy --fix` removes them. Keep explicit lifetimes for the cases that truly need them: multiple inputs feeding one output, structs holding references, or a required `'static`.

## Do
```rust
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

## Don't
```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap_or("")
}
```

## Notes
Use the anonymous lifetime `'_` when a type has a lifetime you want to acknowledge but not name, e.g. `impl Display for Wrapper<'_>`.
