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
- **His decisions of 21 and 22 November that his message names** (the netcode test's marks that moved to the real
  game) are not in `docs/DECISIONS.md` on main 28254a73 (no entry of those two days): the three items bind as his
  message of 22:44:50 UTC words them.

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

## Current step
**The advisor agent reads his three items against the plan on main 28254a73** (notes
`cs3b-loop-closes/advisor-cs3b-start.md`). Then the message to him in at most 6 lines: what changes in size,
whether anything in the plan no longer holds, whether this chat should be compacted (the chat's answer: yes, the
context stands at about 70 %, and his words of 2026-11-23 ask to be told above 40 %).

## Next steps
1. After his compact (if he does it): read CLAUDE.md, this card, the plan's sections 0, 3, 5, 8, 9, 12 and 13, his
   message of 22:44:50 UTC, `advisor-cs3b-start.md`; read the session's model and mode; tell him where things
   stand; then build 3b by the plan's section 5 with the advisor agent's conditions.
2. Owed with 3b's docs pass: his message of 22:44:50 UTC into DECISIONS by script; #65's merge in STATUS, the
   archive and ROADMAP; the stale "no pull request yet" sentences of 3a's report, STATUS and the archive (the
   advisor agent's Low on 1624f084); the build plan's two places on command timing (his item (b)), with the old
   and the new sentences in the report; 3a's follow-ups placed at 3b: 3A-FROM-F64 (the cast from f64: an exact
   tie and a case near the overflow point among the inputs), 3A-HOOK-TEST (the panic hook's test in its own test
   binary), 3A-MANIFEST-COMMENT (the root manifest's comment, with 3b's manifest batch).
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
