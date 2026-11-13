---
id: syn-erasable-only
title: "Write only erasable TypeScript: no enum, namespace or parameter properties"
category: syntax
language: typescript
ts_version: "7.0"
severity: deny
tier: 0
enforceability: gate
tags: [syntax, runtime]
applies_to:
  - "enum"
  - "namespace"
  - "constructor(private x)"
detect:
  tsconfig: [erasableSyntaxOnly]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Use only syntax that disappears when types are stripped: replace `enum` with an `as const` object plus a union type, runtime `namespace` with modules, and constructor parameter properties with explicit fields.

## Why
Node runs `.ts` by stripping types, and bundlers do the same per file. `enum`, runtime `namespace` and parameter properties need code generation, so they fail at runtime with `ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX`. `erasableSyntaxOnly` makes `tsc` report them first. Models write enums from habit, so this rule is always on.

## Do
```ts
export const Status = {
  Active: "active",
  Paused: "paused",
} as const;

export type Status = (typeof Status)[keyof typeof Status];

export class Account {
  readonly id: string;
  status: Status;

  constructor(id: string, status: Status) {
    this.id = id;
    this.status = status;
  }
}
```

## Don't
```ts
export enum Status {
  Active = "active",
  Paused = "paused",
}

export class Account {
  constructor(
    readonly id: string,
    public status: Status,
  ) {}
}
```
