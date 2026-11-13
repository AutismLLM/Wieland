# Wieland — Agent Operating Manual

You are working on **Wieland**: a framework that keeps one canonical corpus of coding rules per language (Rust, TypeScript, Go next) and forges it into every coding-agent format (Claude skill, Cursor rules, AGENTS.md, Copilot, MCP server, HTTPS endpoint, git hooks) plus a gate that blocks violations. Tagline: *rule corpus and gate for Rust, TypeScript and Go.*

This file is your contract. Read it fully before touching anything. Then read `spec/ARCHITECTURE.md` and `spec/ENFORCEMENT.md`. Then work the plan.

---

## Prime directive

Build Wieland by walking `ROADMAP.md` milestone by milestone, executing `TASKS.md` top to bottom. Do not skip ahead. Each task has an explicit **Done when** — a task is finished only when that condition is verified by running something, not by asserting it.

When the human opens a chat here and says "build the app," your job is: find the first unchecked task in `TASKS.md`, do it, verify it, check the box, and continue until the current milestone's exit criteria are met — then stop and report.

---

## What Wieland is (one screen)

- **Corpus** (`rules/*.md`): one Markdown file per rule, YAML frontmatter validated by `spec/schema/rule.schema.yaml`. This is the single source of truth. It must stay openable and understandable with zero tooling.
- **Compiler** (`wieland compile <target>`): a deterministic, LLM-free projection of the corpus into each harness's native artifact. Same corpus in → same bytes out.
- **Live layers** (secondary): an MCP server (`wieland-forge`) and a plain HTTPS GET endpoint serving the same corpus; the MCP `review` tool can run real clippy.
- **Enforcement**: hooks + a compiled `clippy.toml` deny-set that block violations.

The architecture was ruled on and confirmed **static-compile-first** (Option A). MCP/HTTPS are secondary, never primary. Do not invert this.

---

## KNOWN constraints (these bind every task — restate the relevant ones before you implement)

1. **Static-compile-first.** Committed static files are the primary delivery. File-only harnesses are first-class, never a "fallback."
2. **Zero-dep center.** The corpus survives with no tooling. Tooling deps are added deliberately, one at a time, each justified against a task. `wieland-core` stays as lean as possible.
3. **The app is Rust.** Workspace in `crates/`. Idiomatic, `cargo clippy`-clean, `cargo fmt`-clean. Dogfood: Wieland's own crates must pass Wieland's own gate once the gate exists.
4. **Don't prompt what you can gate.** ~50% of rules map to a clippy/ast detector (hard-gate), ~20% are noisy heuristics (advise only), ~25–30% are design judgement (prompt only). See `spec/ENFORCEMENT.md`.
5. **Tier 2/3 requires a detector.** A rule cannot claim it is machine-checked or enforced without a non-empty `detect{}`. `validate_corpus` enforces this today.
6. **L0 core has a hard token budget (~5k).** The compiler must fail if `core.txt` exceeds it. No silent bloat.
7. **Drift gate before breadth.** Build `wieland compile --all --check` (recompile, fail on diff) BEFORE adding a second export target. Staleness is the one real failure mode.
8. **Build for KNOWN only.** Cursor / Copilot / Aider / Codex emitters are SPECULATIVE until a real consumer needs one. Note them; do not build them early.
9. **Age honestly.** Rules carry `rust_edition`, optional `msrv`, and `reviewed` date. Rust ships every 6 weeks with no LTS; stale rules get flagged.

If a constraint conflicts with simplicity or another constraint, state the conflict in one line and the chosen tradeoff. Never resolve it silently.

---

## How to work a task

1. Read the task and its **Done when**.
2. Restate the KNOWN constraints that touch it (constraint 2/3/5 almost always apply to code).
3. Implement the smallest change that satisfies it. Prefer the boring path: fewest deps, fewest new concepts.
4. Verify by executing: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, or running `./target/debug/wieland ...` and observing output. Paste what you observed.
5. Check the box in `TASKS.md` and add a one-line note of what you verified.
6. Commit (see identity rule below). One task ≈ one commit unless tasks are trivially linked.

Definition of done for a milestone = every task checked AND the milestone's exit criteria in `ROADMAP.md` demonstrably pass.

---

## House rules (hard)

- **No code comments.** Rust source carries zero comments. Names and structure do the explaining. (Docs live in `spec/` and Markdown, not in `.rs` files.)
- **Commit identity:** author every commit as `AutismLLM <126010501+AutismLLM@users.noreply.github.com>`. Never as Claude. No Co-Authored-By trailer.
- **No slop.** Rule bodies and docs: no filler, no hedging, no invented clippy lints. Every `Do`/`Don't` must be real, compiling Rust. Every `detect.clippy` must name a lint that exists.
- **Verify, don't assert.** "It should work" is not done. Run it.
- **Ask before outward or irreversible actions** (publishing a skill, deploying the server, pushing). Local build/test/commit is fine to do autonomously.

---

## Map

```
wieland/
├── CLAUDE.md        you are here — the contract
├── README.md        human entry point
├── ROADMAP.md       milestones M0–M6 with exit criteria
├── TASKS.md         the granular, checkbox task list — your worklist
├── PLAN.md          sequencing rationale + the Fosite ruling that shaped it
├── rules/           THE CORPUS (empty until M1)
├── spec/            the design contract — READ FIRST
│   ├── ARCHITECTURE.md   the one idea, tiers, data flow
│   ├── ENFORCEMENT.md    gate vs advise vs judge; why the prompt stays slim
│   ├── MERGE-PLAN.md     how vendor/ becomes rules/ (Fosite-corrected build order)
│   ├── schema/          rule.schema.yaml + EXAMPLE.rule.md
│   ├── taxonomy/        categories.yaml (the 26-category namespace)
│   ├── export/          COMPILE.md (compiler + target specs)
│   ├── mcp/             SPEC.md (wieland-forge)
│   ├── hooks/           SPEC.md (enforcement)
│   ├── research/        SPEC.md (HTTPS endpoint)
│   └── sources/         PROVENANCE.md (license + id mapping)
├── vendor/          raw merge inputs: rust-skills/ (265 rules), rust-guidelines/ (MS)
└── crates/          the Rust workspace (the app)
    ├── wieland-core/    lib: rule model, corpus load, validate
    └── wieland-cli/     bin `wieland`: validate | compile | gate | check-examples
```

Canonical paths: corpus root = `rules/`; compiled output = `dist/` (gitignored until we decide to commit exports); schema = `spec/schema/rule.schema.yaml`.

---

## Current state (update this line as milestones close)

**M6 complete — all milestones done.** `wieland-mcp` now serves a second door: a `tiny_http` HTTP layer (`src/http.rs`) sharing the same `Arc<Corpus>`. `wieland-mcp http [addr]` (default 127.0.0.1:8787) or set `WIELAND_HTTP` to run it beside the stdio MCP server in one process. Routes (GET): `/version` (corpus_sha + `corpus_tag`), `/categories.json`, `/bundle/core.txt` (live-rendered L0), `/rules/{id}` (raw md, 404 on miss). Plain HTTP; TLS at a reverse proxy. Release discipline: `corpus_tag = wieland-corpus-<sha12>`, git-tagged, reproducible via `checkout + compile --all --check`. 4 route tests (30 total). Speculative emitters stay unbuilt (constraint 8).

**Project state: M0–M6 all complete.** Corpus = 311 rules (rust-skills + Microsoft guidelines). One binary `wieland` (validate | compile | gate | check-examples) + one binary `wieland-mcp` (stdio MCP + HTTP). `ci/check.sh` (fmt+clippy+test+validate+drift+dogfood-gate+gate-integration) is the green gate.

**Post-M6 (2026-07-13):** Extracted to the standalone private repo **`AutismLLM/Wieland`** (this repo) via `git subtree split`; work happens here now, not the monorepo. Changes since M6:
- **L0 filled to 10 always-on rules** (~4k tok): ownership (`own-borrow-over-clone`, `own-slice-over-vec`), errors (`err-result-over-panic`, `err-no-unwrap-prod`, `err-question-mark`), types (`type-newtype-validated`, `type-enum-states`, `type-option-nullable`), async (`async-no-lock-await`, `async-clone-before-await`). Budget raised 2k→5k (constraint 6). `err-no-unwrap-prod` is both L0 and in the deny-set.
- **Enforcement posture = graduated strictness** (strong foundation, relaxed periphery): 6 deny/gate rules → 11 clippy lints hard-block; the other 305 advise; hooks don't block. Validated end-to-end against throwaway crates and baked into CI as `ci/fixtures/gate-{bad,good}`.
- **Honesty:** `enforceability: gate` now requires a real `detect.clippy` lint (the gate never runs `ast_query`); `serde-deny-unknown-fields` downgraded `gate→check`, `tier 2→1`. No rule claims enforcement it can't deliver.
- **`wieland check-examples`** compile-checks every Do/Good ```rust block (heavy; in `ci/check-examples.sh`, not the fast path). Corpus is compile-clean: 275 clean / 5 fragments / 0 suspect across 280 blocks (it found + we fixed rustdoc hidden-line leaks in 3 rules).
- Release tag `wieland-corpus-90377841a0af`.

**Hooks shipped (2026-07-17):** `spec/hooks/SPEC.md` went concept → shipped. `wieland hook` = Claude Code PostToolUse adapter (JSON on stdin — `CLAUDE_FILE_PATHS` is empty in some harnesses; `.rs` filter; walks up to the enclosing `Cargo.toml`; runs the gate scoped to the edited file via the new `gate --file <paths>` scope; exit 2 blocks with rule-mapped violations; infra failures warn + exit 0, never block). `wieland install` copies the corpus to `~/.wieland/rules`; `rules_dir()` resolution is now `WIELAND_RULES` → `./rules` → `~/.wieland/rules`, so the binary works from any project. Validated live against Claude Code on a real Rust project (block + pass both observed).

**M7 TypeScript pack (2026-11-13):** `rules/typescript/` = 20 rules (9 gate, 5 check, 6 judge), own taxonomy `spec/taxonomy/categories-typescript.yaml`. Schema gained `language` (rust|typescript|go), `ts_version`/`go_version`, `detect.oxlint`/`tsconfig`/`golangci`; per-language gate honesty in `validate.rs`. `compile --all` renders every pack (`dist/typescript/…`: core, all, skill `wieland-typescript`, `oxlintrc.json` with all categories off, rulemap); Rust artifacts byte-identical. `wieland gate` picks the language by nearest manifest; TypeScript path = required flags via `tsc --showConfig` + `tsc --noEmit` + oxlint type-aware (pinned in `tools/typescript`, installed to `~/.wieland/tools/typescript`, found via `OXLINT_TSGOLINT_PATH`). `wieland hook` gates `.ts/.tsx/.mts/.cts`, blocking only on the edited file. `check-detectors` and `check-examples --pack typescript` run in `ci/check.sh`. Patch-notes skills live in `skills/` (public text; local notes in `~/.wieland/local/skills/`).

**M8 Go pack (2026-11-13):** `rules/go/` = 20 rules (10 gate, 5 check, 5 judge), taxonomy `categories-go.yaml`, plan `spec/GO-STACK.md`. Detector = golangci-lint v2.15.0 pinned (`go install` into `~/.wieland/tools/go/bin` by `ci/install.sh`). `dist/go/` = core, all, skill `wieland-go`, `golangci.yml` (`default: none`, `uniq-by-line: false`, errcheck excludes only `(io.ReadCloser).Close`), rulemap. Gate/hook pick Go by `go.mod`; a finding blocks when its linter belongs to a gate+deny rule, else advice. `check-detectors` compares golangci-lint's enabled set with the corpus; `check-examples --pack go` vets every Do block. Next: M9 agent adapters, M10 public release.

**Two remotes, two histories (2026-11-13):** `origin` carries the full development history. The public remote carries snapshot commits only, starting from one clean commit. Never push `main` to the public remote; publish with `ci/publish-public.sh [remote] [message]`, which adds one commit with HEAD's tree on top of the public `main` (set `WIELAND_PRIVATE_PATTERN` to a regex of names that must never appear; the script refuses when HEAD matches).

