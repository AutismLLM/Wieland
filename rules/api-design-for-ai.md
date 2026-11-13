---
id: api-design-for-ai
title: Design predictable, strongly-typed APIs that agents can use correctly
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [api, ergonomics, types, docs]
applies_to:
  - "public API surface"
  - "stringly-typed parameters"
  - "primitive-obsessed signatures"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-DESIGN-FOR-AI
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Design APIs to be idiomatic, strongly typed, well documented, and testable, so both humans and AI agents use them correctly on the first try.

## Why
An agent's lack of genuine understanding is counterbalanced by Rust's compiler checks — but only when the API leans on the type system. Strong types make a misuse a compile error the agent can see and fix, whereas stringly-typed and primitive-obsessed signatures let wrong calls compile and fail at runtime, where the agent iterates blindly. Idiomatic, greppable APIs that look like the rest of the Rust ecosystem, backed by thorough docs and examples, are the ones an agent can predict without guessing.

## Do
```rust
/// A validated HTTP port (1..=65535).
pub struct Port(u16);

impl Port {
    /// Parses a port, rejecting 0.
    ///
    /// # Errors
    /// Returns `PortError::Zero` if `n` is 0.
    pub fn new(n: u16) -> Result<Self, PortError> {
        if n == 0 { Err(PortError::Zero) } else { Ok(Port(n)) }
    }
}

pub enum Compression { None, Gzip, Zstd }

pub fn connect(host: &str, port: Port, compression: Compression) -> Result<Conn, ConnError> {
    todo!()
}
```

## Don't
```rust
pub fn connect(host: &str, port: i64, compression: &str) -> Conn {
    todo!()
}
```

## Notes
Concretely: follow the Rust API Guidelines and standard patterns; document every public module and item assuming a solid-but-not-expert reader; give directly runnable examples; use newtypes to avoid primitive obsession (per C-NEWTYPE); and make your API testable with mocks or fakes so callers — human or agent — can prove their usage works. Good docs and strong types help agents most because they turn runtime surprises into compile-time feedback.
