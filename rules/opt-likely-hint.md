---
id: opt-likely-hint
title: Structure branches so the hot path falls through
category: optimization
severity: info
tier: 1
enforceability: judge
tags: [performance, branch-prediction, hot-path, control-flow]
applies_to:
  - "hot conditionals where one branch dominates"
  - "match arm ordering in hot code"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#opt-likely-hint
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
On stable Rust, guide branch prediction through structure — early-return the rare cases, put the common variant first, and extract cold branches — rather than reaching for nightly `likely`/`unlikely` intrinsics.

## Why
A mispredicted branch stalls the pipeline for 10-20 cycles, so telling the compiler which path is common pays off in hot code. The compiler already treats early returns as unlikely and lays out fall-through code as hot, so structuring the function does the job on stable without feature flags. `std::hint::cold_path()` (stable since 1.95) is available inside the rare branch when you need an explicit marker.

## Do
```rust
fn process(data: Option<&Data>) -> i32 {
    let data = match data {
        None => return 0,
        Some(d) => d,
    };
    complex_processing(data)
}
```

## Don't
```rust
#![feature(likely_unlikely)]
use std::hint::unlikely;

fn process(data: Option<&Data>) -> i32 {
    if unlikely(data.is_none()) {
        return 0;
    }
    complex_processing(data.unwrap())
}
```

## Notes
Only reorder against real profile data — guessing which branch is hot can make prediction worse. `likely`/`unlikely` intrinsics remain nightly-only as of Rust 1.96; do not gate a stable crate on them.
