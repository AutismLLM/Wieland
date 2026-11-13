---
id: type-exhaustive-switch
title: Make every switch over a union handle every member
category: types
language: typescript
ts_version: "7.0"
severity: deny
tier: 3
enforceability: gate
tags: [types, correctness, unions]
applies_to:
  - "switch on a discriminant"
  - "adding a member to a union"
detect:
  oxlint: [typescript/switch-exhaustiveness-check]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
A `switch` over a union type or literal union has a case for every member; when a default is needed, it proves exhaustiveness with a `never` check instead of silently swallowing new members.

## Why
Unions grow. A switch that misses a member compiles, falls through to nothing, and returns `undefined` or skips work for the new case. The exhaustiveness check turns "added a state" into a list of compile errors at every place that must handle it.

## Do
```ts
type Shape =
  | { readonly kind: "circle"; readonly radius: number }
  | { readonly kind: "square"; readonly side: number };

export function area(shape: Shape): number {
  switch (shape.kind) {
    case "circle":
      return Math.PI * shape.radius ** 2;
    case "square":
      return shape.side ** 2;
    default: {
      const unreachable: never = shape;
      throw new Error(`unhandled shape ${JSON.stringify(unreachable)}`);
    }
  }
}
```

## Don't
```ts
type Shape =
  | { readonly kind: "circle"; readonly radius: number }
  | { readonly kind: "square"; readonly side: number }
  | { readonly kind: "triangle"; readonly base: number; readonly height: number };

export function area(shape: Shape): number {
  switch (shape.kind) {
    case "circle":
      return Math.PI * shape.radius ** 2;
    case "square":
      return shape.side ** 2;
    default:
      return 0;
  }
}
```
