---
id: mod-explicit-type-imports
title: Import types with import type
category: modules
language: typescript
ts_version: "7.0"
severity: info
tier: 1
enforceability: judge
tags: [modules, syntax]
applies_to:
  - "import { Type } from"
  - "verbatimModuleSyntax error TS1484"
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Import names used only as types with `import type` (or an inline `type` modifier), and keep value imports for values.

## Why
Under `verbatimModuleSyntax`, which this pack requires, the emitter keeps every non-type import as written, so a type-only name imported as a value is an error (TS1484) or leaves a runtime import of a module that may have side effects or not exist at runtime. Writing it explicitly also tells the reader the dependency is compile-time only.

## Do
```ts
import type { IncomingMessage } from "node:http";
import { type Stats, statSync } from "node:fs";

export function size(path: string): number {
  const stats: Stats = statSync(path);
  return stats.size;
}

export function method(request: IncomingMessage): string {
  return request.method ?? "GET";
}
```

## Don't
```ts
import { IncomingMessage } from "node:http";

export function method(request: IncomingMessage): string {
  return request.method ?? "GET";
}
```
