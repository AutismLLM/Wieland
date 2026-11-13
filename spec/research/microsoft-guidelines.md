# Microsoft Pragmatic Rust Guidelines — provenance

This is the provenance log for folding Microsoft's [Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/) (89 `M-*` guidelines, `vendor/rust-guidelines/`) into the Wieland corpus (`rules/`). A category-fan-out pass processed all 13 guideline directories (`ai`, `apps`, `correctness`, `docs`, `ffi`, `libs/building`, `libs/interop`, `libs/resilience`, `libs/ux`, `macros`, `performance`, `project`, `universal`). Every one of the 89 guidelines landed in exactly one of three buckets — nothing was dropped silently:

- **44 authored** as new Wieland rules (genuine gaps in the existing 265-rule `rust-skills` corpus).
- **42 covered** by an existing rule already teaching the same content (near-verbatim or substantively equivalent).
- **3 long-form**: broad rationale/philosophy with no crisp standalone Do/Don't, already distributed across other category rules.

44 + 42 + 3 = 89.

Every authored rule's frontmatter `sources:` field cites `microsoft/rust-guidelines#M-<NAME>` verbatim, which is the ground truth used to build the Authored table below (cross-checked against each agent's `notes`).

One conflict surfaced during the merge and is unresolved: **M-NO-PRELUDE** ("crates must not define a prelude") directly contradicts the existing corpus rule `proj-prelude-module.md` ("offer a conservative prelude module", sourced from `rust-skills`). It is marked COVERED below since a rule on the topic exists, but the corpus currently teaches the opposite of Microsoft's position. This needs an explicit maintainer ruling — flip, keep, or hold both with a Notes caveat.

A second residual gap was flagged but deliberately not authored: **M-STATIC-VERIFICATION**'s cargo-audit / cargo-hack / cargo-udeps sub-items have no dedicated rule anywhere in the corpus, though the guideline as a whole is marked COVERED via `lint-workspace-lints` since every other sub-item already has one.

## Authored (new rules)

| M-guideline | new rule id |
|---|---|
| M-ASYNC-FN | async-fn-over-future |
| M-ASYNC-STACK-SIZE | async-future-size |
| M-AVOID-STATICS | anti-static-correctness-state |
| M-AVOID-WRAPPERS | api-avoid-wrapper-params |
| M-BUILD-RESULT | api-builder-validate-on-build |
| M-CRATES-FLAT-FOLDER | proj-crates-flat-folder |
| M-DOCUMENTED-MAGIC | doc-magic-value-rationale |
| M-DOC-INLINE | doc-inline-reexport |
| M-DONT-LEAK-TYPES | api-dont-leak-types |
| M-ERRORS-CANONICAL-STRUCTS | err-backtrace-predicates |
| M-ESCAPE-HATCHES | unsafe-ffi-escape-hatch |
| M-ESSENTIAL-FN-INHERENT | trait-inherent-first |
| M-EXAMPLE-OVER-PROC | macro-declarative-over-proc |
| M-FFI-NAMING | name-crate-sys-ffi-suffix |
| M-FFI-TRANSLATES | proj-ffi-core-split |
| M-FIRST-DOC-SENTENCE | doc-first-sentence-short |
| M-IMPL-IO | api-impl-io |
| M-IMPL-RANGEBOUNDS | api-impl-rangebounds |
| M-ISOLATE-DLL-STATE | unsafe-ffi-dll-portable-state |
| M-LATEST-EDITION | proj-latest-edition |
| M-LINT-OVERRIDE-EXPECT | lint-prefer-expect-attribute |
| M-MACROS-DONT-LIE | macro-dont-lie-about-shape |
| M-MIMALLOC-APPS | perf-mimalloc-app |
| M-NO-META-DESIGN-DOCUMENTATION | doc-no-design-journal |
| M-OOBE | proj-build-no-external-tools |
| M-PANIC-CONTINUATION | err-catch-unwind-restart |
| M-PANIC-MESSAGE | err-panic-message-required |
| M-PARAMETER-CONSISTENCY | api-parameter-consistency |
| M-PROC-IMPLIED-ITEMS | macro-proc-no-implied-items |
| M-REGULAR-FN | api-free-fn-over-assoc |
| M-SERVICES-CLONE | own-service-clone-handle |
| M-SHORT-NAMES | name-no-redundant-prefix |
| M-SHRINK-TO-FIT | mem-shrink-to-fit |
| M-SIMPLE-ABSTRACTIONS | api-no-visible-nesting |
| M-SINGLE-ITEM-PATH | proj-single-item-path |
| M-SMALLER-CRATES | proj-split-crate-boundary |
| M-STRONG-TYPES | type-path-not-string |
| M-SYS-CRATES | proj-sys-crate-vendor-source |
| M-TAUTOLOGICAL-TESTS | test-no-tautological |
| M-TEST-UTIL | proj-test-util-feature |
| M-TYPES-SEND | conc-assert-send |
| M-UNSAFE-IMPLIES-UB | unsafe-implies-ub |
| M-WEASEL-WORDS | name-no-weasel-words |
| M-YIELD-POINTS | async-yield-points |

44 rows.

## Covered by existing rules

| M-guideline | existing rule id |
|---|---|
| M-APP-ERROR | err-anyhow-app |
| M-AVOID-INDIRECTION | opt-cache-friendly |
| M-BALANCED-MODULES | proj-mod-by-feature |
| M-BOX-DST | mem-boxed-slice |
| M-CANONICAL-DOCS | doc-errors-section |
| M-CARGO-WORKSPACE | proj-workspace-deps |
| M-COLLECTION-TRAITS | api-impl-fromiterator |
| M-CRATES-IN-WORKSPACE | proj-workspace-deps |
| M-DESIGN-FOR-AI | api-design-for-ai |
| M-DI-HIERARCHY | trait-dyn-vs-generic |
| M-FAST-HASHER | perf-ahash |
| M-FEATURES-ADDITIVE | proj-feature-additive |
| M-FOREIGN-REEXPORTS | proj-pub-use-reexport |
| M-FROM-ERROR | err-from-impl |
| M-HOTPATH | perf-profile-first |
| M-IMPL-ASREF | api-impl-asref |
| M-INIT-BUILDER | api-builder-pattern |
| M-INIT-CASCADED | api-newtype-safety |
| M-INITIAL-CAPACITY | mem-with-capacity |
| M-INTEGRATION-TESTS | test-integration-dir |
| M-LOG-NOT-PRINT | obs-tracing-over-log |
| M-LOG-OVERHEAD | obs-structured-fields |
| M-LOG-STRUCTURED | obs-structured-fields |
| M-MACRO-HELPERS | macro-private-helpers |
| M-MACRO-LAST-RESORT | macro-prefer-functions |
| M-MACRO-MAIN-CRATE | macro-proc-two-crate |
| M-MEM-REUSE | mem-reuse-collections |
| M-MOCKABLE-SYSCALLS | test-mock-traits |
| M-MODULE-DOCS | doc-module-inner |
| M-MSRV | proj-msrv-declare |
| M-NO-GLOB-REEXPORTS | proj-pub-use-reexport |
| M-NO-PRELUDE | proj-prelude-module (contradicts M-NO-PRELUDE — see note above) |
| M-PANIC-IS-STOP | err-result-over-panic |
| M-PANIC-ON-BUG | err-expect-bugs-only |
| M-PROC-IMPL | macro-proc-two-crate |
| M-PUBLIC-DEBUG | api-common-traits |
| M-PUBLIC-DISPLAY | type-display-vs-debug |
| M-STATIC-VERIFICATION | lint-workspace-lints |
| M-STRONG-TYPES-GUARD | type-newtype-validated |
| M-TARGET-CPU | opt-target-cpu |
| M-UNSAFE | unsafe-needs-reason |
| M-UNSOUND | unsafe-send-sync-manual |

42 rows.

## Long-form (rationale, not a discrete rule)

- **M-RUST-SHAPED** — broad philosophy ("don't 1:1 port C#/Java/C++ idioms into Rust") with only a single throwaway example; no crisp standalone Do/Don't. Substance already distributed across existing category rules (error handling under `err-*`, concurrency under `conc-*`, `trait-dyn-vs-generic`, ownership rules).
- **M-THROUGHPUT** — a bulleted list of high-level systems-design principles (partition work ahead, batch, yield within long items, exploit caches, avoid hot-spin/work-stealing) with no single crisp Do/Don't of its own. Concrete sub-points are already rule material elsewhere (batching → `perf-extend-batch`, yielding → `async-yield-points`).
- **M-UPSTREAM-GUIDELINES** — a meta-pointer to external Rust API Guidelines / Style Guide / Design Patterns docs, not itself a Do/Don't. Its highlighted checklist items (C-CONV, C-GETTER, C-COMMON-TRAITS, C-CTOR, C-FEATURE) are individually already covered by existing rules (`name-into-ownership`/`name-as-free`, `name-no-get-prefix`, `api-common-traits`, `api-default-impl`, `proj-feature-additive` respectively).
