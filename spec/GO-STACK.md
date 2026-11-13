# GO-STACK — the Go pack for Wieland (M8 plan)

Status: plan, 2026-11-13. Same engine, same words as the Rust and TypeScript packs.

## 1. The idea

| Rust | TypeScript | Go pack |
|---|---|---|
| `rules/*.md` | `rules/typescript/*.md` | `rules/go/*.md`, ```` ```go ```` blocks |
| rustc | tsc + required flags | `go build` / type check inside golangci-lint |
| `detect.clippy` | `detect.oxlint`, `detect.tsconfig` | `detect.golangci` (linter names) |
| `deny-set/lints.toml` | `oxlintrc.json` | `dist/go/deny-set/golangci.yml` (`linters.default: none`) |
| `Cargo.toml` | `tsconfig.json` | `go.mod` (nearest manifest decides) |
| skill `wieland-rust` | `wieland-typescript` | `wieland-go` + `go-1-26-patch-notes` |
| `rust_edition` | `ts_version` | `go_version: "1.26"` |

The schema fields (`language: go`, `go_version`, `detect.golangci`) and `categories-go.yaml`
landed with M7, so M8 needs no schema change.

## 2. Measured facts

| Fact | Value |
|---|---|
| Go on this box | 1.26.5 (1.27.4 is current upstream; the pack targets 1.26) |
| Detector | golangci-lint v2.15.0 (2026-10-16, built with go1.26.5), `go install` into `~/.wieland/tools/go/bin` |
| Config shape | `version: "2"`, `linters.default: none`, `linters.enable: [...]` runs only the listed linters |
| Output | `run --output.json.path=stdout`: `Issues[]` with `FromLinter`, `Text`, `Pos.Filename`, `Pos.Line` |
| Type errors | reported by golangci-lint itself (`typecheck`), so the gate blocks on them like on tsc errors |

## 3. Decisions

- **G1 one tool.** golangci-lint carries govet, staticcheck, errcheck and the rest; no separate `go vet` or
  `staticcheck` binary. Pinned by version in `ci/install.sh`.
- **G2 config file, not flags.** The gate writes the compiled `golangci.yml` and passes `-c`; an unknown
  linter name in the file fails, which is the detector existence check.
- **G3 level.** golangci-lint has no per-linter warn level, so the gate runs it once and decides per finding:
  a linter that belongs to a `gate`+`deny` rule blocks, the rest print as advice.
- **G4 examples.** Each ```` ```go ```` Do block is a complete file body (imports and declarations, no
  `package` line); `check-examples` puts each in its own package of one scratch module and runs `go vet`.

## 4. The 20-rule spike

| # | id | tier | enforceability | detector |
|---|---|---|---|---|
| 1 | `err-check-every-error` | 0 | gate, deny | errcheck |
| 2 | `err-wrap-with-w` | 3 | gate, deny | errorlint |
| 3 | `err-no-nil-after-check` | 3 | gate, deny | nilerr |
| 4 | `res-close-http-body` | 3 | gate, deny | bodyclose |
| 5 | `res-close-sql-rows` | 3 | gate, deny | sqlclosecheck, rowserrcheck |
| 6 | `ctx-request-with-context` | 3 | gate, deny | noctx |
| 7 | `conc-vet-clean` | 3 | gate, deny | govet (copylocks, lostcancel, loopclosure, printf …) |
| 8 | `anti-staticcheck-clean` | 3 | gate, deny | staticcheck |
| 9 | `type-checked-type-assertion` | 3 | gate, deny | forcetypeassert |
| 10 | `anti-no-dead-assignments` | 3 | gate, deny | ineffassign, unused |
| 11 | `type-exhaustive-switch` | 2 | check | exhaustive |
| 12 | `err-sentinel-naming` | 2 | check | errname |
| 13 | `err-no-nil-nil` | 2 | check | nilnil |
| 14 | `anti-no-unused-params` | 2 | check | unparam |
| 15 | `ctx-no-stored-context` | 2 | check | containedctx |
| 16 | `conc-goroutine-has-an-owner` | 0 | judge | — |
| 17 | `type-accept-interfaces-return-structs` | 0 | judge | — |
| 18 | `err-handle-once` | 0 | judge | — |
| 19 | `ctx-first-parameter` | 1 | judge | — |
| 20 | `design-earn-the-abstraction` | 1 | judge | — |

## 5. Tasks — M8

1. Record M8 (ROADMAP, TASKS, PLAN).
2. Pin golangci-lint v2.15.0 in `ci/install.sh` (`GOBIN=~/.wieland/tools/go/bin go install …`).
3. Author the 20 rules; `validate rules/go` → 20 rules OK.
4. Compile: `dist/go/` (core, all, `wieland-go` skill, `golangci.yml`, rulemap); drift gate covers it.
5. `check-detectors` for Go: compiled config on an empty module; unknown linter fails; CI self-test.
6. Gate Go path + `ci/fixtures/gate-go-{bad,good}`; hook on `.go` walking to `go.mod`.
7. `check-examples --pack go`; seeded broken block reported.
8. Ship `wieland-go`; close M8 (docs, both CI lanes green).

**M8 exit:** a bad Go edit in a module is blocked by `wieland hook` with a rule id and a fix line; a good
one passes; Rust and TypeScript packs unchanged.
