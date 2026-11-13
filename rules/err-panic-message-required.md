---
id: err-panic-message-required
title: Give every intentional panic, assert, and unreachable a helpful message
category: errors
severity: warn
tier: 1
enforceability: judge
tags: [panic, diagnostics, debugging]
applies_to:
  - "panic!/assert!/unreachable!/todo! without a message"
  - "intentional panics outside tests"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-PANIC-MESSAGE
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When code panics intentionally — via `panic!`, `assert!`, `assert_eq!`, `unreachable!`, or `todo!` — include a message that states what went wrong and, where relevant, the actual values involved.

## Why
A bare panic gives whoever hits it nothing to act on beyond a source location. A message that names the violated condition and includes the actual values turns a crash report into a diagnosis: the difference between "assertion failed" and "buffer too small for header: got 12 bytes, need 16" is the difference between a support ticket and a five-minute fix. Messages aimed at API misuse should speak to the caller; messages for internal bugs should speak to whoever maintains the code next.

## Do
```rust
assert!(
    buffer.len() >= HEADER_SIZE,
    "buffer too small for header: got {} bytes, need {HEADER_SIZE}",
    buffer.len()
);

fn dispatch(kind: PacketKind) -> Response {
    match kind {
        PacketKind::Ping => Response::Pong,
        PacketKind::Data(d) => Response::Ack(d.len()),
        PacketKind::Unknown(code) => {
            unreachable!("dispatch received reserved packet code {code}, should be filtered upstream")
        }
    }
}
```

## Don't
```rust
assert!(buffer.len() >= HEADER_SIZE);

fn dispatch(kind: PacketKind) -> Response {
    match kind {
        PacketKind::Ping => Response::Pong,
        PacketKind::Data(d) => Response::Ack(d.len()),
        PacketKind::Unknown(_) => unreachable!(),
    }
}
```

## Notes
Panic messages inside `#[test]` functions are generally not needed — the failing assertion location plus test name is usually enough context there. This is about intentional panics you write, not the panic machinery itself; `.unwrap()` calls should first be reconsidered under `err-no-unwrap-prod` before reaching for a message on `.expect()`.
