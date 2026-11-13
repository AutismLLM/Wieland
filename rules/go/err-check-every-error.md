---
id: err-check-every-error
title: Check every returned error
category: errors
language: go
go_version: "1.26"
severity: deny
tier: 0
enforceability: gate
tags: [errors, correctness]
applies_to:
  - "f.Close()"
  - "w.Write(...)"
  - "json.Unmarshal(...)"
  - "_ = err"
detect:
  golangci: [errcheck]
autofix: false
sources: [wieland]
status: stable
reviewed: "2026-11-13"
---

## Rule
Every error a call returns is checked, returned or handled; a deliberate drop is written as `_ = call()` only where failure truly cannot matter.

## Why
Go reports failure through return values, so an unchecked error is a failure nobody sees: a write that did not happen, a file that did not close, JSON that did not decode into the struct now full of zero values. Models drop errors from calls whose results they do not need, like `Close` and `Write`.

## Do
```go
import (
	"fmt"
	"os"
)

func writeFile(path string, data []byte) (err error) {
	f, err := os.Create(path)
	if err != nil {
		return fmt.Errorf("create %s: %w", path, err)
	}
	defer func() {
		if cerr := f.Close(); cerr != nil && err == nil {
			err = fmt.Errorf("close %s: %w", path, cerr)
		}
	}()
	if _, err := f.Write(data); err != nil {
		return fmt.Errorf("write %s: %w", path, err)
	}
	return nil
}
```

## Don't
```go
func writeFile(path string, data []byte) {
	f, _ := os.Create(path)
	defer f.Close()
	f.Write(data)
}
```

## Notes
The one exclusion in the compiled config is `(io.ReadCloser).Close`, the deferred close of a body you only read (see `res-close-http-body`). Closing a file you wrote can lose data and stays checked.

