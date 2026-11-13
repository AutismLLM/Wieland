---
id: doc-panics-section
title: Give panic-capable public functions a `# Panics` doc section
category: docs
severity: warn
tier: 2
enforceability: gate
tags: [rustdoc, panics, contract]
applies_to:
  - "pub fn that can panic"
rust_edition: "2024"
msrv: null
detect:
  clippy: [missing_panics_doc]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#doc-panics-section
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Document a `# Panics` section on every public function that can panic, stating the exact conditions that trigger it.

## Why
A panic crashes or unwinds the caller, and the signature gives no warning. Users need the panic conditions so they can uphold preconditions — or avoid the function entirely in contexts where a panic is unacceptable (FFI, `no_std`, embedded). `clippy::missing_panics_doc` flags public functions that can panic but omit the section.

## Do
```rust
/// Returns the element at the given index.
///
/// # Panics
///
/// Panics if `index` is out of bounds (i.e. `index >= self.len()`).
pub fn get(&self, index: usize) -> &T {
    &self.data[index]
}
```

## Don't
```rust
/// Returns the element at the given index.
pub fn get(&self, index: usize) -> &T {
    &self.data[index]
}
```

## Notes
When you document a panicking function, point to a non-panicking alternative — e.g. a `checked_*` variant returning `Option` or `Result`.
