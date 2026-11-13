---
id: name-acronym-word
title: Treat acronyms as words in identifiers, not all-caps
category: naming
severity: warn
tier: 2
enforceability: gate
tags: [naming, acronyms, casing]
applies_to:
  - "type or function names containing acronyms like HTTP, JSON, TCP, UUID"
rust_edition: "2024"
msrv: null
detect:
  clippy: [upper_case_acronyms]
  grep: []
  ast_query: null
autofix: true
sources:
  - leonardomso/rust-skills#name-acronym-word
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Capitalize only the first letter of an acronym inside an identifier (`HttpServer`, `JsonParser`), never write it all-caps.

## Why
All-caps acronyms destroy word boundaries: `HTTPSHandler` could read as "HTTPS Handler" or "HTTP SHandler". Treating each acronym as an ordinary word keeps `UpperCamelCase` boundaries legible and matches the standard library (`TcpStream`, `IpAddr`, `UdpSocket`). Clippy's `upper_case_acronyms` flags the all-caps form.

## Do
```rust
struct HttpServer {
    port: u16,
}

struct JsonParser {
    input: String,
}

fn parse_json(input: &str) -> String {
    input.to_string()
}
```

## Don't
```rust
struct HTTPServer {
    port: u16,
}

struct JSONParser {
    input: String,
}

fn parse_json(input: &str) -> String {
    input.to_string()
}
```

## Notes
Two-letter acronyms like `Io` or `Id` may stay all-caps (`IO`, `ID`) and clippy accepts both, but treating them as words is preferred for consistency.
