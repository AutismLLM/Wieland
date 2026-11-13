---
id: unsafe-needs-reason
title: Use unsafe only for a valid reason, never ad-hoc
category: unsafe
severity: warn
tier: 1
enforceability: judge
tags: [safety, security, correctness]
applies_to:
  - "unsafe block or unsafe fn"
  - "unsafe impl Send / Sync"
  - "transmute"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-UNSAFE
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Reach for `unsafe` only when you have a valid reason — a novel abstraction, a benchmarked performance need, or FFI — and pair every use with plain-text reasoning for its soundness.

## Why
`unsafe` lowers the compiler's guardrails and transfers responsibility for soundness to you; correctness then rests on catching every mistake in review, which is error-prone and where high-severity vulnerabilities are born. Ad-hoc `unsafe` — a `transmute` to skip a match, an `unsafe impl Send` to dodge a bound, a lifetime bypass — takes on that risk to save effort that safe Rust could have spent for free. The three legitimate reasons all demand justification, minimal scope, and (for abstractions) Miri and adversarial tests.

## Do
```rust
/// Reads the element at `index` without a bounds check.
///
/// # Safety
///
/// `index` must be less than `slice.len()`.
unsafe fn get_unchecked_u8(slice: &[u8], index: usize) -> u8 {
    // SAFETY: the caller guarantees index < slice.len(), so the pointer
    // offset is in bounds and dereferenceable. Justified by benchmarking
    // showing the bounds check dominates this hot loop.
    unsafe { *slice.as_ptr().add(index) }
}
```

## Don't
```rust
fn to_bytes(value: &u32) -> [u8; 4] {
    unsafe { std::mem::transmute(*value) }
}

unsafe impl Send for NotActuallyThreadSafe {}
```

## Notes
The only valid reasons are novel abstractions (a new smart pointer or allocator with no established alternative), performance (only after benchmarking, e.g. `get_unchecked`), and FFI/platform calls (prefer an established interop crate). Never use `unsafe` to shorten safe code, bypass `Send`/`Sync`, or defeat lifetimes. Performance and abstraction uses must pass Miri. Building a properly designed, sound abstraction that internally does one of these things is fine; ad-hoc `unsafe` scattered through unrelated code is not.
