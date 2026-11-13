---
id: async-no-floating-promise
title: Await, return or explicitly void every promise
category: async
language: typescript
ts_version: "7.0"
severity: deny
tier: 3
enforceability: gate
tags: [async, correctness]
applies_to:
  - "calling an async function without await"
  - "fire-and-forget"
detect:
  oxlint: [typescript/no-floating-promises]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Every promise you create is awaited, returned, or handed to a handler that deals with rejection; a deliberate fire-and-forget is written as `void promise.catch(handler)`.

## Why
A promise nobody awaits loses its rejection: the error surfaces later as an unhandled rejection with no caller on the stack, or not at all, and the code after it runs before the work is done. Models drop `await` often because the call still type-checks.

## Do
```ts
declare function save(id: string): Promise<void>;
declare function report(error: unknown): void;

export async function saveAll(ids: readonly string[]): Promise<void> {
  for (const id of ids) {
    await save(id);
  }
}

export function saveInBackground(id: string): void {
  void save(id).catch(report);
}
```

## Don't
```ts
declare function save(id: string): Promise<void>;

export function saveAll(ids: readonly string[]): void {
  for (const id of ids) {
    save(id);
  }
}
```

## Notes
`void save(id)` alone silences the linter but still drops the rejection; pair `void` with `.catch`.
