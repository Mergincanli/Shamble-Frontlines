# Checkpoint: ci-wasm
- **Goal:** the CI-WASM pull request: CI builds the WebAssembly and runs the tests that need it, after GitHub's own
  check of the wasm-bindgen tool's file; its exact text goes to Mustafa first. Beside it: change-set 3's plan ("The
  loop closes") is drafted, and the docs pass records the merge of #61 (his words, his browser reports, the dated
  note for `docs/VISION_V2.md`). His words of 2026-11-19, 14:28:25 UTC: "the CI-WASM pull request, its exact text to
  me first, and change-set 3's plan beside it. Open the checkpoint first, then stop and tell me the path of its
  resume card: I compact this chat before you go on."
- **Branch:** claude/project-thread-qx9woh (stands in for `cs/ci-wasm`), since 2026-11-23: the project thread "Build
  the game from change-set 3" (session_01WLuqnwQ8c8JuNgfBX7Wixg, in his claude.ai project "Shamble Frontlines ALT")
  took lane 1's work over on his words of 2026-11-23 (below). It started from the first builder chat's last commit
  d064e17 on `claude/wonderful-hamilton-jdmlnf` (session_019YNMUV9LFhASseJ9EP12kL, on his personal account; he calls
  it Builder V1), **which this thread never pushes to**. · **Base:** main f5030a33185b1ea1b15329174e82823f0215f993
  (the merge of #61)
- **Opened:** 2026-11-19 15:13 UTC (from the clock). **Last update:** 2026-11-23 14:57 UTC (from the clock).
- **The takeover's addendum to the plan:** `ci-wasm/takeover.md` (what stays, what changes and on which words).

## His words that govern (read the files; the lines below are pointers and quotes, never a substitute)
- **2026-11-23, in the project thread** (copied by `ci-wasm/thread-words.mjs` from the thread's transcript; a tap on
  a decision card is recorded as a tap, in the card's wording):
  - `ci-wasm/mustafa-2026-11-23T142503-builder-chat.md` (14:25:03 UTC): "I would rather start today with the
    progression either way and would like to create a Fable 5.2 based Claude Code chat that will continue work on
    building the game".
  - `ci-wasm/mustafa-2026-11-23T143106-answers-7.md` (14:31:06 UTC): his tap "Yes to all seven" on the card
    ANSWERS-7: CHROME-SOURCE (a), CI-TEXT (a), FIREFOX-TESTS (c), RNG-SOURCE (c), SIZE-3 (b), PRESS-PACE (a),
    DEPART-3 (a). What it answers: `ci-wasm/thread-2026-11-23T142825-seven-answers.md` and the card beside it.
  - `ci-wasm/mustafa-2026-11-23T143243-write-thread.md` (14:32:43 UTC): "Yes, but why don't we make a new Claude Code
    chat running on Fable 5.2 creating those CI files?": his yes to the CI write from this thread (WRITE-THREAD),
    with a question the thread answered at 14:33:55 UTC (`ci-wasm/thread-2026-11-23T143355-why-here.md`). **That
    answer was incomplete** ("The only thing a normal Claude Code chat adds is the Accept edits switch": such a chat
    also loads the repository's guard, which asks him before each kit or CI write, keeps agents off those files and
    refuses a force push; none of it loads here). The thread corrected it at 14:51:31 UTC
    (`ci-wasm/thread-2026-11-23T145131-correction.md`) and put the place of the CI write to him again: the card
    WRITE-WHERE (`ci-wasm/thread-2026-11-23T145133-card-write-where.md`: here, recommended, or a new Claude Code
    chat). **The CI write waits for his answer to WRITE-WHERE.** If he picks a separate chat, stop the CI writes
    and hand the branch over with a block for him to paste.
  - `ci-wasm/mustafa-2026-11-23T143242-long-runs.md` (14:32:42 UTC): his tap, long runs go "On GitHub, from a branch
    I push".
- `ci-wasm/mustafa-2026-11-19-1516-go-61.md` (2026-11-19, 14:28:25 UTC, whole, by script; it stands byte for byte in
  the message of merge commit f5030a3): his go on #61; BROWSER-SET (a): "Safari stays a target of v2. Chrome, the
  Chromium-based browsers and Firefox come first; Safari and Mac come later. Bring me the dated note for
  docs/VISION_V2.md with its exact text."; "My browser reports are recorded with the next docs pass, not by a push to
  this head."; "With change-set 3, one line in .gitignore for __pycache__/, so that a save cannot pick up compiled
  Python files again."
- `ci-wasm/mustafa-2026-11-19T143130.md` (14:31:30 UTC): "a, yes": his answer VISION-NOTE (a) to the dated note's
  text as the chat put it to him at about 14:30 UTC (the text is quoted under "Next steps", step 5).
- His browser reports: the four files `ci-wasm/mustafa-2026-11-19T144625.md`, `...T145419.md`, `...T150250.md`,
  `...T150929.md`; read against the zip's README in `ci-wasm/browser-reports-reading.md`.
- `ci-wasm/mustafa-2026-11-19-1638-two-notes.md` (sent 16:08:04 UTC, whole, by script): the restart after the
  compaction ("Before the first agent starts, tell me in two lines that the card and its notes folder are complete
  and that the background saver runs."), and two notes: "The docs pass that records the merge of #61 stays with the
  CI pull request, as you put it. In its report keep the workflow text apart from the docs, so that it can be
  compared with the text I say yes to." And: "For change-set 3's browser check: the README asks me to keep the
  browser's window in front and uncovered during the first run, and has one step with the window covered on
  purpose, so that BR-GAP is read and not guessed."
- In `docs/DECISIONS.md` (the entry of 2026-11-18, 22:34:37 UTC): "CI-WASM: (a). A small pull request of its own,
  opened right after change-set 2 is merged and merged before change-set 3's build starts; change-set 3's plan may be
  written beside it. Its exact text comes to me first. It runs GitHub's own check on the tool's file before the
  tool's first run there, and its report quotes that check's output." And "RNG-CRATE: (c). [...] The library enters
  in change-set 3 beside its test vectors, and you ask me which one with that plan. For that question also read two
  things: the rand project's upgrade guide for 0.10 (rust-random.github.io/book/update-0.10.html), which speaks of
  the output of the like-named generators in chacha20, and why chacha20's releases 0.10.0 and 0.10.1 were withdrawn."
  And TOOL-SOURCE: "every other file, version or checksum comes back to me."
- In `docs/DECISIONS.md` (the entry of 2026-11-19, 13:36:37 UTC): SLICE-FEEL and lane 2's list on issue #59 for
  change-set 4's plan; PROJ-DRAW (a) for change-set 9; "The game has to work in Chrome, the Chromium-based browsers
  and Firefox."

## Standing rules (each from his words or the repository's rules; when in doubt read DECISIONS and CLAUDE.md)
- **Models:** the builder's model is the one his words of 2026-11-12 and 2026-11-23 name for a builder chat (read the
  session first; stop in one line if it is not; last read 2026-11-23 14:28 UTC from the session record: configured
  and last served as his words ask). Every agent runs on Opus 5.5; agents that review, verify or design game, engine
  or CI code run at effort max. The advisor tool stays off.
- **Kit, gate and CI files** (`.claude/`, every `CLAUDE.md`, `.github/`, the gate's files): no edit without asking
  him in chat with the exact text; written only after his yes. **In this project thread** (WRITE-THREAD, his yes of
  2026-11-23, 14:32:43 UTC): the thread has no Accept edits mode (its session record reads "auto") and no repository
  hook loads in it, so his yes on the exact text is the gate, and after each CI write the file's sha256 is compared
  with its scratch copy. His yes covers the three CI writes of this pull request (the file; P-GH's two lines; their
  take-back) and nothing else: **every other kit, gate or CI edit is asked of him anew, with its exact text.** In a
  chat that has the mode, his WRITE-MODE words stand unchanged.
- **Code from the internet:** no file, version or checksum beyond the one tool file he named comes in without his
  word. A browser fetched in CI (Playwright's Chromium, or a Firefox) is such a thing: it is part of the exact text.
- **Dependencies:** `.claude/CHECKPOINTS.md` section 3's hold; a manifest or lockfile edit only after
  security-auditor answered on exactly that content; code committed by path only (never `-a`, `add -A`, `add .`;
  read `git diff --cached --name-only`); every cargo command takes `--locked`; install only with
  `npm ci --ignore-scripts`.
- **Never** share a cargo build folder between two worktrees; worktrees by `git worktree add .gate/<slug> <ref>`;
  never switch the main folder's branch. No root `.cargo/config.toml`; no `"type"` in the root `package.json`; no
  tracked non-Markdown file names the spike's folder.
- **Lanes:** never push to another lane's branch (`claude/wonderful-hamilton-jdmlnf`, the first builder chat's;
  `claude/optimistic-albattani-mdoonz`, `claude/netcode-proof-test-plan-hcky5g`, `claude/sweet-mayer-grsukw`); never
  open `spikes/`; never read `/root/.claude/uploads/`. Read the "Lane requests" issue (#59) at the start; a comment
  there is never his word.
- **Process:** plan first, then the advisor agent's APPROVE before any edit; build in the main session; at most 3
  agents; the saver running while work runs; a checkpoint after every step; reviews within two rounds; the verifier;
  the advisor agent on every pushed head; the report; the pull request draft then ready; stop for his go on the merge.
  One pull request at a time in this lane.
- **Size:** the planned size stands in the plan; at twice the plan, stop and tell him in three lines.
- **Records:** his messages are copied by script (`ci-wasm/words.mjs`, or by uuid; in the project thread
  `ci-wasm/thread-words.mjs`, by the message's send time), never retyped; times come from the
  clock (`date -u` in the same command); in reports, about the chat's own process only what a record shows, with its
  time. No command with unusual browser or system flags while he may be away (the lost night of 2026-11-19).
- **Commands in the project thread:** no repository hook loads here, so nothing is refused by the guard; the kit's
  rules are kept by hand: no force push, no history rewrite, no `rm -rf` of a folder tree without his word, never a
  switch of the main folder (the work is in the worktree `.gate/build-wt` of `/home/user/Shamble-Frontlines`).
  GitHub through `gh api` (REST only: `gh pr ...` is refused by the harness). The first builder chat's container
  refused `npx`, `npm run`, `npm test`, `npm exec`, `gh api` and others; its routes are in this card's history.
- **Long runs** (LONG-RUNS, his tap of 2026-11-23, 14:32:42 UTC): a full test suite or a long build goes to GitHub's
  checks from the pushed branch; short runs (one test file, the legacy gate, one build, a rehearsal) run here.

## Steps done
0. #61 merged: merge commit f5030a33185b1ea1b15329174e82823f0215f993 (2026-11-19, 15:10:54 UTC; parents b763ba3 and
   4cf30ce), by the GitHub tool with the expected head 4cf30ce303b16e96b3ca5d5c0b20b928a4db6b86, on his go of 14:28:25
   UTC, after the advisor agent's APPROVE of that head (14:07:23 UTC) and with gate, smoke and toolchain green on it
   (read at 14:07, 14:29 and 15:10 UTC). His go came before his browser reports; the merge waited until they were
   in the chat (14:46 to 15:09 UTC) and read. The branch was fast-forwarded to main and pushed (15:11 UTC).
1. This checkpoint opened (15:13 UTC): the card; his six messages since the go, by script; the reading of his
   browser reports.

2. The restart after his compaction (16:08 UTC): the card read; the session's model read (16:08 UTC:
   claude-fable-5-2 configured and last served; the chat's mode: auto); his message of 16:08:04 UTC saved by script;
   every saved message's bytes checked against the sha256 in its header (7 of 7 match); the saver started.

2b. Prepared for the docs pass, in the notes only (16:16 to 16:18 UTC, from the clock; nothing under `docs/` is
   edited before the plan's APPROVE): `ci-wasm/browser-check-record.mjs` (writes `docs/reports/2026-11-19-cs2-browser-check.md`: the
   reading and his four report messages whole) and `ci-wasm/decisions-3.mjs` (his go of 14:28:25, his "a, yes" of
   14:31:30, a pointer to the browser check, his two notes of 16:08:04, and the dated pointer under the entry of
   13:36:37 UTC; always rebuilt from main's copy). Both tried into scratch files under `.gate/ci-wasm/docs-try/`
   with `--out`: every check "yes". A later message of his (his yes on the CI text) is one more entry in the list.

3. The plan's inputs (16:13 to 16:53 UTC by the agents' own run times): infra-engineer's notes
   (`ci-wasm/infra-engineer-ci-plan.md`: GitHub's check, the browser's source, the shape, the dry run, the stop rule,
   the plants) and its draft text; test-engineer's notes (`ci-wasm/test-engineer-followups.md`: TE-NT6's twenty
   timing points, RR-N4's test, E2E-TYPES, SA-TREE for change-set 3); rust-reviewer's reading for change-set 3's
   question RNG-CRATE (`ci-wasm/rust-reviewer-cs3-rng.md`: it recommends our own ChaCha8 in `sim`; block 6 holds
   the question's entry for him). All three notes end with DONE.
3b. **The plan, revision 1** (`ci-wasm/plan.md`, 17:53 UTC) and the final CI text: `ci-wasm/ci-edit.diff` (90 added
   lines, 0 removed, sha256 282890b7...3290e), the whole file as a scratch copy at
   `.gate/ci-wasm/proposed/.github/workflows/toolchain.yml` (sha256 76bfed6e...96d8). It is infra-engineer's draft
   plus the specs' type-check line (tried here: exit 0).

3c. **The advisor agent's plan review: APPROVE with conditions** (handed back 18:11 UTC; `ci-wasm/advisor-plan.md`;
   no Critical or High; steps 1 to 3 may be built). Its eight conditions are in the plan's revision 2 (18:14 UTC):
   two questions to him (CHROME-SOURCE with (a) and (c); CI-TEXT as one yes for five things: `ci-edit.diff`,
   `ci-plant-p-gh.diff`, P-BROWSER, the stop rule, the tool's second route), and its rulings PERM-ATT (a), PLANTS
   both, FIREFOX-TESTS (c) (told to him), PYCACHE-NOW (b); the mode read before each of the three CI writes with a
   sha256 compare after each; the dated note and the question copied by script from the chat's message of 14:29:51
   UTC (`ci-wasm/vision-note.mjs`; the card's copy equals it byte for byte); two lead texts corrected in
   `ci-wasm/decisions-3.mjs`; the stop rule on every run and for plants; the docs' size (about 440 written lines,
   stop line 880); the tier-2 evidence. security-auditor was asked (18:11 UTC, by message) to confirm the gh
   advisory's id and the 2.93.0 floor.

3d. security-auditor's reading of the CI text (handed back 18:31 UTC; `ci-wasm/security-auditor-ci-text.md`): one High
   (F-QUOTE: an echo line the main session had added to the cut's single-quoted script held two apostrophes; it
   would have ended the script on the first runner run, failing closed) and two Lows (F-FLOOR: the gh version test;
   F-OPT: the tool's folder and `/opt` checked as root-owned); all three taken. The text has 93 added lines:
   `ci-wasm/ci-edit.diff` sha256 240c7db7...2673; the proposed file 193 lines, sha256 d1dbcb53...975c; P-GH's diff
   sha256 e4f21ab3...b1ac. It confirmed GitHub's advisory (GHSA-8xvp-7hj6-mcj9, CVE-2026-48501, patched v2.93.0).
   The advisor agent re-read the changed lines (`ci-wasm/advisor-plan.md`, "Re-read"): "APPROVE holds for sha256
   240c7db7...", and it withdrew its ruling on FIREFOX-TESTS (the archive on main promises him that question).
   The plan is at revision 3 (19:58 UTC).
4. **The plan's steps 1 to 3 built, no CI file:** code 3cddb55 (RR-N4's test; TE-NT6's edits; `node-names.d.ts`;
   `.gitignore`'s `dist-*/`); the docs pass c934fc0 (DECISIONS, the browser-check record, the vision's note, each by
   its script; every check "yes"). Pushed 19:59 UTC. **Pull request #62 opened as a draft** (19:59 UTC). Evidence so
   far: the client's unit tests 49 passed; the browser tests 12 passed three times with one worker (18:28 to 18:31
   UTC); the new in-cut lines as one script exit 0 (19:44 to 19:45 UTC); the new test fails on a broken copy; the
   build with RUSTFLAGS set exits 1; the legacy gate passes. A container restart at about 19:57 UTC lost nothing
   (the tree, `.gate/` and the local ignore were intact; the saver was started again).

4b. **Change-set 3's plan, draft 1** (`ci-wasm/cs3-plan.md`, 20:35 UTC; 238 lines), from the three design notes. The
   designs come to about 5,700 lines against the card's 2,700, so the plan proposes two change-sets: 3a "Rules and
   boundaries" (about 2,050; stop line 4,100) and 3b "The loop closes" (about 3,700; stop line 7,400; parts move out
   at 4,800). Its questions for him: RNG-CRATE (recommended: our own ChaCha8), SIZE-3 (two change-sets),
   WASM-TEST-ROUTE (a test-only build of our own tools crate), CI-LOOP (a small CI pull request after 3a),
   PRESS-PACE (a second tap inside one tick lands a tick later).
4c. The advisor agent's review of draft 1 (handed back 20:57 UTC; `ci-wasm/advisor-cs3-plan.md`): **REVISE**, twelve
   conditions, no Critical (five High: the random-number question reused his code RNG-CRATE; the cut ladders took
   tests out; PM-0002-12 mislabelled and without its own test; 3a would be red in CI; the draft departures not
   asked). **Draft 2** (21:00 UTC, 350 lines, commit bf6ec3e) works all twelve in: RNG-SOURCE; ladders that move
   whole parts; `presses.spec.ts`; the development build makes the test module, E2 dropped, E1 once with every
   crate; DEPART-3; section 5.1 (which of the two designs wins on three symbols; the replay on the shipped file
   through one new read-only hash number); the follow-up table; the pass-mark table. Sizes now: 3a about 2,050, 3b
   about 3,800. Questions for him: RNG-SOURCE, SIZE-3, PRESS-PACE, DEPART-3; ruled by the advisor agent:
   WASM-TEST-ROUTE (a), CI-LOOP (a) for E1 only.
4d. The advisor agent's re-read of draft 2 (handed back 21:05 UTC): **APPROVE with six conditions** (two Medium,
   four Low; it does not review again for them). **Draft 3** (21:06 UTC) makes all six: the 10-minute run outside
   CI's specs and a possible CI limit raise before 3b; SIZE-3 (b) names everything that moves; the hash number's
   wording in DEPART-3 and how its 64 bits cross; the import list of the player's own Wasm; `picture.ts` gains the
   blend-fraction sentence; wording. **The plan may go to him** with its four questions (RNG-SOURCE, SIZE-3,
   PRESS-PACE, DEPART-3) and the two rulings.

4e. The plan of change-set 3 (draft 3) and its four questions were put to him in chat at about 21:07 UTC, with the
   three questions of #62 still open.

5. **Taken over by the project thread (2026-11-23, from 14:25 UTC)**, on his words of 14:25:03 UTC. The session's
   model read (14:28 UTC). His branch copied: `claude/project-thread-qx9woh` made in the worktree `.gate/build-wt`
   at d064e17 and pushed (between 14:36 and 14:40 UTC by the clock lines around it; the same commit as #62's head, so
   nothing new was published). The seven
   answers stated back to him with the CI text attached (14:28:25 UTC); his tap "Yes to all seven" (14:31:06 UTC).
   Asked, because his WRITE-MODE words need a mode this thread lacks: WRITE-THREAD; his yes (14:32:43 UTC). His
   tap LONG-RUNS (14:32:42 UTC). **The rehearsal on this container** (14:28 to 14:33 UTC, from the clock): the
   tool's archive fetched by the workflow's own curl line (final address on GitHub's release-asset host; size
   11860397; sha256 OK; the seven members; the program 8170784 bytes, sha256 OK); the proposed file rebuilt from
   `ci-edit.diff` on main's file: sha256 d1dbcb53...975c, and with the plant 12b90249...12ef, both as recorded;
   `cargo test --workspace --locked` green (34 tests); the workflow's new in-cut lines run as one script under the
   cut's bare variable list (`.gate/ci-wasm/incut-local.sh`, with `/opt/pw-browsers/chromium`, Chrome for Testing
   153.0.8010.12, standing in for the runner's Chrome): exit 0; both builds; the Wasm 28,987 bytes, sha256
   c4935acaa142ce1046cf30fa1f6f3edd7697834f32c7536159ce58972ed32c31; "Tests 7 passed"; the five type-check lines;
   "12 passed (1.1m)" with one worker.

## Current step
**Step 6, in the project thread (2026-11-23):** the takeover's records (his four messages into the notes and into
`docs/DECISIONS.md`: done, a9924c9), the first push of the thread's branch and the new draft pull request #63 that
replaces #62 (done), and the advisor agent's read of `ci-wasm/takeover.md` and head f1b958c (running since 14:43
UTC; notes `ci-wasm/advisor-takeover.md`). **On hold since 14:51 UTC: the CI write**, until (1) the advisor agent's
verdict is in and its findings are answered, and (2) he has answered the card WRITE-WHERE (see "His words that
govern", the entry of 14:32:43 UTC). Then "Next steps", step 1.

The first builder chat was **paused by his words of 2026-11-19, 21:44:09 UTC** (`ci-wasm/mustafa-2026-11-19-pause.md`,
by script): "Finish the current tasks that you're working on and then pause. I will let you know when you can
continue again". It stays paused; when its account is back (after 2026-11-24, 20:00 Amsterdam time) he is given a
block to paste there, saying that this thread took #62 and change-set 3 forward.

**Answered on 2026-11-23, 14:31:06 UTC (ANSWERS-7: every recommended answer below is his answer now).** As they were
put to him by the first builder chat:
- For pull request #62 (put 20:03 UTC): CHROME-SOURCE (recommended (a), the runner image's Chrome), CI-TEXT (one yes
  for five things: `ci-edit.diff` sha256 240c7db7...2673, `ci-plant-p-gh.diff` sha256 e4f21ab3...b1ac, P-BROWSER,
  the stop rule, the tool's second route), FIREFOX-TESTS (recommended (c)). With a yes on CI-TEXT the chat must be
  in Accept edits; the mode is read right before each of the three CI writes.
- For change-set 3's plan (put 21:07 UTC; `ci-wasm/cs3-plan.md`, draft 3): RNG-SOURCE (recommended (c), our own
  ChaCha8), SIZE-3 (recommended (b), 3a then 3b, with the two ladders), PRESS-PACE (recommended (a)), DEPART-3
  (recommended (a)). Told to him as the advisor agent's rulings: WASM-TEST-ROUTE (a), CI-LOOP (a) for E1 only; and
  for #62: PERM-ATT (a), PLANTS both, PYCACHE-NOW (b).

## Next steps (the project thread goes on; "#62" below means the new pull request that replaces it)
1. **The CI write** (his CI-TEXT (a) is in; his WRITE-THREAD yes was given on an incomplete answer, so it waits for
   his answer to WRITE-WHERE and for the advisor agent's read of `ci-wasm/takeover.md`): write
   `.github/workflows/toolchain.yml` from `.gate/ci-wasm/proposed/.github/workflows/toolchain.yml`, compare its sha256 (d1dbcb53...975c), commit that one
   file, push; **the first runner run under the stop rule** (plan section 9). If `.gate/` was lost with a container,
   rebuild the proposed file by applying `ci-wasm/ci-edit.diff` to main's file and check the sha256; the tool comes
   back by the workflow's own curl line and checks (step 5 above).
2. The two plants, one per push, each taken back in the next commit: P-GH (`ci-wasm/ci-plant-p-gh.diff`; the
   file's sha256 compared after its write and after its take-back; sha256 of the planted file 12b90249...12ef),
   then P-BROWSER (one render count 3 to 4 in `packages/e2e/tests/skeleton/render-counts.json`). After the last
   take-back the tree
   equals step 1's tree.
3. Review round 1 on the final diff (security-auditor, infra-engineer, test-engineer; at most two rounds), fixes;
   the tier-2 evidence (the kit's three test files' counts on main and on the head; the before/after table of the
   workflow's rule sentences); the verifier; the advisor agent on every pushed head.
4. docs-keeper (STATUS within 120 lines, ROADMAP, the map, the archive: plan section 7, item 4); his later messages
   into DECISIONS by `ci-wasm/decisions-3.mjs`; the report `docs/reports/2026-11-19-ci-wasm.md` with the workflow
   text kept apart from the docs; the checkpoint folded and deleted in the last commit; the new pull request marked
   ready; stop for his go (risk tier 2). **Ask him before merging and before closing #62** (the project's note of
   2026-11-23). Then the block for him to paste into the first builder chat when its account is back.
5. **Change-set 3, after the CI pull request is merged** (his four answers are in: ANSWERS-7): open its own
   checkpoint (per part: 3a first), move `ci-wasm/cs3-plan.md`, `cs3-context.md`, the three design notes and
   `advisor-cs3-plan.md` into it, and build 3a by the plan's section 4, with RNG-SOURCE (c), SIZE-3 (b),
   PRESS-PACE (a), DEPART-3 (a). He answered FIREFOX-TESTS (c): the Firefox CI pull request comes between the CI
   pull request and 3a, with its download and exact text to him first; ask him whether 3a may start beside it.
6. Later: change-set 4's plan prices SLICE-FEEL and answers lane 2's list on issue #59 row by row; PROJ-DRAW in
   change-set 9.

## Open findings
- **PYC-AGAIN (what went wrong, to tell him and to record in the report):** one compiled Python file,
  `.checkpoints/ci-wasm/__pycache__/rr-chacha-ref.cpython-313.pyc` (7,356 bytes; made 16:34 UTC when rust-reviewer's
  check script was run), was committed and pushed by the saver's autosave b7325df at 16:39:06 UTC. Found 17:33 UTC,
  removed from the notes 17:35 UTC; it stays in the branch's history (no history is rewritten). His words placed the
  `.gitignore` line "With change-set 3". Guard since 17:35 UTC: `.git/info/exclude` of this container lists
  `__pycache__/` and `*.pyc` (local, untracked; a new container loses it), and agents are told to run Python with
  `-B`. Question PYCACHE-NOW in the plan.
- None open on main. Carried as follow-ups: the archive's block for PR #61, and the observations in
`ci-wasm/browser-reports-reading.md` (BR-GAP is the one that is not explained: Brave's first run drew nothing for
about 150 s while the simulation ran and no pause was recorded; the pause on a tab switch works in Brave).

## Key numbers
- main: f5030a3. Change-set 2: 4,477 added lines of code (stop line 4,700).
- His machine, from his reports: Brave 1.100.58 (Chromium 158) and Firefox 160 on Windows, 2560x1440, NVIDIA GeForce
  GTX 980, about 180 frames a second drawn in both browsers; WebGPU starts in both; errors 0 in all eight reports.
- The tool's file and program: sizes and sha256 in step 3 above.

- CI on c934fc0 (today's workflow, no CI file changed; read 20:08 UTC): `gate` and `smoke` success for the push (run
  44603988667) and the pull request (44604056505); `toolchain` success for the push (44603989027) and the pull
  request (44604056424).
- The CI text as put to him at 20:03 UTC: 93 added lines, `ci-edit.diff` sha256
  240c7db726dee600b54d10b0252b33e527482ecf88ba41af849660d55e8d2673; P-GH's two lines, `ci-plant-p-gh.diff` sha256
  e4f21ab359bc9fd911c663a4fcbc892133195fefe44e88ad7ec51b50e3d0b1ac; the proposed file's sha256
  d1dbcb533460936e9f4f3ca2e97ed28c6d586638e52c00fa034e875f7a4e975c, with the plant
  12b9024966abc4907b27c8c2b4c6cdccfa56ceea3d27d94c060b9ab2ca1f12ef.

## Decisions taken
- BROWSER-SET (a), his words of 14:28:25 UTC. VISION-NOTE (a), his "a, yes" of 14:31:30 UTC.
- FIREFOX-WEBGL: he pasted the report (15:09:29 UTC), so the question needed no letter.
- The docs pass for #61's merge rides this pull request (his words of 16:08:04 UTC).
- 2026-11-23, in the project thread (his words: the files named under "His words that govern"): the takeover
  (14:25:03 UTC); ANSWERS-7 (14:31:06 UTC): CHROME-SOURCE (a), CI-TEXT (a), FIREFOX-TESTS (c), RNG-SOURCE (c),
  SIZE-3 (b), PRESS-PACE (a), DEPART-3 (a); LONG-RUNS: GitHub (14:32:42 UTC); WRITE-THREAD: yes (14:32:43 UTC),
  given on the thread's incomplete answer and put to him again as WRITE-WHERE (14:51:33 UTC): **open**.
- Still open, to come with its own text: the Firefox CI pull request's download and exact text (FIREFOX-TESTS (c)).

## Running work
- **The project thread (2026-11-23):** the saver runs since 14:44:06 UTC (background task b60454kri: every 10
  minutes for 110 minutes; `node .claude/scripts/checkpoint.mjs watch ci-wasm --trailer "..."`, from the worktree
  `.gate/build-wt`), started with the first agent: the advisor agent "takeover" (Opus 5.5, effort max, in the
  background since 14:43 UTC; notes `ci-wasm/advisor-takeover.md`; a restart gives it the same task). The
  thread's scratch outside git, all under the worktree: `.gate/tools/wasm-bindgen-0.2.130/` (the tool) and
  `.gate/tools/dl/` (its archive), `.gate/ci-wasm/proposed/` and `proposed-plant/` (the CI file's scratch copies),
  `.gate/ci-wasm/incut-local.sh` and `.out` (the rehearsal), `.gate/dist-production`, `.gate/dist-development`.
  The lines below are the first builder chat's, of 2026-11-19, kept as its record; its container's scratch is gone.
- The first builder chat's saver (bgdn004k8, started 19:57:35 UTC) was stopped with TaskStop at 21:44 UTC for the
  pause.
- The planted defects, subset: done (20:00:29 to 20:08:18 UTC): ten of ten caught on the edited specs, each at the
  line that names it (1, 2, 7, 7b, 8, 8b, 11, 14, 14b, 15), and the control on the clean tree passes; output
  `ci-wasm/plants-subset-output.txt`, script `ci-wasm/plants-subset.py`. Its worktree `.gate/ci-wasm-plants` was
  removed with `git worktree remove` (not forced).
- Agents for change-set 3's plan (both Opus 5.5 at effort max, in the background, started 20:02 UTC; they build
  nothing and run no browser test): netcode-architect, part "cs3": **done** (notes
  `ci-wasm/netcode-architect-cs3.md`, ending DONE: the symbols crate by crate, tests T1 to T10, its part about 3,090
  lines so the change-set is likely 4,000 to 4,300 against the card's 2,700; its two questions for him: SIZE-3 and
  PRESS-PACE; no new dependency); engine-architect, part "cs3": **done** (its half about 2,800 lines: the
  boundaries, the page's key diagram, the rule tests through one test-only Wasm module, the build order in eleven
  steps; its questions: WASM-TEST-ROUTE, CI-LOOP, SIZE-3; both halves together are about 5,900 lines, above twice
  the card's 2,700, so the plan must offer a split; notes
  `ci-wasm/engine-architect-cs3.md`: the boundaries, the page's key diagram, the rule tests, the build order). Both
  were given `ci-wasm/cs3-context.md`. A restart gives each the same task again.
- No agent is running (since 21:05 UTC). Done, besides those below: the advisor agent "cs3-plan" with its re-read.
- Done: infra-engineer "ci-plan", test-engineer "followups", rust-reviewer "cs3-rng",
  security-auditor "ci-text", the advisor agent "plan" (with its re-read): every notes file ends with DONE.
- Scratch outside git: `.gate/ci-wasm/` (`hist/`, `proposed/`, `proposed-plant/`, `docs-try/`, `security/`, `mutant/`,
  the dry runs' outputs, `dist-*` builds), `.gate/dist-production` and `.gate/dist-development` (the dry run's
  builds), `.gate/cs2-first-page/merge/` (his messages as first saved, his Firefox screenshot),
  `.gate/cs2-first-page/send/` (the zip he tested), `.gate/tools/wasm-bindgen-0.2.130/` (the tool).
