---
id: unsafe-extern-block
title: Wrap extern blocks in unsafe extern and annotate each item safe or unsafe
category: unsafe
severity: warn
tier: 2
enforceability: check
tags: [ffi, edition-2024, abi]
applies_to:
  - "extern \"C\" { ... } blocks"
  - "FFI declarations"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['extern\s+"[A-Za-z0-9_-]+"\s*\{', 'extern\s+"C"']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#unsafe-extern-block
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
In Rust 2024, wrap `extern` blocks in `unsafe extern { }` and mark each item `safe` or `unsafe`.

## Why
Before Rust 2024, every function in an `extern "C" { }` block was implicitly unsafe to call, yet the block carried no `unsafe` keyword — hiding the fact that ABI correctness was entirely the programmer's responsibility. Rust 2024 requires `unsafe extern`, asserting the declarations faithfully describe the external ABI, and lets each item be marked `safe` (callable without an `unsafe` block) or `unsafe` (caller must use `unsafe {}`). This makes FFI boundaries auditable at a glance.

## Do
```rust
unsafe extern "C" {
    pub unsafe fn strlen(s: *const std::ffi::c_char) -> usize;
    pub unsafe fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8;
    pub safe fn rust_version_major() -> u32;
    pub unsafe static errno: std::ffi::c_int;
}

fn copy_bytes(dst: *mut u8, src: *const u8, n: usize) {
    // SAFETY: dst and src are non-overlapping, both valid for n bytes.
    unsafe { memcpy(dst, src, n) };
}
```

## Don't
```rust
extern "C" {
    fn strlen(s: *const std::ffi::c_char) -> usize;
    fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8;
    static errno: std::ffi::c_int;
}
```

## Notes
Run `cargo fix --edition` to apply the mechanical migration, then review each item to decide whether `safe` is warranted — marking an item `safe` that is actually unsafe to call is itself unsound and the compiler will not catch it. Update code generators (`bindgen` 0.70+, `cbindgen`) to emit `unsafe extern` blocks.
