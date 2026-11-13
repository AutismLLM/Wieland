---
id: name-types-camel
title: Use UpperCamelCase for types, traits, and enums
category: naming
severity: warn
tier: 2
enforceability: check
tags: [naming, types, casing]
applies_to:
  - "struct, trait, enum, and type alias declarations"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['\b(struct|trait|enum)\s+[a-z]']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-types-camel
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Name every type, trait, and enum in `UpperCamelCase`.

## Why
Rust splits casing by role: `UpperCamelCase` marks a type and `snake_case` a value, so `HttpClient` and `send_request` are distinguishable on sight. The compiler warns on lowercase type names, and screaming case reads as a constant, so any other convention fights the reader's expectations.

## Do
```rust
struct HttpClient {
    base_url: String,
}

trait Serializable {}

enum ResponseType {
    Success,
    Failure,
}
```

## Don't
```rust
struct http_client {
    base_url: String,
}

trait serializable {}

enum response_type {
    success,
    failure,
}
```

## Notes
Type aliases use the same casing: `type BoxedError = Box<dyn std::error::Error>;`.
