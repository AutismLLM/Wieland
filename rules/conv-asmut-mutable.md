---
id: conv-asmut-mutable
title: Accept impl AsMut<T> for flexible mutable borrowed inputs
category: conversions
severity: info
tier: 1
enforceability: judge
tags: [api, conversions, generics]
applies_to:
  - "a function taking `&mut Vec<T>` or `&mut String` as a write target"
  - "a generic mutable buffer parameter"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#conv-asmut-mutable
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
For genuinely generic mutable write targets, accept `impl AsMut<T>` instead of a concrete mutable reference.

## Why
`AsMut<T>` is the mutable mirror of `AsRef<T>`. Accepting `impl AsMut<[u8]>` instead of `&mut Vec<u8>` lets callers pass a `&mut Vec<u8>`, `&mut [u8]`, or `&mut [u8; N]` array without conversion overhead, widening the function's usefulness at zero runtime cost. Reserve it for genuinely generic write targets — not every `&mut T` parameter needs it.

## Do
```rust
fn fill_zeros(mut buf: impl AsMut<[u8]>) {
    for b in buf.as_mut().iter_mut() {
        *b = 0;
    }
}

fn main() {
    let mut vec_buf = vec![1u8, 2, 3];
    fill_zeros(&mut vec_buf);

    let mut arr_buf = [1u8, 2, 3, 4];
    fill_zeros(&mut arr_buf);

    let mut slice_buf = [5u8, 6, 7];
    fill_zeros(slice_buf.as_mut());
}
```

## Don't
```rust
fn fill_zeros(buf: &mut Vec<u8>) {
    for b in buf.iter_mut() {
        *b = 0;
    }
}

fn main() {
    let mut data = vec![1u8, 2, 3];
    fill_zeros(&mut data);
    // Cannot pass &mut [u8; 3] or &mut [u8]
}
```

## Notes
If a function only ever receives one concrete `&mut` type, keep it concrete — generics add compile time and cognitive overhead for no gain. Don't reach for `AsMut` as a substitute for a trait that captures domain semantics; name the abstraction properly.
