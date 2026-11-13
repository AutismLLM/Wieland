---
id: err-explain-swallowed-catch
title: A catch that drops the error says why
category: errors
language: typescript
ts_version: "7.0"
severity: warn
tier: 0
enforceability: judge
tags: [errors, clarity]
applies_to:
  - "catch {}"
  - "catch (e) { return null }"
  - ".catch(() => undefined)"
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Rethrow, report, or turn the error into a value the caller sees; if a catch block deliberately drops the error, it handles only the expected failure and its code makes the reason obvious.

## Why
An empty catch turns every failure, including bugs, into silent wrong behaviour. Models add `try/catch` to make an error "go away". Catching narrowly and naming the expected case keeps real bugs visible.

## Do
```ts
export function readDraft(storage: Storage, key: string): string {
  try {
    return storage.getItem(key) ?? "";
  } catch (error) {
    if (error instanceof DOMException && error.name === "SecurityError") {
      return "";
    }
    throw error;
  }
}
```

## Don't
```ts
export function readDraft(storage: Storage, key: string): string {
  try {
    return storage.getItem(key) ?? "";
  } catch {
    return "";
  }
}
```

## Notes
This pack writes no code comments as a house style, so the reason lives in the code: a named check for the expected error, or a small function whose name says it.
