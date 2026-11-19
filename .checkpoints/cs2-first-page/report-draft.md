# Advisor report: cs2-first-page (2026-11-19, branch claude/wonderful-hamilton-jdmlnf, base main b763ba3)

Change-set 2 of the build plan, "First page" (foundation step 2.5, first half). Pull request #61. Lane 1, the first
builder chat (session_019YNMUV9LFhASseJ9EP12kL, Fable 5.2; every agent Opus 5.5, the reviewers at effort max). The
branch stands in for `cs/cs2-first-page`.

## Goal
The straight path from the simulation to the screen: the Rust simulation, built as WebAssembly, runs in one Web Worker
at 30 ticks a second and hands the page one block per tick; the page, which never touches WebAssembly, draws an empty
lit ground with a 1 m grid on three.js's WebGPU renderer (WebGL 2 as the fallback) and names its backend. With it: the
small server that lets Mustafa open a build in Brave from a zip.

## Outcome
⟦DONE / PARTIAL⟧ ⟦one sentence, at the end⟧

What a person sees: an empty ground with a grid, in placeholder colours (not the look), and a corner readout: the
backend, ticks at 30 a second, blocks, frames, errors, the last pause, the commit. **In this container every run was on
the WebGL 2 fallback with software rendering: WebGPU itself, Brave and a real GPU are not verified here.** They are his
Brave check ("For Mustafa").

## What went wrong, and what was done about it
1. **A night was lost to an approval prompt nobody could answer.** On 2026-11-19 at about 00:33 UTC a command of the
   main session (one try of WebGPU on a software adapter: the test browser started with `--enable-unsafe-webgpu` and
   SwiftShader flags; an optional reading of the plan) raised an approval prompt. Mustafa had written ten minutes
   before that no permission was needed until 7 am; a prompt of the harness is not something his words can answer.
   The session sat until its worker restarted (seen at 07:13 UTC). Nothing was lost: the branch equalled its pushed
   state, and the saver's autosaves (45537a3, d019d82, 41d4dd9) held the reviewers' notes. The try was not made.
   The rule since: no command with unusual browser or system flags in an unattended stretch, and every agent's
   brief says so.
2. **The production build carried the development tools** (the readout's and the report's code and the test hook were
   built in, though not shown), found by this change-set's own production check at 00:31 UTC before any review. The
   build's switch was read as `import.meta.env["..."]`, which Vite does not fold. Fixed in 29378dd with constants
   Vite writes into the code (`define`); since d257bb6 the build itself refuses a production bundle that holds a
   development marker (`bundleProblems`).
3. **A fix of round 1 broke a failure path, and round 2 caught it.** d257bb6 bounded the error text with
   `message.replace(...)`; a Worker whose script does not load gives an error event with no message, so the hook
   threw, the Worker was not stopped and the page said nothing (engine-architect, round 2, Medium; measured on the
   built page: errors 2, nothing shown). Fixed in round 2's fix pass, with two browser tests that fail without it.
4. **The size ran over its plan twice.** Planned about 2,900 added lines with a stop line at 4,000. At 3,964 the
   session stopped and told him (SIZE-STOP; his answer "A": the line moved to 4,400). At 4,430, after round 1's
   fixes, it stopped again (SIZE-STOP-2) ⟦his answer⟧. What grew: tests and checks the reviewers asked for (the
   small server's tests 230 lines, the build's checks and their tests 366, the Worker's tests 636), not features.
5. **Two planted defects were first caught by a timeout, not by their check** (test-engineer, round 1). The planted
   defects were rewritten so that a plant only counts when the output shows the line that names it, and rerun twice.
6. **Step 0 of the plan ran before the advisor agent's APPROVE:** the reading of wasm-bindgen's health (two web
   pages, read by a fetch agent between 21:04:49 and 21:07:52 UTC on 2026-11-18) was made as reading and notes only.
   Its result (no written release policy) became the question WB-HEALTH.
7. **One time in the resume card was typed about half a minute ahead of the clock** (a run that ended at about
   08:16:35 UTC written as "to 08:17"); corrected in the next save.

## Plan and approval
- The plan: `cs2-first-page/plan.md` in the notes (commit ⟦NOTES-SHA⟧ still holds the notes folder; the last commit of
  this pull request deletes it). Drafted by engine-architect, adopted by the main session with every change listed
  (its section 20), four revisions.
- **The advisor agent: APPROVE with eight conditions** (2026-11-18, 21:05 to 21:27 UTC; seven Mediums, twelve Lows).
  Each condition and where it was met:

| Condition | Met by |
|---|---|
| 1. The overflow-checks question reworded or the register row followed | the row OVERFLOW-ALL is followed: six named overrides in the root `Cargo.toml`, their cost measured (below); no question |
| 2. WB-HEALTH put to Mustafa | asked 22:09 UTC, answered (a) |
| 3. Part 15.2 kept (the picture's interface members; the `e2e` folders) | kept: `picture.ts`, `packages/e2e/tests/skeleton/` and `tests/node/` |
| 4. The verifier spot-checks the web facts behind RNG-CRATE and the wasm-bindgen reading | FAIL on the first read (two Mediums, two Lows), all corrected; PASS on the re-check (21:50:04 UTC) |
| 5. The tool's route: checksum before unpacking, the member list, a new empty folder, the origin proven before any run, or stop and ask | security-auditor: the origin is provable here only in part, so it went to him as TOOL-SOURCE; his answer (a) for this one file; the route ran as written (below) |
| 6. The lockfile rule allows npm's flag-only changes and lists them | 37 entries changed in their `peer` flag only, listed in `deps-lock-compare.txt` |
| 7. netcode-architect in the round; art-director called or the reason written | netcode-architect reviewed; art-director not called: placeholder colours, no look decision (the plan's section 12) |
| 8. The padded runs and hidden-tab steps in the Brave checklist | steps 6, 7 and 10 of "For Mustafa" |

- **Mustafa's answers to the plan's five questions** (2026-11-18, 22:34:37 UTC; whole in `docs/DECISIONS.md`): RNG-CRATE
  (c), no random-number library now; WB-HEALTH (a), wasm-bindgen pinned to exactly 0.2.130; TOOL-SOURCE (a), "for this
  one file only"; DEPART-2 (a), all six departures from the build plan's text; CI-WASM (a), a small pull request of
  its own right after this one is merged. "The size as you plan it, about 2,900 lines with the stop line at 4,000, is
  fine."

## What changed
⟦FINAL-SIZE⟧ added lines in `crates/`, `packages/`, `.gitignore` and the two root manifests (the lockfiles and the
five-line fixture `render-counts.json` not counted), by `git diff --numstat origin/main...HEAD`: client ⟦n⟧,
sim-worker ⟦n⟧, client-endpoint ⟦n⟧, e2e ⟦n⟧, sim-wasm ⟦n⟧, sim ⟦n⟧, root 27, types 3.

**Rust** (commits 3833e3e, 6e76f30, 015a212, 29378dd)
- `crates/sim`: `world.rs` (the world: a `u32` tick and the 32-byte match seed; `authority_step` with `checked_add`;
  the canonical bytes, tick then seed, 36 bytes; `state_hash`), `state_hash.rs` (`fnv1a_64`, FNV-1a 64 bits, RFC 9923's
  test values; provisional until PM-0004-21). `TICK_HZ` 30, `SEED_BYTES` 32.
- `crates/client-endpoint`: `layout.rs` (`Section::align`, the `section` and `field` lookups, used in constants so
  that a misspelt name stops the build), `block.rs` (`BlockWriter`: one buffer, allocated once and never moved; the
  header written field by field, little-endian, no `unsafe`; `MAX_PAD_BYTES` 1 MiB), `schedule.rs` (`LocalSchedule`).
- `crates/sim-wasm`: `lib.rs` (the bindings: `Sim` with its constructor, `block_ptr`, `block_len`, `input`, `step`,
  `rebase`, `newest_input`; `tick_hz()`; `CreateError`; `MODE_LOCAL`). Per tick the Worker makes two calls, `input`
  and `step`, each plain numbers in and out (his WB-HEALTH words, quoted in the file).
- The root `Cargo.toml`: `wasm-bindgen = "=0.2.130"`; six `overflow-checks = true` overrides (cfg-if, thiserror, ts-rs,
  unicode-ident, wasm-bindgen, wasm-bindgen-shared), the list read from `cargo tree`.

**The Worker** (`packages/sim-worker`; 4c32a06, 015a212, the fix passes)
- `pacing.ts` (`Pacer`: at most 7 ticks per wake, a longer stall is dropped and counted), `block-pool.ts`,
  `read-block.ts` (`copyBlock`: one copy out of linear memory, the view made fresh each time), `stats.ts`, `messages.ts`
  (the message types and the guard for what the page sends), `host.ts` (`SimHost`: the tick loop, the hidden-tab
  pause, whole-millisecond sleeps rounded up), `worker.ts` (the entry: draws the seed, loads the Wasm),
  `boundary/sim-wasm.test.mjs` (the host and the copy on the real built Wasm, memory grown half-way).

**The page** (`packages/client`; 784ae3e, a7f785a, de63a30, the fix passes)
- `index.html`, `src/main.ts`, `src/app/` (`boot.ts`, `worker-link.ts`: the page's side of the hand-off, the layout-hash
  check, the input posted once per drawn frame; `error-hook.ts`; `flags.ts`), `src/picture.ts` (the interface file
  between the app and the picture, build plan Part 15.2), `src/render/` (`index.ts` the one entry, `three-picture.ts`,
  `empty-scene.ts`; the only folder that imports three), `src/overlay/` (the readout and the copy report; development
  builds only), `src/harness/` (frame times, the run's header), `src/styles.css`, `vite.config.mjs`.
- `serve.mjs` (the small server: 127.0.0.1 only; GET and HEAD only; only files of its own folder, of five types; a
  request whose Host header is not its own address gets 421; a strict content security policy on every answer) and
  its 16 tests. ⟦count⟧
- `scripts/build.mjs`, `build-checks.mjs`, `wasm-bindgen-tool.mjs` (the build: the tool's file is compared with its
  recorded size and sha256 before every run; the tool's version must equal the lockfile's; build settings from the
  environment are refused; the served files are scanned for what the policy would refuse; a production bundle may
  hold no development marker; the Wasm's glue may sit only in the Worker's chunk).

**The browser tests** (`packages/e2e`; a68e3da, c4a12eb, 74a0c24, the fix passes): `playwright.config.ts` (project
`node`, which CI's `npm test` runs, and project `chromium`, which needs a built page and is not run by CI yet),
`tests/skeleton/` (`skeleton.spec.ts`, `isolation.spec.ts`, `allocation.spec.ts`, `clock.spec.ts`, `file-url.spec.ts`,
`report.ts`, `render-counts.json`).

**Manifests and lockfiles** (cc9fc71, pushed through the hold): the nine files of the release record below.
`.gitignore`: `packages/sim-worker/pkg/`, `packages/e2e/playwright-report/`.

**Departures from the plan's symbol lists** (each small; none changes what crosses a boundary): `BlockWriter::new`
returns an `Option` (the padding is capped, because the number comes from outside); `SEED_BYTES` is public; `rebase`
returns the next due time; `Sim::input` takes the sequence number only, and `newest_input` and `tick_hz()` are added;
`block_ptr` returns `*const u8`; `mode` is a `u32`; the lookups return `Option`; `isPageToWorker`, `host.ts` (a file
of its own beside `worker.ts`) and `build-checks.mjs` (the build's checks as a module of their own) are added; the
boundary test is `.mjs`; `BlockPool.give` returns nothing and the pool counts `refused` beside `misses`;
`render/index.ts` with `createPicture` (engine-architect's F1); `WorkerLink.receive`; the Vite constants
`__SHAMBLE_DEVTOOLS__` and `__SHAMBLE_COMMIT__`.

## The new dependencies: the release record and the security answer it names
His words (2026-11-18, 20:20:09 UTC): "every ready report of a pull request that changes a manifest or a lockfile
quotes each release record and the security answer it names." One record: `cs2-first-page/dependency-check.md`, quoted
whole between the lines:

⟦RELEASE-RECORD⟧

**The security answer it names** (`cs2-first-page/security-auditor-deps.md`, security-auditor, Opus 5.5 at effort max;
its closing lines, quoted):

⟦SECURITY-ANSWER⟧

How its four conditions were met: (1) the nine files were committed by path, each equal to its reviewed sha256 (cc9fc71);
(2) every install was `npm ci --ignore-scripts` and every cargo command took `--locked`; (3) Vite's dev and preview
servers are bound to 127.0.0.1 in `vite.config.mjs` and vite is 8.3.7; (4) its F1 and F2 were written into the tool's
route before the download (plan revision 4, 23:08 UTC), F3 and F4 before the tool's first run (`wasm-bindgen-tool.mjs`,
`build.mjs`).

What entered: Rust: wasm-bindgen 0.2.130 with its macro, macro-support and shared crates, bumpalo 3.20.4, cfg-if
1.0.5, rustversion 1.0.23 (seven lockfile entries, no line removed). npm: three 0.187.0, @types/three 0.187.0 and its
six; vite 8.3.7 declared directly (it was in the lockfile as Vitest's peer).

**The wasm-bindgen tool** (his TOOL-SOURCE (a), "for this one file only"): downloaded once by the plan's route
(`cs2-first-page/tool-route.sh`, 2026-11-18, 23:09:31 to 23:09:33 UTC; output in `wasm-bindgen-cli-check.md`):
`wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz`, 11860397 bytes, sha256
9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f, equal to his values; six members; only the program
taken out: 8170784 bytes, sha256 ca4df773020dae87d92b9eadb2b85412bea7a9cf6318ab9999dd119fd9f46931, **equal to his
values.** It lies under `.gate/tools/` (not in git); its expected values are tracked in
`packages/client/scripts/wasm-bindgen-tool.mjs`, and `build.mjs` compares the file before every run. GitHub's own
check (`gh release verify-asset`) could not be run here either; by his CI-WASM answer it runs in CI before the tool's
first run there. No other file, version or checksum of outside code was taken.

## Evidence
⟦All on the final head FINAL-SHA unless a line says otherwise.⟧

**Checks, all exit 0** ⟦time⟧: `cargo fmt --all --check`; clippy `--workspace --all-targets --locked -- -D warnings`
natively and for `wasm32-unknown-unknown`; `cargo test --workspace --locked` (client-endpoint 15, sim 9, sim-wasm 6,
⟦others⟧); the release tests of sim and protocol; the wasm32 build of the five crates; the generated types written
again with no change; tsc with CI's strict flags on the page's and the Worker's shipped files with the lib check on
(0 errors against @types/three 0.187.0; two casts, both in `three-picture.ts`); the unit tests as CI's `npm test` runs
them (types 7, net 1, sim-worker 30, client ⟦48⟧, e2e's node project 1); the boundary tests on the built Wasm (7);
the development and the production build; `node tools/gate.mjs --no-smoke` (GATE PASS; the smoke check cannot start
in this container and runs in CI); `node .claude/scripts/repo-map-check.mjs`. CI on the pushed head: ⟦runs⟧.

**The browser tests** (the container's Chromium 153, headless, software rendering; `--project=chromium` on the built
development page served by `serve.mjs`): ⟦12⟧ passed, ⟦n⟧ runs in a row. What they hold:
- the page on the default backend and with `?backend=webgl`: the backend is named on the page and in the readout; one
  block per tick, each transferred (none left attached); about one wake per tick; the input posted once per drawn
  frame; errors 0; **zero calls into WebAssembly by the page** (every entry point on the page replaced by one that
  counts and throws; the Worker still loads the Wasm); no request to any address but the small server's; the
  renderer's counts equal the committed ones for the fallback (3 draw calls, 3 triangles, 62 lines, 3 geometries, 3
  textures, 6 shader stages); a screenshot per backend;
- a hidden tab: not one tick of the Worker in two seconds; on return one pause, nothing dropped, the count goes on;
- a Worker that cannot be loaded, and one that throws as it starts: the match stops once and the page says why;
- the isolation flag under Vite's dev and preview servers: `crossOriginIsolated` true with the flag and false without,
  on the page and in the Worker;
- the allocation method of PM-0002-7 (a sampling heap profile by function): about 40 to 70 bytes per frame posting
  the input, about 480 to 520 bytes per block received (the message event's own data and one view), about 16 KB a
  second; the mark itself is judged in change-set 5;
- two facts recorded: Playwright's clock does not reach the Worker (it kept ticking on real time), and opened as a
  file the page does not run in Chromium ("blocked by CORS policy") and the plain line shows.

**The production build** (`production-check.mjs`: PASS): five files, no development marker, no `serve.mjs`; the
development build has each marker (the control); the production page runs with one Worker and no error. Since d257bb6
the build makes this check itself.

**The planted defects** (`cs2-first-page/plants.py`; a throwaway worktree with its own build folder; a plant counts
only when the output shows the line that names it): ⟦22⟧ plants, each "FAILED as it must, at the line that names the
defect", and the clean tree passes the same checks (⟦run, times, file⟧). They are: the page calling into WebAssembly;
a block posted without its transfer list; a copy, and a host, that keeps a view of the Wasm's memory across a growth
(two plants); a header field added in Rust without regenerating (the export test, and the page refusing the block: two
plants); the tool's recorded version one patch off; the tool's recorded sha256 one digit off (the file is not run); a
misspelt field name in the writer (the build stops); a 64-byte array allocated per frame, and per block (two plants);
a Worker that ignores the hidden tab, and one that runs the hidden gap on return (two); a server that lets a path
out of its folder, and one that leaves a broken-off download's file open (two); an isolation flag with one header
missing; the input posted on every other frame; development code in a production build; the page importing the
Wasm's glue; a Worker's missing error message passed on; a Worker's error counted twice; an inline style the page's
policy refuses. Two ran differently from the plan's wording: plant 5 edits the recorded version (a stand-in tool can
never reach the version check, because the checksum refuses it first; the version mismatch itself is unit-tested);
the 7-tick cap is held by the unit and boundary tests, not by the browser test (the plant that runs the hidden gap on
return is caught by the drop count).

**Readings in this container, reported and not judged** (software rendering, 3 to 8 frames a second; a laptop will
differ):
- The Wasm file: 28,987 bytes (11,655 gzip -9, 9,955 Brotli 11); its glue ⟦n⟧. The page's script: about 825 KB (222 KB
  gzip, 180 KB Brotli), nearly all three.js's WebGPU build.
- The six overflow overrides cost 278 bytes of Wasm (about 1 %) and nothing measurable per tick (median 14.71 ns
  against 14.04 ns for `input` and `step` in Node, 7 runs of 2,000,000 ticks, the ranges overlapping).
- The hand-off as a curve over block sizes (8 bytes, then 1, 4, 16 and 64 KiB of padding; `readings-2.txt`, 08:09 to
  08:13 UTC): the Worker's copy 0.000 ms at p50 and 0.1 to 0.2 ms at p99 at every size (its clock steps by 0.1 ms);
  delivery to the page 96 to 166 ms at p50, which is the page's own slow frame here, not the block's size.
- The Worker's pacing: a tick starts 0.77 to 0.93 ms late at p50 and 5.6 to 10.8 ms at p99 (max 8.3 to 43.9); up to
  1 ms of every start (0.5 on average) is the whole-millisecond timer, by design (perf-auditor: on an idle page p99
  1.87 ms); at most 2 ticks in a wake; no drops. Before the timer fix the p50 was 3.7 ms.
- The page's type check (TypeScript 6.0.3, `--ignoreConfig`, warm, 3 runs): 4.46 to 5.42 s with the lib check on, 1.29
  to 1.36 s with `--skipLibCheck`.
- wasm-bindgen's generated `.d.ts` needs `--lib esnext.disposable` beside `es2023` when the lib check is on.
- The software renderer here does not draw a line that crosses behind the camera, so the ground ends in front of it.

## Reviews
Two rounds, within the budget. Round 1 on 30a5a13 (the build as first finished; netcode-architect on 29378dd), seven
reviewers; round 2 on what the fixes changed (`git diff 30a5a13..d257bb6`), six reviewers. **Nothing Critical; the
two Highs of round 1 are fixed and closed; nothing High in round 2. A third round was not due.**

| Reviewer | Round 1 | Round 2 |
|---|---|---|
| engine-architect | Mediums F1 (the app imported the renderer around the interface file), F2 (after a page-side failure the Worker ran on); Lows F3 to F14 | all fixed ones closed; no wrong-way crossing. New: Medium N1 (a Worker that cannot load: silent), Lows N2 to N6 |
| graphics-engineer | Medium F1 (a lost device or WebGPU error did not reach "errors"); Lows F2 to F7 | F1 to F5 closed, F6 partly. Its reading of three 0.187.0: nothing on the WebGPU path needs what the page's policy refuses. New: Lows N1 to N6 |
| rust-reviewer | High F3 (his WB-HEALTH words in a comment had lost a ")"); Medium F1 (the build read the Wasm from `target/` whatever the environment said); Lows | F1, F2, F3, F6, F7 closed; "APPROVE on my ground". New: Lows N1 to N4 |
| security-auditor | Mediums S1 (a broken-off download left its file open), S3 (blocks piled up after a failure), S5 (`--out-dir` could empty any folder); Lows | S1 to S7 closed ("The small server lets no file out of its folder and answers no other site"). New: Lows N1 to N3 |
| perf-auditor | Mediums P1 (fractional timer sleeps made about half the ticks start about 4 ms late), A1 (the allocation test went blind once the code was optimised); Lows L1 to L5 | P1, A1, L1, L5 closed. New: Medium N1 (the allocation test's per-block window straddled the point where the browser optimises the listener), Lows N2, N3 |
| test-engineer | Highs F1 (the memory-growth proof did not cover the host), F2 (the hidden-tab test was flaky and judged the wrong count); Mediums F3 to F7 | F1 to F7 closed. New: Medium NT1 (the failure routes had no test), Lows NT2 to NT8 |
| netcode-architect | nothing above Low; seven Lows for change-set 3's plan | (its ground did not change beyond the timer's rounding, read by perf-auditor and test-engineer) |

**Round 2's three Mediums, all fixed in ⟦FIX-SHA⟧:** the allocation test now samples before the listener is optimised
(blocks 90 to about 360; per block a floor of 430 and a limit of 610; clean 478 to 518 over fourteen runs, the plant
684 to 711); the Worker's error event is cancelled and its missing message replaced, with two browser tests that fail
without the fix; a refusal by the content security policy is now counted as an error, so the readout and every
browser test show one. Also in that pass (Lows that rode along): two more refused build variables and the build-folder
variable no longer refused; `Cross-Origin-Resource-Policy: same-origin` on the small server; three comments made true.

The advisor agent on the final head: ⟦verdict⟧. The verifier: ⟦verdict⟧.

## Decisions and deviations
- **His decisions in this change-set** (whole, by script, in `docs/DECISIONS.md`): 2026-11-18, 20:20:09 UTC (his go on
  #60; WRITE-MODE; REC-OLD; the order of work); 22:34:37 UTC (the five answers); 2026-11-19, 00:18:21 UTC ("A": the
  stop line to 4,400); 00:23:22 UTC (no permission needed until 7 am) ⟦; SIZE-STOP-2⟧. His message of 20:44:29 UTC,
  "the zip saved and unpacked", closes the open point of #58 (STATUS).
- **How "no permission until 7 am" was read:** no stop-and-ask until 06:00 UTC for the work of this change-set. Not
  read as a go for a merge, for an edit to a kit, gate or CI file, or for any other file, version or checksum of code
  from the internet. In that window nothing of the kind was done; the window passed in the stall of point 1.
- **The six departures from the build plan's text** (his DEPART-2 (a)): the hand-off read as a curve over block sizes
  instead of at "12 and 48 soldiers" (no source gives a soldier's row size before change-set 4; that reading is made
  in change-sets 4 and 5); the server prints `http://127.0.0.1:4173`, not `localhost`; the unit vectors in Wasm start
  in change-set 3; the type check's time read with `--ignoreConfig`; two pieces of work the build plan places nowhere
  (the reading of wasm-bindgen's health; how its tool reaches the container); under RNG-CRATE (c) the seeded random
  streams move to change-set 3 and this change-set carries the seed only.
- **No kit, gate or CI file was edited.** `git diff --stat origin/main...HEAD -- .claude .github tools CLAUDE.md` is
  empty ⟦check⟧.
- **Refusals, with the route taken:** a hook refused `gh api -X GET repos/.../actions/runs` (route: the GitHub tool
  for workflow runs); a hook refused `git restore` in a scratch worktree (route: the saved diffs applied in reverse
  with `git apply -R`, then `git worktree remove` without force).
- **Round 2's fixes were prepared in a scratch worktree** while SIZE-STOP-2 was open (no code changed in the
  change-set until his answer), then applied as one diff (`round2-fixes.diff`).
- **The session's model** was read at 20:5x, 22:35 UTC on 2026-11-18 and ⟦time⟧: claude-fable-5-2, configured and last
  served. The advisor tool stayed off.

## Not verified
- **WebGPU.** This container's Chromium has no WebGPU adapter ("No available adapters"): both runs were the WebGL 2
  fallback on SwiftShader. The first WebGPU run is his Brave check. graphics-engineer's prediction for it, not
  measured: the same counts (3 draw calls, 3 triangles, 62 lines, 3 geometries, 3 textures, 6 shader stages).
- **Brave, Firefox, Safari, Windows, a real GPU, a 60 Hz display.** In particular: a really hidden tab (the test
  drives the page's own handler; a headless tab cannot be hidden), Brave's Shields, what a double-click on
  `index.html` does in Brave.
- **CI does not build the Wasm or run the browser tests yet** (his CI-WASM (a): its own pull request after this one).
  Until then those tests ran only in this container.
- A lost GPU device or WebGL context: the route from three.js's error to the page's error count is proven by reading
  the code, not by a test (graphics-engineer N1, a follow-up).
- The e2e specs are run by Playwright but type-checked by no tool.
- The named machines of the pass marks (MACHINES): none of this change-set's readings is on one.

## Follow-ups (each below High; their text is in `docs/STATUS_ARCHIVE.md`, "Follow-ups", "From change-set 2")
- **For the CI-WASM pull request** (right after the merge): the browser assertions that depend on the machine's speed
  (test-engineer's table; in the allocation window 27 to 30 frames against the check's 20 under load here); a test of
  the refused-variables pattern; the builder's registry path inside the Wasm; GitHub's own check of the tool's file.
- **For change-set 3's plan:** netcode-architect's seven Lows (who picks the blend fraction; the hidden-tab rule from
  the mode once online lands; the seed as an authority-only section written last; `step(nowMs)`; the page's per-frame
  number renamed; the Pacer throws on a due time that is not finite; a start tick); engine-architect's (one owner
  for "the match is over"; the endpoint's state into `client-endpoint`; `newest_input` goes; `isWorkerToPage`; the
  Worker's counters in production: decide and record); the random-number library question (his words: read rand's
  upgrade guide for 0.10 and why `chacha20` 0.10.0 and 0.10.1 were withdrawn).
- **For change-sets 5 and 6:** the allocation mark measured on a named machine and another way (the whole page's
  bytes; a listener the browser calls itself); the frame callback's own work time; MSAA memory; a pool of 9 buffers
  when blocks grow; what the page says after a lost device (with ADR 0003's loss handling).
- **Small:** four server branches without a test of their own; the error text's cleaning and the hand-off's time
  stamp have no test; each renderer error prints twice in the console; the npm scope `@shamble` is unclaimed; the
  browser's own request for `/favicon.ico` gets a 404; `dist-*/` is not in `.gitignore`.

## Risks
1. **The first WebGPU run happens on his laptop.** If Brave refuses something or WebGPU fails to start, the page
   falls back and says why in the readout, or shows a line of text; either is a finding for a fix before the merge.
2. **wasm-bindgen has no written release policy** (WB-HEALTH). Held by the exact pin, the tool's checksum and the
   small surface (one type, plain numbers per tick), which keeps a later move off it cheap.
3. **The browser tests are not in CI yet**, so a later change could break them unseen until the CI-WASM pull request
   lands; it is the next pull request.
4. **Speed-dependent assertions** may flake on a slower runner; they passed 30 of 30 here under a load of 5.6.
5. The state hash's function (FNV-1a) is provisional by the plan: PM-0004-21's report confirms or replaces it before
   the first recording is committed (change-set 3).

## Proposed next step
After the merge: the CI-WASM pull request (its exact text to Mustafa first), then change-set 3, "The loop closes".

## Every line the docs pass moved or corrected (his OLD-LINES and REC-OLD: "its report lists each with the old and the new text")
⟦DOCS-LINES⟧

## Decisions needed
⟦DECISIONS⟧

## For Mustafa
⟦FOR-MUSTAFA⟧
