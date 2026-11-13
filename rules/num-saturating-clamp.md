---
id: num-saturating-clamp
title: Bound values with `clamp` and saturating arithmetic instead of min/max chains
category: numerics
severity: warn
tier: 2
enforceability: gate
tags: [clamp, bounds, saturating]
applies_to:
  - "manual if/min/max chain that bounds a value to a range"
  - "range-limiting numeric operation"
rust_edition: "2024"
msrv: null
detect:
  clippy: [manual_clamp]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#num-saturating-clamp
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Express range constraints with `Ord::clamp(min, max)` and stop arithmetic at type limits with `saturating_*`, rather than hand-rolled `if`/`min`/`max` chains.

## Why
Constraining a value to a range is common, but hand-written `if`/`min`/`max` chains are verbose and easy to mis-order, especially when mixing signed and unsigned types. `clamp` states the intent in one call and is available on every `Ord` type and on `f32`/`f64`. When you also need arithmetic that stops at the type's limits instead of panicking or wrapping, compose `clamp` with `saturating_*` methods.

## Do
```rust
fn apply_damage(health: i32, damage: i32) -> i32 {
    health.saturating_sub(damage).clamp(0, i32::MAX)
}

fn clamp_volume(vol: u8, min: u8, max: u8) -> u8 {
    vol.clamp(min, max)
}

fn normalize_alpha(a: f32) -> f32 {
    a.clamp(0.0, 1.0)
}
```

## Don't
```rust
fn apply_damage(health: i32, damage: i32) -> i32 {
    let result = health - damage;
    if result < 0 { 0 } else { result }
}

fn clamp_volume(vol: u8, min: u8, max: u8) -> u8 {
    if vol < min {
        min
    } else if vol > max {
        max
    } else {
        vol
    }
}
```

## Notes
`clamp` panics if `min > max` — a programming error caught in debug builds; validate the order first when the bounds come from input. On `f32`/`f64`, `clamp` propagates `NaN` in `self` and is unspecified if a bound is `NaN`, so filter `NaN` out of untrusted floats before calling it.
