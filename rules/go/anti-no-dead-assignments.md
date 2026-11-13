---
id: anti-no-dead-assignments
title: Remove unused code and assignments that are never read
category: anti-patterns
language: go
go_version: "1.26"
severity: deny
tier: 3
enforceability: gate
tags: [simplicity, correctness]
applies_to:
  - "x := ...; x = ..."
  - "unused function"
  - "unused field"
detect:
  golangci: [ineffassign, unused]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Delete assignments whose value is never read and functions, types, fields and constants nothing uses.

## Why
A value overwritten before it is read is often a missed `if err != nil` between two calls. Unused declarations are what a model leaves behind after a rewrite; they mislead readers into thinking the code is used.

## Do
```go
import (
	"fmt"
	"os"
)

func load(a, b string) ([]byte, []byte, error) {
	first, err := os.ReadFile(a)
	if err != nil {
		return nil, nil, fmt.Errorf("read %s: %w", a, err)
	}
	second, err := os.ReadFile(b)
	if err != nil {
		return nil, nil, fmt.Errorf("read %s: %w", b, err)
	}
	return first, second, nil
}
```

## Don't
```go
func load(a, b string) ([]byte, []byte, error) {
	first, err := os.ReadFile(a)
	second, err := os.ReadFile(b)
	if err != nil {
		return nil, nil, err
	}
	return first, second, nil
}
```
