---
id: coll-seq-choice
title: Default to Vec, use VecDeque for queues, avoid LinkedList
category: collections
severity: warn
tier: 2
enforceability: gate
tags: [performance, data-structures, sequences]
applies_to:
  - "std::collections::LinkedList"
  - "Vec::remove(0) in a loop"
  - "choosing a sequence type for a FIFO queue or sliding window"
rust_edition: "2024"
msrv: null
detect:
  clippy: [linkedlist]
  grep: ['LinkedList', '\.remove\(0\)']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#coll-seq-choice
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Default to `Vec`; use `VecDeque` for queue, deque, or sliding-window behaviour; avoid `LinkedList` unless profiling proves it faster.

## Why
`Vec<T>` is a contiguous growable array whose layout keeps the CPU prefetcher on your side and reaches each element in one dereference. `VecDeque<T>` is a ring buffer with O(1) amortised push and pop at both ends, ideal for a FIFO queue or sliding window. `LinkedList<T>` heap-allocates every node and pointer-chases on each access; for real workloads it is almost always slower than `Vec` or `VecDeque`, and the standard library itself discourages it. Using `Vec::remove(0)` in a loop is O(n) per call and turns the loop into O(n^2).

## Do
```rust
use std::collections::VecDeque;

fn process_queue(items: impl IntoIterator<Item = String>) {
    let mut queue: VecDeque<String> = items.into_iter().collect();
    while let Some(item) = queue.pop_front() {
        println!("processing: {item}");
    }
}

fn sliding_window_max(values: &[i32], k: usize) -> Vec<i32> {
    let mut window: VecDeque<i32> = VecDeque::with_capacity(k);
    let mut result = Vec::with_capacity(values.len().saturating_sub(k) + 1);
    for &v in values {
        window.push_back(v);
        if window.len() > k {
            window.pop_front();
        }
        if window.len() == k {
            result.push(*window.iter().max().unwrap());
        }
    }
    result
}
```

## Don't
```rust
fn process_queue(items: Vec<String>) {
    let mut queue = items;
    while !queue.is_empty() {
        let item = queue.remove(0);
        println!("processing: {item}");
    }
}
```

## Notes
`Vec` and `VecDeque` convert cheaply into each other via `From`/`Into` when you need to switch representation.
