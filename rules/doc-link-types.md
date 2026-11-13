---
id: doc-link-types
title: Connect related types and functions with intra-doc links
category: docs
severity: info
tier: 1
enforceability: judge
tags: [rustdoc, links, navigation]
applies_to:
  - "doc comment describing related types or functions"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#doc-link-types
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Link every type, method, and module you mention in docs with intra-doc link syntax so related items are one click apart.

## Why
`[`TypeName`]` links let readers jump between a function and its result, error, and helper types, and the compiler verifies each target exists. Plain-text names are neither navigable nor checked, so they silently rot as the API changes. Denying `rustdoc::broken_intra_doc_links` catches breakage at build time.

## Do
```rust
/// Parses input and returns a [`ParseResult`].
///
/// # Errors
///
/// Returns [`ParseError::InvalidSyntax`] if the input contains invalid tokens.
///
/// # Related
///
/// - [`Tokenizer`] — the underlying tokenizer used by this parser
/// - [`ParseOptions`] — configuration options for parsing
pub fn parse(input: &str) -> ParseResult {
    todo!()
}
```

## Don't
```rust
/// Parses input and returns a ParseResult.
///
/// See also: ParseError for error types.
/// Uses the Tokenizer internally.
pub fn parse(input: &str) -> ParseResult {
    todo!()
}
```

## Notes
Enforce it in `Cargo.toml` with `[lints.rustdoc] broken_intra_doc_links = "deny"`.
