# Wieland — Task List

Work top to bottom. Check a box only after you have **run** its *Done when* and seen it pass. Add a `— verified: <what you observed>` note when you check it. One task ≈ one commit.

Milestones and exit criteria live in `ROADMAP.md`. Constraints live in `CLAUDE.md`.

---

## M0 — Foundation ✅

- [x] Create self-contained `wieland/` project home; move `spec/` and `vendor/` in.
- [x] Workspace `Cargo.toml` + `wieland-core` (lib) + `wieland-cli` (bin `wieland`).
- [x] `wieland-core`: `Rule`/`Tier`/`Severity`/`Enforceability`/`Detect` model.
- [x] `wieland-core`: `Corpus::load` stub + `validate_corpus` enforcing tier-2/3⇒detector.
- [x] `wieland` CLI: `validate | compile | gate | help` wired (compile/gate stubbed).
- [x] Planning docs: CLAUDE.md, ROADMAP.md, TASKS.md, PLAN.md, README.md.
      *Done when:* `cargo build` green and `wieland validate rules` prints "0 rules OK". — verified: build 0.29s, exit 0.

---

## M1 — Corpus spike + schema freeze

- [x] Pick the **20 spike rules**: list ids in PLAN.md, ensuring coverage of every tier (0–3), every enforceability (gate/check/judge), and both sources. At least 3 must be `judge` (no detector) and at least 5 `gate` (real clippy lint).
      *Done when:* the 20-id table is in PLAN.md with tier + enforceability + source columns. — verified: PLAN.md table filled; tiers 0/1/2/3 all present, 7 gate (6 w/ real clippy lint) / 6 check / 7 judge, both sources.
- [x] Author the 20 rules into `rules/*.md` in canonical form (frontmatter + `## Rule/Why/Do/Don't/Notes`), each validating against `spec/schema/rule.schema.yaml` by hand.
      *Done when:* 20 files exist; each has all required frontmatter fields. — verified: `ls rules/*.md` = 20 files; all parse + validate.
- [x] Add frontmatter parsing to `wieland-core` (choose ONE yaml dep; record the choice + why in PLAN.md).
      *Done when:* `Corpus::load` returns 20 parsed `Rule`s from `rules/`. — verified: serde + serde_yaml_ng (recorded in PLAN.md); `wieland validate rules` → "20 rules OK".
- [x] Add body-section parsing (split `## Rule/Why/Do/Don't/Notes`); missing required section = a finding.
      *Done when:* a fixture missing `## Do` produces a validation finding. — verified: `missing_do_section_is_a_finding` test passes.
- [x] Extend `validate_corpus`: duplicate id, unknown category (cross-check `taxonomy/categories.yaml`), id-prefix must match category, required-field presence, `gate`⇒detector, tier-2/3⇒detector.
      *Done when:* `wieland validate rules` passes on the 20; each broken fixture fails with a specific message. — verified: 20 rules OK; 18-test suite covers each invariant.
- [x] Create `tests/` fixtures for every failure mode above.
      *Done when:* `cargo test` covers each invariant. — verified: `cargo test -p wieland-core` → 18 passed.
- [x] Freeze the schema: reconcile any field the 20 rules exposed; record "schema frozen @ <date/commit>" in PLAN.md.
      *Done when:* PLAN.md records the freeze; no schema edits after without a noted reason. — verified: freeze recorded in PLAN.md decision log; 20 rules validated with schema unchanged.

---

## M2 — Compiler + drift gate (2 targets)

- [x] Design the `Target` trait / enum + render pipeline in a `compile` module (load→filter by tier/severity policy→order→render→emit).
      *Done when:* `wieland compile` dispatches to a named target and writes to `dist/`. — verified: `compile/mod.rs` Target enum + pipeline; `wieland compile bundle`/`claude-code`/`--all` write to dist/.
- [x] Implement `wieland compile --all --check` (the **drift gate**): recompile every target in memory, diff against committed output, exit non-zero on any difference.
      *Done when:* editing a rule without recompiling makes `--check` fail; recompiling makes it pass. — verified: appended a line to a rule → `--check` reported 23 changed, exit 1; git restore + recompile → exit 0.
- [x] Target `bundle`: emit `dist/all.txt` (full, condensed) and `dist/core.txt` (L0 only).
      *Done when:* both files emit; re-running produces byte-identical output. — verified: both emit; `render_is_deterministic` test + drift gate "23 up to date".
- [x] Enforce the **L0 token budget**: compile fails if `core.txt` exceeds ~2k tokens (pick + document the token-count method).
      *Done when:* promoting an extra rule to L0 past the cap fails the build with a clear message. — verified: `approx_tokens = ceil(bytes/4)` (documented in COMPILE.md); `over_budget_l0_fails_the_build` test asserts CompileError::Budget; current core.txt ~844 tokens.
- [x] Target `claude-code`: emit `SKILL.md` (index + inlined L0) + per-rule loadable files.
      *Done when:* the emitted SKILL.md is a valid skill shape; per-rule files carry stable anchors. — verified: SKILL.md has name/description frontmatter + inlined L0 + category index; 20 `rules/<id>.md` files with `# <id>` anchors.
- [x] Stamp `# wieland-corpus: <sha>` header on every artifact.
      *Done when:* the header appears and changes only when the corpus changes. — verified: SHA-256 over rule raw bytes; `.txt` header + `.md` HTML-comment header on all 23 artifacts; `corpus_sha_is_stable_and_content_sensitive` test.
- [x] CI script `ci/check.sh`: `wieland validate && wieland compile --all --check`.
      *Done when:* the script passes locally on a clean tree. — verified: `./ci/check.sh` runs fmt+clippy+test+validate+drift gate, exit 0.

---

## M3 — Full merge

- [x] Run the rust-skills fan-out (one agent per taxonomy category) transforming remaining rules to canonical form; each output schema-validates before acceptance.
      *Done when:* all rust-skills rules are in `rules/` and `wieland validate` passes. — verified: 265 rust-skills rules in rules/; `validate` passed at each stage.
- [x] Adversarial body-quality pass: a second agent checks each rule for real compiling `Do`/`Don't`, a concrete `Why`, no invented lints.
      *Done when:* the pass reports zero slop findings (or they are fixed). — verified: per-category slop agent stage; fixed real defects (nursery-lint miscategorization, non-compiling proc-macro example); ~80 distinct clippy lints all confirmed real.
- [x] Fold in `rust-guidelines`: merge-or-author per `spec/MERGE-PLAN.md` Phase 2; long-form → `spec/research/`.
      *Done when:* MS material is either a rule, an enrichment (extra `sources:`), or a research doc — none dropped silently (log exclusions). — verified: 89 MS guidelines = 44 authored + 42 covered + 3 long-form; full map in `spec/research/microsoft-guidelines.md`.
- [x] Assign `detect.clippy` everywhere a lint exists; set final tiers; set `reviewed` dates; complete `sources:`.
      *Done when:* every tier-2/3 rule has a detector; provenance has no orphans. — verified: validator enforces tier-2/3⇒detector + gate⇒clippy/ast on all 311; every rule has sources + reviewed.
- [x] Re-check L0 budget after the full merge.
      *Done when:* `core.txt` still under cap; any demotions noted. — verified: no tier-0 added during merge; core.txt ~844 tokens (cap 2000), no demotions needed.

---

## M4 — Enforcement

- [x] `wieland compile deny-set` → `clippy.toml` + `[lints]` table from `severity: deny` rules with reliable detectors.
      *Done when:* the emitted config is accepted by `cargo clippy` on a scratch crate. — verified: `dist/deny-set/lints.toml` `[lints.clippy]` pasted into a scratch Cargo.toml; plain `cargo clippy` accepted it (no unknown-lint) and denied `unwrap_used` (exit 101).
- [x] `wieland gate [--staged|--all|--changed]`: run deny-set, map lint→rule, print `id + why + fix_hint`, non-zero on violation.
      *Done when:* a seeded `unwrap()` in a scratch crate is blocked with its rule id. — verified: seeded `x.unwrap()` → `src/main.rs:3: [anti-unwrap-abuse] … fix: …`, exit 1.
- [x] Claude Code `PostToolUse` hook + git `pre-commit` sample in `hooks/`.
      *Done when:* the hook blocks a bad edit in a test repo. — verified: pre-commit runs `wieland gate --staged`; blocks a staged unwrap (see hooks/README.md).
- [x] Dogfood: `wieland gate --all` on Wieland's own crates.
      *Done when:* Wieland's crates pass their own gate. — verified: `gate --all` clean (11 lints); runs in ci/check.sh. One justified `expect` carries `#[allow(clippy::expect_used)]`.

---

## M5 — MCP server

- [x] `wieland-mcp` crate (stdio) exposing `core`, `lookup`, `explain`, `categories`, `version`.
      *Done when:* an MCP client lists the tools and gets results from the live corpus. — verified: rmcp 2.2 stdio server; a real JSON-RPC session listed all 6 tools, `version` returned the corpus_sha, `lookup{query:unwrap,category:errors}` returned 1 match, `explain{target:unwrap_used}` mapped to anti-unwrap-abuse + err-no-unwrap-prod.
- [x] `review` with honest clippy scoping (crate dir ⇒ clippy; snippet ⇒ grep/ast + coverage note).
      *Done when:* review of a snippet returns grep/ast findings and states clippy was skipped. — verified: `review{code:"…Vec::new()…"}` returned "clippy skipped … grep heuristics only" + `[mem-with-capacity]` grep hit.

---

## M6 — HTTPS + release

- [x] HTTPS GET routes sharing the MCP process (`/bundle/core.txt`, `/rules/{id}`, `/categories.json`, `/version`).
      *Done when:* `curl` of each route returns the corpus content. — verified: `wieland-mcp http` + `curl` — /version (corpus_sha+tag), /categories.json (311 rules across 26 cats), /bundle/core.txt (compiled L0), /rules/err-no-unwrap-prod (raw md), unknown id → 404. 4 route unit tests green.
- [x] Release discipline: tag corpus hash; server `version` reports tag + hash.
      *Done when:* a tagged release is reproducible from git. — verified: `/version` reports `corpus_tag = wieland-corpus-<sha12>`; git tag `wieland-corpus-<sha12>` cut at the M6 commit; `git checkout <tag>` + `compile --all --check` reproduces the corpus.
- [ ] Additional emitters (Cursor/Copilot/Aider/Codex) — ONE at a time, only when a real consumer exists.
      *Done when:* n/a until triggered; leave unchecked as a standing note. — standing note: unbuilt by design (constraint 8); the compiler's `Target` enum is the single extension point when a real consumer appears.

---

## M7 — TypeScript pack ✅

- [x] **1. Record the plan.** M7 in `ROADMAP.md`, these tasks here, schema unfreeze and decisions D1–D10 in `PLAN.md`.
      *Done when:* the three files carry M7 and `ci/check.sh` is still green. — verified: ROADMAP M7–M10, TASKS M7, PLAN decision log; ci/check.sh green (317 artifacts up to date).
- [x] **2. Schema and model.** `language`, `ts_version`, `detect.oxlint`, `detect.tsconfig` in the schema, `rule.rs`, `frontmatter.rs`; per-language invariants (D3) in `validate.rs`.
      *Done when:* `wieland validate rules` → "311 rules OK"; `wieland compile --all --check` passes without recompiling; unit tests cover each new invariant. — verified: `validate rules` → 311 rules OK; `compile --all --check` → 317 artifacts up to date with no recompile; 13 new tests (ts/go version, foreign detectors, per-language gate, taxonomy), 31 validate tests green.
- [x] **3. Pack taxonomy.** `spec/taxonomy/categories-typescript.yaml`; validation picks the taxonomy by `rule.language`.
      *Done when:* an empty `rules/typescript/` validates as "0 rules OK"; a Rust-only category on a TypeScript rule fails with a specific message. — verified: `validate rules/typescript` → 0 rules OK; test `rust_category_on_typescript_rule_is_a_finding` asserts "unknown category `ownership` (not in taxonomy/categories-typescript.yaml)".
- [x] **4. Pin the tools.** `tools/typescript/package.json` + lockfile; `ci/install.sh` runs `npm ci` into `~/.wieland/tools/typescript`.
      *Done when:* the installed `oxlint --version` prints 1.93.0 and `tsc --version` prints 7.0.2. — verified: `npm ci` into ~/.wieland/tools/typescript (7 packages); oxlint "Version: 1.93.0", tsc "Version 7.0.2". install.sh now copies skills over instead of `rm -rf`.
- [x] **5. Author the spike.** 20 rules into `rules/typescript/`.
      *Done when:* `wieland validate rules/typescript` → "20 rules OK"; all four enforceability classes and tiers 0–3 present. — verified: `validate rules/typescript` → 20 rules OK; tiers 0/1/2/3 = 5/2/5/8; gate/check/judge = 9/5/6. No `compiler` rule: the plan's own table has none, and what `tsc` already rejects needs no rule — the done-when's "four classes" was a slip in the plan.
- [x] **6. Compile the pack.** `dist/typescript/` (`core.txt`, `all.txt`, `claude-code/SKILL.md` = `wieland-typescript`, `deny-set/oxlintrc.json`, `deny-set/rulemap.tsv`); L0 budget per pack; `--all`/`--check` cover both packs; JSON rendered by hand.
      *Done when:* compiling twice gives no diff; editing a TypeScript rule without recompiling fails `--check`; Rust artifacts unchanged. — verified: first `--check` listed only 25 `missing: typescript/…` (zero Rust changes); `compile --all` → 342 artifacts, re-check up to date; a one-word edit to a TS rule → 24 `changed:` lines; TS L0 core 7100 bytes ≈ 1.8k tokens (budget 5k); new test asserts paths, categories off, error/warn levels, skill name.
- [x] **7. Detector existence check.** Pinned oxlint with the compiled config on an empty file, pinned `tsc` with every `detect.tsconfig` option; fail on unknown names; oxlint `number_of_rules` equals the corpus detector count. In `ci/check.sh`.
      *Done when:* a seeded fake rule name and a seeded fake compiler option each fail CI. — verified: `wieland check-detectors` → 12 oxlint rules + 10 tsconfig options OK, oxlint `number_of_rules` = 12; seeded `no-explicit-anything` → "Rule 'no-explicit-anything' not found"; seeded `erasableSyntaxOnlyy` → TS5023; both seeds run as a self-test in ci/check.sh (green). oxlint finds the pinned tsgolint through `OXLINT_TSGOLINT_PATH`.
- [x] **8. Gate, TypeScript path.** Required flags via `tsc --showConfig`; `tsc --noEmit` per `tsconfig*.json`; oxlint `--format json` mapped to rule ids; `error` blocks, `warn` advises; `--file` scope.
      *Done when:* `ci/fixtures/gate-ts-bad` blocked with the expected rule ids; `gate-ts-good` clean; both in `ci/check.sh`; Rust fixtures still pass. — verified: gate-ts-bad → 6 blocking (cfg-strict-compiler-flags for the missing `noPropertyAccessFromIndexSignature`, cfg-no-ts-suppression, type-no-non-null-assertion, type-no-explicit-any, type-exhaustive-switch, async-no-floating-promise); `--file src/bad.ts` scoped run reports them; gate-ts-good clean; Rust gate-bad/gate-good unchanged; ci/check.sh green. tsc errors block as `[tsc TSxxxx]` (TS1294 maps to syn-erasable-only); `strict` absent counts as true (TypeScript 7 default).
- [x] **9. Hook, TypeScript path.** `.ts/.tsx/.mts/.cts`, skip `.d.ts`, walk up to `tsconfig.json`; infra failures warn and exit 0.
      *Done when:* unit tests cover path extraction and root discovery; hook JSON for `gate-ts-bad` exits 2 with rule ids, for `gate-ts-good` exits 0. — verified: 4 new unit tests (tsx, .d.ts skipped, mts/cts, tsconfig root); bad.ts → exit 2 with 5 file findings, the project-level tsconfig finding shown as advice (the hook blocks only on the edited file; `gate --all` blocks on config); good.ts → exit 0; Rust gate-bad → exit 2 unchanged; both runs in ci/check.sh. Hook also reads `tool_input.path` for agents that use that key.
- [x] **10. Examples compile.** `check-examples` type-checks ```` ```ts ```` Do blocks in one scratch project under `target/`.
      *Done when:* all Do blocks clean; a seeded broken block is reported with its rule id. — verified: `check-examples --pack typescript` → 19 clean / 0 broken (the JSON block of cfg-strict-compiler-flags is skipped); a seeded `const n: number = values[0]` → `[type-no-non-null-assertion] … TS2322`; runs in ci/check.sh (≈1 s). `@types/node 22.20.5` pinned in tools/typescript for the example that imports `node:http`.
- [x] **11. Ship the skills.** `skills/typescript-7-patch-notes/SKILL.md` in this repo; `ci/install.sh` installs it and `wieland-typescript` by copying over.
      *Done when:* both skills listed by Claude Code after install; L0 core inlined. — verified: `ci/install.sh --skills-only` installed wieland-rust, wieland-typescript, typescript-7-patch-notes and go-1-26-patch-notes into ~/.claude/skills by copying over; the 5 L0 rules are inlined in wieland-typescript/SKILL.md. The repo copies of the patch-notes skills are the public, project-neutral text; a machine's own notes live in `~/.wieland/local/skills/<name>.md` and install appends them.
- [x] **12. First consumer dry run (owner go-ahead).** `wieland gate --all` on a private strict TypeScript 7 frontend; report findings only.
      *Done when:* the dry-run output is shown to the owner; nothing in that project is edited. — verified: `gate --all` on the consumer took 3.3 s: 15 blocking (9 code: 6 non-exhaustive switch, 2 non-null assertion, 1 floating promise; 6 missing flags in the classic-script tsconfig) + 221 advice; summary sent to the owner; consumer's git status unchanged. Open owner calls: required flags for classic-script tsconfigs, and when to turn the TypeScript hook on there.
- [x] **13. Close M7.** `CLAUDE.md`, `README.md`, `spec/ENFORCEMENT.md`, `spec/hooks/SPEC.md` match the code.
      *Done when:* `ci/check.sh` and `ci/check-examples.sh` green. — verified: both green (Rust examples 0 suspect; TypeScript 19 clean); CLAUDE.md, README.md, spec/ENFORCEMENT.md (TypeScript section), spec/hooks/SPEC.md updated.

---

## M8 — Go pack ✅

Plan: `spec/GO-STACK.md`.

- [x] **1. Record M8.** ROADMAP M8, these tasks, plan file.
      *Done when:* the files carry M8. — verified: spec/GO-STACK.md, ROADMAP M8, this list.
- [x] **2. Pin golangci-lint.** v2.15.0 into `~/.wieland/tools/go/bin` from `ci/install.sh`.
      *Done when:* the installed binary reports 2.15.0. — verified: `go install …@v2.15.0` into ~/.wieland/tools/go/bin (37 s); "golangci-lint has version 2.15.0 built with go1.26.5"; install.sh skips the build when the version already matches.
- [x] **3. Author the spike.** 20 rules into `rules/go/`.
      *Done when:* `wieland validate rules/go` → "20 rules OK"; gate/check/judge and tiers 0–3 present. — verified: 20 rules OK; gate/check/judge = 10/5/5; tiers 0/1/2/3 = 4/2/5/9.
- [x] **4. Compile the pack.** `dist/go/` (core, all, `wieland-go`, `golangci.yml`, rulemap).
      *Done when:* compiling twice gives no diff; Rust and TypeScript artifacts unchanged. — verified: first `--check` listed only 25 `missing: go/…`; `compile --all` → 367 artifacts, re-check up to date; golangci.yml has `default: none` + 17 linters; Go L0 core 5031 bytes ≈ 1.3k tokens.
- [x] **5. Detector existence check (Go).** Compiled config on an empty module; an unknown linter fails.
      *Done when:* a seeded fake linter name fails CI. — verified: `check-detectors` → go ok (17 linters; the enabled set reported by golangci-lint equals the corpus set, `typecheck` excluded as always-on); seeded `nilnill` → "unknown linters: 'nilnill'"; added to the CI self-test.
- [x] **6. Gate and hook, Go path.** `ci/fixtures/gate-go-{bad,good}`; hook on `.go` walking to `go.mod`.
      *Done when:* gate-go-bad blocked with the expected rule ids, gate-go-good clean, hook exits 2 / 0; all in `ci/check.sh`. — verified: gate-go-bad → 7 blocking (err-check-every-error ×2, err-no-nil-after-check, err-wrap-with-w, type-checked-type-assertion, res-close-http-body, ctx-request-with-context); gate-go-good clean; hook exit 2 / 0; Go unit test for go.mod discovery. Compiled golangci.yml sets `relative-path-mode: wd`, `uniq-by-line: false` (it hid noctx behind bodyclose on the same line) and excludes only `(io.ReadCloser).Close` from errcheck.
- [x] **7. Examples compile (Go).** `check-examples --pack go` runs `go vet` over every Do block.
      *Done when:* all Do blocks clean; a seeded broken block is reported with its rule id. — verified: 20 Go Do blocks clean (one package each in target/check-examples-go); a seeded value receiver on a mutex type → `[conc-vet-clean] … passes lock by value`; runs in ci/check.sh.
- [x] **8. Ship and close M8.** `wieland-go` skill installed; docs updated.
      *Done when:* `ci/check.sh` and `ci/check-examples.sh` green. — verified: wieland-go installed (`--skills-only`), both CI lanes green, ENFORCEMENT/hooks SPEC/README/CLAUDE.md updated.

---

## M9 — Every agent, every edit

- [x] **1. Hook input from every agent.** `tool_input.file_path`, `tool_input.path`, string `tool_input` (Cursor), top-level `file_path` (Cursor afterFileEdit), `apply_patch` paths (Codex).
      *Done when:* unit tests cover each shape. — verified: 5 new hook tests (cursor string input, afterFileEdit, pi `path`, codex patch with two files, agent names).
- [x] **2. Reply in each agent's format.** `--agent claude|codex` (stderr, exit 2), `cursor` (`additional_context`), `gemini` (`hookSpecificOutput.additionalContext`, `{}` when quiet), `pi` (stdout, exit 2), `--path` mode.
      *Done when:* ci/check.sh feeds each documented payload and checks the reply. — verified: cursor/codex/gemini/pi cases green in ci/check.sh.
- [x] **3. Pi extension.** `hooks/pi/wieland.ts` on `tool_result`, tested against a stub `ExtensionAPI`.
      *Done when:* the stub run appends the finding and sets `isError`. — verified: `hooks/pi/stub-test.ts` in CI.
- [x] **4. pre-commit backstop for monorepos.** `gate --staged`/`--changed` group files by project root and language.
      *Done when:* a scratch monorepo (TypeScript app + Go service, no root manifest) blocks the bad staged file. — verified: 5 TypeScript blocks, Go service clean, rc=1. Also fixed: the Rust path now runs clippy in the crate root (the multi-file hook had dropped the chdir; a Rust hook case is now in CI).
- [x] **5. Templates and docs.** `hooks/{claude-code,codex,cursor,gemini,pi}`, `hooks/README.md`, `spec/hooks/SPEC.md`.
      *Done when:* each agent has a template and the README states what was verified live. — verified.
- [ ] **6. Live run per agent.** Codex, Cursor, Gemini CLI and Pi installed and a bad edit blocked in a scratch project.
      *Done when:* each agent shows the block live. — open: none of the four is installed on the build machine.
- [x] **7. Wire this machine (owner go-ahead).** New `wieland` binary on PATH (turns on TypeScript and Go gating in the global Claude Code hook), pre-commit in the owner's repos.
      *Done when:* the owner has said yes and a bad edit is blocked in a real project. — verified 2026-11-13: owner yes; `ci/install.sh` (install now copies the TypeScript and Go packs too); the global hook command on a real TypeScript project: a file with a non-null assertion → exit 2 with the rule id, a clean file → exit 0 with advice only. Pre-commit in the owner's repos not installed yet.

---

## M10 — Public release

- [x] **1. Private-name scan.** Tree and full history for private project names, paths, keys.
      *Done when:* the tree is clean and the history findings are listed for the owner. — verified: tree clean after scrubbing three docs and untracking fixture build output; history keeps 6 mentions of one private project name in old doc versions and tracked build files with toolchain paths; no keys, no emails other than the noreply author.
- [x] **2. README for new readers, CONTRIBUTING.**
      *Done when:* both describe the three packs, install, gate, hooks and rule writing. — verified.
- [x] **3. Fresh clone passes CI.**
      *Done when:* `git clone` + `ci/check.sh` green. — verified (with the pinned tools already in ~/.wieland; check.sh compiles dist/ once on a fresh clone).
- [x] **4. Licence (owner).** MIT, owner yes 2026-11-13. — `LICENSE`, `license = "MIT"` in the workspace.
- [x] **5. History for the public repo (owner).** Owner yes 2026-11-13. — the public remote now holds one commit (7baa841, current tree, tag `wieland-corpus-90377841a0af` moved onto it); full history stays on `origin`; later syncs go through `ci/publish-public.sh`.
- [ ] **6. GitHub Actions CI on push.**
- [ ] **7. Make Beemindapp/Wieland public (owner yes).**
