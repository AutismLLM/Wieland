---
id: async-cancel-safety
title: Keep futures used in select branches cancellation-safe
category: async
severity: warn
tier: 1
enforceability: judge
tags: [select, cancellation, correctness]
applies_to:
  - "tokio::select! with read_exact / read_to_end / accumulating futures"
  - "partial state held inside a select! branch future"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#async-cancel-safety
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Only put cancellation-safe futures directly in `tokio::select!` branches; keep any partial accumulation (buffers, partial reads) in the surrounding scope, not inside the branch future.

## Why
When one `select!` branch completes, every other branch future is dropped along with any state it held. A future halfway through `read_exact` or accumulating into a `Vec` silently loses that progress — it compiles fine and only misbehaves under concurrent load. Tokio documents which primitives are cancel-safe; treat everything else as unsafe to cancel and move its state outside the branch.

## Do
```rust
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

async fn good(
    stream: &mut BufReader<TcpStream>,
    rx: &mut mpsc::Receiver<u8>,
) -> std::io::Result<()> {
    let mut buf = [0u8; 1024];
    let mut filled = 0;
    loop {
        tokio::select! {
            n = stream.read(&mut buf[filled..]) => {
                filled += n?;
                if filled == buf.len() {
                    filled = 0;
                }
            }
            msg = rx.recv() => {
                println!("got message: {msg:?}");
            }
        }
    }
}
```

## Don't
```rust
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

async fn bad(stream: &mut BufReader<TcpStream>, rx: &mut mpsc::Receiver<u8>) {
    let mut buf = [0u8; 1024];
    tokio::select! {
        result = stream.read_exact(&mut buf) => {
            println!("read {} bytes", result.unwrap());
        }
        msg = rx.recv() => {
            println!("got message: {msg:?}");
        }
    }
}
```

## Notes
`recv`, `changed`, `sleep`, `read` (not `read_exact`), and `Mutex::lock` are cancel-safe. For unavoidable non-cancel-safe work, pin the future across iterations or spawn it as a task and select on its cancel-safe `JoinHandle`.
