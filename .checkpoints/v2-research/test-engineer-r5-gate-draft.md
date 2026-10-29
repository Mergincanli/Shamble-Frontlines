# R5 draft, question 7: the gate v2 that replaces today's gate (R5-GATE)

Brief question (quoted): "the gate v2 that replaces today's gate". Task scope: what today's gate checks and why
(tools/gate.mjs, checks 1–4 and 5a–5d), what the v2 gate must check (both build targets, tests, lints, determinism
hashes, size and time budgets), how it runs locally and in CI, what carries over. Research only: no gate code, no tool
or CI change. Sources read 2026-10-28; the repository was read on branch cs/v2-research.

## Findings

### Today's gate (the repository, read 2026-10-28)
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

### Web findings
- **R5-GATE-F1. cargo-mutants inserts bugs and reports where no test fails.** Source:
  https://github.com/sourcefrog/cargo-mutants (README; no version shown) · official (the project) · read 2026-10-28 ·
  confirmed in the source: "Inject bugs and see if your tests catch them!"; it helps by "finding places where bugs could
  be inserted without causing any tests to fail."; one file at a time with `cargo mutants -f src/something.rs`.
- **R5-GATE-F2. Vite's chunk size limit only warns.** Source: https://vite.dev/config/build-options (v8.3.4) · official ·
  read 2026-10-28 · confirmed in the source: `build.chunkSizeWarningLimit` is the "Limit for chunk size warnings (in
  kB)." (default 500); `build.reportCompressedSize` "Enable/disable gzip-compressed size reporting." (default `true`).
  Consequence (inferred): a size budget that fails the build is a check of our own.
- **R5-GATE-F3. cargo-deny has four check classes; the advisories check reads an advisory database that also lists
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

### Legacy facts used
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

### How the gate is run
| Option | Shape | Fit | Risk |
|---|---|---|---|
| A. One Node orchestrator (`node tools/gate.mjs`, v2 edition) that runs named checks and calls cargo, tsc, ESLint, Vitest and Playwright | continuity with today's command, base ref, exit codes and `--json`; Node is needed anyway for Vite; works on Windows | one more script to keep (with its own mutation proofs) |
| B. `cargo xtask gate` (a Rust binary) | one language for the gate and the sim | Mustafa's machine and every agent need the full Rust toolchain for any check, even a TypeScript-only one |
| C. A generic task runner (make, just) | familiar to many | not read; make is not native on Windows; no exit-code or base-ref conventions |
| D. CI only (no local gate) | nothing local to keep | breaks CLAUDE.md's protocol (the gate must PASS before commit) and slows every step |

### When long checks run
| Option | Trade-off |
|---|---|
| Everything on every PR | the strongest guard; PR wall time grows with soak, fuzz and Safari |
| Fast set on every PR, long set nightly and before playtests | PR stays inside the CI budget (R5-CI-REC5); a long-run failure lands a day later, as an issue against main |
| Path-based selection inside the gate (checks pick themselves by the files a change touches) | saves time on TypeScript-only PRs; the selection rule itself must be tested (a wrong rule skips a check silently) |

## Recommendations

- **R5-GATE-REC1. The gate v2's checks, in order, fastest first, each with an id and a reason.**
  | Id | Stage | Check | Why (rests on) |
  |---|---|---|---|
  | G0 | setup | toolchain equals `rust-toolchain.toml`; Node equals the pin; `cargo --locked` and `npm ci` succeed; exit 2 on any setup failure | TL1; R5-LAYOUT-F6 |
  | G1 | format | `cargo fmt --all --check`; `prettier --check` on `packages/**` | R5-RUSTLINT-REC5, R5-TSLINT-REC3 |
  | G2 | Rust lint | clippy `-D warnings` on host and on `wasm32-unknown-unknown`; the determinism and panic lint sets | R5-RUSTLINT-REC1, REC2, REC3; R1-ECS-REC2 mark 2; R1-DET-REC2 |
  | G3 | TypeScript | `tsc -b`; ESLint typed lint, `--max-warnings 0` | R5-TSLINT-REC1, REC2 |
  | G4 | boundary | generated types and schema regenerate with zero diff; wasm-bindgen CLI equals the library in `Cargo.lock` | R5-LAYOUT-REC3; R1-BND-REC4, R1-DATA-REC3, R1-BND-F2 |
  | G5 | dependency rules | cargo-deny bans (sim ↛ transport, async runtime), `multiple-versions` and `wildcards` at deny with a reviewed skip list; same-feature check for `sim` | R5-LAYOUT-REC2, REC4; R2-TRN-REC4 mark 3; F4 |
  | G6 | build | native release (server, devtools); `sim-wasm` to Wasm; Vite build of both entries | — |
  | G7 | budgets | Wasm bytes (raw and gzip), JS bytes per entry, against a committed budget file; fail above it | F2; R1-RR-OQ5 |
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
  Serves: the gate of every §6 item; G10 serves §6.2, §6.6, §6.8; G11 serves §4's fairness target.
  A spike must prove (proposed): see REC3 (every check fails on its planted defect).
- **R5-GATE-REC2. One command, the same locally and in CI, carrying today's conventions (option A).** `node tools/gate.mjs`
  (v2 edition) with today's base-ref rule, exit codes 0, 1 and 2, `--json`, and the TL3 habit as `--allow-removed` (a
  list of exported Rust items and TypeScript exports the approved plan removes; the generated-types diff of G4 shows
  them). `--quick` runs G0–G5 and the tests of changed crates and packages (local use between steps; never for a
  commit); the full run is the default. CI runs the same check ids as parallel jobs (R5-CI-REC5's option G1) and the
  final aggregate job compares the set of check ids that ran against the gate's own list, so a check cannot silently
  drop out of CI. Today's gate keeps running on `legacy/` changes while legacy lives (R5-LAYOUT-REC6).
  Time budget (proposed): `--quick` under 60 s warm; the full local run under 10 minutes warm on a recent desktop; CI
  as R5-CI-REC5 proposes (10 minutes p50 warm, 20 minutes cold).
  Reason: TL1 and KT7 ask to keep the base ref, the exit codes and "never a silent skip"; Node runs on Mustafa's Windows
  machine without the Rust toolchain for TypeScript-only work (options B and D fail CLAUDE.md's protocol or that need);
  a check missing from CI is a silent skip by another route.
  Rests on: legacy TL1, TL3, KT7, KT8; R5-CI-REC5 (folded part).
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
  | G7 | a 200 KB constant table added to the Wasm build | a change inside the budget |
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
- **R5-GATE-OQ1. Which checks run on a TypeScript-only or Rust-only PR?** Path-based selection saves time but can skip
  a check silently. Decides it: the first month's CI times against R5-CI-REC5's budget; until then every check runs.
- **R5-GATE-OQ2. WebGPU in the end-to-end check.** Standard runners have no GPU (R5-CI-OQ5); G12's counts may run on
  the WebGL 2 fallback only. Decides it: graphics-engineer with R3-MEASURE.
- **R5-GATE-OQ3. Where the gate v2 lives while legacy's gate stays.** Both in `tools/`, or the legacy gate inside
  `legacy/`. Decides it: the phase 1 restructure plan (R5-LAYOUT-OQ5).
- **R5-GATE-OQ4. Advisories: in the PR gate or only scheduled and on lockfile changes?** F3 says the verdict can change
  without our change; R1-RR-OQ7 asks for cargo-audit and cargo-deny as CI gates. Decides it: security-auditor with R6's
  security baseline.
- **R5-GATE-OQ5. `--prove` cost.** Rebuilding per mutation may take long; whether mutations can share one build is
  unmeasured. Decides it: the spike's REC3 time mark.

| Recommendation that hinges on a source not read | Source | Reason not read |
|---|---|---|
| REC2 (option C ruled out) | the docs of just and make | not fetched (just.systems outside the allow list) |
| REC3 (cargo-mutants details: diff mode, timeouts) | https://mutants.rs/ | outside the allow list (README read instead) |

## Sources read (R5-GATE)
S27, S28, S59, S60 (notes file); repository: tools/gate.mjs, tools/README.md, .github/workflows/gate.yml,
docs/START_HERE.md §1 and §5, docs/reports/2026-10-14-gate-check-5.md, docs/LEGACY_INVENTORY.md TL1–TL17, KT7–KT9,
KT19, KT21. Not read: mutants.rs, just.systems (outside the allow list).

## Cross-topic notes
- infra-engineer: REC2's aggregate job compares check ids (R5-CI-REC5); scheduled jobs; artefacts.
- security-auditor: OQ4; the gate tools and budget files belong to the stricter review category (TL17).
- graphics-engineer: OQ2; R3-MEASURE-REC3's baseline lives in REC4's budget file.
