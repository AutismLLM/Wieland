---
id: opt-lto-release
title: Enable LTO in the release profile for cross-crate optimization
category: optimization
severity: info
tier: 1
enforceability: judge
tags: [performance, build-config, release-profile, lto]
applies_to:
  - "[profile.release] in Cargo.toml for a performance-critical binary"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#opt-lto-release
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For release binaries where runtime matters, enable link-time optimization (`lto = "thin"` or `"fat"`) so the optimizer works across crate boundaries.

## Why
Normal compilation optimizes each crate in isolation; LTO defers optimization to link time so the compiler can inline across crates, eliminate dead code globally, and devirtualize calls. `thin` LTO captures most of the win with a modest compile-time cost, while `fat` squeezes out the last few percent for the slowest build — together often 5-20% faster runtime.

## Do
```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
```

## Don't
```toml
[profile.release]
opt-level = 3
# no lto — misses all cross-crate optimization
```

## Notes
Leave LTO off for development (fast compiles) and for library crates on crates.io, since downstream users choose their own profile. `thin` is the pragmatic default for CI; reserve `fat` for shipped release artifacts.
