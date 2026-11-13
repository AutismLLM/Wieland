---
id: mem-write-over-format
title: Write into an existing buffer with write! instead of push_str(&format!(...))
category: memory
severity: warn
tier: 2
enforceability: gate
tags: [performance, allocation, strings]
applies_to:
  - "push_str(&format!(...)) appending to a String"
  - "format! inside a loop that fills a buffer"
rust_edition: "2024"
msrv: null
detect:
  clippy: [format_push_string]
  grep: ['push_str\(&format!']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-write-over-format
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When appending formatted text to a buffer, use `write!` into the buffer directly rather than allocating an intermediate `String` with `format!`.

## Why
`format!()` allocates a fresh `String` on every call, so building output by pushing `&format!(...)` in a loop allocates once per iteration and immediately throws each allocation away. `write!` (via `std::fmt::Write` for `String` or `std::io::Write` for `Vec<u8>`) formats straight into the existing buffer and reuses its capacity. Clippy's `format_push_string` flags the `push_str(&format!(...))` anti-pattern.

## Do
```rust
use std::fmt::Write;

fn build_response(items: &[Item]) -> String {
    let mut result = String::with_capacity(items.len() * 64);
    for item in items {
        write!(&mut result, "{}: {}\n", item.name, item.value).unwrap();
    }
    result
}
```

## Don't
```rust
fn build_response(items: &[Item]) -> String {
    let mut result = String::new();
    for item in items {
        result.push_str(&format!("{}: {}\n", item.name, item.value));
    }
    result
}
```

## Notes
`format!()` remains the right choice for one-off formatting outside a loop and for a function that must return a freshly owned `String`, since it cannot borrow a local buffer.
