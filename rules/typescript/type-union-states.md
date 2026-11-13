---
id: type-union-states
title: Model states as a discriminated union, not booleans and optional fields
category: types
language: typescript
ts_version: "7.0"
severity: warn
tier: 0
enforceability: judge
tags: [types, design, unions]
applies_to:
  - "isLoading / error / data fields"
  - "several booleans describing one state"
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
When a value can be in one of several states, write one type per state with a literal `kind` (or `status`) field and join them in a union; do not combine booleans and optional fields.

## Why
`{ loading: boolean; error?: string; data?: T }` allows impossible mixes (loading with data and an error) and forces every reader to guess which fields are set. A union makes each state's fields required in that state, and the compiler narrows on `kind`.

## Do
```ts
export type Load<T> =
  | { readonly kind: "idle" }
  | { readonly kind: "loading" }
  | { readonly kind: "failed"; readonly error: string }
  | { readonly kind: "ready"; readonly data: T };

export function label(state: Load<string>): string {
  switch (state.kind) {
    case "idle":
      return "";
    case "loading":
      return "loading";
    case "failed":
      return state.error;
    case "ready":
      return state.data;
  }
}
```

## Don't
```ts
export type Load<T> = {
  loading: boolean;
  error?: string;
  data?: T;
};
```
