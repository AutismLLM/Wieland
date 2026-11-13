---
id: mem-shrink-to-fit
title: Shrink long-lived collections to fit after incremental building
category: memory
severity: info
tier: 1
enforceability: judge
tags: [performance, allocation, memory-footprint]
applies_to:
  - "Vec or String built via repeated push in a loop then stored long-term"
  - "large collection assembled without with_capacity"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-SHRINK-TO-FIT
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a `Vec`, `String`, or similar collection is built incrementally without an exact `with_capacity` and then stored for the long term, call `shrink_to_fit()` once before storing it.

## Why
Growable collections double their buffer when they run out of room, so a collection built by repeated `push` can end up using up to roughly twice the memory it actually needs. That slack is invisible until the collection is long-lived and multiplied across many instances. `shrink_to_fit()` reallocates once to the exact length, trading a single copy for a smaller permanent footprint.

## Do
```rust
fn build_index(entries: &[Entry]) -> Vec<Summary> {
    let mut index = Vec::new();
    for entry in entries {
        if entry.matches() {
            index.push(entry.summarize());
        }
    }
    index.shrink_to_fit();
    index
}
```

## Don't
```rust
fn build_index(entries: &[Entry]) -> Vec<Summary> {
    let mut index = Vec::new();
    for entry in entries {
        if entry.matches() {
            index.push(entry.summarize());
        }
    }
    index
}
```

## Notes
Skip this for short-lived buffers reused across loop iterations — shrinking there would defeat the point of reuse. Also unnecessary when the final size was known upfront via `with_capacity`, or when converting to `Box<[T]>`/`Box<str>`, since that conversion already shrinks as part of the transformation.
