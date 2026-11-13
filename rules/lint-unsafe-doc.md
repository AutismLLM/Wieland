---
id: lint-unsafe-doc
title: Require a SAFETY comment on every unsafe block via undocumented_unsafe_blocks
category: lints
severity: warn
tier: 2
enforceability: gate
tags: [clippy, unsafe, safety, review]
applies_to:
  - "unsafe { } blocks"
  - "unsafe impl Send / Sync"
  - "raw pointer and FFI operations"
rust_edition: "2024"
msrv: null
detect:
  clippy: [undocumented_unsafe_blocks, multiple_unsafe_ops_per_block]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#lint-unsafe-doc
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Enable `clippy::undocumented_unsafe_blocks` so every `unsafe` block must carry a `// SAFETY:` comment explaining why the operation is sound.

## Why
Unsafe code is where the compiler stops checking and the human takes over, so it is the source of most memory-safety bugs. A `// SAFETY:` comment forces the author to state the invariants being upheld and gives reviewers something concrete to check against. The lint mechanically guarantees the comment exists — it cannot judge whether the reasoning is correct, but it makes silent, unexplained unsafe impossible.

## Do
```rust
pub fn read_data(ptr: *const u8, len: usize) -> &'static [u8] {
    // SAFETY: caller guarantees ptr is valid for `len` bytes, properly
    // aligned, initialized, and not aliased by any mutable reference.
    unsafe { std::slice::from_raw_parts(ptr, len) }
}
```

## Don't
```rust
pub fn read_data(ptr: *const u8, len: usize) -> &'static [u8] {
    unsafe { std::slice::from_raw_parts(ptr, len) }
}
```

## Notes
Pair it with `multiple_unsafe_ops_per_block` to keep one operation per block, so each `// SAFETY:` comment justifies exactly one unsafe action. Promote to `deny` in crates where unsafe is rare and every use must be scrutinized.
