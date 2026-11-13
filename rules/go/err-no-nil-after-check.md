---
id: err-no-nil-after-check
title: Do not return nil error after checking that err is not nil
category: errors
language: go
go_version: "1.26"
severity: deny
tier: 3
enforceability: gate
tags: [errors, correctness]
applies_to:
  - "if err != nil { return nil }"
  - "if err != nil { return value, nil }"
detect:
  golangci: [nilerr]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Inside `if err != nil`, return the error (wrapped) or handle it on purpose; never return `nil` as the error from that branch.

## Why
Returning `nil` from the error branch reports success for a failure. The caller continues with a zero value or partial result. It is a typical copy-paste slip that reads correct at a glance.

## Do
```go
import (
	"fmt"
	"strconv"
)

func parsePort(raw string) (int, error) {
	port, err := strconv.Atoi(raw)
	if err != nil {
		return 0, fmt.Errorf("parse port %q: %w", raw, err)
	}
	return port, nil
}
```

## Don't
```go
func parsePort(raw string) (int, error) {
	port, err := strconv.Atoi(raw)
	if err != nil {
		return 0, nil
	}
	return port, nil
}
```
