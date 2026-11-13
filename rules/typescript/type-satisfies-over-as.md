---
id: type-satisfies-over-as
title: Use satisfies to check a value's shape, not as
category: types
language: typescript
ts_version: "7.0"
severity: warn
tier: 0
enforceability: judge
tags: [types, safety]
applies_to:
  - "const config = {...} as Config"
  - "object literal with a type assertion"
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
To confirm that a literal matches a type, write `value satisfies Type`; keep `as` out of it.

## Why
`as Type` accepts a literal with missing or misspelled fields and widens it to the declared type. `satisfies` reports both mistakes and keeps the literal's narrow type, so `as const` keys and literal values stay usable.

## Do
```ts
type Route = { readonly path: string; readonly auth: boolean };

export const routes = {
  home: { path: "/", auth: false },
  settings: { path: "/settings", auth: true },
} as const satisfies Record<string, Route>;

export const settingsPath = routes.settings.path;
```

## Don't
```ts
type Route = { readonly path: string; readonly auth: boolean };

export const routes = {
  home: { path: "/", auth: false },
  settings: { path: "/settings" },
} as Record<string, Route>;
```
