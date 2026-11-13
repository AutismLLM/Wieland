---
id: name-lifetime-short
title: Use short, conventional lifetime names like 'a and 'de
category: naming
severity: info
tier: 1
enforceability: judge
tags: [naming, lifetimes, readability]
applies_to:
  - "explicit lifetime parameters in signatures and types"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-lifetime-short
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Use short lifetime names — `'a`, `'b`, or a brief conventional one like `'de` or `'src` — never a verbose sentence.

## Why
Lifetimes appear in nearly every borrowing signature, so long names like `'parser_instance_lifetime` bloat the type and bury the actual parameters. Short names keep signatures scannable, and the community conventions (`'de` for deserialization, `'src` for source input) carry meaning without clutter.

## Do
```rust
struct Parser<'a> {
    source: &'a str,
}

fn parse<'a>(input: &'a str) -> &'a str {
    input.trim()
}
```

## Don't
```rust
struct Parser<'parser_instance_lifetime> {
    source: &'parser_instance_lifetime str,
}

fn parse<'input_lifetime>(input: &'input_lifetime str) -> &'input_lifetime str {
    input.trim()
}
```

## Notes
Prefer lifetime elision when it applies — `fn first(s: &str) -> &str` needs no named lifetime at all.
