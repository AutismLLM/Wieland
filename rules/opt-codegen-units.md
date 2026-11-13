---
id: opt-codegen-units
title: Set codegen-units = 1 in the release profile for max optimization
category: optimization
severity: info
tier: 1
enforceability: judge
tags: [performance, build-config, release-profile]
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
  - leonardomso/rust-skills#opt-codegen-units
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For performance-critical release binaries, set `codegen-units = 1` so LLVM optimizes across the whole crate instead of within parallel fragments.

## Why
Cargo defaults to 16 codegen units so it can compile a crate's fragments in parallel, but that boundary blocks cross-unit inlining, dead-code elimination, and constant propagation. Collapsing to a single unit lets LLVM see the whole crate at once, typically buying 5-20% runtime at the cost of a slower, serial compile. Reserve it for final release and benchmark builds.

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
# codegen-units defaults to 16 — fast build, misses cross-unit optimization
```

## Notes
Keep the default (or higher) for `dev` and CI profiles where compile time matters more than the last few percent of runtime; the serial build can take minutes on a large crate.
