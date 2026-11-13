---
id: async-no-misused-promise
title: Do not pass a promise where a plain value or sync callback is expected
category: async
language: typescript
ts_version: "7.0"
severity: deny
tier: 3
enforceability: gate
tags: [async, correctness]
applies_to:
  - "async callback to forEach or addEventListener"
  - "promise in an if condition"
detect:
  oxlint: [typescript/no-misused-promises]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Never use a promise as a condition, and never pass an async function to an API that ignores its return value (`forEach`, event listeners, `setTimeout`); wrap it so the promise is handled.

## Why
A promise object is always truthy, so `if (isAllowed())` is always true. A sync-callback API drops the returned promise, so rejections vanish and work overlaps without ordering. Both compile cleanly.

## Do
```ts
declare function isAllowed(user: string): Promise<boolean>;
declare function sync(): Promise<void>;
declare function report(error: unknown): void;

export async function guard(user: string): Promise<string> {
  if (await isAllowed(user)) {
    return "ok";
  }
  return "denied";
}

export function bind(button: EventTarget): void {
  button.addEventListener("click", () => {
    sync().catch(report);
  });
}
```

## Don't
```ts
declare function isAllowed(user: string): Promise<boolean>;
declare function sync(): Promise<void>;

export function guard(user: string): string {
  if (isAllowed(user)) {
    return "ok";
  }
  return "denied";
}

export function bind(button: EventTarget): void {
  button.addEventListener("click", async () => {
    await sync();
  });
}
```
