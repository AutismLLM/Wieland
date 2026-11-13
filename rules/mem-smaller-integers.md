---
id: mem-smaller-integers
title: Choose the smallest integer type that fits the domain to shrink memory use
category: memory
severity: info
tier: 1
enforceability: judge
tags: [performance, layout, integers]
applies_to:
  - "u64 or i64 field holding a small bounded value"
  - "struct instantiated in bulk"
  - "NonZero niche for Option optimization"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-smaller-integers
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Pick the smallest integer type that covers the value's real range rather than defaulting to `i32`/`u64` for everything.

## Why
An `i64` where a `u8` suffices wastes seven bytes per value, and in arrays or bulk-instantiated structs that waste compounds into megabytes and worse cache utilization. Sizing fields to the domain — `u8` for color channels, `u16` for HTTP status codes — shrinks memory and packs more data per cache line. Widening later is always a safe `into()`; narrowing uses a checked `try_into()`.

## Do
```rust
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

struct HttpStatus {
    code: u16,
    version: u8,
}
```

## Don't
```rust
struct Pixel {
    r: u64,
    g: u64,
    b: u64,
    a: u64,
}

struct HttpStatus {
    code: i32,
    version: i32,
}
```

## Notes
Rust requires `usize` for indexing, so keep it there. When shrinking helps `Option`, reach for `NonZeroU*` types, whose niche lets `Option<NonZeroU64>` fit in 8 bytes instead of 16.
