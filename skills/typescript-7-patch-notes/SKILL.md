---
name: typescript-7-patch-notes
description: Reference for what changed in TypeScript 6.0 (Mar 2026), 7.0 (Jul 2026, the native Go compiler) and the 7.1 beta (Oct 2026) versus the TypeScript 5.x covered in training data. Load this before writing, reviewing, configuring or linting TypeScript on a 6.x/7.x toolchain — covers the removed tsconfig options that are now hard errors, the new defaults (strict, types [], rootDir), the missing JS compiler API in 7.0 (typescript-eslint cannot run on it), the linter that does work (oxlint type-aware), Node running .ts directly, and new stdlib types (Temporal, Map.getOrInsert, RegExp.escape). Easy to get wrong from stale knowledge, e.g. writing baseUrl, moduleResolution node, enums, `import ... assert`, or `require("typescript").createProgram`.
---

# TypeScript 7 — what's actually new

TypeScript 7.0 shipped 2026-07-08. It is the same language as 6.0, but the compiler and language server were ported to Go: 8–12x faster full builds, parallel type checking. 6.0 (the last JavaScript-based release) was the transition release that changed the defaults and deprecated old options; 7.0 turns every one of those deprecations into a hard error. 7.1 is in beta (2026-10-20).

`npm install -D typescript` gives 7.x. Latest stable when this was written: **7.0.2**. Beta: `typescript@beta` (7.1.0-beta). Nightly: `typescript@next`.

Sources (fetch live when exact wording matters; this is a working summary):
- https://devblogs.microsoft.com/typescript/announcing-typescript-6-0/
- https://devblogs.microsoft.com/typescript/announcing-typescript-7-0/
- https://devblogs.microsoft.com/typescript/announcing-typescript-7-1-beta/
- https://github.com/microsoft/typescript-go/blob/main/CHANGES.md (6.0 vs 7.0 differences, mostly JavaScript/JSDoc)

Lines marked **[tested]** were run against `typescript@7.0.2` on Linux (Node v22.23.1) on 2026-11-12.

## tsconfig — removed options are hard errors now

`"ignoreDeprecations": "6.0"` only worked in 6.0. In 7.0 it does not rescue anything **[tested]**.

| Do not write | Error in 7.0 | Write instead |
|---|---|---|
| `"baseUrl": "./src"` | TS5102 removed **[tested]** | drop it; put the prefix in `paths` (`"@app/*": ["./src/app/*"]`) |
| `"target": "es5"` | TS5108 removed **[tested]** | lowest target is `es2015`; normally leave the default |
| `"moduleResolution": "node"` / `"node10"` / `"classic"` | TS5108 removed **[tested]** | `"bundler"` (bundled or browser code) or `"nodenext"` (Node) |
| `"module": "amd" \| "umd" \| "system" \| "none"` | TS5108 removed (`none`: TS6046, not a valid value) **[tested]** | `"esnext"` or `"preserve"` with a bundler, `"nodenext"` for Node |
| `"esModuleInterop": false`, `"allowSyntheticDefaultImports": false` | TS5108 removed **[tested]** | always on; write `import express from "express"` |
| `"alwaysStrict": false` | TS5108 removed **[tested]** | all code is strict-mode JavaScript |
| `"downlevelIteration"` | TS5102 removed **[tested]** | nothing; it only affected ES5 output |
| `"outFile"` | TS5102 removed **[tested]** | use a bundler |

Also: `"lib": ["dom", "dom.iterable"]` is now just `["dom"]`; `dom.iterable` and `dom.asynciterable` are folded in **[tested]**.

## tsconfig — defaults changed (since 6.0)

- `strict` is **true** by default **[tested]**.
- `types` defaults to **`[]`**. `@types/*` packages are no longer picked up automatically; `process`, `describe`, `Bun` and friends are unknown until listed: `"types": ["node"]` **[tested]**. `"types": ["*"]` restores the old behaviour.
- `rootDir` defaults to the tsconfig's directory. If sources are in `src/`, set `"rootDir": "./src"` or output lands in `dist/src/`.
- `module` defaults to `esnext`; `target` defaults to the newest stable ES version. In 7.0.2 that is es2025: with no `lib` set, `RegExp.escape` type-checks and `Map.prototype.getOrInsert` does not **[tested]**.
- `noUncheckedSideEffectImports` is true by default: `import "./missing.js"` is an error (TS2882) **[tested]**.
- `libReplacement` is false by default.
- `stableTypeOrdering` is always on in 7.0 (unions print in a deterministic order; `.d.ts` output can differ from 5.x/6.0).
- `tsc file.ts` in a folder that has a `tsconfig.json` is an error (TS5112). Use `tsc -p .`, or `tsc --ignoreConfig file.ts` **[tested]**.

`tsc --init` in 7.0.2 writes this (comments removed) **[tested]** — a good picture of what "modern default" means:

```json
{
  "compilerOptions": {
    "module": "nodenext",
    "target": "esnext",
    "types": [],
    "sourceMap": true,
    "declaration": true,
    "declarationMap": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true,
    "strict": true,
    "jsx": "react-jsx",
    "verbatimModuleSyntax": true,
    "isolatedModules": true,
    "noUncheckedSideEffectImports": true,
    "moduleDetection": "force",
    "skipLibCheck": true
  }
}
```

## Syntax that is now an error

- `module Foo { }` → TS1540. Write `namespace Foo { }` **[tested]**. The ambient form `declare module "pkg" { }` is not affected.
- `import x from "./x.json" assert { type: "json" }` → TS2880. Write `with { type: "json" }`. Same for `import("./x.json", { assert: { ... } })` **[tested]**.
- Under `erasableSyntaxOnly` (from 5.8, now common because Node strips types): `enum`, runtime `namespace`, and constructor parameter properties (`constructor(private x: number)`) are TS1294 **[tested]**. Use `as const` objects with a union type, ES modules, and explicit fields.

## The compiler has no JavaScript API in 7.0

This is the change most likely to produce wrong advice.

- `require("typescript")` in 7.0.2 exports only `version` and `versionMajorMinor`. There is no `ts.createProgram`, no `ts.transpileModule`, no language-service plugin host **[tested]**.
- Anything that embeds the compiler does not work on 7.0: **typescript-eslint** (peer range is `typescript >=4.8.4 <6.1.0` as of 8.72.2 **[tested]**), ts-morph, ts-node, Volar-based tooling (Vue, Svelte, Astro, MDX), Angular template checking. Tools that only shell out to `tsc`, or that strip types themselves (esbuild, Vite, swc, Node), are unaffected.
- Official side-by-side recipe when a tool still needs the old API: alias `typescript` to the 6.0 compatibility package, and keep 7 for `tsc`.

```json
{
  "devDependencies": {
    "@typescript/native": "npm:typescript@^7.0.2",
    "typescript": "npm:@typescript/typescript6@^6.0.2"
  }
}
```

  `@typescript/typescript6` ships the binary `tsc6` and re-exports the 6.0 API.
- 7.0.2 already carries a preview of the new API under `typescript/unstable/sync`, `/unstable/async`, `/unstable/ast` (works, but the name says what it is). The 7.1 names below do not exist in 7.0.2: `import("typescript/sync")` fails with `ERR_PACKAGE_PATH_NOT_EXPORTED` **[tested]**.
- **7.1** makes it official with split entry points: `typescript/sync`, `typescript/async`, `typescript/fs`, `typescript/ast`, `typescript/ast/is`, `typescript/ast/factory`. It is a new API, not the old one. `API` and `Program` objects hold memory on the Go side and must be released: declare them with `using`, or call `api.close()` / `program.dispose()`.

## Linting on TypeScript 7

- Type-aware eslint (`typescript-eslint`) cannot drive 7.0. Do not add it to a project that pins `typescript@7`, unless you also add the 6.0 alias above. Upstream is working on 7.1 support.
- **oxlint + oxlint-tsgolint** runs the typed rule set natively on the Go compiler. Tested here: `oxlint@1.93.0` with `oxlint-tsgolint@7.0.2006`, 12 typed rules over a 372-file, 62.7k-line codebase in 1.0 s **[tested]**.
  - Run `oxlint --type-aware -c .oxlintrc.json --format json <paths>`. `tsgolint` must be on `PATH` or in the project's `node_modules`.
  - Rule names keep the typescript-eslint spelling: `typescript/no-floating-promises`, `typescript/no-misused-promises`, `typescript/await-thenable`, `typescript/switch-exhaustiveness-check`, `typescript/no-non-null-assertion`, `typescript/no-explicit-any`, `typescript/ban-ts-comment`, `typescript/no-unsafe-type-assertion`, `typescript/no-unnecessary-condition`.
  - Put rules in the config file, not on the command line: a misspelled `-D typescript/xyz` is silently ignored, while the same typo in `.oxlintrc.json` fails with `Rule 'xyz' not found in plugin 'typescript'` **[tested]**.
  - An explicit `"rules"` map does not switch the defaults off: with `"categories": {}` oxlint still ran 93 rules (the `correctness` set, e.g. `eslint/no-debugger`). To run only the listed rules, set every category to `"off"` (`correctness`, `suspicious`, `pedantic`, `perf`, `style`, `restriction`, `nursery`); then exactly the listed rules run **[tested]**.
  - A config that fails to parse exits 1 and prints plain text on stdout instead of JSON, the same exit code as "findings found". Tell them apart by parsing the output **[tested]**.

## Running and testing .ts without a build

- Current Node runs `.ts` files directly: `node file.ts` works with no flag on v22.23.1 **[tested]**. `--experimental-strip-types` is only needed on older Node 22 releases.
- Node only strips types. `enum` and constructor parameter properties fail at runtime with `ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX`; turn on `erasableSyntaxOnly` so `tsc` reports it first **[tested]**.
- `node --test` discovers `*.test.ts` and runs them the same way, so a test runner dependency is optional **[tested]**.
- Relative imports must name the real file: `./k.ts` runs, `./k.js` and `./k` fail with `ERR_MODULE_NOT_FOUND` **[tested]**. For emitted output use `rewriteRelativeImportExtensions`.

## New compiler flags

- `--checkers N` — number of type-check workers, default 4. More is faster on big projects and uses more memory. In rare cases a different count changes results, so pin one value across machines if output must match.
- `--builders N` — parallel project-reference builds under `--build`. Multiplies with `--checkers`.
- `--singleThreaded` — no parallelism; for debugging and small CI runners.
- `--watch` is rebuilt on a Go port of the Parcel watcher.

## Language and library additions

- 6.0: methods that never use `this` no longer block generic inference (method syntax now infers like arrow functions) **[tested]**.
- 6.0: `#/` subpath imports (`"imports": { "#/*": "./dist/*" }`) under `nodenext` and `bundler`. From the release notes; a quick attempt here did not resolve, so check it before relying on it.
- 6.0: `--moduleResolution bundler` may be combined with `--module commonjs`.
- 6.0: `es2025` target and lib. `RegExp.escape`, `Promise.try` and the Set methods (`union` etc.) live there **[tested]**, as do the iterator helpers. With `lib: ["es2024"]`, `RegExp.escape` is TS2550.
- 6.0: `Temporal` types in `esnext` **[tested]**.
- 6.0: `Map`/`WeakMap` `getOrInsert` and `getOrInsertComputed` in `esnext` **[tested]**; they move to `es2026` in 7.1.
- 7.0: template literal inference takes whole Unicode code points: ``"😀abc" extends `${infer H}${infer T}` `` gives `H = "😀"`, not half a surrogate pair **[tested]**.
- 7.0: with `skipLibCheck`, conflicting declarations now report at every site, so a project can show errors that 6.0 hid.
- 7.1 beta: `--target es2026` (`Math.sumPrecise`, `Iterator.concat`, `Error.isError`, `Array.fromAsync`, `Uint8Array` base64/hex, `JSON.rawJSON`); `import source x from "./m.wasm"`; import attributes in ambient module declarations; stricter checks on distributed type parameters in conditional types (can surface new errors).

## JavaScript files checked by tsc (`checkJs`)

7.0 dropped the Closure-style JSDoc dialect. In `.js` files: no `function(string): void` types (write `(s: string) => void`), no standalone `?` type, no postfix `!`, `@enum` and `@class`/`@constructor` have no special meaning, values cannot be used as types (write `typeof value`), and `@typedef` needs its name inside the tag. `.d.ts` output generated from `.js` input changed substantially.

## Editors

The language server speaks LSP. VS Code uses the "TypeScript (Native Preview)" extension until support lands in VS Code itself. Projects that need language-server plugins (Vue, Svelte, Astro, MDX, Angular templates) stay on 6.0 in the editor for now; 7.1 adds content mappers aimed at this.
