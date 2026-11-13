---
id: unsafe-minimize-scope
title: Keep unsafe blocks as small as the operation that actually requires unsafety
category: unsafe
severity: info
tier: 1
enforceability: judge
tags: [audit, review, edition-2024]
applies_to:
  - "unsafe fn bodies"
  - "large unsafe { } blocks"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#unsafe-minimize-scope
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Keep `unsafe` blocks as small as possible — mark only the operation that requires unsafety, not the surrounding safe code.

## Why
When an entire function is `unsafe fn` or wrapped in one huge `unsafe {}`, every line appears equally suspect to an auditor. Shrinking blocks to the minimum isolates exactly which operation violates a safety invariant, making reviews tractable and bugs easier to find. Rust 2024's `unsafe_op_in_unsafe_fn` lint reinforces this: even inside an `unsafe fn`, each unsafe operation needs its own explicit `unsafe {}`.

## Do
```rust
fn sum_at(ptr: *const i32, len: usize, index: usize) -> i32 {
    assert!(index < len, "index out of bounds");
    // SAFETY: index < len guarantees ptr.add(index) is within the allocation.
    let value = unsafe { *ptr.add(index) };
    value + 1
}
```

## Don't
```rust
unsafe fn sum_at(ptr: *const i32, len: usize, index: usize) -> i32 {
    let adjusted_len = len.saturating_sub(1);
    assert!(index <= adjusted_len);
    let value = *ptr.add(index);
    value + 1
}
```

## Notes
If two unsafe operations share the exact same precondition and separating them would only restate the identical justification, a single block covering both is acceptable — but it should still be the minimum necessary scope. A safe wrapper around a small `unsafe {}` is almost always preferable to exposing the whole function as `unsafe fn`.
