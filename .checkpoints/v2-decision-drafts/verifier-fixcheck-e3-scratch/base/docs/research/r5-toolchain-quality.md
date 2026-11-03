# R5: Toolchain and quality

- **Topic:** R5, toolchain and quality (`docs/V2_PHASE0_BRIEF.md` §4, row R5).
- **Author agents:** test-engineer (Opus 5.5, effort high), the topic's owner (wave 2 of
  `.checkpoints/v2-research/plan.md`): questions 1–4 and 7. infra-engineer (Opus 5.5, effort high), the helper:
  questions 5 and 6, folded in unchanged (their ids and words are the helper's; where they meet the owner's parts, the
  owner's open questions R5-XQ-OQ1–OQ3 say so). Notes with every source summary and its exact quotes:
  `.checkpoints/v2-research/test-engineer-r5.md` and `.checkpoints/v2-research/infra-engineer-r5-ci.md`; one draft per
  question: `.checkpoints/v2-research/test-engineer-r5-<id>-draft.md` and `infra-engineer-r5-ci-draft.md`.
- **Date:** 2026-10-28 (every source below was read that day, except the Prettier README's re-read for R5-TSLINT-F22 on
  2026-10-29); assembled 2026-10-29; spot-checked and fixed 2026-10-29.
- **Status:** spot-checked (61 of 115 key claims). The verifier's table, its critical read, the owner's fixes and the
  second pass on them (2026-10-29: closed, nits only) are in "Verifier spot-check" below.
- Glossary and cross-topic resolutions applied 2026-10-30 (see [GLOSSARY.md](GLOSSARY.md) and part 7).
- **The brief's questions for this topic (quoted, `docs/V2_PHASE0_BRIEF.md` §4, row R5):** "Repository layout (a Cargo
  workspace plus TypeScript with Vite); strict TypeScript and lint rules; clippy and rustfmt; tests (cargo test,
  property tests, Vitest, Playwright, golden tests against legacy); CI with Rust caching; **preview links per PR**; the
  gate v2 that replaces today's gate"

| # | Brief question | Ids | Author | Draft (`.checkpoints/v2-research/`) |
|---|---|---|---|---|
| 1 | Repository layout (a Cargo workspace plus TypeScript with Vite) | R5-LAYOUT-F1–F15, REC1–REC6, OQ1–OQ5 | test-engineer | `test-engineer-r5-layout-draft.md` |
| 2 | strict TypeScript and lint rules | R5-TSLINT-F1–F25, REC1–REC4, OQ1–OQ6 | test-engineer | `test-engineer-r5-tslint-draft.md` |
| 3 | clippy and rustfmt | R5-RUSTLINT-F1–F15, REC1–REC6, OQ1–OQ5 | test-engineer | `test-engineer-r5-rustlint-draft.md` |
| 4 | tests (cargo test, property tests, Vitest, Playwright, golden tests against legacy) | R5-TEST-F1–F14, REC1–REC6, OQ1–OQ8 | test-engineer | `test-engineer-r5-test-draft.md` |
| 5 | CI with Rust caching | R5-CI-F1–F24, REC1–REC5, OQ1–OQ7 | infra-engineer | `infra-engineer-r5-ci-draft.md` (section 1) |
| 6 | **[preview](GLOSSARY.md#preview) links per PR** | R5-PREV-F1–F20, REC1–REC5, OQ1–OQ6 | infra-engineer | `infra-engineer-r5-ci-draft.md` (section 2) |
| 7 | the gate v2 that replaces today's gate | R5-GATE-F1–F4 (plus today's gate as legacy facts), REC1–REC4, OQ1–OQ5 | test-engineer | `test-engineer-r5-gate-draft.md` |
| – | Where the helper's parts and the owner's parts meet | – | test-engineer | R5-XQ-OQ1–OQ3 |

**Totals:** 117 findings (owner 73, helper 44), 36 recommendations (owner 26, helper 10), 63 open questions (owner 29,
helper 13, cross-part 3, shared 4 and cross-topic 14 added 2026-10-30); 108 sources read (105 official, 2 shipped game, 1 other; owner 68, helper 44, 4 read by
both), 39 rows of sources not read (the Sources part; a row can group several search leads).

**How to read this file.**
- Certainty marks: "confirmed in the source" means the source's own words, quoted in the finding, say it (the fetch
  tool returns at most about 125 characters verbatim, so long passages are quoted as fragments); "inferred" means my
  reading, a consequence I draw, or a point seen only in the fetch tool's summary; "confirmed by absence" means the page
  was checked for the point and does not contain it; "unverified" means no source read settles it.
- Tiers: official (specifications, the platform's or tool's own documentation, README, release pages, issue tracker and
  source code), shipped game (a shipped game's or a shipping vendor's own library, as R2 defines it), other (expert
  articles: Glenn Fiedler's articles are tiered "other", as R2 does, R2-XU-OQ16).
- clippy's lint list (clippy.rust-lang.org, where rust-lang.github.io/rust-clippy now redirects) is outside the allow
  list, so each clippy lint was read in clippy's own source on raw.githubusercontent.com (master, 2026-10-28).
- Legacy ids (TL1, KT8, CS5, …) are rows of `docs/LEGACY_INVENTORY.md`; their marks are working recommendations (the
  Advisor's R7-MARKS (a)). Today's gate is cited by file and line (`tools/gate.mjs`, `tools/README.md`,
  `.github/workflows/gate.yml`) and by `docs/reports/2026-10-14-gate-check-5.md`.
- Wave 1 and wave 2 ids (R1-…, R2-…, R3-…, R4-…) are cited, not restated as findings of this file.
- Pass marks and budgets are proposed; targets come only from VISION_V2 §3, §4 and §6. Nothing here is decided: the
  decision draft ADR-0002 (code structure, build and toolchain) decides, and Mustafa approves.
- Each recommendation's "Serves" line names the VISION_V2 §6 item it serves, or says "none" for toolchain and process
  work that adds no engine feature.

## TL;DR
- **Layout and lint:** one repo with Cargo and npm workspaces; exact pins (Rust 1.99.1, TypeScript 6.0.x: no typed lint
  for 7 yet); dependency rules as checks; strict tsconfig, `strict-type-checked`, Prettier, clippy `-D warnings` plus
  sim bans (std `HashMap`, clocks, std float maths). **Tests:** fixed seeds, committed legacy goldens, per-tick hashes
  on native, Node, three engines and real Safari; R2's P0–P4 and B1 net sims; bot soaks up to [the largest mode's N](GLOSSARY.md#the-largest-mode-and-its-n); no retries. **CI (helper):**
  cached from `main`, one always-run required check, secret-free previews. **Gate v2:** one command, G0–G12, `--prove`.

## Findings

### Question 1: repository layout (R5-LAYOUT)


- **R5-LAYOUT-F1. A Cargo workspace has one lockfile and one build folder for all its crates.** Source:
  https://doc.rust-lang.org/cargo/reference/workspaces.html (Cargo Book, "Workspaces"; no page date) · official · read
  2026-10-28 · confirmed in the source: "All packages share a common `Cargo.lock` file which resides in the _workspace
  root_." and "All packages share a common [output directory], which defaults to a directory named `target` in the
  _workspace root_."
- **R5-LAYOUT-F2. Crates can inherit package keys and dependency versions from the workspace.** Source: same page,
  "The `package` table" and "The `dependencies` table" · official · read 2026-10-28 · confirmed in the source: "These
  keys can be inherited by defining them in the member package with `{key}.workspace = true`." (both tables "Requires
  1.64+").
- **R5-LAYOUT-F3. A root manifest without a package (a virtual manifest) must name its resolver; edition 2024's
  resolver "3" prefers dependency versions that fit our own Rust version.** Sources: the workspaces page above and
  https://doc.rust-lang.org/cargo/reference/resolver.html ("Resolver versions") · official · read 2026-10-28 · confirmed
  in the source: "a `Cargo.toml` file can be created with a `[workspace]` section but without a `[package]` section. This
  is called a _virtual manifest_."; `resolver` "must be set explicitly ... as they have no `package.edition` to infer it
  from"; `"3"` is the "`edition = "2024"` default, requires Rust 1.84+" and changes "the default for
  `resolver.incompatible-rust-versions` from `allow` to `fallback`"; "With the `fallback` setting, the resolver will
  prefer packages with a Rust version that is less than or equal to your own Rust version."
- **R5-LAYOUT-F4. Cargo builds a shared dependency once with the union of every feature any member asks for.** Source:
  https://doc.rust-lang.org/cargo/reference/features.html ("Feature unification") · official · read 2026-10-28 ·
  confirmed in the source: "When a dependency is used by multiple packages, Cargo will use the union of all features
  enabled on that dependency when building it." and "features should be _additive_". Resolver 2 and later exclude some
  cases: "Features enabled on platform-specific dependencies for target architectures not currently being built are
  ignored." and "Dev-dependencies do not activate features unless building a Cargo target that needs them". What this
  means for us (inferred): if the server crate turns on a maths or SIMD feature of a dependency that the sim also uses,
  the native sim the server runs is built differently from the Wasm sim, which R1-DET-REC2 and R1-RR-OQ1 treat as a
  determinism risk.
- **R5-LAYOUT-F5. Rust's current stable release is 1.99.1 (15 October 2026); some workspace behaviour depends on it.**
  Sources: https://blog.rust-lang.org/releases/ ("Announcing Rust 1.99.1", Oct. 15, 2026; 1.99.0 Oct. 1, 2026) and the
  workspaces page ("`default-features` declared on a package dependency overrides the value set here in Edition 2024
  packages (requires Rust 1.99+)") · official · read 2026-10-28 · confirmed in the source (titles, dates and quote).
- **R5-LAYOUT-F6. A `rust-toolchain.toml` file pins the Rust channel, its components and its targets for everyone who
  builds in the folder.** Source: https://rust-lang.github.io/rustup/overrides.html ("The toolchain file"; no version) ·
  official · read 2026-10-28 · confirmed in the source: "The `[toolchain]` section is mandatory, and at least one
  property must be specified."; the example names `channel`, `components = [ "rustfmt", ... ]`, `targets = [
  "wasm32-unknown-unknown", ... ]`, `profile = "minimal"`; "The components listed here are additive with the current
  profile." Precedence: "The toolchain is chosen in the order listed above, using the first one that is specified."
  (a `+toolchain` shorthand and `RUSTUP_TOOLCHAIN` come before the file).
- **R5-LAYOUT-F7. ts-rs writes its TypeScript files when `cargo test` runs, into a folder set by one variable.** Source:
  https://docs.rs/ts-rs/latest/ts_rs/ (ts-rs 12.0.1) · official (crate docs) · read 2026-10-28 · confirmed in the source:
  "When running `cargo test` or `cargo test export_bindings`, the following TypeScript type will be exported"; "When
  using `#[ts(export)]` on a type, `ts-rs` generates a test which writes the bindings for it to disk."; `TS_RS_EXPORT_DIR`
  is the "Base directory into which bindings will be exported" (default `./bindings`), settable in
  `.cargo/config.toml` as `[env] TS_RS_EXPORT_DIR = { value = "bindings", relative = true }`.
- **R5-LAYOUT-F8. npm workspaces: packages listed in the root `package.json` are linked into the root `node_modules`,
  and a command can run in one package or all.** Source: https://docs.npmjs.com/cli/v11/using-npm/workspaces (npm CLI
  11.21.1; the page header marks this version "(Legacy)") · official · read 2026-10-28 · confirmed in the source:
  "Workspaces are usually defined via the `workspaces` property of the" `package.json`; "the folder `packages/a` will
  get symlinked to the `node_modules` folder of the current working dir."; `--workspace` "can be used to run the given
  command in the context of that specific workspace". A single root lockfile appears only in the page's examples
  (inferred). The "(Legacy)" mark suggests a newer npm major exists; I did not read its page (unverified).
- **R5-LAYOUT-F9. Vite 8.3.4 builds several HTML pages as separate entry points.** Source:
  https://vite.dev/guide/build ("Multi-Page App"; Vite v8.3.4) · official · read 2026-10-28 · confirmed in the source:
  "During build, all you need to do is to specify multiple `.html` files as entry points:". The default build targets
  Baseline "Widely Available" (the page lists Chrome and Edge 111+, Firefox 114+, Safari 16.4+; list from the fetch
  summary, inferred).
- **R5-LAYOUT-F10. Vite only strips types; type checking is a separate `tsc` run, and `isolatedModules` must be on.**
  Source: https://vite.dev/guide/features ("TypeScript"; v8.3.4) · official · read 2026-10-28 · confirmed in the source:
  "Note that Vite only performs transpilation on `.ts` files and does **NOT** perform type checking."; for production,
  "`tsc --noEmit` in addition to Vite's build command"; "You must set `"isolatedModules": true` in your `tsconfig.json`".
- **R5-LAYOUT-F11. Vite's recommended way to load a Web Worker is `new Worker(new URL(...), import.meta.url)`, and a
  `.wasm` file can be imported with `?init`.** Source: same page ("Web Workers", "WebAssembly") · official · read
  2026-10-28 · confirmed in the source: the constructor form "is the **recommended** way to create workers."; with
  `?init`, "The default export will be an initialization function that returns a Promise of the" `WebAssembly.Instance`.
  The page does not mention wasm-bindgen's output (confirmed by absence).
- **R5-LAYOUT-F12. Vite's development server and its `vite preview` server each take their own response headers.**
  Sources: https://vite.dev/config/server-options (`server.headers`) and https://vite.dev/config/preview-options
  (`preview.headers`; v8.3.4) · official · read 2026-10-28 · confirmed in the source: both are described as "Specify
  server response headers."; "Unless noted, the options in this section are only applied to preview." This is where the
  COOP and COEP headers of R1-BND-REC5 (isolation in development and previews) would be set (inferred).
- **R5-LAYOUT-F13. TypeScript project references split a program into projects that `tsc -b` builds in order; the
  handbook does not say they block imports from an unreferenced project.** Source:
  https://www.typescriptlang.org/docs/handbook/project-references.html ("Last updated: Oct 27, 2026") · official · read
  2026-10-28 · confirmed in the source: "Referenced projects must have the new composite setting enabled."; `tsc -b`
  "behaves more like a build orchestrator than a simple compiler."; a benefit is to "enforce logical separation between
  components". Blocking unreferenced imports: not stated (confirmed by absence).
- **R5-LAYOUT-F14. cargo-deny can forbid a crate everywhere except as a direct dependency of named crates.** Source:
  https://embarkstudios.github.io/cargo-deny/checks/bans/cfg.html (no version shown) · official (the tool's docs) · read
  2026-10-28 · confirmed in the source: `deny` "Determines specific crates that are denied."; `wrappers` "allows specific
  crates to have a direct dependency on the banned crate". Whether a workspace crate can itself be the banned crate is not
  stated (unverified).
- **R5-LAYOUT-F15. Vite's build manifest lists, per chunk, its source file and the chunks it imports statically and
  dynamically.** Sources: https://vite.dev/guide/backend-integration and https://vite.dev/config/build-options
  (`build.manifest`, default `false`; v8.3.4) · official · read 2026-10-28 · confirmed in the source: `src` "The input
  file name of this chunk / asset if known"; `isEntry` "Whether this chunk or asset is an entry point"; `imports` "The
  list of statically imported chunks by this chunk."; `dynamicImports` "The list of dynamically imported chunks by this
  chunk."

#### Legacy facts used
- **TL4** (mark redesign): "the player build never loads editor code" (EDITOR_V2 §5) stays as a rule; in v2 it is an
  entry-point or import-graph check, not today's mode-guard heuristics.
- **TL7** (redesign): the layering rule grows: the Rust sim must not depend on the client, the client reaches the sim
  only through the Wasm boundary, the editor stays out of the player build; enforced by crate and package boundaries plus
  an import-graph check.
- **TL11** (drop, but kept as the legacy oracle until legacy retires): `tools/preview.js` is the natural runner for golden
  tests against legacy, so `legacy/` plus the tools that host it must keep running after the move.
- **TL17** (redesign): exact pins with a lockfile and a clean install; optional native dependencies kept out of CI.
- **KT13** (redesign): the netcode path rule today names `server/`, `shared/`, `net/`; its principles move to the Rust
  sim crate and server and the TypeScript net client.
- **KT19** (redesign): LF everywhere; the ignore list must add Rust and Wasm outputs (`target/`, wasm-pack's `pkg/`) and
  the TypeScript build and test caches, decided with this layout.
- **KT21**: only the legacy game's five files are named as moving to `legacy/` (CLAUDE.md, V2_PHASE0_BRIEF §0); where
  the legacy tools and docs go is not stated.
- Wave 1 cited, not restated: R1-BND-REC4 (generate every boundary type from Rust; pin the wasm-bindgen CLI to the
  library), R1-BND-F2 (CLI and library must match since 0.2.128), R1-DATA-REC2 (one content crate, depends on nothing
  above it), R1-DATA-REC3 (schema generated from the Rust types; CI fails when the committed schema differs),
  R1-BND-REC5 (isolation in development and previews), R2-TRN-REC4 (a thin transport interface outside the simulation;
  mark 3: the sim crate has no dependency on transport crates).

### Question 2: strict TypeScript and lint rules (R5-TSLINT)


- **R5-TSLINT-F1. TypeScript 7.0.2 (the native port) is the latest release; 6.0.3 is the latest 6.x.** Source:
  https://github.com/microsoft/TypeScript/releases · official · read 2026-10-28 · confirmed in the source: "TypeScript
  7.0.2" (20 Aug, marked "Latest"), "TypeScript 7.1 Beta" (20 Oct), "TypeScript 6.0.3" (16 Apr), "TypeScript 6.0"
  (23 Mar). The headers carry no year; the asset timestamps say 2026 (inferred). The 7.0.2 entry links the
  typescript-go repository, where the tag "was originally released at".
- **R5-TSLINT-F2. TypeScript 6.0 turned `strict` on by default and changed other defaults.** Source:
  https://www.typescriptlang.org/docs/handbook/release-notes/typescript-6-0.html ("Last updated: Oct 27, 2026") ·
  official · read 2026-10-28 · confirmed in the source: "**`strict` is now `true` by default**"; "**`module` defaults to
  `esnext`**"; the target is "the most recent supported ECMAScript spec version (effectively a floating target)";
  "**`noUncheckedSideEffectImports` is now `true` by default**"; "the default `types` value will be `[]` (an empty
  array)."
- **R5-TSLINT-F3. TypeScript 6.0 is a transition release: options it deprecates are gone in 7.0.** Source: same page ·
  official · read 2026-10-28 · confirmed in the source: "TypeScript 6.0 arrives as a significant transition release,
  designed to prepare developers for TypeScript 7.0, the upcoming native port"; "TypeScript 7.0 _will not_ support any of
  these deprecated options."; `--stableTypeOrdering` "is only intended to help diagnose differences between 6.0 and 7.0".
- **R5-TSLINT-F4. typescript-eslint 8.72.1 supports TypeScript below 6.1 and ESLint 8.57, 9 and 10.** Source:
  https://typescript-eslint.io/users/dependency-versions (site header v8.72.1) · official · read 2026-10-28 · confirmed
  in the source: "The version range of TypeScript currently supported is `>=4.8.4 <6.1.0`."; "The version range of
  ESLint currently supported is `^8.57.0 || ^9.0.0 || ^10.0.0`."
- **R5-TSLINT-F5. typescript-eslint does not work with TypeScript 7 yet, because 7.0 has no compiler API.** Source:
  https://github.com/typescript-eslint/typescript-eslint/issues/12518 ("TypeScript 7.0.2 Support", opened and closed as
  not planned on Jul 8, 2026; marked blocked by #10940 on Aug 10, 2026) · official (the project's tracker) · read
  2026-10-28 · confirmed in the source (maintainer kirkwaiblinger): "typescript-eslint isn't compatible with TS 7 at this
  time, because there is no TS 7 API at this time." and "There is nothing we can do about this until TS 7 provides an
  API."
- **R5-TSLINT-F6. `strict` is a family of checks.** Source: https://www.typescriptlang.org/tsconfig/ (`strict`; no
  date) · official · read 2026-10-28 · confirmed in the source: "The `strict` flag enables a wide range of type checking
  behavior that results in stronger guarantees of program correctness." The related list names alwaysStrict,
  strictNullChecks, strictBindCallApply, strictBuiltinIteratorReturn, strictFunctionTypes, strictPropertyInitialization,
  noImplicitAny, noImplicitThis and useUnknownInCatchVariables (list from the fetch summary, inferred).
- **R5-TSLINT-F7. `noUncheckedIndexedAccess` makes every index read possibly `undefined`.** Source: same page ·
  official · read 2026-10-28 · confirmed in the source: "Turning on `noUncheckedIndexedAccess` will add `undefined` to
  any un-declared field in the type." What it catches for us (inferred): reading slot `n` of a typed array view or of
  a per-team table without a bounds check, the shape of the legacy `SOLDIERS[r.id]` reads.
- **R5-TSLINT-F8. `exactOptionalPropertyTypes` separates "missing" from "set to undefined".** Source: same page ·
  official · read 2026-10-28 · confirmed in the source: "With exactOptionalPropertyTypes enabled, TypeScript applies
  stricter rules around how it handles properties on `type` or `interfaces` which have a `?` prefix."
- **R5-TSLINT-F9. `noImplicitOverride` requires the `override` keyword.** Source: same page · official · read 2026-10-28
  · confirmed in the source: "ensuring that functions which override include the keyword `override`."
- **R5-TSLINT-F10. `noFallthroughCasesInSwitch` reports fall-through cases.** Source: same page · official · read
  2026-10-28 · confirmed in the source: "Report errors for fallthrough cases in switch statements."
- **R5-TSLINT-F11. `noImplicitReturns` checks that every path returns.** Source: same page · official · read
  2026-10-28 · confirmed in the source: "When enabled, TypeScript will check all code paths in a function to ensure they
  return a value."
- **R5-TSLINT-F12. `erasableSyntaxOnly` rejects TypeScript-only constructs that need code generation.** Source: same
  page (read at offset 100000) · official · read 2026-10-28 · confirmed in the source: "The `--erasableSyntaxOnly` flag
  will cause TypeScript to error on most TypeScript-specific constructs". Which constructs (enums, namespaces, parameter
  properties) is from general knowledge, not the read text (unverified).
- **R5-TSLINT-F13. typescript-eslint's shared configs, and which are stable.** Source:
  https://typescript-eslint.io/users/configs · official · read 2026-10-28 · confirmed in the source: `recommended`
  "Recommended rules for code correctness that you can drop in without additional configuration."; `strict` "Contains all
  of `recommended`, as well as additional strict rules that can also catch bugs."; `stylistic` "Rules considered to be
  best practice for modern TypeScript codebases, but that do not impact program logic."; "With the exception of `all`,
  `strict`, and `strict-type-checked`, all configurations are considered "stable"."; `strict` is "not considered
  "stable" under Semantic Versioning (semver)."; "We do not recommend TypeScript projects extend from
  `plugin:@typescript-eslint/all`."; `strict` suits a project "only if a nontrivial percentage" … "of its developers are
  highly proficient in TypeScript." (the verifier's words, 2026-10-29).
- **R5-TSLINT-F14. ESLint's `no-restricted-imports` forbids imports by path pattern, with a message.** Source:
  https://eslint.org/docs/latest/rules/no-restricted-imports (latest is v10.13.0 in the version switcher) · official ·
  read 2026-10-28 · confirmed in the source: "Disallow specified modules when loaded by `import`"; `patterns` "allows
  you to specify multiple modules to restrict using `gitignore`-style patterns or regular expressions."; `message` "used
  to specify a custom message."
- **R5-TSLINT-F15. Typed linting asks TypeScript's service for types and costs a build of the project.** Source:
  https://typescript-eslint.io/getting-started/typed-linting · official · read 2026-10-28 · confirmed in the source:
  `projectService: true` "indicates to ask TypeScript's type checking service for each source file's type
  information."; the cost: "incur the performance penalty of asking TypeScript to do a build of your project before
  ESLint can do its linting."; "For small projects this takes a negligible amount of time (a few seconds or less)".
- **R5-TSLINT-F16. `no-floating-promises` reports promises nobody handles, and `recommended-type-checked` turns it on.**
  Source: https://typescript-eslint.io/rules/no-floating-promises/ · official · read 2026-10-28 · confirmed in the
  source: "Require Promise-like statements to be handled appropriately."; it also "reports when an Array containing
  Promises is created and not properly handled."; "Extending "plugin:@typescript-eslint/recommended-type-checked" in an
  ESLint configuration enables this rule." What it catches for us (inferred): an unawaited WebTransport write, a
  Worker start-up promise or a `KVStore`-style async save whose error vanishes.
- **R5-TSLINT-F17. `switch-exhaustiveness-check` requires every member of a union to have a case.** Source:
  https://typescript-eslint.io/rules/switch-exhaustiveness-check/ · official · read 2026-10-28 · confirmed in the source:
  "Require switch-case statements to be exhaustive."; with `considerDefaultExhaustiveForUnions` (default `false`) off, a
  `default` does not count: "If set to true, a `switch` statement over a union type that includes a `default` case is
  considered exhaustive." Whether a shared config includes it: not stated on the page (confirmed by absence). What it
  catches for us (inferred): a new message kind or entity kind from the generated types that the client does not handle.
- **R5-TSLINT-F18. typescript-eslint advises against ESLint's `no-undef` in TypeScript.** Source:
  https://typescript-eslint.io/troubleshooting/faqs/eslint/ · official · read 2026-10-28 · confirmed in the source: "We
  strongly recommend that you do not use the `no-undef` lint rule on TypeScript projects."; "The checks it provides are
  already provided by TypeScript without the need for configuration."
- **R5-TSLINT-F19. ESLint's `no-restricted-globals` forbids chosen global names.** Source:
  https://eslint.org/docs/latest/rules/no-restricted-globals · official · read 2026-10-28 · confirmed in the source:
  "Disallow specified global variables."; "early Internet Explorer versions exposed the current DOM event as a global
  variable `event`, but using this variable has been considered as a bad practice for a long time."
- **R5-TSLINT-F20. ESLint's flat config is one `eslint.config.*` file with `files` globs per block.** Source:
  https://eslint.org/docs/latest/use/configure/configuration-files (latest v10.13.0) · official · read 2026-10-28 ·
  confirmed in the source: "The ESLint configuration file may be named any of the following:" (`eslint.config.js`,
  `.mjs`, `.cjs`, `.ts`, `.mts`, `.cts`); `files` is "An array of glob patterns indicating the files that the
  configuration object should apply to."
- **R5-TSLINT-F21. ESLint recommends a formatter instead of its own formatting rules, which it deprecated.** Source:
  https://eslint.org/blog/2023/10/deprecating-formatting-rules/ (26 October 2023) · official · read 2026-10-28 ·
  confirmed in the source: "We recommend using a source code formatter instead of ESLint for formatting your code."; the
  rules "will not be removed until at least ESLint v10.0.0 (if not later)." The post names Prettier and dprint (fetch
  summary, inferred). ESLint 10 is now current (F14, F20), so that window has opened; whether 10.x removed the rules
  was not read (unverified).
- **R5-TSLINT-F22. Prettier formats TypeScript, JSON, Markdown and YAML.** Source: https://github.com/prettier/prettier
  (README; no version shown) · official (the project's own page) · read 2026-10-28 · confirmed in the source: "Prettier is
  an opinionated code formatter."; the language list reads "_JavaScript · TypeScript · Flow · JSX · JSON_", "_CSS ·
  SCSS · Less_", "_HTML · Vue · Angular_", "_GraphQL · Markdown · YAML_" (quoted on a re-read of the same page,
  2026-10-29).
- **R5-TSLINT-F23. Biome is a formatter and linter that claims 97 % Prettier compatibility.** Source:
  https://github.com/biomejs/biome (README; no version shown) · official (the project's own page) · read 2026-10-28 ·
  confirmed in the source: "Biome offers formatter and linter, usable via CLI and LSP."; its formatter "scores **97%
  compatibility with _Prettier_**"; "more than 500 rules". Type-aware rules: not stated (confirmed by absence).
- **R5-TSLINT-F24. `isolatedModules` warns about code a single-file transpiler cannot handle, and Vite requires it.**
  Sources: https://www.typescriptlang.org/tsconfig/ (offset 100000) and R5-LAYOUT-F10 · official · read 2026-10-28 ·
  confirmed in the source: "Setting the `isolatedModules` flag tells TypeScript to warn you if you write certain code".
- **R5-TSLINT-F25. `verbatimModuleSyntax` (TypeScript 5.0) is the option for how imports are kept or dropped.** Source:
  https://www.typescriptlang.org/tsconfig/ (offset 100000) · official · read 2026-10-28 · confirmed in the source only as
  "TypeScript 5.0 introduces a new option called `--verbatimModuleSyntax` to simplify the situation."; that it makes
  type-only imports explicit (`import type`) is inferred.

#### Legacy facts used
- **TL6** (drop): check 5b exists because classic scripts resolve names at run time; in v2 the type checker does it.
  The lesson carries: a reviewed allowlist of browser globals, so a typo that happens to be a window property (`name`,
  `status`, `event`) fails. Pin the tool versions (CI pins Node 22).
- **TL10 / KT9** (redesign): keep instant per-edit feedback through exit code 2; in v2 a fast `tsc` on the touched
  project, not `node --check`.
- **TL4, TL7** (redesign): the player build never loads editor code; the client reaches the sim only through the Wasm
  boundary (R5-LAYOUT-REC2).
- CLAUDE.md code rule: dynamic text through `textContent`, never `innerHTML` (security, carried to v2 as a lint target;
  the rule that enforces it was not read: OQ5).

### Question 3: clippy and rustfmt (R5-RUSTLINT)


- **R5-RUSTLINT-F1. Only clippy's `correctness` group is an error by default; `style` warns; `restriction` must never be
  turned on as a whole.** Source: https://doc.rust-lang.org/clippy/lints.html (Clippy book; no version) · official · read
  2026-10-28 · confirmed in the source: "The `clippy::correctness` group is the only lint group in Clippy whose lints are
  deny-by-default"; `style` is "the most opinionated warn-by-default group in Clippy"; for `restriction`: "It is **not**
  recommended to enable the whole group, but rather cherry-pick lints that are useful"; "Clippy will produce a warning if
  it finds a `#![warn(clippy::restriction)]` attribute in your code!"
- **R5-RUSTLINT-F2. `pedantic` is stricter and `nursery` holds lints still being worked on.** Source: same page ·
  official · read 2026-10-28 · confirmed in the source: "The `clippy::pedantic` group makes Clippy even more
  _pedantic_."; "The `clippy::nursery` group contains lints which are buggy or need more work."
- **R5-RUSTLINT-F3. `Cargo.toml` sets lint levels per tool, with a priority that orders groups and single lints; it
  applies only to our own packages.** Source: https://doc.rust-lang.org/cargo/reference/manifest.html ("The `[lints]`
  section") · official · read 2026-10-28 · confirmed in the source: "Override the default level of lints from different
  tools by assigning them to a new level in a table"; example `[lints.rust] unsafe_code = "forbid"`; "`priority` is a
  signed integer that controls which lints or lint groups override other lint groups:"; "lower (particularly negative)
  numbers have lower priority, being overridden by higher numbers"; "Cargo only applies these to the current package and
  not to dependencies."
- **R5-RUSTLINT-F4. A workspace defines lints once and each crate opts in.** Source:
  https://doc.rust-lang.org/cargo/reference/workspaces.html ("The `lints` table") · official · read 2026-10-28 · confirmed
  in the source: "The `workspace.lints` table is where you define lint configuration to be inherited by members of a
  workspace."; members write `[lints]` with `workspace = true`; "Respected as of 1.74".
- **R5-RUSTLINT-F5. clippy reads a `clippy.toml` found from the crate's own folder upward.** Source:
  https://doc.rust-lang.org/clippy/configuration.html · official · read 2026-10-28 · confirmed in the source: "a TOML file
  named `clippy.toml` or `.clippy.toml`"; the search starts at `CLIPPY_CONF_DIR`, then `CARGO_MANIFEST_DIR`, then "The
  current directory."; "Clippy will walk up the directory tree, searching each parent directory until it finds one".
  That a `clippy.toml` inside `crates/sim/` applies to `sim` alone follows from `CARGO_MANIFEST_DIR` coming first
  (inferred).
- **R5-RUSTLINT-F6. A `deny` level makes clippy exit with an error, which is what CI needs.** Source: same page ·
  official · read 2026-10-28 · confirmed in the source: "An error causes Clippy to exit with an error code, so is most
  useful in scripts used in CI/CD."
- **R5-RUSTLINT-F7. `disallowed_methods` bans named functions and methods, including std ones such as
  `std::time::Instant::now`.** Sources:
  https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/disallowed_items.rs (source) and
  https://doc.rust-lang.org/clippy/lint_configuration.html (`disallowed-methods`) · official · read 2026-10-28 · confirmed
  in the source: "Denies the configured methods and functions in clippy.toml" (group `style`,
  `#[clippy::version = "1.49.0"]`); the examples include `{ path = "std::time::Instant::now" }` and `{ path =
  "std::vec::Vec::leak", reason = "no leaking memory" }`; the configuration is "The list of disallowed methods, written as
  fully qualified paths." with optional `reason` and `replacement`. Whether a primitive's method (`f32::sin`) can be named
  is not shown (unverified).
- **R5-RUSTLINT-F8. `disallowed_types` bans named types, such as std collections.** Sources: the same two pages ·
  official · read 2026-10-28 · confirmed in the source: "Denies the configured types in clippy.toml." (`style`,
  `1.55.0`); examples `"std::collections::BTreeMap"` and `{ path = "std::net::TcpListener" }`; "The list of disallowed
  types, written as fully qualified paths."
- **R5-RUSTLINT-F9. `iter_over_hash_type` flags `HashMap` and `HashSet` in `for` loops, and only there.** Source:
  https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/iter_over_hash_type.rs · official
  (source) · read 2026-10-28 · confirmed in the source: "This is a restriction lint which prevents the use of hash types
  (i.e., `HashSet` and `HashMap`) in for loops."; "Because hash types are unordered, when iterated through such as in a
  `for` loop, the values are returned in an undefined order."; `#[clippy::version = "1.76.0"]`; the check starts with
  `if let Some(for_loop) = ForLoop::hir(expr)`, so an iterator chain such as `map.values().map(...).sum()` outside a
  `for` is not flagged (confirmed in the source lines).
- **R5-RUSTLINT-F10. Two nursery lints rewrite float expressions into `mul_add` and other methods.** Source:
  https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/floating_point_arithmetic/mod.rs ·
  official (source) · read 2026-10-28 · confirmed in the source: `suboptimal_flops` "Looks for floating-point expressions
  that can be expressed using built-in methods to improve both accuracy and performance." (its example turns `a * 2.0 +
  4.0` into `a.mul_add(2.0, 4.0)`); `imprecise_flops` "Looks for floating-point expressions that can be expressed using
  built-in methods to improve accuracy at the cost of performance." (`powf(1.0 / 3.0)` into `cbrt()`); both `nursery`,
  `1.43.0`. Consequence (inferred): applying either changes rounding, so it changes state hashes and golden values, and
  `cbrt`, `ln_1p` and `exp_m1` are std transcendental calls R1-DET-REC2 bans in the sim.
- **R5-RUSTLINT-F11. `unwrap_used` and `expect_used` are restriction lints; clippy can exempt tests.** Sources:
  https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/methods/mod.rs (read at offsets 0 and
  100000) and https://doc.rust-lang.org/clippy/lint_configuration.html · official · read 2026-10-28 · confirmed in the
  source: `unwrap_used` "Checks for `.unwrap()` or `.unwrap_err()` calls on `Result`s and `.unwrap()` call on
  `Option`s."; `expect_used` "Checks for `.expect()` or `.expect_err()` calls on `Result`s and `.expect()` call on
  `Option`s." (both `restriction`, `1.45.0`); `allow-unwrap-in-tests`: "Whether `unwrap` should be allowed in test
  functions or `#[cfg(test)]`" (default `false`), and the same for `expect`, `panic` and `indexing_slicing`.
- **R5-RUSTLINT-F12. `indexing_slicing` flags indexing that can panic.** Source:
  https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/indexing_slicing.rs · official
  (source) · read 2026-10-28 · confirmed in the source: "Checks for usage of indexing or slicing that may panic at
  runtime."; "To avoid implicit panics from indexing and slicing." (`restriction`, "pre 1.29.0").
- **R5-RUSTLINT-F13. Overflow checks are on in dev builds and off in release builds by default; a profile can be
  overridden per package.** Source: https://doc.rust-lang.org/cargo/reference/profiles.html · official · read 2026-10-28
  · confirmed in the source: "When overflow-checks are enabled, a panic will occur on overflow."; `overflow-checks = true`
  under `[profile.dev]`, `false` under `[profile.release]`; "Profile settings can be overridden for specific packages and
  build-time crates."; "Overrides cannot specify the `panic`, `lto`, or `rpath` settings." That `overflow-checks` may be
  set per package follows from that list (inferred).
- **R5-RUSTLINT-F14. rustfmt reads `rustfmt.toml`; its edition, style edition, width and line-ending options are stable;
  import grouping is not.** Source: https://raw.githubusercontent.com/rust-lang/rustfmt/master/Configurations.md
  (master; the rendered page at rust-lang.github.io/rustfmt returned only a template) · official · read 2026-10-28 ·
  confirmed in the source: "You can create a TOML file called `rustfmt.toml` or `.rustfmt.toml`, place it in the
  project"; `style_edition` "Controls the edition of the [Rust Style Guide] to use for formatting ([RFC 3338])" (default
  "2015", stable); `edition` "Specifies which edition is used by the parser." (default "2015", stable); `max_width`
  "Maximum width of each line" (100, stable); `newline_style` "Unix or Windows line endings" (default "Auto", stable);
  `imports_granularity` and `group_imports` are not stable (tracking issues #4991, #5083); `unstable_features` "Enable
  unstable features on the unstable channel."
- **R5-RUSTLINT-F15. clippy adds lints release by release.** Source: the `disallowed_items.rs` source above · official ·
  read 2026-10-28 · confirmed in the source: `disallowed_fields` carries `#[clippy::version = "1.95.0"]` while
  `disallowed_methods` carries `1.49.0`. Consequence (inferred): a toolchain upgrade can add warnings, so under
  `-D warnings` the toolchain pin (R5-LAYOUT-F6) is what keeps the gate's verdict stable.

#### Legacy facts used
- **KT13** (redesign): the netcode rule's principles for the Rust core: no wall-clock reads in the tick, seeded RNG,
  deterministic iteration (no HashMap order); "identical in Wasm and native".
- **TL13**: the legacy game reads `performance.now()` / `Date.now()` in 11 places (STATUS:111), which is why same-seed
  legacy matches differ; the lint must stop the same thing in the sim.
- **CS1**: the legacy loop is variable-step, though the docs called it deterministic (code contradicts docs).
- Wave 1 cited, not restated: R1-DET-REC2 (float rules; R5 picks the lint or build check that fails on a std
  transcendental call, `algebraic_*` or `relaxed-simd`), R1-ECS-REC2 mark 2 (a CI check finds zero std HashMap or
  HashSet iteration in the sim's outcome code; tooling is R5's; since 2026-10-30 the mark is [the hash-map ban](GLOSSARY.md#the-hash-map-ban-in-the-sim-crate)), R1-RR-OQ3 (panics reachable from content and input),
  R1-RR-OQ4 (overflow differs between profiles; ADR-0002 with R5), R1-RR-OQ6 (unsafe for the output block),
  R2-TRN-REC4 (decoders never panic).

### Question 4: tests (R5-TEST)


- **R5-TEST-F1. `cargo test` runs a crate's tests on several threads of one process; doc tests run too.** Source:
  https://doc.rust-lang.org/cargo/commands/cargo-test.html · official · read 2026-10-28 · confirmed in the source: "The
  executable automatically runs all functions annotated with the `#[test]` attribute in multiple threads."; `--jobs`
  "does not affect how many threads are used when running the tests."; "Documentation tests are also run by default,
  which is handled by `rustdoc`."; `--no-fail-fast`: "Run all tests regardless of failure." Consequence (inferred): a test
  that reads a clock or shares global state can pass alone and fail beside others.
- **R5-TEST-F2. proptest 1.12.0 runs 256 cases by default, saves failing cases to `proptest-regressions` files, and
  takes its seed from an environment variable.** Sources: https://docs.rs/proptest/latest/proptest/ and
  https://docs.rs/proptest/latest/proptest/test_runner/struct.Config.html (1.12.0) · official (crate docs) · read
  2026-10-28 · confirmed in the source: "Hypothesis-like property-based testing and shrinking."; `cases` "The number of
  successful test cases that must execute for the test as a whole to pass." (default 256, "can be overridden by setting
  the `PROPTEST_CASES` environment variable."); `failure_persistence` "Indicates whether and how to persist failed test
  results." (default `FileFailurePersistence::SourceParallel("proptest-regressions")`); the seed is "Set by using the
  PROPTEST_RNG_SEED environment variable."; "If the environment variable is undefined, a random seed is generated" (the
  verifier's words, 2026-10-29).
- **R5-TEST-F3. insta 1.49.0 keeps snapshots as `.snap` files beside the tests, never writes them in CI, and can round
  floats.** Source: https://docs.rs/insta/latest/insta/ · official · read 2026-10-28 · confirmed in the source: snapshot
  tests "assert values against a reference value (the snapshot)"; stored "Right next to your test in a folder called
  `snapshots` as individual" `.snap` files; the default update mode is "`no` for CI environments or `new` otherwise", and
  `no` "does not write to snapshot files at all; just runs tests"; `rounded_redaction` "Creates a redaction that rounds
  floating point numbers to a given number of decimal places."
- **R5-TEST-F4. cargo-fuzz drives libFuzzer, needs a nightly compiler, and runs only on Unix-like x86-64 and AArch64.**
  Source: https://github.com/rust-fuzz/cargo-fuzz (README; no version shown) · official (the project) · read 2026-10-28 ·
  confirmed in the source: "A `cargo` subcommand for fuzzing with `libFuzzer`! Easy to use!"; it "needs a nightly
  compiler since it uses some unstable command-line flags."; libFuzzer "only works on x86-64 and Aarch64, and only on
  Unix-like operating systems (not Windows)."
- **R5-TEST-F5. wasm-bindgen-test runs Rust tests compiled to Wasm under `cargo test`, in Node.js by default or in
  Chrome, Firefox or Safari through their WebDriver.** Sources:
  https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html and .../browsers.html (no version shown; the
  wasm-bindgen project has since moved organisation, R1-BND-F2, so this guide may be older than the crate) · official ·
  read 2026-10-28 · confirmed in the source: "an experimental test harness for Rust programs compiled to Wasm"; "the
  usual `cargo test` command but with an explicit wasm target."; "By default tests run on Node.js."; for
  `wasm_bindgen_test_configure!(run_in_browser);`, "Note that this will ignore any environment variable set."; drivers
  through `GECKODRIVER`, `CHROMEDRIVER` and `SAFARIDRIVER`.
- **R5-TEST-F6. Vitest 5.0.5 runs tests in a real browser through Playwright (Chromium, Firefox, WebKit) or WebdriverIO
  (Chrome, Edge, Firefox, Safari).** Source: https://vitest.dev/guide/browser/ (v5.0.5) · official · read 2026-10-28 ·
  confirmed in the source: Browser Mode "allows you to run your tests in the browser natively"; "Vitest always requires a
  provider to be defined"; "`playwright` supports these browsers:" Firefox, WebKit, Chromium; "`webdriverio` supports
  these browsers:" Firefox, Chrome, Edge, Safari; "However, to run tests in CI you need to install either `playwright` or
  `webdriverio`." Stable or experimental: not stated (confirmed by absence).
- **R5-TEST-F7. Vitest retries nothing by default, and its fake timers replace `setTimeout` and `setInterval`.**
  Sources: https://vitest.dev/config/retry and https://vitest.dev/guide/mocking/timers · official · read 2026-10-28 ·
  confirmed in the source: `retry` "Retry the test specific number of times if it fails." (Default `0`); fake timers
  "mock calls to `setTimeout` and `setInterval`". Whether they fake `performance.now` is not on the page (confirmed by
  absence).
- **R5-TEST-F8. Playwright's WebKit is not Safari, and Playwright cannot drive real (branded) Safari; for the
  closest-to-Safari result it points to its own WebKit on macOS.** Source: https://playwright.dev/docs/browsers (no
  version shown) · official · read 2026-10-28 · confirmed in the source: "Playwright can run tests on Chromium, WebKit
  and Firefox browsers."; "Playwright's WebKit is derived from the latest WebKit main branch sources," "often before
  these updates are incorporated into Apple Safari"; "Playwright doesn't work with the branded version of Safari since
  it relies on patches."; "While running WebKit on Linux CI is usually the most affordable option," "for the
  closest-to-Safari experience you should run WebKit on mac, for example if you do video playback." (the sentence's end
  in the verifier's words, 2026-10-29); likewise "Playwright doesn't work with the branded version of Firefox since it
  relies on patches." So real Safari is reached only through its WebDriver (F5, F6), not through Playwright.
- **R5-TEST-F9. Playwright retries nothing by default, labels a test that passes only on retry "flaky", and can fail
  the run on any flaky test.** Sources: https://playwright.dev/docs/test-retries and
  https://playwright.dev/docs/api/class-testconfig · official · read 2026-10-28 · confirmed in the source: "By default
  failing tests are not retried."; flaky means "tests that failed on the first run, but passed when retried.";
  `failOnFlakyTests` (added in v1.52) "Whether to exit with an error if any tests are marked as flaky. Useful on CI.";
  `forbidOnly` "Whether to exit with an error if any tests or groups are marked as test.only() or test.describe.only().
  Useful on CI."; `repeatEach` "The number of times to repeat each test, useful for debugging flaky tests."; `webServer`
  "Launch a development web server (or multiple) during the tests."
- **R5-TEST-F10. Playwright's clock controls a page's time sources.** Source: https://playwright.dev/docs/clock ·
  official · read 2026-10-28 · confirmed in the source: it "overrides native global classes and functions related to time
  allowing them to be manually controlled."; `runFor` "Runs the time for a specific duration."; `fastForward` "Fast
  forwards the time." The covered list (Date, timers, `requestAnimationFrame`, `performance`) is from the fetch summary
  (inferred). Workers are not mentioned (confirmed by absence).
- **R5-TEST-F11. turmoil 0.7.2 simulates hosts and a network in one thread, with seeded latency and message failure,
  for Tokio code.** Sources: https://github.com/tokio-rs/turmoil (README) and https://docs.rs/turmoil/latest/turmoil/,
  .../struct.Builder.html (0.7.2) · official · read 2026-10-28 · confirmed in the source: "Turmoil is a family of crates
  for deterministic simulation testing of distributed systems."; it "injects 'hardship' — latency, drops, partitions,
  crashes, torn writes — into the simulated network and filesystem." "under manual control or a seeded RNG."; turmoil-net
  is a "Drop-in replacement for `tokio::net`"; "This module contains the simulated TCP/UDP networking types.";
  `min_message_latency` / `max_message_latency` "The minimum [maximum] latency that a message will take to transfer over
  a link on the network."; `fail_rate` "The failure rate of messages on the network."; `rng_seed` "Seed for random numbers
  generated by turmoil." QUIC is not mentioned (confirmed by absence).
- **R5-TEST-F12. A shipping game transport injects loss, lag, jitter, reordering, duplication and rate limits below the
  protocol, per direction.** Sources:
  https://raw.githubusercontent.com/ValveSoftware/GameNetworkingSockets/master/include/steam/steamnetworkingtypes.h and
  https://github.com/ValveSoftware/GameNetworkingSockets (README: "a basic transport layer for games." that "has shipped
  on consoles, mobile platforms, and non-Steam stores") · shipped game (a shipping vendor's own library, the tier rule of
  R2) · read 2026-10-28 · confirmed in the source: "[global float, 0--100] Randomly discard N pct of packets instead of
  sending/recv"; "This is a global option only, since it is applied at a low level"; "[global int32].  Delay all
  outbound/inbound packets by N ms"; "Simulated jitter/clumping."; "[global float 0--100] Percentage of packets we will
  add additional" [delay to, for reordering]; "[global float 0--100] Globally duplicate some percentage of packets.";
  "[global int32] Global UDP token bucket rate limits."; "[global int32] Trace every UDP packet, similar to Wireshark or
  tcpdump."
- **R5-TEST-F13. quinn 0.11.12 can run on any socket that implements its `AsyncUdpSocket` trait.** Sources:
  https://docs.rs/quinn/latest/quinn/trait.AsyncUdpSocket.html and .../struct.Endpoint.html (0.11.12) · official · read
  2026-10-28 · confirmed in the source: "Abstract implementation of a UDP socket for runtime independence";
  `Endpoint::new_with_abstract_socket(config, server_config, socket: Arc<dyn AsyncUdpSocket>, runtime)` "Construct an
  endpoint with arbitrary configuration and pre-constructed abstract socket". Whether wtransport or web-transport-quinn
  (R2-TRN-REC1) expose this constructor: not read (unverified).
- **R5-TEST-F14. A game-networking practitioner tests against extreme simulated networks and soak-tests overnight.**
  Source: https://gafferongames.com/post/packet_fragmentation_and_reassembly/ (September 6, 2016) · other (R2 tiers
  Glenn Fiedler's articles "other", R2-XU-OQ16) · read 2026-10-28 · confirmed in the source: "Implement network
  simulators and make sure code handles the worst possible network conditions imaginable."; "99% packet loss, 10 seconds
  of latency and +/- several seconds of jitter."; soak with "a huge amount of packet loss, out of order and duplicates
  through a packet simulator."; "Your soak test passes when it runs overnight and doesn't hang or assert."

#### Legacy facts used
- **Evidence habits** (START_HERE §5): targeted harness tests on the real code: animation metrics (hands on sockets
  < 1 cm, planted-foot drift, boots ≥ 0 cm above ground, IK stretch ≤ 0.98), a fuzz of every class and ability with 0
  errors, a 3-minute soak with a flat heap after GC, `sim.js` stall % over at least 3 runs per build (random AI).
- **TL9** (redesign): the smoke shape carries: wait on game state with generous timeouts, check that each step did its
  job, console errors fail, report the renderer string, fresh storage per page; probes move to a v2 test-hook API; the
  network rule flips to allow the local game server's WebTransport and WebSocket only.
- **TL11** (drop, kept as the legacy oracle until legacy retires): `preview.js eval` runs harness snippets on the real
  legacy build; the natural runner for golden values.
- **TL13** (redesign): keep sim.js's metrics (stall %, engaged against free stalls, kill matrix, friendly fire,
  objective counters, slot bias); v2 runs seeded, deterministic bot matches on the native server. Legacy same-seed runs
  differ (11 wall-clock reads, STATUS:111).
- **TL14** (redesign): reachability becomes a 3D navigation check in the Rust sim, callable from the editor and CI.
- **TL15** (redesign): warm-up then measured steps, percentiles, reps, the fairness metric; in v2 a Rust tick bench at [N](GLOSSARY.md#player-count-n)
  players plus a client render bench.
- **LOOP_V2 §11**: evidence per greybox step is the gate, `sim.js` with three teams and at least 3 runs per build,
  objective counters (cashouts, steals, deliveries, harvester kills); then the scale test at N = 12, 24, 36, 48.
- **Golden candidates with recorded numbers**, all marked "rewrite in Rust": CS4 (a ray that starts inside the torso
  cylinder returns -1, inside the head sphere 0), CS5 (a face shot on a 45° crate measures 4.586, was 3.586), CS6 (the
  B7 batch ray gave the same results as the old code on 20,000 of 20,000 random rays, 1,640 rotated hits), GW14 and
  GN10 (the jump reach is 3.07 m by the formula, about 2.98 m as integrated at 60 Hz: frame-rate dependent), CD8
  (locational damage multipliers), CD12 (the weapons table; LOOP_V2 §7 changes the AMK), GE8 (presence count). Marked
  "port to TypeScript": GA5 (planted-foot drift 0.0 on walk, strafe and backpedal; sprint mean 0.14) and GA8 (hand on
  grip 0.0 cm, target < 1 cm). `docs/legacy-tuning.json`: every tuning number, produced by `tools/tuning-dump.js` on the
  real build; two runs per map gave byte-identical output; 17 values checked against their source literal.
- **Gate history** (docs/reports/2026-10-14-gate-check-5.md): 22 of 22 planted defects failed exactly the expected
  checks, and controls (valid code that looks suspicious) passed: the model for proving each test can fail.
- Wave 1 cited, not restated: R1-DET-REC6 (recorded matches, per-tick state hashes; native twice in separate processes,
  Wasm in Node, Wasm in the browsers the runner offers; "R5 picks the tool"), R1-ECS-REC2 mark 1 (two processes, same
  hashes over a 10-minute bot match), R2-XU-OQ12 and r2-netcode.md's sync profiles (P0 = 0 ms; P1 = 50 ms ±10 ms 1 %;
  P2 = 100 ms ±20 ms 2 %; P3 = 150 ms ±30 ms 5 %; P4 = 200 ms ±40 ms 5 %; B1 = P2 plus a 1 s burst of 100 % loss; round
  trip; reordering in every spike), R2-TRN-REC4 marks 1, 2 and 4 (all four browsers on both backends; app RTT within
  ±5 ms of quinn's at 100 ms; 10 million fuzz inputs per message type with 0 panics), R2-TRN-OQ1 (a recurring run
  against browser beta channels), R4's notes to R5 (a counting test for the static BVH build, a parity fuzz, the
  reachability check in the sim crate), R3-MEASURE-REC3 (CI gates counts, not timings).

### Question 5: CI with Rust caching (R5-CI; infra-engineer)

The brief's R5 row (docs/V2_PHASE0_BRIEF.md:125): "Repository layout (a Cargo workspace plus TypeScript with Vite);
strict TypeScript and lint rules; clippy and rustfmt; tests (cargo test, property tests, Vitest, Playwright, golden
tests against legacy); CI with Rust caching; **preview links per PR**; the gate v2 that replaces today's gate". This
section answers "CI with Rust caching", plus how CI runs the gate v2. What the gate v2 checks is the owner's part.


**Rust build caching**

- **R5-CI-F1.** Swatinem/rust-cache caches the Cargo home and the dependencies' build output in `target`, not the
  unpacked registry sources. Latest release v2.9.3 ("20 Oct", no year on the page). Quotes: it caches "`~/.cargo`
  (installed binaries, the cargo registry, cache, and git dependencies)" and "`./target` (build artifacts of
  dependencies)"; "The `~/.cargo/registry/src` directory is not cached." · Source: https://github.com/Swatinem/rust-cache
  (README) and https://github.com/Swatinem/rust-cache/releases · official (the tool's own docs) · read 2026-10-28 ·
  confirmed in the source.
- **R5-CI-F2.** rust-cache keys each cache on the toolchain, the lockfile and the manifests, so a toolchain bump or a
  lockfile change starts a new cache. Quotes: the key holds "the rustc release / host / hash (for all installed
  toolchains when available)" and "a hash of the workspace's `Cargo.lock`" "and of the Cargo.toml of the workspace root
  and every workspace member" (the second part added from the verifier's words, 2026-10-29). The inputs include
  `shared-key` ("stable over multiple jobs"), `save-if` ("Determines whether the cache should be saved."),
  `cache-on-failure`, `workspaces` ("`$workspace -> $target`"), `cache-targets`, `cache-all-crates` and `cache-bin`. ·
  Source: https://github.com/Swatinem/rust-cache (README, inputs and cache key) · official (tool docs) · read 2026-10-28
  · confirmed in the source.
- **R5-CI-F3.** rust-cache caches only dependencies and turns incremental builds off. Quotes: before saving it removes
  "Incremental build artifacts."; "the workspace crates themselves are not cached since doing so is generally not
  effective."; "this action automatically sets `CARGO_INCREMENTAL=0` to disable incremental compilation." · Source:
  https://github.com/Swatinem/rust-cache (README) · official (tool docs) · read 2026-10-28 · confirmed in the source.
- **R5-CI-F4.** GitHub scopes caches by branch. A pull request can restore its base branch's and the default branch's
  caches, but a cache that a pull request writes serves only re-runs of that same pull request. Quotes: "Workflow runs
  can restore caches created in either the current branch or the default branch (usually `main`)."; "If a workflow run
  is triggered for a pull request, it can also restore caches created in the base branch,"; a PR's cache "is created
  for the merge ref (`refs/pull/.../merge`)" and "can only be restored by re-runs of the pull request." "It cannot be
  restored by the base branch or other pull requests". rust-cache says the same: "Caches from base branches are
  available to PRs, but not across unrelated branches." · Sources:
  https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching ("Restrictions for accessing a
  cache"; no page date) and https://github.com/Swatinem/rust-cache · official · read 2026-10-28 · confirmed in the
  source.
- **R5-CI-F5.** The free cache is 10 GB per repository, and entries unused for 7 days are evicted. Going above 10 GB is
  opt-in, paid, and needs a Pro, Team or Enterprise account. Over the limit, the least recently used entries go first.
  Quotes: "By default, the limit is 10 GB per repository"; "GitHub will remove any cache entries that have not been
  accessed in over 7 days."; the default 10 GB and seven-day limits come "at no additional cost."; exceeding 10 GB
  "requires a valid Pro, Team, or Enterprise account."; "Enterprise, Organization, and Repository admins can increase
  these limits."; "the least recently used cache entries are automatically evicted to stay within the configured
  size." rust-cache: "exceeding that limit will cause eviction of older caches." · Sources:
  https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching and
  https://github.blog/changelog/2025-11-20-github-actions-cache-size-can-now-exceed-10-gb-per-repository/ (November 20,
  2025) · official · read 2026-10-28 · confirmed in the source.
- **R5-CI-F6.** Caches can be read and poisoned through pull requests, forks included, so a cache must never hold a
  secret. Quotes: "We recommend that you don't store any sensitive information, such as access tokens or login
  credentials,"; "Anyone with read access can create a pull request on a repository and access the contents of a
  cache."; "forks of a repository can create pull requests on the base branch and access caches on the base branch.";
  "This class of attack is known as *cache poisoning*." · Source:
  https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching · official · read 2026-10-28 ·
  confirmed in the source.
- **R5-CI-F7.** GitHub rate-limits cache traffic per repository. Quotes: "up to 200 uploads per minute per repository,"
  and "1500 downloads per minute per repository." · Source: same page · official · read 2026-10-28 · confirmed in the
  source.
- **R5-CI-F8.** sccache can use the GitHub Actions cache as its storage. The current action is
  mozilla-actions/sccache-action v0.0.11 and the current sccache is v0.19.0 (24 Oct 2026, per the asset timestamps).
  Quotes: "For Rust code, the following environment variables should be set:" `SCCACHE_GHA_ENABLED: "true"` and
  `RUSTC_WRAPPER: "sccache"`; the README pins `mozilla-actions/sccache-action@v0.0.11`, and its version example is
  `version: "v0.16.0"`; "Versions prior to sccache v0.11.0 probably will not work." · Sources:
  https://github.com/Mozilla-Actions/sccache-action (README) and https://github.com/mozilla/sccache/releases · official
  (tool docs) · read 2026-10-28 · confirmed in the source.
- **R5-CI-F9.** sccache cannot cache the crates that matter most to us: our Wasm crate (a `cdylib`, inferred from R1's
  wasm-bindgen design), the server binary, proc-macro crates, and every incrementally compiled crate. Quotes: "Crates
  that invoke the system linker cannot be cached." "This includes `bin`, `dylib`, `cdylib`, and `proc-macro` crates.";
  "Incrementally compiled crates cannot be cached."; "By default, in the debug profile Cargo will use incremental
  compilation for workspace members and path dependencies."; "By default, absolute paths to files must match to get a
  cache hit." · Source: https://github.com/mozilla/sccache (README, "Known Caveats") · official (tool docs) · read
  2026-10-28 · confirmed in the source (the limits); inferred (which of our crates they hit).
- **R5-CI-F10.** Cargo's dev profile is incremental by default, and an environment variable overrides that. Quotes:
  dev `incremental = true`, release `incremental = false`; "The incremental value can be overridden globally with the
  `CARGO_INCREMENTAL` [environment variable]"; `"line-tables-only"` "Generates the minimal amount of debug info for
  backtraces with filename/line number info". · Source: https://doc.rust-lang.org/cargo/reference/profiles.html · official
  · read 2026-10-28 · confirmed in the source.

**Pinning the toolchain, the lockfile and the wasm-bindgen CLI**

- **R5-CI-F11.** `cargo --locked` fails the build when Cargo.lock is missing or would change, and the Cargo book
  suggests it for CI. Quotes: it "Asserts that the exact same dependencies and versions are used as when the existing
  `Cargo.lock` file was originally generated."; "It may be used in environments where deterministic builds are desired,
  such as in CI pipelines."; `--frozen` is "Equivalent to specifying both `--locked` and `--offline`."; `--timings`
  writes "`cargo-timing.html`" to "`target/cargo-timings`" and does "not provide machine-readable timing data." ·
  Source: https://doc.rust-lang.org/cargo/commands/cargo-build.html · official · read 2026-10-28 · confirmed in the
  source.
- **R5-CI-F12.** One `rust-toolchain.toml` pins the channel, the components and the extra targets, wasm32 included.
  Quotes: the example holds `channel = "nightly-2020-07-10"`, `components = [ "rustfmt", "rustc-dev" ]`, `targets = [
  "wasm32-unknown-unknown", "thumbv2-none-eabi" ]` and `profile = "minimal"`; "The `targets` setting contains a list of
  platforms to install for [cross-compilation]."; "The host platform is automatically included". The channel grammar
  allows "<major.minor>|<major.minor.patch>". Precedence, highest first: `cargo +toolchain`, `RUSTUP_TOOLCHAIN`,
  `rustup override`, then `rust-toolchain.toml`, then the default. For pinned projects, "`Cargo.lock` should probably be
  tracked too". · Source: https://rust-lang.github.io/rustup/overrides.html · official · read 2026-10-28 · confirmed in
  the source.
- **R5-CI-F13.** Since rustup 1.28.0 (2 Mar 2025), a missing pinned toolchain is no longer installed automatically, so
  CI must install it explicitly. Quotes: "rustup will no longer automatically install the active toolchain if it is not
  installed."; "run `rustup toolchain install` with no arguments"; `rustup show active-toolchain || rustup toolchain
  install` "installs the active toolchain both before and after this change." 1.28.1 added `RUSTUP_AUTO_INSTALL`, and
  1.28.2 (5 May 2025) added "`rustup set auto-install disable`". The default after 1.28.1 is not stated on the pages
  read. · Sources: https://blog.rust-lang.org/2025/03/02/Rustup-1.28.0/ and
  https://blog.rust-lang.org/2025/05/05/Rustup-1.28.2/ · official · read 2026-10-28 · confirmed in the source.
- **R5-CI-F14.** The latest wasm-bindgen is 0.2.129 (25 Sep 2026, per the asset timestamps). Each release ships prebuilt
  CLI archives per platform with `.sha256sum` files and a release attestation. Quote (0.2.129): "The CLI's schema
  version mismatch error now names the input Wasm file and the path of the running `wasm-bindgen` binary." The rule
  that the CLI and library versions must match is R1-BND-F2 (wave 1); not repeated here. · Source:
  https://github.com/wasm-bindgen/wasm-bindgen/releases · official · read 2026-10-28 · confirmed in the source. The
  x86_64 Linux archive sat behind "Show all 17 assets" and was not seen: inferred present.
- **R5-CI-F15.** taiki-e/install-action installs wasm-bindgen (also wasm-pack, cargo-nextest, cargo-deny and
  cargo-audit) at an exact version from the GitHub release and checks its SHA256. wasm-opt (binaryen) is not on its
  list. Quotes: "To install a specific version, use `@version` syntax:"; "this action will also verify SHA256 checksums
  for downloaded files for all tools covered in this section."; TOOLS.md lists "**wasm-bindgen** (alias:
  `wasm-bindgen-cli`)" from "GitHub Releases"; tools not on the list fall back to cargo-binstall, which `fallback: none`
  turns off; "consider using the `@v<major>.<minor>.<patch>` tag or their hash to pin the version."; "Pinning
  `@<tool_name>` tags by hash is strongly discouraged." · Sources: https://github.com/taiki-e/install-action (README; no
  release shown) and https://raw.githubusercontent.com/taiki-e/install-action/main/TOOLS.md · official (tool docs) ·
  read 2026-10-28 · confirmed in the source.

**Node dependencies, runners and browsers**

- **R5-CI-F16.** actions/setup-node (v7 in its examples) caches npm's global package cache, not `node_modules`, keyed
  by a lockfile it can be pointed at. It advises turning automatic caching off in privileged workflows. Quotes: `cache`
  is "Used to specify a package manager for caching in the default directory. Supported values: npm, yarn, pnpm.";
  `cache-dependency-path` is "Used to specify the path to a dependency file: package-lock.json, yarn.lock, etc.";
  `package-manager-cache` "Controls automatic caching for npm."; "For workflows with elevated privileges or access to
  sensitive information, we recommend disabling automatic caching". · Source: https://github.com/actions/setup-node
  (README) · official · read 2026-10-28 · confirmed in the source for the quotes. That automatic caching turns on when
  package.json has a `packageManager` field is inferred (WebFetch summary only).
- **R5-CI-F17.** Standard GitHub-hosted runners are free and unlimited for public repositories. ubuntu-24.04 has 4 CPUs,
  16 GB of RAM and a 14 GB SSD, an arm64 runner exists, and standard runners have no GPU. Quotes: "Use of the standard
  GitHub-hosted runners is free and unlimited on public repositories."; public `ubuntu-24.04`: CPU "4", RAM "16 GB", SSD
  "14 GB" (private: "2", "8 GB", "14 GB"); "ubuntu-24.04-arm" (arm64, 4 CPUs, 16 GB, 14 GB); GPUs appear only under
  larger runners ("GPU-powered runners"). · Source: https://docs.github.com/en/actions/reference/runners/github-hosted-runners
  (no page date) · official · read 2026-10-28 · confirmed in the source.
- **R5-CI-F18.** Playwright advises against caching browsers in CI and recommends one worker there. Its docs reference
  v1.64.1 (Docker tag), while the legacy toolkit pins 1.56.1 (TL17). Quotes: "Caching browser binaries is not
  recommended, since the amount of time it takes to restore the cache is comparable to the time it takes to download the
  binaries."; "operating system dependencies need to be installed, which are not cacheable."; "We recommend setting
  workers to "1" in CI environments to prioritize stability and reproducibility."; "headed execution requires Xvfb to
  be installed." No GPU or WebGPU guidance. · Source: https://playwright.dev/docs/ci · official · read 2026-10-28 ·
  confirmed in the source.
- **R5-CI-F19.** The Cargo book's CI guide shows a toolchain matrix and a separate, non-blocking job that tests the
  newest dependency versions. It warns that scheduled jobs can be switched off on quiet repositories. Quotes: the
  latest-deps job sets `continue-on-error: true` and `CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS: allow`, and runs
  `cargo update`; "A hosted CI service may disable scheduled jobs for repositories that haven’t been touched in a
  while". · Source: https://doc.rust-lang.org/cargo/guide/continuous-integration.html · official · read 2026-10-28 ·
  confirmed in the source.

**Running the gate in CI: required checks and supply chain**

- **R5-CI-F20.** A workflow skipped by a path filter blocks a required check forever, while a job skipped by a
  condition counts as passed. Quotes: skipped "by path filtering, branch filtering, or a commit message": "Associated
  checks stay in a "Pending" state and block merging"; skipped by a conditional: "The job reports "Success""; a job that
  depends on a failed job "is skipped and may not block merging." · Source:
  https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/collaborating-on-repositories-with-code-quality-features/troubleshooting-required-status-checks
  · official · read 2026-10-28 · confirmed in the source.
- **R5-CI-F21.** GitHub's hardening guide asks for SHA-pinned actions, a minimal `GITHUB_TOKEN`, and untrusted input
  passed through environment variables. Quotes: "Pin references to actions to a commit SHA in your workflow files.";
  "When selecting a SHA, you should verify it is from the action's repository and not a repository fork."; "make sure
  that the `GITHUB_TOKEN` is granted the minimum required permissions."; the intermediate environment variable is the
  preferred handling of untrusted input (WebFetch summary, inferred). · Source:
  https://docs.github.com/en/actions/reference/security/secure-use · official · read 2026-10-28 · confirmed in the source
  (quotes).
- **R5-CI-F22.** GitHub now offers workflow dependency lockfiles (public preview since 14 Oct 2026). A lockfile records
  the commit of every action, transitive ones included, and GitHub calls it stronger than SHA pinning. Quotes: "The best
  measure is to lock your workflow dependencies with a lockfile."; "This records the exact commit for every action your
  workflows depend on, including transitive dependencies,"; "supersedes SHA pinning for repositories that adopt it.";
  the path is `.github/workflows/actions.lock`, made with "the `gh actions-lock` extension"; plans: "GitHub Enterprise
  Cloud, GitHub Enterprise Cloud with data residency, Team, and Free plans." (Pro is not listed); Dependabot "also
  regenerates the matching lockfile entry in the same pull request."; "Locking does not determine whether the
  dependency itself is safe". · Sources: https://docs.github.com/en/actions/reference/security/secure-use,
  https://github.blog/changelog/2026-10-14-github-actions-workflow-lockfiles-in-public-preview/ (October 14, 2026) and
  https://docs.github.com/en/actions/concepts/security/workflow-dependency-lockfiles · official · read 2026-10-28 ·
  confirmed in the source.
- **R5-CI-F23.** A repository setting can require full-length SHA pins, and in a personal account's repository the
  default token is read-only. Quotes: the option "Require actions to be pinned to a full-length commit SHA"; in
  personal-account repositories the "GITHUB_TOKEN only has read access for the contents and packages scopes." That the
  pin rule also covers GitHub's own actions is inferred (summary). · Source:
  https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository
  · official · read 2026-10-28 · confirmed in the source (quotes).
- **R5-CI-F24.** Dependabot can keep action references current. Quote: "You can use Dependabot to keep the actions you
  use updated to the latest versions." The page does not say how Dependabot treats SHA-pinned references. · Source:
  https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/keeping-your-actions-up-to-date-with-dependabot
  · official · read 2026-10-28 · confirmed in the source (the SHA-pin behaviour: unverified).

#### Legacy facts used

- **KT8 (gate.yml).** Today's CI hygiene carries over unchanged: `permissions: contents: read`, actions pinned by SHA
  with a version comment (checkout v7.0.1, setup-node v7.0.0, cache v5.1.0), `persist-credentials: false`,
  `--ignore-scripts`, environment-only inputs, concurrency with cancel-in-progress, and per-job timeouts of 10 and
  15 min (.github/workflows/gate.yml:12-17, :22-29, :34, :54). The legacy numbers set the time scale v2 is judged
  against: gate job 10-17 s; smoke job 57-59 s with Chromium cached and 63-67 s without, about 20 s of it apt. Required
  checks have been on main since 2026-10-17. For v2: the same hygiene plus Rust and TypeScript jobs.
- **KT8 plus R5-CI-F18.** Caching the browser saved about 4-8 s of a roughly 60 s job, close to what Playwright
  predicts (R5-CI-F18). Dropping that cache in v2 costs little.
- **gate.yml:67-72.** Today's CI reads a tool version from the lockfile, checks that it looks like `x.y.z`, and keys
  the cache on it. The same pattern pins the wasm-bindgen CLI to the library version in Cargo.lock (R5-CI-REC3).
- **TL1.** The gate's rules carry over to the gate v2 in CI: an explicit base ref, exit 2 for a setup problem and never
  a silent skip, `--json`, `--allow-drop` tied to the plan, and mutation proofs after every gate change. A required
  check that hangs as "Pending" (R5-CI-F20) would break the "never a silent skip" rule.
- **TL9.** The smoke harness's shape carries over to v2's Playwright job. Its network rule flips: allow the local test
  server's WebTransport and WebSocket, block everything else.
- **TL17.** Exact pins with a lockfile, `npm ci`, optional native dependencies kept out of CI, and the dependency audit
  all carry over. The open legacy recommendation applies: commit the mutation suite so the gate v2 proves itself in one
  command.
- **KT19.** `.gitignore` must add `target/`, the Wasm package output and the TypeScript caches. CI caches never hold
  secrets (R5-CI-F6).
- **Repository visibility.** The repository is public (`gh api repos/Mergincanli/Shamble-Frontlines`:
  `"visibility":"public"`, 2026-10-28). So standard runners cost nothing (R5-CI-F17), forks can open pull requests and
  read base-branch caches (R5-CI-F6), and environment reviewers are available on the Free plan (R5-PREV-F6).

### Question 6: preview links per PR (R5-PREV; infra-engineer)

The brief's R5 row (docs/V2_PHASE0_BRIEF.md:125), quoted in full under Question 5; this section answers "**preview links
per PR**". Mustafa opens each link in Brave on Windows. Which host serves the previews, and its limits and prices, is
R6's: this section names the hosts considered and what the workflow needs from them, and leaves prices to R6.


**Pull requests from forks and the secrets**

- **R5-PREV-F1.** A pull request from a fork gets no secrets and a read-only token. Quotes: "secrets are not passed to
  the runner when a workflow is triggered from a forked repository."; "The `GITHUB_TOKEN` has read-only permissions in
  pull requests from forked repositories." · Source:
  https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows (`pull_request`; no
  page date) · official · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F2.** `pull_request_target` runs the base repository's workflow with its trust, and GitHub says not to use
  it to build pull-request code. Quotes: "This event runs in the context of the default branch of the base repository,
  rather than in the context of the merge commit."; "Running untrusted code on the `pull_request_target` trigger may
  lead to security vulnerabilities."; "Avoid using this event if you need to build or run code from the pull request."
  · Source: same page (`pull_request_target`) · official · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F3.** A `workflow_run` workflow gets secrets and a write token even when the run that triggered it had
  neither, so it must treat that run's artifacts as untrusted and never check out its code. Quotes: it "is able to
  access secrets and write tokens, even if the previous workflow was not."; "Running untrusted code on the
  `workflow_run` trigger may lead to security vulnerabilities."; "Workflows triggered on `workflow_run` should treat
  artifacts uploaded from other workflows with caution."; such workflows "must not explicitly check out untrusted
  code." · Sources: the same events page (`workflow_run`) and
  https://docs.github.com/en/actions/reference/security/secure-use · official · read 2026-10-28 · confirmed in the
  source.
- **R5-PREV-F4.** A workflow can run when a pull request closes or merges. By default `pull_request` runs only on
  opened, synchronize and reopened. Quotes: "To run a workflow when a pull request merges, use the `pull_request`
  `closed` event type along with a conditional that checks the `merged` value of the event."; "By default, a workflow
  only runs when a `pull_request` event's activity type is `opened`, `synchronize`, or `reopened`." · Source: the same
  events page · official · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F5.** Workflow runs from first-time contributors' forks wait for a maintainer's approval by default, and
  stricter settings exist. Quotes: "By default, all first-time contributors require approval to run workflows."; the
  options are "Require approval for first-time contributors who are new to GitHub", "Require approval for first-time
  contributors" and "Require approval for all external contributors"; approvers should be "especially alert to any
  proposed changes in the `.github/workflows/` directory". · Sources:
  https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository
  and https://docs.github.com/en/actions/how-tos/manage-workflow-runs/approve-runs-from-forks · official · read
  2026-10-28 · confirmed in the source.
- **R5-PREV-F6.** A GitHub environment can hold the deploy token behind rules, and the reviewer rule is available to
  public repositories on the Free plan. Quotes: "required reviewers are only available for public repositories" (on
  Free, Pro and Team); "a job cannot access environment secrets until one of the required reviewers approves it";
  "Deployment branches and tags are available for all public repositories."; "Running a workflow that references an
  environment that does not exist will create an environment with the referenced name."; "only repository admins can
  configure the environment". · Sources:
  https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments and
  https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/manage-environments · official ·
  read 2026-10-28 · confirmed in the source.
- **R5-PREV-F7.** A job's environment can carry a URL, which becomes the deployment's `environment_url`. Quotes: "as an
  environment object with the `name` and `url`."; "The URL maps to `environment_url` in the deployments API." That
  GitHub shows this link on the pull request page is not stated on the pages read. · Sources:
  https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax (`jobs.<job_id>.environment`) and
  the manage-environments page · official · read 2026-10-28 · confirmed in the source (the API field); the PR-page link
  is unverified.
- **R5-PREV-F8.** Writing a deployment or commenting on a pull request needs a named token scope. Quotes: "`deployments:
  write` permits an action to create a new deployment."; "`pull-requests: write` permits an action to add a label to a
  pull request." (the comment scope is inferred from the same row). · Source: the workflow-syntax page (`permissions`) ·
  official · read 2026-10-28 · confirmed in the source (quotes).
- **R5-PREV-F9.** A later job or run downloads the build through actions/download-artifact (v8). Artifacts are kept
  90 days by default; public repositories can set 1 to 90. Quotes: "This is required when downloading artifacts from a
  different repository or from a different workflow run." (token "with actions:read permissions"); artifacts and logs
  are "retained for 90 days before they are automatically deleted." · Sources: https://github.com/actions/download-artifact
  (README) and the repository Actions-settings page · official · read 2026-10-28 · confirmed in the source.

**Hosts considered and how they deploy a preview**

- **R5-PREV-F10.** [Cloudflare Pages](GLOSSARY.md#the-client-host) takes a direct upload from GitHub Actions through cloudflare/wrangler-action v4,
  with an account-scoped "Pages Edit" API token stored as a repository secret. With the GitHub token it can also
  create a GitHub Deployment. Quotes: the example command is `"pages deploy YOUR_DIRECTORY_OF_STATIC_ASSETS
  --project-name=YOUR_PROJECT_NAME"`; it "automatically triggers on the current git branch, unless you add a `branch`
  option to the `with` section."; for the token, "select *Account*, *Cloudflare Pages* and *Edit*"; with `deployments:
  write` the action can "create a Deployment on your behalf." · Source:
  https://developers.cloudflare.com/pages/how-to/use-direct-upload-with-continuous-integration/ (Last updated Sep 18,
  2026) · official · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F11.** wrangler-action returns the preview's URLs as outputs and can pin Wrangler's version. Quotes:
  "utilize the `deployment-url` output variable to get the URL of the deployment"; "Pages deployments will also provide
  their alias URL (since Wrangler v3.78.0)"; "The action exposes `preview-url`, `preview-deployment-url`,
  `preview-name`, `preview-id`,"; "You can pass `wranglerVersion` to install a specific version of Wrangler from
  NPM."; "Your API token is encrypted by GitHub, and the action won't print it into logs." Nothing on forks. · Source:
  https://github.com/cloudflare/wrangler-action (README; examples `@v4`, "defaults to **Wrangler v4**") · official (tool)
  · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F12.** Cloudflare Pages previews are public by default, but can be limited to the Cloudflare account. They
  send `noindex`, and get a stable alias per branch. Quotes: "By default, these deployment URLs are public."; "Enabling
  the access policy will restrict viewing project deployments to your Cloudflare account."; "By default, every preview
  deployment generated by Cloudflare Pages includes the X-Robots-Tag: noindex HTTP response header."; "Branch name
  aliases are lowercased and non-alphanumeric characters are replaced with a hyphen." (`fix/api` becomes
  `fix-api.<project>.pages.dev`). · Source: https://developers.cloudflare.com/pages/configuration/preview-deployments/
  (Last updated Oct 12, 2026) · official · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F13.** Cloudflare Pages deployments are deleted with Wrangler, but a branch's latest deployment cannot be.
  Quotes: `npx wrangler pages deployment delete <DEPLOYMENT_ID> --project-name <PROJECT_NAME>`; "Use the `--force` (or
  `-f`) flag to skip the confirmation prompt, and to force deletion of aliased deployments."; "The latest deployment for
  a branch cannot be deleted." · Source: same page · official · read 2026-10-28 · confirmed in the source. How the two
  sentences fit together is not explained (R5-PREV-OQ2).
- **R5-PREV-F14.** Cloudflare Workers make preview aliases at upload. Those are public when version URLs are on, can be
  put behind Cloudflare Access, and only the 1000 most recent are kept. Quotes: `npx wrangler versions upload
  --preview-alias staging`; URL "<alias>-<worker-name>.<subdomain>.workers.dev"; "If Version URLs are enabled, the URL
  is public and available after version creation."; "To require visitors to sign in, use Cloudflare Access."; "The
  alias and Worker name combined with a dash must not exceed 63 characters due to DNS label limits."; "the 1000 most
  recently deployed aliases are retained."; "When a new alias is created beyond this limit, the least recently deployed
  alias is deleted." The page says nothing about static assets or `noindex`. · Source:
  https://developers.cloudflare.com/workers/configuration/previews/ (Last updated Sep 22, 2026) · official · read
  2026-10-28 · confirmed in the source.
- **R5-PREV-F15.** Netlify builds a Deploy Preview for each pull request on its own servers, at a numbered URL, with a
  status on the PR, a password option and `noindex`. Quotes: "When you open a pull / merge request in a connected site
  repository from GitHub, ... Netlify automatically builds a Deploy Preview for you."; "`deploy-preview-42--mysitename.netlify.app`";
  "You can control who can access your site's Deploy Previews by requiring a password"; "Netlify adds an `X-Robots-Tag:
  noindex` header to Deploy Previews."; "a Deploy Preview status will appear on your pull / merge request."; a comment
  with the link comes only "If you have [deploy notifications] enabled". · Source:
  https://docs.netlify.com/site-deploys/deploy-previews/ (Last updated: Oct 8, 2026) · official · read 2026-10-28 ·
  confirmed in the source.
- **R5-PREV-F16.** For public repositories, Netlify by default keeps sensitive variables out of untrusted (fork)
  builds. Quote: "our [sensitive variable policy] default setting for public repositories prevents untrusted pull/merge
  requests from building with sensitive variable values. When there are potentially sensitive environment variables in
  your project configuration, untrusted deploys require approval." · Source:
  https://docs.netlify.com/platform/security-checklist/ (Last updated: Sep 30, 2026) · official · read 2026-10-28 ·
  confirmed in the source.
- **R5-PREV-F17.** Vercel can deploy a preview that Actions built, using three repository secrets, with its own Git
  deployments switched off. Quotes: "vercel pull --yes --environment=preview --token=${{ secrets.VERCEL_TOKEN }}",
  "vercel build --token=...", "vercel deploy --prebuilt --token=..."; secrets `VERCEL_TOKEN`, `VERCEL_ORG_ID`,
  `VERCEL_PROJECT_ID`; "Set `"git": { "deploymentEnabled": false }` in `vercel.json` to hand deployment to Actions
  cleanly." · Source: https://vercel.com/kb/guide/how-can-i-use-github-actions-with-vercel (published 2026-07-23,
  updated 2026-10-16) · official · read 2026-10-28 · confirmed in the source. Vercel's Hobby plan terms are R6's.

**Headers that previews need**

- **R5-PREV-F18.** Cloudflare Pages reads response headers from a `_headers` file in the build output, and its rules
  can match one hostname pattern, such as preview subdomains. Quotes: "a plain text file called `_headers` without a
  file extension, in the static asset directory of your project."; the example rules `https://:project.pages.dev/*` and
  `https://:version.:project.pages.dev/*` (the latter with `X-Robots-Tag: noindex`); "You may define up to 100 header
  rules."; "Each line in the `_headers` file has a 2,000 character limit."; the headers do not apply to Pages Functions
  responses. · Source: https://developers.cloudflare.com/pages/configuration/headers/ (Last updated Aug 25, 2026) ·
  official · read 2026-10-28 · confirmed in the source. Whether the branch-alias hostnames match
  `:version.:project.pages.dev` is inferred (one label before the project, F12).
- **R5-PREV-F19.** Vite 8 (v8.3.4) sets response headers for both its dev server and its local preview server, and says
  the preview server is not for production. Quotes: `server.headers` and `preview.headers` are typed
  `OutgoingHttpHeaders`, "Specify server response headers."; "`vite preview` is intended for previewing the build
  locally and not meant as a production server."; "By default, the build output will be placed at `dist`." · Sources:
  https://vite.dev/config/server-options, https://vite.dev/config/preview-options and
  https://vite.dev/guide/static-deploy · official · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F20.** The fast way to load Wasm needs the `.wasm` file served as `application/wasm`; otherwise it fails.
  Quotes: "If the response has an incorrect MIME type (instead of `application/wasm`), the promise rejects with a
  [`TypeError`]"; "`.wasm` files should be returned with an `application/wasm` MIME type by the server." · Source:
  https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiateStreaming_static (Last
  modified Sep 22, 2026) · official · read 2026-10-28 · confirmed in the source.

#### Legacy facts used

- **KT8 (gate.yml).** Today's CI has no preview, no deploy step and no secret (`permissions: contents: read`,
  gate.yml:12-13). A preview workflow is the first CI job to hold a secret, so the legacy rule "no
  `pull_request_target`" (security-auditor, phase 0, cited in KT8) carries over and extends to `workflow_run`
  (R5-PREV-F3).
- **TL9.** The smoke harness fails on any outside network request. A preview link adds a check after deployment that
  the deployed build is the one CI tested (R5-PREV-REC4).
- **CLAUDE.md, "People".** Mustafa plays in Brave on Windows, and his screenshots are the final visual gate. The
  preview link is how that check reaches him for v2. Isolation's `crossOriginIsolated` must therefore be checked in
  Brave too (R1-BND-REC5 pass mark).
- **Repository visibility.** Public (Question 5's legacy facts): anyone can fork and open a pull request (R5-PREV-F1,
  F5), and environment reviewers are on the Free plan (R5-PREV-F6).

### Question 7: the gate v2 that replaces today's gate (R5-GATE)


#### Today's gate (the repository, read 2026-10-28)
These are legacy facts, cited by file and line and by inventory row, not web findings.
- **What it checks** (tools/gate.mjs:2-17, the header; README "Check 5 in short"): [1] `node --check` on each engine
  file and inline script; [2] the same on each page's scripts joined in load order; [3] top-level symbols, base against
  head, nothing dropped (unless `--allow-drop` names it), no duplicates; [4] no unguarded reference from index.html's
  build to an editor-only symbol; [5a] the real pages in headless Chromium with real input, failing on any page error,
  console error, network request, WebSocket or file read outside the repo; [5b] every bare name resolves (eslint-scope)
  and every `THREE.<member>` exists in r128; [5c] `shamble-core.js` references nothing above it; [5d] the exact script
  order.
- **Why** (TL2, TL3, TL5, TL6, TL8, KT7): checks 2, 3, 4, 5b, 5c and 5d exist because the game is classic scripts sharing
  one global scope in a fixed order (a hard constraint); 5a exists because checks 1–4 passed while the editor threw three
  ReferenceErrors (gate-check-5 report, "Goal").
- **How it runs**: against a base ref, by default the newer of the merge-bases with `main` and `origin/main`
  (gate.mjs:71-89); in a fixed order with the slowest check last (`ORDER` at :194; "[5a] smoke (slowest, so last)" at
  :372); exit 0 PASS, 1 FAIL, 2 for a usage or setup error, including a smoke run with no Chromium when nothing else
  failed (:37, :213), never a silent skip (`--no-smoke` prints SKIP, :373); `--json` output.
- **Time**: about 20 s with the smoke test (up to about 35 s on a loaded machine), 2.5 s without it (tools/README.md);
  CI runs two jobs, `gate` (`--no-smoke`, 10 and 17 s) and `smoke` (57–59 s with a cached browser, 63–67 s uncached)
  (KT8, gate-check-5 report).
- **How it was proven**: 22 of 22 planted defects failed exactly the expected checks, in throwaway worktrees with AST
  edits, and controls (valid code that looks suspicious: valid handlers, a commented-out script tag, an endlessly
  animated Deploy button) passed (docs/reports/2026-10-14-gate-check-5.md, "Evidence"); the README rule: "Never edit the
  gate tools to make a check pass", and after a gate change the listed mutations must each fail.
- **What it does not cover** (START_HERE §5): logic and gameplay (sim.js and targeted tests), visuals (Brave), and
  anything the smoke script does not click.

#### Web findings
- **R5-GATE-F1. cargo-mutants inserts bugs and reports where no test fails.** Source:
  https://github.com/sourcefrog/cargo-mutants (README; no version shown) · official (the project) · read 2026-10-28 ·
  confirmed in the source: "Inject bugs and see if your tests catch them!"; it helps by "finding places where bugs could
  be inserted without causing any tests to fail."; one file at a time with `cargo mutants -f src/something.rs`.
- **R5-GATE-F2. Vite's chunk size limit only warns.** Source: https://vite.dev/config/build-options (v8.3.4) · official ·
  read 2026-10-28 · confirmed in the source: `build.chunkSizeWarningLimit` is the "Limit for chunk size warnings (in
  kB)." (default 500); `build.reportCompressedSize` "Enable/disable gzip-compressed size reporting." (default `true`).
  Consequence (inferred): a size budget that fails the build is a check of our own.
- **R5-GATE-F3. cargo-deny has four check classes; [the advisories check](GLOSSARY.md#the-advisories-check) reads an advisory database that also lists
  unmaintained crates.** Sources: https://embarkstudios.github.io/cargo-deny/ and
  https://embarkstudios.github.io/cargo-deny/checks/advisories/index.html (no version shown) · official (the tool's docs)
  · read 2026-10-28 · confirmed in the source: "cargo-deny supports several classes of checks" (advisories, bans,
  licenses, sources in the chapter list); advisories "is used to detect issues for crates by looking in an advisory
  database."; the database "also contains advisories for unmaintained crates." Consequence (inferred): its verdict can
  change on a day with no code change, when a new advisory is published.
- **R5-GATE-F4. cargo-deny only warns on duplicate crate versions and wildcard versions unless told to deny.** Source:
  https://embarkstudios.github.io/cargo-deny/checks/bans/cfg.html · official · read 2026-10-28 · confirmed in the source:
  `multiple-versions` "Determines what happens when multiple versions of the same crate are encountered." and
  `wildcards` "Determines what happens when a dependency is specified with the `*` (wildcard) version.", each with
  `deny`, `warn` (default) and `allow`.
- Findings of the other questions this part uses: R5-LAYOUT-F4, F6, F14, F15; R5-TSLINT-F4, F15; R5-RUSTLINT-F6, F9,
  F15; R5-TEST-F2, F3, F8, F9.

#### Legacy facts used
- **TL1** (redesign): keep the explicit base ref, exit 2 for setup and never a silent skip, `--json`, `--allow-drop` tied
  to the approved plan, mutation proofs for every gate change, and branch protection requiring the gate; the checks
  themselves do not carry.
- **TL2, TL3, TL5, TL6, TL8** (drop): compilers, modules and the bundler do their jobs; TL3's habit of an explicit
  "removed on purpose" list is worth keeping; TL6's reviewed globals allowlist becomes R5-TSLINT-REC2's restricted list.
- **TL4, TL7** (redesign): the player build never loads editor code; core layering grows into crate and package rules
  (R5-LAYOUT-REC2).
- **TL9** (redesign): the smoke shape carries; probes move to a test-hook API; the network rule flips.
- **TL10, KT9** (redesign): the per-edit hook (R5-TSLINT-REC4).
- **TL17** (redesign): "commit the mutation suite so gate v2 proves itself in one command".
- **KT7**: checks 2–5d exist only because of classic-script globals; keep the real-page smoke run, committed mutation
  tests and "never edit the gate to pass".
- **KT8**: CI hygiene (SHA pins, read-only token, `persist-credentials: false`, `--ignore-scripts`, concurrency
  cancel), required status checks on main since 2026-10-17.
- Wave 1 and wave 2 cited, not restated: R1-BND-REC4 (CI fails on a CLI-library mismatch; regenerated types give zero
  diff), R1-DATA-REC3 (CI fails when the committed schema differs), R1-DET-REC6, R1-ECS-REC2 mark 2, R1-DET-REC2 (a lint
  or build check fails on std transcendental calls), R1-RR-OQ5 (no Wasm size budget yet), R1-RR-OQ7 (cargo-audit and
  cargo-deny as CI gates, decided by R5's CI research and the synthesis), R2-TRN-REC4 marks 3 and 4, R3-MEASURE-REC3
  (CI gates counts: zero console errors, draw calls, triangles, programs, GPU memory totals, zero per-frame allocation in
  the render path; a 10 % rise fails; under 5 minutes), R3-MEASURE-OQ2 (reference machines in CI), R4's counting test for
  the static BVH and the reachability check.

## Options

### Question 1: repository layout (R5-LAYOUT)


#### Where the two workspaces sit
| Option | Shape | Complexity | Risk | Fit |
|---|---|---|---|---|
| A. One repo, both workspaces at the root | `Cargo.toml` (virtual) + `crates/*`; `package.json` + `packages/*`; `legacy/` | low: one checkout, one gate, one PR per change across the boundary | root holds two lockfiles and two toolchain files; tools must agree on paths | a change to a boundary type lands with both sides in one PR (R1-BND-REC4) |
| B. One repo, two roots | `rust/` (Cargo workspace) and `web/` (npm workspace) | low-medium: every command needs a `cd`; generated types cross a folder boundary | the same as A, plus path drift between roots | same as A |
| C. Several repos (sim, server, client) | one per part | high: versions of the Wasm build and the server must be matched across repos | version skew breaks the identical-results rule (R1-DET-REC6) and the CLI-library pin (R1-BND-F2) | poor |

#### Package manager for the TypeScript side
| Option | What we know | Cost | Risk |
|---|---|---|---|
| npm workspaces | read (F8); the legacy toolkit already uses npm with a lockfile and `npm ci` (TL17) | none new | the read page is marked "(Legacy)" for npm 11 |
| pnpm or Yarn workspaces | not read (pnpm.io, yarnpkg.com are outside the allow list) | a second tool on every machine | unknown |

#### Where generated types live
| Option | Trade-off |
|---|---|
| Committed into one TypeScript package, regenerated and diffed in CI | reviewable diffs; a TypeScript-only check runs without cargo; needs a CI diff check (R1-DATA-REC3 already chose this for the schema) |
| Generated at build time, never committed | no drift, but every TypeScript build and editor session needs the Rust toolchain; diffs of the boundary are invisible in review |

### Question 2: strict TypeScript and lint rules (R5-TSLINT)


#### Which TypeScript
| Option | Type check | Typed lint | Risk | Effort |
|---|---|---|---|---|
| A. TypeScript 6.0.x everywhere, exact pin | `tsc -b` (6.0) | typescript-eslint works (F4) | slower checker than 7; 6.0 is a transition release (F3) | low |
| B. TypeScript 7 for checking, 6.0 kept for typed lint | `tsgo` (7.0) | typescript-eslint on 6.0 | two compilers can disagree (F3's `--stableTypeOrdering` exists for 6-vs-7 differences); two pins | medium |
| C. TypeScript 7 plus another type-aware linter | 7.0 | a linter built for 7 (not read: outside the allow list) | unread; replaces typescript-eslint's rules | unknown |

#### Which lint configs
| Option | Catches | Cost |
|---|---|---|
| `recommended-type-checked` + `stylistic-type-checked` | correctness rules with types, `no-floating-promises` (F16) | stable under semver (F13) |
| `strict-type-checked` + `stylistic-type-checked` | more bug-catching rules (F13) | not semver-stable: a minor upgrade can add errors, so pin exactly and upgrade in its own change-set |
| `all` | everything | advised against (F13) |

#### Formatter
| Option | Fit | Risk |
|---|---|---|
| Prettier (F22) | the formatter ESLint's post names (F21); formats TS, JSON, Markdown, YAML | one more dev dependency |
| Biome (F23) | formatter and linter in one, 97 % Prettier-compatible | does not replace typed lint (not stated); a second linter would overlap ESLint |
| No formatter, lint rules only | none | ESLint deprecated its formatting rules (F21) |

### Question 3: clippy and rustfmt (R5-RUSTLINT)


| Option | Shape | Catches | Cost and risk |
|---|---|---|---|
| A. Defaults plus `-D warnings` | clippy's default groups, every warning an error in the gate | correctness, suspicious, style, complexity, perf | nothing for determinism or panics |
| B. A. plus a workspace lint table and per-crate tiers | `[workspace.lints]` for all crates; a `clippy.toml` and extra `[lints]` in `sim`, `protocol`, `content` | determinism (F7–F9), panics in decoders and the content loader (F11, F12) | per-crate lint lists to keep; `restriction` picks need reviewed `#[allow]`s with reasons |
| C. B. plus `pedantic` everywhere | as B, plus pedantic | more style and edge cases (F2) | noise in tools and tests; under `-D warnings` every pedantic warning blocks |

For float use, a ban on all float arithmetic (a "no float" lint) does not fit: wave 1 keeps IEEE floats under rules
(R1-DET-REC7), so the lints target the banned functions, not floats.

### Question 4: tests (R5-TEST)


#### Property tests and fuzzing
| Option | Read? | Fit | Cost |
|---|---|---|---|
| proptest | yes (F2) | shrinking, committed regression files, seedable | a dev-dependency |
| quickcheck, bolero | no (not fetched in this run) | unknown | – |
| cargo-fuzz for decoders | yes (F4) | coverage-guided, for the bytes clients send | nightly; Linux CI only |

#### Where browser tests run
| Option | Browsers | Fit | Risk |
|---|---|---|---|
| Playwright Test | Chromium, Firefox, WebKit (F8) | end-to-end client plus local server (`webServer`, F9); TL9's shape | WebKit is not Safari (F8) |
| Vitest Browser Mode | via Playwright or WebdriverIO (F6) | component and boundary tests inside a page | maturity not stated (F6) |
| wasm-bindgen-test | Node by default; Chrome, Firefox, Safari by WebDriver (F5) | Rust unit tests compiled to Wasm, unchanged | the guide read may be older than the crate |
| Real Safari on macOS (wasm-bindgen-test through safaridriver, F5; or WebdriverIO, which lists Safari among its browsers, F6) | Safari | the only way to test Safari itself; Playwright cannot drive it (F5, F6, F8) | a macOS runner (cost: R5-CI, R6) |

#### Network tests
| Option | What it simulates | Deterministic | Covers the real transport | Cost |
|---|---|---|---|---|
| L1. Our own link simulator behind the transport interface (R2-TRN-REC4) | per-direction latency, jitter, loss, reordering, duplication, rate limit (F12's set) on simulated time | yes, seeded | no (no QUIC, no browser) | small, our code |
| turmoil (F11) | Tokio TCP and UDP with latency and message failure | yes, seeded | QUIC on it not stated | depends on quinn running over it (unverified) |
| L2. An impairment socket under the real quinn endpoint (F13), GNS-style (F12) | the same set, on the server's UDP | no (real time, real OS) | yes, for WebTransport from real browsers | small; WebSocket (TCP) needs a separate proxy |
| OS-level impairment (Linux `tc netem`) | kernel-level delay, loss | no | yes | its docs are outside the allow list (not read) |

### Question 5: CI with Rust caching (R5-CI; infra-engineer)


**Rust build cache**

| Option | Complexity | Cost | Speed | Risk | Fit |
|---|---|---|---|---|---|
| A. No cache | none | $0 | every run builds all dependencies from scratch (time not measured) | none from caches | serves no §6 item; slow feedback |
| B. Swatinem/rust-cache v2.9.3, one shared key per build kind (native, wasm32), saved only from `main` | low: one step per job | $0 within 10 GB (F5) | restores dependency builds; our own crates always rebuild (F3) | poisoning via PRs (F6), limited by saving only on `main`; eviction past 10 GB (F5) | serves fast feedback for every §6 item; adds no engine feature |
| C. sccache v0.19.0 with the GHA backend | medium: a wrapper and environment variables | $0 within 10 GB | per-crate hits, but none for cdylib, bin, proc-macro or incremental crates (F9) | cache entries count against F7's rate limits (inferred: a cache entry per compiled crate) | as B |
| D. B and C together | higher | $0 within 10 GB | possibly more hits on dependency changes (unmeasured) | two caches inside one 10 GB budget | only if measurement shows B is not enough |

**Toolchain and CLI pinning**

| Option | Trade-off |
|---|---|
| T1. `rust-toolchain.toml` with an exact `x.y.z` channel, `targets = ["wasm32-unknown-unknown"]`, components clippy and rustfmt, `profile = "minimal"`; CI runs `rustup toolchain install` (F12, F13) | one file for local, CI and every agent; bumps are explicit PRs; the cache rolls over on each bump (F2) |
| T2. `stable` channel | no pin: a new Rust release changes the build silently; breaks "identical results everywhere" checks (R1-DET-REC6) between runs on different days |
| W1. wasm-bindgen CLI through taiki-e/install-action at the Cargo.lock version, `fallback: none` (F14, F15) | prebuilt and checksum-checked; one more third-party action to pin |
| W2. `cargo install wasm-bindgen-cli --version <lock> --locked`, cached in `~/.cargo/bin` (rust-cache `cache-bin`, F1, F2) | no extra action; builds from source on every cache miss (time not measured; `cargo install` docs not read) |

**How CI runs the gate v2**

| Option | Trade-off |
|---|---|
| G1. One workflow on push and pull_request, parallel jobs (Rust native, Rust wasm32 plus generated types, TypeScript, Playwright end-to-end, determinism), plus one final job that is the only required check | one stable required check name; jobs skip by condition, never by path filter (F20); parallel jobs make wall time the longest job, not the sum |
| G2. Several workflows with path filters, each required | path-filtered workflows leave required checks "Pending" forever (F20): rejected |
| G3. One serial job | simplest; wall time is the sum of every step |

### Question 6: preview links per PR (R5-PREV; infra-engineer)


**Workflow shape (whatever the host)**

| Option | Complexity | Risk | Effort | Fit |
|---|---|---|---|---|
| P1. Build in a `pull_request` job with no secret, upload `dist` as an artifact; a second job (`needs: build`) in an environment `preview` holds the host token, downloads the artifact, uploads it to the host and posts the link. Only same-repository PRs deploy; forks get CI but no preview | low | PR code never runs where the token is; the deploy job runs only our pinned tools on static files (F1, F6) | small | Mustafa's own branches get links; forks rarely matter today |
| P2. Like P1, but forks get a preview after a reviewer's approval of the `preview` environment | low-medium | the approver must read the diff first (F5); static files from a stranger then sit on our preview domain (phishing risk, inferred) | small | only once outside contributors appear |
| P3. `workflow_run` deploys every PR's artifact, forks included | medium | gets secrets and write tokens (F3); a mistake lets PR code run with secrets | medium | not needed: rejected for now |
| P4. `pull_request_target` | low | GitHub warns against building PR code with it (F2) | small | rejected |
| P5. The host builds from Git itself (Netlify, Vercel, Cloudflare Git integration) | low on our side | the host builds fork code on its servers (Netlify guards secrets by default, F16); the host's build image needs our pinned Rust, wasm-bindgen and Node versions, and CI and the host build separately, so the preview is not the artifact CI tested (inferred) | medium | breaks "the preview is the tested build" |

**Hosts considered (technical fit only; limits and prices are R6's)**

| Host | Deploy from CI | Per-PR link | Private option | `noindex` by default | Custom headers for COOP/COEP | Cleanup |
|---|---|---|---|---|---|---|
| Cloudflare Pages | wrangler-action v4, `pages deploy` (F10, F11) | branch alias (F12) | access policy, Cloudflare account (F12) | yes (F12) | `_headers`, host-scoped (F18) | delete by id; a branch's latest deployment cannot be deleted (F13) |
| Cloudflare Workers static assets | `versions upload --preview-alias` (F14) | alias URL (F14) | Cloudflare Access (F14) | not stated | `_headers` (R6's notes) | the 1000 most recent kept (F14) |
| Netlify | Git integration (F15); CLI deploy not read | `deploy-preview-N` (F15) | password (F15) | yes (F15) | `_headers` (R6's notes: global, not per context) | not read |
| Vercel | `vercel deploy --prebuilt` (F17) | preview URL | Deployment Protection (R6's notes) | not read | `vercel.json` headers (R6's notes) | not read |
| GitHub Pages | Actions (Vite guide) | no per-PR site natively (inferred) | none read | not read | none found (R6's notes) | not read |

### Question 7: the gate v2 that replaces today's gate (R5-GATE)


#### How the gate is run
| Option | Shape | Fit | Risk |
|---|---|---|---|
| A. One Node orchestrator (working name `node tools/gate.mjs`, v2 edition; its path is R5-GATE-OQ3) that runs named checks and calls cargo, tsc, ESLint, Vitest and Playwright | continuity with today's command, base ref, exit codes and `--json`; Node is needed anyway for Vite; works on Windows | one more script to keep (with its own mutation proofs); like B, its setup and Rust checks (G0–G2) need the Rust toolchain |
| B. `cargo xtask gate` (a Rust binary) | one language for the gate and the sim | the gate itself must compile before any check runs, and today's Node conventions are rewritten in Rust (inferred); like A, it needs the Rust toolchain |
| C. A generic task runner (make, just) | familiar to many | not read; make is not native on Windows; no exit-code or base-ref conventions |
| D. CI only (no local gate) | nothing local to keep | breaks CLAUDE.md's protocol (the gate must PASS before commit) and slows every step |

#### When long checks run
| Option | Trade-off |
|---|---|
| Everything on every PR | the strongest guard; PR wall time grows with soak, fuzz and Safari |
| Fast set on every PR, long set nightly and before playtests | PR stays inside the CI budget (R5-CI-REC5); a long-run failure lands a day later, as an issue against main |
| Path-based selection inside the gate (checks pick themselves by the files a change touches) | saves time on TypeScript-only PRs; the selection rule itself must be tested (a wrong rule skips a check silently) |

## Recommendations

### Question 1: repository layout (R5-LAYOUT)


- **R5-LAYOUT-REC1. One repository with both workspaces at its root (option A), npm workspaces, and pinned toolchains.**
  Root `Cargo.toml` is a virtual manifest with `resolver = "3"`, `edition = "2024"` inherited through
  `[workspace.package]`, a `rust-version` equal to the pinned toolchain, and all shared dependency versions in
  `[workspace.dependencies]`; `rust-toolchain.toml` pins the exact stable release (1.99.1 today) with `clippy`,
  `rustfmt` and the `wasm32-unknown-unknown` target; root `package.json` lists `packages/*` as npm workspaces with one
  `package-lock.json` and exact versions; Node is pinned the way the legacy toolkit pins 22 (TL17). `legacy/` sits beside
  `crates/` and `packages/`.
  Reason: one lockfile and one build folder per side (F1), inherited versions so the sim, server and Wasm crates cannot
  drift apart (F2), a resolver that prefers versions our pinned Rust supports (F3), and a toolchain file every machine
  and CI honour (F6); npm is the only read option and is already in use (F8, TL17); several repositories would break the
  version matching wave 1 requires (R1-BND-F2).
  Rests on: R5-LAYOUT-F1, F2, F3, F5, F6, F8; legacy TL17; R1-BND-F2 (cited).
  Serves: none directly (toolchain); each crate and package in REC2 names its VISION_V2 §6 item.
  A spike must prove (proposed): from a fresh clone on the CI runner, `cargo test --workspace --locked` and
  `npm ci && npm test --workspaces` pass with no network access after the install steps; `rustc --version` in CI equals
  the pin; deleting the toolchain file makes the gate's setup check fail with exit 2 (TL1), never silently use another
  Rust.
- **R5-LAYOUT-REC2. Start with only the crates and packages a §6 item needs, and enforce who may depend on whom.**
  Crates: `sim` (the simulation; serves §6.1, §6.2, §6.3, §6.4, §6.5), `content` (loads and validates content data,
  R1-DATA-REC2; §6.3, §6.7), `protocol` (message types and decoders that never panic, R2-TRN-REC4; §6.1), `sim-wasm`
  (the wasm-bindgen bindings the client's Worker loads; §6.2, through the client), `server` (the binary with transport
  and rooms; §6.1, §6.5, §6.6, §6.8), and `devtools` (replay runner, hash comparer, legacy-golden reader; §6.6 needs
  re-runnable matches, R1-DET-REC6). Fuzz targets live in `fuzz/`, outside the workspace's default members, because they
  need a nightly compiler (R5-TEST-F4). TypeScript packages: `client` (player entry), `editor` (its own entry,
  EDITOR_V2 §5 via TL4), `net` (WebTransport and WebSocket backends, R2-TRN-REC4), `sim-worker` (hosts `sim-wasm`), `types`
  (generated types, REC3), `e2e` (Playwright tests). No other crate or package until a §6 item asks for it.
  Dependency rules, each a gate check (R5-GATE): `sim` depends on `content` only (no transport, no async runtime, no
  wall clock), checked with cargo-deny's `deny` plus `wrappers` and with `cargo tree` (R2-TRN-REC4 mark 3); `content`
  depends on nothing above it; `client` never reaches `editor`, checked twice: an ESLint `no-restricted-imports` rule
  (R5-TSLINT-REC2) and a walk of Vite's manifest from the player entry through `imports` and `dynamicImports`.
  Reason: wave 1's boundaries (R1-DATA-REC2, R2-TRN-REC4, TL4, TL7) need a mechanical check; project references alone do
  not promise to block an import (F13); the built manifest shows what the player page really loads (F15), and a lint
  rule catches it before the build (R5-TSLINT-F14).
  Rests on: R5-LAYOUT-F9, F13, F14, F15; R5-TSLINT-F14; R5-TEST-F4; legacy TL4, TL7, KT13; R1-DATA-REC2, R2-TRN-REC4
  (cited).
  Serves: §6.1–§6.8 as listed per crate; the editor split serves §6.7 (the faction content pipeline is authored there).
  A spike must prove (proposed): each planted violation fails the gate on the right check and nothing else: `use` of the
  transport crate inside `sim`; a `tokio` dependency added to `sim`; an `import` of an `editor` module in `client`; a
  dynamic `import()` of the editor in `client` (only the manifest walk sees a path built at run time); controls (a
  legal import of `types` in `editor`) pass.
  Cross-topic open questions: R5-XT-OQ3, R5-XT-OQ7 (below).
- **R5-LAYOUT-REC3. Generated TypeScript types live in one committed package, `packages/types`, written by `cargo test`
  and diffed in CI.** ts-rs exports into `packages/types/src/generated/` through `TS_RS_EXPORT_DIR` in
  `.cargo/config.toml`; the JSON Schema of R1-DATA-REC3 and the output-block layout of R1-BND-REC4 (D4) write to the same
  package; wasm-bindgen's own `.d.ts` and glue are build output (ignored, KT19), produced by a CLI pinned to the library
  version (R1-BND-F2). The gate regenerates and fails on any difference.
  Reason: ts-rs writes on `cargo test` and its folder is one setting (F7); one committed place makes every boundary
  change visible in review, and R1-DATA-REC3 already commits the schema with a CI diff check.
  Rests on: R5-LAYOUT-F7; R1-BND-REC4, R1-BND-F2, R1-DATA-REC3 (cited).
  Serves: §6.3 and §6.7 (content and abilities typed from one definition on both sides).
  A spike must prove (proposed): renaming a field of an exported Rust type without regenerating fails the gate; a hand
  edit to a generated file fails the gate; two runs of the generator on the same commit give byte-identical files.
  Cross-topic open questions: R5-XT-OQ9 (below).
- **R5-LAYOUT-REC4. The sim crate is built with one feature set on every target, and the gate checks it.** The `sim`
  crate exposes no feature that changes maths; dependencies it shares with `server` get their maths-relevant features
  set once in `[workspace.dependencies]`; the determinism suite (R5-TEST-REC2) tests the native sim built as the server
  builds it, not as a stand-alone crate.
  Reason: Cargo unifies features over everything built together (F4), so the server's choices can change the native
  sim; wave 1 already flags target-dependent maths in a dependency (R1-RR-OQ1) and forbids non-deterministic float paths
  (R1-DET-REC2).
  Rests on: R5-LAYOUT-F4; R1-DET-REC2, R1-RR-OQ1 (cited).
  Serves: §4 (fair to about 100 ms of ping: prediction and replay need an identical sim on client and server, as
  R1-ECS-REC2 traces it), §6.6 and §6.8 (re-runnable matches).
  A spike must prove (proposed): the resolved feature list of every dependency of `sim` (from `cargo tree -e features`)
  is identical for the `server` build and the `sim-wasm` build; a planted extra feature on a shared dependency in
  `server` makes the check fail.
- **R5-LAYOUT-REC5. Vite: two HTML entries, the Worker by URL, and isolation headers behind a flag.** `client` and
  `editor` are separate HTML entries in one Vite config (or one config each); the Worker that hosts the sim loads with
  `new Worker(new URL(...), import.meta.url)`; `server.headers` and `preview.headers` send COOP `same-origin` and COEP
  `require-corp` only when an isolation flag is set, as R1-BND-REC5 asks for development and previews; `tsc -b` runs
  beside every Vite build.
  Reason: Vite builds several pages (F9), recommends that Worker form (F11), sets headers per server (F12) and does not
  type-check (F10).
  Rests on: R5-LAYOUT-F9, F10, F11, F12; R1-BND-REC5 (cited).
  Serves: §6.2, §6.9 and §4 (the sim in a Worker on the client, as R1's option A2 traces it: the Worker carries the 3D
  sim and keeps [sim step time](GLOSSARY.md#sim-step-time) off the render thread), §6.7 (the editor entry).
  A spike must prove (proposed): with the flag, `crossOriginIsolated === true` in Chromium, Firefox and WebKit under
  Playwright on both `vite` and `vite preview`; without it, `false`; the built player page loads no file from `editor`
  (REC2's manifest walk).
  Cross-topic open questions: R5-XT-OQ8 (below).
- **R5-LAYOUT-REC6. The phase 1 move keeps the legacy oracle runnable.** The five game files move to `legacy/`; `tools/`
  (preview.js, sim.js, battery.js, tuning-dump.js, the gate) moves with them or learns the new path (input to the phase
  1 plan, which decides it, R5-LAYOUT-OQ5; nothing here changes today's tools), so today's gate keeps guarding critical
  fixes to legacy and the legacy oracle can still produce golden values (R5-TEST-REC3) until legacy retires;
  `.gitignore` adds `target/`, `pkg/`, `dist/`, `coverage/`, `playwright-report/`, `test-results/` and proptest's and
  insta's pending files.
  Reason: legacy stays frozen with critical fixes only, so it still needs its gate; the golden tests need it to run
  (TL11); KT19 asks for the ignore list to follow this layout; one `target` folder per workspace (F1).
  Rests on: R5-LAYOUT-F1; legacy TL11, KT19, KT21, TL1.
  Serves: none directly (it protects the legacy build and the golden data for §6.3's tuning tables).
  A spike must prove (proposed): after the move, today's gate run on `legacy/` passes, and the four Phase 0 mutation
  proofs (a deleted function, an unguarded editor call, a syntax error, a cross-file duplicate) still fail on the right
  checks; `node tools/preview.js legacy eval tools/tuning-dump.js` reproduces `docs/legacy-tuning.json` byte for byte.

### Question 2: strict TypeScript and lint rules (R5-TSLINT)


- **R5-TSLINT-REC1. TypeScript 6.0.x, pinned exactly, with one shared strict base tsconfig.** Base options: `strict:
  true` (written out even though 6.0 defaults to it, so the intent survives a default change), `noUncheckedIndexedAccess`,
  `exactOptionalPropertyTypes`, `noImplicitOverride`, `noFallthroughCasesInSwitch`, `noImplicitReturns`,
  `isolatedModules`, `verbatimModuleSyntax`, `erasableSyntaxOnly`, and the `lib` and `types` each package needs set
  explicitly (6.0's `types` default is empty, F2). Every package extends the base and is a project reference built with
  `tsc -b`. Move to TypeScript 7 in its own change-set once typescript-eslint supports it (F5), so one compiler does both.
  Reason: typed lint needs TypeScript below 6.1 (F4, F5), and one compiler avoids two checkers disagreeing (F3); each
  flag closes a gap named in F6–F12 and F24–F25; `erasableSyntaxOnly` and `isolatedModules` keep files safe for Vite's
  per-file transpile (R5-LAYOUT-F10).
  Rests on: R5-TSLINT-F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F24, F25; R5-LAYOUT-F10, F13.
  Serves: none directly (toolchain); it protects the client side of §6.1–§6.9.
  A spike must prove (proposed): one planted violation per flag (an unchecked index read, `x = undefined` into an
  optional field, a missing `override`, a fall-through `case`, a path without `return`, an `enum`, a value import used
  only as a type) each fails `tsc -b`; the clean tree passes; `tsc -b` on the whole client takes under 30 s on the CI
  runner, warm.
- **R5-TSLINT-REC2. ESLint 10 flat config with typescript-eslint `strict-type-checked` plus `stylistic-type-checked`,
  exact pins, and four project rules.** Typed linting through `projectService: true`; `--max-warnings 0` in the gate.
  Rules added on top: `@typescript-eslint/switch-exhaustiveness-check` with `considerDefaultExhaustiveForUnions: false`;
  `no-restricted-imports` patterns for the layering rules (`client` ↛ `editor`; `sim-worker` and `net` ↛ rendering and
  DOM packages; nothing ↛ `legacy/`); `no-restricted-globals` with a reviewed list (`event`, `name`, `status` and the
  rest of TL6's lesson, each with a message); `no-undef` off. `no-floating-promises` comes with the config (F16).
  Reason: strict catches more bugs (F13) and the code is written by agents that work in TypeScript every day, so the
  proficiency caveat matters less; strict is not semver-stable, so the pin is exact and upgrades are their own
  change-set (F13); exhaustive switches catch an unhandled generated message kind (F17); the import rule enforces the
  layering before any build (F14); restricted globals keep the TL6 lesson (F19); TypeScript already does `no-undef`'s
  job (F18).
  Rests on: R5-TSLINT-F4, F13, F14, F15, F16, F17, F18, F19, F20; legacy TL4, TL6, TL7.
  Serves: §6.1 (a missed message kind or an unhandled transport promise is a netcode bug on the client); otherwise
  toolchain.
  A spike must prove (proposed): planted violations fail the lint, one per rule (an unawaited promise, a switch missing
  one generated kind while having a `default`, an import of `editor` from `client`, a bare `event`); the clean tree has 0
  errors and 0 warnings; typed lint of the whole TypeScript tree takes under 60 s on the CI runner, warm (F15 says small
  projects take seconds).
- **R5-TSLINT-REC3. Prettier formats the TypeScript packages and their JSON; the gate runs it in check mode.** Scope:
  `packages/**` and the root config files, not `docs/` (reflowing reviewed prose would bury content changes) and not
  `legacy/` (frozen). Formatting stays out of ESLint.
  Reason: ESLint's own advice (F21); Prettier is the formatter it names and covers our file types (F22); Biome would add
  a second linter beside the typed one without type-aware rules on record (F23).
  Rests on: R5-TSLINT-F21, F22, F23.
  Serves: none (toolchain).
  A spike must prove (proposed): `prettier --check` fails on a planted misformat; after `prettier --write` on the tree,
  ESLint reports 0 problems (no rule fights the formatter); line endings are LF (KT19).
- **R5-TSLINT-REC4. The per-edit hook checks only what changed, fast.** On a `.ts` edit: `tsc -b` on the touched
  project (incremental) and ESLint on the file; on an `.rs` edit: `cargo check -p` on the crate (R5-RUSTLINT); exit 2 on
  failure, as today's syntax hook does. The full gate stays the only judge.
  Reason: TL10 and KT9 keep instant feedback; Vite does not type-check (R5-LAYOUT-F10); typed lint costs a project build
  (F15), so the hook must stay per project.
  Rests on: R5-TSLINT-F15; R5-LAYOUT-F10; legacy TL10, KT9.
  Serves: none (toolchain).
  A spike must prove (proposed): a planted type error in one file is reported by the hook within 10 s, p95 over 20
  edits, on Mustafa's Windows machine and on Linux; the hook path works from any working folder (KT9's fail-open
  relative path is fixed).

### Question 3: clippy and rustfmt (R5-RUSTLINT)


- **R5-RUSTLINT-REC1. One workspace lint table, every crate opts in, and the gate turns warnings into errors (option
  B).** `[workspace.lints.rust]`: `unsafe_code = "deny"` (an exception only in the module R1-RR-OQ6 may need, with a
  written invariant, a test and an `#[allow]` that gives its reason). `[workspace.lints.clippy]`: `all = { level =
  "warn", priority = -1 }`. The gate runs `cargo clippy --workspace --all-targets --locked -- -D warnings`, once for the
  host and once with `--target wasm32-unknown-unknown` for the crates that build for the browser. `pedantic` goes only on
  `sim`, `protocol` and `content`, at warn, which under the same `-D warnings` fails the gate there: each hit is fixed
  or allowed with a reason.
  Reason: only correctness denies by default (F1), so without `-D warnings` the rest only prints (F6); one table keeps
  the crates alike (F3, F4); pedantic is worth its noise in the outcome-deciding crates only (F2); `cfg` differences
  between targets mean one clippy run cannot see both builds (inferred).
  Rests on: R5-RUSTLINT-F1, F2, F3, F4, F6; R1-RR-OQ6 (cited).
  Serves: none directly (toolchain).
  A spike must prove (proposed): a planted correctness bug, a planted `style` warning and a planted `unsafe` block each
  fail the gate; a crate whose `Cargo.toml` lacks `[lints] workspace = true` fails a gate check of its own; clippy on the
  whole workspace takes under 3 min cold and under 30 s warm on the CI runner.
- **R5-RUSTLINT-REC2. The determinism lint set for `sim` (and for the parts of `content` the sim calls).** In
  `crates/sim/clippy.toml`: `disallowed-types` = `std::collections::HashMap`, `std::collections::HashSet` (replacement
  `BTreeMap` or an index-ordered `Vec`), `std::time::Instant`, `std::time::SystemTime`; `disallowed-methods` =
  `std::time::Instant::now`, `std::time::SystemTime::now`, std's transcendental float methods and the `algebraic_*`
  methods (if clippy can name them: OQ1), each with a `reason` that points to R1-DET-REC2 or KT13. In `sim`'s
  `[lints.clippy]`: `disallowed_types` and `disallowed_methods` at `deny`, `iter_over_hash_type = "deny"` as a backstop,
  `suboptimal_flops` and `imprecise_flops` explicitly `allow` with a comment, and never `cargo clippy --fix` on `sim`.
  The `relaxed-simd` target feature is not a lint: the gate reads the sim's build flags and fails if it appears.
  Reason: `iter_over_hash_type` alone misses iterator chains (F9), so the type ban is the main check for R1-ECS-REC2's
  mark 2 and the `for` lint only a second line; `Instant::now` is clippy's own example of a bannable function (F7) and is
  exactly KT13's wall-clock rule; the per-crate `clippy.toml` keeps the bans out of the server and tools, which need
  clocks (F5); the two float lints would rewrite sim maths into fused or banned calls (F10).
  Rests on: R5-RUSTLINT-F5, F7, F8, F9, F10; legacy KT13, TL13; R1-DET-REC2, R1-ECS-REC2 (cited).
  Serves: §4 (fair to about 100 ms of ping: prediction and replay need one sim on client and server, as R1-ECS-REC2
  traces it), §6.6 and §6.8 (telemetry and moments computed from re-runnable matches).
  A spike must prove (proposed): each planted violation fails the gate on clippy, with the reason in the message: a
  `HashMap` field in a sim struct; `map.values().sum()` over a std `HashMap` (missed by F9's lint, caught by the type
  ban); `Instant::now()` in a tick function; `x.sin()` on an `f32` (if OQ1 says clippy cannot name it, the fallback
  check of OQ1 must catch it); the same code in `server` passes (the bans are per crate).
- **R5-RUSTLINT-REC3. No implicit panics in the code that reads untrusted bytes.** In `protocol` (decoders) and in
  `content`'s loader: `unwrap_used`, `expect_used` and `indexing_slicing` at `deny`, with `allow-unwrap-in-tests`,
  `allow-expect-in-tests` and `allow-indexing-slicing-in-tests` set to `true` in those crates' `clippy.toml`. The sim
  does not enable these three restriction lints (restriction lints are picked one by one, F1), since its indices come
  from its own pools; enabling them at `warn` there would, under REC1's `-D warnings`, fail the gate on every index
  read.
  Reason: wave 1 requires decoders that never panic and fuzzing with zero panics (R2-TRN-REC4, R1-RR-OQ3); these lints
  find the common panic sources at review time (F11, F12), and the fuzzing (R5-TEST-REC4) finds the rest; tests may
  unwrap (F11).
  Rests on: R5-RUSTLINT-F1, F11, F12; R2-TRN-REC4, R1-RR-OQ3 (cited).
  Serves: §6.1 (rooms and private matches stay up when a client sends bad bytes).
  A spike must prove (proposed): a planted `.unwrap()` and a planted `buf[i]` in a decoder fail clippy; the same in a
  `#[test]` pass; R2-TRN-REC4's fuzz mark (10 million inputs per message type, 0 panics) holds.
- **R5-RUSTLINT-REC4. One overflow rule on every target: overflow checks on for `sim` and `protocol` in every profile,
  and explicit `wrapping_*` or `checked_*` for counters.** `[profile.release.package.sim] overflow-checks = true` (and the
  same for `protocol`); counters and tick numbers use explicit operations, as R1-DET-REC2 already asks.
  Reason: dev and release differ by default (F13), so a debug client and a release server can disagree or one can panic
  (R1-RR-OQ4); a per-package override is allowed for this setting (F13, inferred), so the rest of the workspace keeps the
  release default.
  Rests on: R5-RUSTLINT-F13; R1-RR-OQ4, R1-DET-REC2 (cited).
  Serves: §4 (client and server must not disagree on a tick, as R1-ECS-REC2 traces determinism), §6.6 and §6.8
  (re-runnable matches).
  A spike must prove (proposed): a planted `u16` counter that overflows with `+` panics in the dev and the release test
  builds, native and Wasm alike; the same counter with `wrapping_add` gives the same value on all four; the checks' cost in
  sim step time is reported with method (R1-TICK's bench), and the ADR sets the limit.
- **R5-RUSTLINT-REC5. rustfmt with stable options only, and the gate in check mode.** `rustfmt.toml` at the root:
  `edition = "2024"`, `style_edition = "2024"`, `newline_style = "Unix"`; everything else default (`max_width` 100). No
  unstable options, so the formatter runs on the pinned stable toolchain. The gate runs `cargo fmt --all --check`.
  Reason: both editions default to 2015 when not set (F14), so writing them pins the style; `newline_style` carries
  KT19's LF rule for Mustafa's Windows machine; the import-grouping options are unstable (F14).
  Rests on: R5-RUSTLINT-F14; legacy KT19.
  Serves: none (toolchain).
  A spike must prove (proposed): a planted misformat and a file saved with CRLF endings each fail `cargo fmt --all
  --check`; the formatted tree is byte-identical on Windows and Linux.
- **R5-RUSTLINT-REC6. The toolchain pin includes clippy and rustfmt; a toolchain upgrade is its own change-set.** The
  change-set reports the new warnings and fixes them or allows them with reasons.
  Reason: lints arrive with releases (F15) and `-D warnings` turns them into failures; the pin (R5-LAYOUT-REC1) keeps
  the verdict stable between runs.
  Rests on: R5-RUSTLINT-F15; R5-LAYOUT-F6.
  Serves: none (toolchain).
  A spike must prove (proposed): CI prints `cargo clippy --version` and `rustfmt --version` equal to the pin; the same
  commit gives the same lint result on two CI runs a week apart.

### Question 4: tests (R5-TEST)


- **R5-TEST-REC1. The layers, the tool for each, and where each runs.**
  | Layer | Tool | What it proves | Runs |
  |---|---|---|---|
  | Rust unit and property | `cargo test`, proptest 1.x | sim rules, maths, decoders, content validation | gate: every PR |
  | Rust in Wasm | wasm-bindgen-test in Node | the same unit tests pass when compiled to Wasm | gate |
  | Rust ↔ TypeScript boundary | Vitest (Node) loading the built `sim-wasm` | the layout hash of R1-BND-REC4, every message type round-trips between Rust encoders and TypeScript decoders | gate |
  | TypeScript unit | Vitest (Node) | client logic that is not rendering: input mapping, interpolation buffers, HUD state | gate |
  | Browser end-to-end | Playwright Test (Chromium, Firefox, WebKit) with the built client and a local server via `webServer` | TL9's smoke shape on v2, both transport backends (R2-TRN-REC4 mark 1), counts of R3-MEASURE-REC3 | gate |
  | Safari | on macOS: the e2e suite in Playwright's WebKit, the closest stand-in Playwright offers (F8); the Rust-in-Wasm and determinism tests in real Safari through safaridriver (wasm-bindgen-test, F5) or WebdriverIO (Vitest Browser Mode, F6); no runner for the e2e suite in real Safari was read (R5-XQ-OQ1) | what WebKit on Linux cannot show (F8) | for PRs touching `client`, `net`, `sim-worker` or `sim`, else nightly (cost: R5-CI) |
  | Legacy golden | `cargo test` reading committed fixtures (REC3) | the behaviour v2 keeps equals legacy within stated tolerances | gate |
  | Determinism | REC2's suite | identical per-tick hashes on every target | gate (short), nightly (long) |
  | Network | REC4's two layers | R2's pass marks under named profiles | gate (L1 short), nightly (L1 long, L2) |
  | Soak and load | REC5 | [48+](GLOSSARY.md#the-design-point-and-headroom) players with bots, hours | nightly and before each playtest |
  | Fuzz | cargo-fuzz on decoders and the content loader | 0 panics, 0 over-limit allocations | nightly, Linux |
  | Benchmarks | R1-TICK's tick bench at N players and a client render bench (TL15); the tool was not read (R5-TEST-OQ8) | sim step time and frame cost, reported with method (R5-RUSTLINT-REC4's overflow cost, REC5's [server tick time](GLOSSARY.md#server-tick-time) [p99](GLOSSARY.md#time-statistics)) | on change-sets that touch the sim or the render path; never a gate pass or fail (R3-MEASURE-REC3: CI gates counts, not timings) |
  Reason: each tool is the read option for its layer (F1, F2, F4–F9); WebKit stands in for Safari only partly, and
  Playwright cannot drive real Safari, so Safari's own runs go through its WebDriver (F5, F6, F8); benchmarks report
  numbers and never decide the gate (R3-MEASURE-REC3).
  Rests on: R5-TEST-F1, F2, F4, F5, F6, F7, F8, F9; legacy TL9, TL15; R1-BND-REC4, R1-TICK-REC2, R2-TRN-REC4,
  R3-MEASURE-REC3 (cited).
  Serves: §6.1–§6.9 through their tests; §4 (four desktop browsers).
  A spike must prove (proposed): one planted defect per layer (the benchmark layer excepted: it never decides the gate)
  fails that layer and is caught by the gate or nightly job
  the table names (for example: a byte-order swap in one encoder fails the boundary layer only; a page error in deploy
  fails e2e only); every layer's clean run passes 20 times in a row on one commit.
  Cross-topic open questions: R5-XT-OQ2, R5-XT-OQ13 (below).
- **R5-TEST-REC2. The determinism suite's tools (R1-DET-REC6 asks R5 to pick them).** A `devtools` replay runner (native,
  release, built as the server builds the sim, R5-LAYOUT-REC4) plays a recorded match and writes one hash per tick; the
  gate runs it twice in two processes and once as Wasm in Node (the same runner compiled to Wasm, or wasm-bindgen-test);
  Playwright opens a replay page that runs the `sim-wasm` Worker in Chromium, Firefox and WebKit and returns the hash
  stream; on macOS the same replay runs in real Safari through its WebDriver, as the replay runner compiled with
  wasm-bindgen-test's `run_in_browser` (F5) or as a Vitest Browser Mode page under WebdriverIO (F6), since Playwright
  cannot drive branded Safari (F8) (REC1's rule). A comparer reports the first mismatching tick and a field-level diff.
  Gate set: three short recordings (proposed: 2 minutes each at the tick rate R1-TICK picks, one per mode: two teams,
  three teams, bots only); nightly: R1-ECS-REC2 mark 1's 10-minute bot match and every recording added since.
  Reason: native and Wasm in Node are cheap and catch most divergence; the browsers must be in the matrix because their
  engines differ (R1-DET-REC6's reason); Playwright drives three engines but not branded Safari (F8), and real Safari
  is reached through safaridriver (F5) or WebdriverIO, which lists Safari among its browsers (F6).
  Rests on: R5-TEST-F5, F6, F8; R5-LAYOUT-F4; R1-DET-REC6, R1-ECS-REC2 (cited).
  Serves: §4 (fair to about 100 ms of ping: prediction and replay need the same ticks on client and server, as
  R1-ECS-REC2 traces it), §6.6 and §6.8 (re-runnable matches); §3 ([the design point](GLOSSARY.md#the-design-point-and-headroom) (48 players)).
  A spike must prove (proposed): R1-DET-REC6's planted divergences (one std `sin`, one `HashMap` iteration, one
  presentation RNG draw inside the sim) are each reported at the first tick they change; the gate's determinism step
  takes under 3 minutes on the CI runner; the nightly comparison of a 10-minute bot match at [the largest mode's N](GLOSSARY.md#the-largest-mode-and-its-n) (48, or 54 under MODES-3 (b)) reports 0 mismatches across
  native ×2, Node, three browser engines under Playwright and real Safari on the macOS runner.
- **R5-TEST-REC3. Golden tests against legacy: captured once from the legacy oracle, committed, read by v2 tests, one
  per kept behaviour.** How: a harness snippet per behaviour runs on the real legacy build through `node tools/preview.js
  legacy eval <snippet>` (TL11) and writes a JSON fixture with the legacy commit SHA, the inventory row id, the inputs
  and the outputs; v2's Rust (or TypeScript, for "port" rows) test reads the fixture and compares within a tolerance the
  fixture states (legacy computes in JavaScript doubles; v2's sim may use `f32`, R1-DET-OQ3). What: only rows the
  inventory marks "rewrite in Rust" or "port to TypeScript" whose rule LOOP_V2 keeps: the ray maths (CS4, CS5, and the
  20,000 seeded random rays of CS6), shot resolution and locational damage (CS7, CD8), the tuning tables imported into
  the content crate (`docs/legacy-tuning.json`, exact for every value v2 keeps: CD4, CD6, CD10–CD14), presence counting
  (GE8), and for the TypeScript animation port the metrics of GA5 and GA8 (drift, hand-on-grip) as thresholds. Not
  golden: anything LOOP_V2 replaces (eras, silos, the tech box), random-AI match statistics (sim.js numbers are
  distributions: compared as ≥ 3 runs per build, TL13), and legacy artefacts such as the frame-rate-dependent jump (GW14:
  OQ3). v2's own outputs (not legacy's) use insta snapshots (F3) for content-validation errors and replay summaries,
  which CI never rewrites.
  Reason: the oracle already runs the real code and produces reproducible dumps (TL11; the tuning dump was byte-identical
  over two runs); a committed fixture survives legacy's retirement; a golden must pin a behaviour we keep, so each names
  its row and its LOOP_V2 status; insta never writes in CI, so a changed snapshot fails (F3).
  Rests on: R5-TEST-F3; legacy TL11, TL13, CS4, CS5, CS6, CS7, CD4, CD6, CD8, CD10–CD14, GE8, GA5, GA8, GW14,
  `docs/legacy-tuning.json`; R1-DET-OQ3 (cited).
  Serves: §6.3 (abilities and tuning as data start from legacy's numbers), §6.2 (hit and sight maths in 3D start from
  the 2D-plus-height maths we keep).
  A spike must prove (proposed): each golden fails when the v2 value is moved by more than its tolerance (planted); the
  CS6 fixture of 20,000 rays reproduces 20,000 of 20,000 within tolerance; regenerating a fixture from legacy on the same
  SHA gives a byte-identical file; a list of goldens per inventory row is printed by the suite.
- **R5-TEST-REC4. The network simulator in two layers, with R2's profiles as the named set.** L1: a link simulator of
  our own behind R2-TRN-REC4's transport interface, seeded, on simulated time, per direction (half the profile's round
  trip each way, proposed), with latency, jitter, loss, reordering, duplication and a rate limit (F12's set) plus a
  packet-fate log; it runs the real server sim and headless bot clients in one process. L2: an impairment wrapper around
  the server's UDP socket under the real quinn endpoint (F13), with the same profiles, for real browsers over
  WebTransport; WebSocket gets a TCP proxy with delay and stalls (loss on TCP shows as delay). Profiles: R2's P0–P4 and
  B1 by name; every network pass mark of R2 names its profile. Fuzz targets (cargo-fuzz) cover every decoder (R2-TRN-REC4
  mark 4) and the content loader (R1-RR-OQ3).
  Reason: L1 is deterministic and fast, so it can run in the gate and replay a failure from its seed; L2 is the only
  layer that tests QUIC, congestion control (R2-TRN-REC6, R2-TRN-OQ8) and the browsers; a shipping game library puts the
  impairment below the protocol in exactly this set (F12), and quinn takes a custom socket (F13); turmoil fits Tokio
  sockets but QUIC on it is unread (F11); one named set settles R2-XU-OQ12; Fiedler's advice is to test past the
  expected worst case (F14).
  Rests on: R5-TEST-F4, F11, F12, F13, F14; R2-TRN-REC4, R2-TRN-REC6, R2-TRN-OQ8, R2-XU-OQ12, R1-RR-OQ3 (cited).
  Serves: §4 (fair to about 100 ms of ping, P2), §6.1 (rooms survive bad networks).
  A spike must prove (proposed): in L1, the same seed and profile give an identical packet-fate log and identical
  per-tick hashes on two runs; measured loss, mean delay and jitter over 100,000 packets fall within 5 % of the profile
  (relative); in L2, app-level RTT agrees with quinn's server-side `rtt()` within ±5 ms at 100 ms injected round trip
  (R2-TRN-REC4 mark 2, as R2 sets it), and is then reported under P2's jitter and loss; a 1-minute
  L1 run per profile P0–P4 and B1 fits in the gate's network step, under 2 minutes in total.
  Cross-topic open questions: R5-XT-OQ10 (below).
- **R5-TEST-REC5. Soak and load at 48 and more, with bots as protocol clients.** Bots are headless Rust clients that
  speak the real protocol (inputs in, snapshots out) so a full room needs no browsers; one nightly run per mode at the
  targets of VISION_V2 §3 (items 1 and 4 of [the test-mode list](GLOSSARY.md#the-test-mode-list): [24v24](GLOSSARY.md#team-size-and-how-modes-are-written) = 48 and [12v12v12](GLOSSARY.md#team-size-and-how-modes-are-written) = 36, plus the largest mode where it is neither ([GLOSSARY](GLOSSARY.md#the-largest-mode-and-its-n); MODES-3: 18v18v18 = 54
  under reading (b)), plus LOOP_V2 §11's N = 12, 24, 36, 48 scale steps) over L1 at
  P2, 30 minutes each (proposed), and one L2 run with 4 real browser clients among the bots. Measured: server tick time p50,
  p99 and max (against the server tick budget, [GLOSSARY](GLOSSARY.md#the-server-tick-budget)), bytes per client per second (against R2-BUD's budget), resident memory slope,
  queue lengths, desyncs (from REC2's hashes on sampled clients), panics and hangs; plus TL13's match metrics (stall %,
  kill matrix, objective counters) for game-designer. Before each playtest, one 3-hour soak (proposed).
  Reason: v2 is designed for 48+ (CLAUDE.md, VISION_V2 §3); legacy's soak and scale habits carry (START_HERE §5, TL15,
  LOOP_V2 §11); an overnight soak passes when nothing hangs or asserts (F14), and we add numbers.
  Rests on: R5-TEST-F14; legacy TL13, TL15, LOOP_V2 §11.
  Serves: §3 (modes and scale), §6.1, §6.6 (the telemetry fields are exercised at full size).
  A spike must prove (proposed): a 30-minute room of bots at [the largest mode's N](GLOSSARY.md#the-largest-mode-and-its-n) (48, or 54 under MODES-3 (b)) at P2 runs with 0 panics, 0 hangs, 0 desyncs, a memory slope
  within 1 MB per 10 minutes after warm-up, and server tick time p99 inside the server tick budget ([GLOSSARY](GLOSSARY.md#the-server-tick-budget); ADR-0004 and ADR-0001 adopt it); a planted leak (one buffer kept
  per snapshot) is flagged by the slope check within the run.
- **R5-TEST-REC6. Flakes: no retries, fixed seeds in the gate, no wall-clock waits; a failure is a defect.** Vitest
  `retry: 0` and Playwright `retries: 0` (both defaults, F7, F9) are written out; Playwright `failOnFlakyTests` and
  `forbidOnly` on in CI (F9). The gate sets one fixed `PROPTEST_RNG_SEED` (proposed: a committed value, the same on
  every run and machine) so a red result reproduces; a nightly run uses fresh seeds and any failure it finds is
  committed to `proptest-regressions` with its fix (F2). Sim and network tests run on simulated time only, never
  `sleep`; browser tests wait on state through the test-hook API (TL9), and page time is driven by Playwright's clock
  where a test needs time to pass (F10). Rust tests hold no shared global state, because tests in a crate run on
  parallel threads (F1). Every failure keeps its seed, recording and trace as CI artefacts, gets an issue, and is
  reproduced with `repeatEach` or the seed before it is fixed; a test is never skipped or retried into green.
  Reason: a retry hides the defect (F9's own label for it is "flaky"); a random seed makes a red run unreproducible
  (F2); parallel threads expose shared state (F1); legacy's smoke already waits on state with generous timeouts (TL9).
  Rests on: R5-TEST-F1, F2, F7, F9, F10; legacy TL9.
  Serves: none directly (it keeps every other test's verdict trustworthy).
  A spike must prove (proposed): the gate's whole suite run 20 times on one commit gives 20 identical verdicts; a planted
  timing test (a 50 ms `setTimeout` raced against a fixed wait) is caught by the 20-run check; a planted test that
  passes only on retry fails the Playwright run.

### Question 5: CI with Rust caching (R5-CI; infra-engineer)


- **R5-CI-REC1. Cache the Rust dependencies with Swatinem/rust-cache v2.9.3, pinned by SHA. Use one `shared-key` per
  build kind (native, wasm32), save only from pushes to `main` (`save-if`), and let pull requests restore from
  `main`. No sccache at the start.** Reason: rust-cache covers the dependency builds that dominate a cold build and
  turns incremental builds off, which the CI cache cannot reuse anyway (F1-F3, F10). Pull requests already read
  `main`'s caches, and a pull request's own cache serves only its re-runs (F4), so saving from PRs mostly spends the
  10 GB (F5). Saving only from `main` also keeps fork-written caches out of what other runs restore (F6). sccache cannot
  cache our cdylib, our binaries, proc macros or incremental crates (F9), so it adds a second cache to one 10 GB budget
  for an unmeasured gain. Rests on: R5-CI-F1, F2, F3, F4, F5, F6, F7, F9, F10. Serves: none directly (CI speed). Every
  §6 item is built and tested faster; no engine feature is added. Spike must prove (proposed): (1) on a warm cache, no
  dependency crate is recompiled in a PR run (checked in `cargo build` output or `--timings`, F11); (2) the
  repository's total cache stays at or below 8 GB with every v2 job cached (read from the repository's Actions cache
  list), leaving room under 10 GB; (3) warm against cold time per Rust job, reported over 3 runs each.
  Cross-topic open questions: R5-XT-OQ14, which names mark 2 (below).
- **R5-CI-REC2. Pin the Rust toolchain exactly and build with `--locked`.** `rust-toolchain.toml` holds an exact
  `x.y.z` channel, `targets = ["wasm32-unknown-unknown"]`, components `clippy` and `rustfmt`, and `profile = "minimal"`.
  CI installs it with `rustup toolchain install` (no arguments) before any cargo call. Cargo.lock is committed, and
  every CI cargo command takes `--locked`. A weekly scheduled job tests the latest dependency versions without blocking
  (`continue-on-error`). Toolchain bumps are their own PR. Reason: one file pins local, CI and agent builds (F12). Since
  rustup 1.28 the pinned toolchain must be installed explicitly (F13). `--locked` fails rather than silently
  re-resolving (F11). A toolchain change rolls the cache over anyway (F2). R1-DET-REC6 compares native and Wasm results,
  which only means something when both come from one compiler version. The Cargo book's latest-deps job finds
  upstream breakage without turning PRs red (F19). Rests on: R5-CI-F2, F11, F12, F13, F19; R1-DET-REC6 (wave 1).
  Serves: §6.6 and §3 through R1-DET-REC6 (reproducible runs); otherwise none. Spike must prove (proposed): (1) a fresh
  runner builds native and wasm32 from `rust-toolchain.toml` alone (no other toolchain step); (2) a PR that edits
  Cargo.toml without updating Cargo.lock fails CI at the first cargo step; (3) `rustc --version` printed in every Rust
  job's log matches the file.
- **R5-CI-REC3. Pin the wasm-bindgen CLI to the library version that Cargo.lock records, and fail CI on any
  mismatch.** As gate.yml:67-72 does for Playwright, a step reads `wasm-bindgen`'s version from Cargo.lock and checks
  that it looks like `x.y.z`. It then installs exactly that CLI through taiki-e/install-action (pinned by SHA, with
  `fallback: none`), and compares `wasm-bindgen --version` with the lock before generating bindings. Reason:
  R1-BND-F2 requires matching versions, and R1-BND-REC4's pass mark 4 asks CI to fail on a mismatch. The action
  downloads the release's prebuilt CLI and checks its SHA256 (F14, F15), which avoids a source build on a cache miss
  (W2). `fallback: none` stops a silent switch to cargo-binstall (F15). Rests on: R5-CI-F14, F15; legacy gate.yml:67-72;
  R1-BND-F2, R1-BND-REC4 (wave 1). Serves: §6.3 and §6.7 through R1-BND-REC4 (generated types for data-driven
  content). Spike must prove (proposed): (1) R1-BND-REC4's pass marks 1 and 4 in CI: regenerating the types gives zero
  diff, and a deliberate CLI/library mismatch fails the job; (2) the install step takes under 30 s (proposed, measured
  over 3 runs).
- **R5-CI-REC4. Node: setup-node v7 with `cache: npm` and `cache-dependency-path` set to the client lockfile, keeping
  `npm ci --ignore-scripts`. Do not cache Playwright's browsers; install them in each run. In any job that holds a
  secret, turn automatic package caching off.** Reason: setup-node caches the global npm cache keyed by the lockfile
  (F16). Playwright says a browser cache restores about as fast as a download (F18), and the legacy numbers agree (KT8:
  about 4-8 s saved), so the browser cache would spend part of the 10 GB (F5) for little. setup-node itself advises
  against automatic caching in privileged workflows (F16), and caches are writable from pull requests (F6). Rests on:
  R5-CI-F5, F6, F16, F18; legacy KT8, TL17. Serves: none (CI hygiene). Spike must prove (proposed): (1) a warm `npm ci`
  is faster than a cold one, reported over 3 runs each; (2) the Playwright job's wall time, browser install included,
  is reported against the legacy smoke job's 57-67 s.
- **R5-CI-REC5. Run the gate v2 in CI as option G1. One workflow on push and pull_request runs parallel jobs, and one
  final job is the only required check; it runs whatever the other jobs did and fails unless every job it needs
  succeeded. No job is skipped by path filter, and the final job treats a skipped job as a failure until R5-GATE-OQ1
  allows a deliberate skip. Carry over the legacy hygiene: SHA pins, `contents: read`,
  `persist-credentials: false`, concurrency cancel, per-job timeouts. Long checks run on a schedule.** The jobs: Rust
  native (rustfmt, clippy, cargo test, the check of R1-ECS-REC2 mark 2, [the hash-map ban](GLOSSARY.md#the-hash-map-ban-in-the-sim-crate)), Rust wasm32 (build, wasm-bindgen,
  generated-type diff), TypeScript (tsc, lint, Vitest), Playwright end-to-end on the built client (TL9's harness shape),
  and determinism (R1-DET-REC6: two native processes, Wasm in Node; browser targets inside the Playwright job). The
  final job waits on exactly these five; the preview deploy (skipped on fork PRs, R5-PREV-REC1 mark 2) and the
  conditional Safari run (R5-TEST-REC1) are not among them (second pass, 2026-10-29). The
  scheduled jobs: R2-TRN-REC4's 10-million-input fuzz run per decoder, and the latest-deps job (REC2); the fuzz run's
  cost and time are unknown. Add Dependabot for `github-actions` to keep the pins moving (F24). Asking Mustafa to turn
  on the "require SHA pinning" setting (F23) is a later step; lockfiles wait until they are generally available and
  confirmed for his plan (R5-CI-OQ3). Reason: one aggregate required check survives jobs being added or renamed; a job
  skipped by condition reports success to GitHub, while path-filtered workflows hang required checks (F20), which is
  why the final job, not GitHub, decides what a skip means. The same finding says a
  job that depends on a failed job "is skipped and may not block merging" (F20), so the final job must run even after a
  failure (for example `if: always()`, then a step that fails when any `needs.<job>.result` is not `success`; that
  syntax is not quoted from the pages read: inferred) and must fail on every dependency that did not succeed, a skipped
  one included until R5-GATE-OQ1 allows a deliberate skip, or a failing PR could merge. Parallel jobs on free 4-CPU
  runners cost nothing in a public repository (F17). The hygiene is GitHub's own advice (F21) and is already proven here
  (KT8). Rests on: R5-CI-F17, F20, F21, F22, F23, F24; legacy KT8, TL1, TL9; R1-DET-REC6, R1-ECS-REC2, R2-TRN-REC4 (wave
  1). Serves: the CI side of every §6 item; the determinism job serves §6.6 and §3 (R1-DET-REC6). Spike must prove
  (proposed): (1) a PR that touches only TypeScript runs, and passes, the single required check; (2) with warm caches,
  the PR wall time is at most 10 min at p50 over 5 runs and at most 20 min cold, per job and in total (proposed budget;
  VISION_V2 sets no CI target; legacy is about 1 min); (3) the mutation suite (TL17) makes each job fail on the fault it
  guards against; (4) a fork PR runs every job with no secret available (R5-PREV-F1); (5) a PR with one deliberately
  failing job, and one with a cancelled job, each turn the single required check red and cannot merge.

### Question 6: preview links per PR (R5-PREV; infra-engineer)


- **R5-PREV-REC1. Use workflow shape P1. Build in the ordinary `pull_request` workflow with no secret, and deploy
  from a separate job that runs in an environment named `preview`, holds the only host token, never checks out or runs
  PR code, and runs only for PRs from this repository. Fork PRs get the full CI but no preview (P2 later, if outside
  contributors appear). No `pull_request_target`, no `workflow_run`.** The deploy job's permissions: `contents: read`,
  `actions: read` (artifact), `deployments: write`, `pull-requests: write`. It sets the environment's `url` to the
  preview link, posts or updates one PR comment with that link, and turns automatic package caching off (R5-CI-F16).
  Reason: fork runs get no secrets anyway (F1). `pull_request_target` and `workflow_run` hand secrets and write tokens
  to runs triggered by pull requests (F2, F3), which today's CI has never done (KT8). Splitting build and deploy keeps
  the token away from every build script and npm or cargo dependency. The environment gives one place for the secret
  and, when wanted, a reviewer gate (F6). Deploying the artifact CI built means the link shows the build that was
  tested (F9). Rests on: R5-PREV-F1, F2, F3, F5, F6, F7, F8, F9; R5-CI-F6, R5-CI-F16; legacy KT8. Serves: none directly
  (process). It carries Mustafa's Brave check for every player-facing §6 item (§6.2, §6.4, §6.9). Spike must prove
  (proposed): (1) for a same-repository PR, the link exists within 5 min after the build job ends (median of 5 runs);
  (2) a fork PR runs the build and test jobs, the deploy job is skipped, and its log shows no secret; (3) the deploy
  job contains no `actions/checkout` of the PR head and no `npm ci` or `cargo` step (checked by reading the workflow, and
  by a test PR that changes package.json scripts: the change never runs in the deploy job).
  Cross-topic open questions: R5-XT-OQ6, R5-XT-OQ12 (below).
- **R5-PREV-REC2. Choose a host that meets these needs: deploy a prebuilt directory from Actions with a scoped token; a
  stable link per PR branch; `noindex`; custom headers per hostname; an access option; deletion through a CLI or API. On
  the docs read, Cloudflare Pages meets every need except full cleanup: a branch's latest deployment cannot be deleted
  (F13, R5-PREV-OQ2). Cloudflare Workers static assets, Netlify and Vercel meet most. GitHub Pages lacks per-PR sites
  and custom headers. R6 hosts the client and the previews on Cloudflare Pages, with Workers static assets as the alternative
  (R6-HOST-REC1; [GLOSSARY](GLOSSARY.md#the-client-host): the client host); whether a closed PR's preview disappears there is PREV-DELETE (below).** Reason: F10-F13 and F18 cover every need for Cloudflare Pages
  but that cleanup point (F13); F14 shows Workers previews without a stated `noindex`; Netlify's natural path is P5
  (F15), which builds outside our tested CI; Vercel's prebuilt path works (F17), but its plan terms are R6's. Rests on:
  R5-PREV-F10, F11, F12, F13, F14, F15, F17, F18; the GitHub Pages, Netlify-headers and Vercel points rest on R6's notes
  and inference, not on an R5 finding. Serves: none directly (process, as REC1). Spike must prove (proposed): on the
  chosen host, every row of REC4's header check passes on a real preview link.
- **R5-PREV-REC3. Clean up when the PR closes, and sweep weekly.** A `pull_request` `closed` job (same-repository
  only, environment `preview`) deletes the PR branch's preview deployments (on Pages: `wrangler pages deployment
  delete … --force`) and edits the PR comment to say the link is gone. A weekly scheduled job lists and deletes
  previews of branches with no open PR. Build artifacts keep 7 days (`retention-days`), not the default 90. Reason:
  closing is an event workflows can catch (F4). Pages deletes by id but keeps a branch's latest deployment (F13), so a
  sweep backs up the close job, and what stays reachable must be known (R5-PREV-OQ2). Short artifact retention limits
  what a public repository keeps lying around (F9). Rests on: R5-PREV-F4, F9, F13, F14. Serves: none. Spike must prove
  (proposed): 24 h after a PR closes, its preview URL no longer serves the build (HTTP 404, or [the access wall](GLOSSARY.md#the-access-wall)), or the
  remaining case is written down with the reason; the sweep leaves 0 previews of closed PRs older than 7 days.
- **R5-PREV-REC4. Preview headers: the preview header set, scoped by hostname, generated by the build. The deploy
  job checks the headers.** The set is the preview header set ([GLOSSARY](GLOSSARY.md#the-production-header-set-and-the-preview-header-set)): production's header set, that is `.wasm` served as
  `application/wasm` and R6's content security policy once written, plus `X-Robots-Tag: noindex` (the host default,
  F12, F15). COOP `same-origin` and COEP `require-corp` are added only on a preview built with the isolation flag on
  ([GLOSSARY](GLOSSARY.md#preview-isolation-and-the-isolation-flag); R1-BND-REC5 G3, for profiling); production stays non-isolated until R1-BND-REC2 mark 5's test decides. Vite's
  `server.headers` and `preview.headers` take the same sets behind the same flag (R5-LAYOUT-REC5), so local development
  matches the preview (F19). The preview Mustafa checks is built with the flag off. After deploying, the job
  fetches the preview's HTML and `.wasm` and fails if a header is missing. This settles R5-XQ-OQ3. Reason: R1-BND-REC5 allows isolation in
  development and previews and asks that `crossOriginIsolated` be proven on a preview link. Cloudflare Pages' `_headers`
  rules can target one hostname pattern (F18), and Vite sets the same headers locally (F19). The wrong `.wasm` type
  breaks streaming instantiation (F20). Rests on: R5-PREV-F12, F15, F18, F19, F20; R1-BND-REC5 (wave 1). Serves: §4 (four
  desktop browsers, including Safari) through R1-BND-REC5; no §6 item names isolation (as R1-BND-REC5 says). Spike must
  prove (proposed): (1) the deploy job's header check passes, and fails when a header is removed (mutation); (2)
  `crossOriginIsolated` is true on a preview built with the isolation flag on, and false on one built with it off, in
  Chrome, Edge, Firefox, Safari and Brave (R1-BND-REC5's pass mark, plus Brave for Mustafa); (3) the production build's
  headers, and those of a preview built with the flag off, do not include COOP/COEP.
  Cross-topic open questions: R5-XT-OQ5 (below).
- **R5-PREV-REC5. Keep previews public but unlisted at the start (`noindex`, branch-name links), and offer Mustafa
  the access wall as a choice, not a default.** Reason: Mustafa opens links in Brave (CLAUDE.md), and an access policy
  limits viewing to the Cloudflare account (F12) or Cloudflare Access (F14), which puts a login in front of every
  check. A preview contains only what the public repository already contains (inferred), so the main exposure is
  search indexing, which `noindex` handles (F12, F15). This changes once a preview carries unreleased art: then the access wall ([GLOSSARY](GLOSSARY.md#the-access-wall)) is on. Whether it is also
  on once a preview talks to [the staging game server](GLOSSARY.md#the-staging-game-server) (R6-HOST-REC3) is PREVIEW-WALL (Mustafa; below); until he
  answers, no preview talks to a game server. Rests on: R5-PREV-F12, F14, F15. Serves: none (process). Spike must
  prove (proposed): a preview link opens in Brave on Windows without a login. If Mustafa picks the access wall, he
  logs in once per session and the link works after that.

### Question 7: the gate v2 that replaces today's gate (R5-GATE)


- **R5-GATE-REC1. The gate v2's checks, in order, fastest first, each with an id and a reason.**
  | Id | Stage | Check | Why (rests on) |
  |---|---|---|---|
  | G0 | setup | toolchain equals `rust-toolchain.toml`; Node equals the pin; `cargo --locked` and `npm ci` succeed; exit 2 on any setup failure | TL1; R5-LAYOUT-F6 |
  | G1 | format | `cargo fmt --all --check`; `prettier --check` on `packages/**` | R5-RUSTLINT-REC5, R5-TSLINT-REC3 |
  | G2 | Rust lint | clippy `-D warnings` on host and on `wasm32-unknown-unknown`; the determinism and panic lint sets | R5-RUSTLINT-REC1, REC2, REC3; R1-ECS-REC2 mark 2; R1-DET-REC2 |
  | G3 | TypeScript | `tsc -b`; ESLint typed lint, `--max-warnings 0` | R5-TSLINT-REC1, REC2 |
  | G4 | boundary | generated types and schema regenerate with zero diff; wasm-bindgen CLI equals the library in `Cargo.lock` | R5-LAYOUT-REC3; R1-BND-REC4, R1-DATA-REC3, R1-BND-F2 |
  | G5 | dependency rules | cargo-deny bans (sim ↛ transport, async runtime), `multiple-versions` and `wildcards` at deny with a reviewed skip list; same-feature check for `sim` | R5-LAYOUT-REC2, REC4; R2-TRN-REC4 mark 3; R5-LAYOUT-F4 (features), R5-GATE-F4 (duplicates, wildcards) |
  | G6 | build | native release (server, devtools); `sim-wasm` to Wasm; Vite build of both entries | — |
  | G7 | budgets | Wasm bytes (raw and gzip), JS bytes per entry, against a committed budget file; fail above it | R5-GATE-F2; R1-RR-OQ5 |
  | G8 | player build | walk Vite's manifest from the player entry: no chunk from `editor` | R5-LAYOUT-REC2; TL4 |
  | G9 | tests | `cargo test --workspace` (unit, property with the gate's fixed seeds, legacy goldens, insta), Wasm tests in Node, Vitest (Node) | R5-TEST-REC1, REC3, REC6 |
  | G10 | determinism | the short recordings: native ×2 processes, Wasm in Node, Wasm in Chromium, Firefox, WebKit; first mismatch reported | R5-TEST-REC2; R1-DET-REC6 |
  | G11 | network | L1 simulator, one minute per profile P0–P4 and B1, R2's pass marks that fit a minute | R5-TEST-REC4; R2-XU-OQ12 |
  | G12 | end-to-end | Playwright on the built client and a local server: TL9's smoke shape, both transport backends, the backend string on every shot, R3-MEASURE-REC3's counts against the committed baseline | R5-TEST-REC1; TL9; R2-TRN-REC4 mark 1; R3-MEASURE-REC3 |
  Scheduled, not in the PR gate (each failure opens an issue): soak and load (R5-TEST-REC5), fuzzing (R2-TRN-REC4 mark
  4), proptest with fresh seeds, cargo-mutants on `sim` and `protocol` (report only, F1), long determinism recordings,
  browser beta channels (R2-TRN-OQ1), cargo-deny advisories (F3: a new advisory must not turn an unrelated PR red; a
  PR that changes `Cargo.lock` runs the advisories check too). Real Safari: see R5-TEST-REC1 and R5-XQ-OQ1.
  Reason: legacy ordered checks fastest-first with the slowest last (gate.mjs:194, :372), so a typo fails in seconds;
  each check enforces one wave 1 or R5 recommendation, so no check is "for later"; size limits only warn in Vite (F2);
  advisories change without code changes (F3); duplicate and wildcard versions only warn by default (F4).
  Rests on: R5-GATE-F1, F2, F3, F4; R5-LAYOUT-F6, F14, F15; R5-TEST-F2, F3; legacy TL1, TL4, TL9, KT7.
  Serves: the gate of every §6 item; G10 serves §4, §6.6, §6.8 (as R1-ECS-REC2 traces determinism); G11 serves §4's
  fairness target.
  A spike must prove (proposed): see REC3 (every check fails on its planted defect).
  Cross-topic open questions: R5-XT-OQ7 (below).
- **R5-GATE-REC2. One command, the same locally and in CI, carrying today's conventions (option A).** A Node
  orchestrator (working name `node tools/gate.mjs`, v2 edition; where it lives beside legacy's gate is R5-GATE-OQ3)
  with today's base-ref rule, exit codes 0, 1 and 2, `--json`, and the TL3 habit as `--allow-removed` (a
  list of exported Rust items and TypeScript exports the approved plan removes; the generated-types diff of G4 shows
  them). `--quick` runs G0–G5 and the tests of changed crates and packages (local use between steps; never for a
  commit); the full run is the default. CI runs the same check ids as parallel jobs (R5-CI-REC5's option G1) and the
  final aggregate job compares the set of check ids that ran against the gate's own list, so a check cannot silently
  drop out of CI. Today's gate keeps running on `legacy/` changes while legacy lives (R5-LAYOUT-REC6).
  Time budget (proposed): `--quick` under 60 s warm; the full local run under 10 minutes warm on a recent desktop; CI
  as R5-CI-REC5 proposes (10 minutes p50 warm, 20 minutes cold).
  Reason: TL1 and KT7 ask to keep the base ref, the exit codes and "never a silent skip", and today's gate is a Node
  script, so option A carries them unchanged; Node is needed anyway for Vite and the TypeScript tools. A and B both
  need the Rust toolchain (G0 checks it, and `--quick` runs G1–G2's cargo fmt and clippy), so the toolchain does not
  separate them: B would build the gate itself before every run and move its conventions into a second language for
  no check it adds (inferred); D fails CLAUDE.md's protocol (the gate must PASS before a commit). A local run without
  the Rust toolchain for TypeScript-only work is not provided here (R5-GATE-OQ1). A check missing from CI is a silent
  skip by another route.
  Rests on: no R5 web finding; legacy facts only: tools/gate.mjs:37, :71-89, :194, :213, :372 and tools/README.md:19
  (the verifier confirmed each in the repository), TL1, TL3, KT7, KT8; R5-CI-REC5 (folded part).
  Serves: none directly (process).
  A spike must prove (proposed): the full gate on a clean tree passes in the budget above, measured over 5 runs, on
  Linux and Windows; removing one check from the CI workflow fails the aggregate job; a missing Playwright browser gives
  exit 2 with the setup command, not exit 0 or 1.
- **R5-GATE-REC3. The gate proves itself in one command: a committed mutation suite with controls.** `node
  tools/gate.mjs --prove` applies each planted defect in a throwaway worktree and expects exactly the named checks to
  fail and every other check to pass; controls (correct code that looks suspicious) must pass. At least one mutation and
  one control per check id. Starting set (each line: mutation → the check that must fail):
  | Check | Mutation (fails) | Control (passes) |
  |---|---|---|
  | G0 | toolchain file removed | toolchain bumped consistently in file and CI |
  | G1 | misformatted `.rs`; CRLF file; misformatted `.ts` | formatted code with long string literals |
  | G2 | `HashMap` field in `sim`; `map.values().sum()` over a std map; `Instant::now()` in a tick; `.unwrap()` in a decoder; `x.sin()` (R5-RUSTLINT-OQ1) | `Instant::now()` in `server`; `.unwrap()` in a test |
  | G3 | unchecked index read; unawaited promise; non-exhaustive switch over a generated union; import of `editor` from `client` | `import type` from `types` in `editor` |
  | G4 | Rust field renamed without regeneration; hand edit to a generated file; wasm-bindgen CLI one patch off | regeneration after a rename |
  | G5 | `quinn` or `tokio` added to `sim`; a server-only feature on a shared maths dependency | a new dev-dependency in `sim` (proptest) |
  | G7 | a constant table added to the Wasm build that crosses the committed budget (its size is set once REC4's budget exists) | a change inside the budget |
  | G8 | a dynamic `import()` of the editor from `client` | the editor entry importing `client` code |
  | G9 | a rule change that breaks a property; a v2 value moved beyond a golden's tolerance; a changed insta snapshot | a refactor with equal outputs |
  | G10 | R1-DET-REC6's three planted divergences (std `sin`, `HashMap` order, presentation RNG in the sim) | a presentation RNG draw outside the sim |
  | G11 | a decoder that drops a reordered packet; a snapshot buffer one slot too short for P2 | reordering at P1 handled |
  | G12 | a throw in the deploy path; a console error 2 s after load; an outbound request to an external host; the WebSocket backend broken; a 20 % rise in draw calls | an endless CSS animation on a button (legacy's control) |
  Every change to the gate, its checks or its budget files runs `--prove` and reports the table; the nightly run does
  too. cargo-mutants (F1) is a separate, report-only measure of test strength on `sim` and `protocol`; missed mutants go to
  test-engineer review, not into the gate's verdict.
  Reason: legacy proved its gate this way (22 of 22, with controls) and TL17 asks for the suite to be committed; a check
  without a failing case can't be shown to work; controls catch checks that fail too much.
  Rests on: R5-GATE-F1; legacy TL1, TL17, KT7, the gate-check-5 report.
  Serves: none directly (it keeps the gate honest for every §6 item).
  A spike must prove (proposed): `--prove` passes on the first gate v2 build: every mutation fails exactly its named
  checks and every control passes; `--prove` itself takes under 30 minutes (it rebuilds per mutation; proposed).
  Cross-topic open questions: R5-XT-OQ12 (below).
- **R5-GATE-REC4. Budgets are data in the repository, reviewed like code.** One committed file holds the size budgets
  (Wasm raw and gzip, JS per entry), R3-MEASURE-REC3's count baselines and their 10 % limits, the gate's time budgets and
  the proptest case counts; raising a budget is a reviewed change with its reason, like `gate-globals.json` today
  (README: "reviewed data, not a gate-tool change"). Starting values come from the first spike's measurements
  (R1-RR-OQ5), not from guesses.
  Reason: Vite's own limit only warns (F2); legacy separates reviewed data from gate code (TL6, README); R1-RR-OQ5 has no
  budget yet.
  Rests on: R5-GATE-F2; legacy TL6; R1-RR-OQ5, R3-MEASURE-REC3 (cited).
  Serves: §4 and §6.9 (the load and render budgets of the minimum machine start from these numbers).
  A spike must prove (proposed): raising a budget without changing the file fails G7; the file's history shows each
  raise with its reason.

## Open questions

### Question 1: repository layout (R5-LAYOUT)

- **R5-LAYOUT-OQ1. npm or pnpm?** pnpm's documentation is outside the allow list and was not read; npm's read page is
  marked "(Legacy)" for npm 11 (F8). Decides it: reading the current npm major's workspace page and, if Mustafa allows the
  domain, pnpm's; until then npm, which is read and already used.
- **R5-LAYOUT-OQ2. wasm-pack or the wasm-bindgen CLI directly?** Neither wasm-pack's current status nor its release
  pages were read in this run; R1-BND-F2 only requires the CLI to match the library. Decides it: engine-architect's
  ADR-0002 draft, after reading wasm-pack's repository.
- **R5-LAYOUT-OQ3. Can cargo-deny ban a workspace crate (for example, forbid `server` as a dependency of `sim`)?** F14
  shows bans for crates; whether a path crate counts is not stated. Decides it: the spike's planted `server` dependency;
  `cargo tree` is the fallback.
- **R5-LAYOUT-OQ4. Do ts-rs exports from parallel test threads conflict when many types write into one folder?** Not
  stated on the read page (F7). Decides it: REC3's byte-identical double run.
- **R5-LAYOUT-OQ5. Where do the legacy docs and the gate go in phase 1?** KT21: only the five game files are named. A
  layout decision for the main session's phase 1 plan, with docs-keeper.

### Question 2: strict TypeScript and lint rules (R5-TSLINT)

- **R5-TSLINT-OQ1. When does typescript-eslint support TypeScript 7?** Tracked in its issue #10940 (not read); F5 says it
  waits on a TypeScript 7 API. Decides it: re-reading #10940 and the dependency-versions page at the phase 1 start.
- **R5-TSLINT-OQ2. Is a TypeScript-7-native type-aware linter worth it (option C)?** Search results point to one; its
  pages are outside the allow list and were not read. Decides it: reading its docs if Mustafa allows the domain.
- **R5-TSLINT-OQ3. Do the generated types pass `erasableSyntaxOnly`?** If ts-rs or wasm-bindgen emit `enum` or
  `namespace`, the flag rejects them (F12). Not read. Decides it: R5-LAYOUT-REC3's spike runs `tsc -b` on the generated
  package.
- **R5-TSLINT-OQ4. `exactOptionalPropertyTypes` and `noUncheckedIndexedAccess` against three.js's own types.** The cost
  of these flags in calls into three.js was not measured. Decides it: the spike counts the errors and the casts the
  client needs.
- **R5-TSLINT-OQ5. Which rule enforces "`textContent`, never `innerHTML`" for dynamic text?** No rule for it was read.
  Decides it: security-auditor with [the security baseline](GLOSSARY.md#the-security-baseline); ESLint's `no-restricted-properties` or `no-restricted-syntax`
  are candidates (not read).
- **R5-TSLINT-OQ6. `skipLibCheck`.** Off checks every `.d.ts`, including the generated ones (tsconfig reference, S17 in the notes: "Skip type
  checking of declaration files."), at a cost not measured. Decides it: the spike's `tsc -b` time with it on and off.

### Question 3: clippy and rustfmt (R5-RUSTLINT)

- **R5-RUSTLINT-OQ1. Can `disallowed-methods` name std's float methods (`f32::sin`, `f64::atan2`, `f32::algebraic_add`)?**
  The read examples name only paths like `std::vec::Vec::leak` (F7). If it cannot, R1-DET-REC2's ban needs another
  check: a reviewed text or syntax scan of the sim crate for those method names (no tool for it was read). Decides it: the
  spike's planted `x.sin()`.
- **R5-RUSTLINT-OQ2. The `panic` and `arithmetic_side_effects` lints were not read.** Their source files were not at the
  paths tried (HTTP 404). They are candidates for REC3 and REC4. Decides it: reading their source or clippy's lint list.
- **R5-RUSTLINT-OQ3. Hash maps from dependencies.** REC2 bans std's `HashMap`; a map type from another crate (for
  example a hasher-backed map re-exported by a dependency) is not covered by F9's lint or the std paths. Decides it:
  rust-reviewer lists the map types of the sim's dependency tree and adds them to `disallowed-types`.
- **R5-RUSTLINT-OQ4. Overflow checks cost on the hot tick.** Not measured. Decides it: REC4's bench.
- **R5-RUSTLINT-OQ5. Allowed HashMap use inside the sim for non-outcome code.** R1-ECS-REC2 limits the ban to outcome
  code; REC2 bans the type crate-wide. Decides it: engine-architect, when the sim's modules exist; an `#[allow]` with a
  reason is the escape. Settled on 2026-10-30 ([GLOSSARY](GLOSSARY.md#the-hash-map-ban-in-the-sim-crate): the hash-map ban): the ban stays crate-wide, and an `#[allow]`
  with a written reason may admit a map that is only looked up and lies outside the serialized sim state
  (R1-DET-REC5, R1-DET-REC6); ADR-0002 records the rule.

### Question 4: tests (R5-TEST)

- **R5-TEST-OQ1. Does Playwright's clock reach the sim's Worker?** F10 does not mention workers. The sim's own time is
  the tick counter (R1-TICK), so this matters only for client-side timers inside the Worker. Decides it: the spike.
- **R5-TEST-OQ2. Can wtransport or web-transport-quinn be built on a custom quinn endpoint, so L2 sits under them?**
  F13 is quinn's; the two candidates (R2-TRN-REC1) were not read for it. Decides it: R2's interop spike.
- **R5-TEST-OQ3. A golden for intended behaviour or for legacy's behaviour?** The jump reach is 3.07 m by the formula
  and about 2.98 m as integrated at 60 Hz (GW14, GN10). With a fixed tick v2 gets one value; which one bots and maps
  are tuned for is a rule question. Decides it: game-designer (from the loop slice on), with Mustafa.
- **R5-TEST-OQ4. How long a real-Safari run takes and costs on macOS runners.** Decides it: infra-engineer (R5-CI) and R6.
- **R5-TEST-OQ5. Is turmoil worth adding for the server's own async code (rooms, timers) besides L1?** QUIC over it is
  unread (F11). Decides it: the rooms spike (R2-ROOM), if L1 cannot express its cases.
- **R5-TEST-OQ6. OS-level impairment as a third layer.** Linux `tc netem`'s documentation is outside the allow list;
  L2 covers the same need for UDP. Decides it: only if L2 cannot reproduce a bug seen in a playtest.
- **R5-TEST-OQ7. quickcheck or bolero instead of proptest.** Not read. Decides it: no need unless proptest fails a
  spike mark; proptest is the read option.
- **R5-TEST-OQ8. cargo-nextest, code coverage and the bench harness were not researched.** cargo-nextest (how it
  isolates tests is unread; whether it would bear on F1's shared-thread risk and REC6's shared-state rule is open), coverage tools (cargo-llvm-cov, Vitest's coverage; R5-LAYOUT-REC6 already ignores `coverage/`) and the
  tool for REC1's benchmark layer are options a reader would expect. Decides it: reading their pages (GitHub, docs.rs
  and vitest.dev are on the allow list; nexte.st is not) for the ADR-0002 draft; until then `cargo test`, no coverage
  gate, and benchmarks reported with method, never gated (R3-MEASURE-REC3).

### Question 5: CI with Rust caching (R5-CI; infra-engineer)


- **R5-CI-OQ1. How big are our caches?** No source gives sizes for a Cargo workspace like ours (native plus wasm32,
  bevy_ecs or hecs, Rapier, quinn and wtransport). Decided by the spike's measurement against R5-CI-REC1's 8 GB pass
  mark. If it is exceeded: fewer cache keys, or Mustafa's paid limit (F5, needs Pro).
- **R5-CI-OQ2. Is sccache worth adding later?** It would help only with dependency changes and rlib crates (F9).
  Decided by measuring dependency rebuild time once the workspace exists, against REC1's pass mark 3.
- **R5-CI-OQ3. Workflow lockfiles.** They are in public preview, Pro is not among the listed plans (F22), and the
  account type of Mustafa's personal repository is not known here. Decided by the feature's general availability and
  by Mustafa's plan. Until then: SHA pins and Dependabot.
- **R5-CI-OQ4. The wasm-opt (binaryen) pin.** It is not on install-action's list (F15). Decided by whether R1 or the
  owner's toolchain uses wasm-opt or wasm-pack, then by its release page (not read).
- **R5-CI-OQ5. GPU for WebGPU tests.** Standard runners have no GPU (F17), and Playwright's CI docs say nothing about
  WebGPU (F18). Whether the end-to-end job exercises WebGPU or only the WebGL 2 fallback is decided by the owner's gate
  checks and R3. A GPU runner is a larger, paid runner (price not read).
- **R5-CI-OQ6. Dependabot and SHA pins.** The page read does not say that Dependabot updates SHA-pinned references
  (F24). Decided by a trial PR on one action.
- **R5-CI-OQ7. An arm64 determinism leg.** `ubuntu-24.04-arm` is free for public repositories (F17). Whether the
  determinism job also runs on arm64 depends on R1-DET-OQ7 (the server's CPU architecture, R6).

### Question 6: preview links per PR (R5-PREV; infra-engineer)


- **R5-PREV-OQ1. Access wall or public-unlisted previews?** It is Mustafa's call (R5-PREV-REC5). What decides it: does
  he accept a Cloudflare login per Brave session, and will previews carry anything not already public? Its staging-server part is shared as PREVIEW-WALL (below).
- **R5-PREV-OQ2. Does a closed PR's preview really disappear on Cloudflare Pages?** "The latest deployment for a branch
  cannot be deleted" and "`--force` ... to force deletion of aliased deployments" (F13) leave the last link's fate
  unclear. Decided by a spike on a test project, or by R6 reading the Pages API docs. Shared as PREV-DELETE (below).
- **R5-PREV-OQ3. Do previews need a game server?** At the start a preview can run the sim locally in the worker
  (R1-BND-REC6: offline play). Multiplayer previews need the staging game server and certificates (R2-TRN-REC5: self-signed for at
  most 14 days, pinned via `serverCertificateHashes`, in dev and previews). Decided by phase 1's slice plan and R6's
  hosting.
- **R5-PREV-OQ4. Which host, at what price?** R6 decides from its price and limit research. This draft gives only the
  technical needs (R5-PREV-REC2). Answered by R6-HOST-REC1: Cloudflare Pages, with Workers static assets as the
  alternative; the start budget's line is 0 USD on Pages' Free plan (R6-COST-REC1, R6-HOST-F3; the price is inferred from the plan's name, no Pages price page was read).
- **R5-PREV-OQ5. Does GitHub show the environment URL on the PR page?** It is not stated in the pages read (F7). If
  not, the PR comment (REC1) is the link Mustafa uses. Decided by the spike.
- **R5-PREV-OQ6. Do branch aliases match `https://:version.:project.pages.dev/*`?** It is inferred only (F18). Decided
  by REC4's header check on a real preview.

### Question 7: the gate v2 that replaces today's gate (R5-GATE)

- **R5-GATE-OQ1. Which checks run on a TypeScript-only or Rust-only PR?** Path-based selection saves time but can skip a
  check silently. It includes whether a local TypeScript-only run without the Rust toolchain is needed (R5-GATE-REC2
  does not provide one). Decides it: the first month's CI times against R5-CI-REC5's budget; until then every check
  runs.
- **R5-GATE-OQ2. WebGPU in the end-to-end check.** Standard runners have no GPU (R5-CI-OQ5); G12's counts may run on
  the WebGL 2 fallback only. Decides it: graphics-engineer with R3-MEASURE.
- **R5-GATE-OQ3. Where the gate v2 lives while legacy's gate stays.** Both in `tools/`, or the legacy gate inside
  `legacy/`. Decides it: the phase 1 restructure plan (R5-LAYOUT-OQ5).
- **R5-GATE-OQ4. Advisories: in the PR gate or only scheduled and on lockfile changes?** F3 says the verdict can change
  without our change; R1-RR-OQ7 asks for cargo-audit and cargo-deny as CI gates. Decides it: security-auditor with the
  security baseline. Shared as BASELINE-WHEN (below). Interim (BASELINE-WHEN (b)): scheduled and on lockfile changes, as
  R5-GATE-REC1 has it; the baseline sets the policy.
- **R5-GATE-OQ5. `--prove` cost.** Rebuilding per mutation may take long; whether mutations can share one build is
  unmeasured. Decides it: the spike's REC3 time mark.

### Where the helper's parts and the owner's parts meet (R5-XQ)

The helper's findings, options, recommendations and open questions above are unchanged. Where they meet the owner's
parts, the difference is written here; nothing is decided.

- **R5-XQ-OQ1. Safari on macOS: in the PR gate or not, and does the end-to-end suite need real Safari?** R5-TEST-REC1
  runs, on macOS, the end-to-end suite in Playwright's WebKit and the determinism replay in real Safari through its
  WebDriver, for pull requests that touch `client`, `net`, `sim-worker` or `sim`, because Playwright's WebKit is not
  Safari and Playwright cannot drive branded Safari (R5-TEST-F8, F5, F6). No runner for the end-to-end suite in real
  Safari was read (a WebdriverIO suite beside Playwright would be one; not researched). R5-CI-REC5's job list has no
  macOS job, and its determinism job puts the browser targets inside the Playwright job (Chromium, Firefox and WebKit on
  Linux). What decides it: the macOS runner's time and cost for a public repository (R5-TEST-OQ4, R6), then the
  synthesis (ADR-0002); until then the macOS run can be nightly without breaking either part. Whether the end-to-end
  suite needs real Safari: decided by whether the WebKit run on macOS misses a Safari bug that a playtest or Mustafa's
  checks find.
- **R5-XQ-OQ2. One npm lockfile or a client lockfile?** R5-CI-REC4 keys the npm cache on "the client lockfile";
  R5-LAYOUT-REC1 has one root `package-lock.json` for all npm workspaces. If the root lockfile is the client's, the two
  agree. What decides it: the phase 1 restructure plan, which fixes the lockfile's path for R5-CI-REC4's
  `cache-dependency-path`.
- **R5-XQ-OQ3. Isolation headers in local development and in previews: always, or behind a flag?** R5-PREV-REC4 uses the
  same COOP and COEP values in Vite's `server.headers` and `preview.headers` "so local development matches the preview".
  R5-LAYOUT-REC5 sends them locally only behind an isolation flag, so that by default the local build behaves like
  production, which R1-BND-REC5 keeps non-isolated at the start. The same gap holds for previews: R5-PREV-REC4 isolates
  every preview while production stays non-isolated, so Mustafa's Brave check on a preview would run a header set
  production does not have. Both serve R1-BND-REC5 (G3: isolation allowed in development and previews). What decides it:
  engine-architect with R1-BND-REC5's adoption test; whichever way, the end-to-end suite should run once isolated and
  once not, so both paths are tested. One option among others: the preview Mustafa checks carries the header set
  production will ship (a production-headers preview for his check and an isolated one for profiling). Settled on
  2026-10-30 ([GLOSSARY](GLOSSARY.md#preview-isolation-and-the-isolation-flag): preview isolation and the isolation flag): R5-LAYOUT-REC5's flag rule holds for local
  development and previews alike, the preview Mustafa checks is built with the flag off, an isolated preview is built
  only for profiling, and the end-to-end suite runs once with the flag and once without.

### Shared open questions (the same words in every topic)

The questions below carry the same words in every topic they touch (resolutions 10, 17, 19 and 21 of the consistency
resolutions, approved by the advisor on 2026-10-30; part 7, "Consistency resolutions (2026-10-30)"). Each is headed by
its decision code.

- **MODES-3 (Mustafa). How are VISION_V2 §3's starting sizes read?** VISION_V2 §3 says "two teams with 2–8 players, and
  three teams with 6–18 players" to start, and "The goal: 12v12v12 or 24v24". (a) Totals: 1v1 to 4v4, and 2v2v2 to
  6v6v6; the largest mode is then the [goal](GLOSSARY.md#the-start-modes-and-the-goal-modes) 24v24, N = 48. (b) Per team: 2v2 to 8v8, and 6v6v6 to 18v18v18; the largest
  mode is then 18v18v18, N = 54. The answer sets the largest mode's N ([GLOSSARY](GLOSSARY.md#the-largest-mode-and-its-n)), which [the stress scene](GLOSSARY.md#the-stress-scene), the offline
  scale test (R1-TICK-OQ7), the soak runs (R5-TEST-REC5) and the cost rows (R6-COST) use, and items 3 and 4 of the
  test-mode list ([GLOSSARY](GLOSSARY.md#the-test-mode-list)). Until it is answered, a mark that needs the largest mode's N gives both values. Also asked
  as R2-BUD-OQ6, R2-XU-OQ13 and R3-BUDGET-OQ4. Decided by: Mustafa.
  **Answered 2026-10-31 (Mustafa): (a), totals.** The largest mode is the goal 24v24, N = 48; where a mark gives
  values for 48 and 54, the 48 value holds.
- **BASELINE-WHEN (the Advisor). Research the deferred security baseline before ADR 0006, or list it open there?** The
  security baseline ([GLOSSARY](GLOSSARY.md#the-security-baseline): content security policy, dependency audits, anti-cheat basics) was deferred by the
  Advisor on 2026-10-27, and "ADR 0006 lists it as open, or it is researched before 0006 is accepted" (R6-SEC-OQ1). Two
  cross-topic disagreements wait on it: who owns the advisories check ([GLOSSARY](GLOSSARY.md#the-advisories-check)) and whether an advisory blocks a pull
  request (R1-RR-OQ7, R2-TRN-REC1 mark 5, R5-GATE-REC1, R5-GATE-OQ4, R6-SEC-OQ1), and one wildcard certificate whose
  private key sits on every game VM (R6-HOST-REC4) against a certificate per game-server hostname (R2-TRN-REC5). (a)
  Research it now, as one security-auditor topic before the drafts. (b) List it open in ADR 0006 with interim positions:
  R5-GATE-REC1's scheduled advisories check stays the mechanism and the baseline sets the policy later; R2-TRN-REC5's
  certificate per hostname stays, and R6's wildcard becomes an option the baseline weighs. (c) Leave both as one shared
  open question. Decided by: the Advisor.
  **Answered 2026-10-31 (the Advisor): (b).** ADR 0006 lists the security baseline as open, with the interim positions
  above; the baseline is researched before the first online playtest, in its own fresh session.
- **PREV-DELETE (a spike). Does a closed PR's preview really disappear?** On Cloudflare Pages, "The latest deployment
  for a branch cannot be deleted", while `--force` serves "to force deletion of aliased deployments" (R5-PREV-F13), so
  the last link's fate is unclear. On Workers static assets, the alternative host, only "the 1000 most recently deployed
  aliases are retained" (R5-PREV-F14), and an explicit deletion was not read. Decided by: a spike on a test project on
  the client host ([GLOSSARY](GLOSSARY.md#the-client-host)), with R5-PREV-REC3's mark: 24 h after a PR closes, its preview URL no longer serves the
  build (HTTP 404, or the access wall), or the remaining case is written down with the reason. If Pages fails it with no
  workaround, Workers static assets is weighed again (R6-HOST-REC1). Also asked as R5-PREV-OQ2.
- **PREVIEW-WALL (Mustafa). Should per-PR previews that talk to the staging game server be private?** Private means
  behind the access wall ([GLOSSARY](GLOSSARY.md#the-access-wall)): the host's access policy on Mustafa's Cloudflare account (R5-PREV-F12, R6-HOST-F4)
  or Cloudflare Access (R5-PREV-F14), so each Brave check needs one login per session (R5-PREV-REC5). Open means anyone
  with the link can open the preview and the staging game server it talks to (R6-HOST-REC3). (a) Private from the first
  preview that reaches the staging server; (b) open, since previews hold no secrets; (c) decided after the security
  baseline (BASELINE-WHEN). Until Mustafa answers, no preview talks to a game server: previews run the match offline in
  the worker and stay public but unlisted (R5-PREV-REC5, R6-HOST-REC3); a preview that carries unreleased art is behind
  the access wall, as R5-PREV-REC5 already says (R5-PREV-OQ1 asks that part). Also asked as R5-PREV-OQ1 and R6-HOST-OQ5.
  Decided by: Mustafa.
  **Answered 2026-10-31 (Mustafa): (a):** previews that reach the staging game server are behind the access wall from
  the first one.

### Cross-topic open questions (consistency critic, 2026-10-29)

The consistency critic's cross-topic items that touch R5, in the critic's words, with both sides' ids (approved by the
advisor on 2026-10-30). The same question stands in each other topic it goes into, under that topic's own id.

- **R5-XT-OQ1. Which R1 items still assume Rapier, now that R4 recommends parry3d?** R4 states openly that some of R1's
  Rapier items change. It does not list R1-TICK-REC2's reason for starting at 60 Hz (Rapier's default step),
  R1-ECS-REC1's reason and mark 5, R1's TL;DR, or whether R1-RR-OQ1's SIMD/FMA hazard applies to parry3d's "3D f32 SIMD"
  BVH (R4-PHYS-OQ3 asks only by analogy). ADR-0004's determinism and snapshot sections rest on whichever crate wins.
  Side A: R1-DET-REC3; R1-ECS-REC4; R1-ECS-REC1; R1-TICK-REC2; R1-RR-OQ1. Side B: R4-PHYS-REC1. Also named: R4-PHYS-OQ3.
  Existing questions: R4 'Wave-1 items R4 changes or answers' (r4:1110-1129 at commit 8dee746) and R4-PHYS-OQ3, partly.
  Decided by: ADR-0004 (sim core: physics), with R4-PHYS-REC1 marks 2-5 and R1-DET-REC6's suite as the evidence.
  (Consistency critic, 2026-10-29, rank 3.)
- **R5-XT-OQ2. Where does the remote-entity interpolation buffer live: in the worker or on the page?** A two-block
  buffer on the page cannot hold R2's 100 to 167 ms delay. So either the worker (Rust) runs R2's adaptive snapshot
  buffer and publishes already-delayed remote poses, or the page (TypeScript) keeps a snapshot buffer, as R5's test
  layer assumes. No topic places R2's interpolation and clock estimate on one side of R1's boundary. Side A:
  R1-TICK-REC5; R1-BND-REC2; R1-BND-REC6. Side B: R2-SYNC-REC4; R5-TEST-REC1. Existing questions: none. Decided by:
  ADR-0001 (interpolation design) with ADR-0004 (boundary: block contents and the number of blocks kept). (Consistency
  critic, 2026-10-29, rank 4.)
- **R5-XT-OQ3. Who hosts, costs and builds the lobby and the room supervisor?** Join tokens, guest identity, room codes
  and crash reports all go through the lobby, and R2 hands its hosting and store to R6. R6 neither hosts nor costs it,
  and R5 has no crate for the lobby or the supervisor, so the infrastructure ADR and the cost budget leave out a
  required service. Side A: R2-ROOM-REC2; R2-ROOM-OQ9. Side B: missing answer (R6-HOST-REC2, R6-OBS-REC3, R5-LAYOUT-REC2
  named). Also named: R2-ROOM-REC8, R6-COST-REC1. Existing questions: R2-ROOM-OQ9 (left to R6, unanswered). Decided by:
  ADR-0006 (hosting and cost of the lobby and its store) with ADR-0002 (crate list) and ADR-0001 (lobby protocol).
  (Consistency critic, 2026-10-29, rank 8.)
- **R5-XT-OQ4. Do browser navigation and server-less previews rest on an undecided offline-play need?** R4 builds
  navigation for the browser, R6 serves previews without a game server, and R5 plans the first previews, all on offline
  play, which R1 leaves as an open VISION question (and, at the largest mode, as R1-TICK-OQ7). Side A: R1-BND-OQ1. Side
  B: R4-NAV-REC4; R6-HOST-REC3; R5-PREV-OQ3. Also named: R1-TICK-OQ7, R1-XU-OQ12. Existing questions: R1-BND-OQ1,
  R1-TICK-OQ7, R1-XU-OQ12. Decided by: Mustafa and the Advisor (VISION_V2), then ADR-0004 and ADR-0006. (Consistency
  critic, 2026-10-29, rank 15.)
- **R5-XT-OQ5. Which recommendations wait on the deferred security baseline?** Recommendations in R2 and R5
  (guest-secret storage, preview headers, a lint rule, token keys) wait on a topic with no recommendation and no date.
  ADR-0001, 0002 and 0006 cannot close those items. Side A: R5-PREV-REC4; R5-TSLINT-OQ5; R2-ROOM-OQ11; R2-ROOM-OQ2. Side
  B: R6-SEC-OQ1. Existing questions: R6-SEC-OQ1. Decided by: The Advisor (research the baseline before ADR-0006, or list
  it open), then security-auditor. Decision code: BASELINE-WHEN. (Consistency critic, 2026-10-29, rank 18.)
- **R5-XT-OQ6. How do multiplayer previews get their server binary and their certificate hash?** Multiplayer previews
  need a server-binary delivery job (a second CI secret and PR-built native code on a VM) and a way to publish the
  self-signed hash on the page's origin. R5's preview design covers only the static client. Side A: R6-HOST-REC3;
  R2-TRN-REC5. Side B: R5-PREV-REC1. Also named: R5-PREV-OQ3. Existing questions: R5-PREV-OQ3 (asks whether previews
  need a server; R6 answers yes, but not how). Decided by: ADR-0002 (preview workflow) and ADR-0006 (staging server),
  with ADR-0001 (where the certificate hash is fetched from). (Consistency critic, 2026-10-29, rank 20.)
- **R5-XT-OQ7. Where do the navigation bake and R4's checks live in the crates and the gate?** Where the bake code lives
  decides whether the generator ships in the player's Wasm (R4-NAV-REC4 wants a bot-free build to skip navigation data)
  and how the editor runs it in a Worker. The gate does not include R4's checks. Side A: R4-NAV-REC1; R4-FLOORS-REC3.
  Side B: missing answer (R5-LAYOUT-REC2 named). Also named: R4-PHYS-REC2, R5-GATE-REC1. Existing questions: none.
  Decided by: ADR-0002 (crate list and gate checks) with ADR-0004 (navigation). (Consistency critic, 2026-10-29, rank
  24.)
- **R5-XT-OQ8. Does the editor still open by double-click with a Wasm validator?** R1 hands this question to R5, and
  R5's editor build does not address the double-click rule EDITOR_V2 keeps. Side A: R1-DATA-OQ11. Side B: missing answer
  (R5-LAYOUT-REC5, R3-WGPU-REC4 named). Existing questions: R1-DATA-OQ11. Decided by: ADR-0002 (editor build), or
  Mustafa if EDITOR_V2 §5's double-click rule must change. (Consistency critic, 2026-10-29, rank 25.)
- **R5-XT-OQ9. Where do content's TypeScript types come from?** R5 gives the outputs one home but does not choose
  between the two sources for content types. Side A: R1-XU-OQ8; R1-BND-REC4; R1-DATA-REC3. Side B: R5-LAYOUT-REC3.
  Existing questions: R1-XU-OQ8. Decided by: ADR-0005 (content data), with ADR-0002. (Consistency critic, 2026-10-29,
  rank 31.)
- **R5-XT-OQ10. Do R2's marks all name one of R5's network profiles?** R5 treats R2's profile set as settled, but R2's
  own marks still use other jitter values and unnamed latencies, so some R2 marks cannot be run as R5 specifies. Side A:
  R5-TEST-REC4; R2-XU-OQ12. Side B: R2-BUD-REC1; R2-BUD-REC4. Also named: R2-INT-REC1. Existing questions: R2-XU-OQ12.
  Decided by: The synthesis (ADR-0001) restates every R2 mark against a named profile. (Consistency critic, 2026-10-29,
  rank 32.)
- **R5-XT-OQ11. Who sets the CI runner costs (GPU, macOS, reference machines, cache)?** Real-Safari runs, WebGPU
  coverage in CI and reference-machine runs all depend on costs no topic sets. Side A: R5-CI-OQ5; R5-TEST-OQ4;
  R3-MEASURE-OQ2. Side B: missing answer. Also named: R5-XQ-OQ1, R6-COST-REC1. Existing questions: R5-CI-OQ5,
  R5-TEST-OQ4, R5-XQ-OQ1, R3-MEASURE-OQ2. Decided by: ADR-0006 (costs) with ADR-0002 (CI); Mustafa for any paid plan.
  (Consistency critic, 2026-10-29, rank 33.)
- **R5-XT-OQ12. Where do the Sentry upload job and the smoke test's network rule fit?** R6's crash reporting adds a CI
  secret job and an external endpoint that R5's CI design and smoke check do not allow for. Side A: none by id. Side B:
  R5-GATE-REC3. Also named: R6-OBS-REC3, R5-PREV-REC1. Existing questions: none. Decided by: ADR-0002 (CI jobs, secrets
  and the smoke test's network rule) with ADR-0006. (Consistency critic, 2026-10-29, rank 34.)
- **R5-XT-OQ13. Which bench method measures R1's budget marks?** R1's budget marks wait on a measuring method R5 has not
  chosen. Side A: R1-ABL-REC6. Side B: R5-TEST-REC1; R5-TEST-OQ8. Also named: R1 pointer to R5 (tick benches). Existing
  questions: R5-TEST-OQ8. Decided by: ADR-0002 (bench tool and method). (Consistency critic, 2026-10-29, rank 36.)
- **R5-XT-OQ14. Does R5's cache question size a workspace no topic proposes?** R5's cache question assumes crates R1 and
  R4 recommend against, so it sizes a workspace no topic proposes. Side A: R5-CI-OQ1. Side B: R1-ECS-REC1; R4-PHYS-REC1.
  Existing questions: none. Decided by: ADR-0002 (crate list), then the cache spike (R5-CI-REC1 mark 2). (Consistency
  critic, 2026-10-29, rank 40.)

### Recommendations that hinge on a source not read

| Recommendation | Source | Reason not read |
|---|---|---|
| R5-LAYOUT-REC1 (npm over pnpm or Yarn) | https://pnpm.io/workspaces, https://yarnpkg.com/features/workspaces | outside the allow list |
| R5-LAYOUT-REC1 (npm's current major) | the current npm major's workspaces page | not found in this run; only v11 "(Legacy)" read |
| R5-TSLINT-REC1 (when to move to TypeScript 7) | https://github.com/typescript-eslint/typescript-eslint/issues/10940 | not fetched in this run; R5-TSLINT-OQ1 |
| R5-TSLINT-REC2 (a TypeScript-7-native linter ruled out for now) | https://oxc.rs/ | outside the allow list; R5-TSLINT-OQ2 |
| R5-TSLINT-REC3 (Prettier over dprint) | https://prettier.io/docs/, https://dprint.dev/ | outside the allow list |
| R5-RUSTLINT-REC2, REC3 (lint names and groups) | https://clippy.rust-lang.org/stable/index.html | outside the allow list (read in clippy's source instead) |
| R5-RUSTLINT-REC3, REC4 (`panic`, `arithmetic_side_effects` left out) | their clippy source files | HTTP 404 at the paths tried; R5-RUSTLINT-OQ2 |
| R5-RUSTLINT-REC5 (rendered rustfmt options) | https://rust-lang.github.io/rustfmt/ | template only (Configurations.md read instead) |
| R5-TEST-REC2, REC4 (L2 under the WebTransport crate) | wtransport and web-transport-quinn docs | not fetched in this run; R5-TEST-OQ2 |
| R5-TEST-REC4 (OS-level impairment left out) | Linux netem documentation (man7.org) | outside the allow list; R5-TEST-OQ6 |
| R5-TEST-REC1 (proptest over alternatives) | docs.rs/quickcheck, docs.rs/bolero | not fetched in this run; R5-TEST-OQ7 |
| R5-TEST-REC1, REC6 (test runner, coverage, bench tool) | cargo-nextest, cargo-llvm-cov, Vitest coverage, a bench harness | not fetched in this run; R5-TEST-OQ8 |
| R5-GATE-REC2 (task runners ruled out) | https://just.systems/ | outside the allow list |
| R5-GATE-REC3 (cargo-mutants options) | https://mutants.rs/ | outside the allow list (README read) |
| R5-CI-…, R5-PREV-… | – | the helper's parts name their unread sources in their own "Sources not read" tables (the Sources part) and in their open questions (for example R5-CI-OQ4, R5-PREV-OQ2); the owner adds none |

### Pointers to other topics

#### From R5-LAYOUT

- engine-architect (ADR-0002): the crate list and the dependency rules; wasm-pack or the CLI (OQ2).
- infra-engineer (R5-CI): one `target` folder per workspace is the Rust cache's unit (F1); the toolchain file is what
  CI installs (F6).
- rust-reviewer: REC4's feature check.

#### From R5-TSLINT

- security-auditor: OQ5 (`innerHTML`), and the dependency pins of the lint stack (R6).
- graphics-engineer: OQ4 (three.js types under the strict flags).
- infra-engineer: the lint and type-check times (REC1, REC2) are CI job budgets.

#### From R5-RUSTLINT

- rust-reviewer: OQ1, OQ3, the unsafe exception (R1-RR-OQ6).
- engine-architect (ADR-0002): REC4's profile rule; OQ5.

#### From R5-TEST

- netcode-architect: REC4 settles R2-XU-OQ12 with R2's own P0–P4 and B1; the per-direction split is proposed; OQ2.
- engine-architect: REC2's replay runner and the recording format (R1-DET-REC6 (1)).
- game-designer: OQ3; REC5's match metrics.
- infra-engineer: Safari on macOS (OQ4), nightly job budget, artefact retention for failures (REC6).

#### From R5-GATE

- infra-engineer: REC2's aggregate job compares check ids (R5-CI-REC5); scheduled jobs; artefacts.
- security-auditor: OQ4; the gate tools and budget files belong to the stricter review category (TL17).
- graphics-engineer: OQ2; R3-MEASURE-REC3's baseline lives in REC4's budget file.

#### From R5-CI and R5-PREV (infra-engineer)

- **For the R5 owner (test-engineer):** R5-CI-REC5's job list is a frame for your gate v2 checks; the check names and
  pass marks are yours. R1-ECS-REC2 pass mark 2 ([the hash-map ban](GLOSSARY.md#the-hash-map-ban-in-the-sim-crate) in the sim crate) needs a tool from you; CI only runs
  it. The Playwright job's WebGPU coverage is R5-CI-OQ5 (no GPU on standard runners). Commit the mutation suite (TL17)
  so each CI job's guard is proven.
- **For R6 (infra-engineer, hosting and costs):** previews need the R5-PREV-REC2 needs list. Please settle
  R5-PREV-OQ2 (Pages deletion of a branch's latest deployment) and R5-PREV-OQ4 (host and price), and whether a GPU
  runner (R5-CI-OQ5) or more than 10 GB of cache (R5-CI-F5, Pro account) would ever be paid. The server CPU
  architecture (R1-DET-OQ7) decides R5-CI-OQ7's arm64 leg.
- **For the security baseline (R6; security-auditor):** the preview deploy job is the first CI job with a secret
  (R5-PREV-REC1). Dependency audits (cargo-deny or cargo-audit; both installable through install-action, R5-CI-F15) and
  the CSP that REC4's header set waits for are yours.
- **For Mustafa (settings he owns, never changed by agents):** the required check name for the gate v2, the
  `preview` environment and its secret, the fork-approval setting, and, later, the SHA-pinning requirement and
  workflow lockfiles.

## Verifier spot-check

One verifier pass ran on 2026-10-29 (wave 2 of `.checkpoints/v2-research/plan.md`), and the owner's fix pass
(test-engineer, the same day) applied it. Verdict: PASS WITH NOTES. The verifier checked 60 of the 115 key claims at
their sources (WebFetch, allow-listed domains): 60 confirmed, 0 wrong, 0 unsupported. Its critical read found two
substantive points (items 1 and 2), three wording fixes beyond nits (items 3 to 5) and smaller points (items 6 to 10).
Its report, verbatim: `.checkpoints/v2-research/verifier-r5-result.md`; its notes with every quote in full:
`.checkpoints/v2-research/verifier-r5.md`; the fix log: `.checkpoints/v2-research/test-engineer-r5-fix.md`.

| Claim id | Claim | Source | Verdict | Location and exact words (as the verifier gave them) | Final state after the fix pass |
|---|---|---|---|---|---|
| R5-LAYOUT-F5 | Rust 1.99.1 on 15 Oct 2026 | blog.rust-lang.org/releases | confirmed | "Releases in 2026": "Announcing Rust 1.99.1" (Oct. 15, 2026); "Announcing Rust 1.99.0" (Oct. 1, 2026) | no change needed |
| R5-LAYOUT-F1, F2 | one lockfile, one target folder, key inheritance | cargo/reference/workspaces.html | confirmed | "All packages share a common `Cargo.lock` file which resides in the _workspace root_."; "...a directory named `target` in the _workspace root_."; "`{key}.workspace = true`"; "(requires Rust 1.99+)" | no change needed |
| R5-LAYOUT-F4 | feature unification | cargo/reference/features.html, "Feature unification" | confirmed | "Cargo will use the union of all features enabled on that dependency when building it."; "features should be _additive_" | no change needed |
| R5-LAYOUT-F7 | ts-rs 12.0.1 and its export folder | docs.rs/ts-rs | confirmed | "ts-rs 12.0.1"; "When running `cargo test` or `cargo test export_bindings`, the following TypeScript type will be exported"; "Base directory into which bindings will be exported" | no change needed |
| R5-LAYOUT-F12 | preview headers | vite.dev/config/preview-options | confirmed | v8.3.4; "Specify server response headers."; "Unless noted, the options in this section are only applied to preview." | no change needed |
| R5-LAYOUT-F14 (and R5-GATE-F4) | cargo-deny bans | cargo-deny bans/cfg.html | confirmed | `deny` "Determines specific crates that are denied."; `wrappers` "allows specific crates to have a direct dependency on the banned crate" "but denies all transitive dependencies on it." | no change needed |
| R5-LAYOUT-F15 | Vite manifest fields | vite.dev/guide/backend-integration | confirmed | "The input file name of this chunk / asset if known"; "Whether this chunk or asset is an entry point"; "The list of statically imported chunks by this chunk"; "The list of dynamically imported chunks by this chunk" | no change needed |
| R5-TSLINT-F1 | TypeScript releases | github.com/microsoft/TypeScript/releases | confirmed | "TypeScript 7.0.2" 20 Aug (Latest); "TypeScript 7.1 Beta" 20 Oct; "TypeScript 6.0.3" 16 Apr; "TypeScript 6.0" 23 Mar | no change needed |
| R5-TSLINT-F4 | supported TypeScript range | typescript-eslint.io dependency-versions (v8.72.1) | confirmed | "The version range of TypeScript currently supported is `>=4.8.4 <6.1.0`."; "...ESLint currently supported is `^8.57.0 \|\| ^9.0.0 \|\| ^10.0.0`." | no change needed |
| R5-TSLINT-F5 | no TypeScript 7 support yet | typescript-eslint issue #12518 | confirmed | "opened on Jul 8, 2026", "closed this as not planned"; "typescript-eslint isn't compatible with TS 7 at this time, because there is no TS 7 API at this time."; marked blocked on Aug 10, 2026, pointing to #10940 | no change needed |
| R5-TSLINT-F13 | `strict` configs are not semver-stable | typescript-eslint.io/users/configs | confirmed (the part marked "inferred" is in the source) | "With the exception of `all`, `strict`, and `strict-type-checked`, all configurations are considered "stable"."; "not considered "stable" under Semantic Versioning (semver)."; "only if a nontrivial percentage" … "of its developers are highly proficient in TypeScript." | fixed: the proficiency caveat is now quoted and confirmed |
| R5-TSLINT-F15 | typed lint costs a project build | typescript-eslint typed-linting page | confirmed | "you incur the performance penalty of asking TypeScript to do a build of your project before ESLint can do its linting."; "a few seconds or less" | no change needed |
| R5-TSLINT-F16 | `no-floating-promises` | its rule page | confirmed (config membership stated without a quote) | "Extending "plugin:@typescript-eslint/recommended-type-checked" in an ESLint configuration enables this rule." | fixed: that quote added |
| R5-TSLINT-F21 | ESLint recommends a formatter | eslint.org blog | confirmed | "Published 26 Oct, 2023"; "We recommend using a source code formatter instead of ESLint for formatting your code."; "will not be removed until at least ESLint v10.0.0" | fixed: notes that ESLint 10 is current and whether it removed the rules was not read (critical read 10) |
| R5-RUSTLINT-F1, F2 | lint groups | clippy/lints.html | confirmed | "The `clippy::correctness` group is the only lint group in Clippy whose lints are deny-by-default"; "It is **not** recommended to enable the whole group"; "contains lints which are buggy or need more work." | no change needed |
| R5-RUSTLINT-F7, F8, F15 | banned methods and types, lint versions | raw clippy disallowed_items.rs | confirmed | DISALLOWED_FIELDS "1.95.0", DISALLOWED_METHODS "1.49.0", DISALLOWED_TYPES "1.55.0"; `{ path = "std::time::Instant::now" },` | no change needed |
| R5-RUSTLINT-F9 | `iter_over_hash_type` covers `for` loops only | raw iter_over_hash_type.rs | confirmed | "...(i.e., `HashSet` and `HashMap`) in for loops."; version "1.76.0"; `if let Some(for_loop) = ForLoop::hir(expr)` | no change needed |
| R5-RUSTLINT-F11 | tests may unwrap if allowed | clippy/lint_configuration.html | confirmed | "Whether `unwrap` should be allowed in test functions or `#[cfg(test)]`", Default `false` (the same for expect and indexing_slicing) | no change needed |
| R5-RUSTLINT-F13 (and R5-CI-F10) | overflow and incremental defaults | cargo/reference/profiles.html | confirmed | `overflow-checks = true` (dev) and `false` (release); "Profile settings can be overridden for specific packages and build-time crates."; "Overrides cannot specify the `panic`, `lto`, or `rpath` settings."; `incremental = true` / `false` | no change needed |
| R5-RUSTLINT-F14 | rustfmt options | raw rustfmt Configurations.md | confirmed | style_edition and edition default "2015", stable "Yes"; max_width `100`; newline_style "Auto"; imports_granularity "No (tracking issue: #4991)"; group_imports "No (… #5083)" | no change needed |
| R5-TEST-F2 | proptest defaults | docs.rs proptest Config | confirmed (the part marked "inferred" is in the source) | "proptest 1.12.0"; "The default is 256, which can be overridden by setting the `PROPTEST_CASES` environment variable."; "Set by using the PROPTEST_RNG_SEED environment variable"; "If the environment variable is undefined, a random seed is generated" | fixed: the seed default is now quoted and confirmed |
| R5-TEST-F3 | insta snapshots | docs.rs/insta | confirmed | "insta 1.49.0"; "`no` for CI environments or `new` otherwise"; "`no`: does not write to snapshot files at all; just runs tests" | no change needed |
| R5-TEST-F5 | wasm-bindgen-test in browsers | rustwasm.github.io …/browsers.html | confirmed | "Use Safari for headless browser testing, and `safaridriver` as its WebDriver."; "Note that this will ignore any environment variable set." | no change needed (now in R5-TEST-REC2's Safari path) |
| R5-TEST-F6 | Vitest Browser Mode providers | vitest.dev/guide/browser | confirmed | "v5.0.5"; "webdriverio supports these browsers:" firefox, chrome, edge, safari | no change needed (now in R5-TEST-REC2's Rests on) |
| R5-TEST-F8 | Playwright's WebKit is not Safari | playwright.dev/docs/browsers | confirmed for the quoted parts; the inferred end of a sentence points to WebKit on macOS, not to real Safari | "Playwright doesn't work with the branded version of Safari since it relies on patches."; "…for the closest-to-Safari experience you should run WebKit on mac" | fixed: the sentence is completed in the source's words and the title says Playwright cannot drive branded Safari; R5-TEST-REC1's Safari row, R5-TEST-REC2 and R5-XQ-OQ1 reworded (critical read 1) |
| R5-TEST-F9 | Playwright flake settings | playwright.dev TestConfig | confirmed | failOnFlakyTests "Added in: v1.52", "Whether to exit with an error if any tests are marked as flaky. Useful on CI."; "By default failing tests are not retried." | no change needed |
| R5-TEST-F12 | GameNetworkingSockets fault injection | raw steamnetworkingtypes.h | confirmed | "[global float, 0--100] Randomly discard N pct of packets instead of sending/recv"; "Delay all outbound/inbound packets by N ms"; "Simulated jitter/clumping."; "Globally duplicate some percentage of packets."; "Global UDP token bucket rate limits."; separate _Send and _Recv values | no change needed |
| R5-TEST-F13 | quinn on a custom socket | docs.rs quinn Endpoint | confirmed | "quinn 0.11.12"; `new_with_abstract_socket(… socket: Arc<dyn AsyncUdpSocket>, …)`; "Construct an endpoint with arbitrary configuration and pre-constructed abstract socket" | no change needed |
| R5-TEST-F14 | Fiedler on network testing | gafferongames.com | confirmed | "Sep 6, 2016"; "99% packet loss, 10 seconds of latency"; "Your soak test passes when it runs overnight and doesn't hang or assert." | no change needed |
| R5-CI-F1, F2, F3 | what rust-cache saves and keys on | github.com/Swatinem/rust-cache | confirmed (nit: F2 leaves out the Cargo.toml part of the key) | "./target (build artifacts of dependencies)"; "the workspace crates themselves are not cached since doing so is generally not effective."; "sets CARGO_INCREMENTAL=0"; key: "the rustc release / host / hash" and "a hash of the workspace's Cargo.lock and of the Cargo.toml of the workspace root and every workspace member" | fixed (F2): the Cargo.toml part of the key added in the verifier's words |
| R5-CI-F5, F7 | cache limits | docs.github.com dependency-caching; github.blog changelog 2025-11-20 | confirmed | "By default, the limit is 10 GB per repository"; "not been accessed in over 7 days."; "up to 200 uploads per minute", "1500 downloads per minute"; "going beyond the 10 GB per repo limit requires a valid Pro, Team, or Enterprise account." | no change needed |
| R5-CI-F9 | what sccache cannot cache | github.com/mozilla/sccache, Known Caveats | confirmed | "This includes `bin`, `dylib`, `cdylib`, and `proc-macro` crates."; "Incrementally compiled crates cannot be cached." | no change needed |
| R5-CI-F13 | rustup 1.28 stops installing toolchains automatically | blog.rust-lang.org Rustup-1.28.0 | confirmed | "rustup will no longer automatically install the active toolchain if it is not installed."; "rustup show active-toolchain \|\| rustup toolchain install" | no change needed |
| R5-CI-F14 | latest wasm-bindgen | wasm-bindgen releases | confirmed | "0.2.129", "released this 25 Sep 07:13"; "Show all 17 assets" | no change needed |
| R5-CI-F15 | install-action | github.com/taiki-e/install-action | confirmed | "this action will also verify SHA256 checksums for downloaded files"; "none: disable all fallback options." | no change needed |
| R5-CI-F17 | hosted runners | docs.github.com github-hosted-runners | confirmed | "free and unlimited on public repositories."; ubuntu-24.04 public "4" \| "16 GB" \| "14 GB"; GPU only under larger runners | no change needed |
| R5-CI-F18 | Playwright in CI | playwright.dev/docs/ci | confirmed | "Caching browser binaries is not recommended…"; "setting workers to '1' in CI"; `mcr.microsoft.com/playwright:v1.64.1-noble` | no change needed |
| R5-CI-F20 | skipped jobs and required checks | docs.github.com troubleshooting-required-status-checks | confirmed | "Associated checks stay in a "Pending" state and block merging."; a job skipped by a condition "reports "Success""; a job that depends on a failed job "is skipped and may not block merging." | no change to the finding; R5-CI-REC5 fixed to use its last clause (critical read 2) |
| R5-CI-F22 | workflow lockfiles | github.blog changelog 2026-10-14 | confirmed (date, plans, path) | "GitHub Enterprise Cloud with data residency, Team, and Free plans."; ".github/workflows/actions.lock" | no change needed |
| R5-PREV-F1 to F4 | fork PRs and trigger events | docs.github.com events-that-trigger-workflows | confirmed | "secrets are not passed to the runner when a workflow is triggered from a forked repository."; "Avoid using this event if you need to build or run code from the pull request."; workflow_run "is able to access secrets and write tokens, even if the previous workflow was not."; default activity types `opened`, `synchronize`, `reopened` | no change needed |
| R5-PREV-F6 | environment reviewers | docs.github.com deployments-and-environments | confirmed | "If you are on a GitHub Free, GitHub Pro, or GitHub Team plan, required reviewers are only available for public repositories." | no change needed |
| R5-PREV-F12, F13 | Pages preview deployments | developers.cloudflare.com pages/preview-deployments (updated Oct 12, 2026) | confirmed | "By default, these deployment URLs are public."; "…includes the X-Robots-Tag: noindex HTTP response header."; "The latest deployment for a branch cannot be deleted." | no change to the findings; R5-PREV-REC2's "meets every need" qualified by F13 (critical read 10) |
| R5-PREV-F14 | Workers previews | developers.cloudflare.com workers/previews (updated Sep 22, 2026) | confirmed | "Only the 1000 most recently deployed aliases are retained."; "must not exceed 63 characters"; "If Version URLs are enabled, the URL is public and available after version creation." | no change needed |
| R5-PREV-F18 | Pages `_headers` file | developers.cloudflare.com pages/headers (updated Aug 25, 2026) | confirmed | "You may define up to 100 header rules."; "Each line in the `_headers` file has a 2,000 character limit."; `https://:version.:project.pages.dev/*` | no change needed |
| R5-PREV-F20 | Wasm MIME type | MDN instantiateStreaming (modified Sep 22, 2026) | confirmed | "If the response has an incorrect MIME type (instead of `application/wasm`), the promise rejects with a TypeError." | no change needed |
| R5-GATE-F1 | cargo-mutants | github.com/sourcefrog/cargo-mutants | confirmed | "Inject bugs and see if your tests catch them!" | no change needed |
| R5-GATE-F2 | Vite size limit only warns | vite.dev/config/build-options | confirmed | "Limit for chunk size warnings (in kB)." Default "500"; reportCompressedSize Default "true" | no change needed (R5-GATE-REC1's G7 row now names it in full) |
| R5-GATE-F4 | cargo-deny defaults | bans/cfg.html | confirmed | `multiple-versions` warn (default) "Prints a warning for each crate with duplicates, but does not fail the check."; the same for `wildcards` | no change needed (R5-GATE-REC1's G5 row now names it in full) |

R5-GATE-REC2 names no R5 finding, so the verifier checked its legacy basis in the repository instead (not counted in
N), all confirmed: tools/gate.mjs:71-89 (the newer merge-base), :37 (exit 2), :194 (`ORDER`), :213 (exit codes 0, 1
and 2), :372 ("[5a] smoke (slowest, so last)"), tools/README.md:19 ("About 20 s … 2.5 s") and
.github/workflows/gate.yml:67-72 (the tool version read from the lockfile).

**The critical read, ranked, and what the fix pass did.**
1. Real Safari does not follow from F8 (R5-TEST-REC1, REC2, R5-XQ-OQ1). Fixed: F8 now quotes the whole sentence and
   says Playwright cannot drive branded Safari; R5-TEST-REC1's Safari row runs the e2e suite in Playwright's WebKit on
   macOS and the Wasm and determinism tests in real Safari through safaridriver (F5) or WebdriverIO (F6), and says no
   runner for an e2e suite in real Safari was read; R5-TEST-REC2 names that WebDriver path, rests on F6 too and adds
   real Safari to its nightly mark; R5-XQ-OQ1 asks whether the e2e suite needs real Safari and what decides it.
2. The single required check can be skipped (R5-CI-REC5, the helper's part). Fixed: the final job runs whatever the
   other jobs did and fails unless every job it needs succeeded (the `if: always()` form is marked inferred: no page
   read quotes it); the reason cites F20's "is skipped and may not block merging"; spike mark 5 makes a failing and a
   cancelled job turn the required check red.
3. "At warn" fails the gate (R5-RUSTLINT-REC3). Fixed: the sim does not enable the three restriction lints, and the
   text says why `warn` would fail every index read under `-D warnings`; R5-RUSTLINT-REC1 says pedantic at warn fails
   the gate in its three crates.
4. R5-GATE-REC2's case against option B applies to A. Fixed: the reason now says A and B both need the Rust toolchain
   and picks A for continuity; options A and B say so; a Rust-free TypeScript run is not provided and is part of
   R5-GATE-OQ1.
5. §6.2 mis-traced. Fixed in R5-LAYOUT-REC4, R5-RUSTLINT-REC2, R5-RUSTLINT-REC4, R5-TEST-REC2 and, for the same
   reason, R5-GATE-REC1's G10 line: they now serve §4 (and §6.6, §6.8), as R1-ECS-REC2 traces determinism. Kept in
   part, with the reason: R5-LAYOUT-REC5's "Serves" is about the sim in a Worker, not an identical sim, and R1 traces
   that Worker (its option A2) to §6.2, §6.9 and §4; the line now names all three.
6. Previews would not match production (R5-PREV-REC4). R5-XQ-OQ3 now covers previews too (one option: a
   production-headers preview for Mustafa's Brave check and an isolated one for profiling); R5-PREV-REC4 points to it
   and is otherwise unchanged.
7. Missing options (cargo-nextest, coverage, a benchmark layer). R5-TEST-REC1's table gains a benchmark row (reported
   with method, never a gate pass or fail, R3-MEASURE-REC3); new open question R5-TEST-OQ8 records that nextest,
   coverage tools and the bench tool were not researched, with what decides it; a not-read row names their pages.
8. Missing "Rests on:" entries. Added: R1-BND-F2 (R5-LAYOUT-REC1); R5-TEST-F4, R1-DATA-REC2, R2-TRN-REC4
   (R5-LAYOUT-REC2); F7 (R5-TEST-REC1); CD4, CD6, CD10–CD14, R1-DET-OQ3 (R5-TEST-REC3); R2-TRN-OQ8 (R5-TEST-REC4);
   R5-PREV-REC2 now says its GitHub Pages, Netlify-headers and Vercel points rest on R6's notes and inference;
   R5-GATE-REC2 now says it rests on no R5 web finding and names the repository lines the verifier confirmed.
9. Wave 1 drift (R5-TEST-REC4's L2 mark). Fixed: app RTT agrees with quinn's server-side `rtt()` within ±5 ms at
   100 ms injected round trip, as R2-TRN-REC4 mark 2 sets it, and is then reported under P2.
10. Nits, all applied: F22's language list is quoted (a re-read of the same README); GATE-REC1's "F2" and "F4" are
    written in full; R5-TEST-REC6 uses one fixed committed seed; the G7 mutation drops the 200 KB figure until a budget
    exists; R5-PREV-REC2 notes the cleanup gap (F13, R5-PREV-OQ2); F21 notes that ESLint 10's removal of the rules was
    not read; the stale "section 1" and "part 8" references are fixed (and one more "section 1" in Question 6's
    legacy facts). Also from the verifier's notes: the two "inferred" parts it found in the sources (F13, F2) are now
    quoted, F16's config quote is added, and CI-F2 has the Cargo.toml part of the key.
- On "research only": R5-GATE-REC2 now calls `node tools/gate.mjs` a working name whose path is R5-GATE-OQ3, and
  R5-LAYOUT-REC6 says the move of `tools/` is input to the phase 1 plan; nothing here changes today's gate, tools or CI.
- Not applied: none. The one partial disagreement is item 5's R5-LAYOUT-REC5 (above).

**Counts.** 60 checked · 60 confirmed · 0 wrong · 0 unsupported. M = 115 key claims (every finding except R5-CI-F8 and
R5-PREV-F16, which no "Rests on:" list names), N = 60 (R5-CI-F10 counted with R5-RUSTLINT-F13). Final state of the
60: 54 no change needed to the finding (for R5-CI-F20 and R5-PREV-F13 the recommendations citing them were fixed), 6
fixed (R5-TSLINT-F13, F16, F21, R5-TEST-F2, F8, R5-CI-F2), 0 re-sourced, 0 now an open question. No
recommendation rests on a wrong or unsupported claim. No recommendation became an open question. The fix pass fetched
one page again (the Prettier README, source 25) to quote F22's language list; it added no finding and no new source.
Not verified by the verifier: parts of R5-LAYOUT-F3, R5-CI-F4, F16, F19, F21, F23, F24, the R5-PREV-F3 quotes from
the secure-use page, TOOLS.md and the rustup 1.28.2 post.

### Second pass (2026-10-29)

The spot-check found more than nits, so the verifier checked what the fix changed (`git diff 9a623a0`; report:
`.checkpoints/v2-research/verifier-r5-second-pass-result.md`, notes `verifier-r5-second-pass.md`). Verdict: PASS WITH
NOTES, nits only.
- All ten points of the critical read are fixed, cut or turned into open questions (R5-XQ-OQ3 for the preview headers,
  R5-TEST-OQ8 for the missing options); the owner's reason for keeping §6.2 in R5-LAYOUT-REC5 is sound.
- Checked: 13 confirmed, 0 wrong, 2 unsupported (both in wording the fix added: F6 credited with safaridriver; the
  nextest sentence in R5-TEST-OQ8), and 6 counts redone and correct. R5-TSLINT-F22, a key claim, is now confirmed at its
  source, so N is 61.
- Applied by the session on 2026-10-29 in the verifier's words: R5-CI-REC5's headline and reason now match its body (a
  skipped job fails the final job until R5-GATE-OQ1 decides), and it names the five jobs the final job waits on; F6 is
  credited only with listing Safari; R5-RUSTLINT-REC3 rests on F1 too; R5-TEST-REC1 rests on R1-TICK-REC2 and its mark
  exempts the benchmark layer; R5-TEST-OQ8 no longer states how nextest isolates tests; R5-XQ-OQ3 gives its answer as
  one option; the re-read date of 2026-10-29 is noted. Review stops here: what remains is nits.

### Consistency resolutions (2026-10-30)

Step 3c of `.checkpoints/v2-research/plan-amendments-2.md`, applied by the owner (test-engineer) to this file only, as
the advisor approved them on 2026-10-30 (`.checkpoints/v2-research/advisor-resolutions-verdict.md`; the texts as folded
into `resolutions.md` and `cross-topic-oqs.md`). Nothing was fetched; no claim, number or source was added. Notes:
`.checkpoints/v2-research/test-engineer-r5-consistency.md`.
- **Resolutions applied** (by the critic's rank and change id; each "before" text matched the file exactly once):
  - rank 1 (the server tick budget): C1.19 (R5-TEST-REC5, measured server tick time against the server tick budget),
    C1.20 (R5-TEST-REC5's spike mark);
  - rank 10 (MODES-3, escalated; interim wording): C10.17 (R5-TEST-REC5 adds the largest mode where it is neither
    goal), C10.18 (MODES-3 inserted);
  - rank 17 (BASELINE-WHEN, escalated): C17.5 (R5-GATE-OQ4 points to the shared question), C17.6 (BASELINE-WHEN
    inserted); C17.b3 is not applied: it waits for the Advisor's BASELINE-WHEN answer;
    Applied 2026-10-31 (docs-after-v2-research, after BASELINE-WHEN (b)): C17.b3, word for word.
  - rank 19 (the client host): C19.1 (R5-PREV-REC2's heading: R6 hosts on Cloudflare Pages, Workers static assets the
    alternative; a recommendation change), C19.2 (R5-PREV-OQ2), C19.3 (R5-PREV-OQ4, in the advisor's revised words),
    C19.4 (PREV-DELETE inserted);
  - rank 21 (PREVIEW-WALL, escalated; interim wording in the advisor's revised words): C21.1 (R5-PREV-REC5: unreleased
    art keeps the access wall on; the staging-server part is PREVIEW-WALL; until it is answered no preview talks to a
    game server; a recommendation change), C21.2 (R5-PREV-OQ1), C21.3 (PREVIEW-WALL inserted);
  - rank 22 (the isolation flag): C22.1 (R5-PREV-REC4's set is the preview header set, COOP and COEP only with the
    isolation flag on; a recommendation change), C22.2 (R5-PREV-REC4 settles R5-XQ-OQ3), C22.3 (R5-PREV-REC4 marks 2
    and 3; a recommendation change), C22.4 (R5-XQ-OQ3 settled);
  - rank 29 (the hash-map ban): C29.4 (R5-RUSTLINT-OQ5 settled);
  - rank 41 (wrong cross-reference, a nit): applied through C22.1 ("until R1-BND-REC2 mark 5's test decides").
  - Ranks 5, 6, 9, 11 and 23 change nothing in R5.
- **Glossary terms** ([GLOSSARY.md](GLOSSARY.md); linked from the header and from the first use of each term the file
  uses): no "Retire (exact)" phrase was in this file. "Retire (in context)" words replaced where they carried the
  term's meaning: "tick time" in R5-LAYOUT-REC5 and "tick cost" in R5-RUSTLINT-REC4's spike (sim step time); the
  benchmark row of R5-TEST-REC1 (sim step time; REC5's server tick time p99); "a preview-only header set" in
  R5-PREV-REC4's heading (the preview header set); "Cloudflare's `_headers`" in R5-PREV-REC4's reason (Cloudflare
  Pages', F18 being the Pages page); "R6's security baseline" in R5-TSLINT-OQ5, R5-GATE-OQ4 and the pointers to R6 (the
  security baseline); "an EU server" in R5-PREV-OQ3 (the staging game server). The other in-context hits stay: they
  name a product, an account, a URL or a source table row, a scale step or the design point (48+), the reviewer's
  proposal in R1-RR-OQ7's words, or an option inside a question. No value changed because of a definition, and no
  R5 time mark is a tick-time mark without a statistic.
- **Shared open questions added** (part "Open questions"): MODES-3, BASELINE-WHEN, PREV-DELETE and PREVIEW-WALL under
  "Shared open questions (the same words in every topic)"; R5-XT-OQ1 to R5-XT-OQ14 (the critic's ranks 3, 4, 8, 15,
  18, 20, 24, 25, 31, 32, 33, 34, 36 and 40) under "Cross-topic open questions (consistency critic, 2026-10-29)", with
  a pointer from each R5 recommendation they name (R5-LAYOUT-REC2, REC3, REC5; R5-TEST-REC1, REC4; R5-CI-REC1;
  R5-PREV-REC1, REC4; R5-GATE-REC1, REC3).
- **Header:** the open-question total becomes 63 (18 added); the status label is unchanged (the step-3d verifier
  recounts it).
- **OBJECT:** none.
- Step 3d fixes (2026-10-30):
  - r5:401-402 (the wave-1 list, R1-ECS-REC2 mark 2): the hash-map ban added beside R1's old words (nit).
  - r5:1477 (R5-TEST-REC5): the list named as items 1 and 4 of the test-mode list (nit).
  - r5:1561 (R5-CI-REC5's job list): the check restated as R1-ECS-REC2 mark 2's, the hash-map ban (nit).
  - r5:2087 (the pointer to the R5 owner): R1-ECS-REC2 pass mark 2 restated as the hash-map ban (nit).
  - r5:58 (TL;DR): "48-bot soaks" becomes bot soaks up to the largest mode's N (nit; REC5's value changed by C10.17).
  - r5:1429 (R5-TEST-REC2's spike): the 10-minute match runs at the largest mode's N, 48 or 54 under MODES-3 (b)
    (recommendation).
  - r5:1487 (R5-TEST-REC5's spike): the 30-minute room runs at the largest mode's N, 48 or 54 under MODES-3 (b)
    (recommendation).
  - r5:1426 (R5-TEST-REC2's Serves line): "the 48-player scale" of VISION_V2 §3, which names §3's design point and not
    the largest mode, now reads "the design point (48 players)", linked (nit).
  - r5:1920 (MODES-3): "the stress scene" linked to its glossary entry, as in R6's copy (nit).
  - r5:1477 (R5-TEST-REC5): the mode names 24v24 and 12v12v12 linked to the glossary's team-size entry (nit).

## Sources

### Sources read: questions 1–4 and 7 (test-engineer)

| # | URL | Title | Version or page date | Tier | Read |
|---|---|---|---|---|---|
| 1 | https://doc.rust-lang.org/cargo/reference/workspaces.html | Cargo Book: Workspaces | no date | official | 2026-10-28 |
| 2 | https://blog.rust-lang.org/releases/ | Rust release announcements | 1.99.1, Oct. 15, 2026 | official | 2026-10-28 |
| 3 | https://doc.rust-lang.org/cargo/reference/resolver.html | Cargo Book: Dependency Resolution | no date | official | 2026-10-28 |
| 4 | https://doc.rust-lang.org/cargo/reference/features.html | Cargo Book: Features | no date | official | 2026-10-28 |
| 5 | https://docs.npmjs.com/cli/v11/using-npm/workspaces | npm Docs: workspaces | npm CLI 11.21.1 ("Legacy") | official | 2026-10-28 |
| 6 | https://vite.dev/guide/features | Vite: Features | v8.3.4 | official | 2026-10-28 |
| 7 | https://vite.dev/guide/build | Vite: Building for Production | v8.3.4 | official | 2026-10-28 |
| 8 | https://vite.dev/config/server-options | Vite: Server Options (also read by the helper) | v8.3.4 | official | 2026-10-28 |
| 9 | https://vite.dev/config/preview-options | Vite: Preview Options (also read by the helper) | v8.3.4 | official | 2026-10-28 |
| 10 | https://www.typescriptlang.org/docs/handbook/project-references.html | TypeScript: Project References | Last updated Oct 27, 2026 | official | 2026-10-28 |
| 11 | https://github.com/microsoft/TypeScript/releases | TypeScript releases | 7.0.2 (20 Aug), 7.1 Beta (20 Oct) | official | 2026-10-28 |
| 12 | https://typescript-eslint.io/users/dependency-versions | typescript-eslint: Dependency Versions | v8.72.1 | official | 2026-10-28 |
| 13 | https://github.com/typescript-eslint/typescript-eslint/issues/12518 | Issue #12518 "TypeScript 7.0.2 Support" | Jul 8, 2026; Aug 10, 2026 | official | 2026-10-28 |
| 14 | https://typescript-eslint.io/users/configs | typescript-eslint: Shared Configs | v8.72.1 | official | 2026-10-28 |
| 15 | https://typescript-eslint.io/getting-started/typed-linting | typescript-eslint: Linting with Type Information | v8.72.1 | official | 2026-10-28 |
| 16 | https://typescript-eslint.io/rules/no-floating-promises/ | Rule: no-floating-promises | v8.72.1 | official | 2026-10-28 |
| 17 | https://typescript-eslint.io/rules/switch-exhaustiveness-check/ | Rule: switch-exhaustiveness-check | v8.72.1 | official | 2026-10-28 |
| 18 | https://www.typescriptlang.org/tsconfig/ | TSConfig Reference (read in two parts) | © 2012-2026, no date | official | 2026-10-28 |
| 19 | https://www.typescriptlang.org/docs/handbook/release-notes/typescript-6-0.html | TypeScript 6.0 release notes | Last updated Oct 27, 2026 | official | 2026-10-28 |
| 20 | https://eslint.org/docs/latest/rules/no-restricted-imports | ESLint rule: no-restricted-imports | latest = v10.13.0 | official | 2026-10-28 |
| 21 | https://eslint.org/blog/2023/10/deprecating-formatting-rules/ | Deprecation of formatting rules | 26 October 2023 | official | 2026-10-28 |
| 22 | https://eslint.org/docs/latest/rules/no-restricted-globals | ESLint rule: no-restricted-globals | latest = v10.13.0 | official | 2026-10-28 |
| 23 | https://typescript-eslint.io/troubleshooting/faqs/eslint/ | typescript-eslint FAQ: ESLint | v8.72.1 | official | 2026-10-28 |
| 24 | https://eslint.org/docs/latest/use/configure/configuration-files | ESLint: Configuration Files | latest = v10.13.0 | official | 2026-10-28 |
| 25 | https://github.com/prettier/prettier | Prettier README | no version shown | official (project) | 2026-10-28 |
| 26 | https://github.com/biomejs/biome | Biome README | no version shown | official (project) | 2026-10-28 |
| 27 | https://docs.rs/ts-rs/latest/ts_rs/ | ts-rs crate docs | 12.0.1 | official | 2026-10-28 |
| 28 | https://embarkstudios.github.io/cargo-deny/checks/bans/cfg.html | cargo-deny: bans configuration | no version | official (tool) | 2026-10-28 |
| 29 | https://embarkstudios.github.io/cargo-deny/ | cargo-deny book | no version | official (tool) | 2026-10-28 |
| 30 | https://embarkstudios.github.io/cargo-deny/checks/advisories/index.html | cargo-deny: advisories | no version | official (tool) | 2026-10-28 |
| 31 | https://rust-lang.github.io/rustup/overrides.html | rustup book: Overrides (also read by the helper) | no version | official | 2026-10-28 |
| 32 | https://doc.rust-lang.org/clippy/lints.html | Clippy book: Clippy's Lints | no version | official | 2026-10-28 |
| 33 | https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/iter_over_hash_type.rs | clippy source: iter_over_hash_type | master, lint 1.76.0 | official (source) | 2026-10-28 |
| 34 | https://doc.rust-lang.org/clippy/lint_configuration.html | Clippy book: Lint Configuration Options | no version | official | 2026-10-28 |
| 35 | https://doc.rust-lang.org/clippy/configuration.html | Clippy book: Configuring Clippy | no version | official | 2026-10-28 |
| 36 | https://doc.rust-lang.org/cargo/reference/manifest.html | Cargo Book: The Manifest Format (`[lints]`) | no date | official | 2026-10-28 |
| 37 | https://github.com/rust-lang/rust-clippy/tree/master/clippy_lints/src | clippy source tree listing (navigation only) | master | official (source) | 2026-10-28 |
| 38 | https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/disallowed_items.rs | clippy source: disallowed_methods, disallowed_types | master | official (source) | 2026-10-28 |
| 39 | https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/floating_point_arithmetic/mod.rs | clippy source: suboptimal_flops, imprecise_flops | master, lints 1.43.0 | official (source) | 2026-10-28 |
| 40 | https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/indexing_slicing.rs | clippy source: indexing_slicing | master | official (source) | 2026-10-28 |
| 41 | https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/methods/mod.rs | clippy source: unwrap_used, expect_used (offsets 0 and 100000) | master, lints 1.45.0 | official (source) | 2026-10-28 |
| 42 | https://doc.rust-lang.org/cargo/reference/profiles.html | Cargo Book: Profiles (also read by the helper) | no date | official | 2026-10-28 |
| 43 | https://raw.githubusercontent.com/rust-lang/rustfmt/master/Configurations.md | rustfmt: Configurations | master | official (source) | 2026-10-28 |
| 44 | https://docs.rs/proptest/latest/proptest/ | proptest crate docs | 1.12.0 | official | 2026-10-28 |
| 45 | https://docs.rs/proptest/latest/proptest/test_runner/struct.Config.html | proptest: Config | 1.12.0 | official | 2026-10-28 |
| 46 | https://github.com/rust-fuzz/cargo-fuzz | cargo-fuzz README | no version shown | official (project) | 2026-10-28 |
| 47 | https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html | wasm-bindgen guide: wasm-bindgen-test | no version (may be older than the crate) | official | 2026-10-28 |
| 48 | https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/browsers.html | wasm-bindgen guide: Testing in Headless Browsers | no version | official | 2026-10-28 |
| 49 | https://vitest.dev/guide/browser/ | Vitest: Browser Mode | v5.0.5 | official | 2026-10-28 |
| 50 | https://vitest.dev/config/retry | Vitest config: retry | v5.0.5 | official | 2026-10-28 |
| 51 | https://vitest.dev/guide/mocking/timers | Vitest: Mocking timers | v5.0.5 | official | 2026-10-28 |
| 52 | https://playwright.dev/docs/browsers | Playwright: Browsers | no version shown | official | 2026-10-28 |
| 53 | https://playwright.dev/docs/test-retries | Playwright: Retries | no version shown | official | 2026-10-28 |
| 54 | https://playwright.dev/docs/api/class-testconfig | Playwright: TestConfig | options from v1.10 to v1.52 | official | 2026-10-28 |
| 55 | https://playwright.dev/docs/clock | Playwright: Clock | no version shown | official | 2026-10-28 |
| 56 | https://github.com/tokio-rs/turmoil | turmoil README | no version shown | official (project) | 2026-10-28 |
| 57 | https://docs.rs/turmoil/latest/turmoil/ | turmoil crate docs | 0.7.2 | official | 2026-10-28 |
| 58 | https://docs.rs/turmoil/latest/turmoil/struct.Builder.html | turmoil: Builder | 0.7.2 | official | 2026-10-28 |
| 59 | https://raw.githubusercontent.com/ValveSoftware/GameNetworkingSockets/master/include/steam/steamnetworkingtypes.h | GameNetworkingSockets: steamnetworkingtypes.h | master | shipped game | 2026-10-28 |
| 60 | https://github.com/ValveSoftware/GameNetworkingSockets | GameNetworkingSockets README | no version shown | shipped game | 2026-10-28 |
| 61 | https://gafferongames.com/post/packet_fragmentation_and_reassembly/ | Packet Fragmentation and Reassembly | September 6, 2016 | other | 2026-10-28 |
| 62 | https://docs.rs/quinn/latest/quinn/trait.AsyncUdpSocket.html | quinn: AsyncUdpSocket | 0.11.12 | official | 2026-10-28 |
| 63 | https://docs.rs/quinn/latest/quinn/struct.Endpoint.html | quinn: Endpoint | 0.11.12 | official | 2026-10-28 |
| 64 | https://docs.rs/insta/latest/insta/ | insta crate docs | 1.49.0 | official | 2026-10-28 |
| 65 | https://doc.rust-lang.org/cargo/commands/cargo-test.html | Cargo Book: cargo test | no date | official | 2026-10-28 |
| 66 | https://github.com/sourcefrog/cargo-mutants | cargo-mutants README | no version shown | official (project) | 2026-10-28 |
| 67 | https://vite.dev/config/build-options | Vite: Build Options | v8.3.4 | official | 2026-10-28 |
| 68 | https://vite.dev/guide/backend-integration | Vite: Backend Integration | v8.3.4 | official | 2026-10-28 |

Owner's count: 68 read: 65 official, 2 shipped game, 1 other. Rows 8, 9, 31 and 42 were also read by the helper.
Repository sources (not web): tools/gate.mjs, tools/README.md, .github/workflows/gate.yml, docs/START_HERE.md §1 and
§5, docs/reports/2026-10-14-gate-check-5.md, docs/LEGACY_INVENTORY.md (TL, KT, CS, CD, GA, GE, GN, GW rows),
docs/LEGACY_TUNING.md and docs/legacy-tuning.json, docs/LOOP_V2.md §11, docs/research/r1-…, r2-…, r3-…, r4-… (ids
cited).

### Sources read: question 5 (infra-engineer)

| URL | Title | Version or page date | Tier | Read |
|---|---|---|---|---|
| https://github.com/Swatinem/rust-cache | Swatinem/rust-cache README | v2.9.3 ("20 Oct") | official (tool) | 2026-10-28 |
| https://github.com/Swatinem/rust-cache/releases | rust-cache releases | v2.9.3, v2.9.2, v2.9.1 | official (tool) | 2026-10-28 |
| https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching | Dependency caching reference | no date | official | 2026-10-28 |
| https://github.blog/changelog/2025-11-20-github-actions-cache-size-can-now-exceed-10-gb-per-repository/ | Actions cache size can now exceed 10 GB | 2025-11-20 | official | 2026-10-28 |
| https://github.com/Mozilla-Actions/sccache-action | sccache-action README | v0.0.11 | official (tool) | 2026-10-28 |
| https://github.com/mozilla/sccache | sccache README | release not shown | official (tool) | 2026-10-28 |
| https://github.com/mozilla/sccache/releases | sccache releases | v0.19.0 (2026-10-24) | official (tool) | 2026-10-28 |
| https://doc.rust-lang.org/cargo/reference/profiles.html | Cargo Book: Profiles | no version | official | 2026-10-28 |
| https://doc.rust-lang.org/cargo/commands/cargo-build.html | Cargo Book: cargo build | no version | official | 2026-10-28 |
| https://doc.rust-lang.org/cargo/guide/continuous-integration.html | Cargo Book: Continuous Integration | no version | official | 2026-10-28 |
| https://rust-lang.github.io/rustup/overrides.html | rustup book: Overrides | no version | official | 2026-10-28 |
| https://blog.rust-lang.org/2025/03/02/Rustup-1.28.0/ | Announcing Rustup 1.28.0 | 2025-03-02 | official | 2026-10-28 |
| https://blog.rust-lang.org/2025/05/05/Rustup-1.28.2/ | Announcing rustup 1.28.2 | 2025-05-05 | official | 2026-10-28 |
| https://github.com/wasm-bindgen/wasm-bindgen/releases | wasm-bindgen releases | 0.2.129 (2026-09-25) | official | 2026-10-28 |
| https://github.com/taiki-e/install-action | install-action README | release not shown | official (tool) | 2026-10-28 |
| https://raw.githubusercontent.com/taiki-e/install-action/main/TOOLS.md | install-action TOOLS.md | main | official (tool) | 2026-10-28 |
| https://github.com/actions/setup-node | setup-node README | v7 | official | 2026-10-28 |
| https://docs.github.com/en/actions/reference/runners/github-hosted-runners | GitHub-hosted runners reference | no date | official | 2026-10-28 |
| https://playwright.dev/docs/ci | Playwright: Continuous Integration | v1.64.1 (Docker tag) | official | 2026-10-28 |
| https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/collaborating-on-repositories-with-code-quality-features/troubleshooting-required-status-checks | Troubleshooting required status checks | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/reference/security/secure-use | Secure use reference | no date | official | 2026-10-28 |
| https://github.blog/changelog/2026-10-14-github-actions-workflow-lockfiles-in-public-preview/ | Workflow lockfiles in public preview | 2026-10-14 | official | 2026-10-28 |
| https://docs.github.com/en/actions/concepts/security/workflow-dependency-lockfiles | Workflow dependency lockfiles | no date | official | 2026-10-28 |
| https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository | Managing GitHub Actions settings for a repository | no date | official | 2026-10-28 |
| https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/keeping-your-actions-up-to-date-with-dependabot | Keeping your actions up to date with Dependabot | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/how-tos/manage-runners/github-hosted-runners/customize-runners | Customizing GitHub-hosted runners (title only; nothing used) | no date | official | 2026-10-28 |

### Sources read: question 6 (infra-engineer)

| URL | Title | Version or page date | Tier | Read |
|---|---|---|---|---|
| https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows | Events that trigger workflows | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/reference/security/secure-use | Secure use reference | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/how-tos/manage-workflow-runs/approve-runs-from-forks | Approving workflow runs from forks | no date | official | 2026-10-28 |
| https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository | Managing GitHub Actions settings for a repository | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments | Deployments and environments | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/manage-environments | Managing environments for deployment | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax | Workflow syntax (characters 0-200000) | no date | official | 2026-10-28 |
| https://github.com/actions/download-artifact | download-artifact README | v8 | official | 2026-10-28 |
| https://developers.cloudflare.com/pages/how-to/use-direct-upload-with-continuous-integration/ | Pages: Direct Upload with CI | Last updated Sep 18, 2026 | official | 2026-10-28 |
| https://github.com/cloudflare/wrangler-action | wrangler-action README | v4 | official (tool) | 2026-10-28 |
| https://developers.cloudflare.com/pages/configuration/preview-deployments/ | Pages: Preview deployments | Last updated Oct 12, 2026 | official | 2026-10-28 |
| https://developers.cloudflare.com/pages/configuration/headers/ | Pages: Headers | Last updated Aug 25, 2026 | official | 2026-10-28 |
| https://developers.cloudflare.com/workers/configuration/previews/ | Workers: Preview URLs | Last updated Sep 22, 2026 | official | 2026-10-28 |
| https://docs.netlify.com/site-deploys/deploy-previews/ | Netlify: Deploy Previews | Last updated Oct 8, 2026 | official | 2026-10-28 |
| https://docs.netlify.com/platform/security-checklist/ | Netlify: Security checklist | Last updated Sep 30, 2026 | official | 2026-10-28 |
| https://vercel.com/kb/guide/how-can-i-use-github-actions-with-vercel | Vercel: GitHub Actions with Vercel | updated 2026-10-16 | official | 2026-10-28 |
| https://vite.dev/config/server-options | Vite: Server options | v8.3.4 | official | 2026-10-28 |
| https://vite.dev/config/preview-options | Vite: Preview options | v8.3.4 | official | 2026-10-28 |
| https://vite.dev/guide/static-deploy | Vite: Deploying a static site | v8.3.4 | official | 2026-10-28 |
| https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiateStreaming_static | MDN: WebAssembly.instantiateStreaming() | Last modified Sep 22, 2026 | official | 2026-10-28 |

### Sources not read: questions 1–4 and 7 (test-engineer)

| URL | Reason |
|---|---|
| https://clippy.rust-lang.org/stable/index.html (the clippy lint list; rust-lang.github.io/rust-clippy redirects here) | outside the allow list (lints read in clippy's source instead) |
| https://rust-lang.github.io/rustfmt/ | the page returned only a template without option text (Configurations.md read instead) |
| https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/disallowed_methods.rs | HTTP 404 (the lint moved to disallowed_items.rs, read) |
| https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_lints/src/arithmetic_side_effects.rs | HTTP 404 (R5-RUSTLINT-OQ2) |
| https://github.com/rust-lang/rust-clippy/tree/master/clippy_lints/src/disallowed | HTTP 404 |
| https://pnpm.io/workspaces | outside the allow list (R5-LAYOUT-OQ1) |
| https://yarnpkg.com/features/workspaces | outside the allow list |
| the current npm major's workspaces page | not found in this run; only npm 11's page, marked "(Legacy)", was read |
| https://prettier.io/docs/ | outside the allow list (the README was read) |
| https://dprint.dev/ | outside the allow list |
| https://biomejs.dev/ | outside the allow list (the README was read) |
| https://oxc.rs/ (oxlint and its type-aware mode) | outside the allow list (R5-TSLINT-OQ2) |
| https://github.com/typescript-eslint/typescript-eslint/issues/10940 | on the allow list; not fetched in this run (R5-TSLINT-OQ1) |
| wasm-pack's repository and releases | on the allow list; not fetched in this run (R5-LAYOUT-OQ2) |
| docs.rs/quickcheck, docs.rs/bolero | on the allow list; not fetched in this run (R5-TEST-OQ7) |
| wtransport and web-transport-quinn docs (endpoint construction) | on the allow list; not fetched in this run (R5-TEST-OQ2) |
| Linux `tc netem` documentation (man7.org) | outside the allow list (R5-TEST-OQ6) |
| https://mutants.rs/ | outside the allow list (the README was read) |
| https://nexte.st/ | outside the allow list |
| github.com/nextest-rs/nextest, docs.rs/cargo-llvm-cov, vitest.dev's coverage guide, a Rust bench harness's docs | on the allow list; not fetched in this run (R5-TEST-OQ8, added by the fix pass) |
| https://just.systems/ | outside the allow list |
| infoq.com, dev.to, ecorpit.com, mergify.com, coddy.tech (search results on TypeScript 7 and linting) | outside the allow list (search leads only; nothing used) |
| devblogs.microsoft.com/typescript (search result) | outside the allow list (search lead only) |
| docsearch.algolia.com (search result for tsgolint) | outside the allow list (search lead only) |
| github.com/Effect-TS/tsgo, github.com/Agenta-AI/agenta/pull/5472 (search results) | on the allow list; not needed (search leads only) |

### Sources not read: question 5 (infra-engineer)

| URL | Reason |
|---|---|
| https://internals.rust-lang.org/t/seeking-beta-testers-for-rustup-v1-28-0/22060 | outside the allow list (search lead) |
| https://sia.hackernoon.com/whats-new-in-rustup-1280 | outside the allow list (search lead) |
| https://daleseo.com/rustup/ ; https://openmamba.org/en/rpms/base/rustup | outside the allow list (search leads) |
| https://azukiazusa.dev/en/blog/github-actions-workflow-lockfiles ; https://horadecodar.com.br/?p=47732 ; https://techenclave.com/news/items/github-actions-previews-lockfiles-for-workflow-dependencies | outside the allow list (search leads) |
| https://www.mexc.com/news/181586 ; https://fast.io/resources/github-storage-limit/ ; https://github.internet2.edu/... ; https://gitea.psi.ch/... | outside the allow list (search leads) |
| https://redirect.github.com/rust-lang/rustup/pull/3985 (and other redirect.github.com leads) | outside the allow list (redirect host) |
| https://docs.github.com/en/actions/how-tos/secure-your-work/lock-workflow-dependencies | on the allow list; not needed (concept page and changelog read) |
| https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax (characters 200000 onward) | not read: the needed sections were found earlier |
| wasm-bindgen guide (wasm-bindgen.github.io) | outside the allow list; R1-BND-F2 covers the version rule |
| doc.rust-lang.org cargo install page | on the allow list; not read in this run (option W2's build time stays unmeasured) |

### Sources not read: question 6 (infra-engineer)

| URL | Reason |
|---|---|
| https://www.netlify.com/blog/2020/05/19/environment-variables-and-preview-deploys-get-a-boost/ | outside the allow list (www.netlify.com; search lead) |
| https://answers.netlify.com/t/how-is-determined-if-environment-variables-are-sensitive-or-not/41608 ; https://community.netlify.com/... | outside the allow list (search leads) |
| https://docs.netlify.com/build/environment-variables/overview/ | on the allow list; not needed (the security checklist gave the policy's default) |
| Netlify CLI deploy, Vercel and Netlify cleanup, Cloudflare Pages REST API | on the allow list; not read in this run (R5-PREV-OQ2, R6) |

---

### Counts by tier

| Part | Read | Official | Shipped game | Other | Not read (rows) |
|---|---|---|---|---|---|
| Questions 1–4 and 7 (test-engineer) | 68 | 65 | 2 | 1 | 25 |
| Question 5 (infra-engineer) | 26 (one title-only, nothing used) | 26 | 0 | 0 | 10 |
| Question 6 (infra-engineer) | 20 | 20 | 0 | 0 | 4 |
| Whole file, each page once | 108 | 105 | 2 | 1 | 39 |

"Each page once": 4 pages were read by both authors (Vite server options, Vite preview options, rustup overrides,
Cargo profiles) and 2 pages appear in both of the helper's tables (GitHub's "Secure use reference" and "Managing GitHub
Actions settings for a repository"), so 68 + 26 + 20 − 4 − 2 = 108. A not-read row can name several pages (search
leads are grouped). Every source was read 2026-10-28, except the Prettier README, re-read for R5-TSLINT-F22 on 2026-10-29.
