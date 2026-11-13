---
id: async-yield-points
title: Insert cooperative yield points in long CPU-bound async loops
category: async
severity: warn
tier: 1
enforceability: judge
tags: [cooperative-scheduling, cpu-bound, runtime, hot-path]
applies_to:
  - "async fn with a long loop performing CPU work with no .await"
  - "batch or bulk processing driven inside a spawned task"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-YIELD-POINTS
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When an async task runs a long CPU-bound loop with no natural `.await` points, insert periodic `tokio::task::yield_now().await` calls so the runtime can schedule other tasks.

## Why
An async runtime multiplexes many tasks onto a handful of worker threads, and it can only switch between tasks at `.await` points. A loop that computes for milliseconds without awaiting starves every other task pinned to that worker, causing latency spikes elsewhere in the system even though the task itself does nothing incorrect. Yielding roughly every 10-100μs of work keeps switching overhead low while giving the scheduler enough openings to stay fair.

## Do
```rust
use tokio::task::yield_now;

async fn decompress_all(chunks: &[Chunk]) {
    for (i, chunk) in chunks.iter().enumerate() {
        decompress(chunk);
        if i % 64 == 0 {
            yield_now().await;
        }
    }
}
```

## Don't
```rust
async fn decompress_all(chunks: &[Chunk]) {
    for chunk in chunks {
        decompress(chunk);
    }
}
```

## Notes
If the loop already performs I/O per iteration, the `.await` inside the body already gives the runtime a natural preemption point and no extra `yield_now` is needed. For unpredictable per-item cost, prefer a runtime's cooperative-budget API (where available) over a fixed iteration count. CPU work that runs far longer than a few hundred microseconds per call belongs on `spawn_blocking` instead of yielding in place.
