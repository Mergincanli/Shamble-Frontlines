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
- B2 finding (F3): the Worker's report reaches the page only from the frame loop: boot.ts:112-116 sends `requestReport()` from the frame callback at most every 250 ms (READOUT_EVERY_MS, boot.ts:18), and the reply is handled between frames. So `report(page).worker` is up to max(250 ms, one frame) + one frame old; at 3 fps here that is ~0.9 s, at 2 fps ~1.25-1.5 s.
- B2 finding (F4, the worst): hidden-tab test skeleton.spec.ts:78-83 reads `hidden` 1000 ms after hiding; if the Worker report on the page is still from before the Worker stopped (F3), the read 2 s later shows a higher count and `toBe(hidden)` fails: a false red on a page slower than ~3 fps. Same staleness hits :88 (`worker.pauses` is counted on RESUME, pacing.ts:118-126) and :90 after only 1500 ms (:85).
- B2 finding (F5): skeleton.spec.ts:91 `(ticks - hidden) <= 60` is a fixed 2 s budget for a window of 1.5 s + two evaluate calls behind slow frames; Worker ticks never run before due (pacing.ts:92), so the honest bound is 30 x the measured seconds + 1; on a slow runner the fixed 60 can be crossed (false red), on a fast one it is looser than needed.
- B2 finding (F6): playwright-core 1.65.0 pins Chromium revision 1254 = 158.0.8127.18 (node_modules/playwright-core/browsers.json:5-8); the container ran /opt/pw-browsers/chromium-1243 (Chromium 153 per the report). Without SHAMBLE_CHROMIUM (executablePath), headless Chromium runs `chromium-headless-shell` (coreBundle.js:45216 `return options.headless ? "chromium-headless-shell" : "chromium"`), a different binary from the full chrome tuned here. Allocation windows (V8's sizes, the ~470-block optimisation point) were tuned on 153's V8 only.
- B2 finding (F7): Vite's dependency cache exists here (packages/client/node_modules/.vite/deps, made 2026-11-19 00:23): every isolation dev-server run here was warm; a fresh CI checkout starts cold (pre-bundling three on the first dev-server test, possibly a reload). Read its duration on the first run.
- B2 finding (F8): playwright 1.65 supports a per-project `workers` (types/test.d.ts:765-802; common/index.js:634): `workers: 1` on the chromium project is one line.
- B2 source (read 2026-11-19): playwright.dev/docs/test-timeouts: "Playwright Test enforces a timeout for each test, 30 seconds by default" (covers the test function, fixture setup, beforeEach); auto-retrying assertions "a separate timeout, 5 seconds by default"; no global timeout by default; the page does not say what an action timeout longer than the test timeout does (the test timeout is the outer bound by construction: the test is torn down at 30 s). Sure: high (docs + the code's 3e4).
- B2 finding (F9): CI sets CI=true inside the cut (toolchain.yml env -i list), so Playwright's default reporter becomes "dot" (playwright/lib/common/index.js:724 `process.env.CI ? "dot" : "list"`): dots only, no test names or durations (the dot reporter does pass test stdout through, runner/index.js:3436-3440). The CI line needs `--reporter=list` to show each test's duration. CI=true also turns on forbidOnly (playwright.config.ts:15): a stray test.only fails the run instead of silently running one test.

## Block 2. TE-NT6: the browser checks that depend on the machine's speed (DONE)

The runner (docs.github.com, read 2026-11-19): ubuntu-24.04, public repository: 4 CPU, 16 GB, no GPU, processor model not
given. This container: 4 cores (Xeon 2.10 GHz), 15 GB, Chromium 153 full binary (`SHAMBLE_CHROMIUM`, headless),
SwiftShader WebGL 2, 3 to 8 frames a second, 2 Playwright workers (the default, 50 % of the cores: F2), a warm Vite cache
(F7). Defaults in force (no line in playwright.config.ts sets them): test timeout 30 s, expect timeout 5 s, webServer
start 60 s (F1). Two structural facts drive most rows: the page asks the Worker for its report only from the frame loop,
so the Worker's numbers on the page are up to one 250 ms period plus one frame old (F3); and every window below is a
fixed wall-clock time, while the page's frame rate and the Worker's 30 ticks a second are what the counts depend on.

| # | file:line | what, value | measured here | risk on a runner (direction) | verdict |
|---|---|---|---|---|---|
| 1 | report.ts:31-39, used by skeleton.spec.ts:30, allocation.spec.ts:30, isolation.spec.ts:28 | `running()`: >= 90 blocks (30 in isolation), wait bound 60 s; but the test timeout is 30 s, so the 60 s is never reached (F1) | passes; the 12 tests took 49.4 s with 2 workers | slow page start (cold Vite for isolation, F7; slower WebGL init) ends as "Test timeout of 30000ms exceeded" (false red) | ROBUST: `timeout: 60_000` on the chromium project (1 line): the wait's own bound becomes the real one; no assertion changes |
| 2 | skeleton.spec.ts:46 | abs(blocks - tick) <= 1 | holds | none: both counts are the page's, read in one evaluate; counts, not time | KEEP |
| 3 | skeleton.spec.ts:49 | wakes <= 1.1 x ticks + 2 | "about one wake per tick" | low: a slower Worker wakes late, which runs more ticks per wake (fewer wakes): easier, not harder | KEEP; print wakes and ticks |
| 4 | skeleton.spec.ts:51, :54 | 2000 ms window, frames > 3 | 3 to 8 fps: about 6 to 16 frames | false red only below about 2 fps (2 browsers on 4 CPU, no GPU) | KEEP (floor needed so the every-other-frame plant is caught by :56); print frames, posts and the window's real ms; with row 13 the load drops |
| 5 | skeleton.spec.ts:55-56 | posts <= frames, frames - posts <= 1 | holds | none: two counters of one evaluate | KEEP |
| 6 | skeleton.spec.ts:42-43 | toHaveAttribute/toContainText, expect 5 s | passes | none: the page is already running (row 1) | KEEP |
| 7 | skeleton.spec.ts:78-79, :83 | hidden read 1000 ms after hiding; "not one tick in 2 s" | 3 runs green | THE WORST (F4): at about 2 fps the Worker report on the page can still predate the stop, so :83 sees a higher count 2 s later: false red | ROBUST: before reading `hidden`, also wait for 4 new page frames since the hide (a report request and its reply both ride frames, F3): +2 lines; the 2 s no-tick window stays |
| 8 | skeleton.spec.ts:85, :88, :90 | 1500 ms after showing: worker.pauses == 1 (counted on resume, pacing.ts:118-126), ticks > hidden | 3 runs green | same staleness as row 7: a report from before the resume gives pauses 0 or no new tick: false red | ROBUST: the same 4-new-frames wait after showing: +1 line |
| 9 | skeleton.spec.ts:91 | ticks since showing <= 60 (2 s at 30 Hz) for a 1.5 s window plus two evaluate calls | 3 runs green | F5: on a slow page the window stretches toward 2 s and past it (false red); on a fast one 60 is looser than needed | ROBUST, and tighter: bound by the measured time, `<= Math.floor(30 * seconds) + 2`, seconds from a `Date.now()` taken before showing to one after the read (ticks never run before due, pacing.ts:92): +2 lines, 1 changed; still catches a Worker that catches up the hidden 3 s (about 90 ticks) |
| 10 | skeleton.spec.ts:105 | the notice's text, expect 5 s, after `boot()` has started the picture (boot.ts:39-43) and then the Worker (:49) | passes | the WebGL fallback's start on a slower CPU counts against 5 s: false red | ROBUST: `{ timeout: 20_000 }` on that one expect: 0 added lines |
| 11 | skeleton.spec.ts:107-108 | errors == 1 and no page error, read once right after the notice | passes; the double-count plant was caught here | not a flake: a second count that lands after the read passes unseen, more likely on a slow page (a weaker negative there, not a red) | KEEP; note (Low): cs3 may read again after a short settle |
| 12 | allocation.spec.ts:40, :55 | 8000 ms window under a 64-byte sampling heap profile, frames > 20 | 27 to 30 frames "under load" (report:442-443): 26 to 33 % headroom | THE SECOND WORST: about 25 % slower per frame and it fails (false red). The floor is the method's statistics (about one 64-byte sample per frame; fewer frames make the floor at 25 bytes trip on noise), not a speed claim | KEEP the bound; ROBUST by row 13 (`workers: 1`: the 27 to 30 were measured with another spec's browser beside it); print the window's real ms and fps; decide on the first run's reading, never by lowering 20 blind |
| 13 | playwright.config.ts (no `workers`) | 2 workers on 4 CPU (F2): files run beside each other | tuned under that load | load from a second browser, SwiftShader's threads, the Worker: the main multiplier on every row above | ROBUST: `workers: 1` on the chromium project (F8; 1 line); cost: the run goes from about 50 s to an estimated 70 to 90 s (not measured) |
| 14 | allocation.spec.ts:54 | window ends before block 420 | ends near block 340 | low: about 336 + 60 x (frame seconds) + polling overshoot; frames > 20 fails long before this one | KEEP (it guards the method: the listener is optimised near block 470); before/after blocks already printed |
| 15 | allocation.spec.ts:56 | blocks > 200 in the window | about 240 | low: blocks keep real time; only Worker drops (a wake late by more than 233 ms; max lateness here 43.9 ms) or blocks still queued behind a frame cut it | KEEP; print the Worker's drops |
| 16 | allocation.spec.ts:61-64 | 25 < bytes/frame < 170; 430 < bytes/block < 610 | 40 to 90; 480 to 530 | not speed: the browser's V8. CI's Chromium is not the container's 153 (Playwright 1.65.0 pins 158.0.8127.18, F6); V8's object sizes and its optimisation point can move these | KEEP; the first run prints them (it does already) with the browser's version; a red here is a reading for the plan, not a retune in this PR |
| 17 | allocation.spec.ts:66 | rate < 100,000 bytes a second (divides by the nominal 8 s) | about 16 KB/s | none (6x headroom; a longer real window only overstates the rate) | KEEP |
| 18 | clock.spec.ts:11-13, :17 | `start` read when the report hook appears, then 2000 ms, needs >= 30 ticks | passes ("no: ... real time") | the hook appears before the Worker has loaded the Wasm (boot.ts:49, :110); a start slower than about 1 s gives < 30 ticks and fact "unclear": false red | ROBUST: wait for the first block (tick >= 1) before reading `start`: 1 line changed |
| 19 | file-url.spec.ts:13 | 3000 ms, then whether the page ran | Chromium refuses the module script from a file (CORS): deterministic | none in Chromium; for a browser that does run from a file (Firefox later) 3 s is a race | KEEP for Chromium; note for the Firefox question |
| 20 | isolation.spec.ts:22-28 | dev and preview servers in the test process; `running(page, 30)`; port 4173 is serve.mjs's, so preview takes the next free one (resolvedUrls read) | passes, warm cache (F7) | cold pre-bundling of three on the first dev-server test (and a reload if a dependency is found late) counts against the 30 s test timeout | KEEP; row 1's 60 s covers it; read the four durations on the first run; a red with "Execution context was destroyed" points to a late-found dependency |

Retries stay 0 (playwright.config.ts:14). Nothing above is retuned from a guess: the only bounds that move are the
two timeouts (rows 1 and 10, which assert no speed), and row 9's fixed 60 becomes the measured time's own bound.

**What the first runner run must print** (so a tuning is made from readings):
1. Environment, once: `nproc` and the CPU model (infra's shell line), the browser's version and binary (full Chromium
   or the headless shell, F6), Playwright's "Running 12 tests using N workers" line (the reporter prints it).
2. Per test: its duration and status: `--reporter=list` on the CI line (F9; the CI default "dot" prints neither).
3. skeleton, both backends: the backend and adapter (the page's own console line, boot.ts:45), frames and posts in the
   window with the window's real ms, wakes and ticks, the page's fps and frame p50/p99, the Worker's lateness p50/p99/
   max and drops. One `console.log` line of those fields from `later`, printed before the expects.
4. hidden tab: `hidden`, `stillHidden`, `after` ticks, the frames waited for, the measured seconds of row 9.
5. allocation: the existing line (bytes per frame and per block, rate, frames, blocks from/to) plus the window's real
   ms and the Worker's drops, still printed before the expects.
6. clock: the existing line (ticks at start, after real time, after the fake minute).
7. broken Worker: the ms from `goto` to the notice (one line each).
8. isolation: the four durations (the list reporter).
A test that fails must still print its line: every print sits before the test's first `expect` on those numbers.
- B2 DONE (section written above). Next: B3 (RR-N4 + VF-L5).
- B3 source (read 2026-11-19): doc.rust-lang.org/cargo/reference/environment-variables.html: CARGO_TARGET_DIR and CARGO_BUILD_TARGET_DIR both name the output folder (build.target-dir); RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, CARGO_BUILD_RUSTFLAGS and CARGO_TARGET_<tuple>_RUSTFLAGS pass flags to rustc; RUSTC replaces the compiler; RUSTC_WRAPPER / RUSTC_WORKSPACE_WRAPPER wrap it; CARGO_PROFILE_<name>_OVERFLOW_CHECKS turns overflow checks on/off; CARGO_HOME moves cargo's cache (must pass: CI sets it); CARGO_INCREMENTAL forces incremental on/off (not in the pattern: changes codegen partitioning, so bytes, not overflow semantics; Low aside for rust-reviewer, not this test). The page does not state the flag-vs-variable precedence. Sure: high.
- B3 finding: scratch te-cargo-overrides-try.mjs reads the pattern's text from build.mjs: the proposed 15 refused + 15 allowed names PASS on today's pattern, and all 7 mutants (no lookahead; lookahead without $; no CARGO_PROFILE family; no ^; no final $; no RUSTC_WORKSPACE_WRAPPER; no WASM_BINDGEN family) are caught.
- B3 finding (Low aside): the pattern is case-sensitive; on Windows env names are case-insensitive, so `Rustflags` would reach cargo unrefused there. The build runs on Linux only (container, CI): no change now; the test must not pin lowercase names as allowed (it would freeze the gap).

## Block 3. RR-N4 with VF-L5: a test of the build's refusal of cargo settings from the environment (DONE)

**Where the pattern lives.** Today `CARGO_OVERRIDES` is private to `packages/client/scripts/build.mjs:39-47` and used
inline at :68-69, after the tool's check (:65). A test that calls `buildWasm()` would need the real tool, so the pattern
moves to `build-checks.mjs`, whose header already says its checks are "each tested without a build", beside a pure
function the build calls. The test goes into the existing `build-checks.test.mjs`, which the client's `vitest run` picks
up (run here: 6 files, 48 tests, `scripts/build-checks.test.mjs` among them) and which CI already runs through
`npm test --workspaces` inside the network cut (toolchain.yml). **No new CI line.** Runs in well under 10 ms.

**Exact symbols.**
- `build-checks.mjs`: add `export const CARGO_OVERRIDES` (moved from build.mjs with its comment; "which `--target-dir`
  below overrides" becomes "which the build's `--target-dir` overrides") and
  `export function refusedBuildSettings(env)`:
  ```js
  /** The names in `env` that the Wasm build refuses, in the environment's order. Only names are read, never a value. */
  export function refusedBuildSettings(env) {
    return Object.keys(env).filter((name) => CARGO_OVERRIDES.test(name));
  }
  ```
- `build.mjs`: delete lines 39-47; add `refusedBuildSettings,` to the import from `./build-checks.mjs` (after
  `lockedWasmBindgenVersion,`); line 68 becomes `const overrides = refusedBuildSettings(process.env);`.
- `build-checks.test.mjs`: add `refusedBuildSettings` to the import (after `MAX_SERVED_BYTES,`), and one test:
  ```js
  test("the Wasm build refuses cargo's and the tool's settings from the environment, and lets the build folder and CI's names through", () => {
    // One name per family of the pattern; the last two: a build-folder name with more after it is refused.
    const refused = [
      "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTC", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "RUSTUP_TOOLCHAIN",
      "CARGO_BUILD_RUSTFLAGS", "CARGO_BUILD_TARGET", "CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS", "CARGO_PROFILE_RELEASE_OPT_LEVEL",
      "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS", "WASM_BINDGEN_KEEP_DEBUG", "RUST_LOG", "CARGO_TARGET_DIR_X",
      "CARGO_BUILD_TARGET_DIRX",
    ];
    // The two build-folder names (the build's --target-dir beats them), every name CI's network cut sets (toolchain.yml,
    // its env -i list, plus the tool's path), and two names that only contain a refused one.
    const allowed = [
      "CARGO_TARGET_DIR", "CARGO_BUILD_TARGET_DIR", "HOME", "PATH", "USER", "CI", "LANG", "PROBE_IP", "CARGO_HOME",
      "RUSTUP_HOME", "RUSTUP_AUTO_INSTALL", "npm_config_update_notifier", "SHAMBLE_WASM_BINDGEN", "XRUSTFLAGS", "RUSTFLAGS_X",
    ];
    const env = Object.fromEntries([...allowed, ...refused].map((name) => [name, "x"]));
    expect(refusedBuildSettings(env)).toEqual(refused);
    expect(refusedBuildSettings({})).toEqual([]);
  });
  ```
  When infra-engineer's final CI text adds names to the env -i list (the tool's path, the browser, the build folder),
  they join `allowed` in the same commit: the test then also pins that CI's own environment passes the refusal.

**The cases, and what each catches** (all checked by the scratch `te-cargo-overrides-try.mjs` against today's pattern:
PASS; and against 7 mutants: each caught): each of the 9 alternatives once (CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS is the
one that matters most: it is the overflow checks); the negative lookahead (the two build-folder names pass; with the
lookahead gone they are refused: caught); the lookahead's own `$` (`CARGO_TARGET_DIR_X`, `CARGO_BUILD_TARGET_DIRX`
refused: caught when the `$` is dropped); the outer anchors (`XRUSTFLAGS`, `RUSTFLAGS_X` pass: caught when `^` or `$`
goes); the harmless names (CI's own: a pattern widened by mistake to `CARGO_.*` or `RUSTUP_.*` would break CI's build
and is caught here first).

**Added lines:** build-checks.mjs +14 (9 moved, 5 new), build.mjs -9 +1 (1 changed), build-checks.test.mjs +19.
Net about +25, of which 19 are the test.

**Not a CI line, but evidence for the report** (the main session, once, in its build step; it stops before cargo runs,
so it writes neither `target/` nor `pkg/`): `RUSTFLAGS=-Coverflow-checks=off SHAMBLE_WASM_BINDGEN=<tool> node
packages/client/scripts/build.mjs --wasm-only` must exit 1 with "build settings from the environment are refused: unset
RUSTFLAGS" (proves the wiring the unit test does not see). Not run by me (no build in this pass).
- B3 DONE (section written above). Next: B4 (E2E-TYPES: run tsc on the e2e specs).
