---
id: type-repr-transparent
title: Use #[repr(transparent)] on FFI newtypes to match ABI layout
category: types
severity: warn
tier: 1
enforceability: judge
tags: [ffi, layout, newtype]
applies_to:
  - "single-field newtype passed across an extern boundary"
  - "wrapper that must share its inner type's ABI"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#type-repr-transparent
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Mark a single-field newtype that crosses an FFI boundary `#[repr(transparent)]` so it is guaranteed to share the inner type's size, alignment, and ABI.

## Why
Rust's default layout is unspecified, so a plain `struct Handle(u64)` is not guaranteed to pass across an `extern "C"` boundary the same way a bare `u64` does. `#[repr(transparent)]` guarantees identical memory layout and calling convention as the inner type, which is what makes type-safe FFI wrappers, `NonZero` optimizations, and handle types sound.

## Do
```rust
use std::os::raw::c_int;

#[repr(transparent)]
struct FileDescriptor(c_int);

unsafe extern "C" {
    fn close(fd: FileDescriptor) -> c_int;
}
```

## Don't
```rust
struct Handle(u64);

unsafe extern "C" {
    fn process_handle(h: Handle);
}
```

## Notes
Only applies to a type with exactly one non-zero-sized field, so a `PhantomData<T>` marker alongside the payload is fine. It preserves niche optimizations too: `Option<NonZeroHandle>` stays the size of `u64`. For pure-Rust newtypes it is optional and harmless.
