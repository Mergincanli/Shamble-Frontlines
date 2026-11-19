Task: CI-WASM, the test side of its follow-ups and what "proven" means for it (test-engineer, part "followups").

_test-engineer, 2026-11-19. Branch claude/wonderful-hamilton-jdmlnf (main f5030a3 + checkpoint commits). Read brief-context.md first (done)._

## Plan (blocks, in order)
1. Read: STATUS_ARCHIVE block "From change-set 2, the first page (PR #61)" (CI-WASM group + cs3 group for context); cs2 report (Evidence, Not verified); five specs + report.ts; playwright.config.ts; build.mjs, build-checks.mjs + tests; root Cargo.toml; sim-wasm.test.mjs.
2. TE-NT6: every speed/timing-dependent assertion in the five specs (file:line, value, measured here, runner risk, verdict, what the first runner run must print).
3. RR-N4 + VF-L5: smallest test of CARGO_OVERRIDES (where the pattern lives, cases, test file, which command runs it, symbols, lines).
4. E2E-TYPES: tsc flags that type-check packages/e2e/tests/**/*.ts + playwright.config.ts today (run it), errors, cost, the CI line, what it catches.
5. SA-TREE: test that fails when a shipped-Wasm runtime crate has no overflow override (how it reads the tree, where, time, network cut, lines; here or cs3).
6. RR-F5 + SA-DIST briefly.
7. What "proven" means: runs green on a runner, planted changes (red), take-backs, report quotes; per follow-up recommendation + lines + total.
Then: open questions for Mustafa; Not verified; DONE.

## Notes (one line per finding / finished block)
- B1 note: CI-WASM group (STATUS_ARCHIVE.md:1073-1083) = TE-NT6, RR-N4, RR-F5, E2E-TYPES, SA-TREE, SA-DIST; VF-L5 sits in the cs3 group (STATUS_ARCHIVE.md:1105) "with RR-N4".
- B1 note: cs2 report readings: container Chromium 153 headless, SwiftShader WebGL 2, 3 to 8 frames a second; allocation window 27 to 30 frames vs the check's 20 "under load here" (report:442-443); 12 passed x3; 49.4 s for the 12 (report:213); Worker tick lateness p50 0.77-0.93 ms, p99 5.6-10.8 ms, max 8.3-43.9 ms (report:275-277); hand-off delivery p50 96-166 ms = the page's slow frame (report:274).
- B1 note: this container: 4 cores (Xeon 2.10 GHz), 15 GB RAM, Node 22.23.4, @playwright/test 1.65.0, browsers in /opt/pw-browsers (chromium-1243, headless_shell-1243).
- B1 finding (F1): playwright.config.ts sets no `timeout`, no `workers`, no `expect.timeout`: defaults apply (test 30 s, expect 5 s, workers = half the logical cores, webServer 60 s). So `running()`'s waitForFunction timeout 60_000 (report.ts:38) can never be reached: the 30 s test timeout ends the test first. On a slow runner a slow start fails as "Test timeout of 30000ms exceeded", not at the wait's own line. To check in Playwright docs (B2).
- B1 finding (F2): workers default 50 % of logical cores -> 2 workers on a 4-vCPU runner and here; files run in parallel, so the allocation spec can share the CPU with another file's browser (the "under load" reading). Tests in one file run in order (fullyParallel not set).
- B1 note: versions here: vitest 5.1.0, typescript 6.0.3, vite 8.3.7, @playwright/test 1.65.0. Client `vitest run --root packages/client`: 6 files, 48 tests, 1.37 s (4.4 s wall) incl. scripts/build-checks.test.mjs (17 tests) and serve.test.mjs (16): a new *.test.mjs in packages/client/scripts/ is picked up by CI's `npm test --workspaces` (toolchain.yml, inside the cut) with no new line.
- B1 note: CARGO_OVERRIDES is a module-private const in build.mjs:44-47; build.mjs runs main() only when it is the entry (build.mjs:145), but importing it pulls `vite` (build.mjs:20) and does nothing else at load. The refusal is inline in buildWasm (build.mjs:68-69), after the tool check (build.mjs:65): a test of buildWasm itself would need the tool.
- B1 note: Cargo.toml:25-54: release profile overflow-checks on; "*" off; named overrides for cfg-if, thiserror, ts-rs, unicode-ident, wasm-bindgen, wasm-bindgen-shared ("compiled into our libraries and tests"; proc-macros and build scripts stay at "*"). The list covers libraries AND tests, wider than the shipped Wasm.
- B1 note: boundary tests (sim-wasm.test.mjs, 7 tests) import ../pkg/sim_wasm.js (needs the build); the package's `test` script is `vitest run --dir src`, so CI's npm test leaves them out. CI's strict tsc flags: toolchain.yml (strict=... line), --ignoreConfig, applied to packages/types only.
- B1 DONE. Next: B2 (TE-NT6), first the Playwright defaults (docs) and the runner's size (docs).
- B2 source (read 2026-11-19): Playwright 1.65.0's own code here: default test timeout 30 s (node_modules/playwright/lib/common/index.js:499 `defaultTimeout = 3e4`); workers default "50%" of os.cpus().length (same file :555, :660); expect timeout 5 s (lib/matchers/expect.js:13204); webServer launch timeout 60 s (lib/runner/index.js:1014). Sure: high (the code that runs).
- B2 source (read 2026-11-19): docs.github.com/en/actions/reference/runners/github-hosted-runners: standard Linux x64 (ubuntu-24.04) = 4 CPU, 16 GB RAM, 14 GB SSD for PUBLIC repositories; 2 CPU, 8 GB RAM for private ones; no GPU on standard runners (GPU only in "larger runners"); no processor model given. Sure: high (official docs). -> on the runner, 2 Playwright workers (as here, 4 cores); if the repository were private, 1 worker.
