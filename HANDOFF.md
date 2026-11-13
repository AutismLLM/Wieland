# Wieland — Handoff (state + open technical questions)

Updated 2026-07-13 (session 2). Read `CLAUDE.md` first (its "Current state" block is the
source of truth), then `PLAN.md` and `spec/research/microsoft-guidelines.md`. This file
tracks what's done and what's still undecided so a fresh chat starts cold.

---

## State snapshot

- **Standalone repo.** Lives at `/home/nicholas/Wieland` = private GitHub **`AutismLLM/Wieland`**.
  Extracted from the `weatherprediction` monorepo via `git subtree split` (8+ commits of real
  history preserved), LFS-stripped. **Work happens here now**; the monorepo copy is stale.
- **M0–M6 complete + post-M6 hardening.** Corpus = **311 rules**, validates clean, **compile-clean**
  (`wieland check-examples` → 0 suspect / 5 fragments / 275 clean across 280 Do/Good blocks).
- **Binaries:** `wieland` (validate | compile | gate | **check-examples**) and `wieland-mcp`
  (stdio MCP + `wieland-mcp http`). Corpus path via `WIELAND_RULES` (default `./rules`).
- **L0 = 10 always-on rules** (~4k tok, budget 5k): ownership/errors/types/async. See CLAUDE.md.
- **Enforcement = graduated strictness** (Nicholas's steer): foundation is strict (6 deny/gate
  rules → 11 clippy lints hard-block), the other 305 rules advise, hooks don't block. Validated
  end-to-end against throwaway crates; baked into CI as `ci/fixtures/gate-{bad,good}`.
- **Green gate:** `ci/check.sh` = fmt + clippy -D + test + validate + drift + dogfood-gate +
  gate-integration. Heavy `wieland check-examples` is a separate lane: `ci/check-examples.sh`.
- **Release:** tag `wieland-corpus-90377841a0af`, reproducible via `checkout + compile --all --check`.
  Re-cut the tag after any corpus change.
- **Not done:** no skill published, no server deployed. (Autonomy line — needs Nicholas's go.)

---

## Resolved since session 1

- **~~#1 Repo & release~~** — extracted to standalone private `AutismLLM/Wieland`, pushed. ✅
- **~~#2 ast_query honesty~~** — `enforceability: gate` now requires a real `detect.clippy` lint;
  the one ast_query-only offender (`serde-deny-unknown-fields`) downgraded `gate→check`, `tier 2→1`.
  Every gate rule is now genuinely clippy-enforceable. (Medium-term ast-grep backend still open — see #2' below.) ✅
- **~~#4 L0 under-populated~~** — filled to 10 always-on rules across the 4 highest-frequency areas. ✅
- **~~#5 rule code not compile-checked~~** — built `wieland check-examples`; corpus is compile-clean.
  It found + we fixed rustdoc hidden-line (`# code`) leaks in 3 rules on the first run. ✅

---

## Open technical questions (ranked; each has a recommended default)

### 2'. ast-grep backend (medium-term, from old #2)
The honesty gap is closed, but `ast_query` strings still don't execute — only clippy runs.
- **Q:** wire an ast-grep backend + a `detect.tool` generalization (also unlocks cargo-deny,
  cargo-semver-checks, miri from ENFORCEMENT.md)? Then rules like `serde-deny-unknown-fields`
  could return to `gate`.
- **Default:** defer until a rule genuinely needs AST-level gating; clippy covers the foundation today.

### 3. `clippy::panic` policy
Panic is left ungated (blanket deny is wrong; `err-expect-bugs-only` legitimises bug-panics).
- **Default:** leave ungated; rely on judge rules `err-result-over-panic` / `anti-panic-expected`. (Effectively decided — keep unless you disagree.)

### 6. Freshness mechanism designed but not built
Every rule has `reviewed:`; the schema promises "warn on rules not reviewed within N releases," but
`validate`/`compile` don't implement it. `msrv` mostly null.
- **Default:** implement a simple `reviewed` age warning (warn if older than ~6 months); defer msrv audit.

### 7. Hooks: PostToolUse can't truly block; inject-core unbuilt
`hooks/post-tool-use.sh` fires after the edit (exit-2 surfaces stderr, can't undo). The specced
`UserPromptSubmit` "inject-core" hook (`wieland inject-core --if-rust`) was never built.
- **Default:** add `wieland inject-core` (cheap proactive path — push L0 into context); leave
  PostToolUse as-is for the deterministic gate path. Fits the graduated-strictness model.

### 8. Deploy shape for MCP/HTTP (only when you decide to serve it)
`wieland-mcp http` serves plain HTTP; TLS assumed at a reverse proxy.
- **Default:** don't deploy yet — static files + local MCP cover every current consumer.

### 9. Corpus content: prelude conflict
`M-NO-PRELUDE` (Microsoft: never define a crate prelude) vs `proj-prelude-module` (rust-skills: do).
Both still in the corpus.
- **Default:** keep `proj-prelude-module` as a *judge* rule scoped to "internal workspace preludes,"
  add the Microsoft caveat to its Notes, mark the tension resolved.

---

## Cheapest next moves
1. #6 freshness warning (small, no deps) and #7 `wieland inject-core` (cheap, high-value proactive path).
2. #9 prelude reconciliation (one editorial edit).
3. #2' ast-grep backend only when a rule needs it.
4. Publish the compiled skill / deploy the server — needs Nicholas's go (outward action).
