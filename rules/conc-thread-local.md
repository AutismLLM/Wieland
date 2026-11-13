---
id: conc-thread-local
title: Prefer thread_local! with Cell/RefCell over static mut
category: concurrency
severity: warn
tier: 2
enforceability: check
tags: [thread-local, static-mut, safety]
applies_to:
  - "static mut used for per-thread scratch state"
  - "unsafe access to a mutable static"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['static\s+mut\s']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#conc-thread-local
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Store per-thread mutable state in a `thread_local!` with `Cell`/`RefCell` rather than a `static mut`.

## Why
`static mut` demands `unsafe` at every access and is undefined behavior if two threads touch it at once — the compiler cannot rule that out, and in Rust 2024 taking a reference to a `static mut` is a hard error (`static_mut_refs`). `thread_local!` gives each thread its own copy behind safe APIs: `Cell` for `Copy` values, `RefCell` for everything else. No synchronization overhead and no unsafe code.

## Do
```rust
use std::cell::RefCell;

thread_local! {
    static BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(4096));
}

fn append_to_buffer(data: &[u8]) {
    BUFFER.with_borrow_mut(|buf| buf.extend_from_slice(data));
}

fn flush_buffer() -> Vec<u8> {
    BUFFER.with_borrow_mut(|buf| std::mem::take(buf))
}
```

## Don't
```rust
static mut BUFFER: Vec<u8> = Vec::new();

fn append_to_buffer(data: &[u8]) {
    unsafe {
        BUFFER.extend_from_slice(data);
    }
}

fn flush_buffer() -> Vec<u8> {
    unsafe {
        std::mem::take(&mut BUFFER)
    }
}
```

## Notes
`with_borrow`/`with_borrow_mut` (stable since 1.73) are cleaner than the older `with(|v| v.borrow_mut())` form; for `Copy` types use `Cell` and its `get`/`set`. Thread-local values are strictly per-thread and invisible to other threads, so do not use them for state that must be shared — use `static` with an immutable type for read-only globals, or `Mutex`/atomics for shared mutable state.
