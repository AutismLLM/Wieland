---
id: type-no-unnecessary-assertion
title: Remove type assertions that do not change the type
category: types
language: typescript
ts_version: "7.0"
severity: info
tier: 2
enforceability: check
tags: [types, clarity]
applies_to:
  - "value as string on a string"
  - "value! on a non-nullable value"
detect:
  oxlint: [typescript/no-unnecessary-type-assertion]
autofix: true
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Delete `as T` and `!` when the expression already has that type.

## Why
A no-op assertion is noise today and a trap later: when the expression's type changes, the assertion forces the old type and hides the error the change should have caused.

## Do
```ts
export function shout(text: string): string {
  return text.toUpperCase();
}
```

## Don't
```ts
export function shout(text: string): string {
  return (text as string).toUpperCase();
}
```
