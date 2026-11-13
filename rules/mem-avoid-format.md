---
id: mem-avoid-format
title: Avoid format! when a string literal or &'static str works
category: memory
severity: warn
tier: 2
enforceability: gate
tags: [performance, allocation, strings]
applies_to:
  - "format! with a single string literal and no arguments"
  - "function returning String for constant text"
rust_edition: "2024"
msrv: null
detect:
  clippy: [useless_format]
  grep: ['format!\("[^"{}]*"\)']
  ast_query: null
autofix: true
sources:
  - leonardomso/rust-skills#mem-avoid-format
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Do not call `format!()` to produce constant text; return a `&'static str` (or `Cow<'static, str>` for mixed cases) instead.

## Why
`format!()` always allocates a fresh `String`, even when the content is a fixed literal with no interpolation. In hot paths and loops those allocations accumulate for no benefit. A string literal or `&'static str` return costs nothing, and clippy's `useless_format` flags the no-argument case directly.

## Do
```rust
fn get_error_message() -> &'static str {
    "An error occurred"
}

use std::borrow::Cow;

fn classify(n: i32) -> Cow<'static, str> {
    if n > 0 {
        Cow::Borrowed("positive")
    } else if n < 0 {
        Cow::Borrowed("negative")
    } else {
        Cow::Borrowed("zero")
    }
}
```

## Don't
```rust
fn get_error_message() -> String {
    format!("An error occurred")
}

fn classify(n: i32) -> String {
    if n > 0 {
        format!("positive")
    } else if n < 0 {
        format!("negative")
    } else {
        format!("zero")
    }
}
```

## Notes
`format!()` is fine on cold paths and whenever you genuinely need an owned `String` built from dynamic parts, such as `format!("Hello, {name}!")`.
