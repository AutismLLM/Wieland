---
id: conv-fromstr-parsing
title: Implement FromStr to enable str::parse for string-to-type conversions
category: conversions
severity: info
tier: 1
enforceability: judge
tags: [api, conversions, parsing]
applies_to:
  - "a function named like `parse_foo` taking `&str` and returning Result"
  - "converting a string into a typed value"
  - "a type consumed by CLI or config parsers"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#conv-fromstr-parsing
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Parse strings into your type by implementing `FromStr`, which unlocks the standard `.parse::<T>()` idiom.

## Why
`FromStr` is the single standard hook for parsing a `&str` into a typed value. Implementing it enables the idiomatic `.parse()` call, integrates with CLI argument parsers like clap and argh, and is the expected interface for string-deserializable types. A bespoke `fn parse_foo(s: &str)` forces callers to learn a private name and breaks generic code constrained on `T: FromStr`.

## Do
```rust
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct ParseColorError(String);

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown color: {}", self.0)
    }
}

impl std::error::Error for ParseColorError {}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            other => Err(ParseColorError(other.to_owned())),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c: Color = "green".parse()?;
    assert_eq!(c, Color::Green);
    Ok(())
}
```

## Don't
```rust
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn parse_color(s: &str) -> Result<Color, String> {
    match s {
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "blue" => Ok(Color::Blue),
        other => Err(format!("unknown color: {other}")),
    }
}

fn main() {
    let c = parse_color("red").unwrap();
}
```

## Notes
Use a concrete `Err` type, not `String`, so callers can pattern-match on failures. `FromStr` pairs naturally with `Display`: if you can parse it in, you should be able to print it out. For infallible string conversions (e.g. wrapping a `String` in a newtype), implement `From<&str>` or `From<String>` instead.
