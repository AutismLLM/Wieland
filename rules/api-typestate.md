---
id: api-typestate
title: Encode state-machine invariants with the typestate pattern to reject bad transitions
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [typestate, state-machine, type-safety]
applies_to:
  - "type with a runtime state field guarding methods"
  - "connection, transaction, or builder with required steps"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#api-typestate
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Model a type's states as distinct types (typically via a generic state parameter) so that methods only exist in the states where they are valid, turning invalid transitions into compile errors.

## Why
A single type with a runtime `state` enum must guard every method with a check that can fail — call `send` before `authenticate` and you get a runtime error, discovered only when that path runs. The typestate pattern moves the state into the type: each state-transition method consumes one state type and returns the next, so a method that requires being authenticated simply does not exist on the unauthenticated type. The compiler enforces the state machine and the runtime checks disappear.

## Do
```rust
struct Disconnected;
struct Connected;
struct Authenticated;

struct Connection<S> {
    state: std::marker::PhantomData<S>,
}

impl Connection<Disconnected> {
    fn new() -> Self {
        Connection { state: std::marker::PhantomData }
    }
    fn connect(self) -> Connection<Connected> {
        Connection { state: std::marker::PhantomData }
    }
}

impl Connection<Connected> {
    fn authenticate(self) -> Connection<Authenticated> {
        Connection { state: std::marker::PhantomData }
    }
}

impl Connection<Authenticated> {
    fn send(&self, _data: &[u8]) {}
}

let conn = Connection::new().connect().authenticate();
conn.send(b"data");
```

## Don't
```rust
#[derive(PartialEq)]
enum State {
    Disconnected,
    Connected,
    Authenticated,
}

struct Connection {
    state: State,
}

impl Connection {
    fn send(&self, _data: &[u8]) -> Result<(), &'static str> {
        if self.state != State::Authenticated {
            return Err("not authenticated");
        }
        Ok(())
    }
}

let conn = Connection { state: State::Connected };
let _ = conn.send(b"data");
```
