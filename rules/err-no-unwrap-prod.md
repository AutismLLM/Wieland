---
id: err-no-unwrap-prod
title: Avoid unwrap in production; use ?, expect, or explicit handling
category: errors
severity: deny
tier: 0
enforceability: gate
tags: [correctness, errors, safety]
applies_to:
  - ".unwrap()"
  - "Option or Result on a request/hot path"
rust_edition: "2024"
msrv: null
detect:
  clippy: [unwrap_used]
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#err-no-unwrap-prod
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Replace `unwrap()` in production code with `?` to propagate, `expect("invariant")` for provable invariants, or an explicit `match`/default.

## Why
`unwrap()` panics on `None` or `Err` with no context about which operation failed or why, producing cryptic crash messages like `called Option::unwrap() on a None value` that are hard to trace back to a line. Propagating with `?` preserves the error type, `expect()` documents the invariant you are relying on, and explicit handling lets you log or fall back. Each choice keeps the failure legible.

## Do
```rust
fn process_request(req: Request) -> Result<Response, AppError> {
    let user_id = req.headers
        .get("X-User-Id")
        .ok_or(AppError::MissingHeader("X-User-Id"))?;

    let user = database.find_user(user_id)?;

    let theme = user.preferences
        .get("theme")
        .ok_or(AppError::MissingPreference("theme"))?;

    Ok(Response::new(theme))
}
```

## Don't
```rust
fn process_request(req: Request) -> Response {
    let user_id = req.headers.get("X-User-Id").unwrap();
    let user = database.find_user(user_id).unwrap();
    let data = user.preferences.get("theme").unwrap();
    Response::new(data)
}
```

## Notes
When a value truly cannot be absent, prefer `expect("config must contain 'port' after validation")` over `unwrap()` so the panic explains the broken invariant. Provide `unwrap_or`/`unwrap_or_default`/`unwrap_or_else` when a sensible fallback exists.
