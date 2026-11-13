---
id: perf-release-profile
title: Tune the release profile with LTO and codegen-units for production
category: performance
severity: info
tier: 1
enforceability: judge
tags: [performance, cargo, build]
applies_to:
  - "Cargo.toml [profile.release] for a production binary"
  - "shipping a release build without LTO tuning"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#perf-release-profile
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For production binaries, tune `[profile.release]` — enable `lto`, set `codegen-units = 1`, and add `strip`/`panic` as appropriate.

## Why
The default release profile favors compile speed over runtime speed: no LTO and 16 codegen units. Turning on full LTO and collapsing to one codegen unit lets the optimizer inline and specialize across the whole crate graph, which can buy 10-40% on some workloads. The tradeoff is longer compile times, so reserve the aggressive profile for the artifacts you actually ship.

## Do
```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
```

## Don't
```toml
[profile.release]
opt-level = 3
lto = false
codegen-units = 16
```

## Notes
`lto = "thin"` is a faster middle ground when full LTO compiles too slowly. Keep a separate profile (`inherits = "release"`, `debug = true`, `strip = false`) for profiling builds so flamegraphs still have symbols, and use `opt-level = "z"`/`"s"` when binary size matters more than speed.
