---
id: doc-safety-section
title: Give every `unsafe` function a `# Safety` doc section
category: docs
severity: warn
tier: 2
enforceability: gate
tags: [rustdoc, unsafe, soundness]
applies_to:
  - "pub unsafe fn"
  - "pub unsafe trait"
rust_edition: "2024"
msrv: null
detect:
  clippy: [missing_safety_doc]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#doc-safety-section
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Document a `# Safety` section on every public `unsafe` function and trait, listing exactly what the caller must guarantee for the call to be sound.

## Why
An `unsafe fn` shifts invariants the compiler cannot check onto the caller. Without a `# Safety` section the caller has no way to know what those invariants are, so they cannot uphold them — undefined behavior is the result. This is a soundness requirement, not a style nicety; `clippy::missing_safety_doc` warns by default when it is missing.

## Do
```rust
/// Reads a value from a raw pointer.
///
/// # Safety
///
/// The caller must ensure that:
/// - `ptr` is valid for reads of `size_of::<T>()` bytes
/// - `ptr` is properly aligned for type `T`
/// - `ptr` points to an initialized value of type `T`
pub unsafe fn read_ptr<T>(ptr: *const T) -> T {
    unsafe { ptr.read() }
}
```

## Don't
```rust
/// Reads a value from a raw pointer.
pub unsafe fn read_ptr<T>(ptr: *const T) -> T {
    unsafe { ptr.read() }
}
```

## Notes
Inside a safe function, annotate each `unsafe` block with a `// SAFETY:` comment justifying why the invariants hold at that call site.
