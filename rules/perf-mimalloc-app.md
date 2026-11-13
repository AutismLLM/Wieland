---
id: perf-mimalloc-app
title: Set mimalloc as the global allocator in applications
category: performance
severity: info
tier: 2
enforceability: check
tags: [performance, allocation, applications, global-allocator]
applies_to:
  - "binary crate main.rs"
  - "allocation-heavy application workloads"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['#\[global_allocator\]']
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-MIMALLOC-APPS
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Applications should set `mimalloc` as the global allocator instead of relying on the platform default (glibc malloc / system allocator).

## Why
The system allocator on most platforms is a general-purpose allocator not tuned for the allocation patterns typical of Rust applications. `mimalloc` is a drop-in replacement that is usually faster along allocation-heavy hot paths, at the cost of one extra dependency and a small binary size increase. This is application-only: libraries must never set the global allocator, since only the final binary gets to choose it.

## Do
```toml
# Cargo.toml
[dependencies]
mimalloc = "0.1"
```
```rust
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    run_app();
}
```

## Don't
```rust
fn main() {
    run_app();
}
```

## Notes
Only set this in the final binary crate, never in a library — a library that installs a `#[global_allocator]` forces that choice on every downstream consumer. Benchmark on your real workload before committing; gains are largest for allocation-heavy hot paths and can be negligible for I/O-bound or allocation-light applications.
