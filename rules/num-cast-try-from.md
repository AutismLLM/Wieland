---
id: num-cast-try-from
title: Avoid `as` for narrowing casts; use `From` to widen and `TryFrom` to narrow
category: numerics
severity: warn
tier: 2
enforceability: gate
tags: [casts, truncation, conversions]
applies_to:
  - "as cast between integer types"
  - "float-to-integer conversion"
rust_edition: "2024"
msrv: null
detect:
  clippy: [cast_possible_truncation, cast_lossless, cast_sign_loss, cast_precision_loss]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#num-cast-try-from
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `From`/`Into` for lossless widening and `TryFrom`/`TryInto` for narrowing; reserve `as` for pointer casts and range-verified float-to-integer conversions.

## Why
The `as` cast silently truncates or wraps on narrowing (`300u32 as u8 == 44`) and turns `NaN` into `0` on float-to-integer conversion. These mistakes never surface at compile time and are invisible in review. `From`/`Into` are lossless by construction and simply will not compile for lossy conversions; `TryFrom`/`TryInto` return a `Result`, forcing the fallible case to be handled.

## Do
```rust
fn widen(x: u8) -> u32 {
    u32::from(x)
}

fn narrow(x: u32) -> Result<u8, std::num::TryFromIntError> {
    u8::try_from(x)
}

fn float_to_index(f: f64, len: usize) -> Option<usize> {
    if f.is_nan() || f < 0.0 || f >= len as f64 {
        return None;
    }
    Some(f as usize)
}
```

## Don't
```rust
fn narrow(x: u32) -> u8 {
    x as u8
}

fn to_index(f: f64) -> usize {
    f as usize
}
```

## Notes
`as` is still the right tool for intentional pointer casts (`*const u8 as *mut u8`), for float-to-integer once you have verified the range, and for `usize` <-> pointer-sized integer where exact semantics are required. When `.try_into()` cannot infer the target, add a type annotation: `let n: u8 = x.try_into()?;`.
