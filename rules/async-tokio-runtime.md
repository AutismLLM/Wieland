---
id: async-tokio-runtime
title: Match the Tokio runtime flavor and worker count to the workload
category: async
severity: info
tier: 1
enforceability: judge
tags: [runtime, configuration, tuning]
applies_to:
  - "#[tokio::main] entry point"
  - "tokio::runtime::Builder configuration"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-tokio-runtime
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Choose the Tokio runtime flavor and worker-thread count deliberately for the workload rather than defaulting the same runtime everywhere.

## Why
The default multi-threaded runtime suits many concurrent I/O tasks but wastes threads for a single-connection CLI, while a `current_thread` runtime cannot use multiple cores for genuinely concurrent work. Mismatched configuration causes blocked workers, poor throughput, or resource exhaustion. Sizing worker threads to the workload — cores for CPU-bound, oversubscribed for I/O-bound — is what keeps the executor efficient.

## Do
```rust
fn main() {
    let parallelism = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(parallelism)
        .enable_all()
        .thread_name("worker")
        .build()
        .unwrap();

    runtime.block_on(async_main());
}
```

## Don't
```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    for _ in 0..1000 {
        tokio::spawn(async { do_io_work().await });
    }
}
```

## Notes
Use `available_parallelism` (stable since 1.59, cgroup-aware) rather than the unmaintained `num_cpus` crate. Run separate runtimes when I/O and CPU pools must not interfere. Prefer `current_thread` for CLIs, tests, and single-connection clients.
