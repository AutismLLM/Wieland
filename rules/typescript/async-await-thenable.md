---
id: async-await-thenable
title: Only await values that can be a promise
category: async
language: typescript
ts_version: "7.0"
severity: deny
tier: 3
enforceability: gate
tags: [async, correctness]
applies_to:
  - "await on a plain value"
  - "await on an array of promises"
detect:
  oxlint: [typescript/await-thenable]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
`await` only expressions whose type can be a thenable; to wait for several promises, `await Promise.all(...)`.

## Why
Awaiting a non-promise is a no-op that hides a mistake: usually a missing `Promise.all`, so the array of promises is "awaited" and the work keeps running unobserved, or a function that was made sync while its callers still await it.

## Do
```ts
declare function load(id: string): Promise<string>;

export async function loadAll(ids: readonly string[]): Promise<string[]> {
  return await Promise.all(ids.map((id) => load(id)));
}
```

## Don't
```ts
declare function load(id: string): Promise<string>;

export async function loadAll(ids: readonly string[]): Promise<Promise<string>[]> {
  return await ids.map((id) => load(id));
}
```
