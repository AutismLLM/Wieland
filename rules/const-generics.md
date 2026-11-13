---
id: const-generics
title: Parameterize over values with const generics
category: const-eval
severity: info
tier: 1
enforceability: judge
tags: [generics, arrays, zero-cost]
applies_to:
  - "functions copy-pasted per fixed array size"
  - "stack buffer carrying a runtime length field"
  - "generic array-based data structures"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#const-generics
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use const generics `<const N: usize>` to make one type or function work for any array size without macros, trait objects, or a runtime length field.

## Why
Const generics let a single definition cover every array size — or other constant value — and the compiler monomorphizes one copy per distinct value, so there is no indirection and no overhead over hand-written per-size code. This is the idiomatic way to write generic array-based data structures and algorithms on stable Rust. Where `N` can be inferred from an argument, callers rarely need to write it.

## Do
```rust
fn sum<const N: usize>(arr: [i32; N]) -> i32 {
    arr.iter().sum()
}

let total = sum([1, 2, 3, 4]);
let total8 = sum([0i32; 8]);

struct Buffer<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> Buffer<N> {
    const fn new() -> Self {
        Self { data: [0u8; N], len: 0 }
    }

    fn push(&mut self, byte: u8) -> bool {
        if self.len < N {
            self.data[self.len] = byte;
            self.len += 1;
            true
        } else {
            false
        }
    }
}

let mut small: Buffer<8> = Buffer::new();
small.push(42);
```

## Don't
```rust
fn sum_4(arr: [i32; 4]) -> i32 {
    arr.iter().sum()
}

fn sum_8(arr: [i32; 8]) -> i32 {
    arr.iter().sum()
}

struct Buffer {
    data: Vec<u8>,
    capacity: usize,
}
```

## Notes
Rust 1.65+ supports const generic defaults (`struct Buf<const N: usize = 64>`), letting callers override a sensible default. Const generics currently accept integer, bool, and char types; floating-point and custom types are not yet stable.
