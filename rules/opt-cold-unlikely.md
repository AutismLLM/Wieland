---
id: opt-cold-unlikely
title: Mark rarely-taken functions with #[cold] to improve code layout
category: optimization
severity: info
tier: 1
enforceability: judge
tags: [performance, hot-path, attributes, branch-prediction]
applies_to:
  - "error/panic/fallback helpers called off the hot path"
  - "#[cold] attribute"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#opt-cold-unlikely
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Extract rarely-executed work — error construction, logging, panics, fallbacks — into `#[cold]` functions so the compiler moves them off the hot path.

## Why
`#[cold]` tells the compiler a function is seldom called, so it places that code in a separate section away from hot code, biases branch prediction toward the common path, and skips inlining it into hot callers. Keeping the hot function small improves instruction-cache utilization and lets the fast path stay resident. Pair with `#[inline(never)]` when the cold body is bulky.

## Do
```rust
fn validate(input: &str) -> Result<Data, ValidationError> {
    if input.is_empty() {
        return cold_empty_error();
    }
    Ok(parse_data(input))
}

#[cold]
#[inline(never)]
fn cold_empty_error() -> Result<Data, ValidationError> {
    Err(ValidationError::Empty)
}
```

## Don't
```rust
fn validate(input: &str) -> Result<Data, ValidationError> {
    if input.is_empty() {
        return Err(ValidationError::Empty);
    }
    Ok(parse_data(input))
}
```

## Notes
Only helps when the branch really is rare and the surrounding function is hot; on cold code it is noise. Verify placement with `nm binary | grep cold` and profile before committing to it.
