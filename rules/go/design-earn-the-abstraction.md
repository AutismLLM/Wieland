---
id: design-earn-the-abstraction
title: Add an abstraction only when a second real use needs it
category: design
language: go
go_version: "1.26"
severity: info
tier: 1
enforceability: judge
tags: [design, simplicity]
applies_to:
  - "interface with one implementation"
  - "generic used with one type"
  - "factory for one struct"
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Write the direct code first; introduce an interface, a generic function or a factory only when a second real caller needs the variation.

## Why
Indirection written for imagined futures costs every reader and rarely fits the future that comes. Go favours plain functions and concrete types; an interface is cheap to add later at the point of use.

## Do
```go
type Invoice struct {
	Net     float64
	TaxRate float64
}

func Total(inv Invoice) float64 {
	return inv.Net * (1 + inv.TaxRate)
}
```

## Don't
```go
type TotalStrategy interface {
	Compute(net, taxRate float64) float64
}

type defaultTotal struct{}

func (defaultTotal) Compute(net, taxRate float64) float64 {
	return net * (1 + taxRate)
}

func NewTotalStrategy() TotalStrategy {
	return defaultTotal{}
}
```
