---
id: anti-premature-optimize
title: Profile before optimizing; keep code simple until data says otherwise
category: anti-patterns
severity: info
tier: 1
enforceability: judge
tags: [design, performance, simplicity]
applies_to:
  - "unsafe or hand-rolled code added for speed without benchmarks"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#anti-premature-optimize
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Write correct, idiomatic code first; optimize only the hotspots a profiler identifies, and verify each change with a measurement.

## Why
Premature optimization wastes effort, complicates code, and usually targets the wrong bottleneck — most code is not performance-critical, and only the hot 10% matters. Reaching for `unsafe`, custom data structures, or speculative caching before profiling adds risk and complexity for no measured gain, and often loses to what the optimizer already does.

## Do
```rust
fn sum(data: &[i32]) -> i32 {
    data.iter().sum()
}
```

## Don't
```rust
fn sum(data: &[i32]) -> i32 {
    unsafe {
        let mut sum = 0;
        for i in 0..data.len() {
            sum += *data.get_unchecked(i);
        }
        sum
    }
}
```

## Notes
When profiling does point at a hotspot, optimize one thing at a time and re-measure to confirm the improvement. Where a hand-tuned path is justified, document the benchmark numbers that justify it so the tradeoff stays honest.
