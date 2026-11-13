---
id: type-never-diverge
title: Use the never type ! for functions that never return
category: types
severity: info
tier: 1
enforceability: judge
tags: [control-flow, never-type, signatures]
applies_to:
  - "function that loops forever, panics, or exits"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#type-never-diverge
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Give functions that never return normally the return type `!`, so the signature states the divergence and the value coerces into any expected type.

## Why
`-> !` tells both the compiler and the reader that the function loops forever, panics, or exits the process. It documents control flow precisely and lets the call site treat the result as any type, which is what makes `panic!` and `std::process::exit` usable in a `match` arm that otherwise yields a value.

## Do
```rust
fn abort_with_error(msg: &str) -> ! {
    eprintln!("Fatal error: {msg}");
    std::process::exit(1);
}

fn get_value(opt: Option<i32>) -> i32 {
    match opt {
        Some(v) => v,
        None => panic!("No value"),
    }
}
```

## Don't
```rust
fn infinite_loop() {
    loop {
        process_events();
    }
}

fn unreachable_code() -> Option<()> {
    panic!("This should never be called");
}
```

## Notes
`-> !` as a return type is stable. Using `!` as a type *argument* (e.g. `Result<(), !>`) is still nightly-only; on stable, use `std::convert::Infallible` as the conventional stand-in.
