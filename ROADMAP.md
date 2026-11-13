# Wieland — Roadmap

Milestones, each with a hard exit criterion you can *run*. The order encodes the Fosite ruling: prove the corpus and the drift gate on a tiny slice before building breadth. Do not reorder.

Legend: ☐ open · ☑ done · ▣ in progress

---

## ☑ M0 — Foundation
Stand up the project home and a compiling Rust skeleton.
- Self-contained folder, spec + vendor moved in, planning docs written.
- 2-crate workspace (`wieland-core`, `wieland-cli`) builds; `wieland validate` runs.
- The tier-2/3-needs-detector invariant is real code, not just prose.

**Exit:** `cargo build` green · `wieland validate rules` exits 0 on the empty corpus · CLAUDE.md/ROADMAP/TASKS/PLAN present. **✅ met.**

---

## ☑ M1 — Corpus spike + schema freeze
Merge a small, representative slice and let it pressure-test the schema before it hardens.
- Hand-merge **20 rules** spanning all 4 tiers, all enforceability classes, and BOTH sources (`vendor/rust-skills` + `vendor/rust-guidelines`).
- Implement real frontmatter parsing + body-section parsing in `wieland-core` (first deliberate deps: a YAML parser + Markdown split).
- `wieland validate` loads the 20, checks schema + invariants (dup ids, unknown category, tier/detector rule, required fields).
- Freeze `spec/schema/rule.schema.yaml` against what the 20 taught us.

**Exit:** `wieland validate rules` parses 20 real rules and passes · deliberately-broken fixtures fail with clear messages · schema tagged frozen in PLAN.md.

---

## ☑ M2 — Compiler + drift gate (2 targets only)
Prove the projection on the smallest useful target set, gate against staleness first.
- Build the **drift gate** `wieland compile --all --check` (recompile every target, fail on any diff) — this lands *before* the second emitter.
- Emit target `bundle`: `dist/all.txt` + `dist/core.txt`.
- Emit target `claude-code`: `SKILL.md` + per-rule files.
- Enforce the **L0 ~2k-token budget** in the compiler (build fails if `core.txt` exceeds it).
- Stamp every artifact with a `# wieland-corpus: <sha>` header.

**Exit:** both targets emit deterministically (re-run = no diff) · drift gate fails when a rule is edited without recompiling · L0 over-budget fails the build.

---

## ☑ M3 — Full merge
Pour the rest of the corpus with quality control.
- Merge remaining ~245 rust-skills rules (fan-out: one agent per category, schema-validated, adversarial body-quality second pass).
- Fold in `rust-guidelines`: enrich matching rules, author `wieland`-originals for genuine gaps, route long-form rationale to `spec/research/` (not into rules).
- Assign every rule `enforceability` + `detect{}` where a clippy lint exists; set tiers; complete provenance; set `reviewed` dates.

**Exit:** full corpus validates clean · every tier-2/3 rule has a real detector · L0 core still under budget · provenance complete (no orphan rules).

---

## ☑ M4 — Enforcement (hooks + deny-set)
Turn the deny rules into gates that actually block.
- `wieland compile deny-set` → real `clippy.toml` + `[lints]` table.
- `wieland gate [--staged|--all|--changed]` runs the deny-set, maps lints back to rule ids, returns `id + why + fix_hint`.
- Claude Code `PostToolUse` hook + generic git `pre-commit` wired.
- **Dogfood:** run the gate on Wieland's own crates; they must pass.

**Exit:** a seeded violation in a scratch crate is blocked with a rule-referenced message · Wieland's own `cargo clippy`/gate is clean.

---

## ☑ M5 — MCP server (wieland-forge)
Serve the live corpus to MCP-speaking agents.
- `wieland-mcp` crate: stdio transport first; tools `core`, `lookup`, `explain`, `categories`, `version`.
- `review` with honest clippy scoping (crate context required; snippet path = grep/ast only, says so).

**Exit:** an MCP client can `lookup` a category and `explain` a lint against the live corpus · `version` reports the corpus hash.

---

## ☑ M6 — HTTPS endpoint + release discipline
Reach non-MCP consumers and make updates releases, not live pushes.
- HTTPS GET routes (`/bundle/core.txt`, `/rules/{id}`, `/categories.json`, `/version`) sharing the MCP server process.
- Corpus versioning: tag a release = tag the corpus hash; server reports it.
- Add further export targets (Cursor/Copilot/Aider/Codex) **only if** a real consumer now needs one.

**Exit:** `GET /bundle/core.txt` serves the current L0 · a tagged corpus release is reproducible from git.

---

## ☑ M7 — TypeScript pack
Same engine, same words, second language. Rules in `rules/typescript/`, detectors `tsc` (required strict flags) and oxlint type-aware, pinned by Wieland.
- Schema grows four optional fields (`language`, `ts_version`, `detect.oxlint`, `detect.tsconfig`); per-language gate honesty.
- 20-rule spike: 9 gate/deny, 5 check, 6 judge.
- `compile --all` emits `dist/typescript/` (core, all, skill `wieland-typescript`, `oxlintrc.json`, rulemap).
- `wieland gate` and `wieland hook` pick the language from the nearest manifest (`Cargo.toml` → Rust, `tsconfig.json` → TypeScript).

**Exit:** a bad TypeScript edit in a project with a `tsconfig.json` is blocked by `wieland hook` with a rule id and a fix line; a good one passes; the Rust pack is byte-identical to before. **✅ met** (ci/check.sh hook integration; Rust artifacts unchanged).

---

## ☑ M8 — Go pack
Third language on the same engine. Rules in `rules/go/`, detector golangci-lint v2.15.0 (pinned; carries govet, staticcheck, errcheck), plan in `spec/GO-STACK.md`.

**Exit:** a bad Go edit in a module with a `go.mod` is blocked by `wieland hook` with a rule id and a fix line; a good one passes; Rust and TypeScript packs unchanged. **✅ met** (ci/check.sh Go gate + hook integration).

---

## ▣ M9 — Every agent, every edit
The gate runs no matter which tool wrote the code: Claude Code, Cursor, Codex, Gemini CLI, Pi, or a human. Per-agent hook adapters plus a git `pre-commit` hook as the backstop for everything else; `wieland install` sets up tools, rules and skills for all three packs.

**Exit:** for each supported agent, a scratch project shows a bad edit blocked and a good edit passing; `pre-commit` blocks a bad commit made without any agent.

---

## ▣ M10 — Public release
Wieland as an open-source project: README for strangers, licence, contribution guide, CI on push, no private references anywhere in the tree or history.

**Exit:** a fresh clone on a clean machine runs `ci/install.sh` and `ci/check.sh` green; a scan for private names over the tree and the history is empty.

---

## Speculative (not scheduled — note, don't build)
- More languages (Python, …) reusing the same engine with language-specific detectors.
- Rule authoring UI / web rulebook render.
- Personalized per-project rule profiles.
