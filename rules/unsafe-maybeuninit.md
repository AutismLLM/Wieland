---
id: unsafe-maybeuninit
title: Use MaybeUninit for uninitialized memory, never mem::uninitialized or zeroed
category: unsafe
severity: warn
tier: 2
enforceability: check
tags: [memory, undefined-behavior, initialization]
applies_to:
  - "std::mem::uninitialized"
  - "std::mem::zeroed"
  - "uninitialized buffers"
rust_edition: "2024"
msrv: null
detect:
  clippy: [uninit_assumed_init, uninit_vec]
  grep: ['mem::uninitialized', 'mem::zeroed']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#unsafe-maybeuninit
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `MaybeUninit<T>` for uninitialized memory; never use `mem::uninitialized()` or `mem::zeroed()` for types with validity invariants.

## Why
`mem::uninitialized()` was deprecated in Rust 1.39 and is immediate undefined behavior for any type whose bit patterns have validity invariants — `bool`, `&T`, `NonZeroU32`, `char`, enums. Even `mem::zeroed()` is UB for references and `NonZero*` types. `MaybeUninit<T>` wraps uninitialized memory without ever producing an invalid `T`, and the optimizer cannot miscompile around it. The standard library uses it pervasively.

## Do
```rust
use std::mem::MaybeUninit;

let mut x = MaybeUninit::<u32>::uninit();
x.write(42);
// SAFETY: we just wrote a valid u32 via `write`, so it is initialized.
let value: u32 = unsafe { x.assume_init() };

fn fill_vec(v: &mut Vec<u8>, extra: usize) {
    v.reserve(extra);
    let spare = v.spare_capacity_mut();
    for slot in spare.iter_mut().take(extra) {
        slot.write(0u8);
    }
    // SAFETY: we initialized `extra` elements in the spare capacity.
    unsafe { v.set_len(v.len() + extra) };
}
```

## Don't
```rust
use std::mem;

// Instant UB: `bool` has validity invariants; uninitialized bits are not
// guaranteed to be 0 or 1.
let b: bool = unsafe { mem::uninitialized() };

// Also UB for references — a zero reference is immediately invalid.
let r: &u32 = unsafe { mem::zeroed() };
```

## Notes
`assume_init` is only sound after every byte is initialized. For zeroed memory where zero is valid for all fields (e.g. `u8`, plain C structs with no references) `mem::zeroed()` is technically sound, but `MaybeUninit` is still preferred for clarity.
