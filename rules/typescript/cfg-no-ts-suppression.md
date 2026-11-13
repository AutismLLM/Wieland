---
id: cfg-no-ts-suppression
title: Do not suppress compiler errors with ts-ignore or ts-nocheck
category: config
language: typescript
ts_version: "7.0"
severity: deny
tier: 3
enforceability: gate
tags: [config, safety]
applies_to:
  - "// @ts-ignore"
  - "// @ts-nocheck"
  - "// @ts-expect-error"
detect:
  oxlint: [typescript/ban-ts-comment]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Do not write `@ts-ignore` or `@ts-nocheck`; fix the type error. `@ts-expect-error` is allowed only with a description of why the error is expected.

## Why
A suppression hides the error it was written for and every later error on that line, so it outlives the reason it was added. It is the most common way a model "fixes" a type error it does not understand.

## Do
```ts
export function port(raw: string | undefined): number {
  const value = Number(raw ?? "8080");
  if (!Number.isInteger(value)) {
    throw new Error(`invalid port ${raw ?? ""}`);
  }
  return value;
}
```

## Don't
```ts
export function port(raw: string | undefined): number {
  // @ts-ignore
  return parseInt(raw);
}
```

## Notes
In test code that checks a type error on purpose, `// @ts-expect-error -- passing a number must be rejected` documents the expectation and fails once the error disappears.
