# Wieland

Rule corpus and gate for Rust, TypeScript and Go.

Each rule is one Markdown file with YAML frontmatter. The corpus compiles into a skill file, a short
always-on core and a linter configuration, and the same corpus drives a gate that runs after every
file edit and before every commit.

Every rule declares how far a tool can enforce it:

| Class | Detector | Effect |
|---|---|---|
| `gate` | a reliable linter rule or compiler option | blocks the edit or commit |
| `check` | a linter rule with false positives | printed as advice |
| `judge` | none | documented in the skill only |
| `compiler` | the compiler already rejects it | usually not a rule |

## Packs

| Pack | Rules | Detectors (pinned) | Project root |
|---|---|---|---|
| Rust | 311 | `cargo clippy` | `Cargo.toml` |
| TypeScript 7 | 20 | `tsc` 7.0.2 with required strict flags, `oxlint` 1.93.0 type-aware | `tsconfig.json` |
| Go 1.26 | 20 | `golangci-lint` v2.15.0 | `go.mod` |

The Rust pack merges [leonardomso/rust-skills](https://github.com/leonardomso/rust-skills) and
[microsoft/rust-guidelines](https://github.com/microsoft/rust-guidelines) (both MIT, sources in
`vendor/`). The TypeScript and Go packs are original. `skills/` holds two version notes,
`typescript-7-patch-notes` and `go-1-26-patch-notes`, covering what changed in those releases.

Status: all three packs compile, gate and pass CI. Version 0.x, corpus tag
`wieland-corpus-90377841a0af`.

## Install

Requires Rust stable; Node 22+ with npm for TypeScript; Go 1.26 for Go.

```sh
git clone https://github.com/Beemindapp/Wieland
cd Wieland
bash ci/install.sh   # binaries to ~/.cargo/bin, corpus to ~/.wieland/rules, pinned tools to ~/.wieland/tools
bash ci/check.sh     # full test suite
```

## Use

```sh
wieland gate --all       # language from the nearest Cargo.toml, tsconfig.json or go.mod
wieland gate --staged    # staged files, grouped per project root (monorepos work)
wieland gate --file src/app.ts
```

Output on a violation, exit code 1:

```
src/bad.ts:34: [async-no-floating-promise] Await, return or explicitly void every promise
    fix: Every promise you create is awaited, returned, or handed to a handler that deals with rejection; a deliberate fire-and-forget is written as `void promise.catch(handler)`.
gate: 1 violation(s)
```

A project can add its own checks in `wieland.yaml` (a command, plus the file extensions and paths it
covers); the hook runs them on matching edits and a failure blocks as `[project:<name>]`.

Only findings in the edited file block an edit. Project-level findings (a missing compiler option)
block `gate --all`, the pre-commit hook and CI.

## Editor and agent hooks

`wieland hook` reads the hook payload on stdin, gates the edited file and answers in the caller's
format. Templates and payload details are in [`hooks/README.md`](hooks/README.md).

| Caller | Event | Reply |
|---|---|---|
| Claude Code | `PostToolUse` | stderr, exit 2 |
| Codex CLI | `PostToolUse` (`apply_patch`) | stderr, exit 2 |
| Cursor | `postToolUse` | `additional_context` JSON |
| Gemini CLI | `AfterTool` | `additionalContext` JSON |
| Pi | `tool_result` extension | appended to the tool result |
| git | `pre-commit` | blocks the commit |

## Commands

```
wieland validate [dir]                     schema-check a pack (rules, rules/typescript, rules/go)
wieland compile --all [--check]            build dist/ for every pack; --check fails on drift
wieland gate [--all|--staged|--changed|--file <paths>]
wieland hook [--agent claude|codex|cursor|gemini|pi] [--path <file>]
wieland check-detectors                    every detector in the corpus exists in the pinned tools
wieland check-examples [--pack rust|typescript|go]
wieland install                            copy the corpus to ~/.wieland/rules
```

`wieland-mcp` serves the Rust corpus over MCP (stdio) and HTTP.

## CI guarantees

- Every `gate` rule names a detector that exists: oxlint rejects an unknown rule in the compiled
  config, `tsc` rejects an unknown option (TS5023), golangci-lint rejects an unknown linter.
- The compiled oxlint config turns every built-in category off, so only corpus rules run.
- Every `Do` example compiles: Rust via `cargo check`, TypeScript via `tsc` under the pack's flags,
  Go via `go vet`.
- `compile --all --check` fails when `dist/` is stale; the Rust artifacts are byte-stable.
- Bad and good fixtures per language prove the gate and the hook block and pass as expected.

## Writing a rule

Frontmatter follows `spec/schema/rule.schema.yaml`; the body has `## Rule`, `## Why`, `## Do` and
`## Don't`. See [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Layout

```
rules/            Rust pack; rules/typescript/ and rules/go/
spec/             schema, taxonomies, enforcement and hook specs
crates/           wieland-core (model, validate, compile), wieland-cli, wieland-mcp
tools/typescript/ exact pins of tsc, oxlint, oxlint-tsgolint
skills/           version notes for TypeScript 7 and Go 1.26
hooks/            hook templates
ci/               check.sh, check-examples.sh, install.sh, publish-public.sh, fixtures
```

## Licence

MIT.
