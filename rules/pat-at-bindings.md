---
id: pat-at-bindings
title: Use @ bindings to capture a value while matching it against a pattern
category: patterns
severity: info
tier: 1
enforceability: judge
tags: [pattern-matching, readability, idiom]
applies_to:
  - "match arm with a range or sub-pattern whose value is also needed"
  - "guard that re-tests a condition already expressed in the pattern"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#pat-at-bindings
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use `name @ pattern` to bind a value and test it against a pattern in the same arm instead of re-accessing the expression or duplicating the condition in a guard.

## Why
The `name @ pattern` syntax tests a value against `pattern` and binds it to `name` in one readable unit. Without `@`, you either re-access the original expression or add a guard that repeats the condition and re-extracts the value. Binding a whole variant while inspecting its payload avoids reconstructing it later, and the constraint and the binding stay visually together.

## Do
```rust
#[derive(Debug, Clone)]
enum Packet {
    Data(Vec<u8>),
    Control(u8),
}

fn log_data(packet: &Packet) {
    match packet {
        whole @ Packet::Data(bytes) if !bytes.is_empty() => {
            println!("non-empty packet: {whole:?}");
        }
        Packet::Data(_) => println!("empty data packet"),
        Packet::Control(code) => println!("control: {code}"),
    }
}

fn classify(n: u32) -> String {
    match n {
        id @ 1..=9 => format!("single digit: {id}"),
        id @ 10..=99 => format!("two digits: {id}"),
        _ => String::from("large"),
    }
}
```

## Don't
```rust
#[derive(Debug)]
enum Command {
    Move { x: i32, y: i32 },
}

fn validate_move(cmd: &Command) {
    match cmd {
        Command::Move { x, y } if *x >= 0 && *x <= 100 => {
            println!("valid move to x={x}, y={y}");
        }
        _ => println!("invalid command"),
    }
}
```

## Notes
`@` bindings work in every pattern position: `match`, `if let`, `while let`, `let ... else`, and function parameters.
