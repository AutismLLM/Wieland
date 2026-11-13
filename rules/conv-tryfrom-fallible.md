---
id: conv-tryfrom-fallible
title: Implement TryFrom for fallible conversions instead of ad-hoc functions
category: conversions
severity: info
tier: 1
enforceability: judge
tags: [api, conversions, ergonomics]
applies_to:
  - "a function named like `foo_from_bar` returning Result"
  - "converting one type into another where the conversion can fail"
  - "designing a type's construction surface"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#conv-tryfrom-fallible
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Expose fallible conversions by implementing `TryFrom<T>` rather than a bespoke conversion function.

## Why
`TryFrom`/`TryInto` is the standard trait pair for conversions that can fail. Implementing `TryFrom<T>` automatically provides `TryInto` for callers via the blanket impl, enabling the ergonomic `.try_into()?` idiom that composes with the `?` operator, ecosystem crates, and generic bounds like `T: TryFrom<U>`. Ad-hoc functions scatter the conversion surface and force callers to learn private names.

## Do
```rust
use std::fmt;

#[derive(Debug)]
struct Port(u16);

#[derive(Debug)]
struct PortError(u32);

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "port {} is out of range (0-65535)", self.0)
    }
}

impl std::error::Error for PortError {}

impl TryFrom<u32> for Port {
    type Error = PortError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        u16::try_from(value).map(Port).map_err(|_| PortError(value))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p: Port = 8080_u32.try_into()?;
    println!("port: {}", p.0);
    Ok(())
}
```

## Don't
```rust
struct Port(u16);

fn port_from_u32(n: u32) -> Result<Port, String> {
    if n > u16::MAX as u32 {
        return Err(format!("port {} out of range", n));
    }
    Ok(Port(n as u16))
}

fn main() {
    let p = port_from_u32(8080).unwrap();
}
```

## Notes
Implement `TryFrom`, not `TryInto` — the blanket impl provides `TryInto` automatically. Use a concrete error type, not `String` or `Box<dyn Error>`, so callers can match on it. When the conversion is truly infallible, implement `From` instead.
