---
id: mem-thinvec
title: Use ThinVec for often-empty or optional collections to cut per-instance overhead
category: memory
severity: info
tier: 1
enforceability: judge
tags: [performance, layout, allocation]
applies_to:
  - "Vec field that is usually empty across many instances"
  - "Option<Vec<T>> pattern"
  - "thin_vec"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-thinvec
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When a struct holds a collection that is usually empty across many instances, use `ThinVec<T>` so the field costs one pointer instead of three words.

## Why
A `Vec<T>` is 24 bytes even when empty, because length and capacity live inline. `ThinVec` stores length and capacity in the heap header, so the field is a single 8-byte pointer, and an empty `ThinVec` is just a null pointer. For tree nodes that are mostly leaves or structs with several optional vecs, this cuts memory sharply, and `Option<ThinVec<T>>` gets niche optimization that `Option<Vec<T>>` does not.

## Do
```rust
use thin_vec::ThinVec;

struct TreeNode {
    value: i32,
    children: ThinVec<TreeNode>,
}
```

## Don't
```rust
struct TreeNode {
    value: i32,
    children: Vec<TreeNode>,
}
```

## Notes
Avoid `ThinVec` in hot loops and performance-critical iteration, where every length check goes through a pointer indirection that a plain `Vec` avoids. Saving 16 bytes once, for a handful of instances, is not worth it.
