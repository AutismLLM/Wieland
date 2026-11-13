---
id: bound-parse-at-the-edge
title: Treat untrusted input as unknown and check it once at the edge
category: boundaries
language: typescript
ts_version: "7.0"
severity: deny
tier: 0
enforceability: judge
tags: [boundaries, safety, parsing]
applies_to:
  - "JSON.parse"
  - "fetch().json()"
  - "localStorage.getItem"
  - "URLSearchParams"
  - "postMessage"
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Data from outside the program (`JSON.parse`, `response.json()`, storage, URL parameters, `postMessage`, environment) is `unknown` until one function checks it and returns a typed value; the rest of the code only sees the typed value.

## Why
The type system cannot see across the boundary. A cast there makes every later line trust data nobody checked, so a missing field crashes deep inside unrelated code. One parser per input keeps the trust decision in one place and gives a clear error at the moment bad data arrives.

## Do
```ts
type Settings = { readonly theme: "light" | "dark"; readonly fontSize: number };

export function parseSettings(raw: unknown): Settings {
  if (typeof raw !== "object" || raw === null) {
    throw new Error("settings must be an object");
  }
  const theme = "theme" in raw ? raw.theme : undefined;
  const fontSize = "fontSize" in raw ? raw.fontSize : undefined;
  if (theme !== "light" && theme !== "dark") {
    throw new Error("settings.theme must be light or dark");
  }
  if (typeof fontSize !== "number") {
    throw new Error("settings.fontSize must be a number");
  }
  return { theme, fontSize };
}

export async function loadSettings(url: string): Promise<Settings> {
  const response = await fetch(url);
  const body: unknown = await response.json();
  return parseSettings(body);
}
```

## Don't
```ts
type Settings = { readonly theme: "light" | "dark"; readonly fontSize: number };

export async function loadSettings(url: string): Promise<Settings> {
  const response = await fetch(url);
  return (await response.json()) as Settings;
}
```
