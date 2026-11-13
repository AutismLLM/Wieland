---
id: coll-binaryheap
title: Use BinaryHeap for a priority queue or repeated max-extraction
category: collections
severity: info
tier: 1
enforceability: judge
tags: [performance, data-structures, priority-queue]
applies_to:
  - "repeatedly extracting the largest or smallest element"
  - "re-sorting a Vec after every insertion"
  - "implementing a scheduler or top-k selection"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#coll-binaryheap
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When you repeatedly need the largest (or smallest) element from a changing set, use `BinaryHeap` rather than re-sorting or re-scanning a `Vec`.

## Why
`BinaryHeap<T>` is a max-heap with O(log n) `push` and `pop` and O(1) `peek`. Sorting or linearly scanning a `Vec` after every change costs O(n log n) or O(n) per operation, turning a loop into O(n^2). For a min-heap, wrap values in `std::cmp::Reverse<T>` so the ordering flips and the smallest value is extracted first.

## Do
```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Task {
    priority: u32,
    name: String,
}

fn run_scheduler(tasks: impl IntoIterator<Item = (u32, &'static str)>) {
    let mut heap: BinaryHeap<Task> = tasks
        .into_iter()
        .map(|(priority, name)| Task { priority, name: name.to_string() })
        .collect();

    while let Some(task) = heap.pop() {
        println!("running [priority={}]: {}", task.priority, task.name);
    }
}

fn top_k_largest(values: &[i32], k: usize) -> Vec<i32> {
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k + 1);
    for &v in values {
        min_heap.push(Reverse(v));
        if min_heap.len() > k {
            min_heap.pop();
        }
    }
    let mut result: Vec<i32> = min_heap.into_iter().map(|Reverse(v)| v).collect();
    result.sort_unstable_by(|a, b| b.cmp(a));
    result
}
```

## Don't
```rust
fn top_priority_task(tasks: &mut Vec<(u32, String)>) -> Option<String> {
    let max_idx = tasks
        .iter()
        .enumerate()
        .max_by_key(|(_, (p, _))| *p)
        .map(|(i, _)| i)?;
    Some(tasks.remove(max_idx).1)
}

fn drain_all(tasks: &mut Vec<(u32, String)>) {
    while let Some(task) = top_priority_task(tasks) {
        println!("running: {task}");
    }
}
```

## Notes
`BinaryHeap` does not support efficient arbitrary removal or priority updates. If you need those, reach for a keyed priority-queue crate or restructure with lazy deletion.
