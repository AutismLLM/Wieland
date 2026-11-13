---
id: pat-exhaustive-enum
title: Match owned enums exhaustively; avoid catch-all _ that hides new variants
category: patterns
severity: warn
tier: 2
enforceability: gate
tags: [pattern-matching, exhaustiveness, correctness]
applies_to:
  - "match on an enum you own"
  - "_ => wildcard arm on a local enum"
rust_edition: "2024"
msrv: null
detect:
  clippy: [wildcard_enum_match_arm]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#pat-exhaustive-enum
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Match enums you own exhaustively, listing every variant; reserve a `_` wildcard for foreign `#[non_exhaustive]` enums that force one.

## Why
A `_ =>` arm silently absorbs any variant added to an enum you own, turning what should be a compile-time error into a silent runtime no-op. Exhaustive matches make the compiler a checklist: add a variant and every unhandled site fails to build. Grouping shared variants with `|` keeps that guarantee while staying concise; a wildcard throws it away.

## Do
```rust
#[derive(Debug)]
enum Status {
    Active,
    Pending,
    Closed,
}

fn describe(s: &Status) -> &'static str {
    match s {
        Status::Active => "active",
        Status::Pending => "pending",
        Status::Closed => "closed",
    }
}

fn is_terminal(s: &Status) -> bool {
    match s {
        Status::Closed | Status::Pending => true,
        Status::Active => false,
    }
}
```

## Don't
```rust
#[derive(Debug)]
enum Status {
    Active,
    Pending,
    Closed,
}

fn describe(s: &Status) -> &'static str {
    match s {
        Status::Active => "active",
        _ => "inactive",
    }
}
```

## Notes
Foreign crates may mark enums `#[non_exhaustive]`, which forces a wildcard arm — that use of `_` is required by the language, so keep it and document that it is an intentional no-op for unknown variants.
