---
id: proj-mod-by-feature
title: Organize modules by feature, not by technical layer
category: project-layout
severity: info
tier: 1
enforceability: judge
tags: [layout, modules, cohesion]
applies_to:
  - "modules split into controllers/, models/, services/ layers"
  - "code for one feature scattered across many folders"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#proj-mod-by-feature
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Group modules by feature (all of `user` together) rather than by technical layer (all handlers in one folder, all models in another).

## Why
Feature-based layout keeps related code together, so a change stays localized and a feature is understood, added, or deleted in one place. Layer-based layout scatters a single feature across four or more folders, forcing you to jump between them to follow one flow and leaving ownership unclear.

## Do
```rust
// src/lib.rs — one module per feature
mod user;
mod order;
mod product;

// src/user/mod.rs — everything for `user` lives here
mod model;
mod repository;
mod service;
mod handler;

pub use handler::router;
pub use model::{User, UserId};
```

## Don't
```rust
// src/lib.rs — sliced by technical layer
mod controllers;   // user_controller, order_controller, product_controller
mod models;        // user, order, product
mod services;      // user_service, order_service, product_service
mod repositories;  // user_repository, order_repository, product_repository
```

## Notes
Keep genuinely cross-cutting concerns (connection pool, shared error type, middleware) in a `shared` module. For large features, nest further by concern (`billing/invoice`, `billing/payment`), and do not over-nest tiny modules.
