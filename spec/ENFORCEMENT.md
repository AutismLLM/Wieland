# Wieland — Enforcement Model

> You don't prompt what you can gate. The base skill stays slim because most rules are caught by machines, not by nagging the model.

This is the design principle behind the tier system. It answers: *how much can hooks/build actually stop, and what must the prompt carry?*

---

## The four rings (cheapest / earliest first)

Rust's toolchain already enforces an enormous amount. Wieland orchestrates these; it never reimplements them.

| Ring | Tool | Catches | Cost | LLM can ship a violation? |
|------|------|---------|------|---------------------------|
| 1 | **rustc** | memory safety, borrow/lifetime, type errors, match exhaustiveness, object safety, unsafe boundaries | free, at compile | **No** — won't compile |
| 2 | **rustfmt** | all formatting | instant, autofix | shouldn't reach a human |
| 3 | **clippy** (~750 lints) | correctness, style, complexity, perf, and the `restriction` group | seconds | only until the gate runs |
| 4 | **cargo-deny · semver-checks · miri · ast-grep** | deps/licenses, API breakage, UB, custom structural patterns | varies | only until CI |

Ring 1 alone removes most of what other languages need lint rules for. That is why a Rust skill can be genuinely slim.

---

## The `enforceability` field — a rule's mechanical ceiling

Every rule declares what a machine *can* do about it. This is a **property**, not a choice:

| `enforceability` | Meaning | Detector | Max tier | Can hard-block? |
|------------------|---------|----------|----------|-----------------|
| `compiler` | rustc already enforces it | — (usually not a rule at all) | — | n/a |
| `gate` | reliable clippy/ast/config detector exists | required (`detect.clippy` or `detect.ast_query`) | 3 | **yes** |
| `check` | detectable but noisy or context-dependent | `detect.grep`/ast, advisory | 2 | no — surface only |
| `judge` | design taste, no mechanical detector | none | 0/1 | no — prompt/human only |

**Invariant:** `enforceability: gate` requires a populated `detect`. A rule cannot claim it blocks without naming what does the blocking.

### Rough split of the merged corpus (~265 rules)
- **~45–55% `gate`** — mostly clippy `restriction` lints: `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `undocumented_unsafe_blocks`, `await_holding_lock`, `missing_errors_doc`/`missing_panics_doc`/`missing_safety_doc`, `float_cmp`, `large_enum_variant`, `needless_collect`. Plus ast-grep for things like serde `deny_unknown_fields`, and Cargo-profile checks for `opt-lto-release` etc.
- **~20% `check`** — contextual perf/memory: `with_capacity` opportunity, smallvec/arena choice, cache-friendliness. A grep flags the pattern but can't know intent, so it advises, never blocks.
- **~25–30% `judge`** — the irreducible core: *should this be a newtype? does this abstraction earn its cost? is this the right error variant? by-feature or by-layer?* No gate can ever catch these. **This is the only bucket the prompt truly must carry.**

---

## tier = enforceability × frequency

`tier` is the *decision* of how hard to deliver a rule. It falls out of two inputs:

```
                 mechanically gateable?
                 gate / check          judge
frequent   ┌───────────────────────┬──────────────────────┐
           │ GATE + put in L0 too   │ L0 prompt            │
           │ (enforce AND skip the  │ (can't enforce,      │
           │  write→block→fix loop) │  must advise loudly) │
rare       ├───────────────────────┼──────────────────────┤
           │ GATE only              │ L1 retrieval         │
           │ (keep OUT of prompt —  │ (pulled on demand    │
           │  this is what keeps    │  when relevant)      │
           │  the skill slim)       │                      │
           └───────────────────────┴──────────────────────┘
```

The bottom-left cell is the whole trick: a rare, gateable rule is enforced perfectly by clippy and **never costs a prompt token**. Slimness comes from *trusting the gate*.

---

## Two different "says no" — don't conflate them

**Gate says no** (deterministic, *after* the model writes): the hook/clippy blocks the edit and returns `rule_id + why + fix_hint`. Works *even if the model never knew the rule*. The model reads the finding, fixes, moves on. → this is what lets the base skill be slim.

**Model says no** (proactive, *while* writing, citing rule X): only possible if the rule is in context. Reserve this for the frequent rules where the write→block→fix loop would otherwise burn tokens and latency.

Either way, the model never needs 265 rules in context to *comply*. It needs:
1. the **L0 core** it should internalize and cite proactively (the frequent + the judge-critical);
2. a **gate** to catch everything mechanical it didn't memorize;
3. an **MCP `explain(rule_id)`** lookup so, when the gate fires, the *why* and the idiomatic fix are one call away.

That is the "slim base, strong references" model: thin prompt, deep gate, on-demand corpus.

---

## What gates fundamentally cannot do (the prompt's job)

- Judge **intent**: "no unwrap" is gateable; "this is the right error type" is not.
- Judge **design**: newtype vs primitive, builder vs args, typestate, module boundaries, whether an abstraction pays for itself.
- Judge **taste**: is this name clear, is this doc *good* (not merely present), is this the semantically right collection.

Gates enforce the presence/absence of patterns. Everything above the pattern level is why a human — and a well-aimed prompt — still matters.

---

## Consequence for the merge
When merging (see `MERGE-PLAN.md` Phase 3), every rule gets an `enforceability` verdict and, if `gate`, a real `detect`. That verdict, crossed with frequency, assigns `tier`. The L0 core (`core.txt`) ends up small on purpose: the frequent-and-judge rules, plus the handful of gate rules common enough to be worth pre-empting. Everything else is gated or retrieved.

---

## TypeScript pack (M7)

Same four classes, same tiers, different tools. The rings for TypeScript 7:

| Ring | Tool | Catches | LLM can ship a violation? |
|---|---|---|---|
| 1 | **tsc** with the pack's required flags | type errors, missing returns, unchecked index access, optional-vs-undefined, non-erasable syntax | **No**: the gate blocks on any tsc error |
| 2 | **oxlint type-aware** (`oxlint` + `oxlint-tsgolint`, pinned) | floating and misused promises, non-exhaustive switches, `any`, `!`, `@ts-ignore`, unsafe casts | only until the gate runs |

| `enforceability` | TypeScript detector | Can hard-block? |
|---|---|---|
| `compiler` | tsc already rejects it; usually not a rule | n/a |
| `gate` | `detect.oxlint` (a typed lint rule) or `detect.tsconfig` (a compiler option that must resolve to `true`) | **yes** when `severity: deny` |
| `check` | `detect.oxlint`, run at `warn` | no: printed as advice |
| `judge` | none | no: prompt only |

Posture matches Rust: a strong foundation (7 oxlint rules and 10 compiler options block) and a relaxed periphery (5 oxlint rules advise). The compiled `dist/typescript/deny-set/oxlintrc.json` turns every oxlint category off, so only corpus rules run and every finding maps to a rule id; `wieland check-detectors` proves each name exists in the pinned tools.

---

## Go pack (M8)

| Ring | Tool | Catches | LLM can ship a violation? |
|---|---|---|---|
| 1 | **go type check** (inside golangci-lint) | type errors | **No**: reported as `[go typecheck]` and blocks |
| 2 | **golangci-lint v2.15.0** (pinned) with `linters.default: none` | unchecked errors, `%v` instead of `%w`, nil after err check, unclosed bodies and rows, context-less I/O, vet and staticcheck findings, unchecked type assertions, dead assignments | only until the gate runs |

`detect.golangci` names linters. golangci-lint has no per-linter warn level, so the gate runs every corpus linter once and decides per finding: a linter owned by a `gate`+`deny` rule blocks (12 linters), the rest advise (5). The compiled `dist/go/deny-set/golangci.yml` keeps every issue (`uniq-by-line: false`) and makes one exclusion: errcheck ignores `(io.ReadCloser).Close`, the deferred close of a body you only read.

