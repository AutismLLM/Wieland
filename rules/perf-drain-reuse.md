---
id: perf-drain-reuse
title: Use drain and clear to reuse a collection's allocation across iterations
category: performance
severity: info
tier: 1
enforceability: judge
tags: [performance, allocation, collections]
applies_to:
  - "a new Vec allocated inside a loop body"
  - "batch/chunk processing that empties and refills a buffer"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#perf-drain-reuse
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
To empty a collection while keeping its heap allocation, use `clear()` or `drain(..)` and refill it, rather than constructing a new collection each pass.

## Why
`clear()` and `drain()` remove elements but retain the buffer's capacity, so the next fill reuses the same allocation. Allocating a fresh `Vec` inside a loop pays an allocate/deallocate cycle every iteration; reusing one buffer amortizes it to a single up-front allocation. `drain` additionally hands you the removed elements as an iterator for transfer between collections.

## Do
```rust
fn reuse_buffer() {
    let mut buffer = Vec::new();
    for _ in 0..1000 {
        buffer.clear();
        fill_buffer(&mut buffer);
        process(&buffer);
    }
}

fn transfer_all(src: &mut Vec<Item>, dst: &mut Vec<Item>) {
    dst.extend(src.drain(..));
}
```

## Don't
```rust
fn reuse_buffer() {
    for _ in 0..1000 {
        let mut buffer = Vec::new();
        fill_buffer(&mut buffer);
        process(&buffer);
    }
}
```

## Notes
Use `clear` to just empty, `drain(..)` to empty and consume the elements, and `mem::take` when you want to move the whole collection out and reset it to a zero-capacity empty one.
