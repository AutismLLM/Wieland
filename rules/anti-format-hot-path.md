---
id: anti-format-hot-path
title: Avoid format! in hot paths — reuse a buffer with write! instead
category: anti-patterns
severity: warn
tier: 2
enforceability: check
tags: [performance, allocation, hot-path]
applies_to:
  - "format! inside a loop or frequently called function"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['format!\(']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#anti-format-hot-path
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
In a hot path, build strings by clearing and reusing a single buffer with `write!` instead of allocating a fresh `String` per iteration with `format!`.

## Why
`format!()` allocates a new `String` on every call. Inside a loop or a frequently invoked function this is allocation churn that shows up in profiles. Reusing one pre-sized buffer with `std::fmt::Write::write!` turns N allocations into one.

## Do
```rust
use std::fmt::Write;

fn log_events(events: &[Event]) {
    let mut buffer = String::with_capacity(256);
    for event in events {
        buffer.clear();
        write!(buffer, "[{}] {}", event.level, event.message).unwrap();
        logger.log(&buffer);
    }
}
```

## Don't
```rust
fn log_events(events: &[Event]) {
    for event in events {
        let message = format!("[{}] {}", event.level, event.message);
        logger.log(&message);
    }
}
```

## Notes
`format!` is fine for one-off work: initialization, error messages, and debug output are not hot paths. Only reach for a reused buffer where profiling shows the allocation actually matters.
