---
id: perf-profile-first
title: Profile before optimizing; measure the real bottleneck
category: performance
severity: info
tier: 1
enforceability: judge
tags: [performance, profiling, methodology]
applies_to:
  - "an optimization made on a hunch without measurement"
  - "#[inline(always)] or micro-tuning on rarely-called code"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#perf-profile-first
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Profile under realistic load to find the actual hot spot before changing code for speed, then optimize that one thing and re-measure.

## Why
Intuition about performance is routinely wrong: the code you assume is slow often isn't, while the real cost hides somewhere unexpected. A flamegraph or time profiler shows exactly where cycles go, so you spend effort where it moves the needle instead of hardening code that runs once at startup. The biggest wins usually come from algorithm and data-structure changes the profile reveals, not from scattered low-level tweaks.

## Do
```rust
fn process(data: &[Item]) -> Vec<Output> {
    let cloned: Vec<_> = data.iter().cloned().collect();
    cloned.par_iter()
        .map(|x| expensive_computation(x))
        .collect()
}
```

## Don't
```rust
fn process(data: &[Item]) -> Vec<Output> {
    let cloned: Vec<_> = data.iter().cloned().collect();
    cloned.iter().map(|x| expensive_computation(x)).collect()
}

#[inline(always)]
fn rarely_called() {
}
```

## Notes
Reach for `cargo flamegraph`, `perf`, DHAT for heap allocations, and `criterion` for microbenchmarks. The workflow is: correct code first, benchmark the hot path, profile, fix one bottleneck, measure the gain, repeat.
