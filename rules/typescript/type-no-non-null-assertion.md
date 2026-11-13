---
id: type-no-non-null-assertion
title: Do not use the non-null assertion operator
category: types
language: typescript
ts_version: "7.0"
severity: deny
tier: 3
enforceability: gate
tags: [types, safety]
applies_to:
  - "value!"
  - "map.get(key)!"
  - "document.querySelector(...)!"
detect:
  oxlint: [typescript/no-non-null-assertion]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Do not write postfix `!`; narrow with a check, give a default, or throw an error that says which invariant broke.

## Why
`!` tells the compiler a value cannot be `null` or `undefined` without proving it. When the claim is wrong the crash happens later, far from the cause, as "cannot read properties of undefined". With `noUncheckedIndexedAccess` on, `!` is the easy way to switch the check back off, so it spreads.

## Do
```ts
export function requireElement(id: string): HTMLElement {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`missing element #${id}`);
  }
  return element;
}

export function firstOr(values: readonly number[], fallback: number): number {
  return values[0] ?? fallback;
}
```

## Don't
```ts
export function requireElement(id: string): HTMLElement {
  return document.getElementById(id)!;
}

export function first(values: readonly number[]): number {
  return values[0]!;
}
```
