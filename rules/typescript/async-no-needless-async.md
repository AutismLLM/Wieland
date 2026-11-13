---
id: async-no-needless-async
title: Do not mark a function async when it never awaits
category: async
language: typescript
ts_version: "7.0"
severity: info
tier: 2
enforceability: check
tags: [async, clarity]
applies_to:
  - "async function without await"
detect:
  oxlint: [typescript/require-await]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Mark a function `async` only when it awaits; otherwise return the value, or the promise, directly.

## Why
`async` without `await` wraps the result in a promise for no reason, forces every caller to await, and changes how thrown errors arrive (as rejections instead of synchronous throws). It is usually left over from a refactor.

## Do
```ts
declare function fetchName(id: string): Promise<string>;

export function name(id: string): Promise<string> {
  return fetchName(id);
}

export function label(id: string): string {
  return `user ${id}`;
}
```

## Don't
```ts
export async function label(id: string): Promise<string> {
  return `user ${id}`;
}
```

## Notes
Advisory: an interface may require a promise-returning method whose implementation is synchronous; there `async` is fine.
