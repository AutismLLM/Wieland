---
id: anti-clone-excessive
title: Borrow with a reference instead of cloning when you only need to read
category: anti-patterns
severity: warn
tier: 2
enforceability: gate
tags: [ownership, allocation, performance]
applies_to:
  - "x.clone() passed to a function that only reads it"
  - "iterating over a cloned collection"
rust_edition: "2024"
msrv: null
detect:
  clippy: [redundant_clone, clone_on_copy, clone_on_ref_ptr]
  grep: ['\.clone\(\)']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#anti-clone-excessive
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When you only need to read a value, borrow it with `&T` instead of calling `.clone()`.

## Why
`.clone()` allocates and copies data; a shared reference is free. Cloning to satisfy a function that only reads, or cloning a whole collection just to iterate it, wastes memory and CPU and usually signals a misunderstanding of ownership. The compiler's `Deref` coercions mean a borrow is almost always what the callee actually wants.

## Do
```rust
fn print_name(name: &str) {
    println!("{name}");
}

let name = "Alice".to_string();
print_name(&name);
print_name(&name);
```

## Don't
```rust
fn print_name(name: String) {
    println!("{name}");
}

let name = "Alice".to_string();
print_name(name.clone());
print_name(name);
```

## Notes
Cloning is correct when you genuinely need owned data: moving into an `async move`/thread closure, storing a value in a struct, or sharing ownership (prefer `Arc<T>` when that is frequent). Reach for `Cow<str>` when you clone only on the write path.
