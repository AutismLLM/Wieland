---
id: lint-prefer-expect-attribute
title: "Override a lint locally with #[expect(..., reason = \"...\")] instead of #[allow]"
category: lints
severity: info
tier: 2
enforceability: gate
tags: [clippy, lints, expect, maintainability]
applies_to:
  - "#[allow(...)] on a submodule, item, or statement"
  - "lint overrides intended to be temporary or scoped"
rust_edition: "2024"
msrv: "1.81"
detect:
  clippy: [allow_attributes]
  grep: ['#\[allow\(']
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-LINT-OVERRIDE-EXPECT
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When silencing a project-wide lint on a specific item or block, use `#[expect(lint, reason = "...")]` rather than `#[allow(lint)]`.

## Why
`#[allow]` silences a lint indefinitely, even after the code changes and the lint would no longer fire — the override rots silently and can mask a future regression. `#[expect]` (stable since Rust 1.81) instead emits a warning the moment the marked lint stops triggering, so a stale suppression surfaces on its own rather than lingering unnoticed.

## Do
```rust
#[expect(clippy::unused_async, reason = "API fixed, will use I/O later")]
pub async fn ping_server() {}
```

## Don't
```rust
#[allow(clippy::unused_async)]
pub async fn ping_server() {}
```

## Notes
`#[expect]` (stable since Rust 1.81) emits a warning the day the marked lint stops firing, so stale overrides surface instead of accumulating silently forever. `#[allow]` is still the right call in generated code and inside macro expansions, where an `#[expect]` could go unfulfilled for reasons outside your control. Clippy's `allow_attributes` lint flags plain `#[allow]` uses that should be `#[expect]`.
