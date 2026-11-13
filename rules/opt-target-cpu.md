---
id: opt-target-cpu
title: Set target-cpu for known deployment hardware to unlock modern instructions
category: optimization
severity: info
tier: 1
enforceability: judge
tags: [performance, build-config, target-cpu, simd]
applies_to:
  - ".cargo/config.toml rustflags for a fixed deployment target"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#opt-target-cpu
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When you control the deployment hardware, set `-C target-cpu` (`native` for the build host, or a named CPU/`x86-64-v3` for a fixed target) instead of shipping the generic baseline.

## Why
By default rustc targets a decade-old generic baseline (roughly SSE2 on x86-64), leaving AVX2, FMA, BMI, and other modern instructions unused. Naming the actual CPU lets LLVM autovectorize with wide registers and emit better instruction selection, often a significant speedup for numeric code. The tradeoff: the binary now requires those features, so it will fault on older hardware.

## Do
```toml
# .cargo/config.toml — fixed deployment target
[build]
rustflags = ["-C", "target-cpu=x86-64-v3"]
```

## Don't
```toml
# .cargo/config.toml — ships the generic SSE2 baseline everywhere
[build]
rustflags = []
```

## Notes
Never use `target-cpu=native` for artifacts that run on other machines — build and deployment CPUs may differ, producing SIGILL crashes. For portable binaries, compile the baseline and gate wide paths behind `is_x86_feature_detected!` at runtime.
