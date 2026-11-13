---
id: unsafe-safety-comment
title: Document every unsafe block with SAFETY and every unsafe fn with # Safety
category: unsafe
severity: deny
tier: 3
enforceability: gate
tags: [safety, correctness, docs]
applies_to:
  - "unsafe {} block"
  - "pub unsafe fn"
  - "raw pointer dereference"
rust_edition: "2024"
msrv: null
detect:
  clippy: [undocumented_unsafe_blocks]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#unsafe-safety-comment
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Write a `// SAFETY:` comment above every `unsafe {}` block and a `# Safety` doc section on every `unsafe fn`.

## Why
Unsafe code cannot be audited without its justification: a reviewer cannot verify invariants they cannot read. The two forms serve different audiences — a `# Safety` doc section states the *caller's* obligations, while an inline `// SAFETY:` comment explains why *this specific* operation upholds those obligations. The `clippy::undocumented_unsafe_blocks` lint enforces the inline form mechanically, and the standard library, tokio, and bevy all require both before merging.

## Do
```rust
/// Returns the byte at `ptr + offset`.
///
/// # Safety
///
/// - `ptr` must be valid for reads for at least `offset + 1` bytes.
/// - `ptr` must be non-null and properly aligned for `u8`.
/// - The memory must not be mutated for the duration of this call.
pub unsafe fn read_at(ptr: *const u8, offset: usize) -> u8 {
    // SAFETY: caller guarantees ptr is valid for at least offset + 1 bytes,
    // so ptr.add(offset) is in bounds and dereferenceable.
    unsafe { *ptr.add(offset) }
}
```

## Don't
```rust
pub unsafe fn read_at(ptr: *const u8, offset: usize) -> u8 {
    unsafe { *ptr.add(offset) }
}

fn process(slice: &[u8]) -> u8 {
    unsafe { *slice.as_ptr().add(10) }
}
```

## Notes
Under the 2024 edition's `unsafe_op_in_unsafe_fn` lint, inner `unsafe {}` blocks inside an `unsafe fn` still need their own `// SAFETY:` comments. When a block spans several operations, address each distinct invariant in one comment or split into smaller blocks. Enable the lint with `#![warn(clippy::undocumented_unsafe_blocks)]`.
