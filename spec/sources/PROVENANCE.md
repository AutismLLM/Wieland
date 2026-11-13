# Wieland — Provenance & Sources

Every rule records its origin in frontmatter `sources:`. This file is the ledger: where the raw material comes from, how ids map, and the license posture. Merges must stay auditable — no rule of unknown origin ever enters the corpus.

---

## Vendored inputs

Both live as siblings under `../` (their `.git` stripped; vendored as plain files, re-clone to refresh):

### `leonardomso/rust-skills` → `../rust-skills/`
- 265 rules, 26 categories, one Markdown file per rule under `rules/`.
- Already close to Wieland's shape: small, id-prefixed, category-scoped. The **primary corpus**.
- Provenance tag: `leonardomso/rust-skills#<original-filename-without-.md>`.
- License: see `../rust-skills/LICENSE`. Record and honor at merge; keep attribution in `sources:`.

### `microsoft/rust-guidelines` → `../rust-guidelines/`
- Prose guidelines + condensed `src/agents/all.txt` (~34k tokens) written for LLM consumption.
- Less atomic than rust-skills; contributes **depth, rationale, and API-design guidance**, plus the served `all.txt` research bundle.
- Provenance tag: `microsoft/rust-guidelines#<section-anchor>`.
- License: see `../rust-guidelines/LICENSE.md`. Record and honor at merge.

### `wieland` (originals)
- Rules with no upstream — house additions (gaps neither source covers).
- Provenance tag: `wieland`.

---

## Id & de-duplication policy

- rust-skills ids are adopted **as-is** where they already fit the `<prefix>-<slug>` scheme and the taxonomy prefix.
- Where both sources cover the same idea, they merge into **one** rule: the richer body wins, both `sources:` are listed, and any dropped id is recorded in `supersedes:`.
- No silent loss: if a source rule is intentionally excluded (redundant, outdated, wrong for edition 2024), it is logged in `MERGE-LOG.md` (created at merge) with a reason.

---

## License note

Wieland is a *derivative synthesis*. At merge:
1. confirm both upstream licenses permit derivative + redistribution;
2. carry required attribution in each rule's `sources:` and in a top-level `NOTICE`;
3. state Wieland's own license once the merge is done.

Until the merge runs, this scaffold contains **no** copied rule text — only schema, taxonomy, and one original example rule. The vendored sources sit untouched beside it.
