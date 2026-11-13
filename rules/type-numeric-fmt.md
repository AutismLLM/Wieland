---
id: type-numeric-fmt
title: Forward LowerHex/UpperHex/Octal/Binary on numeric newtypes
category: types
severity: info
tier: 1
enforceability: judge
tags: [newtype, formatting, numeric]
applies_to:
  - "newtype wrapping a primitive integer"
  - "mask, address, or identifier type"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#type-numeric-fmt
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For a newtype wrapping a primitive integer, forward `LowerHex`, `UpperHex`, `Octal`, and `Binary` to the inner value so `{:x}`, `{:X}`, `{:o}`, and `{:b}` keep working.

## Why
The API Guidelines (C-NUM-FMT) say numeric types should support the same format specifiers as their underlying integer. A newtype that only implements `Display` turns `{:x}` on a bitmask or address into a compile error — an ergonomic regression. Each impl is a one-liner that forwards to the inner formatter, preserving flags like `#`, `0`, and width.

## Do
```rust
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Mask(u32);

impl fmt::LowerHex for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl fmt::UpperHex for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

impl fmt::Binary for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.0, f)
    }
}
```

## Don't
```rust
use std::fmt;

struct Mask(u32);

impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

let m = Mask(0xDEAD_BEEF);
println!("{:x}", m);
```

## Notes
Always forward through the inner type's trait impl (`fmt::LowerHex::fmt(&self.0, f)`) so format flags are honored. Skip `Octal`/`Binary` for a purely decimal `Count`, but always provide `LowerHex`/`UpperHex` for masks, addresses, and identifiers.
