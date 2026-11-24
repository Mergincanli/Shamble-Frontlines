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

## Current step
**The reviews, round 1, on the code head 5c92311** (plan section 4: rust-reviewer, engine-architect, test-engineer;
then security-auditor and perf-auditor; Opus 5.5, effort max; at most 3 at a time; each with its notes in this
folder), started 13:56 UTC. Then one fix pass; round 2 on what changed; the verifier until PASS; the advisor agent
on the pushed head; docs-keeper; DECISIONS by script; the report; the zip and its README; the pull request's title
and text.

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
- **Round 1, test-engineer (on 5c92311; `test-engineer-review1.md`, its scripts in `.gate/test-engineer-cs3a/`): 0
  Critical, 2 High, 1 Medium, 10 Low.** It re-ran 21 of the 66 plants (21 caught), checked the float digests with
  a third implementation (C) and the stream file with its own ChaCha, and found TE-L, TE-NT3 to -NT5, TE-MONO and
  TE-104 closed as meant; PM-0002-53 four of four; no new speed-dependent verdict.
  - **H1 (High): `mul_add` gives different bits natively and in WebAssembly, and the 10,000 cases cannot see it.**
    For (0x01000001, 0x32FFFFFE, 0x007FFFFF) the exact fused result is 0x007FFFFF; native here (an FMA CPU)
    gives it; the test-only Wasm, release and dev, gives 0x00800000, the two-roundings value. **Reproduced by the
    builder at 14:53 UTC** (`.gate/cs3a/hard/`: a scratch native probe and the module under Node). The toolchain's
    software `fmaf` is not the exact one; a server CPU without FMA probably gives the Wasm's value (not tested).
    ADR 0004 part 7 allows "explicit `mul_add`, which std specifies exactly": its premise does not hold on the
    pinned toolchain. Nothing in `sim` calls `mul_add`. Its fix: the constructed cases join the inputs and stay;
    never drop the case to get green; the rule is the advisor's or his. **The advisor agent is ruling on MUL-ADD
    (started 14:53 UTC; `advisor-mul-add.md`): (a) an interim ban in the scan, the cases kept, the difference
    pinned by a test, and the question to him with the report; (b) keep part 7 as written; (c) stop and ask first.**
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
- **Reviewers in the background** (round 1, on 5c92311; engine-architect, rust-reviewer, test-engineer and security-auditor ended; perf-auditor still reads; the advisor agent rules on MUL-ADD; perf-auditor started 14:30 UTC, notes `perf-auditor-review1.md`; security-auditor started 14:21 UTC
  with notes in `security-auditor-review1.md`; perf-auditor follows): rust-reviewer
  (`rust-reviewer-review1.md`), engine-architect (`engine-architect-review1.md`), test-engineer
  (`test-engineer-review1.md`); each works in its own `.gate/<name>-cs3a/` folder. security-auditor and
  perf-auditor follow when a place is free. No source file is edited while a round is being read. No saver.
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
