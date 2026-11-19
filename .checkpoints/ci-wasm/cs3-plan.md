# Plan (draft 2): change-set 3, "The loop closes", proposed as two change-sets, 3a and 3b

_Draft 1 written by the main session, 2026-11-19, 20:35 UTC; **draft 2, 21:00 UTC (from the clock)**, after the
advisor agent's review of draft 1 (`advisor-cs3-plan.md`: REVISE, twelve conditions, no Critical; all twelve are
worked in here and listed in section 14). Written beside the CI-WASM pull request (#62), as his answer CI-WASM (a)
allows ("change-set 3's plan may be written beside it"). Nothing of it is built before #62 is merged, before the
advisor agent's APPROVE of this plan, and before his answers to section 12. It moves into change-set 3's own
checkpoint when that opens. Drafted from three notes in this folder, each ending DONE: `netcode-architect-cs3.md`
(the messages, the room, the endpoint, the step, the replay runner, the link simulator), `engine-architect-cs3.md`
(the boundaries, the page, the rule tests, the build order) and `rust-reviewer-cs3-rng.md` (the random-number
question). Those notes hold the wire formats, every symbol and every file:line; this plan holds what is decided, in
which order, and what he is asked. Every line count is an estimate (no code is written; the two architects give plus
or minus 30 %)._

## 0. In one screen
- **The card** (`docs/reports/2026-11-17-build-plan-slice-1.md`, lines 249 and 316-333) plans one change-set of 2,700
  lines: the real messages over the loopback, the first recording, the tests that settle the simulation's rules,
  and a page that is alive ("press keys and a small key diagram lights up, echoed back by the local server through
  the real messages; counters show ticks, messages and zero lost presses").
- **The designs come to about 5,850 lines**, more than twice the card: the netcode half about 3,090 (48 % of it
  tests), the boundary and rule-test half about 2,800, about 150 counted twice, and about 110 more for a pass mark's
  own test that draft 1 lacked. Change-set 2 ran 1.54 times its plan for the same reason: the proof is the bulk. His
  rule stops a change-set at twice its plan, so this plan does not start one that would begin there. **It proposes
  two change-sets (question SIZE-3):**
  - **3a, "Rules and boundaries"** (about 2,050 lines; stop line 4,100): the follow-ups that tidy the boundary, the
    scan test, the float and random-stream vectors natively and in WebAssembly, the panic hook, the test-only
    WebAssembly module. Almost nothing new to see: the readout's frame-rate line is corrected.
  - **3b, "The loop closes"** (about 3,800 lines; stop line 7,600): the codec, the loopback, the room, the endpoint,
    the Worker's step in the draft's order, the page alive, the replay runner with the first recording, the link
    simulator L1. His browser check of the living page.
- **No new outside code** with the recommended answer to RNG-SOURCE (our own ChaCha8); the rule tests run in
  WebAssembly through a test-only build of our own tools crate, made with the bindings tool he already approved.
- **Questions for Mustafa (section 12):** RNG-SOURCE, SIZE-3, PRESS-PACE, DEPART-3. The advisor agent ruled on two
  more (WASM-TEST-ROUTE (a); CI-LOOP (a)); he is told and can object.
- **Not in either:** the two-instance comparison (Part 15 gives it to lane 2, after 3b merges, never merged);
  soldiers, movement, content (change-set 4); Firefox in CI (question FIREFOX-TESTS, with #62).
- **Lane 1's pull requests, in order, one at a time on its one branch:** #62 (CI-WASM); the Firefox CI pull request,
  if he answers FIREFOX-TESTS (c); 3a; one CI line (E1, section 6); 3b; 3c only if a part moves out of 3b
  (section 5); then change-set 4. If he answers FIREFOX-TESTS (b), "with change-set 3", Firefox joins E1's CI pull
  request after 3a.

## 1. Goal
After 3a: the simulation's rules are pinned by tests that run natively and in WebAssembly, before any game code
rests on them (the card: "Tests that settle rules before code rests on them"). After 3b: the Worker's client
endpoint and the local server exchange `Join`, `JoinAccepted`, `Input` and `Snapshot` through the real codec over
the loopback, every tick runs on the newest post, a press or a release is never lost or doubled, the first recording
replays to the same per-tick hashes natively twice and in WebAssembly, and the link simulator sits behind the same
interface (ADR 0001's and ADR 0004's milestones of step 2.5; section 9 places every pass mark).

## 2. What Mustafa can see after each
- **3a:** the same empty lit ground. The readout's "frames a second" is a count over time (in his Firefox it said
  239.2 where the run's average was 179.9), and a value that rounds to zero no longer prints as "-0.00".
- **3b:** the page, alive. He presses W, A, S, D, Space or Shift and a small key diagram lights each key up from the
  server's echo; the readout counts ticks, messages sent, received and refused, and presses made, echoed and lost
  (lost stays 0).

## 3. Before any edit
1. Pull request #62 merged, and main taken in.
2. The advisor agent's APPROVE of this plan (CLAUDE.md step 2 names the advisor; the other reviewers read the built
   change-set, and the dependency hold brings security-auditor in at build time).
3. His answers to section 12. RNG-SOURCE decides what the manifests hold; SIZE-3 the shape; DEPART-3 the readings.
4. The dependency rule as it stands: one batch of manifest changes per change-set, uncommitted and unbuilt until
   security-auditor has answered on exactly that content; the lockfile regenerated by the resolve-only commands
   change-set 2 used (`cargo update --workspace --dry-run`, then `cargo update --workspace`; no build); every other
   cargo command with `--locked`; `npm ci --ignore-scripts` only; the ready report quotes each release record and
   the security answer it names.
5. PM-0004-21's short report (which state-hash function, its cost per tick) is written by the main session with
   rust-reviewer before the first recording is committed (3b, step 7): a recording pins the hash. Today the hash is
   our own code; if that report wanted a hash library, it would be his question first.
6. A checkpoint of its own per part (`cs3a-...`, `cs3b-...`); at most 3 agents at a time; the saver running.

## 4. Change-set 3a, "Rules and boundaries"
Lane 1 throughout (Part 15: `overlay/`, `app/`, `scripts/`, `packages/sim-worker/`, the crates, and the specs under
`packages/e2e/tests/skeleton/` and a new `packages/e2e/tests/rules/`). Symbols and files in full:
`engine-architect-cs3.md`, blocks 2 and 4.
| Step | What | Main files | Lines (about) |
|---|---|---|---|
| 1 | Follow-ups that need nothing new (section 8 lists each id): the Worker-to-page messages parsed by one function; test-only members out of the shipped TypeScript; one owner of "the match is over", with the Worker's error reaching the link; the build names the Worker's chunk by its source module instead of a name prefix; the policy header on every answer, the small server's untested branches, `/favicon.ico` answered; the isolation spec restores its two variables; the error text's cleaning and the hand-off time tested; the readout's frame rate as a count over time and no "-0.00"; `.gitignore`: `__pycache__/` (his words of 14:28:25 UTC) | `packages/sim-worker/src/messages.ts`, `packages/client/src/app/worker-link.ts`, `boot.ts`, `scripts/build.mjs`, `build-checks.mjs`, `serve.mjs`, their tests | 410 |
| 2 | The scan test: fails on a float method outside ADR 0004 part 7's list, a hash map without part 9's escape, a clock call, with planted defects and controls; PM-0002-53's native half (a planted counter overflow panics in debug and release) | `crates/sim/tests/rules_scan.rs`, `crates/sim/src/` | 300 |
| 3 | One batch of manifest changes (the hold): `devtools` can also be built as a WebAssembly module and depends on our own crates; path dependencies between our crates. No new outside package if RNG-SOURCE (c) | `crates/*/Cargo.toml`, `packages/sim-worker/package.json` | 15 |
| 4 | **If RNG-SOURCE (c):** our own ChaCha8 with the published test vectors inside its tests; the 10,000-word vectors at four edge stream ids; their generator script. **If (a) or (b):** the library enters in step 3's batch instead (the hold; four or three more overflow-override lines; with (b) a CI line that runs the vectors on each of its code paths, asked of him with its text), and this step keeps only the vectors and their test: about 190 lines fewer | `crates/sim/src/rng.rs`, `crates/devtools/tests/stream_vectors.rs`, `make_chacha8_vectors.py`, `chacha8-words.bin` (160,384 bytes, data) | 430 |
| 5 | The float vectors: about 37 operations of the allow-list and `sqrt`, 120,000 bytes of inputs, expected results as digests, an exact reference generator | `crates/devtools/src/rules/f32_ops.rs`, `tests/f32_vectors.rs`, `make_f32_vectors.py`, `f32-inputs.bin` (data) | 340 |
| 6 | The panic hook that keeps a trap's message (a fixed 1,024-byte buffer the host reads once), the Worker's one guard around every WebAssembly call, the page's notice with the message | `crates/sim-wasm/src/panic.rs`, `packages/sim-worker/src/host.ts`, `panic-message.ts` | 235 |
| 7 | The test-only WebAssembly module, built by the development build into its ignored folder (so CI, which already runs that build, needs no new line and 3a is green alone); the vectors, the planted overflow in debug and release, the hook's text, and the built Wasm's import list (no clock import: PM-0004-10) run in Node and in Chromium. A test never skips because its module is missing | `crates/devtools/src/wasm.rs`, `packages/client/scripts/build.mjs`, `packages/sim-worker/boundary/rules.test.mjs`, `packages/e2e/tests/rules/wasm-rules.spec.ts` | 230 |
| 8 | The build refuses a runtime crate with no overflow override, and compares the simulation's feature list across its builds (SA-TREE; PM-0004-74); the builder's path out of the WebAssembly (RR-F5, by rust-reviewer's mechanism; no root `.cargo/config.toml`) | `packages/client/scripts/build-checks.mjs`, `build.mjs`, the root `Cargo.toml` | 95 |
**3a's planned size: about 2,050 lines, stop line 4,100.** A cut never takes a test out by itself (his words of
2026-11-19, 13:36:37 UTC: "I do not take tests out to fit a number"): it moves a whole part, with its tests, to a
named place. 3a's ladder, asked in SIZE-3: first the hygiene items of step 1 that 3a's own changes do not need
(TE-L, VF-L4, FAVICON, SA-ENV, TE-NT5, each with its tests: about 85 lines) move to 3b; then step 8 (about 95)
moves to E1's CI pull request. He is told when a part moves.
- **The interim scan test** stands in for the 2.3 chat's lints under the advisor agent's three conditions (the build
  plan, lines 949-953): it checks exactly ADR 0004 part 7's allow-list, read as closed at the four extra methods the
  draft names (DEPART-3); it runs in today's CI test job (a plain `cargo test`: no CI edit); the report calls it
  interim and names the ruling it replaces.
- **"The first 10,000 outputs of each random stream"** (the card): no stream exists yet and nothing draws one in
  change-set 3. The plan reads it as the generator at four edge stream ids; each purpose's stream joins with its
  first draw (weapon spread first, change-set 8). The advisor agent let this reading stand.
- **The two generator scripts and the data files** (the advisor agent's four conditions): Python's standard library
  only, run with `-B`; regenerated byte for byte by the build session and by the verifier; the sha256 of each data
  file in a text manifest beside it and in the report. About 280 KB of committed data in 3a.
- **Evidence:** every new test natively in debug and release and, for the vectors, the overflow plant and the hook,
  in WebAssembly under Node and in Chromium; the published ChaCha8 vectors (the IETF draft
  "draft-strombergson-chacha-test-vectors-01": 8 rounds, 256-bit key, 64-bit IV) cited and checked by the verifier;
  each new check shown failing on a planted defect; the Wasm's size with and without the hook; the legacy gate
  unchanged.
- **Reviewers** (Opus 5.5, effort max): rust-reviewer, engine-architect, test-engineer; then security-auditor (the
  manifests, the build's new part, the small server) and perf-auditor (a guard on every per-tick WebAssembly call;
  the hook's bytes). The verifier until PASS; the advisor agent on every pushed head; docs-keeper.
- **His browser check (about 6 minutes; Brave, then once Firefox).** Each step with what he should see:
  1. Unzip, run `node serve.mjs`, open the printed address in Brave: the grey ground with its grid, "errors: 0".
  2. Keep Brave's window in front and uncovered for 2 minutes: nothing changes; the frame-rate line holds steady.
  3. Cover the window completely with another window for 1 minute, then uncover it (the BR-GAP step, on purpose).
  4. Switch to another tab for 1 minute, then come back: "last pause" shows about 60 s hidden, 0 ticks while hidden.
  5. Press "Copy report" and paste it; say whether Brave Shields was up, and what a double-click on `index.html`
     showed (two things his first check did not show).
  6. In Firefox: open the address, wait 2 minutes, "Copy report", paste: "frames a second" now matches the average.

## 5. Change-set 3b, "The loop closes"
Lane 1 throughout (`input/`, `overlay/`, `app/`, `packages/sim-worker/`, the crates, `packages/e2e/tests/skeleton/`;
`render/` and `picture.ts` unchanged). Symbols, wire formats and tests in full: `netcode-architect-cs3.md`, blocks 2
and 3; `engine-architect-cs3.md`, block 3.
| Step | What | Main files | Lines (about) |
|---|---|---|---|
| 1 | `protocol`: a bounds-checked reader and writers; wrap-aware compares (`checked_sub`, never a plain minus); the codec of `Join`, `JoinAccepted`, `JoinRejected`, `Input`, `Snapshot` with decoders that return errors and never panic; the transport interface and the loopback (fixed buffers, nothing allocated per tick) | `crates/protocol/src/wire.rs`, `wrap.rs`, `messages.rs`, `codec.rs`, `transport.rs`, `loopback.rs` | 785 |
| 2 | `sim`: the match setup with a start tick, the applied command of the one player, the state as tick, player record, then the seed last, so that the shared state is a prefix | `crates/sim/src/world.rs` | 100 |
| 3 | `room`: one slot, the checks and refusals, the command queue; `client-endpoint`: the join exchange, the command built from the newest post with its press counters (by his answer PRESS-PACE), the snapshot decoder, the endpoint's state moved out of `sim-wasm` | `crates/room/src/`, `crates/client-endpoint/src/endpoint.rs`, `press.rs`, `post.rs` | 815 |
| 4 | The Worker's one step in the draft's order (the command for tick T reaches the local server before it runs T); what crosses the bindings (section 5.1); the block's two new sections (the echo, the link's counters); who picks the blend fraction stays one sentence until change-set 4 gives the block its field | `crates/sim-wasm/src/local_match.rs`, `lib.rs`, `crates/client-endpoint/src/layout.rs`, `block.rs` | 400 |
| 5 | The Worker and the page's messages: the newest post kept by post number, an older one ignored and counted; held keys cleared on a hidden tab; a due time that is not finite throws; each tick's start stamped for PM-0002-13's first reading | `packages/sim-worker/src/messages.ts`, `host.ts`, `pacing.ts` | 190 |
| 6 | The page, alive: the keyboard by physical key, running press totals, held keys cleared on blur; the key diagram lit from the server's echo; the readout's new rows; the browser test of the loop (a 1-minute local match); PM-0002-12's own test: a seeded shim on the page's posting side delays each post by 0 to 100 ms and sends 10 % twice, 10,000 scripted presses, some shorter than a tick, 0 lost and 0 doubled presses and releases; the allocation spec covers the new per-frame code | `packages/client/src/input/keyboard.ts`, `overlay/keys.ts`, `app/press-tally.ts`, `overlay/report.ts`, `packages/e2e/tests/skeleton/loop.spec.ts`, `presses.spec.ts`, `allocation.spec.ts` | 640 |
| 7 | `devtools`: the replay runner (one state hash per tick), the comparer (the first differing tick, shown on one planted divergence), the first recording (1,800 ticks, under 58 KB, data) replayed natively in three processes and in WebAssembly, in Node and in Chromium, on the file players get (section 5.1) | `crates/devtools/src/replay.rs`, `compare.rs`, `crates/devtools/recordings/first-loop.rec`, `packages/sim-worker/boundary/sim-wasm.test.mjs` | 275 |
| 8 | The link simulator L1, a second backend of the same interface (seeded: delay, jitter, loss, reordering, duplication, a rate limit, a packet-fate log), with its own random draws kept apart from the simulation's | `crates/devtools/src/link.rs`, `tests/loop_over_l1.rs`, `tests/l1_profiles.rs` | 595 |
**3b's planned size: about 3,800 lines, stop line 7,600.** A cut moves a whole part with its tests (as in 3a). 3b's
ladder, asked in SIZE-3, so that his yes is given in advance and he is told when a part moves: if 3b reaches 4,900
lines, first L1's rate limit and duplication with their tests and the duplication run of the presses-under-loss
test (about 90; a departure from the card, which lists both); then L1 as a whole with its tests (about 600); last
the first recording with its tests (about 300). Each moves to a change-set 3c, built right after 3b and before
change-set 4 (whose own milestone needs L1's profiles). Never moved: the codec with its decoder tests, the
wrap-aware compares, the loopback, the room's checks, the press counters with the echo, the join exchange, the
step's order, the shared prefix and the start tick.

### 5.1 Where the two designs differ, and which wins (the main session's choice; the reviewers confirm at step 4)
- **What `input` takes:** six plain numbers (the post number, the held keys as bits, four packed press words), as
  the boundary design has it. The netcode design also passed a movement vector; nothing reads one before change-set
  4, so the command's movement and aim fields, their clamp and its tests enter there (about 40 lines leave 3b).
- **Creation:** `new(seed, mode, pad)` without the host's time; the first `step(now_ms)` sets the anchor, so the
  anchor has one writer.
- **Where the replay runs in WebAssembly:** on the exact file players get, through the calls it already has
  (`input`, `step`) and one new read-only number, the state's hash. No call that takes a block of bytes is added to
  the shipped file (the netcode design had one, which goes beyond his WB-HEALTH (a) words, "one type, plain numbers
  on every tick"), and the replay is not moved into the test-only module (the boundary design had that, which
  leaves ADR 0002's line 509, determinism on the exact shipped file, unmet). The one new number is asked in DEPART-3.
- **Evidence (netcode: deterministic tests with injected latency, jitter and loss; netcode-architect's T1 to T10):**
  the exchange through the Worker's own step with L1 at each profile, 1,800 ticks each, 0 decode errors and 0
  refusals (ADR 0001's milestone, headless); the same seed gives the same packet-fate log and hashes, and over
  1,000,000 datagrams per profile loss, delay and jitter stay within 5 % (PM-0002-49, read as DEPART-3 says); L1 with
  every impairment off equals the plain loopback's hashes; 10,000 one-shot presses under 5 % loss, bursts and 1 %
  duplication arrive exactly, 0 lost and 0 doubled (the headless half of PM-0001-38's kind of run); counters wrap
  cleanly; the first recording natively in three processes and in WebAssembly; every tick runs the newest post; each
  decoder takes 100,000 random and 100,000 mutated inputs without a panic. In the browser, in Chromium here:
  PM-0002-12 by its own method (step 6), with its two bites (a Worker that reads only the held bits; a Worker that
  takes the last post to arrive instead of the newest); a 1-minute local match with 0 Wasm calls and 0 network
  messages by the page (PM-0002-64's Chromium half); once, a 10-minute run with 0 errors and 18,000 ticks
  (PM-0002-5's reading, in the report). Each new check shown failing on a planted defect.
- **Reviewers** (Opus 5.5, effort max): netcode-architect, engine-architect, rust-reviewer; then test-engineer,
  security-auditor (the decoders, the page's key handling), perf-auditor (nothing allocated per tick or per frame
  on the hot path). The verifier until PASS; the advisor agent on every pushed head; docs-keeper.
- **His browser check (about 15 minutes; Brave, then once Firefox).** Each step with what he should see:
  1. Unzip, run `node serve.mjs`, open the address in Brave, window in front and uncovered: the grid, a small box of
     six keys (W, A, S, D, Space, Shift) beside the readout, "errors: 0".
  2. Press and hold W: its box lights a moment later (the light comes back from the local server through the real
     messages) and stays lit; let go: it goes dark.
  3. Tap Space quickly ten times: the box flashes each time; "presses lost" stays 0.
  4. Hold A, switch to another tab for 30 seconds, come back without touching the keyboard: A is dark (held keys
     are cleared), "last pause" shows the time, 0 ticks while hidden.
  5. Cover the window with another window for 1 minute, uncover it (the BR-GAP step).
  6. Leave it 2 minutes; "Copy report"; paste it: messages sent and received climb with the ticks, refused 0.
  7. In Firefox: steps 1 to 3 and 6 once; paste the report.

## 6. Kit, gate and CI edits (each asked of him with its exact text; none is written before his yes)
- **No kit or gate edit.** `tools/gate.mjs` is the legacy game's and stays untouched.
- **One CI line, E1, in a small CI pull request right after 3a** (the advisor agent's ruling CI-LOOP (a)), in
  `.github/workflows/toolchain.yml` as #62 leaves it: the release test line
  `cargo test --release -p sim -p protocol --locked` becomes
  `cargo test --release -p sim -p protocol -p room -p client-endpoint -p sim-wasm -p devtools --locked`, written
  once with every crate, so that no CI edit has to follow 3b. Its exact text is put to him then, tried here first.
- No other CI line: the scan test and every new native test already run in today's `cargo test --workspace` (L1's
  too); the test-only module is built by the development build CI already runs; `boundary/` and every spec under
  `packages/e2e/tests/` are already run by #62's lines. If the development-build route fails when it is tried, the
  question comes back to him.

## 7. Risks
- **Size.** Both halves are estimates; change-set 2 ran 1.54 times its plan. The split, the stop lines and the
  ladders are the answer; SIZE-3 asks him.
- **Our own ChaCha8** (if RNG-SOURCE (c)): a mistake of ours would be the same on the server and in every browser
  (never a split), and the published vectors catch it at the first test. Its speed against a library's fast code is
  not measured; the tick's cost is measured when soldiers exist.
- **Not yet tried, each a first step of the build, with rust-reviewer:** how the compiler turns `floor`, `ceil`,
  `trunc`, `round` and `mul_add` into code natively and in WebAssembly (the float vectors exist to catch a
  difference); the tools crate linked with the bindings crate in one test module; six numbers per call with no
  allocation in the glue; Vite taking the Worker's chunk name from the build; the path remap for RR-F5; the
  development build also building the test module.
- **The browser tests grow** by PM-0002-12's 10,000 scripted presses (a minute or more): the run's time on a runner
  is read against #62's 5-minute limit before the limit is touched (a CI edit, his yes).

## 8. The follow-ups, each placed
| Id (from `docs/STATUS_ARCHIVE.md`, the block of #61, and his browser check) | Where |
|---|---|
| EA-F8, EA-F10 (TypeScript part), EA-F13, EA-N3, EA-N6 with VF-L7, VF-L4, TE-L, SA-ENV, FAVICON, TE-NT3, TE-NT4, TE-NT5, FPS-LINE, NEG-ZERO, SA-TREE, RR-F5, the `__pycache__/` line | 3a |
| NA-N1 (the step gets the host's time), NA-N2 (held keys cleared on hide; the pause from the mode waits for online play), NA-N4, NA-N5, NA-N6, NA-N7, EA-F9, EA-F10 (the bindings part), TE-NT7 and PA-N2 (the allocation spec covers the new per-frame code), FF-WAKES (a reading with the pacing) | 3b |
| NA-N3 (who picks the blend fraction): one sentence stays in 3b; the field comes with the block's soldier section | change-set 4 |
| BR-GAP, BR-LATE, NOT-SHOWN (his first check): read again by 3a's browser check, steps 2 to 5 | 3a's check |
| ADAPTER, TQ-FF (what Brave and Firefox do not report about the graphics card): recorded; they matter to the GPU half of the measurement harness | change-sets 5 and 6 (lane 2's harness) |
| EA-N5, GE-N1, GE-N4, GE-N5, GE-N6, PA-L2 (all Low; the picture's loss handling and pool size): before `render/` changes hands. The advisor agent allowed the move; change-set 2's report already listed two of them for change-sets 5 and 6 | change-set 5 |
| VF-L5, RR-N4, TE-NT6, E2E-TYPES, SA-DIST | done in #62 |
| "The table of build options" (the card, line 330; defined nowhere; one likely column needs a new tool) | a follow-up of its own, by the card's rule |

## 9. The pass marks and done-when lines placed at change-set 3, each with its step and test
| Line or mark | Step and test | State after 3b |
|---|---|---|
| ADR 0001's milestone: the four messages over the loopback, L1 behind it | 3b steps 1 to 4 and 8; the exchange test over L1 (headless), `loop.spec.ts` (in the Worker, Chromium) | shown here; the browser half in CI with #62's lines |
| ADR 0003's milestone, second half: every tick runs on the newest post | 3b steps 4 and 5; the newest-post test; `loop.spec.ts` | met |
| ADR 0004's milestone: the first recording's hashes natively twice and in Wasm | 3b step 7 | met natively and in Node and Chromium here |
| ADR 0004's foundation line: the rules as lints, a replay runner, the first recording in CI | the scan test (3a step 2) stands in for the lints (interim); the runner and the recording (3b step 7) | the lints are the 2.3 chat's |
| PM-0002-12: presses and releases never lost or doubled | 3b step 6, `presses.spec.ts`, by the mark's own method | Chromium here; Firefox and WebKit when CI runs them |
| PM-0002-13: a press reaches its tick quickly | 3b steps 5 and 6: a first reading printed by `loop.spec.ts`, not judged | a reading; the mark in change-set 5 on a named machine |
| PM-0002-26 (PM-0004-68 to -70): the Worker's pacing | the readout's pacing numbers in both browser checks (with FF-WAKES) | a reading |
| PM-0002-5: one build runs 10 minutes with 0 errors | 3b: one 10-minute run in Chromium here, in the report | a reading; the browsers and machines later |
| PM-0002-49: L1's fate log and its 5 % bands | 3b step 8 | met headless, read as DEPART-3 says |
| PM-0002-53: a planted counter overflow panics in the four builds | 3a step 2 (natively, debug and release), step 7 (WebAssembly, debug and release, in the test module) | met but for its cost clause, which waits for the tick bench |
| PM-0002-64: a 1-minute local match in three engines | 3b step 6, `loop.spec.ts` | Chromium here; Firefox and WebKit when CI runs them |
| PM-0002-69: "the foundation is done" | the empty scene through the full stack (3b) | shown for the scene; the mark needs the preview link (step 2.6) |
| PM-0004-19 with -17, -18, -24: the same result everywhere, tick by tick | 3b step 7 gives the runner and the first recording; the suite's matches need soldiers and bots | begun; -17's two-instance run, -18's 36,000 ticks and -24's finite-state read come with the determinism suite (change-set 4 on) |
| PM-0004-21: a divergence is caught at its tick; the hash function fixed | the hash report (section 3, item 5); the comparer shown on one planted divergence (3b step 7) | the report and one of the three plants; the rest with the suite |
| PM-0004-22: the maths functions and `sqrt`, bit for bit | 3a step 5 | natively, Node and Chromium here |
| PM-0004-27: the random streams replay exactly | 3a step 4 | natively, Node and Chromium here; ChaCha8's cost per tick when soldiers exist |
| PM-0004-23 with -10 and -74: the rules enforced by the build | 3a step 2 (the scan), step 7 (no clock import in the built Wasm), step 8 (one feature set) | interim, until the 2.3 chat's lints |

## 10. Change type and risk tier
Game and engine code, tests, the page: risk tier 1 for 3a and 3b, his go on each merge after his browser check. E1
is risk tier 2 (a CI file), in its own pull request.

## 11. Not in it
The two-instance comparison (lane 2, after 3b merges); soldiers, movement, content, the camera (change-sets 4 and
5); online play; Firefox and WebKit in CI; any new outside package (unless he answers RNG-SOURCE (a) or (b)).

## 12. Open questions for Mustafa (four), and the advisor agent's rulings on two more
1. **RNG-SOURCE: where does the game's random-number code come from?** His answer RNG-CRATE (c) of 2026-11-18 stands:
   "The library enters in change-set 3 beside its test vectors, and you ask me which one with that plan." This is
   that question, with one option more than "which one": option (c) brings no library at all, so it lies outside
   what he was promised and is his to take or refuse.
   - Why it matters: every random outcome (bot choices, weapon spread, upgrade offers) must come out identical on
     the server and in every browser for years; a mismatch means the server keeps correcting players.
   - (a) `rand_chacha` 0.10.0, the library the engine draft names. Four outside packages, one of which runs a program
     while the game builds. Its makers say it is "likely to be discontinued"; it runs different fast code on the
     server than in the browser. The engine draft's line stays as it is.
   - (b) `chacha20` 0.10.2, the library the rand project itself moved to. Two outside packages (three on the
     server). The server picks one of three fast code paths by its processor; one of them had undefined behaviour in
     the two releases withdrawn in August 2026; its next release adds another path for browsers.
   - (c) Our own ChaCha8 in the simulation: about 150 lines plus tests, checked against published test numbers.
     Nothing from outside enters. We write and review it; on the server it may run slower than a library's fast
     code (not measured). A mistake of ours would be caught by the published numbers at the first test, and would
     be the same mistake everywhere, never a difference between server and browser.
   - What his two readings say. The upgrade guide: the new library's generators are "a direct replacement for the
     like-named `rand_chacha` types; these maintain reproducibility of output". It names no test for that, and says
     nothing about the seed, the stream, the position or the order of the output words. rand's own tests compare the
     two libraries at 12 rounds only; nothing upstream checks the 8 rounds we would use. The withdrawn releases:
     taken back on 2026-08-27 because one fast path used a processor instruction it had not checked for ("The
     yanked versions contain UB in SSE2 backend, see #580"), not for wrong output; the maintainers also wrote that
     the tests of the generator's 64-bit counter are weak (their open issue #582). One difference between the two
     libraries in the order the stream and the position must be set: neither the guide nor either change log says
     so.
   - What (a) or (b) would add to 3a: the dependency hold with security-auditor's answer on the exact manifest,
     four or three overflow-override lines, with (b) one more CI line for his yes, and about 190 lines fewer of our
     own.
   - Recommendation: (c). It is the only option where the server and every browser run the very same code, and no
     code from the internet enters.
2. **SIZE-3: how is change-set 3 built, now that its designs come to about 5,850 lines against the card's 2,700?**
   Why it matters: his rule stops a change-set at twice its plan, and this one would begin there.
   - (a) One change-set, planned at about 5,850 lines, stop line 11,700.
   - (b) Two: 3a, the rules and boundaries (about 2,050 lines, stop line 4,100, a 6-minute browser check), then 3b,
     the loop (about 3,800 lines, stop line 7,600, the 15-minute check). With it, his yes in advance to the two
     ladders: in 3a, the hygiene items move to 3b, then the overflow-override check moves to E1's pull request; in
     3b from 4,900 lines, L1's rate limit and duplication, then L1 as a whole, then the first recording, each with
     its tests, move to a change-set 3c before change-set 4. A part moves whole; no test is dropped; he is told.
   - (c) Three from the start: the rules (about 2,050, stop line 4,100), the loop without L1 and the recording
     (about 2,900, stop line 5,800), then L1 with the first recording (about 900, stop line 1,800).
   - Recommendation: (b). The rules settle before the loop rests on them, which is the card's own order; each part
     can be reviewed whole; (c) adds a third review round that is only needed if 3b grows.
3. **PRESS-PACE: a key tapped twice within one game tick (a thirtieth of a second): what happens to the second tap?**
   Why it matters: it decides how fast tapping feels, and the draft's anti-cheat rule allows one press per key per
   tick. 3b's press counting follows his answer.
   - (a) It is applied one tick (33 ms) later; nothing is ever lost.
   - (b) It is dropped.
   - (c) Several presses count in one tick, which needs the draft's rule changed (turbo macros would get through).
   - Recommendation: (a): people rarely tap one key more than about ten times a second, so it almost never shows.
4. **DEPART-3: does he accept the places where change-set 3 reads or departs from the decision drafts?** Why it
   matters: the drafts are his decision documents; each departure is recorded by a dated pointer at their next
   revision, never by rewording them now. As DEPART-2 was asked for change-set 2. Options: (a) accept all; (b)
   accept with his changes. Recommendation: (a). The list:
   - The network draft, ADR 0001: commands are sent whole, so the input message can reach about 277 bytes where the
     draft marks 200 per datagram (lines 220-224, 728: no cost on the loopback; the compression must land before
     online play); three limits are code, not data, until the content crate exists in change-set 4 (line 221); a
     fifth message, `JoinRejected`, beside the four it names (line 1781).
   - The structure draft, ADR 0002: the bindings crate also holds the small piece that joins the endpoint, the room
     and the transport, and the tools crate becomes a second WebAssembly module for tests only (part 3, line 191);
     the calls that cross the bindings (part 5, lines 330-334): `input` with six numbers, the time passed with the
     step, the panic buffer, **and one new read-only number on the file players get, the state's hash**, so that a
     recording can be replayed on that exact file (line 509) with no call that takes a block of bytes; "the newest
     by sequence number" read as the page's post number (PM-0002-12, PM-0002-64); PM-0002-49's 5 % band read as
     exactly 0 at the clean profile, the burst profile judged by its own check, delay and jitter read from the
     drawn delay; a 1 % duplication profile, which no draft sets; PM-0002-53's "a test in sim" met in WebAssembly by
     a planted counter in the tools crate's test module.
   - The simulation draft, ADR 0004: part 7's list of extra float methods read as closed at the four it names
     (lines 267-268); part 8's `rand_chacha` (line 309), which stays under RNG-SOURCE (a) and changes under (b) or
     (c); PM-0004-22 and PM-0004-27, which name the tool `wasm-bindgen-test`, met through our own test module.

**Ruled by the advisor agent** (`advisor-cs3-plan.md`; he can overrule either):
- **WASM-TEST-ROUTE: (a).** The rule tests run in WebAssembly through a test-only build of our own tools crate, made
  with the bindings tool he approved. No new outside code; it stays inside his TOOL-SOURCE yes. The other way, the
  tool the drafts name (`wasm-bindgen-test`), would bring new outside packages and a second program from the
  archive.
- **CI-LOOP: (a), for E1 only.** One CI line in a small pull request after 3a, its exact text to him first.
Also decided by the advisor agent, no question: the readout's new rows and 3a's short check (his browser check
covers them); the generator scripts and the committed data on four conditions (section 4); the readings of section
4 and the two-instance comparison left to lane 2; the Low follow-ups moved to change-set 5.

## 13. Not verified
Nothing of this plan is built or run. Every line count is an estimate. The libraries' own 8-round output was read
from their code, not run. The points of section 7, "Not yet tried". The choices of section 5.1 are the main
session's between two designs; netcode-architect and engine-architect have not read them yet. The advisor agent has
read draft 1, not this draft.

## 14. The advisor agent's twelve conditions on draft 1, and where each is met
1. A new code for the random-number question, his sentence quoted, (c) named as outside "which one", step 4
   conditional, what (a) or (b) add: section 12, question 1; section 4, steps 3 and 4.
2. What rust-reviewer found in his two readings and draft 1 left out: section 12, question 1.
3. No cut takes a test out; both ladders whole in SIZE-3 (b): sections 4 and 5; section 12, question 2.
4. The presses-under-loss run named as what it is; PM-0002-12 by its own method: section 5, step 6 and "Evidence".
5. 3a green in CI alone (the development build makes the test module); E2 dropped; E1 once with every crate:
   section 4, step 7; section 6.
6. DEPART-3: section 12, question 4.
7. Which design wins on the three symbols: section 5.1.
8. One follow-up table: section 8.
9. One pass-mark table: section 9.
10. "After 3b merges", and lane 1's pull requests in order: section 0.
11. No plan read by test-engineer and security-auditor; perf-auditor on 3a; the verifier and docs-keeper named;
    effort max; a checkpoint per part: sections 3, 4 and 5.
12. Numbered browser checklists; the engine draft's line under (a); the resolve-only command; who writes the hash
    report; Part 15's "while the 2.3 chat is stopped" (it applies to the gate files; the workflows are lane 1's
    outright, so draft 1's line on it is gone): sections 3, 4, 5 and 12.
