---
id: type-exhaustive-switch
title: Handle every constant of an enum type in a switch
category: types
language: go
go_version: "1.26"
severity: warn
tier: 2
enforceability: check
tags: [types, enums]
applies_to:
  - "switch on a named constant type"
  - "adding a new const to an iota block"
detect:
  golangci: [exhaustive]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
A `switch` over a named type with declared constants lists every constant, or has a `default` that reports the unexpected value.

## Why
Go enums are open: adding a constant compiles everywhere and silently falls through every switch that does not know it. The check turns a new state into a list of places to update.

## Do
```go
import "fmt"

type State int

const (
	Idle State = iota
	Running
	Stopped
)

func (s State) String() string {
	switch s {
	case Idle:
		return "idle"
	case Running:
		return "running"
	case Stopped:
		return "stopped"
	default:
		return fmt.Sprintf("State(%d)", int(s))
	}
}
```

## Don't
```go
func (s State) String() string {
	switch s {
	case Idle:
		return "idle"
	case Running:
		return "running"
	}
	return ""
}
```

## Notes
Advisory: some switches handle a subset on purpose. Keep the `default` explicit so the intent is visible.
