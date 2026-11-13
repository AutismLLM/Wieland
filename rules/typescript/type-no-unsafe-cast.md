---
id: type-no-unsafe-cast
title: Do not narrow a type with an as assertion
category: types
language: typescript
ts_version: "7.0"
severity: warn
tier: 2
enforceability: check
tags: [types, safety]
applies_to:
  - "value as Foo"
  - "as unknown as"
detect:
  oxlint: [typescript/no-unsafe-type-assertion]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Do not use `as` to turn a wider type into a narrower one; check the value, or use `satisfies` when you only want to confirm a shape.

## Why
`as` is a claim the compiler cannot verify. When the data does not match, the error appears later where the missing field is used. Most `as` casts at a boundary are a parse that was skipped.

## Do
```ts
type User = { readonly name: string };

function isUser(value: unknown): value is User {
  return typeof value === "object" && value !== null && "name" in value && typeof value.name === "string";
}

export function readUser(text: string): User {
  const value: unknown = JSON.parse(text);
  if (!isUser(value)) {
    throw new Error("not a user");
  }
  return value;
}
```

## Don't
```ts
type User = { readonly name: string };

export function readUser(text: string): User {
  return JSON.parse(text) as User;
}
```

## Notes
Advisory: some casts are unavoidable at framework edges (DOM event targets, generic containers). Keep each one next to the check that justifies it.
