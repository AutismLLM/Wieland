---
id: err-handle-once
title: "Handle an error once: log it or return it, not both"
category: errors
language: go
go_version: "1.26"
severity: warn
tier: 0
enforceability: judge
tags: [errors, logging]
applies_to:
  - "log.Printf(...); return err"
  - "logging at every layer"
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
At each level, either return the error with added context or handle it (log, retry, fall back) and stop; do not log and also return it.

## Why
Logging and returning makes the same failure appear once per layer, so one error becomes five log lines with no clear origin. Returning with context builds one message that says where and why; the top level logs it once.

## Do
```go
import (
	"fmt"
	"log/slog"
	"os"
)

func loadSettings(path string) ([]byte, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("load settings: %w", err)
	}
	return data, nil
}

func start(path string) {
	if _, err := loadSettings(path); err != nil {
		slog.Error("start failed", "err", err)
	}
}
```

## Don't
```go
func loadSettings(path string) ([]byte, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		slog.Error("read failed", "err", err)
		return nil, err
	}
	return data, nil
}
```
