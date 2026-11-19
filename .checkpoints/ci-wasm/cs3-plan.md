# Plan (draft 1): change-set 3, "The loop closes", proposed as two change-sets, 3a and 3b

_Written by the main session, 2026-11-19, 20:35 UTC (from the clock), beside the CI-WASM pull request (#62), as his
answer CI-WASM (a) allows ("change-set 3's plan may be written beside it"). Nothing of it is built before #62 is
merged, before the advisor agent's APPROVE of this plan, and before his answers to section 12. It moves into
change-set 3's own checkpoint when that opens. Drafted from three notes in this folder, each ending DONE:
`netcode-architect-cs3.md` (the messages, the room, the endpoint, the step, the replay runner, the link simulator),
`engine-architect-cs3.md` (the boundaries, the page, the rule tests, the build order) and `rust-reviewer-cs3-rng.md`
(the random-number question). Those notes hold the wire formats, every symbol and every file:line; this plan holds
what is decided, in which order, and what he is asked. Every line count below is an estimate (no code is written;
the two architects give plus or minus 30 %)._

## 0. In one screen
- **The card** (`docs/reports/2026-11-17-build-plan-slice-1.md`, lines 249 and 316-333) plans one change-set of 2,700
  lines: the real messages over the loopback, the first recording, the tests that settle the simulation's rules,
  and a page that is alive ("press keys and a small key diagram lights up, echoed back by the local server through
  the real messages; counters show ticks, messages and zero lost presses").
- **The designs come to about 5,700 lines**, more than twice the card: the netcode half about 3,090 (48 % of it
  tests), the boundary and rule-test half about 2,800, with about 150 counted twice. Change-set 2 ran 1.54 times its
  plan for the same reason: the proof is the bulk. His rule stops a change-set at twice its plan, so this plan does
  not start one that would begin there. **It proposes two change-sets (question SIZE-3):**
  - **3a, "Rules and boundaries"** (about 2,050 lines; stop line 4,100): the follow-ups that tidy the boundary, the
    scan test, the float and random-stream vectors natively and in WebAssembly, the panic hook, the test-only
    WebAssembly module. Almost nothing new to see: the readout's frame-rate line is corrected.
  - **3b, "The loop closes"** (about 3,700 lines; stop line 7,400; the first parts move out at 4,800): the codec, the
    loopback, the room, the endpoint, the Worker's step in the draft's order, the page alive, the replay runner with
    the first recording, the link simulator L1. His browser check of the living page.
- **No new outside code** with the recommended answers: our own ChaCha8 (RNG-CRATE (c)) and a test-only build of our
  own tools crate with the bindings tool he already approved (WASM-TEST-ROUTE (a)).
- **Questions for Mustafa (section 12):** RNG-CRATE, SIZE-3, WASM-TEST-ROUTE, CI-LOOP, PRESS-PACE.
- **Not in either:** the two-instance test (Part 15 gives that comparison to lane 2, after change-set 3 merges,
  never merged); soldiers, movement, content (change-set 4); Firefox in CI (its own question, FIREFOX-TESTS, with #62).

## 1. Goal
After 3a: the simulation's rules are pinned by tests that run natively and in the shipped kind of WebAssembly, before
any game code rests on them (the card: "Tests that settle rules before code rests on them"). After 3b: the Worker's
client endpoint and the local server exchange `Join`, `JoinAccepted`, `Input` and `Snapshot` through the real codec
over the loopback, every tick runs on the newest post, a press is never lost or doubled, the first recording replays
to the same per-tick hashes natively twice and in WebAssembly, and the link simulator sits behind the same interface
(ADR 0001's and ADR 0004's milestones of step 2.5; PM-0002-12, -49, -53, -64 as far as one slot and no soldier can
show them).

## 2. What Mustafa can see after each
- **3a:** the same empty lit ground. The readout's "frames a second" is a count over time (in his Firefox it said
  239.2 where the run's average was 179.9), and a value that rounds to zero no longer prints as "-0.00". A short
  browser check (about 6 minutes) carries the BR-GAP steps he asked for.
- **3b:** the page, alive. He presses W, A, S, D, Space or Shift and a small key diagram lights each key up from the
  server's echo; the readout counts ticks, messages sent, received and refused, and presses made, echoed and lost
  (lost stays 0). The full browser check, in Brave and once in Firefox.

## 3. Before any edit
1. Pull request #62 merged, and main taken in.
2. The advisor agent's APPROVE of this plan; test-engineer and security-auditor read it before it goes to him (the
   card names both as reviewers).
3. His answers to section 12. RNG-CRATE and WASM-TEST-ROUTE decide what the manifests hold; SIZE-3 decides the shape.
4. The dependency rule as it stands: one batch of manifest changes, uncommitted and unbuilt until security-auditor
   has answered on exactly that content; every cargo command with `--locked`; `npm ci --ignore-scripts` only; the
   ready report quotes each release record and the security answer it names.
5. PM-0004-21's short report (which state-hash function, its cost per tick) exists before the first recording is
   committed (3b): a recording pins the hash.

## 4. Change-set 3a, "Rules and boundaries"
Lane 1 throughout. Symbols and files in full: `engine-architect-cs3.md`, blocks 2 and 4.
| Step | What | Main files | Lines (about) |
|---|---|---|---|
| 1 | Follow-ups that need nothing new (the archive's block of #61): the Worker-to-page messages parsed by one function (EA-F8); test-only members out of the shipped TypeScript (EA-F10); one owner of "the match is over" with the Worker's error reaching the link (EA-N3); the build names the Worker's chunk by its source module instead of a name prefix (EA-N6, VF-L7); the policy header on every answer, the small server's four untested branches, `/favicon.ico` answered (VF-L4, TE-L, FAVICON); the isolation spec restores its two variables (SA-ENV); the error text's cleaning and the hand-off time tested (TE-NT3 to TE-NT5); the readout's frame rate as a count over time (FPS-LINE) and no "-0.00" (NEG-ZERO); `.gitignore`: `__pycache__/` (his words of 14:28:25 UTC) | `packages/sim-worker/src/messages.ts`, `packages/client/src/app/worker-link.ts`, `boot.ts`, `scripts/build.mjs`, `build-checks.mjs`, `serve.mjs`, their tests | 410 |
| 2 | The scan test: fails on a float method outside ADR 0004 part 7's list, a hash map without part 9's escape, a clock call, with planted defects and controls; PM-0002-53's native half (a planted counter overflow panics in debug and release) | `crates/sim/tests/rules_scan.rs`, `crates/sim/src/` | 300 |
| 3 | One batch of manifest changes (the hold): `devtools` becomes a library that can also be built as WebAssembly and depends on our own crates; path dependencies between our crates; no new outside package | `crates/*/Cargo.toml`, `packages/sim-worker/package.json` | 15 |
| 4 | Our own ChaCha8 (RNG-CRATE (c)): the generator with the published test vectors inside its tests, the 10,000-word vectors of four edge stream ids, their generator script | `crates/sim/src/rng.rs`, `crates/devtools/tests/stream_vectors.rs`, `make_chacha8_vectors.py`, `chacha8-words.bin` (160,384 bytes, data) | 430 |
| 5 | The float vectors: about 37 operations of the allow-list and `sqrt`, 120,000 bytes of inputs, expected results as digests, an exact reference generator | `crates/devtools/src/rules/f32_ops.rs`, `tests/f32_vectors.rs`, `make_f32_vectors.py`, `f32-inputs.bin` (data) | 340 |
| 6 | The panic hook that keeps a trap's message (a fixed 1,024-byte buffer the host reads once), the Worker's one guard around every WebAssembly call, the page's notice with the message | `crates/sim-wasm/src/panic.rs`, `packages/sim-worker/src/host.ts`, `panic-message.ts` | 235 |
| 7 | The test-only WebAssembly module (WASM-TEST-ROUTE (a)) and `build.mjs --devtools-wasm`; the vectors, the planted overflow in debug and release, and the hook's text run in Node and in Chromium | `crates/devtools/src/wasm.rs`, `packages/sim-worker/boundary/rules.test.mjs`, `packages/e2e/tests/rules/wasm-rules.spec.ts` | 230 |
| 8 | SA-TREE (the build refuses a runtime crate with no overflow override; test-engineer's draft) and RR-F5 (the builder's path out of the WebAssembly, by rust-reviewer's mechanism; no root `.cargo/config.toml`) | `packages/client/scripts/build-checks.mjs`, `build.mjs`, the root `Cargo.toml` | 95 |
**3a's planned size: about 2,050 lines, stop line 4,100.** If it grows, in this order: the hygiene of step 1 that 3a's
own changes do not need (TE-L, VF-L4, FAVICON, SA-ENV, TE-NT5: about 85) becomes a follow-up; step 8 moves to the CI
pull request after 3a; the vectors' Chromium run moves out (their Node run stays). Never cut: the scan test, the
vectors natively and in Node, PM-0002-53, the panic hook.
- **The interim scan test** stands in for the 2.3 chat's lints under the advisor agent's three conditions (the build
  plan, lines 949-953): it checks exactly ADR 0004 part 7's allow-list; it runs in today's CI test job (it is a plain
  `cargo test`, so no CI edit); the report calls it interim and names the ruling it replaces.
- **"The first 10,000 outputs of each random stream"** (the card): no stream exists yet and nothing draws one in
  change-set 3. The plan reads it as the generator at four edge stream ids; each purpose's stream joins with its
  first draw (weapon spread first, change-set 8). A stated reading.
- **Evidence:** every new test natively in debug and release and, for the vectors, the overflow plant and the hook,
  in WebAssembly under Node and in Chromium; the published ChaCha8 vectors (the IETF draft
  "draft-strombergson-chacha-test-vectors-01": 8 rounds, 256-bit key, 64-bit IV) cited and checked by the
  verifier; each new check shown failing on a planted defect; the Wasm's size with and without the hook; the legacy
  gate unchanged.
- **Reviewers:** rust-reviewer, engine-architect, test-engineer; then security-auditor (the manifests, the new build
  mode, the small server). The verifier, the advisor agent on every pushed head, docs-keeper.
- **His browser check (about 6 minutes, Brave and once Firefox):** the zip's README asks him to keep the browser's
  window in front and uncovered during the first run, and has one step with the window covered on purpose, so that
  BR-GAP is read and not guessed (his words of 16:08:04 UTC); then "Copy report".

## 5. Change-set 3b, "The loop closes"
Lane 1 throughout. Symbols, wire formats and tests in full: `netcode-architect-cs3.md`, blocks 2 and 3;
`engine-architect-cs3.md`, block 3.
| Step | What | Main files | Lines (about) |
|---|---|---|---|
| 1 | `protocol`: a bounds-checked reader and writers; wrap-aware compares (`checked_sub`, never a plain minus: NA-N answers); the codec of `Join`, `JoinAccepted`, `JoinRejected`, `Input`, `Snapshot` with decoders that return errors and never panic; the transport interface and the loopback (fixed buffers, nothing allocated per tick) | `crates/protocol/src/wire.rs`, `wrap.rs`, `messages.rs`, `codec.rs`, `transport.rs`, `loopback.rs` | 785 |
| 2 | `sim`: the match setup with a start tick (NA-N6), the applied command of the one player, the state as tick, player record, then the seed last, so the shared state is a prefix (NA-N4) | `crates/sim/src/world.rs` | 100 |
| 3 | `room`: one slot, the checks and refusals, the command queue; `client-endpoint`: the join exchange, the command built from the newest post with its press counters, the snapshot decoder, the endpoint's state moved out of `sim-wasm` (EA-F9) | `crates/room/src/`, `crates/client-endpoint/src/endpoint.rs`, `press.rs`, `post.rs` | 855 |
| 4 | The Worker's one step in the draft's order (the command for tick T reaches the local server before it runs T); the step takes the host's time (NA-N1); what crosses the bindings stays plain numbers: `input(post, held, four press words)`, `step(now_ms)`; `newest_input` and `now_ms` at creation leave (EA-F10, EA-F13); the block's two new sections (the echo, the link's counters) | `crates/sim-wasm/src/local_match.rs`, `lib.rs`, `crates/client-endpoint/src/layout.rs`, `block.rs` | 400 |
| 5 | The Worker and the page's messages: the newest post kept by post number, an older one ignored and counted (NA-N5, PM-0002-12); held keys cleared on a hidden tab (NA-N2); a due time that is not finite throws (NA-N7); each tick's start stamped for PM-0002-13's first reading | `packages/sim-worker/src/messages.ts`, `host.ts`, `pacing.ts` | 190 |
| 6 | The page, alive: the keyboard by physical key, running press totals, held keys cleared on blur; the key diagram lit from the server's echo; the readout's new rows; the browser test of the loop | `packages/client/src/input/keyboard.ts`, `overlay/keys.ts`, `app/press-tally.ts`, `overlay/report.ts`, `packages/e2e/tests/skeleton/loop.spec.ts` | 530 |
| 7 | `devtools`: the replay runner (one state hash per tick), the comparer (the first differing tick), the first recording (1,800 ticks, 57,650 bytes, data) replayed natively twice and in WebAssembly through the shipped replay entry | `crates/devtools/src/replay.rs`, `compare.rs`, `crates/sim-wasm/src/replay.rs`, `crates/devtools/recordings/first-loop.rec` | 275 |
| 8 | The link simulator L1, a second backend of the same interface (seeded: delay, jitter, loss, reordering, duplication, a rate limit, a packet-fate log), with its own random draws kept apart from the simulation's | `crates/devtools/src/link.rs`, `tests/loop_over_l1.rs`, `tests/l1_profiles.rs` | 595 |
**3b's planned size: about 3,700 lines, stop line 7,400.** Because that line is far away, the plan names an earlier
one: at 4,800 lines the first parts move out without asking, in this order: L1's rate limit and duplication (about
90, to before the gate's network step); the recording's Chromium run; then L1 as a whole (about 600) and, last, the
first recording (about 300) become a change-set 3c before change-set 4, and he is told. Never cut: the codec with
its decoder tests, the wrap-aware compares, the loopback, the room's checks, the press counters with the echo, the
join exchange, the step's order, NA-N4 and NA-N6.
- **Evidence (netcode: deterministic tests with injected latency, jitter and loss; netcode-architect's T1 to T10):**
  the exchange through the Worker's own step with L1 at each profile, 1,800 ticks each, 0 decode errors and 0
  refusals (ADR 0001's milestone, headless); the same seed gives the same packet-fate log and hashes, and over
  1,000,000 datagrams per profile loss, delay and jitter stay within 5 % (PM-0002-49; exactly 0 at the clean
  profile; delay and jitter read from the drawn delay); L1 with every impairment off equals the plain loopback's
  hashes; 10,000 presses under loss, bursts and 1 % duplication arrive exactly, 0 lost and 0 doubled (PM-0002-12);
  counters wrap cleanly; the first recording natively in three processes and in WebAssembly under Node (ADR 0004's
  milestone); every tick runs the newest post; each decoder takes 100,000 random and 100,000 mutated inputs without
  a panic. In the browser: a 1-minute local match in Chromium (PM-0002-64's Chromium half), 0 Wasm calls and 0
  network messages by the page. Each new check shown failing on a planted defect.
- **Reviewers:** netcode-architect, engine-architect, rust-reviewer; then test-engineer, security-auditor (the
  decoders, the page's key handling), perf-auditor (nothing allocated per tick or per frame on the hot path).
- **His browser check (about 15 minutes, Brave and once Firefox):** the keys light up, lost presses stay 0, a hidden
  tab pauses the match and clears held keys; "Copy report" pasted back. The zip's README keeps the Firefox steps and
  the BR-GAP steps.

## 6. Kit, gate and CI edits (each asked of him with its exact text; none is written before his yes)
- **No kit or gate edit.** `tools/gate.mjs` is the legacy game's and stays untouched.
- **Two CI lines follow 3a** (question CI-LOOP), in `.github/workflows/toolchain.yml` as #62 leaves it:
  - E1: the release test line also runs our tools crate (and the room, the endpoint and the bindings crate once 3b
    lands): `cargo test --release -p sim -p protocol --locked` gains `-p devtools`.
  - E2: one line inside the cut before the boundary step: `node packages/client/scripts/build.mjs --devtools-wasm`.
  Their exact text is put to him when 3a's lines exist (the flags are tried here first, as #62's were).
- The scan test and every new native test already run in today's `cargo test --workspace`; L1's tests too.

## 7. Risks
- **Size.** Both halves are estimates; change-set 2 ran 1.54 times its plan. The split, the two stop lines and the
  named cuts are the answer; SIZE-3 asks him.
- **Our own ChaCha8** (if RNG-CRATE (c)): a mistake of ours would be the same on the server and in every browser
  (never a split), and the published vectors catch it at the first test. Its speed against a library's fast code is
  not measured; the tick's cost is measured when soldiers exist.
- **Not yet tried, each a first step of the build, with rust-reviewer:** how the compiler turns `floor`, `ceil`,
  `trunc`, `round` and `mul_add` into code natively and in WebAssembly (the float vectors exist to catch a
  difference); the tools crate linked with the bindings crate in one test module; six numbers per call with no
  allocation in the glue; Vite taking the Worker's chunk name from the build; the path remap for RR-F5.
- **The input message is larger than the draft's mark** (about 277 bytes at the cap against ADR 0001's 200 per
  datagram): no cost on the loopback; the draft's compression of repeated inputs must land before online play. A
  departure to record.
- **Three constants are code, not data, until the content crate exists** (change-set 4): the redundancy cap, the
  catch-up cap, the lead limit. A departure to record.

## 8. Change type and risk tier
Game and engine code, tests, the page: risk tier 1 for both change-sets, his go on each merge. The two CI lines are
risk tier 2 wherever they land (CI-LOOP).

## 9. Where the build plan and the decision drafts are out of date for this change-set
Twenty-eight entries with file and line stand in the two architects' notes (block 6 and block 5). The ones that
change what is built or asked:
- The card's size (2,700) against the designs (about 5,700): SIZE-3.
- The two-instance test: the card (lines 327-331) puts it here; Part 15 (lines 78-80 and 105, newer, his LANE-SPLIT
  (a)) gives it to lane 2 as a comparison that is never merged. This plan follows Part 15.
- "The table of build options" (card, line 330) is defined nowhere; one of its likely columns needs a new tool
  (`wasm-opt`), which would be his decision. Not built here; the Wasm's size is read with and without the hook.
- The card does not list what the echo and "zero lost presses" need: the applied command in the simulation, an
  own-state section in `Snapshot`, a page post that carries keys and press counts; and the step 2.5 table omits
  PM-0002-12 and PM-0002-13's first reading, which the build plan's Part 3 places here.
- ADR 0004 part 8 names `rand_chacha`; any answer to RNG-CRATE changes that line (a decision draft: his go, by a
  dated pointer at the drafts' next revision). Two pass marks name `wasm-bindgen-test` or "a test in sim"; with
  WASM-TEST-ROUTE (a) their Wasm halves run through our own test module.
- "CI files are the 2.3 chat's" and every "2.3 after 3" in the step 2.5 table: by Part 15 (line 174) lane 1 owns
  the workflow files while the 2.3 chat is stopped.
- A fifth message, `JoinRejected`, beside the four the drafts name.

## 10. Not in it
The two-instance comparison (lane 2); soldiers, movement, content, the camera (change-sets 4 and 5); the picture's
lost-device handling and the reason a fallback runs (EA-N5, GE-N1, GE-N4 to GE-N6, PA-L2: before `render/` changes
hands, change-set 5); online play; Firefox and WebKit in CI; any new outside package.

## 11. Records due
RNG-CRATE's answer and the dated pointer for ADR 0004 part 8; the departures of section 7; the readings (the Wasm's
size, PM-0002-13's first reading, L1's run time); what the browser checks find (BR-GAP first); the follow-ups moved
to change-set 5.

## 12. Open questions for Mustafa
1. **RNG-CRATE: where does the game's random-number code come from: an outside library (which one), or a small piece
   of our own?** Why it matters: every random outcome (bot choices, weapon spread, upgrade offers) must come out
   identical on the server and in every browser for years; a mismatch means the server keeps correcting players.
   - (a) `rand_chacha` 0.10.0, the library the engine draft names. Four outside packages, one of which runs a program
     while the game builds. Its makers say it is "likely to be discontinued"; it runs different fast code on the
     server than in the browser.
   - (b) `chacha20` 0.10.2, the library the rand project itself moved to. Two outside packages (three on the
     server). The server picks one of three fast code paths by its processor; one of them had undefined behaviour in
     the two releases withdrawn in August 2026 (no source reports wrong numbers).
   - (c) Our own ChaCha8 in the simulation: about 150 lines plus tests, checked against published test numbers.
     Nothing from outside enters. On the server it may run slower than a library's fast code (not measured).
   - What his two readings say: the rand project's upgrade guide calls the new library's generators "a direct
     replacement for the like-named `rand_chacha` types; these maintain reproducibility of output"; the two releases
     were withdrawn on 2026-08-27 because one fast path used a processor instruction it had not checked for
     ("The yanked versions contain UB in SSE2 backend, see #580"), not for wrong output. One difference between the two
     libraries stands in no document: the order in which the stream and the position must be set.
   - Recommendation: (c). It is the only option where the server and every browser run the very same code, and no
     code from the internet enters.
2. **SIZE-3: how is change-set 3 built, now that its designs come to about 5,700 lines against the card's 2,700?**
   Why it matters: his rule stops a change-set at twice its plan, and this one would begin there.
   - (a) One change-set, planned at about 5,700 lines.
   - (b) Two: 3a, the rules and boundaries (about 2,050 lines, a 6-minute browser check), then 3b, the loop (about
     3,700 lines, the full browser check), each with its own stop line; if 3b reaches 4,800, the link simulator and
     the first recording become a third.
   - (c) Three from the start: the rules, the loop, then the link simulator with the first recording.
   - Recommendation: (b). The rules settle before the loop rests on them, which is the card's own order; each part
     can be reviewed whole; (c) adds a third review round that is only needed if 3b grows.
3. **WASM-TEST-ROUTE: how do the rule tests run inside WebAssembly?** Why it matters: it decides whether new outside
   code enters, and how literally two pass marks are met.
   - (a) A test-only build of our own tools crate, made with the bindings tool he already approved, run in the test
     steps that exist. No new outside code; the two marks' wording changes at the drafts' next revision.
   - (b) The tool the drafts name, `wasm-bindgen-test`: new outside packages, and CI must unpack a second program
     from the archive he approved.
   - Recommendation: (a).
4. **CI-LOOP: where do the two CI lines of section 6 go?** Why it matters: a CI edit makes a pull request the higher
   risk tier, and he asked for the CI text apart from the rest in #62.
   - (a) A small CI pull request right after 3a, as CI-WASM followed change-set 2.
   - (b) Inside 3a's pull request, with their exact text approved first.
   - Recommendation: (a), as he chose for CI-WASM.
5. **PRESS-PACE: a key tapped twice within one game tick (a thirtieth of a second): what happens to the second tap?**
   Why it matters: it decides how fast tapping feels, and the draft's anti-cheat rule allows one press per key per
   tick.
   - (a) It is applied one tick (33 ms) later; nothing is ever lost.
   - (b) It is dropped.
   - (c) Several presses count in one tick, which needs the draft's rule changed (turbo macros would get through).
   - Recommendation: (a): people rarely tap one key more than about ten times a second, so it almost never shows.

## 13. Not verified
Nothing of this plan is built or run. Every line count is an estimate. The libraries' own 8-round output was read
from their code, not run. The points of section 7, "Not yet tried". test-engineer, security-auditor and the advisor
agent have not read this draft yet.
