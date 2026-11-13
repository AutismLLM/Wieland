---
id: proj-pub-super-parent
title: Use pub(super) for helpers shared only with the parent module
category: project-layout
severity: info
tier: 1
enforceability: judge
tags: [visibility, encapsulation, modules]
applies_to:
  - "helpers submodules share but the rest of the crate should not see"
  - "pub(crate) items only used within one module tree"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-pub-super-parent
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Scope items that only a module and its submodules need with `pub(super)` rather than widening them to `pub(crate)`.

## Why
`pub(super)` exposes an item to the immediate parent module and its children only. For helper functions and types shared between sibling submodules — a `Token` shared by `lexer` and `ast` — this keeps the shared surface inside the module tree that owns it, so the rest of the crate cannot reach in and couple to a local detail.

## Do
```rust
// src/parser/mod.rs — shared only within parser/*
pub(super) struct Token {
    pub(super) kind: TokenKind,
    pub(super) span: Span,
}

pub(super) fn shared_helper() -> Token {
    Token { kind: TokenKind::Eof, span: Span::dummy() }
}

// src/parser/lexer.rs
use super::{shared_helper, Token};

pub fn lex(_input: &str) -> Vec<Token> {
    vec![shared_helper()]
}
```

## Don't
```rust
// src/parser/lexer.rs — leaks a parser-only helper to the whole crate
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: Span,
}

pub(crate) fn shared_helper() -> Token {
    Token { kind: TokenKind::Eof, span: Span::dummy() }
}
```

## Notes
Use `pub(super)` for sibling-shared helpers, submodule-only types, and module-tree test utilities; step up to `pub(crate)` only when something genuinely needs to be crate-wide.
