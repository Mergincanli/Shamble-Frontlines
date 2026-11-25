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

8. **Step 6, the page alive: commits 64c65184 and the keyboard test's added case (the commit after f8db0de3).**
   The keyboard by physical key (W, A, S, D, Space, either Shift), the press tally from every block's echo, the
   key diagram lit from the echo (development builds only, on the production check's list), the readout's
   "messages" and "presses" lines and the report's rows, the key-to-tick reading (PM-0002-13's first reading),
   `picture.ts`'s sentence on the blend fraction (NA-N3). Browser tests: `loop.spec.ts` (a one-minute local match:
   here 1,826 ticks in 60.9 s, 117 presses made and echoed, 0 refused, 0 WebAssembly calls by the page; key to
   tick median 191 ms at this container's 6 frames a second) and `presses.spec.ts` (PM-0002-12 by its own method:
   10,000 presses and 10,000 releases made and echoed exactly, 396 posts delayed, 38 sent a second time behind two
   newer posts, the Worker ignored 38, 0 refused). **Its two bites, planted by hand, each turned it red**
   (`.gate/cs3b/presses-bites.log`, scratch): held-only: the taps never come back; last-to-arrive: the simulation
   refused 141 late posts (the Rust side keeps the newest post by number too, so no press was lost even then).
   The allocation test sums the listener's whole body and the new per-frame code (TE-NT7, PA-N2); its first
   reading found the key diagram allocating (a float array read per frame), fixed with whole-number times.
   **A lesson:** the press test's first script chose only three of the six keys (the low bits of its generator),
   which overflowed the waiting queue (255 per button, by design); the test now asserts each key's share.
9. **3A-FROM-F64: commit f8db0de3.** 20 constructed cases after the existing 10,048 (exact halves at one and
   below the smallest f32; from the largest f32 to 2^128), each result worked out by hand in the generator's
   self-check; the old inputs are the new file's first 120,576 bytes (same sha256 as before) and the 16 pinned
   lines are unchanged. New: `f32-inputs.bin` 120,816 bytes, sha256 3e9d6149...3579; `f32-expected.txt` sha256
   12c0c2de...d1d1. Exact natively and in WebAssembly (Node; Chromium, both profiles), no new pin.
10. **The clean copy (C6), `.gate/cs3b-clean` at f8db0de3:** CI's whole sequence from a cold start took 4 min 57 s
   (00:39:14 to 00:44:11 UTC); **the browser tests: 16 passed in 3.4 minutes** of CI's 5-minute limit (the two
   new ones 1.1 and 1.3 minutes). Under the 4 minutes at which he is told first. The players' file there is byte
   for byte the one built here (56,717 bytes, sha256 ded90598...f4f8). The legacy gate did not run in the clean
   copy (its own tools were not installed there); it passes in the worktree.
11. **The planted defects of 3b: `cs3b-loop-closes/plants-3b.py`, first run: 34 of 34 caught** (2026-11-25,
   00:48:46 UTC; `.gate/cs3b/plants-3b-run1.log`, scratch): the message code (6, among them the unchecked index
   planted in the real reader, found by the decoder run at input 1 of the JoinAccepted run), the simulation, the
   room and the endpoint (11), the local match's step (3, among them a 16-byte allocation per step), the Worker
   and the page (10), and four in the browser (PM-0002-12's two bites; one 64-byte array per frame and per block
   against the allocation test).

## Size so far (counted as 3a was: added lines under `crates/`, `packages/`, `.gitignore` and the root manifests; lockfiles and generated or fixture files apart)
- **After steps 1 to 6 and 3A-FROM-F64: 5,825 added lines** (plus 102 in generated and fixture files), against
  about 3,095 planned for them: 1.9 times. **After the fix pass of round 1: 6,528** (plus 105): the fixes added
  about 700, of which `any_bytes.rs` is 403. The stop line is 7,600; round 2's fixes are still to come. He was
  told the 6,500 on 2026-11-25, about 01:56 UTC.
  **With round 2's first fixes: 6,637.**
- **The ladder was used:** the link simulator (its rate limit and duplication, then the whole of it) and the first
  recording, each with its tests, are change-set 3c's, built right after 3b and before change-set 4. **He was told
  in three lines on 2026-11-24, about 23:56 UTC, before anything moved** (his message of 23:24:50 UTC, point 4;
  his item (c)). Nothing of steps 7 and 8 is built, and nobody builds the link simulator elsewhere. With the
  recording, the one new read-only number on the players' file (the state's hash, DEPART-3) waits for 3c too.
- With both moved, `devtools`' three new path edges serve nothing in 3b (security-auditor's Low L2): to be taken
  out in a small manifest batch under the hold, answered first, before the reviews end.

## Current step
**The fix pass of review round 1 is built and verified here; next: commit and push it, read CI, then round 2**
(three reviewers at a time: first test-engineer, security-auditor and perf-auditor, who finish their own plans on
the fixed head; then engine-architect, netcode-architect and rust-reviewer on what changed), the verifier until
PASS, the advisor agent on the pushed head (with the question on M1), the docs pass, the report. Updated
2026-11-25, 01:55 UTC (from the clock). The board add-on is active (nothing of it is copied here, by his rule).

**Verified here on the fix pass, before the commit:** formatting and the lint check clean (all targets); cargo
debug 111 tests (26, 3, 2, 19, 8, 13, 17, 5, 1, 13, 2, 1, 1); release protocol 19 and 8 (the decoder run 5.8 s),
sim 17, 5, 1; client 83, sim-worker 46, types 7, net 1; the seven type checks; boundary 18; the gate; 16 browser
tests in 3.3 minutes (the loop 1,823 ticks, 128 presses and releases made and echoed, 0 doubled, 0 older posts;
the press test 10,000 presses and releases, 505 posts delayed, 46 sent twice, 46 sent behind a newer one, 46
ignored by the Worker, 0 ticks ran an older post, 0 refused; allocation 102 bytes a frame, 576 a block); the
players' Wasm 53,783 bytes (21,983 gzip, 18,368 brotli), sha256
cd63b8718f7b0ba0d461f50e683dd42d7c37e7d2305923af4bb199ecf47686e7. **Planted defects: 44 of 44 caught on the fixed
tree (2026-11-25, 01:58:23 UTC; `.gate/cs3b/plants-3b-run2.log`, scratch)**, among them the test reviewer's whole
second bite (`bite-counted-taken`, caught at `expect(end.olderPosts).toBe(0)`); afterwards the true tree was built
again and gives the same Wasm.

**What CI must show on the fix pass's commit (written before the push):** cargo debug, twice: 26, 3, 2, 19, 8,
13, 17, 5, 1, 13, 2, 1, 1 (111); release: protocol 19 and 8, sim 17, 5 and 1; client 83, sim-worker 46, types 7,
net 1, the browser package's Node project 1; boundary 2 files and 18 tests; 16 browser tests inside the 5 minutes;
the players' Wasm 53,783 bytes with the sha256 above.

**CI, read line by line:** before the fix pass, green on d117fca5 and on eb0d2314 (toolchain run 45520531338,
4 min 55 s of 15; 16 browser tests in 3.4 of 5 minutes; every count as written before the push). **On the fix
pass 0dfca749: gate, smoke and toolchain green** (toolchain run 45529670690, job 138921570476, 01:59:13 to
02:04:00 UTC: 4 min 47 s of its 15 minutes); **every count as written above before the push** (cargo debug twice
111; release 19, 8 with the decoder run in 5.04 s, 17, 5, 1; client 83, sim-worker 46, types 7, net 1, the
browser package's Node project 1; boundary 18; 16 browser tests in 3.3 of the 5 minutes); the players' Wasm
53,783 bytes with the same sha256 as here. There: the loop 1,816 ticks in 60.5 s, 186 presses and releases made
and echoed, 0 doubled, 0 refused, key to tick (now from the key event's own time) median 167.7 ms and 95th
percentile 185.0 ms at about 10 frames a second; the press test 10,000 presses and releases made and echoed, 713
posts delayed, 68 sent twice, 68 sent behind a newer one, 67 ignored by the Worker when the report was read, 0
ticks ran an older post, 0 refused; allocation 90 bytes a frame and 597 a block at 8 frames a second (3.8 blocks
per frame; the limits there: 160 and 680). The log: `.gate/cs3b/ci-0dfca749-toolchain.log` (scratch).

**Round 2 so far (updated 2026-11-25, 02:27 UTC):**
- **security-auditor: PASS** (`security-auditor-review2.md`). Its Medium M1 (no bound on how late a command may
  run): **answer accepted, deferred in writing, not closed**: the hard stop is "before online play" (no room
  behind a network transport without the bound); build it in change-set 7 with each of the two timing ways, each
  with a long-hold test (idle N ticks, then the backlog: at most the repeat window of two-command ticks; honest
  outages 0 refused); the report must not read as if running on arrival had no bound (both options bound the
  backlog as their sources build them). L1 fixed, L2 accepted. **L3 (Low, named again):** `World::shared_hash`
  left the whole state, seed included, in the caller's buffer: **fixed now** (the bytes behind the shared ones are
  overwritten and cut off). **New Low:** the random-message test's "no command twice" was only "not the one that
  ran last": **fixed**, and the stronger check found something (next line). Nit (the boundary test's filter on
  wasm-bindgen's own names): **fixed** (the names are listed outright). Its line on press counters (an honest
  client after an outage brings up to 255 presses of one button in one command): written on
  `sim::PlayerCommand`; **follow-ups for change-sets 4, 8 and 9** (one action or its cooldown's worth, each with
  a test).
- **Found by the stronger check, in the room:** the queue's window followed every command taken out, also one the
  room then refused. So one refused command far ahead shut out the honest ones behind it for up to 32,767
  commands, and behind it a sequence could come round the circle again. **Fixed:** the window follows the last
  command that RAN (`CommandQueue::ran`, called by the room after its checks); tests in the queue and the room;
  two plants. The random-message test now asserts, per match, that the sequence moved by exactly the commands
  run plus the ones counted missing. **netcode-architect and rust-reviewer review this in round 2** (their head
  is the commit that holds it).
- **engine-architect: PASS** (`engine-architect-review2.md`): its Medium and its Lows fixed or their answers
  accepted ("no code path exists only offline"). **Four new Lows, all fixed in the next commit:** the
  random-message test sat in `sim-wasm` though neither half meets the other side: **moved**, each half into the
  crate whose rules it checks (`crates/room/tests/any_bytes.rs`, `crates/client-endpoint/tests/any_bytes.rs`; no
  manifest change); two counters named `unsent` with different meanings: the endpoint's is `unwritten` now;
  `Sim::input`'s doc says the last post handed in is used; a Worker that starts again no longer counts one
  spurious older post (`WorkerLink` resets the post used on `ready`; a test and a plant). **For 3c** (into its
  plan): a `link()` accessor on `LocalMatch`; the time reaches the simulator through `LocalMatch` from recorded
  times; the simulator's loss and rate limit as silent drops; duplicated datagrams only; corruption as loss; the
  host's time recorded with every tick; the state-hash export returns the shared hash only, as plain numbers,
  read only during a replay, its buffer private to `Room`. **Follow-up for Slice 3:** the block's `refused` gets a
  field of its own for what online play has no room count for.
- **CI on 6380df45 (the security reviewer's points and the queue's window): gate, smoke and toolchain green**
  (toolchain run 45533209579, 02:28:40 to 02:33:49 UTC), every count as written before the push, 16 browser tests
  in 3.4 minutes, the Wasm's sha256 as here.
- **test-engineer: PASS** (`test-engineer-review2.md`; nothing Critical or High open). Its High of round 1:
  fixed (its own plant now fails on the mark's quantities). The releases: **answer accepted with corrections**:
  met at the echo's resolution "0 lost and 0 doubled releases"; **not met: "the Worker counts the presses and
  releases it applies through the edge counters"** (there is no release counter, and the page counts); the two
  drafts disagree (ADR 0002 part 5, line 319: the post carries "press and release edge counters… (their exact
  fields are ADR 0001's)"; ADR 0001 part 5, line 226: press counters only); a release counter is the owner's
  decision; **the report quotes both and never calls the release half met by the mark's method.** The allocation
  test: answer accepted. **Three new Mediums and three Lows, all fixed in the next commit:**
  - the press test compared the shim's live count with the Worker's report (red in 2 of 5 runs on a fast page):
    the shim is switched off after the script, and the two counts must be equal;
  - presses were compared as totals (its plant: two buttons' counts swapped passed): **button by button** now
    (`lostAndDoubled`, `presses.lost` and `presses.doubled` apart);
  - every second copy counted as "sent behind a newer one", so that line could not fail: first copies overtaken
    are counted apart and printed (0 or 1 at this page's frame rate: **the mark's "reorders most neighbours" holds
    only on a page that draws fast; the report says so**);
  - "as far as the echo allows" was not reached: **the link now checks every block's held keys against the post
    its tick used** (`WorkerLink.wrongKeys`, asserted 0 in both specs): a release or press that comes early, late
    or rolled back shows per tick, with no message change;
  - Lows: the two "20 cases" comments say 24; a test reads the block's `refused` through the layout; the loop
    test bounds the key-to-tick reading (above 0, the 95th percentile under 2 seconds).
- **perf-auditor: PASS** (`perf-auditor-review2.md`). The fix pass made the page cheaper (in Node 186 to 197 bytes
  a block at every frame rate, 213 to 348 before), cost the Worker nothing (658 ns a tick against 663; 451 or 462
  bytes a tick on both trees; a warm Rust tick allocates nothing) and cut 2,934 bytes of Wasm (growth over main:
  22,065 bytes: codec 7,940, endpoint 4,631, room 3,253, Vec growth 1,402, sim-wasm 1,077, UTF-8 check 694,
  function names 2,300; ADR 0002 part 4 sets no limit yet). **It found the cause of the per-block rise:** the
  browser's optimising compilers install code for the drawing path and that is charged to the listener's own
  frame; the link's own work is 523 to 568 bytes a block at every rate (17 runs, 1.1 to 59.7 frames a second).
  Low (the spec's header said the cause was unknown): **fixed**, and the link's own sum is printed. Pool misses
  at 6 frames a second are by design (0.3 to 3 KB a second; 0 at 10, 30 and 60). Key to tick in its model: the
  95th percentile 34.8 to 48.0 ms with 1 ms from the key's own time to its handler; **2 of 16 phases go over the
  mark's 50 ms at a 4 ms delay, 6 of 16 at 8 ms.** **Follow-ups:** post the keys from the key handler too
  (32.9 to 40.1 ms in every phase) before PM-0002-13 is judged; the pool from 4 to 16 once the block grows;
  for PM-0002-8, under key input on a saturated main thread Chrome held blocks back 2 to 4.5 s here; change-set
  5's allocation method runs with the optimising compilers off or sums under the link's own function.
- **Found while re-running, a flaw in the loop test itself:** it started its clock after the first reading came
  back, so on a slow page the measured minute could be shorter than the ticks' minute: red by one tick, once
  (1,823 ticks against a bound of 1,822). **Fixed:** the clock starts before the first reading.
- **netcode-architect: PASS** (`netcode-architect-review2.md`, on 6380df45). Its Medium and Lows: fixed or
  answers accepted (on its own test link, 9 honest runs with delay, jitter, 2 to 5 % loss, 1 to 5 % doubled, heavy
  reorder and outages of 45, 100 and 3,000 ticks: refused 0, late 427 to 3,619, and received + late + refused
  equalled the datagrams delivered in every run). **The queue's window is right** (40 seeds of 250,000 random
  operations against its own model, hostile sequences anywhere and 0 to 100 % of taken commands refused: 893,122
  runs, 726,413 refusals, 0 deviations; the same operations on round 1's window: 24,393 runs not newer than the
  last and 249,445 honest commands shut out); nothing changes for an honest client; a refused copy offered again
  costs about 40 ns. **Four new Lows:** two more checks in the random-message tests (per step the sequence moves
  by exactly the commands run plus the missing counted in that tick; per match received + late + refused equals
  the datagrams handed in): **to fix**; `ran(sequence)` trusts its caller: **to fix** (the queue remembers what
  it took out last); `Stale`'s doc covers more than a late repeat: **to fix**; the follow-up on the press check
  past 255 ticks should say presses are LOST there (its 3,000-tick outage lost 1,024 of 2,520 presses per button,
  exactly 4 times 256; PRESS-PACE asked about taps within one tick, so no rule of his is loosened): **the
  follow-up's wording, in the report**.
  - **On M1 it agrees** with the deferral, the deadline (no room behind a network transport: Slice 3's server)
    and the place, change-set 7's scheduler seam. The form under running on arrival: a per-client budget, +1 per
    room tick, banked to at most the repeat window (8), one spent per command run, commands over it dropped and
    counted outside `refused()`. The form under buffering: a command whose tick lies more than 8 ticks before the
    tick being run is dropped as late. One mark for both: idle 30, 300 or 3,000 ticks, then the backlog gives at
    most 8 extra commands; honest outages give refused 0 and at most 7 two-command ticks. **For the owner's
    decision at the next edit of ADR 0001:** PM-0001-21's lag-switch case holds for only 200 ms (6 ticks, inside
    the window), so the row passes a room with no bound; a long-hold case (60 s) is proposed.
  - **For 3c:** the link simulator models corruption as loss only (what an authenticated transport delivers:
    RFC 9001 section 6.6, RFC 8446 section 5.2); `late` is checked against the simulator's own reorder count.
    **Before online play (change-set 7):** the endpoint refuses and counts a snapshot more than the estimate plus
    30 ticks ahead and a confirmation newer than the newest sequence sent.
- **rust-reviewer: PASS** (`rust-reviewer-review2.md`, on a85b9dbb): all eight Lows fixed or their answers hold
  (the Wasm's formatting and panic code is main's again; the growth over main, 22,065 bytes, sits in protocol
  7,930, client-endpoint 4,625, room 3,249, alloc 1,398, sim-wasm 1,076, core 693, the name section 2,300, data
  540; without the name section, the open follow-up 3A-NAME-SECTION, the file would be 43,560). Its probes: the
  queue's window against a model over 16 million operations; the room 4 start ticks of 2 million calls and the
  endpoint 3 runs of 2 million calls, no panic, the per-step and per-datagram rules exact, 0 allocations after
  warm-up; the block's bytes over 1 million ticks hash the same natively and in the Wasm under Node
  (7dc617c2063b06d3). **Two new Lows:** `ran` held only if nothing was offered between a taking out and its run:
  **fixed** (the queue names its own last command; a debug-build check with a test; a plant); the report quotes
  each head's own compressed sizes: **in the report**.
- **All six reviewers: PASS in round 2.** Nothing Critical or High is open. What they found in round 2 (Mediums:
  three from test-engineer; Lows from all) is fixed in the commits after 0dfca749, or is a named follow-up.

**What CI must show on the commit that closes round 2's fixes (written before the push):** cargo debug, twice:
26, 1, 3, 2, 19, 8, 16, 1, 17, 5, 1, 13, 1, 1 (114); release: protocol 19 and 8, sim 17, 5 and 1; client 85,
sim-worker 46, types 7, net 1, the browser package's Node project 1; boundary 18; 16 browser tests inside the 5
minutes; the players' Wasm 53,813 bytes (21,994 gzip, 18,464 brotli), sha256
5ebae15400f9435dfd31108fe380e96253695b8bed4a7e639ce94bc180b41262. Run here first: all of it; the 16 browser tests
in 3.3 minutes on this Wasm and this page (the loop 1,813 ticks, 127 presses and releases, 0 lost, 0 doubled, 0
older posts, 0 wrong keys; the press test 10,000 presses and releases, 495 posts delayed, 45 sent twice and all
45 behind a newer post, 1 first copy overtaken, 46 ignored by the Worker, exactly; allocation 111 bytes a frame,
590 a block, 561 of them the link's own work). Planted defects: 53; the seven that need a browser were caught
(03:02:14 UTC), the queue's and the tests' seven too (03:03:44 UTC); the full run on the final head follows.
Size: 6,909 added lines (stop line 7,600).
**CI on 6abf3d2a (the final code): gate, smoke and toolchain green** (toolchain run 45537884772, job
138948885336, 03:08:37 to 03:13:35 UTC: 4 min 58 s of its 15 minutes); every count as written above; 16 browser
tests; the Wasm's sha256 as here. There: the loop 1,814 ticks in 60.6 s, 138 presses and releases, 0 lost, 0
doubled, key to tick median 238.1 ms and 95th percentile 257.8 ms on a runner that drew about 7 frames a second;
the press test 10,000 presses and releases made and echoed, 0 lost, 0 doubled, 502 posts delayed, 46 sent twice
(all 46 behind a newer post), 1 first copy overtaken, 47 ignored by the Worker (exactly the sum), 0 ticks ran an
older post, 0 ran other keys than their post's; allocation 91 bytes a frame, 590 a block (569 of them the link's
own work). The log: `.gate/cs3b/ci-6abf3d2a-toolchain.log` (scratch).

**After the code (started 2026-11-25, about 03:09 UTC):** the full run of the 53 planted defects on 6abf3d2a;
the verifier (its copy `.gate/verifier-cs3b`, its notes `verifier-cs3b.md`); docs-keeper's docs pass (its notes
`docs-keeper-3b.md`). Then the report, the advisor agent on that head (with the question on M1), the zip.

**What CI must show on the commit with engine-architect's four Lows (written before the push):** cargo debug,
twice: 26, 1, 3, 2, 19, 8, 15, 1, 17, 5, 1, 13, 1, 1 (113: the two halves of the random-message test are now one
test each in `client-endpoint` and `room`); release: protocol 19 and 8, sim 17, 5 and 1; client 84, sim-worker
46, types 7, net 1, the browser package's Node project 1; boundary 18; 16 browser tests inside the 5 minutes; the
players' Wasm unchanged: 53,783 bytes, sha256 1e0e2dd5...0a4a. Run here first: all of it except the browser
tests (nothing the browser runs changed but one reset on a second `ready`, which no browser test reaches).
Planted defects: 47; the five that this commit touches were run and caught (02:36:05 UTC). Size: 6,747.
**CI on a85b9dbb (that commit and the one that removes the moved test's old file): gate, smoke and toolchain
green** (toolchain run 45534233480, 02:37:05 to 02:41:58 UTC: 4 min 53 s); every count as written above; 16
browser tests; the Wasm's sha256 as here. That runner drew about 6 frames a second (49 frames in the allocation
window): 80 bytes a frame, 582 a block; the press test 10,000 presses and releases made and echoed, 47 posts sent
behind a newer one, 46 ignored when the report was read, 0 ticks ran an older post.

**What CI must show on the commit with these fixes (written before the push):** cargo debug, twice: 26, 3, 2, 19,
8, 15, 17, 5, 1, 13, 2, 1, 1 (113); release: protocol 19 and 8, sim 17, 5 and 1; client 83, sim-worker 46, types
7, net 1, the browser package's Node project 1; boundary 18; 16 browser tests inside the 5 minutes; the players'
Wasm 53,783 bytes, sha256 1e0e2dd5ddc929ae1926590e48954ac8105551e92dbe09e02690713b68ab0a4a. Run here first: all
of it; of the browser tests the two that reach the room (the loop and the 10,000 presses: 2 passed in 2.4
minutes, 10,000 presses and releases made and echoed, 42 posts sent behind a newer one, 42 ignored, 0 older
posts), the other 14 by CI (the port is shared with two reviewers). Planted defects:
46; the six that touch the queue and the room's order were run and caught (02:24:53 UTC).

**Round 2, first three (started 2026-11-25, about 02:00 UTC):** test-engineer, security-auditor and perf-auditor
(Opus 5.5, effort max), each on its own copy `.gate/<name>-cs3b2` of 0dfca749; what they were given:
`cs3b-loop-closes/review-context-2.md`; their notes `<name>-review2.md`. Then engine-architect, netcode-architect
and rust-reviewer on what changed.

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

## Open findings (review round 1 on bbbf09e4; the fix pass is built; updated 2026-11-25, 01:55 UTC)
**What happened to the round:** engine-architect, netcode-architect and rust-reviewer finished. **The container
restarted at about 01:32 UTC and stopped test-engineer, security-auditor and perf-auditor before their answers**;
their notes (`<name>-review1.md`, written as they went) hold their findings up to then, every one of them is taken
into the fix pass below, and the three finish on the fixed head in round 2 (their remaining plan items plus the
check of the fixes). Nothing else was lost: the fix pass's edits were on disk.

- **engine-architect: CHANGES NEEDED, one Medium, eight Lows** (`engine-architect-review1.md`).
  - Medium EA-1 (the block writer and the newest post lived in `sim-wasm`): **fixed**: `Endpoint` owns both
    (`new(hash, first_sequence, pad_bytes)`, `set_post`, `block`, `block_len`, `write_block`); `LocalMatch` keeps
    the room, the endpoint, the link, the schedule and a count of messages the link did not take.
  - Low EA-2 (a section's start worked out twice): **fixed**: `layout::end_of` is the one place; the generator
    uses it and prints `BLOCK_BYTES`, which the page uses.
  - Low EA-3 (the newest post guarded twice): **fixed**: one owner, the host (TypeScript); Rust takes the post it
    is given; `Post::newer_than` and the Rust count of stale posts are gone.
  - Low EA-4 (a key held when the focus moves into a text box stays held): **fixed** in `KeyboardInput.key`
    (a key-up counts wherever it lands), with a test.
  - Low EA-5 (the host time's check read by nothing): **fixed**: gone; `step` keeps taking the time (3c's link
    simulator reads it).
  - Low EA-6 (`devtools` names three crates it does not use until 3c): **no manifest change**; in the report.
  - Low EA-7 (the boundary test did not pin `Sim`'s methods): **fixed**.
  - Low EA-8 (PA-N2 not met): **report**: carries over to change-set 5, never "done".
  - Low EA-9 (the key diagram's CSS reaches a production build): **follow-up**.
- **netcode-architect: PASS, one Medium, six Lows** (`netcode-architect-review1.md`).
  - Medium NA-1 (late or doubled snapshots counted as refused): **fixed**: `Readout.late`.
  - Low NA-2 (no window before the first command is taken): **fixed** in `CommandQueue::insert`, with a test.
  - Low NA-3 (the newest command could stop going out): **fixed**: it always goes; only repeats are filtered.
  - Low NA-4 (`press.rs` said "never dropped"): **fixed** (the doc states the bound of 255 waiting); the report
    names it a bounded departure; the bound's size is a game-design **follow-up**.
  - Low NA-5: **fixed now**: the Join's version before its token. **Follow-ups before online play:** the acks'
    lower bound and one ack per group; the press check past 255 ticks.
  - Low NA-6: fell away with EA-5. Low NA-7: **into 3c's plan** (the simulator's clock through `LocalMatch`; its
    reliable channel delivers exactly once).
- **rust-reviewer: PASS, eight Lows** (`rust-reviewer-review1.md`).
  - RR-1 (two panics' debug formatting cost 3,121 bytes of Wasm): **fixed**: one `assert!` with a plain message;
    the players' Wasm is 53,783 bytes now (56,717 before).
  - RR-2 (`post.rs`, `press.rs` denied two of the four lints): **fixed**.
  - RR-3 (a changed datagram that still decodes stalls the exchange): **into 3c's plan** (the simulator treats
    corruption as loss, or the endpoint gets a plausibility bound).
  - RR-4 (no random-input test of the room and the endpoint in CI): **fixed**: `crates/sim-wasm/tests/any_bytes.rs`
    (100 matches of 3,000 calls each side, each side's rules asserted at every step).
  - RR-5 = EA-6. RR-6 (`#[must_use]`, doc comments): **fixed** where a false result leaves a stale buffer;
    `pub mod queue` and `pub mod wire` stay public (**follow-up**, nit).
  - RR-7 (plain forms; one record per button in `press.rs`): **fixed**.
  - RR-8 (two more cases of the cast from f64): **fixed**: 4 cases appended (both signs); 10,072 cases;
    `f32-inputs.bin` 120,864 bytes.
- **test-engineer (cut short; its notes up to claim 5 of 8):**
  - **High, proven by its own plant:** the presses mark's second bite planted whole (the Worker counts a late
    post as ignored and takes it all the same) passed every line of `presses.spec.ts`. **Fixed:** the page now
    counts ticks that ran an older post than the tick before (`WorkerLink.olderPosts`) and buttons held again with
    no press (`EchoTally.heldWithoutPress`); both specs assert 0; the whole bite is plant `bite-counted-taken`.
  - **High candidate:** the releases are worked out from the presses and the held bits, so their line cannot fail
    for a doubled release. **Fixed as far as the echo allows:** a lost release still shows as a release that never
    comes back; a doubled one shows as `heldWithoutPress` (asserted 0). The echo has no counter for releases
    (ADR 0001's command carries press counters only): **the report says so against the mark's wording.**
  - **High candidate:** the allocation test's per-block limit holds only for a slow page (its own readings: 593
    bytes a block at 6 frames a second, 742 to 797 at 42 to 60). **Partly fixed:** the list of returned buffers
    no longer empties (no list grows inside the block listener); measured again at six frame rates (478, 567,
    591, 765, 747, 629 bytes a block at 0.9, 5.9, 10.1, 23.2, 38.7, 60.0 frames a second): the rise in the middle
    stays and its cause is not known. So the spec now refuses by name a run with fewer than 2.5 blocks per frame,
    and its per-frame limit follows the blocks per frame. **Follow-up for change-set 5** (the mark's own method).
  - Low (the decoder test printed the token's cap for every control message): **fixed** (a cap per type).
  - Low (no case of nine whole commands): **fixed** (and five whole acks).
  - The shim's count of posts sent behind a newer one (the mark's "reordered, above 0"): **added and asserted**.
- **security-auditor (cut short after its findings; CHANGES NEEDED in its own memory line):**
  - **Medium M1:** the room has no bound on how late a command may run, so a client that sends nothing for N
    ticks and then a backlog runs two commands a tick for N ticks (its probe; the total stays within the ADR's
    bound). Nothing moves in 3b, so nothing is gained yet. **Not built in 3b; put to the advisor agent with the
    fixed head:** the bound belongs to the command-timing choice (ADR 0001's open question 6, whose option (b)
    names "late commands dropped"), which Mustafa gave to Slice 3 on 2026-11-24. Stated in the code where the
    check will go (`Room::on_datagram`'s doc) and a **follow-up before online play**.
  - Low L1 (the checks not made yet are not named where they will go): **fixed** (that doc).
  - Low L2 = EA-6. Low L3: not in its notes (lost with the restart); round 2 names it again.
- **perf-auditor (cut short; its notes up to question 5 of 6):** no finding so far. Its readings: the Worker's
  tick 268 ns on main, 650 ns now, 24 to 29 bytes more per tick; key to tick in its model at 60 frames a second:
  95th percentile 34.8 to 48.0 ms over 16 phases, none above the mark's 50 ms; the reading started at the
  handler, not at the key event's own time: **fixed** (`event.timeStamp`).

## Key numbers
- main 28254a73; 3a's last commit 1624f084; 3a's notes 6491b839; the players' Wasm on main: 31,718 bytes, sha256
  383f53039ca93651f3d1655deb956040f688b45b0a9d347199c6f08f9f833963 (here and on the runner).
- 3a: 3,562 added lines against about 2,080 planned (1.71 times). 3b: about 3,800 planned, stop line 7,600.
- CI on main today: cargo debug client-endpoint 15, devtools f32 3 and stream 2, protocol 2, room 1, sim 15, the
  scan 5, the allocation count 1, sim-wasm 9, twice; release protocol 2, sim 15, 5 and 1; client 70, sim-worker
  42, types 7, net 1, the browser package's Node project 1; boundary 2 files and 17 tests; 14 browser tests.

## Decisions taken
- 2026-11-24, 22:44:50 UTC: his go for #65 was his tap; the next CI edit is an Opus 5.5 chat's; 3b is next.
