---
id: trait-inherent-first
title: Put essential functionality on inherent methods and have trait impls forward to them
category: traits
severity: info
tier: 1
enforceability: judge
tags: [api-design, discoverability, traits]
applies_to:
  - "core method defined only inside a trait impl"
  - "type whose primary functionality requires an extra use Trait import"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-ESSENTIAL-FN-INHERENT
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Implement a type's essential functionality as inherent methods first; when the same behavior is also exposed through a trait, make the trait method a thin forwarder to the inherent one, not the only place the logic lives.

## Why
Inherent methods resolve without any `use` beyond the type itself, so they show up in autocomplete and docs the moment a user has a value in hand. Functionality that lives only in a trait impl forces every caller to first discover and import that trait — an easy way to make a type's main feature invisible. Forwarding keeps a single implementation while still satisfying the trait bound generic callers may need.

## Do
```rust
trait Download {
    fn download_file(&self, url: &str);
}

struct HttpClient;

impl HttpClient {
    fn download_file(&self, url: &str) {
        let _ = url;
    }
}

impl Download for HttpClient {
    fn download_file(&self, url: &str) {
        Self::download_file(self, url)
    }
}
```

## Don't
```rust
trait Download {
    fn download_file(&self, url: &str);
}

struct HttpClient;

impl Download for HttpClient {
    fn download_file(&self, url: &str) {
        let _ = url;
    }
}
```

## Notes
This applies to a type's primary, defining operations, not every trait a type happens to implement — `Display`, `Clone`, and marker traits have no inherent counterpart to forward to.
