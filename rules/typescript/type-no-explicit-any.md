---
id: type-no-explicit-any
title: Do not write any; use unknown and narrow
category: types
language: typescript
ts_version: "7.0"
severity: deny
tier: 3
enforceability: gate
tags: [types, safety]
applies_to:
  - "any"
  - "as any"
  - "Record<string, any>"
detect:
  oxlint: [typescript/no-explicit-any]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Never write `any`. Use `unknown` for values you have not checked yet, a generic for values you pass through, and a real type everywhere else.

## Why
`any` turns type checking off for the value and for everything it touches: properties, calls and assignments all pass, and the error moves to runtime. `unknown` keeps the same flexibility at the boundary but forces a check before use.

## Do
```ts
export function parseCount(text: string): number {
  const value: unknown = JSON.parse(text);
  if (typeof value !== "number" || !Number.isInteger(value)) {
    throw new Error("count must be an integer");
  }
  return value;
}

export function firstItem<T>(items: readonly T[]): T | undefined {
  return items[0];
}
```

## Don't
```ts
export function parseCount(text: string): number {
  const value: any = JSON.parse(text);
  return value.count;
}

export function firstItem(items: any[]): any {
  return items[0];
}
```
