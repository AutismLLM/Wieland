---
id: err-sentinel-naming
title: Name sentinel errors ErrX and error types XError
category: errors
language: go
go_version: "1.26"
severity: info
tier: 2
enforceability: check
tags: [errors, naming]
applies_to:
  - "var notFound = errors.New"
  - "type NotFound struct"
detect:
  golangci: [errname]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Sentinel error variables start with `Err` (`ErrNotFound`), error types end with `Error` (`ValidationError`).

## Why
The convention tells every reader and every `errors.Is`/`errors.As` call site what a name is without opening its declaration, and it is what the standard library does.

## Do
```go
import "errors"

var ErrNotFound = errors.New("not found")

type ValidationError struct {
	Field string
}

func (e *ValidationError) Error() string {
	return "invalid " + e.Field
}
```

## Don't
```go
var NotFound = errors.New("not found")

type Validation struct {
	Field string
}

func (e *Validation) Error() string {
	return "invalid " + e.Field
}
```
