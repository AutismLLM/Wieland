---
id: name-is-has-bool
title: Prefix boolean methods with is_, has_, can_, or should_
category: naming
severity: info
tier: 1
enforceability: check
tags: [naming, api, readability]
applies_to:
  - "method returning bool"
  - "predicate without a verb prefix"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: ['-> bool']
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#name-is-has-bool
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Name boolean-returning methods with an `is_`, `has_`, `can_`, or `should_` prefix so they read as the yes/no question they answer.

## Why
A boolean method answers a yes/no question, and the prefix makes that explicit: `user.is_active()` and `buffer.has_remaining()` read naturally at the call site. Without a prefix, `user.active()` is ambiguous — does it check state or activate it? — and the reader must consult the docs or the return type to be sure. The prefix removes that ambiguity and matches the standard library's convention (`is_empty`, `is_some`, `is_ok`).

## Do
```rust
impl User {
    fn is_active(&self) -> bool { self.active }
    fn is_admin(&self) -> bool { self.role == Role::Admin }
    fn has_permission(&self, perm: Permission) -> bool { self.perms.contains(&perm) }
    fn can_edit(&self) -> bool { self.is_active() && self.is_admin() }
}

if user.is_active() && user.has_permission(Permission::Write) {
    // ...
}
```

## Don't
```rust
impl User {
    fn active(&self) -> bool { self.active }
    fn deleted(&self) -> bool { self.deleted }
    fn admin(&self) -> bool { self.role == Role::Admin }
}

if user.active() { /* checking, or activating? */ }
```

## Notes
Methods that take an argument and read as a verb phrase (`str::contains`, `starts_with`) don't need the prefix. Prefer the positive form with caller-side negation (`!user.is_active()`) over a separate `is_inactive` to avoid double negatives. Plain struct *fields* may drop the prefix (`enabled: bool`), but accessor methods keep it.
