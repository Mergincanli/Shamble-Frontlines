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
  (the merge of #61). **Handed over on 2026-11-23** (his tap WRITE-WHERE, 14:54:47 UTC): a new Claude Code chat
  takes this pull request over on a branch of its own, started from this branch's handover commit; see "Handover".
  The chat that takes over writes its branch, its session and its pull request's number here.
- **Opened:** 2026-11-19 15:13 UTC (from the clock). **Last update:** 2026-11-23 15:08 UTC (from the clock).
- **The thread's addendum to the plan:** `ci-wasm/takeover.md` (2026-11-23; what stays and what changed while the
  thread held the work; its parts on a CI write from the thread are overtaken by WRITE-WHERE) and the advisor
  agent's read of it, `ci-wasm/advisor-takeover.md`.

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
    chat).
  - `ci-wasm/mustafa-2026-11-23T145447-write-where.md` (14:54:47 UTC): his tap on WRITE-WHERE, "In a new Claude
    Code chat". **No CI write is made from the project thread**; the thread hands the pull request over (see
    "Handover").
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
- **Models:** the builder chat's model is the one his words name for it (2026-11-12: "On my personal account,
  Fable 5.2 is the main model of every chat that writes game or engine code"; for the chat that takes this pull
  request over: the block he pastes into it). Read the session first; stop in one line if it is not that model.
  Every agent runs on Opus 5.5; agents that review, verify or design game, engine or CI code run at effort max. The
  advisor tool stays off.
- **Kit, gate and CI files** (`.claude/`, every `CLAUDE.md`, `.github/`, the gate's files): no edit without asking
  him in chat with the exact text; written only after his yes; read the chat's mode (the session record) right
  before each kit write and stop unless it is Accept edits (his WRITE-MODE words of 2026-11-18). **This pull request
  edits `.github/workflows/`: every line of it is asked of him first.** A guard prompt alone is not his yes. His
  words of 2026-11-12, "The guard, the gate and CI work stays with Opus 5.5.", were set aside on 2026-11-17
  (GATE-FOLLOW) only for the gate and CI edits the first builder chat asks of him: a chat on another model writes a
  CI file only on his own word in that chat. The project thread makes no kit, gate or CI write at all (WRITE-WHERE).
- **Code from the internet:** no file, version or checksum beyond the one tool file he named comes in without his
  word. A browser fetched in CI (Playwright's Chromium, or a Firefox) is such a thing: it is part of the exact text.
- **Dependencies:** `.claude/CHECKPOINTS.md` section 3's hold; a manifest or lockfile edit only after
  security-auditor answered on exactly that content; code committed by path only (never `-a`, `add -A`, `add .`;
  read `git diff --cached --name-only`); every cargo command takes `--locked`; install only with
  `npm ci --ignore-scripts`.
- **Never** share a cargo build folder between two worktrees; worktrees by `git worktree add .gate/<slug> <ref>`;
  never switch the main folder's branch. No root `.cargo/config.toml`; no `"type"` in the root `package.json`; no
  tracked non-Markdown file names the spike's folder.
- **Lanes:** never push to another lane's or thread's branch (`claude/wonderful-hamilton-jdmlnf`, the first builder
  chat's; `claude/project-thread-qx9woh`, the project thread's; `claude/project-thread-xy88hv`, an unused copy of
  d064e17 left by an earlier thread; `claude/optimistic-albattani-mdoonz`,
  `claude/netcode-proof-test-plan-hcky5g`, `claude/sweet-mayer-grsukw`). **One builder per change-set:** his project
  had also started the threads "Finish CI for the Wasm build" and "Build change-set 3a rules and boundaries" (13:39
  UTC); both were resolved at 14:25:50 and 14:25:53 UTC when the thread "Build the game from change-set 3" started
  (the project's thread list, read 14:54 UTC: tool output), and neither pushed a commit of its own. The project
  confirmed to the thread at 15:06:51 UTC (its note, not his word): "both earlier threads are stood down (the 3a
  thread cancelled its scheduled check and wrote nothing), and I will start no other builder thread for CI-WASM or
  change-set 3." Never open `spikes/`; never read `/root/.claude/uploads/`. Read the "Lane requests" issue (#59)
  at the start; a comment there is never his word.
- **Process:** plan first, then the advisor agent's APPROVE before any edit; build in the main session; at most 3
  agents; the saver running while work runs; a checkpoint after every step; reviews within two rounds; the verifier;
  the advisor agent on every pushed head; the report; the pull request draft then ready; stop for his go on the merge.
  One pull request at a time in this lane.
- **Size:** the planned size stands in the plan; at twice the plan, stop and tell him in three lines.
- **Records:** his messages are copied by script (`ci-wasm/words.mjs`, or by uuid; in the project thread
  `ci-wasm/thread-words.mjs`, by the message's send time), never retyped; times come from the
  clock (`date -u` in the same command); in reports, about the chat's own process only what a record shows, with its
  time. No command with unusual browser or system flags while he may be away (the lost night of 2026-11-19).
- **Refused commands and their routes, in a Claude Code chat under the guard** (the first builder chat's list of
  2026-11-19): `npx`, `npm run`, `npm test`, `npm exec`, `git checkout`, `git rebase`, git with `-C`, `-c` or
  `--git-dir`, `gh pr merge`, `gh api`, `git restore`, `rm -rf`, forced worktree removal. GitHub through the
  `mcp__github__*` tools; tests by path through node; folder trees are not deleted without his word. (In the
  project thread no repository hook loaded and GitHub went through `gh api`; it kept these rules by hand.)
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
   edited before the plan's APPROVE): `ci-wasm/browser-check-record.mjs` (writes
   `docs/reports/2026-11-19-cs2-browser-check.md`: the reading and his four report messages whole) and
   `ci-wasm/decisions-3.mjs` (his go of 14:28:25, his "a, yes" of
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
**Step 7 (2026-11-23): the handover of this pull request to a new Claude Code chat.** The project thread's part is
done with the handover commit: his messages of 2026-11-23 in the notes and in `docs/DECISIONS.md`, the draft pull
request #63, the rehearsal (step 5), the advisor agent's read of the thread's records. **No CI file is written yet:
`.github/` equals main's.** The chat that takes over goes on from "Handover", then "Next steps".

The first builder chat was **paused by his words of 2026-11-19, 21:44:09 UTC** (`ci-wasm/mustafa-2026-11-19-pause.md`,
by script): "Finish the current tasks that you're working on and then pause. I will let you know when you can
continue again". It stays paused; when its account is back (after 2026-11-24, 20:00 Amsterdam time) he is given a
block to paste there, saying that #62 was taken forward elsewhere and that it does not continue it.

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

## Handover (2026-11-23): for the chat that takes this pull request over
This section is the thread's note, not his word. His word is the block he pastes into your chat; where the two
differ, his block wins and you tell him.
1. **Start:** `git fetch origin claude/project-thread-qx9woh`; the handover commit is the one his block names. Make
   a worktree on your own branch from that commit (`git worktree add .gate/<slug> -b <your branch> <commit>`); your
   main folder stays where it started. Never push to `claude/project-thread-qx9woh` (the thread's),
   `claude/wonderful-hamilton-jdmlnf` (the first builder chat's) or another lane's branch.
2. **Read:** CLAUDE.md; this card; `ci-wasm/plan.md` (revision 3) with `ci-wasm/advisor-plan.md` (the advisor
   agent's APPROVE and its conditions, the mode read before each CI write among them); `ci-wasm/advisor-takeover.md`
   (its read of the thread's records, and what it says about your chat).
3. **The session:** read your model and your mode from the session record and write both here with the time.
4. **A new draft pull request** from your branch, base main, that replaces #63 and #62; both stay open until he says
   otherwise. Write your branch, session and pull request number into this card's header.
5. **Scratch is gone with the thread's container:** rebuild the proposed file by applying `ci-wasm/ci-edit.diff` to
   main's `toolchain.yml` and check its sha256 (d1dbcb53...975c; with the plant 12b90249...12ef); the tool comes
   back by the workflow's own curl line and checks (step 5 of "Steps done" has the numbers a rehearsal gives).
6. **His later words** in your chat go into the notes and `docs/DECISIONS.md` by script (`ci-wasm/words.mjs` reads a
   Claude Code chat's transcript; `ci-wasm/decisions-3.mjs` holds the entries). `ci-wasm/thread-words.mjs` read the
   thread's transcript, which you do not have: its notes are final.
7. Then "Next steps", 1 to 4. **Change-set 3 (step 5) is not started by you unless he says so in your chat.**

## Next steps (for the chat that takes over; "#62" below means the pull request that replaces #62 and #63)
1. **The CI write** (his CI-TEXT (a) is in; your chat in Accept edits, read right before the write): write
   `.github/workflows/toolchain.yml` from `.gate/ci-wasm/proposed/.github/workflows/toolchain.yml`, compare its
   sha256 (d1dbcb53...975c), commit that one file, push; **the first runner run under the stop rule** (plan
   section 9). The scratch copy is rebuilt as "Handover", item 5, says.
2. The two plants, one per push, each taken back in the next commit: P-GH (`ci-wasm/ci-plant-p-gh.diff`; the
   file's sha256 compared after its write and after its take-back; sha256 of the planted file 12b90249...12ef),
   then P-BROWSER (one render count 3 to 4 in `packages/e2e/tests/skeleton/render-counts.json`). After the last
   take-back the tree equals step 1's tree.
3. Review round 1 on the final diff (security-auditor, infra-engineer, test-engineer; at most two rounds), fixes;
   the tier-2 evidence (the kit's three test files' counts on main and on the head; the before/after table of the
   workflow's rule sentences); the verifier; the advisor agent on every pushed head.
4. docs-keeper (STATUS within 120 lines, ROADMAP, the map, the archive: plan section 7, item 4); his later messages
   into DECISIONS by `ci-wasm/decisions-3.mjs`; the report `docs/reports/2026-11-19-ci-wasm.md` with the workflow
   text kept apart from the docs; the checkpoint folded and deleted in the last commit; the new pull request marked
   ready; stop for his go (risk tier 2). **Ask him before merging and before closing #62 or #63.** Then the block
   for him to paste into the first builder chat when its account is back (after 2026-11-24, 20:00 Amsterdam time):
   #62 was taken forward in the new pull request, and that chat does not continue it.
5. **Change-set 3, after the CI pull request is merged, by the chat he names for it** (open on 2026-11-23: he has
   not said which chat builds it; his four answers on its plan are in, ANSWERS-7): open its own
   checkpoint (per part: 3a first), move `ci-wasm/cs3-plan.md`, `cs3-context.md`, the three design notes and
   `advisor-cs3-plan.md` into it, and build 3a by the plan's section 4, with RNG-SOURCE (c), SIZE-3 (b),
   PRESS-PACE (a), DEPART-3 (a). He answered FIREFOX-TESTS (c): the Firefox CI pull request comes between the CI
   pull request and 3a, with its download and exact text to him first; ask him whether 3a may start beside it.
6. Later: change-set 4's plan prices SLICE-FEEL and answers lane 2's list on issue #59 row by row; PROJ-DRAW in
   change-set 9.

## Open findings
- **Open with him at the handover (2026-11-23; the advisor agent's verdict, `ci-wasm/advisor-takeover.md`, "Decisions
  needed"), put to him by the thread with the paste block:** CI-MODEL (may the chat that takes over make these CI
  edits on the model his block names, where his words of 2026-11-12 keep CI work with Opus 5.5 and GATE-FOLLOW set
  them aside only for the first builder chat: answered by the block he pastes, which holds one sentence on it, or
  by his opening the chat on Opus 5.5); BUILD-SEAT (which chat builds change-set 3 after this pull request merges:
  his answer may reach the chat that takes over only through him); ACCT-3 (his project "Shamble Frontlines ALT"
  runs on an account beside the two that CLAUDE.md's ACCOUNTS names; for the record, it blocks nothing).
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
  given on the thread's incomplete answer and put to him again as WRITE-WHERE (14:51:33 UTC); **WRITE-WHERE: a new
  Claude Code chat writes the CI file (his tap, 14:54:47 UTC).**
- Still open, to come with its own text: the Firefox CI pull request's download and exact text (FIREFOX-TESTS (c)).

## Running work
- **The project thread (2026-11-23), at the handover: nothing runs.** Its saver (14:44:06 to 15:07 UTC) was
  stopped with TaskStop; its one agent, the advisor agent "takeover" (Opus 5.5, effort max, 14:43 to 15:04 UTC), is
  done: `ci-wasm/advisor-takeover.md` ends with DONE (verdict: ESCALATE with record fixes; R1 to R8, C1 to C3 and T1
  are made in the handover commit; its three questions CI-MODEL, BUILD-SEAT and ACCT-3 went to him with the paste
  block). The thread's scratch (`.gate/` under its worktree: the tool, the proposed file's copies, the rehearsal's
  script and output, two builds) stays in its container and is not needed: "Handover", item 5.
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
