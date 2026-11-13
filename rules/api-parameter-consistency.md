---
id: api-parameter-consistency
title: Keep parameter order consistent across functions sharing the same concepts
category: api-design
severity: info
tier: 1
enforceability: judge
tags: [api-design, ergonomics, consistency]
applies_to:
  - "sibling functions taking the same conceptual parameters in different order"
  - "a shared &Logger or context parameter placed inconsistently"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - microsoft/rust-guidelines#M-PARAMETER-CONSISTENCY
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
When the same conceptual parameters recur across multiple functions, give them the same order everywhere: call-specific parameters first, ubiquitous ones (a logger, a context) last, and closures always last of all.

## Why
Muscle memory forms around parameter position; a caller who has used `create_user(tenant_id, user_id, ..)` will reach for that order at every other call site. Flipping the order between sibling functions costs a lookup every time and invites transposed-argument bugs when two parameters share a type. Anchoring ubiquitous, rarely-varying parameters at the end keeps the call-specific, high-signal parameters together at the front.

## Do
```rust
struct TenantId(u64);
struct UserId(u64);
struct Logger;

fn create_user(tenant_id: TenantId, user_id: UserId, logger: &Logger) {}
fn delete_user(tenant_id: TenantId, user_id: UserId, logger: &Logger) {}
fn rename_user(tenant_id: TenantId, user_id: UserId, new_name: &str, logger: &Logger) {}
```

## Don't
```rust
struct TenantId(u64);
struct UserId(u64);
struct Logger;

fn create_user(logger: &Logger, user_id: UserId, tenant_id: TenantId) {}
fn delete_user(tenant_id: TenantId, user_id: UserId, logger: &Logger) {}
fn rename_user(user_id: UserId, new_name: &str, tenant_id: TenantId, logger: &Logger) {}
```

## Notes
Applies within a crate and across crates in the same ecosystem or API family. When a function takes exactly one closure, it goes last, after every other parameter.
