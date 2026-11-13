---
id: design-earn-the-abstraction
title: Add an abstraction only when a second real use needs it
category: design
language: typescript
ts_version: "7.0"
severity: info
tier: 1
enforceability: judge
tags: [design, simplicity]
applies_to:
  - "new interface with one implementation"
  - "factory for one class"
  - "generic used with one type"
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Write the direct code first; introduce an interface, generic, factory or base class only when a second real caller needs the variation.

## Why
Abstractions written for imagined futures add indirection that every reader pays for and that rarely fits the future that arrives. Models over-abstract because it looks thorough. One concrete function is easier to read, test and later generalise than a framework with one user.

## Do
```ts
export type Invoice = { readonly net: number; readonly taxRate: number };

export function total(invoice: Invoice): number {
  return invoice.net * (1 + invoice.taxRate);
}
```

## Don't
```ts
export interface TotalStrategy {
  compute(net: number, taxRate: number): number;
}

export class DefaultTotalStrategy implements TotalStrategy {
  compute(net: number, taxRate: number): number {
    return net * (1 + taxRate);
  }
}

export class TotalStrategyFactory {
  static create(): TotalStrategy {
    return new DefaultTotalStrategy();
  }
}
```
