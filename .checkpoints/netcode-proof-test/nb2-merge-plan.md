# NB2-MERGE: take Netcode Builder 2's branch into the netcode branch (plan, 2026-11-25 00:4x UTC)

## As approved
Not yet: the advisor agent's plan check (output advisor-nb2-merge.md) starts at 03:51 UTC with step 7 and M4b
(LEAN-MODE item 3: new agent work only in a reset's first 30 minutes).

## Why
Mustafa's word, 2026-11-25 00:07 UTC, item 4 (history.md:7909, word for word there). The lines this plan rests on:
- "merge claude/practical-ride-igo41n into your branch, both parts in one merge."
- "Before you merge, check that every commit after 72651a3c on that branch touches only its notes."
- "You choose the order with U6c's merge, R1's path goes first, and the two merges never overlap."
- "After the merge: your whole check (node spikes/netcode-proof/check.mjs), then the re-judge of the PM-0001-47 set on
  your real logs, which must read MOVED. If it reads anything else, stop and tell me in three lines."
Not part of this merge: item 5's take-overs (his 24 November message into docs/DECISIONS.md by script at the next docs
pass; NPT_KC_BURSTS after R1's run; the killed-room check before R5) and item 2's NPT-L2-ONEWAY (Builder 2's own
change-set after this merge; the pointers are passed).

## Order (his item 4 leaves it to me)
- After U6c's merge is done: M8's commit, C9, M9's push with CI green on its SHA (every check run success and PR #40
  mergeable: advisor-main-65.md section 4 (a)), u6c-report.md, the board line. Never during any U6c step.
- Then in a gap of R1's path: no R1-path step running, and none due to need the lane or the main folder before this
  merge's projected end. Projected about 45 minutes of lane: U6b's warm counted run took 23.5 minutes (05:38:21 to
  06:01:53 UTC, .gate/u6b-merge/m5-check2.log), plus the judge's build and two judge calls. The first such gap is
  expected between U6c's M9 and 08:50 UTC: R1's next steps are U7's 0b check and R-half, in a reset's first 30 minutes,
  then U8's merge with the advisor's head read (the card's "Next three", item 3, and its U8 line). A shorter gap: the
  merge waits for the next one. Nothing else of the lane runs during steps 1 to 8.

## Facts checked (00:2x-00:4x UTC, read only)
- Builder 2's tip, fetched by name at 00:4x: 0e621dae6e5b009a4cf00b6401b46d771b2215a3. 72651a3c is its ancestor, with
  5 commits after it (`git rev-list --count`); each one's `git show --stat` lists only `.checkpoints/netcode-judge.md`,
  `.checkpoints/netcode-judge/mustafa-2026-11-25.md` and `.checkpoints/netcode-judge/save-message.mjs`; together 3
  files, +96/-60 (`git diff --stat 72651a3c 0e621dae`). His condition holds today; step 1 checks it again.
- The merge base with HEAD (5e6711e, equal to the remote): e2c0e896dedd5c09c2536e2fbe9323353756fe99, the netcode head
  Builder 2 started from.
- `git diff --name-status e2c0e896 0e621dae`: 46 paths, +6,951/-208; 32 added, 14 modified, none deleted or renamed.
  22 are Builder 2's notes (`.checkpoints/netcode-judge.md` and `.checkpoints/netcode-judge/`, all added). 24 are under
  spikes/netcode-proof/ (+4,712/-208): crates/judge/ (src 7 modified, tests 3 modified, fixtures/README.md modified,
  10 fixture logs added under fixtures/pm-0001-30/) and marks/ (README.md, SERIES.md, marks.json modified). No manifest
  or lockfile.
- Kit and CI: the guard's full pathspec (main-61-plan.md:37-40) over e2c0e896..0e621dae prints nothing: no `.claude/`,
  CLAUDE.md, AGENTS.md, `.github/`, `tools/` or gate file. So the merge brings no settings, hook or agent file (Q1).
- This branch since e2c0e896 changes nothing under crates/judge, marks/ or `.checkpoints/netcode-judge*`
  (`git diff --stat e2c0e896 HEAD -- …` prints nothing). After U6c's M8 it changes marks/SERIES.md by U6c's 51 added
  lines alone (33 table rows; today's working-tree lines 909-959).
- SERIES.md: Builder 2's hunks sit at base lines 262-297 and 402-421, U6c's at 909-959. No series name is in both:
  `comm -12` of the first cells of Builder 2's 25 added or rewritten rows and U6c's 33 prints nothing.
- Trial merge on today's head: `git merge-tree --write-tree --name-only HEAD 0e621dae` gives tree
  cbb0d5206ab3d27781698b306ec76c21f8ff4948 and no conflict. Step 1 repeats it on the post-U6c head.
- U7 and U8: their patches touch no file under marks/ or crates/judge (`diff --git` headers: wip/u8-build.patch 43
  files, wip/u7-t-build-t5i.patch 23), so their later `git apply --check` does not depend on this merge's order.
- marks.json: this branch's is e2c0e896's (c1935a5b…); the tip's sha256 is
  c3396db225b394695b6cf41460723e1d98b96f42e1d42d412976f4ca702f1b7a (Builder 2's card). An older judge refuses it
  (unknown field `moved`), so judge and marks.json must come in together: one merge does that.
- The -47 set's raw logs: .gate/netcode-proof-test/runs/u4f-47-b57221ed139d/, unpacked, 51 MB, never packed: measured/
  (l1-47-s1 to s3) and bites/ (bite-newest-sample, bite-tick-quantised), written by c-u4f-runs.mjs at b57221e
  (results/u4f/b57221ed139d/pm-0001-47/runs.txt: two judge calls, measured and bites, U4r-F2's split).
- At the tip PM-0001-47 carries a mark-level `moved` (to "Slice 1 change-set 7 (Lag)", by "NPT-TRIM, 2026-11-21 (the
  re-plan, approved 2026-11-22)"); a mark-level moved returns MOVED early, its runs counted, not judged
  (replan-marks.md:198-203).
- The re-judge command (Builder 2's card, "Open findings"): `cargo run -q --locked -p npt-judge -- --marks
  marks/marks.json --logs <the -47 set's logs> --only PM-0001-47`, expected MOVED. `--only` judges against the whole
  marks file now (JDG-ONLY-1 fixed), so PM-0001-2 need not be named beside it.
- Expected counts: Builder 2's whole check went from 807/904 at e2c0e896 to 854/951 at 72651a3c (+47 each; the later
  commits change no code), self 78 of 78 at both. So after this merge: U6c's M5 counts + 47 each (908 and 1005 if M5
  reads 861 and 958), self as M5.
- Free disk at 00:4x: 10,255 MiB (`df -m --output=avail`). U6c's near-cold M5 runs first and uses some of it.

## Steps (each command in its own call)
1. Pre-checks (read only), then the last save before the merge:
   a. `git fetch origin claude/practical-ride-igo41n`; T = `git rev-parse FETCH_HEAD`.
   b. `git merge-base --is-ancestor 72651a3c… T` exit 0; for each SHA of `git rev-list 72651a3c..T`,
      `git show --stat --format= <sha>` lists only `.checkpoints/netcode-judge.md` or paths under
      `.checkpoints/netcode-judge/`. Anything else: no merge, three lines to him (his condition).
   c. `git diff --name-status e2c0e896 T`: only the three prefixes above (46 paths at 0e621dae); the guard's pathspec
      over e2c0e896..T prints nothing; no manifest or lockfile.
   d. `git merge-base HEAD T` = e2c0e896.
   e. `git merge-tree --write-tree --name-only HEAD T`: one tree line, no conflict; a conflict stops, the advisor rules.
   f. Free disk at least 8,192 MiB (advisor-u6b-m6.md:158-165); under it, the approved dev part-clean
      (u4r-disk-plan.md) and measure again; still under: stop, the advisor rules.
   g. The base light run (`main-run.sh … node check.mjs --only refs,marks,self,redact`) into
      .gate/nb2-merge/light-base.log: N0 (refs), the marks, self and redact lines.
   h. The saver stopped (it restarts at U6c's M9): TaskStop, then `pgrep -f "checkpoint.mjs watch"` exit 1.
   i. The card's resume line by Edit ("NB2-MERGE: step 2 begins at H0; if cut, the local merge commit is unpushed:
      finish steps 3-8 from this plan, never reset"), then the card save, the last until step 8. H0 = that save,
      equal to the remote, named in the chat.
2. `git merge --no-ff -m "<subject>" -m "<body>" T`, the body naming his word (history.md:7909), T and 72651a3c, and
   ending with the two attribution lines. Expected: exit 0, "Merge made by the 'ort' strategy."
3. Content checks (read only):
   - `git rev-parse HEAD^1 HEAD^2` = H0 and T.
   - `git diff --shortstat H0 HEAD` equals `git diff --shortstat e2c0e896 T` (46 files, +6,951/-208 at 0e621dae).
   - `git diff --stat T HEAD -- spikes/netcode-proof/crates/judge spikes/netcode-proof/marks/marks.json
     spikes/netcode-proof/marks/README.md .checkpoints/netcode-judge.md .checkpoints/netcode-judge` prints nothing.
   - `git diff --numstat T HEAD -- spikes/netcode-proof/marks/SERIES.md` reads 51 added, 0 deleted (U6c's rows alone).
   - `git diff --stat H0 HEAD -- .claude CLAUDE.md AGENTS.md .github tools` prints nothing.
   - `sha256sum spikes/netcode-proof/marks/marks.json` = c3396db2….
4. His whole check: free disk again (as 1f), then `bash .checkpoints/netcode-proof-test/main-run.sh
   /home/user/Shamble-Frontlines/.gate/nb2-merge/check.log env CARGO_INCREMENTAL=0 node check.mjs` in the background,
   timeout 7200000. Bar: all 11 steps pass; measured and bites = U6c's M5 counts + 47 each; 0 failed, 0 skipped; marks
   81 read, rows unchanged; self as M5; redact 0 findings; refs N0 + 32; wasm as M5.
5. His re-judge, in the set's two calls, each through main-run.sh (the lane lock; cargo builds the judge):
   - `bash .checkpoints/netcode-proof-test/main-run.sh /home/user/Shamble-Frontlines/.gate/nb2-merge/rejudge-47-measured.log
     cargo run -q --locked -p npt-judge -- --marks marks/marks.json --logs
     /home/user/Shamble-Frontlines/.gate/netcode-proof-test/runs/u4f-47-b57221ed139d/measured --only PM-0001-47 --out
     /home/user/Shamble-Frontlines/.gate/nb2-merge/rejudge-47-measured.json`
   - the same with `bites` in place of `measured` (log, logs folder and out file).
   - Bar: both calls' headline reads PM-0001-47 MOVED (to "Slice 1 change-set 7 (Lag)"), and both out files say MOVED
     for it; exit codes recorded. The logs are only read. Projected judge peaks 372 and 349 MiB (runs.txt's six-point
     fit) against the 10,240 MiB line.
6. `node tools/gate.mjs --no-smoke` PASS, against the merge-base with main.
7. Nothing is edited by hand: no rule, mark, number or file of Builder 2's.
8. The card save (it pushes the merge and the save); CI on the pushed head: gate, smoke and toolchain green and PR #40
   mergeable; the saver restarts; one board line (done, next, commit link) and one reply in Builder 2's MOVED thread
   saying the merge is in at that SHA, so its NPT-L2-ONEWAY can take this head in first.
9. The advisor's check of the pushed head by SHA (MAIN-65 step 7's form), in the next reset's first 30 minutes.

## Stop rules
- 1b fails: no merge; three lines to him. 1e or step 2 shows a conflict: nothing more; the advisor rules
  (`git merge --abort` is not used: the deny list holds `git reset --merge*`).
- Steps 3-6: any check off its bar stops and nothing is pushed. Step 5 reading anything but MOVED: three lines to him
  (his words), and the advisor at the next window. Step 4: a failing step or test goes to the advisor, no re-run on my
  own (U6c's M6 form); M4b's answer 2 (its timing-test and disk fallbacks), if given, applies here the same way.
- A stop leaves the merge commit unpushed, and a save would push it: Q2.
- CI red after the push: root cause first (the PR rules), never a skipped test.

## Risks
- Builder 2 pushes after 1a: the merge is by SHA T; its card says it waits for this merge, and its notes alone change.
- Another lane's notes in this branch: Builder 2's 22 notes files come into PR #40; PR #40's close drops `.checkpoints/`
  with this branch's own notes.
- U6c's timing-sensitive tests run again in step 4; that is the reason for step 4's fallback line.
- Disk: step 4 follows U6c's near-cold M5 and is mostly warm (the judge and what builds on it rebuild), so it needs
  less than M5; 1f and step 4 measure it anyway.

## Change type, tier, size
- A merge of another lane's branch, reviewed and approved there, nothing edited by hand: 46 files, +6,951/-208 (24 spike
  files, +4,712/-208; 22 notes files). Part a: test-engineer and rust-reviewer, then the advisor agent's APPROVE of
  0e0f6547; part b: the same two, a re-check of its one High, then the advisor agent's APPROVE of 72651a3c; his main
  Advisor checked 72651a3c (history.md:7909).
- His rule of 2026-11-15 (history.md:2127): every change to marks.json or the judge gets a reviewer's read before its
  unit closes, and the report names it in its own line with what covered it. Covered: the reviews above; this
  merge's report line names them.
- Risk tier 1 (spike code and notes; no kit, CI or gate file). Reviewers: the advisor (this plan; the pushed head).
  Mustafa: nothing to check in Brave.

## Questions for the checker
- **Q1.** CLAUDE.md's Never line counts `merge` as switching the main folder, for the reason that a branch's
  `.claude/settings.json` and hooks would load. Here the main folder stays on this branch, his word names the merge,
  and the merged branch changes no settings, hook or agent file (facts; step 3's empty diff proves it after). May the
  merge run in the main folder? If not: which route (a worktree at .gate/nb2-merge and then what)?
- **Q2.** If a stop in steps 3-6 leaves the merge commit unpushed: (i) no save until your ruling at the next window
  (R1's path waits that long), or (ii) `git revert -m 1 --no-edit HEAD` at once and the pair pushed, so R1's path goes
  on, the merge re-done later by reverting the revert. My recommendation: (i) by default; (ii) only if an R1-path step
  needs a save before your ruling.

## Record
- "Choices taken without him", in msg-2026-11-26.md: "Builder 2's merge after U6c's, in a gap of R1's path (his item 4
  left the order to me)".
- Not run here: the re-plan's real-data line also names "-57's folder still PASS" (replan-marks.md:207); his item 4
  names only -47. Offered to the checker, not planned.
