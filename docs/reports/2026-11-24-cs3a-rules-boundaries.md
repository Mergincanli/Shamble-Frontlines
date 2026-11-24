# Advisor report: cs3a-rules-boundaries (2026-11-24, branch `claude/loving-hopper-6p0mg2` standing in for `cs/3a-rules-boundaries`, base main 1ec6f6c)

Change-set 3a, "Rules and boundaries": the first half of change-set 3 (the answer SIZE-3 (b), two change-sets, 3a
then 3b, one of the seven recommended answers Mustafa confirmed on 2026-11-23 by choosing "Yes to all seven").
Built by the builder chat he calls Builder V3 (session_01WLuqnvpAh9XTitx8ehKJcQ) on his
words of 2026-11-24, 11:03:53 UTC: "build change-set 3a by the plan without waiting for me". Risk tier 1 (game code,
tests, build scripts) plus `docs/DECISIONS.md`, a decision doc: it merges only on his go, after his browser check.
No pull request exists yet: by his words of 2026-11-23, 23:44:10 UTC the chat puts the title and text in its notes
folder, his main Advisor opens the draft, and he presses Ready and Merge.

## Goal
Put the simulation's rules under tests before the game loop rests on them, and tidy the edges between the Rust
simulation, its WebAssembly wrapper, the Worker and the page. A player sees almost nothing new: two lines of the
readout changed ("frames" is now a count over time; a value that rounds to zero no longer prints as "-0.00"), and
if the simulation ever stops with an error, its own text now reaches the page instead of "unreachable".

## Outcome
DONE, waiting for his go. Eight planned steps are built; three review rounds are closed; the verifier's PASS and
the advisor agent's APPROVE cover the corrected head 8d277cf2; his browser check is in (errors 0 in Brave and in
Firefox). The last commit that changed what runs is 8619d539 (8619d53968e997846fc1a9174051862030a41a94); 8d277cf2
(8d277cf21cf54919c2ae54e5232d45e944e7b040) after it changed comments in two Rust files, and the commit that holds
this last version of the report changes docs only and removes the notes folder. CI's `gate` and `toolchain` jobs
were green on 8619d539, c5f6a85a and 8d277cf2 with every test count as written down before the push, and the
game's WebAssembly file has the same bytes on all three. One finding changed a rule: the fused multiply-add
(`mul_add`, the operation that computes a * b + c with one rounding) does not give the same bits natively and in
WebAssembly on our pinned Rust toolchain. Mustafa decided on 2026-11-24, 15:58:25 UTC that it stays banned in the
simulation ("MUL-ADD: (a)"). Size: 3,562 added lines against about 2,080 planned (1.71 times); his stop line of
4,100 was not reached, and nothing was cut.

## Plan and approval
- **The plan:** `cs3-plan.md`, draft 3, section 4 (written by the first builder chat, 2026-11-19; the advisor agent
  read draft 1: REVISE with twelve conditions; draft 2: APPROVE with conditions, worked into draft 3). On 2026-11-23
  he chose "Yes to all seven" on the card that confirmed the recommended answers (`docs/DECISIONS.md`; the
  answers' wording there is the register's, not his): RNG-SOURCE (c), our own ChaCha8 checked against the
  published test vectors; SIZE-3 (b), two change-sets, with his yes in advance to the cut ladder; PRESS-PACE (a),
  a second tap inside one tick is applied one tick later (it concerns 3b); DEPART-3 (a), he accepts the plan's
  list of small departures from the drafts.
- **The eight steps:** (1) follow-ups from change-set 2 that need nothing new; (2) a scan of the simulation's source
  for forbidden float methods, hash maps and clocks, and a planted counter overflow; (3) one batch of manifest
  changes; (4) our own ChaCha8 random generator; (5) float test vectors against an exact reference; (6) a panic
  hook that keeps the simulation's error text, and one guard around every WebAssembly call; (7) a test-only
  WebAssembly module that runs these tests in Node and in the browser; (8) the build refuses a crate without
  overflow checks and a feature set that differs between builds, and writes no builder path into the WebAssembly.
- **The advisor agent before the first edit** (2026-11-24, on main 1ec6f6c): APPROVE (build) with nine conditions,
  all met: the browser spec uses only the declared Node names, and the five type checks ran here first (C1); the
  release tests of the tools crate and the wrapper crate ran here on the final code head (C2: `cargo test --release
  -p devtools -p sim-wasm --locked` at 16:26 UTC on 8619d539's code, cargo 1.99.1 (5f94df478), rustc 1.99.1
  (58cf8f9c3), no refused environment name set; f32 vectors 3, stream vectors 2, sim-wasm 9; CI runs these two
  crates in debug only until the next CI edit); expected counts written before every evidence push, each log read
  line by line, a red never re-run (C3: one red, below); the refusal of cargo's nightly-only names joined step 8
  (C4); the lockfile entry moved in the same batch as its manifest (C5); the test-only module is built behind the
  build's existing checks, never by a production build (C6); the size counted as change-set 2 counted it (C7);
  notes pushed at least every 30 minutes, the final head carries no skip marker (C8); the plan's stale lines were
  corrected in the notes, not in the approved plan (C9).
- **The dependency hold** (step 3): security-auditor answered on the exact four files before they were committed
  or built: "The dependency change may be committed and built: yes with conditions". No outside package entered.
  The players' WebAssembly built with only those four files changed was byte for byte the one without them.

## What changed
56 files of code and tests, 21 of them new; 63 with the docs pass. No file under `.github/`, `.claude/`,
`docs/adr/` or `docs/research/`, and no gate file.
- **`crates/sim`:** `src/rng.rs` (new: `ChaCha8`, our own generator: `new`, `next_u32`, `next_u64`, `position`,
  `set_position`, `stream`; the eight published vectors inside its tests); `tests/rules_scan.rs` (new: the scan);
  `tests/state_hash_alloc.rs` (new: the state hash allocates nothing after its first call, counted by an
  allocator); `src/lib.rs` (the planted counter), `src/world.rs` (one integer call written in its path form).
- **`crates/sim-wasm`:** `src/panic.rs` (new: the hook writes the panic's place and message into a fixed
  1,024-byte buffer; `panic_buffer` tells the host where it lies).
- **`crates/devtools`:** `src/rules/f32_ops.rs`, `src/rules/stream.rs`, `src/wasm.rs` (new: the two vector checks
  and the planted counter as exports of the test-only module); `tests/f32_vectors.rs`, `tests/stream_vectors.rs`;
  `vectors/`: two generator scripts (Python's standard library only) and their data (`chacha8-words.bin` 160,384
  bytes, sha256 8531eb01...68f2; `f32-inputs.bin` 120,576 bytes, sha256 f4599394...3396; `f32-expected.txt`;
  `chacha8.manifest.txt`). Both scripts regenerate their files byte for byte (`--check`).
- **`packages/sim-worker`:** `src/panic-message.ts` (new: reads the hook's buffer, builds the one sentence the page
  shows); `src/host.ts` (one guard around every WebAssembly call; a trapped instance is never called again);
  `src/messages.ts` (one function checks every message from the Worker); `boundary/rules.test.mjs` (new).
- **`packages/client`:** `scripts/build.mjs`, `scripts/build-checks.mjs` (the two refusals, the path remap, the
  test-only module built into `.gate/rules-module/`, outside every package); `src/app/worker-link.ts`, `boot.ts`
  (one owner of "the match is over"); `harness/frame-times.ts`, `overlay/report.ts` (the two readout lines);
  `vite.config.mjs` (the dev server reads only the folders the page loads from); `serve.mjs` (the icon request).
- **`packages/e2e`:** `tests/rules/wasm-rules.spec.ts` (new: the rule tests in the browser's own WebAssembly
  engine), changes to the skeleton specs. **Manifests (step 3's one batch):** `crates/devtools/Cargo.toml`,
  `packages/sim-worker/package.json` and the two lockfiles. **Root:** `.gitignore` (two lines).
- **Legacy gate:** symbols +0 −0 (the five legacy files are untouched).

## Evidence
- **CI on the code head 8619d539, read line by line** (`gate` run 45435601291: success; `toolchain` run
  45435601223, job 138592422504, 16:18:37 to 16:21:42 UTC, Chrome 157.0.8103.78): Rust tests in debug, twice:
  client-endpoint 15, f32 vectors 3, stream vectors 2, protocol 2, room 1, sim 15, the scan 5, the allocation count
  1, sim-wasm 9; in release: protocol 2, sim 15, 5 and 1; unit tests: client 70, sim-worker 42, types 7, net 1, the
  browser package's Node project 1; the boundary tests on the built WebAssembly: 2 files, 17 tests; "Running 14
  tests", "14 passed"; nothing skipped or ignored. The baseline on main was 12 browser tests and 7 boundary tests.
- **The same WebAssembly everywhere:** the players' file is 31,718 bytes with sha256 383f5303...3963 in this
  container and on GitHub's runner, and holds no builder path (before the hook: 28,987 bytes here, 28,991 on the
  runner, whose file then still held its own cargo path).
- **One red run, not re-run:** on a1b2e802 the `toolchain` job failed at one assertion I had added last ("not 200"
  for the test-only module's old folder). On a clean machine no stale copy lies there, and Vite's dev server
  answers 200 with the page itself for a file that is not there; here a stale copy made it 403, so the local run
  could not show it. Fixed forward in 8619d539: the test now asks what comes back (no WebAssembly) and the
  control asks for the file's type too.
- **Planted defects:** 96 defects, each planted in the real source, run against the check that should catch it,
  and taken back: 96 of 96 caught in one run on a1b2e802's tree (`plants.py`; the two dev-server ones again on
  8619d539). Beside them the scan's own test holds 43 planted breaches and 12 controls.
- **Our own ChaCha8:** the keystream equals the eight published vectors for 8 rounds and a 256-bit key (TC1 to TC8
  of the Internet-Draft draft-strombergson-chacha-test-vectors-01; its text as fetched: 76,754 bytes, sha256
  e0a49fdd...4d55); 10,000 words at each of four edge stream ids and across the block counter's carry equal a
  second implementation's file, natively, in Node and in the browser.
- **The float vectors:** 38 operations on 10,048 fixed inputs against results worked out in exact rational
  arithmetic and rounded once. Natively every result is exact, in debug and release. In WebAssembly (Node, the
  container's Chromium 153, the runner's Chrome 157; release and dev profiles) every result is exact except
  `mul_add` on 16 inputs (next section).
- **The error text end to end:** with a panic planted in the players' WebAssembly at tick 90 (on step 6's tree,
  the code of 0624557; never committed), the page read "The match stopped: the simulation stopped: panicked at
  crates/sim-wasm/src/lib.rs:140:9:" and "planted: tick 90 is one too many"; the Worker stopped and was not called
  again. The advisor agent repeated it with a panic of its own on the final code: its text reached the page.
- **Cost** (perf-auditor, measured): no regression per block, per frame or per Worker wake; the page's check of a
  block allocates 280.0 bytes per block in its harness, as on main, and nothing once optimised.

## The fused multiply-add (the finding that changed a rule)
- **What was measured** (Rust 1.99.1): for inputs built so that the exact result and the result of two roundings
  differ, the native build on a processor with the FMA instruction gives the exact result on all of them. The
  WebAssembly build gives the exact result where the result is a normal float, and the two-roundings value where
  it lies below the smallest normal float. Example: `0x01000001, 0x32FFFFFE, 0x007FFFFF` gives `0x007FFFFF`
  natively and `0x00800000` in WebAssembly. rust-reviewer measured 22,048 cases of its own and found the same for
  the whole class; test-engineer confirmed with a C check against glibc.
- **Why it is the toolchain's defect:** Rust's page for `f32::mul_add` says "The result of this operation is
  guaranteed to be the rounded infinite-precision result" (doc.rust-lang.org/1.99.1, fetched 2026-11-24). The
  software routine the toolchain uses where no instruction exists corrects a tie only for normal results.
- **What 3a built** (the advisor agent's interim ruling, then his decision): the scan refuses `mul_add` in the
  simulation through a one-name list of its own (ADR 0004 part 7's list of 14 stays word for word in the scan and
  in the draft); 48 constructed inputs joined the 10,000 and none was dropped; natively all must be exact; in
  WebAssembly the 16 are pinned at their measured values, so any change of the toolchain fails the test by case.
- **His decision** (2026-11-24, 15:58:25 UTC, in `docs/DECISIONS.md`): "MUL-ADD: (a). The fused multiply-add stays
  banned in the simulation; it writes a * b + c." And: "No word of ADR 0004 changes in 3a; part 7 gets its dated
  pointer at its next revision, with its exact sentence shown to me first."
- **Not run anywhere:** a processor without FMA. rust-reviewer read the toolchain's code and ran it under a debugger
  with the processor's FMA flags cleared: the native result was then bit for bit WebAssembly's, and the three
  native float tests failed. So two servers, one with and one without the instruction, would disagree.

## Reviews (Opus 5.5, effort max; one severity scale)
- **Round 1, on the eight steps (5c92311): 0 Critical, 2 High, 7 Medium.** test-engineer: the `mul_add` difference
  (High; above) and the feature compare missing its own named case (High; rebuilt on real cargo output, with
  dev-dependency edges). rust-reviewer: four Medium (a cast test that tested nothing new; a wrong sentence in the
  hook's doc; the tests' feature list; holes in the scan), all fixed. security-auditor: one Medium (a crate from
  outside the repository counted as ours; fixed) and the dev server (below). engine-architect: one Medium (the
  same feature compare). perf-auditor: OK, two Low, fixed.
- **Round 2, on the fix pass (79666ad9): 1 High, 2 Medium.** security-auditor: the dev server still reached the
  test-only module through npm's link in `node_modules` (High: my evidence did not hold). Fixed: the module is
  built outside every package (`.gate/rules-module/`), the dev server's list names the one library instead of
  `node_modules`, and the browser test asserts the real place and the link. rust-reviewer: my range fix made calls
  on a tuple's field invisible to the scan (Medium, my regression; fixed, with planted cases). test-engineer:
  APPROVE; the scan missed per-processor code imported in a nested form (Medium; fixed). engine-architect: no
  finding of its own above Low. perf-auditor: OK.
- **Round 3, security-auditor alone, on the dev-server fix (8619d539): APPROVE; the High is closed.** On its own
  copy it sent 99 request spellings to a fresh dev server each, 88 again to a warmed one, and read every answer's
  body: none held the test-only module's bytes or its glue. Its condition for step 3's manifests (no server of the
  repository hands out the test-only module) is met for the dev server, the preview server and the small server.
  One new Low (wording): the comments that list what a cargo config file can still set leave out source
  replacement and `build.rustc` (follow-up 3A-CONFIG-FILE).
- **The verifier (on 8619d539): PASS, nothing above Low.** It read CI's log line by line (every count matches),
  ran the whole sequence on a clean copy of its own (same counts, same WebAssembly bytes), ran 40 of the 96
  planted defects itself (40 caught; 38 for the planted reason, 2 at their named test for a neighbouring reason),
  measured `mul_add` on all 10,048 inputs natively and in WebAssembly (only cases 10024 to 10039 differ, each at
  its pinned value), and checked the draft's and the Rust page's text and checksums, the size, and that part 7's
  list of 14 is the scan's word for word. Its four Lows: one stale sentence in the notes (corrected); the plant
  script leaves a plant's build on disk, so one later plant shows an extra failure beside its own (follow-up
  3A-PLANTS); two exports were built under other names than the design notes planned (next section); the
  runner's image version changed between runs with no read of the allocation numbers recorded (its own read: 61
  bytes a frame and 529 a block, inside the test's windows).
- **The advisor agent on c5f6a85a (the first head with this report): REVISE, for wording only.** By its own gate run,
  its own build (the same WebAssembly bytes), its own ChaCha8 and exact fractions it found the code fit and proven;
  its seven conditions on `mul_add` are met, and its nine start conditions too, but for the eighth, which closes only
  with the last commit (no skip marker; the pull request's text final before the notes go). Its High: four places put
  the register's wording in quotation marks as Mustafa's own (this report twice, a comment in `rng.rs`, the pull
  request's text); his recorded choice was "Yes to all seven". Corrected in the commit after c5f6a85a, with its
  Mediums (a comment in the scan that still said "until Mustafa answers"; the open security follow-ups named for his
  go; the docs pass's section below) and Lows.
- **The advisor agent on the corrected head 8d277cf2: APPROVE.** Every point of its read of c5f6a85a is closed; its
  own build gave the same WebAssembly bytes again, and its own script found every quotation given as his in his
  saved messages. It names 8d277cf2 the corrected head, not the merge head: his go is for the full SHA of the
  commit that holds this last version of the report, after its read of that commit (docs only; the notes removed).
- **The verifier on 8d277cf2: PASS.** The code change since 8619d539 is 15 comment lines in two Rust files; its own
  run and CI agree; all 12 quotations given as his in this report stand word for word in his saved messages. Its
  three wording points and the advisor agent's two are corrected in this last version.

## Decisions and deviations
- **MUL-ADD (a)**, his (above). The scan therefore checks "part 7's list exactly, less `mul_add`", which the
  advisor agent ruled is within its build-plan condition as long as part 7's own list stays word for word.
- **Departures from the drafts beyond the plan's accepted list, as pointers for their next revision, never as
  rewording now:** the feature compare reads PM-0004-74 (the pass mark "One feature set builds the sim on every
  target") as three builds, with dev-dependency edges for the tests; ADR 0002 part 8 names `wasm-bindgen-test` for
  Rust tests in WebAssembly, and 3a runs them through its own test-only module instead; PM-0004-27's "each stream" is
  read as four edge stream ids (no stream exists yet); the feature compare would refuse the dev-only feature ADR 0002
  part 5 plans, loudly, the day it arrives.
- **Pass marks, scoped honestly:** PM-0004-22 (the float pass mark) is met in part: `sqrt` bit for bit and equal to
  a correctly rounded reference, natively, in Node and in Chromium; part 7's other operations too, not `mul_add`;
  not yet: its `libm` clause, the three min/max/clamp helpers, Firefox, WebKit, Safari, 20 runs on one commit.
  PM-0004-23 (the scan): met, and the scan also refuses `mul_add`. PM-0002-53 (the overflow plant): four of four.
- **Names that differ from the design notes** (the verifier's finding; nothing removed, the legacy symbols are
  +0 −0): the test-only module's exports are `chacha8_vectors(file)` and `f32_vectors(inputs, expected)` where
  the notes planned `chacha8_words(...)` and `f32_vectors(inputs)` (the checks moved into the WebAssembly: each
  takes its whole file and answers with what differs), and 8 of the 48 new public names stand in no plan note:
  the hook buffer's and the vector file's layout constants (`BUFFER_BYTES`, `HEADER_BYTES`, `CARRY_WORDS`,
  `FILE_BYTES`, `RESTORE_AT`, `STREAMS`), the row type `Op`, and `chacha8_vectors`.
- **The scan is interim:** it stands in for the lints of the quality gate step (step 2.3) and goes when they merge.
- **Size:** 1.71 times the plan. More than half of the added lines are tests (1,704), and 430 are the two reference
  scripts; the fix passes added 516 lines, most of them tests and planted cases. Nothing moved down the ladder.

## The docs pass (in this change-set)
- **His words, by script** (`docs/DECISIONS.md`; each message whole, copied from the chat's transcript with its
  byte count and sha256 and the question it answers): 2026-11-23, 23:44:10 UTC (his go for #64; how later pull
  requests are opened); 23:51:57 UTC (a short message the chat could not read either way and asked about);
  2026-11-24, 10:16:15 UTC (#64 merged by his main Advisor on his tap); 10:50:43 UTC (his compact of the chat);
  11:03:53 UTC ("build change-set 3a by the plan without waiting for me"); 15:58:25 UTC (MUL-ADD (a), his Advisor's
  two conditions, the stop line, plain words with him, and his four decisions of 24 November, which "change
  nothing in 3a": "No separate animation editor. Rigging and animation happen in Blender, with the Blender MCP.";
  "Testlines stays for feel and metrics. The real level editor goes in the main game (Editor V2)."; "The level
  editor can start against the offline core, but 3b stays first in line for the main builder."; "VFX repo stays
  separate for now; we merge later. Both sides should agree on one effect file format, so the merge is easy.").
- **#64's merge** (main 1ec6f6c, 2026-11-24, 10:15:31 UTC; #63 and #62 closed): STATUS, the archive and ROADMAP.
- **Five dated corrections and one dated addition** (the pointer to commit 5911b56) **to #64's report**
  (`docs/reports/2026-11-19-ci-wasm.md`), asked by the advisor agent's read of #64's final head; no word of his
  changed.
- **His browser check of 3a** (next section): `docs/reports/2026-11-24-cs3a-browser-check.md`, his message whole
  and by script, with a pointer in `docs/DECISIONS.md`.

## His browser check (2026-11-24, 21:48:49 UTC; the zip built from c5f6a85a3)
The record, with his message whole: `docs/reports/2026-11-24-cs3a-browser-check.md`. In short:
- **Brave** (Shields up): two reports, WebGPU, errors 0, the commit c5f6a85a3. The "frames" line reads 180.0 a
  second and the frames over the ticked time come to 180.0 and 180.1. With the window covered on purpose, in his
  words, "it dropped to 179 fps briefly"; no gap opened between frames and ticks. After the tab switch: "ticks
  while hidden: 0", no drop. The page opened as a file shows its one line that says to run `node serve.mjs`.
- **Firefox:** WebGPU, errors 0; the "frames" line reads 178.4 a second against 179.9 over the ticked time (at
  change-set 2's check it read 239.2 against 179.9: the line is fixed).
- **One line is not what the zip's README said to expect:** "last pause" shows 16.5 s hidden, not about 60 s. The
  report counts six pauses and holds only the last one's length; by the two reports' clock times the match stood
  still for about 140 s between them, which fits a covered minute and a minute in another tab. So his Brave
  reports a fully covered window as hidden too, and the match pauses for it. Nothing ran while hidden.
- **STATUS, the archive, ROADMAP, the map:** 3a's line; 23 follow-ups by id (3A-…) with their text; the follow-ups
  of #61 and #64 that 3a closed, each with its evidence; every new path (the map check passes).

## Steps and commits
The notes of every step (the resume card, the reviewers', the verifier's and the advisor agent's notes, his saved
messages, the planted-defect script, the pull request's title and text) stay readable at commit
6491b839dc7243caa796be807b67c5ae5abb1990, the last one before this report's last version removed them
(`git show 6491b839:.checkpoints/cs3a-rules-boundaries.md`).
| What | Commit |
|---|---|
| Step 1, the follow-ups from change-set 2 | a1c59e8 |
| Step 2, the scan and the planted counter | 50656d6 |
| Step 3, the manifests (with its release record) | 0412814, f5a9f67 |
| Steps 4 and 5, our own ChaCha8 and the float vectors | ec27f85 |
| Step 6, the panic hook and the Worker's guard | 0624557 |
| Step 7, the test-only WebAssembly module | df72a58 |
| Step 8, the build's refusals and the path remap | 5c92311 |
| Round 1's fix pass | 79666ad9 |
| Round 2's fix pass, and the corrected test after the red run | a1b2e802, 8619d539 |
| The docs pass | c5f6a85a |
| The advisor agent's wording corrections (comments in two Rust files, docs) | 8d277cf2 |
| His browser check recorded, the checkpoint folded and removed (docs only) | the commit that holds this version |

## Not verified
A processor without FMA (emulated only). Firefox, WebKit and Safari for the vectors (Firefox is his browser check's
step 6 for the page; the vectors run in CI's Chrome only). The dev server and the builds on Windows. The toolchain's
`fmaf` source was read by rust-reviewer, not by the builder. A single hidden stretch of a known length in his
browser (a report holds only the last pause). CI on the commit that holds this version (a report cannot hold the
result of its own commit): the chat reads it line by line before his go is asked.

## Risks
1. A library that enters the simulation later may call the fused multiply-add itself; the scan sees only our code
   (follow-up 3A-FMA-CHECK: by his Advisor's condition it is built in the change-set that brings the first outside
   crate into the simulation, which is planned as change-set 4).
2. The scan reads text, not types: it fails closed on some valid code (our own ordered types' `max`) and misses a
   macro that builds a call. It is interim until the lints.
3. CI does not yet check that the vector files match their generators (a CI edit; with the small CI pull request).

## Follow-ups
In `docs/STATUS.md`, "Follow-ups", ids 3A-…, with their text in the archive. The two his Advisor set: the check that
the players' WebAssembly holds no fused multiply-add is built in the change-set that brings the first outside crate
into the simulation (planned: change-set 4), and that plan names it; reporting the defect to the Rust project, with
nothing posted anywhere without his go.

## Proposed next step
The small CI pull request: every crate's release tests (the plan's E1), Firefox in CI (his FIREFOX-TESTS (b)), the
two generators' checks and a build step of its own for the test-only module. Then change-set 3b, "The loop closes",
after his Advisor's three items.

## Decisions needed
- **CI-MODEL-2: which model writes the next CI edit?** His rule of 2026-11-12: "The guard, the gate and CI work
  stays with Opus 5.5." On 2026-11-23 he set it aside once, in these words: "For this chat's CI edits of the CI-WASM
  pull request, each asked of me with its exact text". That pull request (#64) is merged, so for the small CI pull
  request after 3a the rule of 2026-11-12 stands again. Why it matters: this chat runs on Fable 5.2, and a CI file
  is risk tier 2. Options: (a) an Opus 5.5 chat writes that pull request's CI text, as his rule says; (b) he sets
  the rule aside again for this chat, each text asked of him first, as for #64. Recommendation: (a), his rule as
  it stands. Nothing is needed for 3a's merge.

## For Mustafa
1. Done: the browser check (above).
2. Your go for the merge, for the full SHA the advisor agent approves (the commit that holds this version), once
   your main Advisor has opened the draft from `pr-title.txt` and `pr-body.md` (in the notes folder at commit
   6491b839). With that go you also accept that three small
   security follow-ups stay open: the copy report's rows are not yet built from one fixed list (3A-STATS-ROWS); a
   cargo settings file outside the repository can still change a build profile, the linker, a compiler wrapper
   or where a crate's source comes from (3A-CONFIG-FILE, for the quality gate step); one comment in the root
   manifest is out of date, and a later licence-and-advisory check must allow the tools crate to use
   wasm-bindgen too (3A-MANIFEST-COMMENT, for 3b's manifest batch).
