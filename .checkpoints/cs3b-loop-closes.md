# Checkpoint: cs3b-loop-closes
- **Goal:** change-set 3b, "The loop closes": the second half of the card's change-set 3 (the answer SIZE-3 (b),
  which Mustafa confirmed on 2026-11-23 by choosing "Yes to all seven"). The plan: `cs3b-loop-closes/cs3-plan.md`
  (draft 3), section 5; its symbols, wire formats and tests in `cs3b-loop-closes/netcode-architect-cs3.md` (blocks
  2 and 3) and `cs3b-loop-closes/engine-architect-cs3.md` (block 3). **Not started: before the first edit come
  his three items, the advisor agent's read of them, the message to him in at most 6 lines, and (recommended)
  his compact of this chat.**
- **Who builds:** the chat Mustafa calls Builder V3 (session_01WLuqnvpAh9XTitx8ehKJcQ, a Claude Code chat under the
  repository's guard). The session record, read 2026-11-24, 22:47 UTC: model `claude-fable-5-2` (configured and
  last served), permission mode `auto`, **context 703,899 of 1,000,000 tokens (about 70 %)**. Every agent runs on
  Opus 5.5 (`model: "opus"`); agents that review, verify or design run at effort max; docs-keeper at its own
  file's effort.
- **Branch on GitHub: `claude/loving-hopper-6p0mg2`** (stands in for `cs/3b-loop-closes`). **Worktree:
  `/home/user/Shamble-Frontlines/.gate/ci-wasm-wt`** (the folder keeps its old name), local branch
  `cs/3b-loop-closes` (its upstream reads origin/main: never a plain push). **Every push:**
  `git push origin cs/3b-loop-closes:claude/loving-hopper-6p0mg2`, alone on its command line. The main folder
  `/home/user/Shamble-Frontlines` is never switched (his block: "Do not switch your main folder."). Always
  `cd /home/user/Shamble-Frontlines/.gate/ci-wasm-wt &&` first; git never with `-C` or `-c`. **Base:** main
  28254a73ab226379373e380eeee64336e58ef916 (the merge of #65, change-set 3a).
- **Opened:** 2026-11-24 22:48 UTC (from the clock).

## His words that govern (read them where they stand; these lines are pointers, never a substitute)
- **2026-11-24, 22:44:50 UTC** (`cs3b-loop-closes/mustafa-2026-11-24T224450-merged-65-next-3b.md`, by script: 2,601
  bytes, sha256 f654912d...1820; **not yet in DECISIONS: it goes in with 3b's docs pass, by a script made from
  `decisions-4.mjs`**):
  1. #65 is merged by his main Advisor on his tap of 22:40:00 UTC with 1624f084... as the expected head; main is
     28254a73...; "That tap was my go. I typed no go line in your chat. Record it by script from this message."
  2. The next CI edit: (a), an Opus 5.5 chat writes the small CI pull request. "You write no CI file. If that
     holds up 3b anywhere, tell me in three lines."
  3. Next is 3b. Three items from his Advisor first: (a) the decoder test (PM-0001-34): its bar is its row in ADR
     0001 exactly as written, "10 million random and mutated inputs per message type, 0 panics and 0 allocations
     above the size limits", and its planted bug (an unchecked index in one decoder) found within the run; in 3b
     the test reaches the row's count and both zeros for every message type 3b adds, with seeded inputs and a
     counting allocator of our own; no new tool or package in 3b (the row names cargo-fuzz: his decision later);
     "Until then your report says the mark has started, never that it passed." (b) command timing at the server
     (OQ-0001-6): the proof test no longer picks it; Slice 3 picks it and change-set 7 still builds both ways; his
     go: with 3b's docs pass the build plan's two places that say the proof test chooses say this instead, the old
     and the new sentences shown to him in the report. (c) the link simulator: 3b builds its own as planned (his
     Advisor's lean; the choice is the plan's); "If the ladder moves the link simulator out of 3b, tell me before
     anyone builds it a second time."
  4. "Before 3b's first edit, your advisor agent reads these three against the plan on today's main. Then tell me
     in at most 6 lines what changes in size, whether anything in the plan no longer holds, and whether this chat
     should be compacted before the build starts."
- **2026-11-24, 15:58:25 UTC** (in `docs/DECISIONS.md`): MUL-ADD (a), the fused multiply-add stays refused in the
  simulation; **"From now on use plain words with me: no tag or code without its explanation in the same
  sentence."**; the check on the built file comes with the first outside crate in the simulation; nothing is
  posted to the Rust project without his go.
- **In `docs/DECISIONS.md`, 2026-11-23:** 23:44:10 UTC (GH-NEXT (a) with his change: the chat puts each pull
  request's title and text in the notes folder, his main Advisor opens the draft and sets them, he presses Ready
  and Merge); 21:24:59 UTC (above 40 % of context the chat tells him first; FIREFOX-TESTS (b)); 16:30:46 UTC (his
  block: never push to another lane's or thread's branch; less paper, his words by script, each with the question
  it answers; the chat's own messages are not copied into files); 14:31:06 UTC (the seven confirmed answers:
  PRESS-PACE (a), a second tap inside one tick is applied one tick later; SIZE-3 (b) with the plan's two ladders;
  DEPART-3 (a)); 14:32:42 UTC (LONG-RUNS: a full suite or a long build runs on GitHub from the pushed branch).
- **Older, still binding:** 2026-11-19, 13:36:37 UTC ("I do not take tests out to fit a number"); 2026-11-18,
  22:34:37 UTC (TOOL-SOURCE: every other file, version or checksum comes back to him); 2026-11-14 (the size rule:
  at twice the plan, stop and tell him in three lines); 2026-11-12 ("The guard, the gate and CI work stays with
  Opus 5.5.": it stands, point 2 above).
- **2026-11-24, 22:59:49 UTC** (`cs3b-loop-closes/mustafa-2026-11-24T225949-where-the-decisions-are.md`, by script;
  into DECISIONS with 3b's docs pass): his decisions of 21 and 22 November are recorded on the netcode test's
  branch `claude/netcode-proof-test-plan-hcky5g`, not on main: `.checkpoints/netcode-proof-test/history.md`, lines
  7561 to 7607 (NPT-TRIM of 21 November; his answers to the re-plan of 22 November), and
  `.checkpoints/netcode-proof-test/moved-block.md`, the part "FOR CHANGE-SET 3 ITSELF". "Read them with git show,
  read only. That is not the spikes/ folder. My message with the three items is my word either way." **Read at
  23:00 UTC at that branch's head e8e276f0bf8b201f6a4db3b59315ac6de4c84a04 (fetched by name; nothing is ever pushed
  there):** NPT-TRIM moves the proof test's marks -34 and -35 "to the real protocol and its dependency checks",
  "nothing dropped or loosened"; the moved-marks part for change-set 3 has the same three items as his message of
  22:44:50 UTC and names the build plan's lines: change-set 7 builds both ways of command timing (lines 389 to
  391; line 390 still says the proof test chooses), the build plan's line for that open question (line 909) and
  its Part 6 change on his go; change-set 3's own link simulator (lines 321 to 323; line 322 lists a rate limit
  and duplication, which the proof test's simulator lacks). Those line numbers were checked there on main f5030a3:
  read them again on today's main.

- **2026-11-24, 23:24:50 UTC** (`cs3b-loop-closes/mustafa-2026-11-24T232450-after-the-compact.md`, by script:
  1,043 bytes, sha256 4da9ba90...6f73; he dates it 25 November; into DECISIONS with 3b's docs pass): the compact is
  done; rebuild from the repository; **the build plan's third sentence (its risk 8): (a)**, "Change it with the
  other two in 3b's docs pass, and show me all three old and new sentences in your report."; at most 6 lines on
  where things stand and what is built first; "If the card, the repo and the summary disagree, or something is
  missing, say what and stop. Otherwise build change-set 3b by the plan under your advisor agent's eight
  conditions, without waiting for me."; past 4,900 lines "tell me in three lines before a part moves"; "Time the
  browser tests before the first push, as you said."; "A short block for the agent board may follow from me.
  Install nothing for it on your own."

## Standing rules (from his words and the repository's rules; when in doubt read DECISIONS and CLAUDE.md)
- **Process:** the plan is approved (the advisor agent, 2026-11-19: `cs3b-loop-closes/advisor-cs3-plan.md`). Build
  in the main session; at most 3 agents at a time, each with its notes in this folder, none commits, the shared
  tree read-only for them (builds, plants and servers on their own copies under `.gate/<name>-cs3b/`); the gate
  after every step; reviews within two rounds (a third only for an open Critical or High); the verifier until
  PASS; the advisor agent on every pushed head that changes more than notes; docs-keeper; the report; then his go.
- **Dependencies (the hold):** any manifest or lockfile change stays uncommitted and unbuilt until security-auditor
  has answered on exactly that content; every cargo command with `--locked`; `npm ci --ignore-scripts` only; code
  committed by path only (never `-a`, `add -A`, `add .`). No new tool or package in 3b (his item (a)).
- **Kit, gate and CI files:** this chat writes none (his point 2). If 3b needs a CI line, tell him in three lines.
- **GitHub:** this chat reads (`gh api`, GET only) and pushes; it cannot open, edit, mark ready or merge a pull
  request. The title and text go into this folder (`pr-title.txt`, `pr-body.md`), his main Advisor opens the
  draft, he presses Ready and Merge. For the merge: the advisor agent approves the full SHA; gate, smoke and
  toolchain green on it; nothing pushed after; the chat gives him the SHA. The last commit folds this checkpoint
  into the report and removes it (`git rm -r` of this card and its folder; the notes' last commit is named in the
  report).
- **Saves:** no saver script here: commit `.checkpoints/` by path and push by hand after every step, before and
  after every agent run, at least every 30 minutes; notes-only commits carry the skip marker; the head given to
  him carries none.
- **Commands that are refused, and their routes:** a `git push` chained with anything; git with `-C` or `-c`;
  `git checkout`, `restore`, `rebase`; `npx`, `npm run`, `npm test` (start tools through node); `gh pr`, `gh api`
  writes; `rm -rf`, **and deleting a folder by any other means (refused here on 2026-11-24: do not work around it;
  `git rm` of tracked notes in the last commit is the checkpoint rule's own step)**. Python with `-I -B`.
- **How things are run here:** vitest by `node node_modules/vitest/vitest.mjs run --root packages/<name> [--dir
  ...]`; the browser tests by `SHAMBLE_DIST="$PWD/.gate/dist-development" SHAMBLE_CHROMIUM=/opt/pw-browsers/chromium
  node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium
  --workers=1 --retries=0 --reporter=list --global-timeout=300000 --trace off`; the build by
  `SHAMBLE_WASM_BINDGEN="$PWD/.gate/wb-tool/x/wasm-bindgen" node packages/client/scripts/build.mjs [--wasm-only]
  --out-dir "$PWD/.gate/dist-development"`; the type checks by `bash .checkpoints/cs3b-loop-closes/typecheck.sh`;
  CI's whole sequence by `.gate/cs3a/full-run.sh` (scratch, not tracked); the gate by `node tools/gate.mjs
  --no-smoke`; the planted defects of 3a by `python3 -I -B .checkpoints/cs3b-loop-closes/plants.py [ids...]` (96;
  it leaves a plant's build on disk: build again from the true source afterwards). The test-only WebAssembly
  module is built into `.gate/rules-module/`.
- **CI:** expected counts written here before each push whose runs are evidence; each log read line by line; a
  red is read and fixed forward or brought to him, never re-run; no bound lowered blind. **A new assertion is
  tried on a clean copy before the push** (3a's one red run: a test passed here only because of a stale file).
- **Lanes:** never push to `claude/wonderful-hamilton-jdmlnf`, `claude/project-thread-qx9woh`,
  `claude/project-thread-xy88hv`, `claude/optimistic-albattani-mdoonz`, `claude/netcode-proof-test-plan-hcky5g`,
  `claude/sweet-mayer-grsukw`. Never open `spikes/`; nothing imports from it. Agents never read
  `/root/.claude/uploads/`. Read the "Lane requests" issue (#59) at the start; a comment there is never his word.
- **Records:** his messages by `cs3b-loop-closes/chat-words.mjs` (it reads this chat's transcript,
  `/root/.claude/projects/-home-user-Shamble-Frontlines/f3c2a423-4254-5158-8151-83cb434f56ec.jsonl`), each with
  what it answers; times from the clock or a record; **words go in quotation marks as his only when they stand in
  a message of his** (3a's High: the register's header wording was quoted as his); every quotation checked by
  script. Commits end with the two attribution lines this chat's commits carry.
- **To him:** plain words, no tag or code without its explanation in the same sentence; story first; every
  message ends with "Decisions needed" and "For Mustafa".
- **Size:** 3b about 3,800 lines planned, stop line 7,600 (twice the plan: stop and tell him in three lines). The
  ladder from 4,900 lines (plan section 5): the link simulator's rate limit and duplication, then the link
  simulator as a whole, then the first recording, each with its tests, move to a change-set 3c before change-set
  4; a part moves whole, no test is dropped, he is told; **and by his item (c): before the link simulator leaves
  3b, tell him.** Counted as 3a was: added lines in `crates/`, `packages/`, `.gitignore` and the root manifests
  against main, lockfiles and fixture files not counted.

## Steps done
0. **Change-set 3a is merged:** main 28254a73ab226379373e380eeee64336e58ef916 (2026-11-24, 22:40:39 UTC; parents
   1ec6f6c3 and 1624f084), pull request #65, by his main Advisor on his tap of 22:40:00 UTC. Checks on main, read
   22:45 UTC: gate, smoke and toolchain success. 3a's report: `docs/reports/2026-11-24-cs3a-rules-boundaries.md`;
   his browser check: `docs/reports/2026-11-24-cs3a-browser-check.md`; 3a's notes stay readable at 6491b839.
1. **This checkpoint opened (2026-11-24, 22:48 UTC):** the local branch `cs/3b-loop-closes` from origin/main; the
   remote branch moved from 1624f084 to 28254a73 by fast-forward. Restored from 6491b839, each with the same
   sha256: `cs3-plan.md`, `cs3-context.md`, `engine-architect-cs3.md`, `netcode-architect-cs3.md`,
   `advisor-cs3-plan.md`, `test-engineer-followups.md`, `typecheck.sh`, `plants.py`, `make-zip.py`,
   `decisions-4.mjs`, and `chat-words.mjs` (pointed at this folder). His message of 22:44:50 UTC saved by script.

2. **The advisor agent's read of his three items against the plan on main 28254a73: APPROVE, build 3b by the plan
   (draft 3, section 5) under eight conditions; nothing needs his decision; compact first** (2026-11-24, read
   23:12 UTC; Opus 5.5, effort max; its notes `cs3b-loop-closes/advisor-cs3b-start.md`). It checked both of his
   messages against the transcript byte for byte. What it found, by his items:
   - **(a) The decoder test.** The plan has 100,000 random plus 100,000 mutated inputs per decoder (plan lines 166
     to 167; `netcode-architect-cs3.md` T9, lines 380 to 389). The row (ADR 0001 line 1626) asks for 10 million
     per message type. 3b gives decoders to five types: Join, JoinAccepted, JoinRejected, Input, Snapshot (the
     recording file is not a message type). Its estimate: about +120 lines; 10 million of each kind for 5 types is
     100 million decodes, about 20 to 60 s in release and minutes per debug run. **No CI file is needed:** CI's
     release line already runs `-p protocol` (`toolchain.yml` line 159) and release keeps overflow checks, so the
     full count runs there and the debug runs keep the plan's count. The counting allocator: build on
     `crates/sim/tests/state_hash_alloc.rs` (it counts calls; sizes must be added). Seeds: a test-only SplitMix64
     or xorshift64*, never `sim`'s ChaCha8 or a match seed (`protocol` does not depend on `sim`).
   - **(b) Command timing.** The build plan (`docs/reports/2026-11-17-build-plan-slice-1.md`, unchanged since
     f5030a3) says it in three places: lines 389 to 391 ("…because the netcode proof test still chooses between
     them (Part 6) and the slice's mark runs under each."); Part 6's row at line 909, under "What the proof test
     may change" (lines 904 and 907); and risk 8, lines 1257 to 1258 ("…the largest is how commands are timed.").
     His go names two places: **the third is asked of him** (the message of 23:12 UTC). The new sentences it
     accepts: line 390 "…because Slice 3 chooses between them, no longer the netcode proof test (Mustafa,
     2026-11-24), and the slice's mark runs under each"; the row of line 909 moves under line 919's "Not the proof
     test's, kept open the same way:", reading "Slice 3 picks it, no longer the proof test; change-set 7 still
     builds both ways behind one command scheduler in `room`"; risk 8 ends "how commands are timed is no longer
     among them: Slice 3 picks it". Nothing in 3b depends on the pick (one slot, lockstep loopback).
   - **(c) The link simulator** is step 8, the last (about 595 lines). The ladder from 4,900 lines: its rate limit
     and duplication first (about 90 lines, exactly what the proof test's simulator lacks), then the whole
     simulator (about 600), then the first recording (about 300), each to a change-set 3c.
   - **No longer true in the plan (corrected here, not in the approved plan):** the CI pull request placed before
     3b and written by this chat (plan lines 40 to 45, 190 to 198, 211 to 213; "#62" there is #64, merged; the
     small CI pull request is now an Opus 5.5 chat's, beside 3b); the old decoder count; section 5 has no
     manifest-batch step though 3b adds path dependencies; 3A-HOOK-TEST and 3A-FROM-F64 are code, not docs.
     The swappable-transport mark (PM-0001-35, ADR 0001 line 1627) is Slice 3's whole: nothing of it touches 3b's
     transport interface or loopback, and the report must not count it as started.
   - **Size:** about +150 lines, so about 3,950 planned; the stop line stays 7,600 and the ladder's trigger 4,900.
     Steps 1 to 6 come to about 3,080 planned lines; at change-set 2's ratio (1.54) or 3a's (1.71) that is 4,740
     to 5,270 before steps 7 and 8, so the ladder will likely move the link simulator and the recording to 3c.
   - **CI's limits and today's times (its read of main's run 45503161333):** the job 2 min 07 s of 15 minutes; the
     browser tests "14 passed (54.8s)" of 5 minutes.
   **Its conditions:**
   - **C1 (High):** item (a) exactly as the row. For each of the 5 types one release run of 10 million random plus
     10 million mutated seeded inputs (if that proves too slow: 10 million in all with the split named, and back
     to it). The run shows 0 panics, 0 allocations above the codec's caps, and the planted index found, with its
     input number. A control (the counter sees an allocation of known size) and one planted over-allocation, both
     caught in debug too (release may optimise an unused allocation away). The report says the mark has started,
     never that it passed; the recording keeps the plan's count, outside the mark.
   - **C2 (Medium):** the test in its own integration-test binary in `protocol`; the full count only when debug
     assertions are off; the time measured here and the expected counts written down before the push.
   - **C3 (Medium):** the three old/new pairs in the report, his words quoted only as they stand; ADR 0001 lines
     252, 1156 and 1699 to 1700 untouched (a follow-up for a dated pointer); the comment at `messages.rs` lines 5
     to 6 updated for command timing only; the report says 3b's queue picks neither way.
   - **C4 (Medium):** before steps 7 and 8, project the final size (lines so far plus the step's plan times the
     ratio so far). Past 4,900 the parts not yet built move in the ladder's order until the projection is under
     4,900. He is told first, and about the simulator before anyone builds it (his item (c)).
   - **C5 (Medium):** first one manifest batch with every new path dependency plus 3A-MANIFEST-COMMENT, answered
     by security-auditor before any commit or build. Then 3A-HOOK-TEST in step 4, before sim-wasm's first test
     that panics; 3A-FROM-F64 as its own step: new rows after the existing 10,048 so the 16 pinned rows keep their
     places, `--check`, the new sha256s recorded, the verifier regenerates the files.
   - **C6 (Medium):** on a clean copy, with the new specs, time the browser run and the whole job before the first
     evidence push. If the browser run would pass about 4 minutes, three lines to him first. A spec over 90 s sets
     its own limit. The 10-minute run stays outside `packages/e2e/tests/`.
   - **C7 (Low):** the release runs of room, client-endpoint, sim-wasm and devtools are local until the CI pull
     request lands; the recording's "three processes" means CI's two debug runs plus the local release run;
     whichever of 3b and the CI pull request merges second takes the other in and is checked again; the
     development build is run after steps 7 and 8.
   - **C8 (Low):** both of his messages into DECISIONS by script; his decisions of 21 and 22 November as a pointer
     to e8e276f0, not copied as his words on main.
   Not verified by it: it built and ran nothing (line counts, run times and the debug slowdown are estimates); on
   the netcode branch it read only the lines he named.

3. **The manifest batch (C5): commit 1ee3e918.** Path dependencies between our own crates and the root manifest's
   comment (3A-MANIFEST-COMMENT); held unbuilt until security-auditor (Opus 5.5, effort max) answered on exactly
   the six files: "The dependency change may be committed and built: yes", no finding, no condition, three Lows
   (`security-auditor-deps.md`; the record with the six hashes: `dependency-check.md`). CI on it alone: green,
   main's counts; the players' file 31,728 bytes (10 more than main's).
4. **Step 1, the message code: commits bab5a569 and 63d2bf7a.** `protocol`: the reader, the wrap-aware compares,
   the codec of the five messages, the transport interface, the loopback; the decoder test at its row's count
   (C1, C2): per type 10 million random and 10 million mutated seeded inputs in release (100,000 of each in
   debug, the test's name says which), 0 panics, no memory above the caps (our own counting allocator), the
   planted unchecked index found at input 17 and the planted over-allocation at input 0, a control for the
   counter. The release run of all five types takes about 4.4 s here. The crate denies unwrap, expect, indexing
   and plain arithmetic outside tests (clippy; shown biting on a plant; CI has no lint step). The types package
   exports the three new types. **Choice made (plan 5.1):** the player's record and the snapshot's own part hold
   buttons and press counters, no movement (it enters with change-set 4); the wire's command keeps its fields.
5. **Step 2, the simulation: commit 90da28c1.** The match setup with a start tick; the world's step takes the
   player's commands; the state is the tick, the player's record, then the seed last (54 bytes; the shared prefix
   is 22); `shared_hash`; known answers from a script.
6. **Step 3, the room and the client endpoint: commit 409b9434.** As the plan's row; the queue is a fixed array
   kept in sequence order (simpler than the design's ring; same behaviour); the endpoint's lost and doubled
   counts compare the room's echo with the command it says it ran. Releases are not counted in Rust: the page
   counts them from the echo (step 6).

7. **Steps 4 and 5, the Worker's one step and the messages: commit 4eeec33a.** `sim-wasm`: the local match
   (plain Rust, generic over the transport; the step in the draft's order), the bindings as a thin wrapper
   (`new(seed, mode, pad)`, `input` with six plain numbers, `step(now_ms)`; `newest_input` gone), the block's two
   new sections `echo` and `link` (44 bytes; each section's place printed into the generated layout),
   3A-HOOK-TEST (the hook's test in `tests/panic_hook.rs`). Native tests: every tick runs the newest post by
   number (1,800 ticks, late posts planted); 10,000 presses applied exactly once each; the tick and the sequence
   across their wraps with two runs agreeing on every state hash; 18,000 ticks take no memory once warm (a
   counting allocator of the test's own). The Worker: the newest post by its number, held buttons cleared on a
   hidden tab, the step told the host's time, each tick's start stamped (`startAt`), a due time that is not
   finite stops the match; the page's post renamed and still without keys. **Not built, with the recording (3c):**
   the one new read-only number on the players' file, the state's hash (DEPART-3).
   The players' WebAssembly file is now 56,717 bytes (23,023 gzip, 19,497 brotli), from 31,800 after step 3.

## Size so far (counted as 3a was: added lines under `crates/`, `packages/`, `.gitignore` and the root manifests; lockfiles and generated or fixture files apart)
- After steps 1 to 3: 3,723 added lines against about 1,835 planned for them (2.0 times). **After steps 1 to 5:
  see the count in "Key numbers"** (written with each save).
- **Projection (C4) and the ladder:** steps 1 to 6 with the two code follow-ups project to about 6,250 lines: over
  the ladder's 4,900, under the stop line of 7,600. With the first recording and the link simulator it would pass
  7,600. **So all three parts move to a change-set 3c: the link simulator's rate limit and duplication, the link
  simulator as a whole, the first recording, each with its tests. He was told in three lines on 2026-11-24, about
  23:56 UTC, before anything moved** (his message of 23:24:50 UTC, point 4; his item (c)); nothing of steps 7 and
  8 is built, and nobody builds the link simulator elsewhere.
- With both moved, `devtools`' three new path edges serve nothing in 3b (security-auditor's Low L2): looked at
  again before the report; a change to that manifest is a new batch under the hold.

## Current step
**Step 6 next: the page, alive** (the keyboard by physical key with running press counts, the key diagram lit
from the server's echo, the press tally, the readout's and the report's new rows, the browser test of the loop,
the presses test by PM-0002-12's own method, the allocation spec over the new per-frame code). Then 3A-FROM-F64,
the size check, the reviews. Updated 2026-11-25, 00:12 UTC (from the clock). The board add-on is active (nothing
of it is copied here, by his rule).

**CI on 5c582ddd (steps 1 to 3), read line by line:** gate, smoke and toolchain green (toolchain run
45514237788); every count as written before the push; the decoder test's five "10_million" names in the release
run (4.98 s on the runner); the players' file sha256 e7b7c2a2...7d71, the same as here; 14 browser tests in 1.1
minutes.

**CI, expected on the push of 4eeec33a** (written before the push): cargo debug, twice: client-endpoint 25,
devtools' f32 vectors 3 and stream vectors 2, protocol 19 and its decoder test 8, room 12, sim 17, the scan 5,
the allocation count 1, sim-wasm 12 with its hook test 1 and its step allocation count 1. Release (sim and
protocol only): protocol 19, the decoder test 8 (the "10_million" names), sim 17, 5 and 1. Node: client 70,
sim-worker 46, types 7, net 1, the browser package's Node project 1; boundary 2 files and 18 tests; 14 browser
tests; the type checks all "ok"; the players' file 56,717 bytes, sha256 ded90598...f4f8. Locally the whole
sequence ran green on 4eeec33a (`.gate/cs3b/run-steps45.log`, scratch; the browser tests in 1.0 minute).

## Next steps
1. (Reading done after his compact; the build order stands:) read CLAUDE.md, this card, the plan's sections 0, 3, 5, 8, 9, 12 and 13, his
   two messages of 22:44:50 and 22:59:49 UTC, `advisor-cs3b-start.md`; read the session's model and mode; tell him
   where things stand; then build 3b by the plan's section 5 under C1 to C8, in this order: (i) the manifest batch
   under the dependency hold (C5); (ii) the plan's steps 1 to 6, with the decoder test at its row's count (C1,
   C2), 3A-HOOK-TEST in step 4 and 3A-FROM-F64 as a step of its own (C5); (iii) the size projection before steps
   7 and 8 (C4), telling him first if a part moves; (iv) steps 7 and 8 as far as the ladder leaves them.
2. Owed with 3b's docs pass: his message of 22:44:50 UTC into DECISIONS by script; #65's merge in STATUS, the
   archive and ROADMAP; the stale "no pull request yet" sentences of 3a's report, STATUS and the archive (the
   advisor agent's Low on 1624f084); the build plan's two places on command timing (his item (b)), with the old
   and the new sentences in the report (three pairs if he says yes to the third, C3); his two messages of
   2026-11-24 by script and a pointer to his decisions of 21 and 22 November at e8e276f0 (C8). (3a's follow-ups
   3A-FROM-F64, 3A-HOOK-TEST and 3A-MANIFEST-COMMENT are build steps, C5, not docs.)
3. Not this chat's: the small CI pull request (an Opus 5.5 chat: Firefox in CI, every crate's release tests, the
   two generators' checks, a build step of its own for the test-only module).

## Open findings
- None yet for 3b.

## Key numbers
- main 28254a73; 3a's last commit 1624f084; 3a's notes 6491b839; the players' Wasm on main: 31,718 bytes, sha256
  383f53039ca93651f3d1655deb956040f688b45b0a9d347199c6f08f9f833963 (here and on the runner).
- 3a: 3,562 added lines against about 2,080 planned (1.71 times). 3b: about 3,800 planned, stop line 7,600.
- CI on main today: cargo debug client-endpoint 15, devtools f32 3 and stream 2, protocol 2, room 1, sim 15, the
  scan 5, the allocation count 1, sim-wasm 9, twice; release protocol 2, sim 15, 5 and 1; client 70, sim-worker
  42, types 7, net 1, the browser package's Node project 1; boundary 2 files and 17 tests; 14 browser tests.

## Decisions taken
- 2026-11-24, 22:44:50 UTC: his go for #65 was his tap; the next CI edit is an Opus 5.5 chat's; 3b is next.
