---
id: anti-vec-for-slice
title: Accept &[T] instead of &Vec<T> in function signatures
category: anti-patterns
severity: warn
tier: 2
enforceability: gate
tags: [api-design, idiom, allocation]
applies_to:
  - "a function parameter typed &Vec<T> or &mut Vec<T>"
rust_edition: "2024"
msrv: null
detect:
  clippy: [ptr_arg]
  grep: ['&Vec<', '&mut Vec<']
  ast_query: null
autofix: true
sources:
  - leonardomso/rust-skills#anti-vec-for-slice
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Take `&[T]` (or `&mut [T]`) rather than `&Vec<T>` for parameters you only read or mutate in place.

## Why
`&Vec<T>` is strictly less flexible than `&[T]`. A slice accepts a `Vec` (via deref coercion), arrays, and sub-slices; `&Vec<T>` demands exactly a `Vec`, forcing callers with an array or slice to allocate one. Clippy's `ptr_arg` flags this directly.

## Do
```rust
fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

sum(&[1, 2, 3, 4, 5]);
sum(&vec![1, 2, 3]);
```

## Don't
```rust
fn sum(numbers: &Vec<i32>) -> i32 {
    numbers.iter().sum()
}

let arr = [1, 2, 3, 4, 5];
sum(&arr.to_vec());
```

## Notes
Accept `&Vec<T>` or `&mut Vec<T>` only when you need `Vec`-specific operations like `capacity()` or `push()` that a slice cannot provide. The same rule applies to `&String` → `&str` and `&PathBuf` → `&Path`.
