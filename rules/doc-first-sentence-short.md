---
id: doc-first-sentence-short
title: Keep a doc comment's first sentence one short line (~15 words)
category: docs
severity: info
tier: 1
enforceability: judge
tags: [rustdoc, summary, readability]
applies_to:
  - "the opening line of a /// or //! doc comment"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-FIRST-DOC-SENTENCE
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Write the first sentence of a doc comment as one short line, roughly 15 words or fewer, then blank-line into extended documentation.

## Why
Rustdoc extracts an item's first sentence as its summary and shows it wherever the item is listed — module indexes, search results, autocomplete. A long, wrapped first sentence reads as an ugly, hard-to-skim fragment in those cramped contexts; a short one stays legible everywhere it is reused.

## Do
```rust
/// Parses a duration from a human-readable string.
///
/// Accepts suffixes `s`, `m`, `h`, and `d`, and combinations such as
/// `"1h30m"`. Returns an error if the string is empty or contains an
/// unrecognized suffix.
pub fn parse_duration(s: &str) -> Result<Duration, Error> {
    todo!()
}
```

## Don't
```rust
/// Parses a duration from a human-readable string, accepting suffixes
/// like `s`, `m`, `h`, and `d` as well as combinations of them such as `"1h30m"`,
/// and returns an error if the input is empty or has an unrecognized suffix.
pub fn parse_duration(s: &str) -> Result<Duration, Error> {
    todo!()
}
```

## Notes
Put every detail that doesn't fit the summary after the blank line — it still renders on the item's own page, just not in listings.
