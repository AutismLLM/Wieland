---
id: cfg-strict-compiler-flags
title: Turn on the strict compiler flags the pack relies on
category: config
language: typescript
ts_version: "7.0"
severity: deny
tier: 3
enforceability: gate
tags: [config, strictness]
applies_to:
  - "tsconfig.json"
  - "compilerOptions"
detect:
  tsconfig: [strict, noUncheckedIndexedAccess, exactOptionalPropertyTypes, noImplicitOverride, noImplicitReturns, noFallthroughCasesInSwitch, noPropertyAccessFromIndexSignature, verbatimModuleSyntax, isolatedModules]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Every `tsconfig*.json` in the project resolves these options to `true`: `strict`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noImplicitOverride`, `noImplicitReturns`, `noFallthroughCasesInSwitch`, `noPropertyAccessFromIndexSignature`, `verbatimModuleSyntax`, `isolatedModules`.

## Why
The other rules in this pack assume them. `noUncheckedIndexedAccess` makes `array[i]` possibly `undefined`, `exactOptionalPropertyTypes` separates "missing" from "set to undefined", and `verbatimModuleSyntax` with `isolatedModules` keeps every file compilable on its own, which is what type-stripping runtimes and bundlers do. Without them the compiler accepts code that fails at runtime.

## Do
```json
{
  "compilerOptions": {
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true,
    "noImplicitOverride": true,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true,
    "noPropertyAccessFromIndexSignature": true,
    "verbatimModuleSyntax": true,
    "isolatedModules": true,
    "erasableSyntaxOnly": true
  }
}
```

## Don't
```json
{
  "compilerOptions": {
    "strict": false,
    "skipLibCheck": true
  }
}
```

## Notes
The gate reads the resolved options with `tsc -p <file> --showConfig`, so values inherited through `extends` count. `strict` is already the default in TypeScript 7; setting it explicitly documents intent.
