# Wieland — Execution Plan

The narrative behind `ROADMAP.md`: *why* this order, what was ruled, and the running log of decisions (schema freeze, dep choices, spike-rule selection). Update the log as you go — it is the project's memory.

---

## The shape (settled)

Static-compile-first. One canonical corpus (`rules/*.md`) is the source of truth; a deterministic, LLM-free compiler projects it into every harness artifact; an MCP server and HTTPS endpoint are secondary live layers over the same corpus. Full rationale in `spec/ARCHITECTURE.md`; enforcement model in `spec/ENFORCEMENT.md`.

### The Fosite ruling (tech-lead review, folded in)
Verdict: **approve Option A (static-compile-first).** Reasoning: the hard constraints ("survive with no tooling", "file-only harnesses can't be second-class") mandate static-primary; a server-first design demotes file readers, violating them. Option A already gives MCP agents first-class access to the same corpus, so the only advantage of server-first (freshness) collapses to recompile latency — a CI problem, not an architecture one. Concept correction accepted: *"instantly live" has near-zero value for a rules corpus* — rules change on human-review timescales, so corpus updates are treated as **releases**, not live pushes.

Corrections that reshaped the plan:
1. **Drift gate is the one real failure mode** → build `compile --all --check` before any second emitter (M2, first task).
2. **Build order was inverted** → spike 20 rules and freeze the schema (M1) before building 6 emitters; only 2 targets at M2; Cursor/Copilot/Aider/Codex are SPECULATIVE until a consumer needs them.
3. **L0 will bloat** → compiler enforces a ~2k-token budget, build fails over it.
4. **clippy needs a crate, not a snippet** → `review` scopes clippy honestly; snippets get grep/ast only.
5. **enforceability could drift into prompt-stuffing** → schema invariant: tier 2/3 (and `gate`) require a non-empty `detect{}`. Already enforced in `validate_corpus`.
6. **advice ages** → `reviewed` date field + stale-rule warnings.

---

## Why this milestone order

The expensive mistake is writing emitters and tooling against a schema that hasn't met real rules yet — you rewrite them all when the first 20 rules expose what the schema got wrong. So: **prove the corpus small, freeze the schema, gate against drift, then scale.** M0 gives a compiling spine. M1 is the pressure test. M2 proves the projection on the smallest useful set with the drift gate first. M3 scales the corpus with fan-out + adversarial QC. M4–M6 add the enforcement and serving layers that only make sense once the corpus is real.

---

## Decision log

- **2026-07-12 — architecture:** static-compile-first confirmed (Fosite Option A). MCP/HTTPS secondary.
- **2026-07-12 — language:** tooling is Rust (dogfooding, single-binary, native toolchain interop). Corpus is language-agnostic Markdown.
- **2026-07-12 — M0 workspace:** 2 crates only (`wieland-core`, `wieland-cli`), zero external deps at M0 by choice; deps added deliberately at M1. Boring path per constraint.
- **2026-07-12 — invariant landed:** `validate_corpus` enforces tier-2/3 ⇒ non-empty `detect{}` (Fosite #5) in real code.
- **2026-07-12 — yaml dep:** `serde` (derive) + `serde_yaml_ng` for frontmatter. Why: `serde_yaml` is archived/unmaintained; `serde_yaml_ng` is the maintained drop-in fork with the same API. serde gives strict `deny_unknown_fields` deserialization straight into the enum model, which is exactly the `additionalProperties:false` the schema demands — no hand-rolled YAML footguns. Body split (`---` fence, `## ` sections) is done by hand; no markdown crate needed.
- **2026-07-12 — spike-20 selection:** table below (20 rules; tiers 0–3, 7 gate / 6 check / 7 judge, both sources).
- **2026-07-13 — M3 full merge:** corpus is **311 rules**. 265 rust-skills rules transformed to canonical form (spike-20 + fan-out of the remaining ~247, one agent per taxonomy category + adversarial slop pass), all schema-valid. Microsoft rust-guidelines (89 `M-*`) folded in: **44 authored** as new wieland rules, **42 covered** by existing rust-skills rules, **3 long-form** routed to `spec/research/microsoft-guidelines.md` (full coverage map there — none dropped silently). No invented clippy lints (every `detect.clippy` entry is a real lint). No new tier-0 rules, so L0 `core.txt` stays ~844 tokens. `wieland validate rules` → "311 rules OK"; drift gate clean (314 artifacts).
- **2026-07-12 — schema freeze:** `spec/schema/rule.schema.yaml` **FROZEN** as of the M1 corpus-spike commit. The 20 spike rules exposed no missing or awkward field — every one validated clean (`wieland validate rules` → "20 rules OK") with the schema unchanged. No schema edits after this without a noted reason here. Runtime parsing tightened two invariants beyond the M0 stub: `enforceability=gate` now requires `detect.clippy` OR `detect.ast_query` specifically (not any non-empty detect), and `id` prefix must equal the category's taxonomy `prefix`.

- **2026-11-13 — schema unfreeze (M7, TypeScript pack):** four optional fields added: `language` (`rust` | `typescript` | `go`, default `rust`), `ts_version` (required for TypeScript), `detect.oxlint`, `detect.tsconfig`. Reason: a second language needs its own detectors and version stamp. The 311 Rust files need no edit; their compiled artifacts stay byte-identical. Decisions:
  - D1 a pack is a directory: `rules/` stays the Rust pack, `rules/typescript/` is a separate corpus (`Corpus::load` is not recursive, so the Rust sha and release tag are untouched). Ids unique per pack.
  - D2 the four fields above.
  - D3 gate honesty per language: Rust `gate` needs `detect.clippy`; TypeScript `gate` needs `detect.oxlint` or `detect.tsconfig`; a rule may not carry another language's detector.
  - D4 own taxonomy per pack (`categories-typescript.yaml`, nine categories); the 26 Rust categories are not ported.
  - D5 detector = oxlint type-aware, pinned exactly (`typescript 7.0.2`, `oxlint 1.93.0`, `oxlint-tsgolint 7.0.2006`) in `tools/typescript/`, installed to `~/.wieland/tools/typescript`. Consumer repos get zero new dependencies. Tradeoff vs constraint 2: the corpus still needs no tooling and no Rust crate is added; oxlint plays the role `cargo clippy` plays for Rust.
  - D6 the gate reads a compiled config file, never `-D` flags: a misspelled CLI flag is silently ignored, a misspelled config entry fails. All seven oxlint categories are `off`, so only corpus rules run.
  - D7 `tsc` is the authority on types; oxlint only lints.
  - D8 project kind = nearest manifest (`Cargo.toml` → Rust, `tsconfig.json` → TypeScript); required flags read via `tsc --showConfig`.
  - D9 small corpus: a 20-rule spike, then grow from real findings.
  - D10 the `typescript-7-patch-notes` skill ships in this repo with the pack.
  - The Go fields (`go_version`, `detect.golangci`, `categories-go.yaml`) land in the same unfreeze so the schema opens once for both new packs (M8 uses them).

---

## Spike-20 selection (fill at M1)

| # | rule id | category | tier | enforceability | source | notes |
|---|---------|----------|------|----------------|--------|-------|
| 1 | own-borrow-over-clone | ownership | 0 | judge | rust-skills | frequent design call; L0 core |
| 2 | err-result-over-panic | errors | 0 | judge | rust-skills | frequent design call; L0 core |
| 3 | anti-unwrap-abuse | anti-patterns | 3 | gate | rust-skills | clippy::unwrap_used, expect_used |
| 4 | err-no-unwrap-prod | errors | 3 | gate | rust-skills | clippy::unwrap_used |
| 5 | unsafe-safety-comment | unsafe | 3 | gate | rust-skills | clippy::undocumented_unsafe_blocks |
| 6 | async-no-lock-await | async | 3 | gate | rust-skills | clippy::await_holding_lock |
| 7 | num-float-compare | numerics | 2 | gate | rust-skills | clippy::float_cmp |
| 8 | pat-let-else | patterns | 2 | gate | rust-skills | clippy::manual_let_else |
| 9 | serde-deny-unknown-fields | serde | 2 | gate | rust-skills | ast_query (no clippy lint) |
| 10 | mem-box-large-variant | memory | 2 | check | rust-skills | clippy::large_enum_variant (advisory) |
| 11 | err-doc-errors | errors | 2 | check | rust-skills | clippy::missing_errors_doc (advisory) |
| 12 | mem-with-capacity | memory | 2 | check | rust-skills | grep heuristic (already EXAMPLE.rule.md) |
| 13 | perf-entry-api | performance | 2 | check | rust-skills | grep heuristic |
| 14 | own-slice-over-vec | ownership | 1 | check | rust-skills | grep heuristic; on-demand |
| 15 | err-thiserror-lib | errors | 1 | judge | rust-skills | error-type design choice |
| 16 | api-parse-dont-validate | api-design | 1 | judge | rust-skills | design choice |
| 17 | name-is-has-bool | naming | 1 | check | rust-skills | grep heuristic |
| 18 | test-descriptive-names | testing | 1 | judge | rust-skills | taste |
| 19 | M-UNSAFE | unsafe | 1 | judge | rust-guidelines | authored as `unsafe-needs-reason` (id must be kebab) |
| 20 | M-DESIGN-FOR-AI | api-design | 1 | judge | rust-guidelines | authored as `api-design-for-ai` |

Coverage: tiers 0/1/2/3 all present (2×T0, 8×T1, 8×T2, 4×T3). Enforceability: 7 `gate` (6 with a real clippy lint — rows 3–8 — plus row 9 via `ast_query`), 6 `check`, 7 `judge`. Both `vendor/` sources represented (18 rust-skills, 2 rust-guidelines). Note: the two Microsoft guidelines carry `M-UPPERCASE` anchors in source; the schema's `id` pattern is kebab-case, so they are authored under kebab ids (`unsafe-needs-reason`, `api-design-for-ai`) with `sources: [microsoft/rust-guidelines#M-UNSAFE]` etc.

---

## Note — language-agnostic engine (speculative, not scheduled)

The engine (schema → parse → validate → compile/gate) has nothing Rust-specific in it; only the *rule content* and the *detectors* are. A `wieland` corpus for Go (detectors: `go vet`, `staticcheck`, `golangci-lint`) or TypeScript (`eslint`, `tsc`) would drop into the same compiler and tiers unchanged. Now scheduled as M7 (TypeScript) and M8 (Go). Noted originally so we don't accidentally hard-wire Rust assumptions into `wieland-core`. Keep the core's rule model detector-agnostic (it already is: `detect.clippy` is just a `Vec<String>` of tool-lint names; a future `detect.tool` generalization is the only change needed).
