# R5 draft, question 1: repository layout (R5-LAYOUT)

Brief question (quoted): "Repository layout (a Cargo workspace plus TypeScript with Vite)". Task scope: crates (sim, content,
server, Wasm bindings), TypeScript packages, where generated types live, the legacy game's move to `legacy/` in phase 1,
package-manager workspaces. All sources read 2026-10-28.

## Findings

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

### Legacy facts used
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

## Options

### Where the two workspaces sit
| Option | Shape | Complexity | Risk | Fit |
|---|---|---|---|---|
| A. One repo, both workspaces at the root | `Cargo.toml` (virtual) + `crates/*`; `package.json` + `packages/*`; `legacy/` | low: one checkout, one gate, one PR per change across the boundary | root holds two lockfiles and two toolchain files; tools must agree on paths | a change to a boundary type lands with both sides in one PR (R1-BND-REC4) |
| B. One repo, two roots | `rust/` (Cargo workspace) and `web/` (npm workspace) | low-medium: every command needs a `cd`; generated types cross a folder boundary | the same as A, plus path drift between roots | same as A |
| C. Several repos (sim, server, client) | one per part | high: versions of the Wasm build and the server must be matched across repos | version skew breaks the identical-results rule (R1-DET-REC6) and the CLI-library pin (R1-BND-F2) | poor |

### Package manager for the TypeScript side
| Option | What we know | Cost | Risk |
|---|---|---|---|
| npm workspaces | read (F8); the legacy toolkit already uses npm with a lockfile and `npm ci` (TL17) | none new | the read page is marked "(Legacy)" for npm 11 |
| pnpm or Yarn workspaces | not read (pnpm.io, yarnpkg.com are outside the allow list) | a second tool on every machine | unknown |

### Where generated types live
| Option | Trade-off |
|---|---|
| Committed into one TypeScript package, regenerated and diffed in CI | reviewable diffs; a TypeScript-only check runs without cargo; needs a CI diff check (R1-DATA-REC3 already chose this for the schema) |
| Generated at build time, never committed | no drift, but every TypeScript build and editor session needs the Rust toolchain; diffs of the boundary are invisible in review |

## Recommendations

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
  Rests on: R5-LAYOUT-F1, F2, F3, F5, F6, F8; legacy TL17.
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
  Rests on: R5-LAYOUT-F9, F13, F14, F15; R5-TSLINT-F14; legacy TL4, TL7, KT13.
  Serves: §6.1–§6.8 as listed per crate; the editor split serves §6.7 (the faction content pipeline is authored there).
  A spike must prove (proposed): each planted violation fails the gate on the right check and nothing else: `use` of the
  transport crate inside `sim`; a `tokio` dependency added to `sim`; an `import` of an `editor` module in `client`; a
  dynamic `import()` of the editor in `client` (only the manifest walk sees a path built at run time); controls (a
  legal import of `types` in `editor`) pass.
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
- **R5-LAYOUT-REC4. The sim crate is built with one feature set on every target, and the gate checks it.** The `sim`
  crate exposes no feature that changes maths; dependencies it shares with `server` get their maths-relevant features
  set once in `[workspace.dependencies]`; the determinism suite (R5-TEST-REC2) tests the native sim built as the server
  builds it, not as a stand-alone crate.
  Reason: Cargo unifies features over everything built together (F4), so the server's choices can change the native
  sim; wave 1 already flags target-dependent maths in a dependency (R1-RR-OQ1) and forbids non-deterministic float paths
  (R1-DET-REC2).
  Rests on: R5-LAYOUT-F4; R1-DET-REC2, R1-RR-OQ1 (cited).
  Serves: §6.2 (identical sim on client and server).
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
  Serves: §6.2 (the sim in a Worker on the client), §6.7 (the editor entry).
  A spike must prove (proposed): with the flag, `crossOriginIsolated === true` in Chromium, Firefox and WebKit under
  Playwright on both `vite` and `vite preview`; without it, `false`; the built player page loads no file from `editor`
  (REC2's manifest walk).
- **R5-LAYOUT-REC6. The phase 1 move keeps the legacy oracle runnable.** The five game files move to `legacy/`;
  `tools/` (preview.js, sim.js, battery.js, tuning-dump.js, the gate) moves with them or learns the new path, so today's
  gate keeps guarding critical fixes to legacy and the legacy oracle can still produce golden values (R5-TEST-REC3) until
  legacy retires; `.gitignore` adds `target/`, `pkg/`, `dist/`, `coverage/`, `playwright-report/`, `test-results/` and
  proptest's and insta's pending files.
  Reason: legacy stays frozen with critical fixes only, so it still needs its gate; the golden tests need it to run
  (TL11); KT19 asks for the ignore list to follow this layout; one `target` folder per workspace (F1).
  Rests on: R5-LAYOUT-F1; legacy TL11, KT19, KT21, TL1.
  Serves: none directly (it protects the legacy build and the golden data for §6.3's tuning tables).
  A spike must prove (proposed): after the move, today's gate run on `legacy/` passes, and the four Phase 0 mutation
  proofs (a deleted function, an unguarded editor call, a syntax error, a cross-file duplicate) still fail on the right
  checks; `node tools/preview.js legacy eval tools/tuning-dump.js` reproduces `docs/legacy-tuning.json` byte for byte.

## Open questions
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

| Recommendation that hinges on a source not read | Source | Reason not read |
|---|---|---|
| REC1 (package manager choice) | https://pnpm.io/workspaces | outside the allow list |
| REC1 (package manager choice) | the current npm major's workspaces page | not found in this run; only v11 "(Legacy)" read |

## Sources read (R5-LAYOUT)
S1 workspaces, S2 Rust releases, S3 resolver, S4 features, S5 npm workspaces, S6 Vite features, S7 Vite build, S8 Vite
server and preview options, S9 project references, S26 ts-rs, S27 cargo-deny bans, S29 rustup overrides, S60 Vite build
options, S61 Vite backend integration (full list in the notes, `.checkpoints/v2-research/test-engineer-r5.md`).
Not read: pnpm.io and yarnpkg.com (outside the allow list).

## Cross-topic notes
- engine-architect (ADR-0002): the crate list and the dependency rules; wasm-pack or the CLI (OQ2).
- infra-engineer (R5-CI): one `target` folder per workspace is the Rust cache's unit (F1); the toolchain file is what
  CI installs (F6).
- rust-reviewer: REC4's feature check.
