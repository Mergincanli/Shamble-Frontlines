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
- **Opened:** 2026-11-24 10:34 UTC (from the clock). **Last update:** 2026-11-24 14:53 UTC (from the clock).

## His words that govern (read them where they stand; these lines are pointers, never a substitute)
- **2026-11-24, 15:58:25 UTC** (`mustafa-2026-11-24T155825-mul-add-a.md`; in `decisions-4.mjs`): **MUL-ADD (a)**: the
  fused multiply-add stays banned in the simulation and what 3a built for it stays as it is; no word of ADR 0004
  changes in 3a (part 7's dated pointer comes at its next revision, its exact sentence shown to him first). For the
  report and STATUS's follow-ups: the check on the built file comes in the change-set that brings the first outside
  crate into the simulation (the collision library, change-set 4), and that plan names it; reporting the defect to
  the Rust project is a follow-up, nothing is posted anywhere without his go. The stop line of 4,100 stays: if round
  2's fixes would pass it, cut by the plan's ladder and tell him in three lines. Four decisions of his to record by
  script with the docs pass (the animation editor, Testlines and the level editor, 3b first in line, the VFX
  repository). **"From now on use plain words with me: no tag or code without its explanation in the same
  sentence."** When the pull request's title and text are in the notes folder, tell him; his main Advisor opens
  the draft.
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

   **CI on a1c59e8, read line by line (the log: `.gate/cs3a/ci-a1c59e8-toolchain.log`):** gate success (run
   45385683163), toolchain success (run 45385682879, job 138415844793, 11:36:13 to 11:38:31 UTC): every count as
   expected, 0 skipped, 0 ignored; the Wasm's sha256 on the runner 22dad2eb...018b, as on main; Chrome
   157.0.8103.78; allocation 43 bytes a frame and 539 a block.
5. **Plan step 2 (the scan test and the planted counter): 50656d6**, pushed 11:42 UTC (3 files).
   `crates/sim/tests/rules_scan.rs` (387 lines): std 1.99.1's 79 inherent float methods less ADR 0004 part 7's 14;
   the hash-map ban with part 9's escape recognised on an item with a written reason only; the clock ban; 22
   planted breaches in the file, each found once and by name, and 8 controls. It found one real case, the tick's
   bytes in the method form (`tick.to_le_bytes()`), now `u32::to_le_bytes(*tick)`: **integer code in `sim` writes
   the path form on its own type, because a scan of text sees no types** (so will the ChaCha8 code). PM-0002-53's
   native half: the planted `u16` counter and its wrapping control, debug and release. Checked: `cargo test
   --workspace --locked` (sim 9, `rules_scan` 4, `state_hash_alloc` 1); sim in release; fmt; clippy clean; the gate
   PASS. **Planted on the crate itself: 5 of 5 caught** (a hash map, a float method, a clock, the method form
   again, and the release profile's overflow checks turned off on cargo's command line). **Size so far: 1,228
   added lines against about 710 planned for steps 1 and 2.** **CI on 50656d6:** gate success (run 45386600136),
   toolchain success (run 45386600112, job 138419013852; the log: `.gate/cs3a/ci-50656d6-toolchain.log`): cargo debug
   15, 1, 2, 1, 9, 4, 1, 6, twice; release protocol 2, sim 9, 4 and 1; the unit and browser counts as on a1c59e8.

6. **Plan step 3 (the one batch of manifest changes): 0412814**, with its release record f5a9f67, pushed 13:36 UTC.
   security-auditor's answer (Opus 5.5, effort max, 13:12 to 13:25 UTC; `security-auditor-deps.md`): "The dependency
   change may be committed and built: yes with conditions"; neither condition changes one of the four files. Its
   condition 1 is met and in `dependency-check.md`: the players' Wasm from the head's code plus only the four
   files, in the clean worktree `.gate/cs3a-batch` with a clean target folder, is byte for byte the Wasm without
   them (28,987 bytes, sha256 c4935aca...2c31); the four hashes unchanged after `npm ci --offline
   --ignore-scripts` and after the build. **CI on f5a9f67:** gate success (run 45405162456), toolchain success (run
   45405162281, job 138483741233; `.gate/cs3a/ci-f5a9f67-toolchain.log`): the counts as on step 2, and the Wasm's
   sha256 on the runner still 22dad2eb...018b. **Its condition 2 binds step 7** (below). Its Low follow-up: the
   root `Cargo.toml`'s comment "in sim-wasm only" (line 14), for the next manifest batch (3b's).
7. **Plan steps 4 and 5 (our own ChaCha8 with its vectors; the float vectors): ec27f85**, one commit because the two
   share `crates/devtools/src/rules.rs`. First real run after the hold: the 8 published vectors, the 160,384-byte
   stream file and all 38 float operations passed at once, debug and release. `rng.rs` 299 lines;
   `chacha8-words.bin` sha256 8531eb01...68f2 (its manifest beside it); `f32-inputs.bin` 120,000 bytes, sha256
   85629398...ae6b (in `f32-expected.txt`'s first line); both generators regenerate their files byte for byte
   (`--check`). **Planted: 15 of 15 caught** (8 in the generator: a rotation, the rounds, the stream id's high
   half, the counter's carry, 64-bit draws, a stale block, the position; 7 in the float operations: `round`,
   `mul_add` as two roundings, `total_cmp`, `fract`, a cast, `sqrt` one unit off, a NaN's own bits).
8. **Plan step 6 (the panic hook and the Worker's guard): 0624557**, pushed 13:43 UTC with steps 4 and 5. The
   players' Wasm with the hook: 31,809 bytes raw, 12,658 gzip, 10,805 Brotli here (28,987, 11,655, 9,955 without);
   sha256 here 55ca031f...00b9. Checked on clean builds (`.gate/cs3a/step6.log`): cargo debug (client-endpoint 15,
   devtools f32 2 and stream 2, protocol 2, room 1, sim 15, `rules_scan` 4, `state_hash_alloc` 1, sim-wasm 9) and
   release (sim, protocol, devtools, sim-wasm); fmt; clippy clean; both pages; vitest types 7, sim-worker 40,
   boundary 8, client 60, net 1; the five type checks; the browser tests 12 passed (53.2 s; allocation 52 and 537);
   the gate PASS. **Planted: 9 of 9 caught** (one plant passed at first: the compiler had joined three constant
   text pieces into one, so "nothing follows the cut" was not exercised; the test now uses pieces known only at
   run time). **End to end, with a panic planted in the players' Wasm at tick 90 (restored, never committed;
   `trap-on-page.mjs`, output `.gate/cs3a/trap-on-page.out`):** the page's notice reads "The match stopped: the
   simulation stopped: panicked at crates/sim-wasm/src/lib.rs:140:9:" and on its next line "planted: tick 90 is one
   too many"; errors 1; the readout's last error the same on one line; the newest block tick 89; no page error; the
   Worker stopped.
   **Size after step 6, counted as change-set 2 counted (fixtures and lockfiles not counted): 2,558 added lines
   against about 1,730 planned for steps 1 to 6 (1.48 times).** Steps 7 and 8 are planned at about 355: at the same
   ratio 3a ends near 3,100, under the stop line 4,100.
   **Expected in CI on 0624557 (C3):** cargo debug as above, twice; the release line (sim, protocol only) sim 15, 4
   and 1, protocol 2; `npm test --workspaces`: types 7, sim-worker 40, client 60, net 1, e2e's node project 1;
   boundary 1 file, 8 tests; the browser tests "Running 12 tests", "12 passed"; 0 skipped, 0 ignored.

   **CI on 0624557 (steps 4 to 6), read line by line:** gate success (run 45406415137), toolchain success (run
   45406414987, job 138488241220): every count as expected; the Wasm with the hook on the runner 31,817 bytes,
   sha256 534bb6bd...0ca6 (31,809 here: the builder's path, RR-F5).
9. **Plan step 7 (the test-only WebAssembly module): df72a58**, pushed 13:50 UTC. `crates/devtools/src/wasm.rs`
   (the two vector checks, the planted counter; `sim-wasm` linked whole, so the module carries the players' hook
   and its start function: tried first by hand, it holds); `build.mjs` `buildRulesModule` (every build but
   `--production`: release and dev profile, its own cargo call `-p devtools --lib` and target folder
   `.gate/target-rules`, the settings refused and the tool checked before each run, into
   `packages/sim-worker/pkg-rules/{release,dev}`, ignored in the same commit); `wasmProblems` (a built page with
   any WebAssembly that is not byte for byte the players' own is refused); `boundary/rules.test.mjs` (Node) and
   `e2e/tests/rules/wasm-rules.spec.ts` (Chromium, a made-up address the spec answers itself; only the Node names
   `node-names.d.ts` declares: C1). In both profiles, Node and Chromium: the 38 float operations and the stream
   file pass; the counter traps and the hook's text reads "panicked at crates/devtools/src/wasm.rs:34:5:" then
   "attempt to add with overflow"; the players' Wasm imports three functions of its own glue and nothing else.
   **security-auditor's condition 2: met** (its last clause shown on a planted copy: with the module's Wasm in
   `packages/client/public/` the build stops with "extra_bg.wasm: a WebAssembly file that is not the players'
   own"). **C3's once: shown** (with `pkg-rules` moved away the boundary file fails at its imports and both spec
   tests fail; none skips; `.gate/cs3a/step7-missing-and-foreign.out`). **Planted: 6 of 6 caught** (with the
   release profile's overflow checks off only the release module's test fails, 14 of 15 pass: PM-0002-53's four
   builds each shown). **CI on df72a58, read line by line:** gate success (run 45407675059), toolchain success
   (run 45407674933, job 138492663865, 13:50:54 to 13:53:47 UTC, with no CI edit): the development build made the
   module inside the network cut; boundary 2 files and 15 tests; "Running 14 tests", "14 passed (1.2m)", the two
   rules tests in the runner's Chrome in both profiles; client 61; the rest as before.
10. **Plan step 8 (the build's two refusals, the path remap, F-CHANNEL): 5c92311**, pushed 13:56 UTC.
   `build-checks.mjs`: `runtimeCrates`, `overriddenCrates`, `missingOverflowOverrides` (SA-TREE);
   `simFeatureList`, `simFeatureDifferences` (PM-0004-74: the players' Wasm, the test-only module and the native
   tests; today each list is `sim feature "default"` alone); `registryRemapFlag`; the refusal pattern widened by
   every `RUSTC_` name, `__CARGO_` and `CARGO_UNSTABLE_` (F-CHANNEL; CI's cut names stay allowed). `build.mjs`:
   `checkCrates()` before the players' cargo build; `--config target.wasm32-unknown-unknown.rustflags=[the
   remap]` on both Wasm builds. The players' Wasm here is now 31,805 bytes, sha256 e138cae8...2bc5, and holds
   `/cargo/registry/src/...` in place of `/root/.cargo/registry/src/...`; **the same sha256 is expected on the
   runner: read it in CI on 5c92311** (its line "the builder paths it carries" should print "no registry path in
   the Wasm"). **Planted: 6 of 6 caught**, two of them by the real build's refusal (an override removed from the
   root manifest; a feature of `sim` turned on by the test-only module alone). Checked on clean builds
   (`.gate/cs3a/step8.log`): cargo debug 9 result lines and release 7, all ok; fmt; clippy clean; both pages;
   vitest types 7, sim-worker 40, boundary 16, client 68, net 1; the five type checks; the browser tests 14 passed
   (52.7 s); the gate PASS.
   **Size after step 8, counted as change-set 2 counted (fixtures and lockfiles not counted): 3,046 added lines
   against about 2,080 planned (1.46 times); the stop line 4,100 is not reached.** By package: sim 809, client
   786, devtools 752, sim-worker 437, sim-wasm 159, e2e 101, root 2; 1,428 of them in test files, and 346 in the
   two reference scripts. **Planted defects so far: 66 planted, 66 caught** (`plants.py`, by step: 25, 5, 15, 9, 6 and 6;
   beside them the 22 breaches planted inside `rules_scan.rs` itself), one test strengthened on the way.
   **Expected in CI on 5c92311 (C3):** cargo debug client-endpoint 15, devtools f32 2 and stream 2, protocol 2,
   room 1, sim 15, `rules_scan` 4, `state_hash_alloc` 1, sim-wasm 9, twice; the release line protocol 2, sim 15, 4
   and 1; `npm test --workspaces`: types 7, sim-worker 40, client 68, net 1, e2e's node project 1; boundary 2
   files, 16 tests; "Running 14 tests", "14 passed"; 0 skipped, 0 ignored; the Wasm's sha256 e138cae8...2bc5.

   **CI on 5c92311 (step 8), read line by line:** gate success (run 45408570716), toolchain success (run
   45408571090, job 138495835913, 13:56:06 to 13:58:29 UTC; image ubuntu-24.04 20261116.376.1, Chrome
   157.0.8103.78; `.gate/cs3a/ci-5c92311-toolchain.log`): every count as expected (client 68, boundary 2 files and
   16 tests, "14 passed (59.8s)", allocation 50 and 525). **RR-F5 is shown: the players' Wasm on the runner is
   31,805 bytes with sha256 e138cae8b686fed4fe317602332fb65616fb7235194c9043ee49c4451ed72bc5, the very bytes of
   this container's build, and the job's line reads "no registry path in the Wasm".**

11. **Round 1's fix pass: 79666ad9** (79666ad9c8ac3a8d756f874873c820b1c178bc35), pushed 15:27 UTC. One pass for the
   five reviewers' findings (0 Critical, 2 High, 7 Medium, Lows where cheap):
   - **`mul_add` (H1; the advisor agent's interim ruling in `advisor-mul-add.md`: (a) on seven conditions).** The
     float vectors gain 48 constructed cases after the 10,000 (10,048; `f32-inputs.bin` 120,576 bytes, sha256
     f4599394...3396; its first 120,000 bytes are the old file's). Each is asserted by the generator to be a true
     double-rounding case, and it asserts that none of the 10,000 others is. **Measured at 15:14 UTC** (scratch probe
     `.gate/cs3a/hard2/`, Rust 1.99.1): natively on this FMA processor all 48 are exact; in wasm32, release and dev
     alike, the 32 with a normal result are exact and the 16 whose result lies below the smallest normal f32 give
     the two-roundings value. Those 16 are `pin` lines in `f32-expected.txt` with their measured values. In
     WebAssembly every case is still checked, the 16 at their measured value (any change fails by case: "MUL-ADD is
     open again"); natively nothing is pinned and every case must be exact; the native test prints whether the
     processor has FMA (`check_as`, and the test `a_pinned_case_that_no_longer_gives_its_measured_value_fails_by_name`).
     The scan refuses `mul_add` in `sim` through its own one-name list `HELD`; part 7's `ALLOWED` (14) and its test
     are untouched. std 1.99.1's page for `f32::mul_add` says "The result of this operation is guaranteed to be the
     rounded infinite-precision result" (fetched 15:25 UTC, 346,838 bytes, sha256 039fc4ea...65e4): a toolchain
     defect. **Not run anywhere: a processor without FMA.**
   - `from_f64` rounds a real f64 (two inputs' bits side by side); its digest is no longer `mul`'s.
   - The scan (rust-reviewer's M4 and L6, test-engineer's Medium): a banned name counts after any path that is not
     an integer's or `Ord`, `PartialOrd`, `Iterator`, `cmp`; names holding `HashMap` or `HashSet`; `target_feature`
     and `core::arch`; `#[path]` and `include!`; the escape only on `type, struct, enum, static, const, use`;
     `0..min(` and a string's line break read right; the macro gap in its doc. 35 planted breaches, 10 controls.
   - Build checks (H2, security-auditor's M1, rust-reviewer's M3, L1, L2): a member is a folder under `crates/`;
     the feature list is every feature of `sim`'s dependency closure from the whole tree, with dev edges for the
     tests' list; fixtures are real cargo output; the remap goes through `CARGO_ENCODED_RUSTFLAGS` in cargo's
     child environment; refused names are compared in upper case; an empty `CARGO_HOME` counts as unset.
   - The hook (rust-reviewer's M2, engine-architect's F2 and F3, security-auditor's L3): the doc says an
     out-of-memory stop leaves no message; `#[wasm_bindgen(start, private)]` (the glue exports exactly `Sim`,
     `default`, `initSync`, `panic_buffer`, `tick_hz`); `PANIC_TEXT_ROOM = 1016` and `trapMessage` in
     `panic-message.ts`.
   - security-auditor's L1 (its condition 2, clause (d)): the dev server reads only the Worker's `src` and `pkg`;
     the isolation spec asserts 200 for the players' Wasm and 403 for the test-only module's two files.
   - Small: the draft's sha256 beside the published vectors; `next_u64` across a block's end; a const assert on
     the seed's length; perf-auditor's two Lows (`doneAt` read once; the comment).
   **Checked** (`.gate/cs3a/fix1.log`, 15:23 to 15:24 UTC, before the commit, same tree): fmt, clippy clean; both
   generators `--check`; cargo debug client-endpoint 15, devtools f32 3 and stream 2, protocol 2, room 1, sim 15,
   `rules_scan` 5, `state_hash_alloc` 1, sim-wasm 9, twice; release (with devtools and sim-wasm) the same counts;
   vitest client 70, net 1, sim-worker 42, types 7, boundary 17; the five type checks; the browser tests 14 passed
   (52.5 s); the gate PASS. The players' Wasm: 31,718 bytes, sha256 383f5303...3963.
   **Planted: 90 plants, 90 caught** (`plants.py`; the full run `.gate/cs3a/plants-fix1.log`, 15:22 to 15:26 UTC:
   89 caught, and `features-blind` failed three tests but not the one its line named, a test this pass renamed;
   re-aimed and caught at 15:26 UTC). 24 of the 90 are new in this pass, 4 were re-aimed at moved code.
   **Size: 3,496 added lines** (fixtures and lockfiles not counted) against about 2,080 planned (1.68 times); the
   stop line 4,100 is not reached (604 lines of room). By package: devtools 946, sim 903, client 884, sim-worker
   488, sim-wasm 164, e2e 109, root 2; 1,656 in test files, 430 in the two reference scripts.
   **CI on 79666ad9, read line by line:** gate success (run 45425663173); toolchain success (run 45425663247, job
   138556835447, 15:27:12 to 15:29:44 UTC; image ubuntu-24.04 20261116.376.1, Chrome 157.0.8103.78;
   `.gate/cs3a/ci-79666ad9-toolchain.log`): every count as expected (cargo debug 15, 3, 2, 2, 1, 15, 5, 1, 9,
   twice; release 2, 15, 5, 1; client 70, e2e's node project 1, net 1, sim-worker 42, types 7; boundary 2 files
   and 17 tests; "Running 14 tests", "14 passed (1.1m)"; nothing skipped or ignored). The players' Wasm on the
   runner: 31,718 bytes, sha256 383f53039ca93651f3d1655deb956040f688b45b0a9d347199c6f08f9f833963, this
   container's bytes again; "no registry path in the Wasm". The pinned cases hold in the runner's Chrome too (the
   rule tests passed in both profiles), and the native float test passed there (so that runner had FMA, or its
   software path is exact: not told apart by the log).

12. **Round 2's fix pass: a1b2e802, then 8619d539** (8619d53968e997846fc1a9174051862030a41a94), pushed 16:13 and
   16:18 UTC. Round 2 (five reviewers on 79666ad9): security-auditor REVISE with 1 High; rust-reviewer REVISE with
   1 new Medium; test-engineer APPROVE with 1 new Medium; engine-architect REVISE only for that High; perf-auditor
   OK.
   - **The High (the dev server reached the test-only module through npm's link):** the module is now built into
     `.gate/rules-module/{release,dev}` at the repository's root, outside every package (`build.mjs` `rulesDir`;
     engine-architect: the right fix, the boundary then holds by where the module sits). The dev server's list
     holds `node_modules/three` instead of `node_modules`. The isolation spec asserts: the players' Wasm 200 with
     type `application/wasm`; 403 for the module's two files at their real place; 403 for the Worker's package
     through npm's link (two files); no WebAssembly from the module's old place. The `.gitignore` line for the old
     folder stays (a stale copy lies in this worktree; deleting it was refused here, and nothing reaches it).
   - **The scan:** a range's `..` is one token, so `self.0.sin()` and `m.0.mul_add(..)` are seen again (my
     regression in step 11's range fix: rust-reviewer's N1); `arch` counts as a path step or a `use` entry however
     it is brought in, and names holding `_relaxed_` (test-engineer's Medium); `#[cfg_attr(.., path = "..")]`;
     the escape does not cover a `const fn`. 43 planted breaches and 12 controls inside it.
   - Small: `registryRemapFlag(cargoHome, homeDir, base)` resolves a relative home against the folder cargo runs
     in and refuses a control character in the path; comments say what a cargo config file can still set; the
     WebAssembly float test's title and doc name the pinned cases; the FMA line allows for FMA4; `panic.rs`'s doc
     on `start`; the feature compare's doc names its clash with ADR 0002 part 5's planned dev-only feature.
   - **CI was red on a1b2e802** (run 45434633338, job 138588985695, 16:13:42 to 16:16:23 UTC): 12 of 14 browser
     tests passed; the two dev-server cases failed at the one assertion I had added last, "not 200" for the
     module's old place. On a clean machine no stale copy lies there, and Vite's dev server answers 200 with the
     page itself for a file that is not there; here the stale copy made it 403, so the local run could not show
     it (measured after: `.gate/cs3a/devserver-probe.mjs`). Not re-run. **Fixed forward in 8619d539:** the
     assertion asks that what comes back is no WebAssembly, and the players' control asks for the file's type too.
   **Checked** (`.gate/cs3a/fix2.log`, 16:09 to 16:11 UTC, the tree of a1b2e802): the same counts as step 11; the
   players' Wasm unchanged (31,718 bytes, sha256 383f5303...3963). **Planted: 96 plants, 96 caught in one run**
   (`.gate/cs3a/plants-fix2.log`, 16:11 to 16:13 UTC; the two dev-server plants again on 8619d539's tree at 16:18
   UTC). **Size: 3,557 added lines** (1.71 times the plan; 543 of room to the stop line 4,100); by package: devtools
   946, sim 936, client 904, sim-worker 489, sim-wasm 165, e2e 115, root 2; 8619d539 adds 4 more (3,561; 539 of room).
   **CI on 8619d539, read line by line:** gate success (run 45435601291); toolchain success (run 45435601223, job
   138592422504, 16:18:37 to 16:21:42 UTC; image version 20261116.133.1, Chrome 157.0.8103.78;
   `.gate/cs3a/ci-8619d539-toolchain.log`): cargo debug 15, 3, 2, 2, 1, 15, 5, 1, 9, twice; release 2, 15, 5, 1;
   client 70, e2e's node project 1, net 1, sim-worker 42, types 7; boundary 2 files and 17 tests; "Running 14
   tests", "14 passed (1.1m)"; nothing skipped or ignored; the players' Wasm sha256 383f5303...3963; "no registry
   path in the Wasm".

13. **Round 3, the verifier, the docs pass: c5f6a85a**, pushed 17:06 UTC. security-auditor's round 3 on 8619d539:
   APPROVE, the High closed (99 request spellings, none returned the test-only module). The verifier on 8619d539:
   PASS, nothing above Low (CI read line by line; its own full run on a clean copy; 40 of 40 plants; `mul_add` on
   all 10,048 inputs; the draft's and the Rust page's text and checksums). C2's release run at 16:26 UTC
   (`.gate/cs3a/c2-8619d539.log`): devtools f32 3 and stream 2, sim-wasm 9; cargo 1.99.1 (5f94df478), rustc 1.99.1
   (58cf8f9c3); refused environment names: none. The docs pass: `docs/DECISIONS.md` by `decisions-4.mjs` (six
   entries; `--check` passes); the report `docs/reports/2026-11-24-cs3a-rules-boundaries.md`; docs-keeper's
   STATUS (120 lines), archive (23 follow-ups 3A-…, the closed marks), ROADMAP, REPO_MAP (the map check passes),
   and the dated corrections to #64's report.
   **CI on c5f6a85a, read line by line:** gate success (run 45444844867); toolchain success (run 45444844763, job
   138625652760, 17:06:38 to 17:09:06 UTC; `.gate/cs3a/ci-c5f6a85a-toolchain.log`): the counts of step 12 again;
   "14 passed (59.3s)"; the players' Wasm sha256 383f5303...3963; "no registry path in the Wasm".
   **The zip for his browser check** (`make-zip.py`, from the development build at c5f6a85a):
   `.gate/cs3a/zip/shamble-v2-03a-rules-boundaries-c5f6a85a3/shamble-v2-03a-rules-boundaries.zip`, 246,904 bytes,
   sha256 8915aedebf1800f06f9e6d98bc7b1cf33855d3e22c2e8b4a4612df75d3c366f7, 10 files, each equal to its line in
   `SHA256SUMS.txt` after unpacking. Its unpacked copy run as he will run it (`.gate/cs3a/zip-check.mjs`, its own
   server, Chromium): "errors: 0", "commit: c5f6a85a3", ticks at 30 a second; opened as a file it shows the one
   line that says to run `node serve.mjs`. **Not sent yet: it goes to him with the advisor agent's verdict.**

14. **The advisor agent on c5f6a85a: REVISE, for wording only; corrected in 8d277cf2**, pushed 17:38 UTC. By its
   own gate run, build (the same Wasm bytes), ChaCha8 and exact fractions the code is fit and proven; its nine
   start conditions (C8 open until the final head) and its seven MUL-ADD conditions are met. **Its High (A): four
   places put the register's wording in quotation marks as Mustafa's own** ("two change-sets, 3a then 3b"; "our own
   ChaCha8, checked against the published test vectors": the report twice, `rng.rs`'s doc, `pr-body.md`); his
   recorded choice was "Yes to all seven". Its Mediums: B the scan's comment still said "until Mustafa answers"; C
   the open security follow-ups were not named as what his go accepts; E no docs-pass section in the report. Lows
   F to J. All worked into 8d277cf2 (comments in two Rust files, the report, the archive, STATUS, `pr-body.md`);
   every quotation of his 15:58:25 UTC message in the report checked by script against his saved message. The
   players' Wasm unchanged (383f5303...3963). **CI on 8d277cf2, read line by line:** gate success (run
   45450856180); toolchain success (run 45450856204, job 138647229600, 17:38:49 to 17:41:48 UTC;
   `.gate/cs3a/ci-8d277cf2-toolchain.log`): the counts of step 12 again; "14 passed (1.2m)"; the Wasm's sha256
   383f5303...3963; "no registry path in the Wasm". **The zip was sent to him** (built from c5f6a85a3; the game
   files are the same; the advisor agent: "The zip may go to him now").

## Current step
**Waiting for three things** (17:42 UTC): the advisor agent's verdict on the corrected head 8d277cf2
(8d277cf21cf54919c2ae54e5232d45e944e7b040; asked after 17:38 UTC, its notes `advisor-head.md`); the verifier's
cover of that head (comments only since 8619d539; `verifier-final.md`); **Mustafa's browser check of the zip (sent
to him after 17:38 UTC) and his main Advisor's draft pull request from `pr-title.txt` and `pr-body.md` here**
(told to him with the zip). Then the LAST commit: the advisor agent's verdict on 8d277cf2 and his browser reports
into the docs (his messages by `chat-words.mjs` and a DECISIONS entry by script), the report's notes pointer
(the commit where the notes stay readable: this branch's last commit before the deletion), the checkpoint
folded and deleted, no skip marker; CI; the verifier and the advisor agent on that head; his go for its full SHA.
Plain words with him: no tag or code without its explanation in the same sentence.
(Card written 15:31 UTC.)

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
- **The advisor agent on 8d277cf2 (`advisor-head.md`, "Review 2"; read 17:45 UTC): APPROVE
  8d277cf21cf54919c2ae54e5232d45e944e7b040.** Every letter A to J of its read of c5f6a85a is closed (its own
  build: the same Wasm bytes; its own script: every quotation given as his stands in his saved messages; CI green
  on the first attempt). **It is the corrected head, not the merge head: his go waits for the last commit's full
  SHA and its read of that.** Two Lows for the last commit: the report says its nine start conditions "are met",
  but C8 closes only with the last commit; "five dated corrections" to #64's report are five corrections plus one
  dated addition (the 5911b56 pointer). **The last commit must hold:** its verdict on 8d277cf2; his browser
  reports, whole and by script; the commit where the notes stay readable, named in the steps table's heading; the
  checkpoint folded in and `.checkpoints/` deleted (`git ls-tree -r --name-only HEAD -- .checkpoints` prints
  nothing); no skip marker; no code, manifest or lockfile change (the Wasm stays 383f5303...3963, so his check
  still covers it; a fault he finds is a new head and a new read); `pr-title.txt` and `pr-body.md` exactly as they
  are now (they are read from the last commit that still has them). Then gate, smoke and toolchain green on it
  and its read. The zip built from c5f6a85a3 covers this head.
- **Round 3, security-auditor (judged 8619d539; `security-auditor-review3.md`; read 16:37 UTC): APPROVE. The High is
  closed; its condition 2, clause (d), is met for Vite's dev server.** 99 request spellings on fresh dev servers, 88
  on a warmed one, 31 in sequence, each answer's body read: none held the test-only module. N1, N2 and the
  relative-home note fixed. New Low N3 (wording; a STATUS follow-up with 3A-CONFIG-FILE): the comments that list
  what a cargo config file can still set leave out source replacement and `build.rustc`. Not verified by it:
  Windows and macOS; the e2e suite and the plants (read from CI and the builder's log).
- **No Critical, High or Medium is open.** The verifier (started 16:23 UTC, `verifier-final.md`) and docs-keeper
  (`docs-keeper-cs3a.md`) run. Prepared: `docs/DECISIONS.md` by script (six entries), the report's draft
  (`docs/reports/2026-11-24-cs3a-rules-boundaries.md`), `pr-title.txt` and `pr-body.md` here, C2's release run
  (`.gate/cs3a/c2-8619d539.log`, 16:26 UTC). Left: the verifier's verdict into the report; docs commit and push;
  CI; the advisor agent on that head; the development build at that head, the zip (`make-zip.py`) and its send;
  tell him the title and text are here; after his browser check: his reports into the docs, the checkpoint folded
  and deleted, the advisor agent on the final head, his go.
- **State at 8619d539 (16:22 UTC): round 2's High, both Mediums and the cheap Lows are taken (step 12); round 3
  (security-auditor) reads the High's fix.** Left as follow-ups for STATUS: CI runs neither generator's `--check`
  (with E1's CI pull request); `from_f64` has no exact tie and no case between the largest f32 and the overflow
  point (test-engineer and rust-reviewer); the feature closure matches packages by name, not name and version
  (engine-architect's NEW-A; fails closed); the scan refuses part 7's own helpers when imported by name or called
  on our own ordered newtypes (engine-architect's NEW-B: with the helpers' change-set or the quality gate step);
  the clash between the feature compare and ADR 0002 part 5's planned dev-only feature (NEW-C: for the step that
  first nests generated types); hashed maps not named after std's; a passing native float test does not show in
  the log which path `mul_add` took (cargo hides a passing test's line).
- **Round 2, engine-architect (on 79666ad9; `engine-architect-review2.md`): REVISE only for security-auditor's
  High; its own: 0 Critical, 0 High, 0 Medium, 3 Low** (NEW-A, NEW-B, NEW-C above). Its round-1 findings are
  fixed or deferred as agreed. `cfg!(target_arch)` in the tools crate's check is the right place (it only chooses
  which expected values are compared; nothing per-target is in `sim` or `sim-wasm`). On the dev server: (B), the
  module outside every package, and (A) as well.
- **Round 2, perf-auditor (on 79666ad9; `perf-auditor-review2.md`; read 16:05 UTC): OK, no finding in its field.**
  Both its Lows are fixed: the block check allocates 280.0 bytes per block again in its harness (296.0 before the
  fix, main's 280.0), 0 once optimised; the comment is true. No regression per block, per frame or per wake; the
  players' Wasm went from 31,805 to 31,718 bytes for the reasons it lists (a dropped panic function and its name).
  One Low outside its field: `panic.rs`'s comment says the host cannot call `start`; the raw Wasm exports still
  hold it (harmless: it installs the same hook again). **Wording taken in the round-2 pass.**
- **Round 2, rust-reviewer (on 79666ad9; `rust-reviewer-review2.md`; read 16:00 UTC): REVISE, 0 Critical, 0 High, 1
  new Medium (made by the fix pass), 10 Low.** Its four round-1 Mediums are fixed, each checked by itself. It
  measured `mul_add` on 22,048 cases of its own: natively (FMA) all exact; in wasm32 every result below the smallest
  normal f32 is the two-roundings value and every normal one is exact; the committed file differs at exactly cases
  10024 to 10039, each at its pinned value: **the defect is the whole class of such results, not 16 points**. The
  pin mechanism is sound.
  - Its answers to the advisor agent's question: (a) no other allowed operation picks its native path by
    processor (it read the toolchain's whole x86-64 compiler_builtins: only `fma` and `fmaf` do; ARM64 not
    checked); (b) without FMA (and without AMD's FMA4) the native `fmaf` takes the same software path as wasm32:
    run under a debugger with the FMA flags cleared it was bit for bit wasm32's on all 22,048 cases, and the
    repository's three native float tests then fail, 3 of 3 (never on a real such processor).
  - **Medium N1, `rules_scan.rs`: the range fix made tuple-field calls invisible.** The lexer drops numbers, so
    `self.0.sin()` reads as two dots and `before_2 != "."` lets it through; `m.0.mul_add(..)` too. Fix: lex `..`
    as one token where the dots stand side by side, drop the `before_2` test, bites for both. **Taken in the
    round-2 pass; it says no third round is needed for it.**
  - Lows: the scan misses `#[cfg_attr(x, path = "..")]` (taken) and hashed maps not named after std's (`DashMap`,
    hashbrown's `HashTable`: the comment is corrected; the type lint of step 2.3 is the real net); `const` in the
    escape's item words covers a `const fn` body (taken if cheap); the FMA line says "NO FMA" on a processor with
    FMA4 only, where the path is exact (wording taken); a relative `CARGO_HOME` against the root (taken); the
    feature closure matches by crate name (fails closed; left); `from_f64` has no exact tie (a STATUS follow-up,
    as test-engineer's); the generator's block copy and the hook's test binary stay follow-ups. For the report:
    without FMA the three native float tests fail, not "may fail".
- **Round 2, test-engineer (on 79666ad9; `test-engineer-review2.md`; read 15:52 UTC): APPROVE, 0 Critical, 0 High
  (both round-1 Highs fixed), 1 new Medium, 4 new Low, 1 nit.** It confirmed the advisor agent's conditions 1 to 4
  for MUL-ADD by its own runs (a C check with glibc's `fmaf`: 48 of 48 constructed cases are true double-rounding
  cases, 0 of the 10,000; its own measurement of the module on all 48: the 16 pins field for field), PM-0004-74's
  own case on four small workspaces with real cargo, and re-ran 23 plants (23 caught). Its answer to the advisor
  agent's question: every named hard-case class for `floor`, `ceil`, `trunc`, `round` is present, none missing.
  - Medium, `rules_scan.rs`: `arch` counts only right after `std::` or `core::`; `use core::{arch::wasm32::..}`
    and `use core as k; k::arch::..` pass, and the relaxed-SIMD calls then build with no flag. Fix: `arch` before
    `::` wherever it stands, names holding `_relaxed_`, two more bites; `cfg!(target_arch)` branches stay out of
    scope, said in the doc. **Taken in the round-2 pass.**
  - Lows: the test title and `wasm.rs`'s doc still say "exact" for WebAssembly (taken); the FMA line is printed by
    a passing test, which cargo hides (left as a follow-up: a test that asserts the instruction would fail on a
    processor with FMA4 only, where the result is exact; only the line's wording changed); CI runs neither generator's `--check`
    (a follow-up for E1's CI pull request: a CI file); `from_f64` has no exact tie and no case between the largest
    f32 and the overflow point (a STATUS follow-up); the fixture's comment says `-p tools` (taken).
  - Not verified by it: a processor without FMA; Firefox, WebKit, Safari; the toolchain's `fmaf` source; the
    devtools tests in release on CI (CI's release line is `-p sim -p protocol`); 20 runs on one commit.
- **Round 2, security-auditor (on 79666ad9; `security-auditor-review2.md`; read 15:51 UTC): REVISE, 0 Critical, 1
  High, 0 Medium, 3 Low.** Round 1's M1, L2, L3, L5, L6 confirmed fixed.
  - **High (L1 not fixed; "evidence that does not hold"): the dev server still reaches the test-only module.**
    `vite.config.mjs` still allows `../../node_modules`, and npm's link `node_modules/@shamble/sim-worker ->
    ../../packages/sim-worker` (seen here) puts `pkg-rules/` under it: its run gave 200 with the module's bytes
    through the link, and 403 only for the direct spelling the spec tries. Condition 2's clause (d) stays not met
    for the dev server. **The fix, once the round has ended: build the test-only module outside `packages/`
    (under `.gate/`), narrow the list to `node_modules/three`, assert the real place and a link spelling in the
    isolation spec, plants for both.** A third round on this fix is within the budget (an open High).
  - New Lows, in the same pass: N1 a control character in cargo's home path splits `CARGO_ENCODED_RUSTFLAGS`
    (refuse it in `registryRemapFlag`); N2 the comments that say a config-file flag "does not reach the compiler"
    hold for rustflags only (name the linker, the compiler wrappers, profiles and `[env]` as outside); its note
    that a relative `CARGO_HOME` is resolved against node's folder while cargo uses the root (resolve against the
    root). L-STATS stays its STATUS follow-up.
- **State after the fix pass 79666ad9: every round-1 finding marked "fix now" below is taken; round 2 confirms.**
  perf-auditor's round 1 (`perf-auditor-review1.md`): OK, 0 Critical, 0 High, 0 Medium, 2 Low,
  both taken (`doneAt` read once; the comment that said the report check allocates nothing).
- **MUL-ADD is decided: (a), his words of 15:58:25 UTC.** The ban, the 48 cases and the 16 pinned values stay as
  built. Two follow-ups for the report and STATUS (his Advisor's conditions): the check that the players'
  WebAssembly holds no fused multiply-add is built in the change-set that brings the first outside crate into the
  simulation (planned: change-set 4, the collision library), named in that plan; reporting the defect to the Rust
  project, never posted without his go. Part 7's dated pointer: at the draft's next revision, the sentence shown to
  him first.
- **Follow-ups this pass leaves** (for STATUS with the docs pass): step 2.3's lint carries his MUL-ADD answer; when
  an outside crate enters `sim`, a check that the players' Wasm holds no `fmaf`; the stricter lint's leftovers in
  `rules_scan.rs` and `rng.rs` (a long function, similar names; his quoted words keep their spelling); no plant
  for the six newly stubbed WebAssembly names or the Worker's start path; `module-no-hook` and `tel-not-a-file`
  are caught at their named test for a neighbouring reason; a config-file profile is outside the build's refusals.
- **Round 1, test-engineer (on 5c92311; `test-engineer-review1.md`, its scripts in `.gate/test-engineer-cs3a/`): 0
  Critical, 2 High, 1 Medium, 10 Low.** It re-ran 21 of the 66 plants (21 caught), checked the float digests with
  a third implementation (C) and the stream file with its own ChaCha, and found TE-L, TE-NT3 to -NT5, TE-MONO and
  TE-104 closed as meant; PM-0002-53 four of four; no new speed-dependent verdict.
  - **H1 (High): `mul_add` gives different bits natively and in WebAssembly, and the 10,000 cases cannot see it.**
    For (0x01000001, 0x32FFFFFE, 0x007FFFFF) the exact fused result is 0x007FFFFF; native here (an FMA CPU)
    gives it; the test-only Wasm, release and dev, gives 0x00800000, the two-roundings value. **Reproduced by the
    builder at 14:52 UTC** (the files' times) (`.gate/cs3a/hard/`: a scratch native probe and the module under Node). The toolchain's
    software `fmaf` is not the exact one; a server CPU without FMA probably gives the Wasm's value (not tested).
    ADR 0004 part 7 allows "explicit `mul_add`, which std specifies exactly": its premise does not hold on the
    pinned toolchain. Nothing in `sim` calls `mul_add`. Its fix: the constructed cases join the inputs and stay;
    never drop the case to get green; the rule is the advisor's or his. **The advisor agent ruled (a) on seven
    conditions (`advisor-mul-add.md`, written 15:07 UTC): the interim ban is built (step 11); the rule stays
    Mustafa's; 3a merges only after his own answer to MUL-ADD; an answer that changes 3a's code is a new head.
    Asked in the chat with the update after the fix pass, and again in the report. Round 2 confirms conditions 1
    to 4; F-MULADD then counts as fixed and MUL-ADD is a decision, not an open High.**
  - **H2 (High): the feature compare misses PM-0004-74's own named failure.** cargo prints a feature that another
    package turns on for a dependency it shares with `sim` under that other package, not under `sim`; the unit
    test's planted fixture puts it where cargo never prints it; normal edges hide dev-dependency features;
    "(command-line)" is stripped on one kind of line only. Fix: per build, every feature line of every package in
    `sim`'s dependency closure, from the whole tree; dev edges for the tests' list; real cargo output as fixtures.
    (engine-architect's F1 and rust-reviewer's M3 are the same check.)
  - M (Medium): the scan lets relaxed SIMD through (`#[target_feature]`, `core::arch`): ban both in `sim`, with two
    planted cases (PM-0004-23 names the relaxed-simd feature).
  - Lows: (a) `from_f64` = rust-reviewer's M1; (b) the vectors are f32 only: the script's first line claims too
    much; (c) PM-0002-53's counter is devtools' function in Wasm (accepted by DEPART-3 (a); say so); (d) record
    the draft's sha256 e0a49fdd...4d55 (76,754 bytes) beside the published vectors; (e) `next_u64` from an odd
    position across a block's end has no test; (f) two plants are caught at their named test but for another
    reason (`tel-not-a-file` by a broken connection, `module-no-hook` by a missing function); (g) no plant for
    the six new WebAssembly stub names or the Worker's start path; (h) = rust-reviewer's M4; (i) = the hook's
    test in its own binary (3b); (j) `step8.log` was a run on df72a58 plus the working tree: the final evidence
    run is made on the final head, with C2's versions and refused names.
  - Card sentences it does not carry: step 10's "PM-0004-74" (H2); step 7's "all 38 float operations passed"
    (true, but not bit for bit for `mul_add`: H1); "each at its own check" (f).
- **Round 1, security-auditor (on 5c92311; `security-auditor-review1.md`): 0 Critical, 0 High, 1 Medium, 6 Low. Its
  condition 2: clauses (a), (b), (c), (e) met; (d) not met for Vite's dev server.**
  - M1 (Medium, fix now; = rust-reviewer's L2): `runtimeCrates` takes any crate with a source in brackets for a
    workspace member; count a member only when the bracket is a folder under the repository's `crates/`; refuse a
    tree without its `sim-wasm` line; plants for both.
  - L1 (fix now: clause (d)): `vite.config.mjs` lets the dev server read the whole `../sim-worker` folder, so the
    dev server the isolation spec starts answers for `pkg-rules/`; allow only `../sim-worker/src` and `/pkg`.
  - L2: an empty or relative `CARGO_HOME` makes the remap do nothing (resolve it; empty counts as unset). L3: the
    panic reader refuses any room but 1,016. L5: the generators' run lines say `python3 -I -B`. L6: the refusal
    pattern compares names in upper case (Windows). Follow-up: L4 (the copy report's rows from the known list).
- **Round 1, rust-reviewer (on 5c92311; `rust-reviewer-review1.md`): 0 Critical, 0 High, 4 Medium (fix now), 7
  Low.** It reproduced the ChaCha8 file and the Wasm's sha256 e138cae8...2bc5 with its own tools, and read the
  machine code: on x86-64 `floor`, `ceil`, `trunc`, `round` and `mul_add` are calls into the toolchain's own
  compiler_builtins and `sqrt` is an instruction; on wasm32 `floor`, `ceil`, `trunc` and `sqrt` are instructions and
  `round` and `mul_add` call compiler_builtins inside the module. For the fix pass:
  - M1: `from_f64` tests nothing that `mul` does not (the compiler turns it into an f32 multiply; the two digests
    are equal). Fix: `f(f64::from_bits(wide(a, b)) as f32)` and a reference that rounds that double once;
    regenerate. Until then the count is 37 operations.
  - M2: `panic.rs`'s doc gives a wrong reason ("a panic may be a failed allocation"): std formats a message with
    run-time arguments into a new String before a custom hook runs, and an out-of-memory stop never reaches the
    hook on stable (it prints and aborts). Fix the doc; the report says an out-of-memory stop shows no message.
  - M3 (the same root as engine-architect's F1): the native tests' feature list is read without dev-dependency
    edges. Fix: read `sim`'s own features for that list from a tree with `dev` edges (the subtree stays on normal
    edges), with the reviewers' plant as a test; the doc says what is not modelled.
  - M4: the scan's rules let real constructs through (16 probes): a type alias of `f32`, `Self::sin` in an impl for
    `f32`, `<f32 as Trait>::powi`, a generic `F::sin`, a macro that takes the method's name, `#[path]` and
    `include!`, and hash maps under other names (`FxHashMap`). Fix: a banned name after `::` counts unless the
    segment before it is an integer type or `Ord`, `cmp` or `Iterator`; names that contain `HashMap` or `HashSet`;
    flag `#[path` and `include!`; the macro gap written in the file's doc.
  - Lows to ride the fix pass where cheap: L1 the remap through `CARGO_ENCODED_RUSTFLAGS` in the cargo child's
    environment (it shuts config files' rustflags out; a config-file profile stays "not covered", said in the
    note); L2 `runtimeCrates` counts a crate with a source outside the repository as third-party; L3
    `saturating_sub` in the hook's writer; L6 the escape no longer covers a whole `fn`, `impl`, `mod` or `trait`,
    `0..min(a, b)` is not a method call, line numbers after a backslash line break; L7 nits (a const assert on the
    seed's length, `wrapping_add` in the test allocator, docs). Follow-ups: L3's name section (a quarter of the
    players' Wasm: with "the table of build options"), L4 the generator copies its 64-byte block on every draw
    (with PM-0004-27's cost, change-set 8), L5 the hook's test in its own test binary (3b, as engine-architect's F6).
- **Round 1, engine-architect (on 5c92311; `engine-architect-review1.md`): 0 Critical, 0 High, 1 Medium, 5 Low; the
  five follow-ups EA-F8, -F10, -F13, -N3, -N6 closed as meant.** For the fix pass:
  - F1 (Medium, fix now): the feature compare's "native tests" list reads normal edges only, so a feature of `sim`
    turned on through a dev-dependency passes though `cargo test --workspace` builds it (measured in its scratch).
    Fix: the check's doc and the report say it compares normal edges only (dev-dependency features and CI's release
    line are not modelled), and the doc names ADR 0002 part 3, not part 4; the exact check goes with the quality
    gate step's G5 (a STATUS follow-up, with its plant).
  - F2 (Low): `#[wasm_bindgen(start)] pub fn start()` also puts a callable `start()` into the players' glue;
    `#[wasm_bindgen(start, private)]` removes it and the start still runs; plus one boundary assertion that the
    glue exports exactly `default`, `initSync`, `Sim`, `panic_buffer`, `tick_hz`. Take it in the fix pass.
  - F3 (Low): the trap's sentence is built in `host.ts` and in `worker.ts`: one function beside
    `readPanicMessage`. Take it in the fix pass.
  - F4 (Low): nine `clippy::pedantic` warnings in `rng.rs` and `rules_scan.rs` (the later lint of ADR 0002 part 7):
    doc backticks now where cheap, the rest a follow-up for the quality gate step.
  - F5 (Low, forward): `devtools` is both the native tools crate and the wasm32 module of every development build;
    its first native-only dependency must be target-gated: a STATUS follow-up, and a separate rules build step
    when CI's lines are next edited (E1).
  - F6 (Low, forward): the hook's test installs a process-wide hook inside sim-wasm's shared unit-test binary;
    move it to its own test binary in 3b.
  - Departures it names beyond the plan's section 12 list, for the docs pass (pointers, not rewording): PM-0004-74
    read as three builds over normal edges; ADR 0002 part 8's row "Rust in Wasm: wasm-bindgen-test" (line 494);
    PM-0004-27's "each stream" read as four edge stream ids; ADR 0002 part 5 (lines 353 to 357) plans a feature
    turned on only by dev-dependencies, which PM-0004-74's control would count.
- None on main. The follow-ups of #64 and #61: `docs/STATUS.md`, "Follow-ups", and the archive's two blocks.
- WORK-FOLDER (not asked yet): hand saves hold as long as the worktree's local name differs from the branch on
  GitHub; if they get heavy, ask him whether 3a may be built in the main folder, where the saver script works.
- The card's old rule "never read `/root/.claude/uploads/`" has no source in his words; a file he attaches lies
  there. Agents are still told to stay out of it.

## Key numbers
- main 1ec6f6c; #64's final head 8aadfb0, its code head 6d0b5b1, its notes' last commit 5911b56.
- 3a: about 2,050 (2,080) lines planned, stop line 4,100; **3,561 at 8619d539** (539 of room); about 281 KB
  of committed data (the vectors). The players' Wasm at 79666ad9: 31,718 bytes, sha256 383f5303...3963 (here and on
  the runner).
- CI today: one `toolchain` job, about 2 to 2.5 minutes; the Wasm on the runners 28,991 bytes, sha256
  22dad2ebf4be21ce0f123753bb77cfa25ca06886eddace43651601831ee8018b.
- This container (it can be replaced at any time): cargo 1.99.1, Node v22.23.5; no `node_modules` installed and no
  wasm-bindgen tool (the workflow's own curl line and checks bring it: TOOL-SOURCE). Untracked scratch of #64 under
  `.gate/ci-wasm/` in the worktree (the logs, the notes' copy, `advisor-final.md`): not needed to resume.

## Decisions taken
- 2026-11-24, 15:58:25 UTC: MUL-ADD (a), his; the stop line stays; plain words with him (above).
- 2026-11-24: MUL-ADD's interim, by the advisor agent: (a), the ban and the pins, on seven conditions; the rule
  itself is his (asked in the chat; the merge waits for his answer).
- 2026-11-24: his main Advisor marked #64 ready and merged it on his tap (the merge commit's message).
- 2026-11-23, 23:44:10 UTC: GH-NEXT (a) with his change; CLOSE-OLD (a); the order after the merge.
- KIT-QUOTE: the advisor agent ruled (a) (his words of 2026-11-13 stay in the archive and DECISIONS, with a pointer
  in STATUS); told to him with #64's ask, no answer needed.

## Running work
- **Round 2 has ended; round 3 is security-auditor alone (above).** What follows describes round 2:
- **Reviewers in the background** (round 2, on 79666ad9, only what changed since 5c92311): rust-reviewer
  (`rust-reviewer-review2.md`; with the advisor agent's question whether another allowed operation picks its
  native path by processor), test-engineer (`test-engineer-review2.md`; with its question on the hard-case classes
  for `floor`, `ceil`, `trunc`, `round`), security-auditor (`security-auditor-review2.md`; condition 2's clause
  (d)). security-auditor and test-engineer have ended; engine-architect and perf-auditor run (card 15:52 UTC). Each works on copies under `.gate/<name>-cs3a/`; the shared
  tree is read-only for them, builds included. No source file is edited while a round is being read. No saver.
- Prepared for the docs pass and the check, not yet run on the tree: `decisions-4.mjs` (his five messages of
  2026-11-23, 23:44:10 UTC to 2026-11-24, 11:03:53 UTC into DECISIONS, on top of main's copy; preview in
  `.gate/cs3a/DECISIONS.preview.md`), the two newly saved messages (`mustafa-2026-11-24T105043-compact-command.md`,
  `mustafa-2026-11-24T110353-compact-done.md`), and `make-zip.py` (the zip `shamble-v2-03a-rules-boundaries` with
  the README's seven steps; run it on the final head's development build).
- A second worktree `.gate/cs3a-batch` (condition 1's builds) may stay.
- In a folder outside the repository cargo is the default 1.97.0, not the pin: scratch projects use `cargo +1.99.1`.
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
