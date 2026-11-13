---
id: err-wrap-with-w
title: Wrap errors with %w and inspect them with errors.Is and errors.As
category: errors
language: go
go_version: "1.26"
severity: deny
tier: 3
enforceability: gate
tags: [errors, correctness]
applies_to:
  - "fmt.Errorf(\"...: %v\", err)"
  - "err == ErrX"
  - "err.(*MyError)"
detect:
  golangci: [errorlint]
autofix: true
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Add context with `fmt.Errorf("...: %w", err)`, and compare or unpack errors with `errors.Is` and `errors.As`, never with `==` or a type assertion.

## Why
`%v` flattens the error into text, so callers can no longer detect the cause. `==` and `err.(*T)` only see the outermost error and fail as soon as anyone adds context. The mismatch compiles and passes tests until a wrap appears in between.

## Do
```go
import (
	"errors"
	"fmt"
	"io/fs"
	"os"
)

func readConfig(path string) ([]byte, error) {
	data, err := os.ReadFile(path)
	if errors.Is(err, fs.ErrNotExist) {
		return nil, nil
	}
	if err != nil {
		return nil, fmt.Errorf("read config %s: %w", path, err)
	}
	return data, nil
}

func pathOf(err error) string {
	var pathErr *fs.PathError
	if errors.As(err, &pathErr) {
		return pathErr.Path
	}
	return ""
}
```

## Don't
```go
func readConfig(path string) ([]byte, error) {
	data, err := os.ReadFile(path)
	if err == fs.ErrNotExist {
		return nil, nil
	}
	if err != nil {
		return nil, fmt.Errorf("read config %s: %v", path, err)
	}
	return data, nil
}
```
