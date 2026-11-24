# Checkpoint: cs3a-rules-boundaries
- **Goal:** change-set 3a, "Rules and boundaries": the first half of the card's change-set 3 (his SIZE-3 (b)). The
  follow-ups that tidy the boundary, the scan test, our own ChaCha8 with its vectors, the float vectors, the panic
  hook, the test-only WebAssembly module, the build's overflow-override and feature checks. Almost nothing new to
  see. The plan: `cs3a-rules-boundaries/cs3-plan.md` (draft 3), section 4, steps 1 to 8.
- **Who builds:** the chat Mustafa calls Builder V3 (session_01WLuqnvpAh9XTitx8ehKJcQ, a Claude Code chat under the
  repository's guard), by his taps BUILD-SEAT and his words of 2026-11-23, 21:24:59 UTC. Its model is Fable 5.2 by
  his block of 2026-11-23, 16:30:46 UTC: read the session's model first (the session record) and stop in one line
  if it is another. Every agent runs on Opus 5.5 (`model: "opus"`); agents that review, verify or design run at
  effort max; docs-keeper at its own file's effort.
- **Branch on GitHub: `claude/loving-hopper-6p0mg2`** (stands in for `cs/3a-rules-boundaries`). **Worktree:
  `/home/user/Shamble-Frontlines/.gate/ci-wasm-wt`** (the folder keeps its name from the last change-set), local
  branch `cs/3a-rules-boundaries` (its upstream reads origin/main: never a plain push). **Every push:**
  `git push origin cs/3a-rules-boundaries:claude/loving-hopper-6p0mg2`, alone on its command line. The main folder
  `/home/user/Shamble-Frontlines` holds the local name `claude/loving-hopper-6p0mg2` at f5030a3 and is never
  switched (his block: "Do not switch your main folder."). **Base:** main
  1ec6f6c3852d2922989f299bfc053c403d0e52d3 (the merge of #64).
- **Opened:** 2026-11-24 10:34 UTC (from the clock). **Last update:** 2026-11-24 11:36 UTC (from the clock).

## His words that govern (read them where they stand; these lines are pointers, never a substitute)
- **In `docs/DECISIONS.md`, entries of 2026-11-23 (newest first):** 21:24:59 UTC (FIREFOX-TESTS (b): Firefox in CI
  joins the small CI pull request after 3a, none between #64 and 3a; 3a starts when #64 is merged, no second ask;
  above 40 % of context the chat tells him first; three items for 3b come from his Advisor before 3b's build and
  "do not touch 3a"); 17:15:07 UTC (his mode rule and GH-TOOLS (a), both given for #64); 16:30:46 UTC (his block:
  point 9, never push to another lane's or thread's branch, a merge only on his go for the approved head; point 12,
  less paper, his words by script, each with the question it answers, the chat's own messages are not copied into
  files); 14:31:06 UTC (ANSWERS-7: **RNG-SOURCE (c)** our own ChaCha8 checked against the published vectors;
  **SIZE-3 (b)** 3a then 3b, with the plan's two ladders; **PRESS-PACE (a)**; **DEPART-3 (a)**; the advisor agent's
  rulings stand: WASM-TEST-ROUTE (a), CI-LOOP (a) for E1 only); 14:32:42 UTC (LONG-RUNS: a full suite or a long
  build runs on GitHub from the pushed branch).
- **Older, still binding:** 2026-11-19, 14:28:25 UTC ("With change-set 3, one line in .gitignore for __pycache__/");
  2026-11-19, 13:36:37 UTC ("I do not take tests out to fit a number"); 2026-11-19, 16:08:04 UTC (for change-set
  3's browser check the README asks him to keep the window in front and uncovered during the first run and has one
  step with the window covered on purpose); 2026-11-18, 22:34:37 UTC (TOOL-SOURCE: every other file, version or
  checksum comes back to him); 2026-11-18, 20:20:09 UTC (WRITE-MODE: the mode is read right before each kit write);
  2026-11-14 (the size rule: at twice the plan, stop and tell him in three lines); 2026-11-12 ("The guard, the gate
  and CI work stays with Opus 5.5."; his set-aside in the block's point 6 was for the CI edits of #64 only).
- **Not yet in DECISIONS (by script in this folder, each with what it answers; they go in with 3a's docs pass, by a
  script made from `decisions-3.mjs` as it stands at 5911b56):**
  - `cs3a-rules-boundaries/mustafa-2026-11-23T234410-go-64.md`: his go on #64 for 8aadfb0; **GH-NEXT (a) for
    change-set 3a, with one change: the chat puts each pull request's title and text in the notes folder, his main
    Advisor opens the draft and sets them, and he presses Ready and Merge**; (c), a kit change, is planned with the
    gate step, not now; CLOSE-OLD (a); the order of work after the merge, ending: "Build nothing before I have
    compacted this chat."
  - `cs3a-rules-boundaries/mustafa-2026-11-23T235157-ready-then-merge.md` and
    `cs3a-rules-boundaries/mustafa-2026-11-24T101615-merged.md`: #64 was marked ready and merged by his main Advisor
    on his tap, with the expected head 8aadfb09640c9d37a3e6ad8be078165f5074f399; the merge commit's message records
    the tap. (At 23:15:41 UTC on 2026-11-23 the platform delivered his message of 21:24:59 UTC a second time, byte
    for byte: nothing new.)

## Standing rules (from his words and the repository's rules; when in doubt read DECISIONS and CLAUDE.md)
- **Process:** the plan is approved (the advisor agent, 2026-11-19: APPROVE with six conditions, all made in draft
  3: `cs3a-rules-boundaries/advisor-cs3-plan.md`). Build in the main session; at most 3 agents at a time, each with
  its notes in this folder, none commits; the gate after every step; reviews within two rounds; the verifier until
  PASS; the advisor agent on every pushed head that changes more than notes; docs-keeper; the report; then his go.
- **Dependencies (the hold, plan section 3 item 4):** one batch of manifest changes, uncommitted and unbuilt until
  security-auditor has answered on exactly that content; the lockfile by the resolve-only commands; every other
  cargo command with `--locked`; `npm ci --ignore-scripts` only; code committed by path only (never `-a`, `add -A`,
  `add .`; read `git diff --cached --name-only`). No root `.cargo/config.toml`; no `"type"` in the root
  `package.json`; no outside package with RNG-SOURCE (c).
- **Kit, gate and CI files:** none is planned in 3a (step 7: the development build makes the test module, so CI
  needs no new line). Any such edit is asked of him first with its exact text, written only after his yes, with the
  mode read right before it; the guard asks him at the write. For a CI edit by this chat after #64 his words of
  2026-11-12 stand until he says otherwise (ask before E1's CI pull request, which now also carries Firefox).
- **GitHub:** this chat reads (`gh api repos/Mergincanli/Shamble-Frontlines/...`, GET only) and pushes; it cannot
  open, edit, mark ready, comment on or merge a pull request (the platform refuses `gh pr`, the guard keeps `gh api`
  read-only). By GH-NEXT (a): the title and text go into this folder (`pr-title.txt`, `pr-body.md`), his main
  Advisor opens the draft and sets them, he presses Ready and Merge. For the merge, as for #64: the advisor agent
  approves the full SHA; gate, smoke and toolchain green on it; nothing pushed after; the chat gives him the SHA.
- **Saves:** no saver script here (it pushes to the local branch's own name; the advisor agent ruled hand saves
  acceptable for #64): commit `.checkpoints/` by path and push by hand after every step, before and after every
  agent run, at least every 30 minutes; notes-only commits carry the skip marker; the last commit of a head whose
  runs are evidence carries none; no push while a run that is evidence is still going (a push cancels it).
- **Commands that are refused, and their routes:** a `git push` chained with anything that holds a `+` (push
  alone); git with `-C` or `-c`; `git checkout`, `restore`, `rebase`, branch renames, forced worktree removal;
  `npx`, `npm run`, `npm test` (a test by its path through node); `gh pr merge`, `gh api` writes; `rm -rf`. Python
  with `-B`; literal paths in a shell write that the guard must read.
- **CI:** a red `toolchain` is read and brought to him, never re-run into green; no bound is lowered blind; on an
  image or Chrome change read the allocation test's byte windows and the default backend first (TE-WATCH).
  `toolchain` is not a required check: read it by hand before every merge.
- **Lanes:** never push to `claude/wonderful-hamilton-jdmlnf`, `claude/project-thread-qx9woh`,
  `claude/project-thread-xy88hv`, `claude/optimistic-albattani-mdoonz`, `claude/netcode-proof-test-plan-hcky5g`,
  `claude/sweet-mayer-grsukw`. Never open `spikes/`. Read the "Lane requests" issue (#59) at the start; a comment
  there is never his word.
- **Records:** his messages by `cs3a-rules-boundaries/chat-words.mjs` (it reads this chat's transcript,
  `/root/.claude/projects/-home-user-Shamble-Frontlines/f3c2a423-4254-5158-8151-83cb434f56ec.jsonl`; typed and
  queued messages), each with what it answers; times from the clock; about the chat's own process only what a
  record shows. Commits end with the two attribution lines this chat's commits carry.
- **Size:** 3a about 2,050 lines, stop line 4,100; a cut moves a whole part with its tests to a named place
  (plan section 4's ladder), and he is told.

## Steps done
0. **#64 merged:** main 1ec6f6c3852d2922989f299bfc053c403d0e52d3 (2026-11-24, 10:15:31 UTC; parents f5030a3 and
   8aadfb0), by his main Advisor on his tap. Main's checks on it, read 10:17 UTC: gate and smoke success (run
   45373057216), toolchain success (run 45373056953, job 138371964348, 119 s, image ubuntu-24.04 20261116.376.1,
   Chrome 157.0.8103.78), its log 18 of 18 by the stop-rule script. #63 and #62 are closed (read 10:18 UTC). The
   report of #64: `docs/reports/2026-11-19-ci-wasm.md`.
1. **This checkpoint opened (2026-11-24, 10:34 UTC):** the branch at main's new head (the remote branch moves from
   8aadfb0 by fast-forward); restored from the notes' last commit 5911b56, each with the same sha256:
   `cs3-plan.md`, `cs3-context.md`, `engine-architect-cs3.md`, `netcode-architect-cs3.md`,
   `rust-reviewer-cs3-rng.md`, `advisor-cs3-plan.md`, `rr-chacha-ref.py`, `test-engineer-followups.md` (it holds
   SA-TREE's draft) and `chat-words.mjs` (pointed at this folder); his three messages since his go, by script.
2. **Rebuilt after his /compact (2026-11-24, 11:04 to 11:11 UTC):** his message of 11:03 UTC (the compact is done;
   rebuild from the repo; tell him in at most 6 lines; then "build change-set 3a by the plan without waiting for
   me"; it goes into DECISIONS with the docs pass, by script). Read: CLAUDE.md, this card, the plan, his entries of
   2026-11-23 in DECISIONS and the three messages here; card, repo and summary agree. The session record: model
   `claude-fable-5-2` (last served the same), permission mode `auto`, context about 10 %. Issue #59: one comment
   (lane 2, 2026-11-18, for change-set 4's plan), nothing for 3a. Set up in this container: `npm ci
   --ignore-scripts` at the root and in `tools/`; the wasm-bindgen archive by the workflow's own curl line into
   `.gate/wb-tool/` (11,860,397 bytes, sha256 9687f3bb...227f; the one program 8,170,784 bytes, sha256
   ca4df773...6931: his TOOL-SOURCE values, both checked). **Baseline on 8a62e79 (code = main 1ec6f6c):** `cargo test
   --workspace --locked` ok (client-endpoint 15, devtools 1, protocol 2, room 1, sim 9, sim-wasm 6); the development
   build ok (Wasm 28,987 bytes here); vitest: types 7, sim-worker src 30, boundary 7, client 49, net 1; the browser
   tests 12 passed (52.6 s, the container's Chromium); `node tools/gate.mjs --no-smoke`: GATE PASS, symbols +0 -0.

3. **The advisor agent's short read of the plan on main 1ec6f6c: APPROVE (build), nine conditions (three Medium,
   six Low), nothing for him to decide** (2026-11-24, 11:06 to 11:25 UTC; Opus 5.5, effort max; its notes:
   `cs3a-rules-boundaries/advisor-cs3a-start.md`). FIREFOX-TESTS (b) changes nothing in section 4; step 7's route
   holds on main's CI text with no CI edit; the release run of `devtools` and `sim-wasm` from this container is
   acceptable evidence until E1; F-CHANNEL is inside the approved plan as a small part of step 8; no step touches a
   protected file (`.gitignore` is none); risk tier 1, his go after his browser check; hand saves carry over;
   nothing on main contradicts a step. **Its conditions, each with where it is met:**
   - C1 (Medium, before the push that adds the rules spec): the spec uses only the Node names
     `packages/e2e/node-names.d.ts` declares, or 3a adds the names there; the five type checks of
     `toolchain.yml` lines 180 to 191 are run here first.
   - C2 (Medium, by the report): `cargo test --release -p devtools -p sim-wasm --locked` here on the final code
     head; record the SHA, cargo's and rustc's versions, the empty list of refused environment names (names only),
     the counts per test binary; keep the log; the report says CI runs these in debug only until E1; again if
     those crates change.
   - C3 (Medium, before the first push whose runs are evidence): the expected counts in this card before each
     such push (cargo per binary, the boundary files and tests, the browser tests, 0 skipped, 0 ignored); each
     run's counts read line by line, a mismatch or a skip is a stop; a red is never re-run; a red the log pins on
     3a's own change is fixed by a new push and named in the report; any other red goes to him with the lines
     (TE-WATCH first); once, here, the test module's folder deleted: both rules tests fail, not skip.
   - C4 (Low, **done here**): **F-CHANNEL joins step 8**: `CARGO_OVERRIDES` gains `__CARGO_.*`,
     `CARGO_UNSTABLE_.*` and `RUSTC_.*`, one refused name per new family in the test; CI's cut names stay allowed
     (no loose `RUST.*`: `RUSTUP_HOME` must pass); it moves with step 8 down the ladder; security-auditor reads it.
   - C5 (Low, step 3): `package-lock.json`'s sim-worker entry changes in the same batch as EA-F13 (4)'s
     `package.json` change, is shown to security-auditor, and is checked with `npm ci --ignore-scripts`.
   - C6 (Low, step 7): the test module's cargo and tool runs behind `build.mjs`'s existing checks (`checkedTool`
     before each tool run, `refusedBuildSettings` before each cargo call, `--target-dir`); its `.gitignore` line
     in the same commit; `--production` never builds it; the boundary file's header says which mode builds it;
     slow boundary tests get their own time limit in the file (vitest's default is 5 s).
   - C7 (Low, **done here**): **the size is counted as change-set 2 counted it**: added lines in `crates/`,
     `packages/`, `.gitignore` and the two root manifests by `git diff --numstat origin/main...HEAD`, the
     lockfiles and the fixture files not counted, the fixtures named (`chacha8-words.bin`, `f32-inputs.bin`, their
     manifests and `f32-expected.txt`). Planned about 2,050 (with F-CHANNEL, the Node names and TE-104/TE-MONO
     about 2,080: inside the estimate, the plan's number stays); **the stop line stays 4,100**.
   - C8 (Low, ongoing): a background agent's notes committed and pushed at least every 30 minutes while it runs;
     the head given to him for his go carries no skip marker; the pull request's title and text are final before
     the commit that deletes the checkpoint.
   - C9 (Low, **done here**): the plan's out-of-date lines are corrected in this card, not in the approved plan:
     where the plan says "#62" (its lines 39 to 42, 190, 191, 224) read #64, merged as main 1ec6f6c (E1's base
     line is `toolchain.yml` line 159); where it says "the saver running" (line 75) read the hand saves of
     "Saves" above.
   Also from its read: TE-104 and TE-MONO ride only with a change to `skeleton.spec.ts` (TE-NT5 makes one; if the
   ladder moves TE-NT5 they move with it); `wasm-bindgen-tool.mjs` is never edited in 3a (CI reads it line by line
   as his TOOL-SOURCE values).
   **Tried in scratch before the first edit (`.gate/cs3a/`, no source touched):** Vite 8.3.7 runs an inline
   `worker.plugins` beside the config file's `worker` settings and its `generateBundle` sees one entry chunk whose
   `facadeModuleId` is `packages/sim-worker/src/worker.ts` (EA-N6's route holds); Node's HTTP parser hands the
   request target `*` to a server (the small server's leading-slash branch gets a case of its own) and refuses
   `app.js` itself; vitest 5.1.0's test context has `skip(condition, note)`. The standard library's inherent
   methods of `f32` and `f64` for the scan test: 79 names each, the same set, read from
   doc.rust-lang.org/1.99.1/std/primitive.f32.html and .f64.html on 2026-11-24, 11:12 UTC (rustdoc "1.99.1
   (58cf8f9c3 2026-10-13)").

4. **Plan step 1 (the follow-ups that need nothing new): a1c59e8**, pushed 11:36 UTC (26 files). What it holds: the
   commit's message. Left for later steps, as planned: EA-F13 (4), `@shamble/types` to sim-worker's
   devDependencies, with step 3's batch; EA-N3's Worker half (one guard around every Wasm call) with step 6; EA-F10's
   bindings part and EA-F13's one-writer item are 3b's. **Checked here before the push:** `cargo test --workspace
   --locked` (client-endpoint 15, devtools 1, protocol 2, room 1, sim 8 and its new `state_hash_alloc` 1, sim-wasm
   6); sim also in release (8 and 1); `cargo fmt --check`, clippy with no warning; both builds (the Wasm still
   28,987 bytes here; the Worker's chunk 8,259 bytes against 8,226); vitest: types 7, sim-worker src 35, boundary 7,
   client 60, net 1; the five type checks of `toolchain.yml` (`cs3a-rules-boundaries/typecheck.sh`); the browser
   tests 12 passed (52.7 s; allocation 54 bytes a frame and 516 a block, inside 25 to 170 and 430 to 610); the
   legacy gate PASS, symbols +0 -0. **Planted defects: 25 of 25 caught, each at its own check**
   (`cs3a-rules-boundaries/plants.py`; outputs in `.gate/cs3a/plant-*.out`; the tree's diff was the same before
   and after): among them TE-NT5's (a Worker that runs on while hidden, with no fresh report: caught only by the new
   line on the page's own tick), TE-104's (90 extra ticks on resume with no drop counted: caught at the bound that
   had never been seen hit) and VF-L7's lookalike chunk. **Size so far, counted as change-set 2 counted: 826 added
   lines against about 410 planned for this step** (tests are more than half; `state_hash_alloc.rs` alone 104).
   The whole of 3a is planned at about 2,050 with the stop line at 4,100: if the later steps also ran at twice
   their estimate, 3a would end at the stop line, so each later step is written as tight as its proof allows, the
   running total is read after every step, and the ladder (plan section 4) is used before the line is reached.
   **Expected in CI on a1c59e8** (toolchain; C3): cargo debug as above, twice; the release line sim 8 and 1,
   protocol 2; `npm test --workspaces`: types 7, sim-worker 35, client 60, net 1, e2e's node project 1; boundary 1
   file, 7 tests; the browser tests "Running 12 tests", "12 passed"; 0 skipped, 0 ignored.

## Current step
**Plan step 2** (the scan test `crates/sim/tests/rules_scan.rs` and PM-0002-53's native half), started 11:36 UTC.
Read CI's run on a1c59e8 before the next code push.

## Next steps (after the compact)
1. Read CLAUDE.md, this card, the plan (`cs3-plan.md`: sections 0, 3, 4, 8, 9, 12 and 13), `cs3-context.md`, and
   from `engine-architect-cs3.md` blocks 2 and 4 (3a's symbols and files). Read the session's model and say it.
2. **A short read by the advisor agent before the first edit:** does the plan (approved 2026-11-19) still stand on
   main 1ec6f6c and under his words since? What changed: FIREFOX-TESTS (b); E1's CI pull request after 3a now also
   carries Firefox; #64 added `refusedBuildSettings` and widened its pattern (step 8 edits the same file); what 3a
   takes from #64's follow-ups (the archive's block for #64): F-CHANNEL with step 8's build work, SA-TREE and
   RR-F5's fix (step 8), the `__pycache__/` line, TE-MONO and TE-104 only if the hidden-tab spec is touched.
3. **Build 3a by the plan's section 4, steps 1 to 8, in order**; step 3 is the one batch of manifest changes (the
   hold); step 4 is our own ChaCha8 (`rust-reviewer-cs3-rng.md`, `rr-chacha-ref.py`). Long runs on GitHub.
4. Reviews (rust-reviewer, engine-architect, test-engineer; then security-auditor and perf-auditor), the verifier,
   the advisor agent, docs-keeper, DECISIONS, the report, the zip and README for his browser check (plan section 4:
   six steps, the covered-window step on purpose), the pull request's title and text in this folder.
5. **Owed with 3a's docs pass, from the advisor agent's read of #64's final head** (Medium unless marked): the
   report of #64 says each checksum was shown to him after its write (true for the planted lines only with the
   chat's message of 23:17 UTC); it says every agent ran at effort max (docs-keeper ran at `high`); the archive
   calls the moved "Phase" block byte for byte (its first two lines were reworded; his quoted words are equal); the
   archive points to "the checkpoint commit that report names" and the report names none (it is 5911b56). Low: the
   report's list of heads the advisor agent read; "Four Lows" in its review lines; STATUS's line on #64's verdicts
   and its "Next" line, which leaves out the 40 % condition.
6. **Owed outside 3a:** the block for him to paste into the first builder chat when its account is back (after
   2026-11-24, 20:00 Amsterdam time): #62 was taken forward in #64, which is merged, and that chat continues neither
   it nor change-set 3. The CLAUDE.md line for ACCT-3 is his to give. His Advisor's three items come before 3b.

## Open findings
- None on main. The follow-ups of #64 and #61: `docs/STATUS.md`, "Follow-ups", and the archive's two blocks.
- WORK-FOLDER (not asked yet): hand saves hold as long as the worktree's local name differs from the branch on
  GitHub; if they get heavy, ask him whether 3a may be built in the main folder, where the saver script works.
- The card's old rule "never read `/root/.claude/uploads/`" has no source in his words; a file he attaches lies
  there. Agents are still told to stay out of it.

## Key numbers
- main 1ec6f6c; #64's final head 8aadfb0, its code head 6d0b5b1, its notes' last commit 5911b56.
- 3a: about 2,050 lines, stop line 4,100; about 280 KB of committed data (the vectors).
- CI today: one `toolchain` job, about 2 to 2.5 minutes; the Wasm on the runners 28,991 bytes, sha256
  22dad2ebf4be21ce0f123753bb77cfa25ca06886eddace43651601831ee8018b.
- This container (it can be replaced at any time): cargo 1.99.1, Node v22.23.5; no `node_modules` installed and no
  wasm-bindgen tool (the workflow's own curl line and checks bring it: TOOL-SOURCE). Untracked scratch of #64 under
  `.gate/ci-wasm/` in the worktree (the logs, the notes' copy, `advisor-final.md`): not needed to resume.

## Decisions taken
- 2026-11-24: his main Advisor marked #64 ready and merged it on his tap (the merge commit's message).
- 2026-11-23, 23:44:10 UTC: GH-NEXT (a) with his change; CLOSE-OLD (a); the order after the merge.
- KIT-QUOTE: the advisor agent ruled (a) (his words of 2026-11-13 stay in the archive and DECISIONS, with a pointer
  in STATUS); told to him with #64's ask, no answer needed.

## Running work
- Nothing runs in the background. No saver.
- How things are run here (the guard refuses `npx`, `npm run`, `npm test`): vitest by
  `node node_modules/vitest/vitest.mjs run --root packages/<name> [--dir ...]`; the browser tests by
  `SHAMBLE_DIST="$PWD/.gate/dist-development" SHAMBLE_CHROMIUM=/opt/pw-browsers/chromium node
  node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --workers=1
  --retries=0 --reporter=list --global-timeout=300000 --trace off`; the build by
  `SHAMBLE_WASM_BINDGEN="$PWD/.gate/wb-tool/x/wasm-bindgen" node packages/client/scripts/build.mjs --out-dir
  "$PWD/.gate/dist-development"`; logs under `.gate/cs3a/`.
- Where step 1's items are spelled out: the reviewers' notes of change-set 2 at commit e9f31f0
  (`git show e9f31f0:.checkpoints/cs2-first-page/test-engineer-review1.md`, `-review2.md`,
  `engine-architect-review1.md`, `-review2.md`): TE-L's four server branches (the leading slash, the lexical "..",
  the empty path, not a file), the silent returns of the link cases, the unstubbed WebAssembly names, the
  one-Worker assert, the scratch-buffer proxy; TE-NT3 (the error text's cleaning), TE-NT4 (the hand-off time read
  before the copy), TE-NT5 (the hidden-tab equality also holds on stale reports).
