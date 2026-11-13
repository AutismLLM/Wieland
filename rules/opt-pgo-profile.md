---
id: opt-pgo-profile
title: Use profile-guided optimization for the last mile of performance
category: optimization
severity: info
tier: 1
enforceability: judge
tags: [performance, build-config, pgo, release]
applies_to:
  - "performance-critical binaries with a stable, representative workload"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#opt-pgo-profile
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For a binary with a stable, representative workload, apply profile-guided optimization: build instrumented, run real workloads, then rebuild with the collected profile.

## Why
PGO feeds real runtime behavior back into the compiler so it optimizes the paths that are actually hot and deprioritizes the ones that are not — often 10-30% beyond ordinary `-O3` plus LTO. The catch is the three-step pipeline and the need for a workload that reflects production; a profile from artificial microbenchmarks can misdirect the optimizer.

## Do
```bash
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release
./target/release/my_app < typical_workload.txt
llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release
```

## Don't
```bash
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release
for _ in $(seq 1000000); do ./target/release/my_app --tiny-op; done
llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release
```

## Notes
Skip PGO for development builds, quick iteration, and libraries (downstream users profile their own workload). It pays off only with a genuinely representative profiling corpus and a workload that stays stable across releases.
