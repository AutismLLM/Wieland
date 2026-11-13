---
id: opt-inline-never-cold
title: Keep error paths out of hot code with #[inline(never)] and #[cold]
category: optimization
severity: info
tier: 1
enforceability: judge
tags: [performance, inlining, error-paths, attributes]
applies_to:
  - "error construction inlined into a hot function"
  - "#[inline(never)] / #[cold] helpers"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#opt-inline-never-cold
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Move error construction and other rarely-run code into `#[cold] #[inline(never)]` helpers so it does not bloat the hot function it guards.

## Why
Inlining a fat error path — string formatting, backtrace capture, `Vec` of suggestions — into a hot function wastes instruction-cache space and can block optimizations on the common path. `#[inline(never)]` keeps that code in its own function, and `#[cold]` marks the branch unlikely so the compiler lays out and predicts against it. The hot path stays small and fast.

## Do
```rust
fn process_data(data: &[u8]) -> Result<Output, Error> {
    if data.is_empty() {
        return Err(empty_data_error());
    }
    do_processing(data)
}

#[cold]
#[inline(never)]
fn empty_data_error() -> Error {
    Error::Empty {
        context: format!("expected data, got empty slice"),
        suggestions: vec!["check input", "validate before calling"],
    }
}
```

## Don't
```rust
fn process_data(data: &[u8]) -> Result<Output, Error> {
    if data.is_empty() {
        return Err(Error::Empty {
            context: format!("expected data, got empty slice"),
            suggestions: vec!["check input", "validate before calling"],
        });
    }
    do_processing(data)
}
```

## Notes
On stable Rust, an early return already hints "unlikely" to the compiler, so extraction matters most when the error body is large. Skip it for trivial error values where the construction cost is negligible.
