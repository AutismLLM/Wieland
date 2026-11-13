---
id: anti-no-unused-params
title: Remove parameters and results a function never uses
category: anti-patterns
language: go
go_version: "1.26"
severity: info
tier: 2
enforceability: check
tags: [simplicity, api]
applies_to:
  - "parameter never read"
  - "result always nil"
detect:
  golangci: [unparam]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Drop a parameter the function never reads and a result it always returns as the same value, unless an interface or callback signature requires them.

## Why
Unused parameters make callers compute and pass values that do nothing; an always-nil error result makes them write error handling for a failure that cannot happen.

## Do
```go
func greeting(name string) string {
	return "hello " + name
}
```

## Don't
```go
func greeting(name string, verbose bool) (string, error) {
	return "hello " + name, nil
}
```

## Notes
Advisory: a function that satisfies an interface or `http.HandlerFunc` must keep the signature; name an unused parameter `_`.
