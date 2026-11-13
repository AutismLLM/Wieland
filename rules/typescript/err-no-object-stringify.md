---
id: err-no-object-stringify
title: Do not turn objects into text with String() or template literals
category: errors
language: typescript
ts_version: "7.0"
severity: warn
tier: 2
enforceability: check
tags: [errors, clarity]
applies_to:
  - "`${error}`"
  - "String(value)"
  - "value + \"\""
detect:
  oxlint: [typescript/no-base-to-string]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Do not stringify a value whose type has no useful `toString`; pick the field you mean, use `JSON.stringify`, or format an `Error` through its `message`.

## Why
Plain objects print as `[object Object]`, which lands in logs, error messages and the UI and tells nobody anything. It happens most in error paths, where the information matters most.

## Do
```ts
type Failure = { readonly code: number; readonly reason: string };

export function describe(failure: Failure): string {
  return `failed with ${failure.code}: ${failure.reason}`;
}

export function message(error: unknown): string {
  return error instanceof Error ? error.message : JSON.stringify(error);
}
```

## Don't
```ts
type Failure = { readonly code: number; readonly reason: string };

export function describe(failure: Failure): string {
  return `failed: ${String(failure)}`;
}
```
