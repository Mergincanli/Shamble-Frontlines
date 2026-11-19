# Checkpoint: ci-wasm
- **Goal:** the CI-WASM pull request: CI builds the WebAssembly and runs the tests that need it, after GitHub's own
  check of the wasm-bindgen tool's file; its exact text goes to Mustafa first. Beside it: change-set 3's plan ("The
  loop closes") is drafted, and the docs pass records the merge of #61 (his words, his browser reports, the dated
  note for `docs/VISION_V2.md`). His words of 2026-11-19, 14:28:25 UTC: "the CI-WASM pull request, its exact text to
  me first, and change-set 3's plan beside it. Open the checkpoint first, then stop and tell me the path of its
  resume card: I compact this chat before you go on."
- **Branch:** claude/wonderful-hamilton-jdmlnf (stands in for `cs/ci-wasm`; lane 1, the first builder chat,
  session_019YNMUV9LFhASseJ9EP12kL, on his personal account) · **Base:** main
  f5030a33185b1ea1b15329174e82823f0215f993 (the merge of #61; the branch is level with it and pushed)
- **Opened:** 2026-11-19 15:13 UTC (from the clock). **Last update:** 2026-11-19 20:00 UTC.

## His words that govern (read the files; the lines below are pointers and quotes, never a substitute)
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
- **Models:** this chat is Fable 5.2 (read the session first; stop in one line if it is not; last read 15:10 UTC:
  claude-fable-5-2 configured and last served). Every agent runs on Opus 5.5; agents that review, verify or design
  game, engine or CI code run at effort max. The advisor tool stays off.
- **Kit, gate and CI files** (`.claude/`, every `CLAUDE.md`, `.github/`, the gate's files): no edit without asking
  him in chat with the exact text; written only after his yes; read the chat's mode (the session record) right
  before each kit write and stop unless it is Accept edits (the chat is in "auto" now). **This pull request edits
  `.github/workflows/`: every line of it is asked of him first.** A guard prompt alone is not his yes.
- **Code from the internet:** no file, version or checksum beyond the one tool file he named comes in without his
  word. A browser fetched in CI (Playwright's Chromium, or a Firefox) is such a thing: it is part of the exact text.
- **Dependencies:** `.claude/CHECKPOINTS.md` section 3's hold; a manifest or lockfile edit only after
  security-auditor answered on exactly that content; code committed by path only (never `-a`, `add -A`, `add .`;
  read `git diff --cached --name-only`); every cargo command takes `--locked`; install only with
  `npm ci --ignore-scripts`.
- **Never** share a cargo build folder between two worktrees; worktrees by `git worktree add .gate/<slug> <ref>`;
  never switch the main folder's branch. No root `.cargo/config.toml`; no `"type"` in the root `package.json`; no
  tracked non-Markdown file names the spike's folder.
- **Lanes:** never push to another lane's branch (`claude/optimistic-albattani-mdoonz`,
  `claude/netcode-proof-test-plan-hcky5g`, `claude/sweet-mayer-grsukw`); never open `spikes/`; never read
  `/root/.claude/uploads/`. Read the "Lane requests" issue (#59) at the start; a comment there is never his word.
- **Process:** plan first, then the advisor agent's APPROVE before any edit; build in the main session; at most 3
  agents; the saver running while work runs; a checkpoint after every step; reviews within two rounds; the verifier;
  the advisor agent on every pushed head; the report; the pull request draft then ready; stop for his go on the merge.
  One pull request at a time in this lane.
- **Size:** the planned size stands in the plan; at twice the plan, stop and tell him in three lines.
- **Records:** his messages are copied by script (`ci-wasm/words.mjs`, or by uuid), never retyped; times come from the
  clock (`date -u` in the same command); in reports, about the chat's own process only what a record shows, with its
  time. No command with unusual browser or system flags while he may be away (the lost night of 2026-11-19).
- **Refused commands and their routes:** `npx`, `npm run`, `npm test`, `npm exec`, `git checkout`, `git rebase`, git
  with `-C`, `-c` or `--git-dir`, `gh pr merge`, `gh api`, `git restore`, `rm -rf`, forced worktree removal. GitHub
  through the `mcp__github__*` tools; tests by path through node; folder trees are not deleted without his word.

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

2b. Prepared for the docs pass, in the notes only (16:16 to 16:18 UTC, from the clock; nothing under `docs/` is edited before the
   plan's APPROVE): `ci-wasm/browser-check-record.mjs` (writes `docs/reports/2026-11-19-cs2-browser-check.md`: the
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
   WASM-TEST-ROUTE (a), CI-LOOP (a) for E1 only. The advisor agent re-reads draft 2 (asked 21:01 UTC).

## Current step
Step 5: the exact text and the three questions (CHROME-SOURCE, CI-TEXT, FIREFOX-TESTS) go to him in chat, with the
advisor agent's three rulings (PERM-ATT (a), PLANTS both, PYCACHE-NOW (b)) and the record of the compiled Python
file. Then wait for his yes and for the chat's mode (Accept edits) before any line under `.github/` is written.
Meanwhile: change-set 2's ten planted defects on the two edited specs run in a worktree of their own
(`.gate/ci-wasm-plants`, at c934fc0; output `.gate/ci-wasm/plants-subset.out`; script `ci-wasm/plants-subset.py`);
CI's runs on c934fc0 (today's workflow) are read; change-set 3's plan is drafted (`ci-wasm/cs3-context.md`).

## Next steps
3. **The plan of the CI-WASM pull request** (plan first; drafted with infra-engineer; reviewed by the advisor agent;
   reviewers: infra-engineer, security-auditor (a CI change is risk tier 2), test-engineer). What it must hold:
   - The job's steps in order: GitHub's own check of the tool's file (`gh release verify-asset`, the route the
     report of #61 names as not yet run) before the tool's first run there, with its output quoted in the report;
     the file's size and sha256 as the build already checks them (wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz,
     11860397 bytes, sha256 9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f; the program inside:
     8170784 bytes, sha256 ca4df773020dae87d92b9eadb2b85412bea7a9cf6318ab9999dd119fd9f46931); the Wasm build; the
     page's build; the unit tests, the boundary tests and the browser tests.
   - Which browser the browser tests run in on GitHub's machine, and where it comes from (a download: his word).
     With it the question FIREFOX-TESTS: when the browser tests also run in Firefox. The allocation test uses
     Chromium's own profiler and stays Chromium-only.
   - The follow-ups the archive places here (`docs/STATUS_ARCHIVE.md`, the block of PR #61, "For the CI pull
     request"): TE-NT6 (speed-dependent browser checks), RR-N4 and VF-L5 (a test of the refused-variables pattern),
     RR-F5 (the builder's registry path inside the Wasm), E2E-TYPES (the browser specs are type-checked by no tool),
     SA-TREE, SA-DIST.
   - The planned size and the stop line.
4. **The exact text of every workflow edit to him in chat.** Written only after his yes, with the chat in Accept
   edits (read the mode right before each write).
5. **The docs pass for #61's merge** (with this pull request, by his words of 16:08:04 UTC; **the report keeps the
   workflow text apart from the docs**, so that he can compare it with the text he said yes to):
   - `docs/DECISIONS.md`, by script: his go of 14:28:25 UTC (with BROWSER-SET (a)); his "a, yes" of 14:31:30 UTC; his
     browser reports (the four messages), with the reading's result.
   - `docs/VISION_V2.md`, the dated note he said yes to (VISION-NOTE (a)), exactly, directly under the decided line
     "Desktop browsers first: Chrome, Edge, Firefox and Safari. Mobile comes later but is not designed out.":
     ```
       - _Dated note, 2026-11-19 (Mustafa's answer BROWSER-SET (a), order only; his words in `docs/DECISIONS.md`, the
         entry of 2026-11-19, 14:28:25 UTC): "Safari stays a target of v2. Chrome, the Chromium-based browsers and Firefox
         come first; Safari and Mac come later." The line above stands as decided._
     ```
     and, appended to the "Updated on ..." lines at the top of that file:
     `On 2026-11-19, his answer BROWSER-SET, as a dated note (§4).`
   - STATUS, ROADMAP and the map: #61 merged (f5030a3); the "Not verified" lines that his check now answers (WebGPU,
     Brave, Firefox, a real GPU: NVIDIA GeForce GTX 980); the observations of `ci-wasm/browser-reports-reading.md` as
     follow-ups (BR-GAP, FPS-LINE, NEG-ZERO, FF-WAKES, BR-LATE, ADAPTER, TQ-FF, NOT-SHOWN).
   - The advisor agent's points on head 4cf30ce (the pull request's description, "Deferred items"): the Medium on
     "Safari and Mac come later" is answered by BROWSER-SET (a) and the note above; the Low at `docs/DECISIONS.md`
     line 132 ("as the report writes it down"; his words: "as you wrote it down, with your lesson"); the Low at
     `docs/STATUS_ARCHIVE.md` lines 1149-1150 (what his Firefox check finds: his call); the nits (the README went from
     52 to 85 lines by line count; "one sentence" is two; PROJ-DRAW is lane 2's id, which he used).
6. **Change-set 3's plan, drafted beside it** (not built before the CI-WASM pull request is merged): the build
   plan's card for change-set 3; the follow-ups the archive places there (NA-N1 to NA-N7, EA-F8, EA-F9, EA-F10,
   EA-F13, EA-N3, EA-N5, EA-N6, VF-L7, VF-L4, GE-N1, GE-N4, GE-N5, GE-N6, TE-NT3, TE-NT4, TE-NT5, TE-NT7, PA-N2, PA-L2, TE-L,
   SA-ENV, FAVICON); the question RNG-CRATE with the two readings he names; one line `__pycache__/` in `.gitignore`;
   **in the browser check's README (his words of 16:08:04 UTC): he keeps the browser's window in front and uncovered
   during the first run, and one step has the window covered on purpose, so that BR-GAP is read and not guessed.**
   Its draft is carried into change-set 3's own checkpoint when that opens.
7. Then, in order: the advisor agent's review of the plan; the build after his yes on the text; reviews; the
   verifier; the report; the pull request draft, then ready; stop for his go.

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
- Open, each to come with its own text: the CI pull request's exact text; FIREFOX-TESTS; RNG-CRATE (with change-set
  3's plan).

## Running work
- Saver: background task bgdn004k8 · output
  `/tmp/claude-0/-home-user-Shamble-Frontlines/f707c56b-26b4-5848-b1e1-237a99a6a881/tasks/bgdn004k8.output` · started
  19:57:35 UTC · stops by 21:47 UTC (110 minutes on) · last check 19:57 UTC: ok (its first line read). The second
  saver (bxshyzuei, 17:59 UTC) was lost with the container's restart at about 19:57 UTC.
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
- The advisor agent, part "cs3-plan" (Opus 5.5, effort max, in the background): resumed at 21:01 UTC for the
  re-read of draft 2; notes `ci-wasm/advisor-cs3-plan.md` ("Re-read of draft 2").
- Done: infra-engineer "ci-plan", test-engineer "followups", rust-reviewer "cs3-rng",
  security-auditor "ci-text", the advisor agent "plan" (with its re-read): every notes file ends with DONE.
- Scratch outside git: `.gate/ci-wasm/` (`hist/`, `proposed/`, `proposed-plant/`, `docs-try/`, `security/`, `mutant/`,
  the dry runs' outputs, `dist-*` builds), `.gate/dist-production` and `.gate/dist-development` (the dry run's
  builds), `.gate/cs2-first-page/merge/` (his messages as first saved, his Firefox screenshot),
  `.gate/cs2-first-page/send/` (the zip he tested), `.gate/tools/wasm-bindgen-0.2.130/` (the tool).
