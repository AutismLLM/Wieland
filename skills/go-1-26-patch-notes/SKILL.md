---
name: go-1-26-patch-notes
description: Reference for what changed in Go 1.26 (released Feb 2026) versus earlier Go versions covered in training data. Load this before writing, reviewing, or reasoning about Go code on 1.26+ toolchains — covers new language features, GC/runtime behavior changes, GODEBUG/rand-parameter behavior changes in crypto/*, and stdlib additions that are easy to miss or get wrong from stale knowledge (e.g. assuming `rand` params still matter, or `go fix`/`go/types` behaving like pre-1.26).
---

# Go 1.26 — what's actually new

Go 1.26 shipped Feb 2026, six months after 1.25. Full maintains Go 1 compat. This is the delta from what a pre-1.26-trained model assumes. Full text: https://go.dev/doc/go1.26 (fetch it live via context7-docs or WebFetch if you need exact wording/edge cases — this is a working summary, not a substitute for the source on anything load-bearing).

## Language changes (small but easy to miss)

- **`new(expr)`**: `new` now accepts an *expression*, not just a type, and returns a pointer to a variable initialized with that value. `ptr := new(computeAge(born))` instead of the old two-step `x := computeAge(born); ptr := &x`. Useful for optional pointer fields in struct literals (`Age: new(yearsSince(born))`).
- **Self-referential generics**: a generic type can now name itself in its own type parameter list. `type Adder[A Adder[A]] interface { Add(A) A }` now compiles — was previously rejected. If you see this pattern in Go 1.26+ code, it's not a typo.

## Runtime — do not assume old defaults

- **Green Tea GC is ON by default.** It was opt-in experimental in 1.25; now it's the default collector (scans contiguous spans instead of scattered objects, 10-40% less GC overhead, extra ~10% on Ice Lake/Zen4+ via vector instructions). If someone needs the old collector for a regression/bisect: `GOEXPERIMENT=nogreenteagc` at build time. Slated for removal (i.e. mandatory) in 1.27 — don't build tooling that assumes the opt-out flag survives.
- **Heap base address is randomized by default on 64-bit.** Breaks any code that assumed stable/predictable heap addresses (mainly a cgo hazard). Opt out with `GOEXPERIMENT=norandomizedheapbase64` for debugging, not as a permanent fix.
- **cgo calls are ~30% cheaper** at baseline — don't assume old cgo-avoidance perf workarounds are still worth their complexity cost; re-measure.
- Compiler now stack-allocates slice backing stores in more cases. If a slice you expected to escape to heap doesn't show up in a heap profile anymore, this is likely why (`-gcflags=all=-d=variablemakehash=n` disables it for bisecting).

## crypto/* — `rand` parameters are now widely ignored

This is the single biggest gotcha for existing code and for writing new code that looks idiomatic but is stale: several long-standing crypto APIs that took a `rand io.Reader` parameter **now ignore it** and always use secure crypto randomness internally. Passing a custom/deterministic `rand.Reader` no longer does anything in these:

- `crypto/dsa.GenerateKey(rand, params)`
- `crypto/ecdh.Curve.GenerateKey(rand)`
- `crypto/ecdsa.GenerateKey(rand)`, `SignASN1(rand, ...)`, `Sign(rand, ...)`, `PrivateKey.Sign(rand, ...)`
- `crypto/ed25519.GenerateKey(rand)` — only when `rand` is nil does this kick in
- `crypto/rand.Prime(rand, ...)`
- `crypto/rsa.GenerateKey(rand, ...)`, `GenerateMultiPrimeKey(rand, ...)`, `EncryptPKCS1v15(rand, ...)`

If you're writing **tests** that need deterministic crypto output, the `rand` parameter is not the mechanism anymore — use the new `testing/cryptotest.SetGlobalRandom(func)` package instead. `GODEBUG=cryptocustomrand=1` restores the old rand-respecting behavior as an escape hatch, but treat that as legacy-compat, not the way to write new tests.

Also: `crypto/ecdsa.PublicKey.X/Y` and `PrivateKey.D` (the `big.Int` fields) are **deprecated** — new code touching ECDSA keys directly should avoid depending on those fields staying around. `crypto/rsa` PKCS#1 v1.5 encryption functions (`EncryptPKCS1v15`, `DecryptPKCS1v15`, `DecryptPKCS1v15SessionKey`) are deprecated too.

## crypto/tls — defaults changed

- Post-quantum hybrid key exchanges (`SecP256r1MLKEM768`, `SecP384r1MLKEM1024`) are **enabled by default** now. `Config.CurvePreferences` or `GODEBUG=tlssecpmlkem=0` to opt out.
- A batch of GODEBUG settings tied to old/weak TLS behavior (`tlsunsafeekm`, `tlsrsakex`, `tls10server`, `tls3des`, `x509keypairleaf`) are on notice for removal in **1.27** — don't build anything new that depends on them surviving.

## `net/url` — stricter host parsing

`url.Parse` now **rejects** malformed URLs with colons in the host, e.g. `http://::1/` or `http://localhost:80:80/`. Properly bracketed IPv6 (`http://[::1]/`) is still fine. `GODEBUG=urlstrictcolons=0` restores the old lenient behavior if you hit this in a migration.

## Tooling

- **`go fix` is a different tool now.** It was rewritten on the Go analysis framework (same engine as `go vet`) and is now the home of "modernizers" — automated fixers that rewrite code to current idioms, plus a source-level inliner driven by `//go:fix inline` directives. If you know `go fix` as the old, mostly-inert historical-fixer tool, that's gone — all the old fixers were removed. Worth running on legacy repos being touched in 1.26+.
- **`go mod init` picks a lower `go` directive than you'd expect.** New `go.mod` files default to an older Go version than the toolchain running them (roughly N-1 or N-2 versions back), to encourage broader compatibility. Don't be surprised by a `go.mod` saying `go 1.24.0` when built with a 1.26 toolchain — that's intentional, not a mistake to "fix" by bumping it. Override explicitly with `go get go@version` if a newer directive is actually needed.
- **`cmd/doc` / `go tool doc` are gone.** Use `go doc` (same flags/behavior) — if you see either of the old names in a script or Makefile, it needs updating.
- `pprof -http` now opens to the **flame graph** view by default (was call graph). Call graph is still there under View → Graph / `/ui/graph`.

## Stdlib additions worth knowing about

- `crypto/hpke` — Hybrid Public Key Encryption (RFC 9180), new.
- `errors.AsType[T](err)` — generic, type-safe alternative to `errors.As` (no more passing a `**T` target).
- `reflect.Type`/`reflect.Value` gained iterator methods: `Fields()`, `Methods()`, and on `Type` also `Ins()`/`Outs()` — idiomatic to use with `range` now instead of the old indexed `NumField()`/`Field(i)` loops.
- `log/slog.NewMultiHandler(handlers...)` — fan out to multiple handlers without hand-rolling one.
- `bytes.Buffer.Peek(n)` — read ahead without consuming.
- `net.Dialer` gained `DialIP`/`DialTCP`/`DialUDP`/`DialUnix` context-aware methods.
- `io.ReadAll` is ~2x faster / ~50% less memory — no action needed, just don't "optimize" it away thinking it's still the old slow version.
- `net/http/httputil.ReverseProxy.Director` is deprecated in favor of `Rewrite` — flag it in review if you see new code still using `Director`.
- `testing.T/B/F.ArtifactDir()` + `go test -artifacts` — proper place for tests to write output artifacts now, instead of ad hoc `t.TempDir()` abuse for anything meant to be inspected after the run.

## Experimental (opt-in via `GOEXPERIMENT=`, not default)

- `simd/archsimd` (`GOEXPERIMENT=simd`, amd64 only) — SIMD vector types/ops, API explicitly unstable.
- `runtime/secret` (`GOEXPERIMENT=runtimesecret`, linux amd64/arm64 only) — secure erasure of secret temporaries for crypto code.
- `goroutineleak` pprof profile (`GOEXPERIMENT=goroutineleakprofile`) — reachability-based leaked-goroutine detection, exposed at `/debug/pprof/goroutineleak`. Expected default-on in 1.27.

## Platform notes

- macOS: 1.26 is the **last** release supporting macOS 12 Monterey (1.27 requires macOS 13+).
- `windows/arm` (32-bit) port removed.
- `linux/riscv64` now supports the race detector.
- PowerPC big-endian (`linux/ppc64`): 1.26 is last release on ELFv1 ABI; 1.27 switches to ELFv2.
- Bootstrap now requires Go 1.24.6+.
