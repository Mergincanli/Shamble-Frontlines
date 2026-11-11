# Checkpoint: netcode-proof-test
- **Goal:** the netcode proof test (SPIKE_PLAN item 1), built unit by unit under its approved plan
  (`.checkpoints/netcode-proof-test/plan.md`, revision 3.3; approved by the advisor and Mustafa, 2026-11-06) · verdict:
  `.checkpoints/netcode-proof-test/advisor-verdict.md` · PR #40 (draft)
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`) ·
  **Base:** main, last merged in at 5ca77eb (main bb2d817, PR #49, the STATUS split)
- **Last update:** 2026-11-11 about 20:15 UTC (U4j merged: 4d2b0ef and d50396a, pushed together; the chat at 96,748
  of 1,000,000 (10 %) at 19:58 UTC, after Mustafa's /compact)

## Resume card (read this first; open the rest only when a step needs it)
**The HOLD is lifted** (Mustafa, 2026-11-11 about 16:45 UTC, verbatim in history.md "Decisions taken": "The hold
lifts when the save is pushed."): the save of both work folders is 4ad80a9. USAGE-SAVINGS (a) is in force with his
limits (Rules in force). The card's blocks of 2026-11-11 morning are in history.md, "Running work, U4 wave 1 (2)".

**The current step: U4 wave 1's close and wave 2's preparation.** Plan files, with the advisor's text batch of
2026-11-11 and U4j's merge text applied: plan.md (rev 3.3, sha256 2bef127b…), u4-build-plan.md (rev 2, 4b8eaab6…),
u4s-carries.md (carries 1-65, 32acca91…); the advisor's rulings: advisor-u4.md, advisor-u4-wave1.md (with B1.4's
erratum), advisor-u4j.md, advisor-u4d-close.md, advisor-u4j-r2.md.
- **Merged into the spike branch:** U4s f080551, U4b 411eeee, U4a 952f734, U4c f3f7c2f, U4d with W1-S1's npt-proto half
  39f4290 (report u4d-report.md); CI green on each. Main merged in at 5ca77eb (PR #49). **U4j: 4d2b0ef (the merge)
  and d50396a (round 2's wording), pushed together** (u4j-build.md, "Merge into the spike branch"). At d50396a:
  check.mjs 10 steps, measured 396, bites 443, self 52: the base counts for the next lanes, U4e's `<base>` included.
- **Running:**
  - the advisor's check of the U4e brief rev 3.2 (a general-purpose agent, opus, xhigh, in the background, started
    about 20:25 UTC; its brief advisor-u4e-brief-brief.md, its file advisor-u4e-brief.md; on its return read the file
    in full: it rules on a brief's tests and bites). If it is cut, start it again from its brief file.
  - U4d-F's run A, the build (task wmqlz92zb; run wf_3dbc5af1-0d4; script workflows/scripts/
    npt-u4d-f-build-wf_3dbc5af1-0d4.js, started about 20:45 UTC): four fresh builders in turn in
    `.claude/worktrees/wf_9ce88fa3-452-1`, groups 0-1, 2-3, 4-5, 6-7 (with the End checks); logs under .gate/u4df/;
    notes u4df-build.md in that worktree. If cut: resume with `Workflow({scriptPath, resumeFromRunId})` (finished groups
    replay from cache; a cut group's fresh builder reads the notes and the diff first). On its return: read every group's
    result; then cut the packet (`python3 -I .checkpoints/netcode-proof-test/review_packet.py
    .checkpoints/netcode-proof-test/u4df-packet.json`) and start run B, the review.
  - Done meanwhile: TE2-5 (b), ten of ten runs exit 0, each 443 passed, 0 failed, 12 ignored (te2-5-record.md).
  - the saver, restarted after U4j's push (about 20:15 UTC, 110 minutes): `node .claude/scripts/checkpoint.mjs watch
    netcode-proof-test --trailer "Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>" --trailer "Claude-Session:
    https://claude.ai/code/session_0173vVo6igCNyQYBWPvhH36A"`. Stop it before a merge's apply and restart it after the
    push, as at U4j's.
  - U4d-F's run A starts when TE2-5's runs end (its route: the U4d-F block below).

**The three cut workflows, and what to do with each when the hold lifts.** Scripts are under
/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/. A resumed
run (`Workflow({scriptPath, resumeFromRunId})`) keeps its run id; finished agents replay from cache, and only the cut
ones run again.
1. **U4j, the judge and marks change: MERGED** (4d2b0ef, the merge; d50396a, round 2's wording; pushed together;
   run wf_0906017a-a9b finished). Round 1 (three REVISE; F1 High), the fix pass (FX-1 to FX-14), round 2 (three
   APPROVE WITH FINDINGS; F1 closed by all three; no round 3), the advisor's round-2 ruling (advisor-u4j-r2.md:
   APPROVE on M1-M9), then the merge: M1 66 OK, M2 true, M3 64 files (the snapshot equal to 4ad80a9's), M4 64 OK, M5
   and M8.2 measured 396, bites 443, self 52, M6 no failure, M7 gate PASS with --no-smoke (this container's Chromium
   does not match tools/'s Playwright). The record: u4j-build.md, "Merge into the spike branch" and "Readings, as
   ruled". The old block: history.md, "Running work, U4 wave 1 (3)".
   - Open from U4j, as routed: carries 61 (§U4e), 62 and 63 (§U4r), 64 (the next change to crates/judge), 65
     (§U4d2, or the main session sooner); C-U6-9 to C-U6-11 (plan.md's U6 row); the FU list for PR #40's report (the
     closing checklist); TE2-5 (b) (next steps; L-c8's re-run done, 9cd7c39); one more reading only if U4e's client
     drops stale groups (advisor-u4j-r2.md B5).
2. **The U4e builder brief, rev 3** (task wnwulchr5; run wf_c92cd986-072; script npt-u4e-brief-r3-wf_c92cd986-072.js).
   - Done: the draft, u4e-brief.md (719 lines; notes u4e-brief-r3-notes.md): W1-U4E's answers, W1-S1, W1-N2, W1-TARGET's
     lock, the carries re-quoted by script, 8 of rev 2's 28 recheck rows changed and R29-R38 added, two new questions
     (U4E-Q12, who plants -22/unbounded; U4E-Q13, the owner-to-all bite's id).
   - Resumed about 17:55 UTC (task wwyzh9w7f) for the test-engineer's check (u4e-brief-r3-check.md), then one
     revision. Before it, the main session folded the brief by script: 3.1 re-copied from u4s-carries.md §U4e (carry
     51's corrected span, carries 53-57), 3.2 rewritten to record advisor-u4d-close.md's rulings, 7.2 item 3 done, a
     note under the title. The check's prompt no longer carries the drafter's summary; a SINCE note lists what changed
     after the draft (the draft call is unchanged, so it replayed from cache).
   - Then: fold advisor-u4d-close.md's U4e carries 53-57 (with the text batch) into the brief's carries section; the
     advisor checks the brief and answers Q12 and Q13; fill `<base>`; U4e's build. U4e need not wait for U4d-F.
3. **U4d-F, the estimate scene fix** (task wg0l1ugq1; run wf_9ce88fa3-452; script npt-u4d-f-wf_9ce88fa3-452.js).
   - The approved plan: advisor-u4d-close.md B1.1-B1.3 (steps 0-7; the files and symbols allowed; the measurement on 3
     seeds as the acceptance). No setting works → stop and bring the numbers to the advisor; the 9.5 floor never moves.
   - Cut early: its builder ran about 9 minutes in `.claude/worktrees/wf_9ce88fa3-452-1` (branch u4df-build at
     dad9566). Its notes (.checkpoints/netcode-proof-test/u4df-build.md there) hold the plan and an empty Progress;
     world.rs has an uncommitted partial edit.
   - **Do not resume the run as it is:** its builder uses `isolation: 'worktree'` and would start again in a fresh
     copy. Edit the script: drop the isolation; the builder works in `.claude/worktrees/wf_9ce88fa3-452-1`, reads `git
     diff` there first, completes or undoes the partial edit by Edit, and continues from its notes. Then the review
     (netcode-architect, test-engineer) as scripted.
   - Saved: wip/u4df.patch (4ad80a9: world.rs on dad9566, applies cleanly) and u4df-build.md. Under USAGE-SAVINGS the
     builder runs as fresh copies, one per step group of B1.2 (0-1, 2-3, 4-5, 6-7), each reading its notes and the
     diff in the worktree before it changes anything; pass or fail by the exit code and the test runner's own summary
     line, never a grep; long output to files under .gate/. Its review starts from a packet cut by review_packet.py.
   - It merges before U4d2's brief is final; U4d2 pins only U4d-F's recordings (carry 52).
   - **The new route (2026-11-11 about 20:30 UTC), in place of resuming wf_9ce88fa3-452:** run A, the build: a new
     workflow, four fresh general-purpose builders (opus, xhigh) in turn in that worktree, no isolation, one per step
     group (0-1, 2-3, 4-5, 6-7 with the End checks), each reading the notes and the diff first; every cargo and
     check.mjs run through u4df-run.sh (the lane lock, shared, or `-x` for counted, check.mjs and bite runs; logs under
     .gate/u4df/); TE2-5's rule and B6's plant rule in its brief. Then the main session cuts the review packet
     (review_packet.py with a u4df-packet.json, diffs against the base copy .gate/u4df-prefix, a worktree at dad9566).
     Run B, the review: netcode-architect read only, then test-engineer proves, from the packet; their agent types have
     no Edit tool, so each rewrites its notes whole with Write and saves each planted file's diff beside its run (B6).
     Run A starts when TE2-5's ten runs end (a builder waiting on their exclusive lock could hit its Bash timeout).

**The advisor's text batch: APPLIED by the main session (2026-11-11 about 16:50 UTC, no agent, as Mustafa asked after
his /compact).** `python3 -I .checkpoints/netcode-proof-test/text_batch.py` (its dry run first) copied every block and
every quoted string from advisor-u4d-close.md by line range: anchors checked unique, every new text found in place,
changed prose lines within 120 characters. Its log lists the 29 edits: carries 52's tail, 51's span, 53-57 (§U4e),
58-60 (§U4r), F3 (40), F7 (:14, 13); B1.4's erratum; B1.5 and F5, F6, F9-F11 in u4-build-plan.md; F4, NA1, NA4, NA5,
C-U6-8 and TUNE-1's header in plan.md; B1.6 in u4d-report.md, beside the main session's own line on the answers.
Carries 40, 51 and 52 were rewrapped within the entry, his words' label lines untouched (52 once more by hand, so that
"tick 0." stays on one line).
- Still open from that ruling: a test-engineer check of the applied batch once agents are allowed (the script's log
  is its brief); B4's TE2-TE4 plants, a day-session run on 1ccff06's code with the skeptic's cargo lines once the
  shared target is quiet (a plant that already fails a test drops out of C-U6-8); B4's Lows (TE5-TE7, NA2, NA3, NA6)
  as follow-ups for U3d's part of PR #40's report; B2's U4g and U4h notes stay follow-ups (named in u4d-report.md).

**Next steps, in order (Mustafa: "follow the card's order: U4j's round 2, then the U4e brief's check, then U4d-F"):**
1. **U4j: MERGED** (4d2b0ef, d50396a; about 20:10 UTC), M1-M9 met. **U4c's L-c8 judged re-run: done** (9cd7c39;
   u4c-lc8-rerun.md): U2's script at 6586991; the five L1 parts PASS with every check's outcome and value equal to
   U2's; the four bites fail exactly U2's checks; the hash half holds through U4c's scratch (equal whole-v2-log
   hashes), since npt-lab still records only the whole log's hash (U4c's netcode F2, Medium, open; a carry for U4d2,
   crates/lab's lane in wave 2, goes to the advisor with the U4e brief's check). **TE2-5 (b): done** (advisor-u4j-r2.md
   B2.3; te2-5-record.md): the bites build's workspace tests ten times with `--no-fail-fast` (te2-5-runs.sh), 20:20 to
   20:41 UTC: every run exit 0, 443 passed, 0 failed, 12 ignored. None failed in ten; the cause stays unknown, and
   TE2-5's rule stays until carry 65 is met or a failure is caught by name.
2. The U4e brief: **check and revision done (about 18:35 UTC; read in full).** The check said FAIL with 3 High (F1: the
   main session's fold reached 3.1 and 3.2 only, not e12, 2.6, the return list or carry 51's route; F2: the bite
   proofs as worded clash with a green check.mjs, so wave 1's two-part form is stated; F3: the owner-to-all proof was a
   group check that cannot fail on the open route, so carry 53's per-record test is the proof), 5 Medium (F4 carry
   22's accepting edges; F5 e7's exercise counts; F6 the three room-side checks' rules and clean negatives; F7 e5a's
   gap alignment; F8 routes for carries 54 and 56) and 5 Low. The revision made rev 3.1 (893 lines): all 13 fixed,
   none rejected; a new question U4E-Q14 (does a 1 s gap still evict B on Loopback; recommended (b)). Next, now that
   U4j's merge added carry 61 to §U4e: re-copy §U4e into the brief's 3.1 by script and redo its R28 row ("U4j has not
   merged") on the merged code (the advisor's conditions, advisor-u4j-r2.md B2.1 and B5); then the advisor's check of
   the brief (U4E-Q12, Q13, Q14 and the High fixes); `<base>` (the head at launch, d50396a or later; base counts 396 /
   443 / 52); U4e's build.
3. Then U4d-F (workflow 3) as above; then U4d2's builder brief (carrying carry 52, N2-M2's ruling, W1-TARGET), with
   the advisor.
4. U4r's brief: the night's rev 2 at `cs/npt-u4r-brief` ed973c9 goes to the advisor after U4c, U4d2, U4e and U4j merge.
- **Done for the report of 2026-11-11 evening:** the table per reviewer role (review-roles-wave1.md: findings rated
  Medium or higher only that role found in wave 1, from the notes, no agents): netcode 8, engine-architect 3,
  test-engineer 7, rust 0, security 0 (U4j's round 2 not in it). The panels stay as they are until he decides. Every
  report gives the chat's size.
- **Tonight (NIGHT-TONIGHT (c)):** one item on issue #16, item 6 npt-text-batch-check (plan night-2026-11-12.md,
  brief text-batch-log.md): the test-engineer's check of the applied text batch, with one line each on R-31 and
  U3D-MISS's sampling case. Fold it in the morning. PR #40's report lists those two edits as readings for Mustafa to
  confirm (closing checklist). The 19:30 UTC check-in (trig_01W9mBGauhxrWnn1rSuweDic) is deleted: its question is
  answered.
- **Housekeeping later** (approved method: leftovers to a local branch, then a plain `git worktree remove`):
  `.gate/u4a-prefix`, `u4b-prefix`, `u4b-wt`, `u4c-prefix`, `u4d-prefix`, `u4d-s1`, `u4j-prefix`; `.claude/worktrees/`
  agent-a9297644e7c0354f3 (U4a), wf_f54246ff-e0d-1 (U4d), wf_f54246ff-e0d-2 (U4c), agent-a3b8dfeb01ed52b50 (U4j,
  merged at 4d2b0ef).

**Open questions for Mustafa:** none open.
- Answered today (verbatim in history.md): NIGHT-BUDGET (a); the drift fix before U4d2's pins (carry 52); the effect
  row's owner deferred to U4e (carry 51); U4j's F1 by the review budget; the HOLD, lifted at the save (4ad80a9);
  USAGE-SAVINGS (a) with his limits; NIGHT-TONIGHT (c), one item.

**Rules in force** (his words verbatim in history.md "Decisions taken" unless marked)
- **Mustafa, 2026-11-11:** USAGE-SAVINGS (a), all three, with his limits: builders' pass or fail from the exit code
  and the test runner's own summary line, never a grep, and a fresh copy reads its notes and the diff in its worktree
  before it changes anything; the main session reads in full every finding rated Medium or higher and everything that
  touches the judge, the marks or a pass bar; the review packet is cut by script, holds no conclusion of the builder or
  the main session, ends with the ids of the carries it left out, and reviewers stay free to read any file and run
  any test; the chat's size in every report (get_session, context_usage), and he types /compact when told it is at
  35-40 %, after the card is refreshed. NIGHT-BUDGET (a) (switch to (b) on the first night whose queue does not fit,
  and tell him); the 9.5 floor does not move; a Low never triggers a push; U4j's F1 open after round 3 → stop and
  bring it to him; STATUS keeps one #40 line; his words move between docs by script and line ranges, never retyped;
  the review panels stay as they are until he decides.
- **Mustafa, 2026-11-10:** RESUME-CARD; WIP-CLEAN; SIZE-CHECK (three lines when a unit grows well past its plan);
  TCP-FREEZE; NETEM-BAR (a); SQUASH-40 with SQUASH-CITES (a) (the closing checklist under Next steps); PROMOTE at
  U4's close; the second lane: merge main as usual, by the plain merge the guard allows.
- **Mustafa, 2026-11-09:** the night shift's role (post the next night items each evening; fold them in the morning;
  building and testing stay by day); one shared Cargo target `.gate/shared-target` (touch loop, provenance, counts);
  LEFTOVER-DELETE: "leave them".
- **Standing:** write files with Write and Edit, never cd plus a redirect; never switch the main folder (the guard
  merges only origin/main into it); CHECKPOINTS.md (3 agents at most; save as you go); the review budget and one
  severity scale; Opus at xhigh for netcode; the advisor is a general-purpose agent that reads `git show
  origin/main:.claude/agents/advisor.md`; commits end with the Co-Authored-By and Claude-Session lines.
- **U4:** the five briefs (U4e, U4r, U4f, U4g, U4h) go to the advisor before their builds; carries word for word from
  u4s-carries.md, listed as met by number in each report; the wave-1 seam freeze (lifted for W1-S1 only); wave 2's
  builds take W1-TARGET's flock lock (the main session's counted runs too: main-run.sh holds it exclusively).
- **The advisor's U4j round-2 ruling (advisor-u4j-r2.md), for the main session and every builder brief:**
  - TE2-5's rule, until carry 65 is met (B2.3, its words): "a FAIL in check.mjs's test step is never retried into
    green unnamed. The failing build's crate is re-run at once through the runner with `--no-fail-fast`, its whole
    output saved under .gate/, and the failing test's name and panic go into the notes, tagged TE2-5. A failure in a
    crate the unit changes, or in a test that reads its files, stops that unit's merge until it is explained."
  - B6: plants by Edit in place, or the planted file's diff saved beside the run's output; the next review workflow's
    agents get the Edit tool, or are told to rewrite their notes whole with Write from the start.

**Pointers**
- Plan: plan.md. U4: u4-build-plan.md, advisor-u4*.md, u4s-carries.md, the unit notes (u4*-build.md, u4*-review*.md,
  u4*-round2-*.md, u4d-report.md, s1-proto.md).
- History: history.md (Steps done U0-U3d; Decisions taken, newest first; Running work up to U3c; "Running work, U4 wave
  1" (the card's blocks of 2026-11-10) and "(2)" (2026-11-11's, moved at the HOLD)).
- The closing checklist for PR #40: under Next steps.
- Usage: usage-20261111.md (the 24-hour split and the three savings; script usage24.py). The text batch:
  text_batch.py, its log text-batch-log.md. Review packets: review_packet.py with one JSON config per round
  (u4j-round2-packet.json). Tonight's night plan: night-2026-11-12.md.

## Steps done
Moved to history.md (RESUME-CARD, 2026-11-10): every step from U0 to U3d, with its SHA.

## Current step
Superseded by the resume card at the top; the old text (U3a era) is in history.md.

## Next steps
**Now (2026-11-09, about 22:25 UTC):** wave 1 of U4. Agent slots (3 at most, reviewers included) in this order as they
free:
1. **The workflow wf_f54246ff-e0d runs its own lanes** (U4d, U4c): builder, then reviewers, then a test-engineer.
   When it ends, fold each lane's report and notes and run the fix pass within the review budget (two rounds at most).
2. **The first free slot outside it: U4j's builder.** The brief is ready in `u4j-brief.md`: its "Prompt" section,
   with `<base>` set to the branch head at launch. It runs as general-purpose, opus, xhigh, with
   `isolation: worktree` (U4S-BUILDER).
3. **When U4a's builder (agent a9297644e7c0354f3) returns: one review round for U4a and U4b together.**
   netcode-architect, rust-reviewer and test-engineer each cover both diffs: U4a in its worktree, U4b in
   `.gate/u4b-wt` (record u4b-build.md).
4. **Merges, one at a time, on committed state:** U4b and U4a after their review; U4d and U4c after their lane; U4j
   after its review (test-engineer, netcode-architect, security-auditor). After each merge, check.mjs and the gate,
   with counts as deltas.
5. **Morning:** fold the night items (issue #16 items 2-5: the cs/npt-* branches' notes) per "About 22:05 UTC" in
   Running work.
6. **Then wave 2:** U4d2 and U4e. U4e's brief comes from the night draft and goes to the advisor before its build.
**Closing checklist for PR #40** (written 2026-11-10 by Mustafa's SQUASH-40):
- **PR #40 merges by squash** (SQUASH-40: "Its history carries about 28 MB, most of it checkpoints").
  - **Conflict to settle first:** plan.md:1436 says "merges with a merge commit, never squashed". That rule comes from
    the advisor's finding 4 on #45: ADR 0001 and docs/design/ws-reconnect.md on main cite bf9d248, 975e72b, 95a04a1
    and 1e020a7, which exist only on this branch.
  - **Decided, SQUASH-CITES (a) (Mustafa, 2026-11-10):** in the closing commit, re-point the four cites to files in
    the final tree, with the advisor's check; plan.md:1436 changes to match.
- **No .checkpoints/ in the final tree:** fold the checkpoint into the report and delete `.checkpoints/netcode-proof-test*`
  in the last commit before the PR is marked ready (CHECKPOINTS.md §9).
- **No wip/*.patch left** (WIP-CLEAN: each one is deleted in the commit that merges or replaces its builder).
- **PROMOTE's one-pager** (engine-architect) at U4's close, not before. Moving crates "as they are" would reverse
  SPIKE-SCOPE (a) (plan.md:347; §4.9 :653-662, "the foundation writes its crates fresh"), so that decision is
  Mustafa's, with the advisor's view.
- **Readings for Mustafa to confirm** (his NIGHT-TONIGHT (c), 2026-11-11: "PR #40's report lists those two edits as
  readings for me to confirm"): R-31's new wording (plan.md:1424, NA1) and U3D-MISS's sampling case (plan.md:1248, the
  U8 row), each with the night check's one-line reading on whether any pass bar's number or direction moved.
- **U4j's part of the report** (advisor-u4j-r2.md B5, :269-275), by id, severity and owner: the open security
  findings, which keep PR #40 needs-mustafa: FU-1 (SEC-F7, Low), FU-2 (SEC-F9's later half, Medium), FU-3 (SEC-F10,
  Low), FU-4 (SEC-F12, Low), FU-5 (C3's flag, Low) and FU-7 (SEC-R2-N2, Low); FU-6 (the advisor's A-U4J-1, Medium:
  the message-size condition), open; round 2's follow-ups and carries, entries 61 to 65 and C-U6-9 to C-U6-11; the
  review count: two rounds, F1 (High) closed in round 2, no round 3. The readings PR before R5 carries the orphan
  reading (u4-build-plan.md §10, §11: nine readings), plus one more only if U4e's client drops stale groups.
**Older next steps** (U1 era, kept for the record):
1. On Mustafa's "merge #41": check gate and smoke green on 1b4094a, merge with that SHA as the expected head; then
   `git merge origin/main` into this branch (only reviewed main comes in) so marks.json's row hashes match; remove
   the worktree `.gate/npt-readings`.
1b. The catalogue's questions Q1-Q9 (netcode-architect's notes `netcode-architect-catalogue.md`) and test-engineer's
   marks questions go to the advisor together, in one request.
2. U1 part 2: npt-log's record types and writer; npt-judge with its PASS, FAIL and INVALID fixtures (PM-0006-14's
   short padding among them); marks.json (test-engineer drafts it per `marks/README.md`, with SERIES.md); the drift
   test (after the merge); the dependency-rule checks (a cargo metadata test in npt-lab) and the no-reference grep;
   redact-check; the message catalogue (netcode-architect) and its class audit test; then docs-keeper (REPO_MAP gets
   spikes/ and the catalogue; STATUS: PR #38's line, the Mac line at 44-45, the plan's approval, UNDER-WAY (a)).
3. For Mustafa, before R1: Chrome and Firefox (stable) on the laptop; MSI GPU switch on hybrid if offered.
4. Write files with the Write and Edit tools only, never shell redirects or scripts that write (Mustafa, 2026-11-06:
   the guard can't follow cd, so such writes ask him); run commands from the main folder without cd.

## Open findings
- **From the advisor's U2 review (advisor-u2.md), each with the unit that fixes it:**
  - **C1 (U4, before PM-0001-18's L1 leg counts):** L1's stream retries back off (each waits twice the last timeout:
    RFC 6298 §5.5, RFC 9002 §6.2.1); loss drawn per 1,448 B segment, any lost segment delaying the message one timeout
    and holding back the ones behind it (E15's L2 model); unit tests; L1 runs record each stream's stall share.
  - **C2 (U4, before npt-room reads `rtt_ns`):** a sample only from a packet that is the newest of its link and
    direction to arrive (RFC 9002 §5.1); a stream message's sample from its own packet's arrival, not its in-order
    delivery; tests for both.
  - **C3 (U6; moved from U4 by the advisor's Q6 ruling, 2026-11-09, advisor-u4.md):** `loss` on Link, with §4.4's
    send-buffer space settled at the same time, joining with its first consumer, rate adaptation; each backend reports
    its own estimate, never the simulator's true fate; plan §4.7 then.
  - **C4 (U4, with the next judge change):** INVALID, with fixtures: a measured run (or its twin) whose build lists
    `bites` or names a bite; a bite run whose build lacks `bites`.
  - **C5 (U3/U6):** ImpairSocket's reorder step uses L1's model; SERIES.md names the binary that writes L2's probe
    log, not only the npt-net library.
  - **Note (U6):** links are `Box<dyn Link>` without `Send`; if npt-server moves links across tasks, the L2 backends
    need `+ Send`.
- **From the advisor's U4 verdict (advisor-u4.md, 2026-11-09), carried past U4:**
  - **C-U7-1 (U7):** U4d2's native-against-Wasm check does not cover wasm-opt, while ADR 0002:500-505 checks "the
    exact file G6 ships"; U7 reruns U4d2's check on npt-hash-wasm passed through the same wasm-opt step and flags as
    the shipped npt-client-wasm, or states why not (condition 8; also in plan.md's U7 row).
- Review round 1's 49 findings: fixed in revision 2 (§0.1). Review round 2's 46 findings: fixed in revision 3 (§0.2). The advisor's C1-C12: folded into revision 3.1 (§0.3).
- **Carried forward from the advisor's U1 rulings** (advisor-u1.md): C4 (`params.killed` only with npt-super's kill
  record) and C6 (tokens only in their prefixed form, with a test) at U6; A3's systemd readiness check for real at
  R1's bring-up, before any PM-0001-16 start counts; ImpairSocket's release precision at U3 (worse than 0.25 ms goes
  back to the advisor before R1); the results ledger (invalid runs superseded only with a written cause; failing
  runs only by a build fixing the found cause; the judge refuses a folder missing a listed run) at U8, reviewed by
  the advisor; Q13 (R4 creates LG with its IPv6 /64) and Q23 (-11 and -25 once per WebTransport crate) in the run
  plans; Q15 (a), -27's one-sided "within 5 %", in a readings PR before R5's first counted run (Mustafa's go).
- **For the readings PR before R5, also:** docs/design/ws-reconnect.md's WS-RECONNECT-BAR (b) record (Mustafa,
  2026-11-08), already on the spike branch.
- **For the readings PR before R5** (with Q15 (a)): the PR Advisor's Low note on PM-0001-31's "L2's TCP proxy on RS",
  which R-25 makes the WebSocket stream wrapper for the proof test: a pointer in that row.
- **Raised by the session, for the advisor at U4:** marks -14, -16 to -25 and -27 run inside an S30 room or the stress
  scene by their rows, but their parts do not select on the scene, so `pre:PM-0001-2` is not on them yet. When the
  L1 runner and the run plans name each run's scene, their selects add scene and clients, and the precondition with
  them.
- **A nuance, recorded:** an expect-invalid bite counts as shown if its rule reports a missing counter too; that
  cannot give a false PASS, because measured runs without the counter are INVALID by the same rule.
- **The flaky loopback P2 test (wt_tests.rs:203), measured 2026-11-07** (the record, since .gate/ is gitignored):
  - **How:** an instrumented throwaway worktree, .gate/flaky-wt at 2ec1533, printing per run quinn's ConnectionStats
    (frame_tx/frame_rx.datagram, PathStats) and ImpairSocket's run summary. Seed 5, P2, 200 datagrams each way at
    100 B, release build with both features.
  - **5 ms spacing (200/s, as the test is today), 80 impaired route-runs:**
    - 1 run under 180: wtq up 178. The client's quinn sent 200 frames; ImpairSocket lost exactly 7 up packets and
      dropped nothing else; the server's quinn received 178 and the app got all 178. So 22 datagrams rode in 7 packets.
    - That run had 75 up packets reordered, 17 congestion events, 32 packets quinn called lost, client cwnd 2,904 B.
    - Over all 80 runs: quinn's client "lost" 24-52 (median 38) against 6-8 real; congestion events 12-28 (median 20);
      cwnd 2.9-4.6 KB.
    - Up received: wt min 189, median 195; wtq min 178, median 195. Down: min 193, median 198.
  - **33.3 ms spacing (the tick), 50 impaired route-runs:**
    - 0 under 180; up min 191 (wt) and 192 (wtq), median 193-194; down min 197.
    - quinn's "lost" 15-32 (median 21) against 8-10 real; congestion events 4-10 (median 8); cwnd at its 2,904 B
      floor.
    - About 23 up packets are still reordered per run: ACK-only and other small packets sent between the datagrams.
    - In every run, both directions, the datagrams missing were never more than ImpairSocket's real losses (the
      advisor checked).
  - **Cause:** independent per-packet jitter (P2: +-10 ms each way) at 5 ms spacing reorders packets. quinn's
    packet-threshold detection declares spurious losses and cuts cwnd. Datagrams queue and are packed several per
    packet, so one real loss takes several datagrams.
  - **The advisor's ruling (afa86ecfb9b00e463, "Side ruling" in advisor-u3c-plan.md): APPROVE the fix** — pace the
    impaired P2 runs at TICK, keep the clean reference at 5 ms, keep >= 180 — **with four conditions:**
    - FT1: the sender twin of the new assertion (each sender quinn's DATAGRAM frames sent = the app's sends);
    - FT2: an exact check beside >= 180: per direction, datagrams missing <= ImpairSocket's real losses;
    - FT3: these numbers in the checkpoint (done here);
    - FT4: a comment at the pacing on why the impaired run uses the tick.
  - **Apply after U3c's build A merges.**
- **C6 widened (U6, before R3; the advisor 2026-11-07, (a)):**
  - **What it runs:** 30 Hz, snapshot-sized datagrams down (about 1,000 B) and input-sized up, at P1-P4 and B1, with one
    FullState on its stream mid-run.
  - **What it records:**
    - quinn's cwnd (minimum and median), lost_packets and congestion_events;
    - the datagrams quinn dropped from its send buffer (app sends minus frames sent);
    - ImpairSocket's real losses and reorder counts;
    - delivery delay p50 and p99.
  - **Plus one comparison run:** a test-only ImpairSocket variant that keeps each flow in order (each delivery at the
    later of its own time and just after the previous packet's), to size what the model's reordering costs WebTransport.
  - **Why:** at 30 Hz the client's cwnd already sits at its 2,904 B floor with 100 B datagrams; with 1 KB snapshots at a
    100 ms RTT about 3.1 KB is in flight, so a floored window would limit snapshots. This ties in with U6-1 (a small
    datagram send buffer).
- **JITTER-ORDER: parked; possibly Mustafa's before R3** (the advisor's (b), 2026-11-07). Not an escalation today.
  - **The model as written:** independent jitter on back-to-back packets is the intended model. ADR 0001 part 13
    (:569-584) gives each direction "a uniform jitter of ± half its jitter" (OQ-0001-16), and :1471-1472 says
    "Reordering is what the simulator produces under the profile's jitter". PM-0001-3's bands check exactly that, with
    R-5's step on top, so no unit changes it on its own.
  - **Where its premise falls short:** part 13's "jitter alone cannot reorder 30 Hz inputs" holds for the inputs
    themselves, but misses QUIC's ACK-only, probe and stream packets, which do reorder.
  - **The advisor's reading, not verified:** on real paths, delay variation comes mostly from queues, which keep order.
  - **When it goes to Mustafa:** an order-keeping model would change an ADR profile definition, so it is his call. It
    goes to him only if C6 shows the model's reordering moves a WebTransport result (-17, -18's 500 ms, snapshot delay
    or drops), before R3, with numbers and the advisor's recommendation. If nothing moves, it is recorded and the model
    stays.

## Key numbers
- Hetzner, excl. VAT, read 2026-11-05 (infra digest D): CCX23 EUR 0.1386/h, CAX21 EUR 0.0176/h (plan prices include
  IPv4); 1 RS + 1 LG = 0.1562/h; each VM rounds up per session; RS<->LG traffic internal and free.
- Rust stable 1.99.1 (2026-10-13; static.rust-lang.org answers 200); container has 1.97.0, x86_64 target only.
- macOS runner images: macos-26 arm64 Safari 26.6.2; macos-15 arm64 Safari 27.2 (infra digest F).

## Facts from Mustafa
- **2026-11-07 about 11:10 UTC, his Windows laptop:** `Get-NetTCPSetting | Select-Object SettingName, InitialRtoMs,
  MaxSynRetransmissions` gave MaxSynRetransmissions 4 for every template (Internet, InternetCustom, Datacenter,
  DatacenterCustom, Compat; Automatic blank) and InitialRtoMs blank (the built-in default). Read with: Catchpoint's
  write-up (docs.catchpoint.com/docs/catchpoint-nodes-tcp-syn-retries: "Windows 10 is configured out of the box with
  an initial RTO of 1 second and 4 Syn Retransmits", SYNs at 0, 1, 3, 7, 15 s, final timeout 21 s) and a Windows 10
  1909 `netsh interface tcp show global` dump (my-computer-memo.blogspot.com, 2020-01: "Initial RTO : 1000",
  "Max SYN Retransmissions : 4"; earlier Windows used 3 s). So a current Windows client retries a lost SYN after 1 s,
  doubling: the "Windows-like client, first SYN timeout 1000 ms" rows apply (never abandoned 645/886/1,484/2,871 on
  60,000 phases); the 3 s rows do not. Optional direct reading on his machine: `netsh interface tcp show global`
  (its "Initial RTO" line).

## Decisions taken
Moved to history.md (RESUME-CARD, 2026-11-10). The rules still in force are listed by name on the resume card, each pointing there.

## Running work
- **2026-11-09 about 04:30 UTC: U3d CLOSED, and U3 (U3a-U3d) with it, on the closing head ef2bcef** (spike code
  1ccff06, the dry run's record 78b0c07; pushed). The advisor's close ruling (advisor-u3d.md, "Close ruling: U3d"):
  APPROVE WITH CONDITIONS, five text-only conditions, all met: (1) the report fixed (a)-(i) (u3d-report.md, folded:
  round 3's lines, its down direction the first E-vs-(-3) disagreement, explained by -3's p99.9 capping a datagram's
  excess at 2a = 10 ms while E counts all of it; the CPU note's attribution and size; best-yet up only; loss per
  direction; commit-and-push; ImpairSocket's own drops vs kernel drops; the 04:05:56 snmp read; the reader-stall
  limit; "one stall" consistent, not shown); (2) R-31 amended ("predicts -3's P1 p99.9 exactly while no lateness
  exceeds 2a + 0.25 ms; past that it can MISS where -3 passes, never the reverse"); (3) the U8 row's slack rule
  replaced with U3d's built rule; (4) C-U8-6 (U3D-RCVBUF: the probe socket's own drops and the Udp/Udp6 counters
  around every L2 leg; a P1 drop makes the self-test INVALID and goes to the advisor, never re-run away; no SO_RCVBUF
  sizing without its ruling) added, its run condition in C-U8-4; (5) this entry and the header (1c902e1 committed
  (1)-(4)). The advisor reproduced check.mjs 9/9 (267/281) and the gate on ef2bcef. Nothing for Mustafa but
  LEFTOVER-DELETE, still open.
  - **Lesson (the advisor's):** a future dry run goes in the background without ending the turn, so the stop hook
    never commits during a measured run.
  - **Next: U4's plan** (plan §12.1's U4 row: npt-sim, npt-proto, npt-room and npt-client; npt-lab's scenario runner;
    `due_ticks` with PM-0004-67's bites on a fake clock; the native-against-Wasm hash test; then the L1 row groups
    with their bites). Plan first, then the advisor; agent copies build in .gate/shared-target (Mustafa's rule).
    Spike follow-ons still owed: marks.json -18 split (parts by transport, the WS three checks, count >= 480, the
    bites); the ws-practice validity rule; SERIES burst_heal and burst_return; the R3 run plan; fixes.rs's
    run-structured bound (needs U4's scene sizes).
  - **About 11:50 UTC: U4's build plan APPROVED WITH CONDITIONS by the advisor** (advisor-u4.md; the approved plan
    pinned at aebe8ea's rev 2, u4-build-plan.md sha256 27e03158...). Twelve sub-units in dependency order: U4s alone
    (seam types, the grid, every manifest), then U4a wire, U4b loop policy (main session), U4c link model (C1', C2),
    U4d stand-in sim, U4j judge and marks (main session), U4d2 Wasm-in-Node hash and determinism step, U4e room and
    codec, U4r the L1 runner and its judged runs, U4f clock and commands, U4g prediction, interpolation and events,
    U4h lag compensation. Conditions 1-11 (refusals outside the state hash; -30's pre-check a recorded npt-lab test
    from a P0 recording with 0 replacements as setup; -22's exception in README §4; the precondition passes; -18's
    bite end to end; U4d2's checks refuse and hash-wasm as the third unsafe exception; C-U7-1 for wasm-opt; `replay`
    tested at U4 or moved; lanes; the readings list) + nits (a)-(g); condition 12: only those edits without coming back;
    the five briefs (U4e, U4r, U4f, U4g, U4h) to the advisor before their builds. 26 questions ruled; six readings
    (Q11, Q16's clamp, Q17, Q22's -22 exception, Q24, Q26) go to Mustafa in the readings PR before R5. Nit (g) done:
    the stray spikes/netcode-proof/target (209 MiB, with CACHEDIR.TAG) removed by `cargo clean --target-dir`.
  - **About 12:05 UTC: the conditions applied** (u4-build-plan.md, 1,012 lines, sha256 f7961cfe..., checked complete by
    the test-engineer, u4-conditions-check.md); the 13 plan.md edits (u4-plan-md-edits.md) and the checkpoint's C3 and
    C-U7-1 lines made by the main session (c8abc85).
  - **About 12:10 UTC: U4s's build launched, workflow `wf_aa2cc979-004` (npt-u4s) on c8abc85:** build:u4s
    (general-purpose, xhigh, alone in a worktree, the shared target; notes u4s-build.md in the worktree), then
    review:netcode and review:arch read-only (u4s-review-netcode.md, u4s-review-arch.md), then prove:rust (the only one
    that builds and plants bites; also the libm dependency per §4.8; u4s-review-rust.md). **A deviation to report:**
    the plan's §7.2 has the main session build U4s; it is one builder agent alone instead (the same isolation, nothing
    else running), to keep the main session's context for merges; U4s's report says so. libm 0.2.16 is already in
    Cargo.lock (no new package). If cut: snapshot the builder's worktree (worktree_patch.py ... wip/u4s.patch u4s-),
    then Workflow({scriptPath:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u4s-wf_aa2cc979-004.js,
    resumeFromRunId: "wf_aa2cc979-004", args: {sha: "c8abc857d9bc5e7fb3a848ddcf430ab9c4b65c31"}}).
  - **About 13:10 UTC: U4s built and reviewed** (worktree .claude/worktrees/wf_aa2cc979-004-1, uncommitted on c8abc85;
    snapshot wip/u4s.patch). Builder: check.mjs 9/9 twice (273 measured / 288 bites, self 45), 9 of 9 bites, cargo tree
    -d unchanged, Cargo.lock +12 lines offline. Reviews: netcode-architect, engine-architect, rust-reviewer all APPROVE
    WITH FINDINGS (u4s-review-*.md), no high. Medium: netcode 1 (the sim reads only accepted fields; tick and view time
    belong to the room), netcode 2 (canonical command order), arch F1 (the grid's bite ids must sit where their code
    lives: -57's coarsening to npt-proto's Quant), arch F2 (the per-kind field layout defined in npt-sim; VELOCITY fixed
    for wave 1).
  - **About 18:15 UTC: the advisor's F1/F2 ruling applied** (advisor-u4.md "Ruling: F1 and F2"; plan sha256
    0d69332c…, carries 505e54d6…): F1 the sizes (16.8 MB, 50.5 MB, 4.2 million; "about 17 MB" at :46, :138, :946);
    F2: no new tick count is needed by U4b or U4e; the Input refill and the window budget take the rate as an argument
    (carry 6's pattern, pinned at 30 and 60 Hz); anything found later goes to the main session's own commit by carry
    18's route once no lane owns crates/sim or check.mjs. **U4a's builder launched** (single background agent,
    worktree, base dffef0e, shared target; carries 1-5 and 34 word for word; notes u4a-build.md in its worktree): the
    third agent beside the U4d/U4c workflow. Its reviews (netcode, rust, test per §5) wait for free slots. If cut:
    snapshot its worktree (`git worktree list` names it agent-…) with worktree_patch.py (wip/u4a.patch u4a-), then a
    fresh builder continues from its notes. Then U4b and U4j.
  - **About 21:45 UTC: wave 1 cut by the spend limit and snapshotted; the main merge in progress; new rules.**
    - All three wave-1 builders stopped about 18:40 UTC on the org's monthly spend limit (reset 21:40 UTC):
      wf_f54246ff-e0d's build:u4d and build:u4c (the workflow ended, both failed), and U4a's agent a9297644e7c0354f3.
      Snapshots at 21:44 (worktree_patch.py; the worktrees are kept): **wip/u4d.patch** (base 097f897; crates/sim, 10
      files, 150 KB; notes u4d-build.md: step 1 library written, clippy clean host, bites and wasm32; tests next;
      carry 13's counting allocator cannot compile under `unsafe_code = "forbid"`, a storage-identity test stands in,
      to be reported as not met); **wip/u4c.patch** (base 097f897; crates/l1 lib.rs and tests.rs, 117 KB; notes
      u4c-build.md and u4c-build/: item 1 done, U2's full hashes reproduced on the old code, the three FNV pins
      recorded); **wip/u4a.patch** (base dffef0e; crates/proto, 11 files, 130 KB; notes u4a-build.md: plan only).
    - **Resume:** each builder continues in its own worktree (.claude/worktrees/wf_f54246ff-e0d-1 = U4d,
      wf_f54246ff-e0d-2 = U4c, agent-a9297644e7c0354f3 = U4a) from its notes; if a worktree is gone, a fresh one at
      its base applies the patch first. Then the reviewers per lane, within the review budget.
    - **The main merge (kit-v3-4, origin/main 44446a5) in progress** in the detached worktree `.gate/main-merge`
      (from 0d97e58): conflicts in docs/REPO_MAP.md (take main's "Last checked") and docs/STATUS.md (main's
      _Last updated_ and In-flight text, plus the spike's "Also in flight" netcode paragraph brought up to date; main's
      kit-v3-4 and vfx-xp-notes rows plus the spike's netcode-proof-test row). Then commit, push to this branch,
      `git merge --ff-only` in the main folder, check.mjs (marks step: main changed docs/adr/0006) and the gate, and a
      plain `git worktree remove .gate/main-merge`. Why now: the night shift switches only to a branch whose rules,
      CI and gate files equal main's, so its cs/ branches start from this branch after the merge.
    - **New rules from main (PR #44, kit-v3-4):** risk tiers 0-3 (spikes tier 1: CI, the advisor, the type's
      reviewers); review budget (two rounds at most, the second on what changed; a third only for an open Critical
      or High; lower findings become STATUS follow-ups; a Low never triggers a push on its own); one severity scale
      (Critical, High, Medium, Low); the rollback rule. **Advisor calls:** main's advisor.md changed after this
      session started, so the advisor is called as a general-purpose agent that reads
      `git show origin/main:.claude/agents/advisor.md` and acts as it.
    - **Mustafa, 2026-11-09 (verbatim, the adjusted version):** "From tonight, the night shift takes night work off
      your hands. Each evening, post the next night items to issue #16 as approved Night queue entries: review rounds
      and skeptic checks on the day's merged code, scouting and plan drafts for the next unit. The night shift works
      them on its own branch; you fold its results in the next morning. Items that need building or testing stay
      with you during the day." Tonight's items, after the main merge: the U4e brief draft; a skeptic check of U4s's
      merged code (f080551); the U4r brief draft (with O1); a skeptic check of U3d's merged code (1ccff06).
  - **About 22:05 UTC: main merged, tonight's items posted, wave 1 resumed.**
    - **Main merged: df8975b** (merge of e3e11d2 and origin/main 44446a5; conflicts in docs/REPO_MAP.md, main's date,
      and docs/STATUS.md, main's text plus the netcode lines; pushed from `.gate/main-merge`). The guard lets only
      origin/main merge into the main folder, so the main folder did the plain merge itself (allowed: the branch
      changed no kit file since the merge base), with the same resolution, and committed with the same message and
      dates: the same commit, df8975b (the SSH signature is deterministic; `git commit-tree` checked it first). Kit,
      CI and gate files equal main's (`git diff origin/main -- .claude .github CLAUDE.md tools`: empty). Checks: gate
      PASS `--no-smoke`; check.mjs `--only marks,refs,redact` all pass (81 marks; main's ADR 0006 edits touch no
      marked row); repo-map check OK. `.gate/main-merge` removed (plain `git worktree remove`). This session now runs
      main's kit-v3-4 rules (CLAUDE.md: risk tiers, review budget, one severity scale, rollback).
    - **Tonight's items posted:** plans `night-2026-11-10.md` (pushed at 2dc1f31); issue #16's body, Queue items 2-5
      as `- [ ]` APPROVED entries (2 npt-u4e-brief, 3 npt-u4s-skeptic, 4 npt-u4r-brief, 5 npt-u3d-skeptic), each on
      its own cs/ branch from origin/main, reading this branch at df8975b in a read-only worktree `.gate/npt-spike`;
      no build, no test run, no pull request; comment 6752613836. **Morning:** fetch each cs/npt-* branch, read its
      checkpoint and notes, fold the drafts and findings into this branch's notes (Write), a Critical or High from a
      skeptic check to the day's work (the budget's third round), Medium and Low to the unit report's follow-ups.
    - **Wave 1 resumed:** U4a's builder continued by SendMessage (agent a9297644e7c0354f3, its own worktree);
      U4d and U4c by the edited workflow script (wf_f54246ff-e0d resumed, task w0ynfq9yu: the builders continue in
      their existing worktrees wf_f54246ff-e0d-1 and -2 from their notes, no new worktree). Three agents running.
  - **2026-11-10 about 06:35 UTC: morning.**
    - **The builders were cut again** by the spend limit at about 22:35 UTC; the limit reset at 02:40.
      - The 06:30 snapshots show their sources unchanged since 22:28; only U4a's notes gained a line (it was at its
        final checks: measured 296, bites 321, self 52 expected; carries 1-5 and 34 met; four items for the main
        session).
      - Resumed at 06:33: U4a by SendMessage (with night finding S1 to answer in its report); U4d and U4c by
        wf_f54246ff-e0d again (task wz87zkv57, the RESUME preface dated).
      - Saver restarted (bjdku8jwv).
    - **The night shift's results (issue #16 report, comment 6759962485):** one run, from 06:04 to 06:49 Amsterdam.
      - **Item 2 done:** `cs/npt-u4e-brief` at 0dccbf0.
        - `.checkpoints/npt-u4e-brief/u4e-brief-draft.md`, rev 2 (rev 1 at ea2dd8a), and
          `test-engineer-u4e-check.md`: 23 findings (11 Medium, 12 Low), all fixed in rev 2.
        - A 28-row recheck table against U4a, U4b and U4d, and 9 questions for the advisor (U4E-Q1 to Q6, Q8, Q10,
          Q11).
        - Needs a day run: the base counts, check.mjs's step count, recheck row R11.
      - **Item 3 done:** `cs/npt-u4s-skeptic` at 6652be4 (summary `.checkpoints/npt-u4s-skeptic.md`). Nothing
        Critical or High.
        - **S1, Medium:** `EntityKind::layout()`'s soldier list (seam.rs:114-116) serves two catalogue rows. It sends
          velocity to everyone, while the remote-soldier row (netcode-proof-messages.md:137) leaves it out, and it
          has no ammo or fire timers for the owner's own-state row (:136). About +54 % bytes per remote soldier (a
          false FAIL risk, never a false pass); High if U4g's prediction needs the weapon timers. Fix: an audience
          per field, or an own-state layout, before U4a's table lands. Goes to U4a's review and the advisor.
        - **11 Lows**, follow-ups for U4's report: S2 (a room/World tick mismatch turns lag compensation off
          silently), T1/F-L1 (`LOOP_CATCHUP` is 0 at 1-3 Hz, which the range check allows; U4b's `Schedule::new`
          panics on cap 0), F-L4 (integer fields above 2^24 through the f32 grid), F-L7 to F-L9 (three check.mjs grep
          gaps), the rest in the summary.
      - **Items 4 (U4r brief) and 5 (U3d skeptic): not started** (06:45 rule). They stay queued for tonight
        (NQ-CARRY (a): U4r's brief is not needed before then).
    - **NS-START found:** the active routine "Shamble Frontline Night shift" (trig_01QpV6y6UaG1Na2q3jumRDxh) has cron
      `CRON_TZ=Europe/Amsterdam 0 6 * * *`, one run a day at 06:00 Amsterdam (fires about 05:04 UTC), last updated
      2026-11-07 00:02 UTC; last run SUCCEEDED 05:04-05:07 UTC. STATUS's `10 0-7 * * *` is the retired "Night shift"
      routine's (trig_019LtRqmAdzddyk6K3e3kDJz, disabled). Changing it is Mustafa's setting: a decision for him.
  - **About 22:15 UTC: U4b built by the main session** (worktree `.gate/u4b-wt`, local branch u4b-build from
    4b5812a, uncommitted there; snapshot wip/u4b.patch; record u4b-build.md). schedule.rs (`Schedule`, `Due`,
    `Dropped`, `LoopCounters`, `due_ticks`; bites: `with_bite`, `BITES`), lib.rs re-exports, server/clippy.toml
    (the clock ban; tokio's entries `allow-invalid`, dormant until U6 adds tokio), tests/schedule.rs: 10 clean tests,
    16 in bites builds; clippy clean both builds; the three bites' in-place plants each FAILED its own named test
    (file restored, sha256 dfb7da2c…); carries 6 and 35 met. **check.mjs twice: all 10 steps pass both times**
    (measured 285 = 275 + 10, bites 308 = 292 + 16, self 48 of 48). CI was green on df8975b (PR #40's last CI run;
    the checkpoint commits after it skip CI). **Next for U4b:** its review (netcode-architect, rust-reviewer,
    test-engineer) when slots free; then merge it into the main folder (on committed state, between lane merges).
  - **About 18:00 UTC: conditions 1-3 applied** (u4-build-plan.md sha256 7ec1b324…, recorded under the ruling; carries
    completed and corrected per the check, sha256 4a8c0ee8…, 45 entries; F1 sizes and F2 the U4b/U4e tick-constant
    route to the advisor, background agent). **Wave 1 lanes U4d and U4c launched:** workflow `wf_f54246ff-e0d`
    (npt-u4-wave1-cd) on 097f897: per lane a builder (worktree, shared target; U4d's brief holds carries 7-20 and
    36-38 word for word), read-only reviewers (U4d: netcode, rust, then arch; U4c: netcode, rust), then a
    test-engineer with the bites (u4d-*.md, u4c-*.md notes). U4c builds and unit-tests only; its L-c8 judged re-run
    waits for U4j. U4a, U4b (after F2) and U4j follow as slots free (at most 3 agents; the workflow's cap is 2). If cut:
    snapshot each builder's worktree, then Workflow({scriptPath:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u4-wave1-cd-wf_f54246ff-e0d.js,
    resumeFromRunId: "wf_f54246ff-e0d", args: {sha: "097f89704d9925dee0dc62f7d7373cb13fbbc9a0"}}).
  - **About 17:45 UTC: the advisor ACCEPTED U4s WITH CONDITIONS** (advisor-u4.md, "Ruling: U4s"): wave 1 may start once
    conditions 1 (the approved plan edits (a)-(d): carries 30-33 and 5; NF-D4's new D-4; §7.2 = Lanes on the builder;
    the wave-1 seam freeze) and 2 (u4s-carries.md completed; O1 for U4r's brief) are in the text; 3 the freeze; 4 the
    wave-1 briefs carry the entries word for word and each report lists the carries met by number. NF-D4: (a), the
    counter goes, new D-4 as worded. U4S-BUILDER: acceptable (also U4b and U4j, as a builder in a free lane).
    Applying 1-3: workflow `wf_0edfc217-58a` (npt-u4s-conditions; netcode-architect then a test-engineer check; notes
    u4s-conditions-notes.md, u4s-conditions-check.md). Then record the plan's new sha256 under the ruling, then wave 1.
  - **About 17:30 UTC: U4s MERGED at f080551** (pushed). The last small round (G1 bite-id uniqueness; G2 the doc step,
    check.mjs now 10 steps; the npt-sim membership message) checked by the rust-reviewer: one low gap G1a (the bite-id
    pattern missed 16 of 114 marks.json ids: upper case, dots), fixed by the main session (BITE_ID widened; tests: every
    marks.json id matches, an upper-case id in two crates refused; the old pattern fails the test). check.mjs 10/10 in
    the main folder (275 measured, 292 bites, self 48, doc 3.2 s, redact 157 items 0 findings); gate PASS --no-smoke.
    Report u4s-report.md to the advisor (background agent, about 17:31): U4S-ACCEPT, NF-D4, U4S-BUILDER (also U4b,
    U4j), U4S-CARRIES; its ruling to be appended to advisor-u4.md. Next: wave 1 (U4a wire, U4c link model, U4d stand-in
    sim in lanes; U4b loop policy and U4j judge by the main session or as the advisor rules), at most 3 agents.
  - **About 17:05 UTC: the fix pass's check DONE** (resumed run; u4s-fix-review-rust.md): all 11 items in, no
    regression; two low gaps: G1 no check that a bite id lives in one crate only (a re-added id in grid.rs passes
    everything); G2 check.mjs runs no rustdoc (a broken doc link passes). Nits: s6's ALL.len() literal (U4d carry 12),
    the npt-sim allow-list silent if the crate is missing, check.mjs:609 by design.
  - **About 17:06 UTC: U4s's last small round launched, workflow `wf_4e7cf236-450` (npt-u4s-fix2):** fix:u4s-g (G1's
    uniqueness check, G2's doc step making check.mjs 10 steps, the missing-crate message; same worktree), then
    check:u4s-g (rust-reviewer; u4s-fix2-review-rust.md). If cut: Workflow({scriptPath:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u4s-fix2-wf_4e7cf236-450.js,
    resumeFromRunId: "wf_4e7cf236-450"}). Then merge U4s, check.mjs and the gate in the main folder, commit, push, the
    U4s report to the advisor (the builder deviation; NF-D4).
  - **About 16:45 UTC (save at Mustafa's ask; the spend limit cut the checker, reset 16:40):** fix:u4s DONE (items
    1-11 in; 31 bites as expected; check.mjs 9/9 twice: 275 measured / 292 bites, self 47; carries in u4s-carries.md,
    33 entries by brief, with the advisor's question NF-D4 on non_finite and D-4). check:u4s-fix (rust-reviewer) was
    cut partway (its notes u4s-fix-review-rust.md, 39 lines, on disk). The worktree snapshotted again
    (wip/u4s.patch, 97,516 B, 18 files). **Resume:** the script's CHECK prompt now tells the checker to continue its
    notes; Workflow({scriptPath: the same, resumeFromRunId: "wf_841d72c0-291"}) replays fix:u4s from cache.
  - **About 13:15 UTC: U4s's fix pass launched, workflow `wf_841d72c0-291` (npt-u4s-fix):** fix:u4s in the same
    worktree (items 1-11; non_finite kept and documented as always 0, its drop and D-4's rewording a question for the
    advisor), then check:u4s-fix (rust-reviewer; u4s-fix-review-rust.md); the carries for the U4a/U4d/U4h briefs in
    u4s-carries.md. If cut: snapshot the worktree, then Workflow({scriptPath:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u4s-fix-wf_841d72c0-291.js,
    resumeFromRunId: "wf_841d72c0-291"}). After it: merge, check.mjs 9/9 and the gate in the main folder, commit,
    push, a short U4s report to the advisor (with the main-session-builds deviation and the non_finite question).
  - **Done:** a workflow applied conditions 1-11 and the nits to u4-build-plan.md (netcode-architect) and drafted the
    exact plan.md edits (§4.2 three exceptions; §4.7 RoomControl; E7's text; §4.4's per-tick inputs and the clamp
    column; :562 loss to U6; the U7 row's C-U7-1; risk 11's moves) for the main session; a test-engineer checks
    completeness and that nothing else changed; then the main session applies the plan.md edits, then U4s.
  - **Done (about 09:50-11:45 UTC): U4's planning workflow `wf_899b9cf1-c90` (npt-u4-plan) on f985226:** scouts
    (engine-architect u4-scout-arch.md; test-engineer u4-scout-marks.md; netcode-architect u4-scout-netcode.md), a
    netcode-architect drafter (u4-build-plan.md rev 1, u4-build-plan-notes.md), critics (u4-plan-review-test.md,
    u4-plan-review-arch.md), the fold (rev 2, u4-plan-rev2-notes.md), then the advisor (its verdict returned as text;
    the main session writes advisor-u4.md). Read-only agents in the main folder. If cut: Workflow({scriptPath:
    /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/npt-u4-plan-wf_899b9cf1-c90.js,
    resumeFromRunId: "wf_899b9cf1-c90", args: {head: "f985226"}}).
- **Older entries (U3c and before)** moved to history.md (RESUME-CARD, 2026-11-10).
