---
id: mem-arena-allocator
title: Use arena allocators for batch, request-scoped, or parse-tree allocations
category: memory
severity: info
tier: 1
enforceability: judge
tags: [performance, allocation, arena]
applies_to:
  - "many small Box allocations in a loop"
  - "request-scoped or parse-tree allocation"
  - "bumpalo"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#mem-arena-allocator
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When many short-lived values share a lifetime, allocate them from an arena (bump allocator) and free them all at once instead of allocating and freeing each individually.

## Why
An arena allocates by bumping a pointer, which is far cheaper than a general allocator call, and it frees the whole region in O(1) when dropped. This suits parse trees, per-request scratch data, and batch processing, where thousands of small allocations otherwise dominate the work. The trade-off is that arena memory lives until reset, so it is wrong for long-lived data or values that escape the scope.

## Do
```rust
use bumpalo::Bump;

fn parse<'a>(input: &str, arena: &'a Bump) -> Vec<&'a Node> {
    let mut nodes = Vec::new();
    for token in tokenize(input) {
        let node = arena.alloc(Node::new(token));
        nodes.push(node);
    }
    nodes
}
```

## Don't
```rust
fn parse(input: &str) -> Vec<Box<Node>> {
    let mut nodes = Vec::new();
    for token in tokenize(input) {
        nodes.push(Box::new(Node::new(token)));
    }
    nodes
}
```

## Notes
Do not use an arena for long-lived data, values that must escape the scope (copy them out first), or simple programs where the allocation volume never justifies the extra dependency and lifetime plumbing.
