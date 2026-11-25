# Advisor report: cs3b-the-loop-closes (2026-11-25, branch `claude/loving-hopper-6p0mg2` standing in for `cs/3b-loop-closes`, base main 28254a73)

Change-set 3b, "The loop closes": the second half of change-set 3. Built by the builder chat Mustafa calls Builder V3
(session_01WLuqnvpAh9XTitx8ehKJcQ) on his words of 2026-11-24, 23:24:50 UTC: "Otherwise build change-set 3b by the
plan under your advisor agent's eight conditions, without waiting for me." Risk tier 1 (game code, tests, a page a
player sees) plus `docs/DECISIONS.md`, a decision doc: it merges only on his go, after his browser check. No pull
request exists yet: the chat puts the title and text in its notes folder, his main Advisor opens the draft, and he
presses Ready and Merge.

**State of this version of the report:** the code is final at 6abf3d2a; both review rounds are closed and the
verifier's PASS is in; the advisor agent read the head 09b0043d (REVISE for the text only, the code approved as it
stands) and its points are worked into this version; its review of these changes and his browser check are not in
yet (this report is updated when they arrive).

## Goal
Close the loop for one player on one machine: a key pressed on the page goes to a local game server inside the page
through the real network messages and comes back, 30 times a second. A player sees a row of six boxes (W, A, S, D,
Space, Shift) that light when the server has applied the key, and two new readout lines ("messages", "presses").
Nothing moves yet: the soldier comes with change-set 4.

## Outcome
PARTIAL by plan, DONE for what stayed in it. Steps 1 to 6 of the plan are built; steps 7 and 8 (the first recording
and the link simulator) moved to a new change-set 3c by the plan's size ladder, which he approved in advance and was
told about before anything moved. Two review rounds by six reviewers are closed with every reviewer at PASS and
nothing Critical or High open. The final code is 6abf3d2a (6abf3d2a3d347ac8b2d5a169cbe5ac0aabc3d9b1, 2026-11-25,
03:08 UTC); GitHub's `gate`, `smoke`
and `toolchain` are green on it with every test count as written down before the push. Size: 6,909 added lines against
about 3,800 planned (1.82 times); his stop line of 7,600 was not reached. One decision is asked of him
(RELEASE-COUNT, below); it does not block his browser check.

## Plan and approval
- **The plan:** `cs3-plan.md`, draft 3, section 5 (advisor agent: APPROVE with conditions, worked into draft 3; his
  "Yes to all seven" of 2026-11-23 covers SIZE-3 (b), two change-sets with the cut ladder, and PRESS-PACE (a), a
  second tap inside one tick is applied one tick later).
- **The advisor agent before the first edit** (2026-11-24, on main 28254a73): APPROVE under eight conditions. How
  each was met: **C1, C2** the decoder test runs ADR 0001's row PM-0001-34 as written (below; the mark has started,
  it has not passed); **C3** the build plan's three sentences are changed in this docs pass and shown below, his
  words quoted only as they stand; ADR 0001 is untouched; **C4** the size was projected before steps 7 and 8, and he
  was told in three lines before the link simulator and the recording moved; **C5** one manifest batch (path
  dependencies between our own crates, no outside package), answered by security-auditor on exactly those six files
  before it was committed or built ("yes", no condition); 3A-HOOK-TEST and 3A-FROM-F64 built as steps; **C6** the
  browser tests were timed on a clean copy (16 tests in 3.4 of CI's 5 minutes) before the first push that held a
  browser spec (eb0d2314). Said plainly against his words ("Time the browser tests before the first push, as you
  said."): three code pushes came before that timing (the manifest batch and steps 1 to 5); none of them changed a
  browser spec;
  **C7** release runs of room, client-endpoint, sim-wasm and devtools are local only until the CI pull request of
  another chat lands; **C8** his messages go into DECISIONS by script.
- **The ladder (his item (c)):** at step 6 the size stood at 1.9 times the plan. The link simulator and the first
  recording, each with its tests, moved to 3c; with the recording, the one new read-only number on the players'
  file (the state's hash) waits for 3c too. Nobody builds the simulator elsewhere.

## What changed
74 files of code and tests under `crates/` and `packages/`, 26 of them new, and the root `Cargo.toml` and
`Cargo.lock` (the answered batch 1ee3e918; no manifest or lockfile changed after it). Nothing under `.github/`,
`.claude/`, `tools/` or `legacy/`.
- **`crates/protocol`**: the first wire form of five messages (Join, JoinAccepted, JoinRejected, Input, Snapshot):
  a reader with checked reads, wrap-aware compares for sequences and press counters, a codec that never panics and
  allocates nothing before a message is whole, a transport interface and a fixed-slot loopback.
- **`crates/sim`**: the match starts at a given tick; the world applies the player's command (held buttons and a
  running press counter per button); the state hash covers the tick, the player's record and the seed; a second hash
  of the shared part only, which leaves nothing of the seed in its buffer.
- **`crates/room`**: one player's slot: the join exchange, the checks on every command (acks, lead, strictly later
  tick, at most one press per button per tick), a queue in sequence order whose window follows the last command that
  ran, at most two commands per tick, the snapshot with the server's echo.
- **`crates/client-endpoint`**: the client's side: the join, each tick's command from the page's newest key message,
  the press pacer (one press per button per command; the rest waits its turn), repeats of unconfirmed commands, the
  snapshots, and the block the page reads (two new sections: the echo and the link's counts; 44 bytes).
- **`crates/sim-wasm`**: local play's one step (endpoint → loopback → room → loopback → endpoint → block), plain
  Rust behind thin bindings (`input` with six plain numbers, `step`).
- **`packages/sim-worker`**: the Worker keeps the newest key message by its number, lets go of every key on a hidden
  tab, stamps each tick's start. **`packages/client`**: the keyboard by physical key, the tally of the server's echo
  (per button), the key diagram (development builds only), the readout's and the report's new lines, and three
  page-side checks of the server's echo (below). **`packages/types`**: the new message types and the block's length.
- **Tests:** `crates/protocol/tests/decoder_inputs.rs`; `crates/room/tests/any_bytes.rs` and
  `crates/client-endpoint/tests/any_bytes.rs`; `crates/sim-wasm/tests/step_alloc.rs` and `panic_hook.rs`; browser
  tests `loop.spec.ts`, `presses.spec.ts`, `allocation.spec.ts`; 24 more cases in the float vectors.

## Evidence
- **Gate:** `node tools/gate.mjs --no-smoke` PASS here; CI's `gate` and `smoke` green on 6abf3d2a.
- **Tests on 6abf3d2a, here and on GitHub with the same counts** (toolchain run 45537884772, 4 min 58 s of 15):
  114 Rust tests in debug, twice; release: protocol 19 and 8, sim 17, 5 and 1; Node: client 85, sim-worker 46,
  types 7, net 1; 18 boundary tests on the built Wasm; the seven type checks; **16 browser tests in 3.4 of 5
  minutes** (Chromium). Formatting and the lint check are clean (the lint check runs here only: CI has no lint step).
- **The decoder mark (PM-0001-34; C1, C2): started, not passed.** Per message type 10 million random and 10 million
  mutated seeded inputs in release (about 5 s for all five on GitHub's runner): 0 panics, no memory above the caps
  (largest single request: Join 1,024 bytes, JoinAccepted 64, the others 0), a planted unchecked index found at
  input 17 and a planted over-allocation at input 0. Not met, so "started" is all it is: the row names cargo-fuzz,
  which is not used, and every message type of the catalogue, of which 5 of 16 have decoders. The
  swappable-transport mark belongs to Slice 3 and is not started.
- **The presses mark (PM-0002-12), in Chromium only: by the mark's method for presses; for releases as far as the
  server's echo can show them, not by the row's method (RELEASE-COUNT below); and "reorders most neighbours" holds
  only on a page that draws fast.** 10,000 scripted presses, most shorter than a tick, under a shim that delays
  every key message by 0 to 100 ms and sends one in ten twice. On GitHub's run of 6abf3d2a: 10,000 presses made
  and echoed, 0 lost and 0 doubled, button by button; 10,000 releases made and worked out from the echo; 502 key
  messages delayed, 46 sent twice (all 46 behind a newer one), 1 first copy overtaken; the Worker ignored exactly
  47; 0 ticks ran older keys than the tick before; 0 ticks held other keys than their message's; 0 refused. Its
  bites all turn it red: held buttons only; the last message to arrive; a late message counted as ignored and taken
  all the same; a late message's keys under the newest number; two buttons' counts swapped. **What is not met** is
  in "Decisions and deviations".
- **The one-minute loop (PM-0002-64, in Chromium):** on GitHub 1,814 ticks in 60.6 s, one command sent and one
  snapshot received per tick, 0 refused, 138 presses and releases made and echoed, the page makes 0 WebAssembly
  calls. **Key to tick (PM-0002-13): a reading, not judged:** median 238 ms, 95th percentile 258 ms on a runner that
  draws about 7 frames a second (the mark is 50 ms at 60 frames a second on a named machine).
- **The plan's one 10-minute run (PM-0002-5's reading; Chromium here, from a script of its own, never a spec CI
  runs):** 603.1 s on the build of 09b0043d (the code of 6abf3d2a), WebGL 2 fallback, about 6.5 frames a second:
  18,089 ticks, 0 errors, 18,182 messages sent and 18,183 received, 0 refused; a key tapped every two seconds,
  269 presses and releases made and echoed, 0 lost, 0 doubled, 0 ticks on older or wrong keys; the Worker dropped
  no tick and never ran more than two in one wake (`ten-minutes.mjs` in the notes; 2026-11-25, 04:04 to 04:14
  UTC). A reading on one machine; the mark's browsers and machines come later.
- **Never a panic on what arrives, and the rules hold under lies:** the room against a client that lies in one
  message in five and whose bytes are changed one time in five (100 matches of 3,000 calls: 81,106 commands run,
  every refusal kind reached): at most two commands per tick, in every tick the sequence moves by exactly the
  commands run plus the ones counted missing, no button pressed more often than once per tick since the join. The
  endpoint against a room that lies (84,744 inputs built): every input valid, the next sequence for the next tick
  first, no counter moved by more than one, every datagram counted exactly once as taken, late or refused.
- **Allocation:** a warm tick of the local match allocates nothing in Rust (18,000 ticks, a counting allocator).
  On the page (sampling heap profile, GitHub's runner): 91 bytes per frame, 590 per block, 569 of them the link's
  own work; perf-auditor: the fix pass made the page cheaper and the Worker's tick is 658 ns (268 ns on main).
- **The players' Wasm:** 53,813 bytes (21,994 gzip, 18,464 brotli), sha256 5ebae154…1262, the same here and on
  GitHub; 31,718 on main. The growth is the codec, the endpoint and the room; ADR 0002 sets no size limit yet.
- **Planted defects:** `plants-3b.py`, 53, each run against the check that should catch it, on the final code:
  53 caught (52 in the full run of 03:16 UTC; the last one met a busy browser port there and was caught when run
  again alone at 03:20 UTC).
- **Float vectors (3A-FROM-F64):** 24 constructed cases of the cast from f64 after the existing ones; 10,072 cases;
  exact natively and in WebAssembly; the 16 pinned lines unchanged.

## Reviews (Opus 5.5, effort max; one severity scale; two rounds)
Round 1 on bbbf09e4, round 2 on the fix pass 0dfca749 and the commits after it. A restart of the container stopped
three round-1 reviews before their answers; their notes held their findings, all of which went into the fix pass, and
those three finished in round 2. **All six: PASS in round 2.**
- **test-engineer:** High (the presses mark's second bite, planted whole, passed every line): **fixed**; the page now
  counts ticks that ran older keys, buttons held again with no press, and ticks whose held keys are not their
  message's. Mediums in round 2 (totals hid two swapped buttons; a comparison that went red on a fast page; second
  copies counted as overtakes): **fixed**. Allocation limit valid only on a slow page: the test now refuses a run
  outside the frame rates it was measured at; the cause was found by perf-auditor.
- **security-auditor:** Medium (the room has no bound on how late a command may run: a client that sends nothing
  for N ticks and then a backlog gets two commands per tick for N ticks): **deferred in writing, not closed**
  (below). Lows: fixed.
- **engine-architect:** Medium (the block writer and the newest key message lived in the Wasm wrapper): **fixed**,
  the client endpoint owns both. Lows: fixed.
- **netcode-architect:** Medium (late snapshots counted as refused): **fixed**. It checked the queue's new window
  against its own model: 893,122 commands run, 726,413 refused, 0 deviations.
- **rust-reviewer:** eight Lows, fixed or answered; the block's bytes over 1 million ticks hash the same natively
  and in the Wasm under Node. **perf-auditor:** no finding against 3b; follow-ups below.
- **The verifier on 6abf3d2a: PASS** (2026-11-25, about 03:35 UTC). In its own copy: the gate; formatting and the
  lint check clean; 114 debug tests and the release runs; the decoder run (7.2 s); the Node tests, the type checks,
  the boundary tests; a development and a production build (no development code in production); the same Wasm
  hash; 16 browser tests in 3.3 minutes (the press test: 10,000 and 10,000, 0 lost, 0 doubled, 45 copies all behind
  a newer one, 45 ignored, 0 older keys, 0 wrong keys); 13 planted defects of its choice and the presses mark's 5
  bites, all caught; CI's counts equal what was written down; 16 numbers of the notes checked, all hold. It
  confirmed the test reviewer's ruling on the presses mark. Four Lows: two test gaps found by its own plants (the
  listener that lets go of keys when the window loses the focus has no test; the room's "allocates nothing" is
  counted for honest ticks only), both named follow-ups; two corrections to the notes, made.
- **The advisor agent on 09b0043d (2026-11-25, about 04:00 UTC): REVISE, the text only; "the code, final at
  6abf3d2a3d347ac8b2d5a169cbe5ac0aabc3d9b1, is approved as it stands".** Its eight conditions: all met, C6 "in
  purpose" (the timing came before the first push with a browser spec, after three code pushes). The size:
  still one reviewable unit. The reviews: within the budget; it read the last fixes itself and no reviewer needs to
  look again before the merge. The late-command bound: the deferral is its to approve, and it approves it with
  3B-LATE-BOUND's written conditions. RELEASE-COUNT: rightly his; it blocks the claim that the mark is met, not the
  merge. Its findings were one High and four Mediums in wording (this report and the pull-request text called
  the presses mark met "by its own method" for releases too; what his go accepts as still open was not listed;
  RELEASE-COUNT's reason was wrong; the option he chose for press pacing was not quoted; OVERFLOW-PANIC-RULE's due
  point was missing) and one Medium that was a missing reading (the plan's 10-minute run); each is corrected or
  done in this version.
- **The advisor agent on the corrected text, 8f835d114dcbf03c80cb7227c9cf189539dc6d09 (2026-11-25, about 04:25
  UTC): APPROVE "for the next step only: the zip may go to him on this code".** Not a merge approval: the last
  commit (his check's result, the notes folder removed) gets its own review. All its findings closed; one new
  Medium (this report named the wrong pull request for CI's lint step) and three Lows, corrected in this version.

## Decisions and deviations
- **The build plan's three sentences (his words of 23:24:50 UTC: "(a). Change it with the other two in 3b's docs
  pass, and show me all three old and new sentences in your report.").** In
  `docs/reports/2026-11-17-build-plan-slice-1.md`:
  1. Old: "The room's command scheduler behind one seam, with both ways of timing commands built (run on arrival;
     buffered, one per tick), because the netcode proof test still chooses between them (Part 6) and the slice's
     mark runs under each." New: "The room's command scheduler behind one seam, with both ways of timing commands
     built (run on arrival; buffered, one per tick), because Slice 3 chooses between them, no longer the netcode
     proof test (Mustafa, 2026-11-24), and the slice's mark runs under each."
  2. Old, a row of the table "What the proof test may change", removed there: "| How commands are timed at the
     server (OQ-0001-6): run on arrival, or buffered one per tick | undecided; an input buffer of 1 to 3 ticks, as
     data | one command scheduler in `room`; one function in the client endpoint that says when the next tick is
     due (change-set 7) | one module |". New, a sentence at the end of the paragraph "Not the proof test's, kept
     open the same way:": "How commands are timed at the server (OQ-0001-6) is Slice 3's pick, no longer the proof
     test's (Mustafa, 2026-11-24); change-set 7 still builds both ways behind one command scheduler in `room`."
     (The row's two other details, the input buffer of 1 to 3 ticks and the client endpoint's function, are no
     longer in that file; the netcode decision record still holds both.)
  3. Old (risk 8): "Each is one module or one data value by design; the largest is how commands are timed." New:
     "Each is one module or one data value by design; how commands are timed is no longer among them: Slice 3
     picks it (Mustafa, 2026-11-24)."
  3b's own queue picks neither way: one slot, a lockstep loopback.
- **The presses mark, what is and is not met.** Met in Chromium: 0 lost and 0 doubled presses over 10,000, button
  by button; both of the mark's bites turn the test red. Not met, and stated plainly: (1) the row asks for Chromium,
  Firefox and WebKit; CI has Chromium only. (2) The row says "the Worker counts the presses and releases it applies
  through the edge counters". There is no release counter: ADR 0001 part 5 defines press counters only, while ADR
  0002 part 5 says the post carries "press and release edge counters". Releases are worked out on the page from the
  presses and the held keys; a doubled one shows as a button held again with no press, and every tick's held keys
  are checked against the message that tick used. "0 lost and 0 doubled releases" holds as far as the echo can show
  it; the mark's method for releases is not built (RELEASE-COUNT below). (3) "Reorders most neighbours" holds only
  on a page that draws fast: on CI's slow page only the second copies arrive out of order (1 first copy overtaken
  in 502).
- **Not built: a bound on how late a command may run (security-auditor's Medium).** Nothing moves in 3b, so the
  doubled rate gains nothing yet; the bound is part of the command-timing choice, which is Slice 3's. Named in the
  code where the check will go. Both reviewers set the condition: no room behind a network transport without it;
  built in change-set 7 with each of the two timing ways and a long-hold test. Put to the advisor agent with this
  head.
- **Two limits against PRESS-PACE (a), each his decision when it comes due.** The option he said yes to (the
  plan's words, chosen with his "Yes to all seven"; not his own words) reads: "It is applied one tick (33 ms)
  later; nothing is ever lost." As built: (1) at most 255 presses of one button wait their turn; one beyond that
  is dropped and counted (only a machine gets there: more than 30 presses a second on one key for over 8 seconds);
  (2) after an outage in which one button is pressed more than 255 times unconfirmed, presses are lost in steps of
  256 (the counters are one byte). No person can reach either in 3b (the second needs a network), so the advisor
  agent asks nothing before this merge; they are 3B-PACER-255 (due once a press fires a weapon) and 3B-OUTAGE-256
  (due before online play).
- **OVERFLOW-PANIC-RULE fell due with this change-set.** With his go for #48 he accepted it as left open, "due
  with the first code in the room crate or a server crate" (`docs/DECISIONS.md`), and 3b is that first code. The
  finding: an overflow in the room's own arithmetic panics in release, and on a server's tick that would end the
  match, repeatably if a client can drive the value. What 3b does about it: `protocol` and `room` (whole crates)
  and the client endpoint's three modules that take values from outside (`endpoint`, `post`, `press`) deny plain
  arithmetic outside tests (the lint `arithmetic_side_effects`), and every sum or difference of a value from
  outside is checked, wrapping or saturating on purpose; the lint check is clean here, in the verifier's copy and
  in the advisor agent's. What stays open: CI runs no lint step. That stage belongs to the gate step (2.3, its
  lint stage G2), whose chat is stopped; the small CI pull request another chat writes does not hold it. Until
  then every change-set that touches `room`, `protocol` or a server crate runs the lint by hand and says so in its
  report. The drafts' wording (ADR 0004 part 20, ADR 0002 parts 4 and 7,
  the marks) stays with their owners at each draft's next edit, with his go. No draft is edited here.
- **Found and fixed on the way:** a refused command far ahead shut out the honest ones behind it (the queue's
  window now follows the last command that ran); the one-minute test started its clock after its first reading and
  went red once by one tick on a slow page; the press test's first script chose three of six keys.

## The docs pass (in this change-set; docs-keeper)
- `docs/DECISIONS.md`: his three messages of 2026-11-24 (22:44:50, 22:59:49 and 23:24:50 UTC), each with what it
  answers, written by script from the chat's transcript and checked against it (`decisions-5.mjs --check`: "is
  what this script writes (3 entries on top of 28254a7's)"). No word of his was typed by hand, and the file holds
  no line of the chat's own (a pointer of the chat's moved to the archive, the advisor agent's Low).
- The build plan's three sentences, as above. `docs/STATUS.md` (120 lines), `docs/STATUS_ARCHIVE.md`,
  `docs/ROADMAP.md`: 3a is merged (#65); 3b's row; 3c next with its contents; RELEASE-COUNT listed as open; 16
  follow-ups of 3b (3B-LATE-BOUND to 3B-OQ6-POINTER), each with who named it and when it is due. To keep STATUS at
  its 120 lines, the follow-up blocks of gate step 2.3 moved to the archive under a dated header. 3a's report got a
  dated note after its "No pull request exists yet". `docs/REPO_MAP.md`: about 20 new rows (the check: "repo map OK:
  193 entries cover 109 required paths"). No ADR was edited.

## Steps and commits
1ee3e918 the manifest batch · bab5a569, 63d2bf7a step 1 (messages, decoders, loopback) · 90da28c1 step 2 (the
world) · 409b9434 step 3 (room, endpoint) · 4eeec33a steps 4 and 5 (the one step; Worker and page messages) ·
64c65184, bbbf09e4 step 6 (the page alive) · f8db0de3 3A-FROM-F64 · 0dfca749 the fix pass of round 1 · 6380df45,
6803671d, a85b9dbb, 6abf3d2a the fixes of round 2.

## Not verified
Firefox, WebKit and Brave in CI (his check covers Brave and Firefox by hand); real hardware and the WebGPU backend
(every headless run falls back to WebGL 2); the key-to-tick mark (a reading only); the decoder mark's full count per
release; release-profile runs of room, client-endpoint, sim-wasm and devtools on GitHub (local only); anything over a
link that delays, drops or reorders (3c).

## Risks
1. The late-command bound stays open until change-set 7; a room must not reach a network transport before it.
2. The size ran at 1.82 times the plan: 3c and change-set 4 should be planned at that ratio.
3. The allocation test's limits are right only for a slow page; a much faster CI runner makes it refuse the run by
   name. Change-set 5 replaces the method.

## Follow-ups (each in STATUS)
3c: the link simulator and the first recording, with what the reviewers named for its plan. Before online play: the
late-command bound; the endpoint's plausibility bound on a snapshot's tick; acks bounded from below; presses over an
outage. Before the marks are judged: keys posted from the key handler too (key to tick); the allocation method
(change-set 5). For his decision at the netcode record's next edit: a long-hold case for its lag-switch mark.
Systems that act on presses apply at most their own rule per tick (change-sets 4, 8, 9). Two test gaps from the
verifier: a browser test for keys let go when the window loses the focus (3B-BLUR-TEST); a counted "no allocation"
for hostile bytes in the room and the endpoint (3B-HOSTILE-ALLOC, with 3c's tests).

## Proposed next step
Change-set 3c, the link simulator and the first recording, planned first and reviewed by the advisor agent.

## Decisions needed
- **RELEASE-COUNT (how releases are counted for the presses mark).** The mark's row says the Worker counts presses
  and releases "through the edge counters"; the two decision records disagree, and what is built carries counters
  for presses only. Why it matters: the report cannot call the release half met by the mark's own method. Options:
  (a) accept as built (releases worked out, plus the two page-side checks) and correct the row's sentence when the
  records are next edited; (b) add a release counter per button to the messages: 16 bytes more per command, so up
  to 128 more per input message (292 to 420) until delta coding, and about 150 to 200 lines, in 3c.
  Recommendation: (a). The releases follow exactly from the held keys and the press counters (so will a charged
  shot that fires when its button is let go), and the per-tick key check sees every wrong held state. It blocks
  the claim that the mark is met, not this merge (the advisor agent's ruling).

## For Mustafa
1. The browser check (Brave, then Firefox once; about 20 minutes) with the zip `shamble-v2-03b-the-loop-closes`,
   whose README has the steps: hold W and see its box light and go dark; press S, D, the left Shift and the right
   Shift, each lighting only its own box; tap Space ten times and read "presses … 0 lost · 0 doubled"; hold D,
   click on another window, let go there and come back (D is dark); hold A across a hidden tab; cover the window
   for a minute; copy the report after two minutes (all of `messages.refused`, `olderPosts`, `wrongKeys`,
   `presses.lost`, `presses.doubled`, `releases.doubled` should be 0).
2. RELEASE-COUNT: (a) or (b).
3. After the advisor agent's APPROVE and your check: your go for the merge of the pull request your main Advisor
   opens from the title and text this chat hands over. **Your go accepts as still open:** the bound on how late a
   command may run (3B-LATE-BOUND: security-auditor's Medium, deferred to change-set 7; no room on a network
   without it); OVERFLOW-PANIC-RULE (kept in the code by hand; no lint step in CI yet; the drafts' wording with
   their owners); and the two press limits (3B-PACER-255, 3B-OUTAGE-256), each your decision when it comes due.
