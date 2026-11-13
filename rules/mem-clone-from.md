---
id: mem-clone-from
title: Use clone_from to reuse an allocation when repeatedly cloning into a variable
category: memory
severity: warn
tier: 2
enforceability: gate
tags: [performance, allocation, clone]
applies_to:
  - "x = y.clone() reassigning an existing owned value"
  - "cloning into a reused buffer inside a loop"
rust_edition: "2024"
msrv: null
detect:
  clippy: [assigning_clones]
  grep: []
  ast_query: null
autofix: true
sources:
  - leonardomso/rust-skills#mem-clone-from
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When you clone into a variable that already holds an allocation, use `x.clone_from(&y)` instead of `x = y.clone()`.

## Why
`x = y.clone()` drops `x`'s existing buffer and allocates a fresh one for the copy. `x.clone_from(&y)` reuses `x`'s capacity when it is large enough, so no allocation happens at all. In a loop that clones into the same buffer, this removes a drop-and-reallocate on every iteration; clippy's `assigning_clones` flags the pattern.

## Do
```rust
let mut buffer = String::with_capacity(1024);

for source in sources {
    buffer.clone_from(source);
    process(&buffer);
}
```

## Don't
```rust
let mut buffer = String::with_capacity(1024);

for source in sources {
    buffer = source.clone();
    process(&buffer);
}
```

## Notes
There is nothing to reuse for a single one-off clone or for `Copy` types, so `clone_from` only pays off when the destination already owns an allocation you can keep.
