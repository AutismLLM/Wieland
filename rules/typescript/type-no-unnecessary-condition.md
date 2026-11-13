---
id: type-no-unnecessary-condition
title: Remove conditions the types already decide
category: types
language: typescript
ts_version: "7.0"
severity: warn
tier: 2
enforceability: check
tags: [types, clarity]
applies_to:
  - "value ?? fallback on a non-nullable value"
  - "if on an always-truthy value"
detect:
  oxlint: [typescript/no-unnecessary-condition]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Do not test what the type already guarantees: no `??` on a non-nullable value, no `?.` on a defined object, no `if` on a value that is always truthy.

## Why
A dead check misleads the reader into thinking the value can be missing, and it often hides a type that is wrong: the value really can be missing, but its type says otherwise. Either the check goes or the type gets fixed.

## Do
```ts
type Config = { readonly name: string; readonly label?: string };

export function title(config: Config): string {
  return config.label ?? config.name;
}
```

## Don't
```ts
type Config = { readonly name: string; readonly label?: string };

export function title(config: Config): string {
  return config.label ?? config.name ?? "untitled";
}
```
