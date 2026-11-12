# Checkpoint: netcode-proof-test
- **Goal:** the netcode proof test (SPIKE_PLAN item 1), built unit by unit under its approved plan
  (`.checkpoints/netcode-proof-test/plan.md`, revision 3.3; approved by the advisor and Mustafa, 2026-11-06) · verdict:
  `.checkpoints/netcode-proof-test/advisor-verdict.md` · PR #40 (draft)
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`) ·
  **Base:** main, last merged in at 5ca77eb (main bb2d817, PR #49, the STATUS split)
- **Last update:** 2026-11-12 about 02:05 UTC (U4d-F merged at adb1bab, CI green; U4e's L3 at 28a85e7; next, U4e's
  launch)

## Resume card (read this first; open the rest only when a step needs it)
**The HOLD is lifted** (Mustafa, 2026-11-11 about 16:45 UTC, verbatim in history.md "Decisions taken": "The hold
lifts when the save is pushed."): the save of both work folders is 4ad80a9. USAGE-SAVINGS (a) is in force with his
limits (Rules in force). The card's blocks of 2026-11-11 morning are in history.md, "Running work, U4 wave 1 (2)".

**The current step: U4 wave 2's start.** U4e's launch (L4; L3, the carry-51 commit, done at 28a85e7); U4d2's brief with
the advisor. Plan files: plan.md (rev 3.3, sha256 2bef127b…), u4-build-plan.md (rev 2 with the U4e brief ruling's
L2 edits and L3's §4.1 line at 1e8c188, 686a583c…; it read d928358b… before 1e8c188), u4s-carries.md (carries 1-72, cdab0240…); the advisor's rulings: advisor-u4.md,
advisor-u4-wave1.md (with B1.4's erratum), advisor-u4j.md, advisor-u4d-close.md, advisor-u4j-r2.md,
advisor-u4e-brief.md, advisor-u4df.md.
- **Merged into the spike branch:** U4s f080551, U4b 411eeee, U4a 952f734, U4c f3f7c2f, U4d with W1-S1's npt-proto half
  39f4290 (report u4d-report.md), U4j 4d2b0ef and d50396a (u4j-build.md), and **U4d-F** (the commit "U4d-F: the
  estimate scene fix (npt-sim), merged after review round 2", adb1bab; report u4df-report.md; record u4df-build.md);
  CI green on each (U4d-F's: run 43137225827). Main merged in at 5ca77eb (PR #49). At U4d-F's merge: check.mjs 10
  steps, measured 399, bites 446, self 52: the base counts for the next lanes. U4e's `<base>` is L3's commit 28a85e7,
  with the same counts (its counted run, .gate/u4e-l3/l3-check.log; the accessor adds no test).
- **U4e's build (L4): DONE** (05:41 UTC; first run wf_0fa6595a-3e2 cut by the usage limit at 02:03 after 49 s, no
  code; relaunched as wf_1e3b5232-26a, one fresh builder, no isolation, in the worktree
  `.claude/worktrees/wf_0fa6595a-3e2-1`, branch worktree-wf_0fa6595a-3e2-1 at 97bb9d3, uncommitted). Nothing stopped.
  check.mjs twice (.gate/u4e/b11-counted1.log, b11-counted2.log): all 10 steps PASS, measured 458 (+59), bites 524
  (+78), self 52 of 52. Snapshot wip/u4e.patch (23 files on 97bb9d3, applies cleanly) and its record u4e-build.md:
  db49108. Its return list (section 6) read in full by the main session. Its points for the advisor: e7's "at least
  one split and one resent" not met at P0 on Loopback (split shown by e2, resends by the gap test); under -18's bite
  the baseline (tick 329's frame) is still in the client's memory and only the ring's age rule refuses it (§5 U4e's
  condition 6: the advisor confirms); owner-to-all gives each client the next slot's records; carry 26 one copy per
  tick; proposed series for carries 54 (`event.snap_refused.tick_count`) and 61 (`tick.s.snap_too_old_count`);
  Kick(replaced) left to the caller. **The main session's own points for the ruling** (not in any packet): carry 68
  (any ES byte figure from the synthetic recording states the two shares beside R2's) is not in its carries list,
  though carry 56's ES figure is reported; carry 61's choice: decode within the ring, drop and count beyond it (does
  B5's extra reading apply?).
- **Running: U4e's review round 1,** workflow wf_ed170231-913 (task wfdlzh70j; script
  workflows/scripts/npt-u4e-review-r1-wf_ed170231-913.js), launched about 05:50 UTC. Packet u4e-packet.md (sha256
  89426183…) and its side file u4e-packet-tests.md, cut by review_packet.py from u4e-packet.json (the script now
  takes an `advisor-` file whose name holds "-brief"). Base copy `.gate/u4e-prefix` (1e8c188, as L3 says); read copy
  `.gate/u4e-r1` (97bb9d3 plus the patch, byte-equal to the builder's crates). Batch 1 (Read): netcode-architect,
  security-auditor, rust-reviewer, read only. Batch 2 (Prove): engine-architect (the read copy, no runs) beside the
  test-engineer (general-purpose with Edit: plants by Edit in the builder's copy, every bite's part (ii), the other
  seats' findings settled by run, one counted check.mjs). Notes u4e-review-{netcode,security,rust,arch,test}.md; logs
  .gate/u4e/r1-*. If cut: first check every file's sha256 against the test-engineer's step (1) (a plant may be in
  place), then resume with `Workflow({scriptPath, resumeFromRunId})`. On return: read every finding rated Medium or
  higher in full, and everything touching the judge, the marks or a pass bar; then the advisor's ruling on round 1
  (with the points above), the fix pass, round 2 on what changed.
  - **By 06:40 UTC four seats are done** (journal of wf_ed170231-913): netcode REVISE (1 High, 5 Medium, 5 Low; 6
    before the merge), security APPROVE WITH FINDINGS (1 Medium, 8 Low), rust APPROVE WITH FINDINGS (1 Medium, 9 Low),
    arch REVISE (6 Medium, 8 Low; 6 before the merge). The test-engineer is still running (logs .gate/u4e/r1-te-*).
  - **Cut at about 06:42 UTC by the org's spend limit** ("You've hit your org's monthly spend limit ... your 5-hour
    limit resets 7:10am (UTC)"): the test-engineer's seat failed mid-run, with plant P-D left in place in the builder's
    copy (client/src/lib.rs:670-674). **Restored by the main session at 07:35 UTC** by Edit: sha256 bd83870c… =
    start; the copy's crates/ equal .gate/u4e-r1's (`diff -rq`); its re-cut patch equals wip/u4e.patch (cmp); a
    labelled main-session block at the end of u4e-review-test.md says so (P-B's and P-D's logs have no notes line).
    **Resumed about 07:36 UTC** (`resumeFromRunId` wf_ed170231-913, task wwogpkxp1): the four done seats replay from cache,
    the test-engineer runs live (agent ae67a5f5…). Read in full by the main session: every Medium and High of the four
    seats, and the Lows touching a mark, a bite or the judge (NC-L1 to NC-L4, NC-L5-L12, A7, A13, A14, SEC-U4E-6).
  - **Round 1 DONE at about 08:07 UTC.** The test seat: REVISE (TE-1 High, settling NC-H1 by run; TE-2 to TE-6
    Medium, all before the merge: plants that pass every test; TE-7 to TE-10 Low); its counted check.mjs run
    (.gate/u4e/r1-te-check-counted.log) all 10 steps PASS, 458 / 524 / 52, as the builder's. The copy is clean at the
    end (`diff -rq` against .gate/u4e-r1: none). Read in full by the main session: its High and Mediums, TE-7, TE-8.
- **Running: the advisor's ruling on U4e's round 1,** workflow wf_9da34c56-a1a (task w77dvfluc; script
  workflows/scripts/npt-u4e-advisor-r1-wf_9da34c56-a1a.js), launched about 08:08 UTC: general-purpose, opus, xhigh,
  reading main's advisor.md; brief advisor-u4e-r1-brief.md (items 1-9, with the main session's points in item 7);
  ruling to advisor-u4e-r1.md, block by block. If cut: resume with `Workflow({scriptPath, resumeFromRunId})`. Then:
  the fix pass in the same worktree on the ruling's scope, round 2 on what changed, the merge on its conditions.
  - **The ruling: REVISE** (advisor-u4e-r1.md, 08:34 UTC; R1-R9, texts T1-T6). Round 1 counts as one round. R1: the
    High holds; the client's three cases for a header tick (inside the ring: decode; from the first snapshot on and
    the ring behind: drop and count, `stale`; before the first snapshot and the ring behind: a bad header, the
    sentinel filed under that tick, which the judge counts as an orphan); the tick16 twin asserts what the judge counts.
    Fix pass FX-1 to FX-13 (R8: one builder in the same worktree; bounds crates/room, crates/server but schedule.rs,
    crates/client; one new pub item `RttSource`; evidence per FX: plant, FAILED by name, restore, sha256). Carries
    73-81 and 62 (h) (T2), C-U6-12 to C-U6-15 (T3), marks/README.md §1's case-3 sentence (T1), SERIES.md's
    `event.snap_refused.tick_count` (T4), the report's T5, §4.1's rows (T6): all by script in the merge commit (M8).
    Round 2: five seats in two batches (R8). Merge on M1-M9 (R9). Nothing to Mustafa. Read in full by the main
    session: R1, R8, T1-T4 (the marks texts: no pass bar's number or direction moves; T1 is stricter).
- **Running: U4e's fix pass,** workflow wf_4c29cbe6-2fd (task wsgylkqit; script
  workflows/scripts/npt-u4e-fixpass-r1-wf_4c29cbe6-2fd.js), launched about 08:35 UTC: one builder (general-purpose,
  opus, xhigh), no isolation, in `.claude/worktrees/wf_0fa6595a-3e2-1`; its record's new section "Fix pass, round 1"
  in the worktree's u4e-build.md; logs .gate/u4e/fx-*; end hashes .gate/u4e/fx-hashes-end.sha256. If cut: check the
  copy for a plant in place (its record's last plant line; restore by Edit), then resume with `Workflow({scriptPath,
  resumeFromRunId})`. On return: read its stops and TE2-5 lines in full; check the copy against fx-hashes-end.sha256;
  re-cut wip/u4e.patch and commit it; cut round 2's packet (u4e-r2-packet.json: db49108's patch against the new one,
  this ruling as the "judged against" half); round 2 (R8's seats).
- **Running: U4d2's builder brief** (next step 3), workflow wf_ec1d2b01-aab (task wgkj3qotm; script
  workflows/scripts/npt-u4d2-brief-wf_ec1d2b01-aab.js), launched 06:45 UTC in the slot round 1's batch 2 left free,
  one agent at a time: the rust-reviewer drafts rev 1 (u4d2-brief.md; notes u4d2-brief-notes.md), the test-engineer
  checks it (u4d2-brief-check.md, in the form of u4e-brief-r3-check.md), the drafter revises (rev 1.1). No cargo
  runs; the carries' words and Mustafa's words are left as placeholders, for the main session to copy by script.
  Then: the main session's fold, and the advisor's check before the build. If cut: resume with
  `Workflow({scriptPath, resumeFromRunId})`. **Its first launch failed in 5 s** (all three agents: the spend limit,
  06:45 UTC; nothing written); **resumed about 07:36 UTC** (task wp2rtgeks).
  - **DONE about 09:00 UTC: rev 1.1** (u4d2-brief.md, 929 lines, sha256 e6bcefc8…). The check (u4d2-brief-check.md):
    REVISE, TB-1 to TB-3 High (placeholders by moving line ranges; no pass or fail rule for the bites half; a
    read-back no test can fail), TB-4 to TB-11 Medium, TB-12 to TB-20 Low; rev 1.1 applies 18 as written and 2 with a
    change (U4D2-Q12). Questions for the advisor: U4D2-Q1 to Q12. Placeholders P1-P8 (by text anchor and sha256; P6
    `<base>` and counts; P7 the runner u4d2-run.sh) for the main session at launch. Noted for the main session:
    server/clippy.toml lacks Q9 (a)'s hash-map bans; crates/client-wasm/Cargo.toml:9-13 says "two" exceptions. The
    drafter's slips: two `cp` copies and one stderr redirect, all in the session scratchpad, outside the repo.
- **Running: the advisor's check of U4d2's brief rev 1.1,** workflow wf_6f64e718-539 (task wkw0bz0ml; script
  workflows/scripts/npt-u4d2-brief-advisor-wf_6f64e718-539.js), launched about 09:00 UTC; ruling to
  advisor-u4d2-brief.md (in advisor-u4e-brief.md's form; items 1-7). Then the main session folds its conditions and
  plan edits by script, fills P1-P8 and launches U4d2's builder (three agents at most, with U4e's round 2).
- The saver: `node .claude/scripts/checkpoint.mjs watch netcode-proof-test --trailer
  "Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>" --trailer "Claude-Session:
  https://claude.ai/code/session_0173vVo6igCNyQYBWPvhH36A"` (background, 120 minutes; restart it when it stops). Stop
  it before a merge's apply and restart it after the push.

**Finished units, and what stays open from each** (their old blocks: history.md, "Running work, U4 wave 1 (3)" for
U4j, and "Running work, U4d-F" for U4d-F and the U4e brief's drafting):
1. **U4j: MERGED** (4d2b0ef, d50396a). Open, as routed: carries 61 (§U4e), 62 and 63 (§U4r), 64 (the next change to
   crates/judge), 65 (§U4d2, or the main session sooner); C-U6-9 to C-U6-11 (plan.md's U6 row); the FU list for PR
   #40's report (the closing checklist); one more reading only if U4e's client drops stale groups (advisor-u4j-r2.md
   B5).
2. **U4d-F, the estimate scene fix: MERGED at adb1bab** (01:50 UTC; CI green). Run A (four builders; 3b's re-home;
   `aim_one_in` 40), round 1 (both seats APPROVE WITH FINDINGS: NF-M1 and TF-M1 Medium), the advisor's ruling
   (advisor-u4df.md: APPROVE WITH CONDITIONS; RM, carry 67; FX-1 and FX-2 before the merge), the fix pass (FX-1 exact
   on 954,628 sky aims; Q1-Q5 each fail it by name), round 2 (both seats APPROVE WITH FINDINGS; one Low, NC2-L1), the
   merge on M1-M9 (u4df-build.md, "Merge into the spike branch": M4 17 OK; M5 399 / 446 / 52; M7 gate PASS with
   --no-smoke). Open, as routed: carries 67 (RM), 68 (§U4e), 69 (§U4r), 70 (§U4g), 71 and 72; carry 52's last part
   (U4d2 pins only `Recording::GENERATOR` 2's recordings and names U4d-F's commit); NC2-L1 (u4df-report.md,
   follow-ups).
3. **The U4e brief: rev 3.3, approved** (advisor-u4e-brief.md: APPROVE on L1 to L4; L1 and L2 done at 3088bb5).

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

**Next steps, in order:**
1. **U4e's L3: DONE** (advisor-u4e-brief.md; carry 51's words): `Scene::effect_rows()` at 28a85e7 (its own commit;
   CI green, run 43138206491), its §4.1 line at 1e8c188 (the saver's last autosave caught the plan edit when it
   stopped: stop the saver before editing toward a commit that must hold both). The counted run at 28a85e7: 10 steps
   PASS, 399 / 446 / 52 (.gate/u4e-l3/l3-check.log), filled into the brief's 7.2 item 1; gate PASS (--no-smoke).
   U4e's review reads its diff from 1e8c188 (engine-architect: a new pub item, A19; rust-reviewer).
2. **L4, U4e's launch,** as the brief's section 1 says: one builder (general-purpose, opus, xhigh) in a worktree, under
   W1-TARGET's lock; at most three agents at a time, reviewers included.
3. **U4d2's builder brief,** with the advisor: carries 52, 65 and 66, N2-M2's ruling and W1-TARGET; it bases on U4d-F's
   commit and names it.
4. **U4r's brief:** the night's rev 2 at `cs/npt-u4r-brief` ed973c9 goes to the advisor after U4c, U4d2, U4e and U4j
   merge (carry 59's new sentence and carries 62, 63 and 69 to fold).
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
  `.gate/u4a-prefix`, `u4b-prefix`, `u4b-wt`, `u4c-prefix`, `u4d-prefix`, `u4d-s1`, `u4j-prefix`, `u4df-prefix`,
  `u4df-r1` (after U4d-F's merge), `u4e-prefix`, `u4e-r1`, `u4e-empty` (after U4e's merge); `.claude/worktrees/`
  agent-a9297644e7c0354f3 (U4a), wf_f54246ff-e0d-1 (U4d), wf_f54246ff-e0d-2 (U4c), agent-a3b8dfeb01ed52b50 (U4j,
  merged at 4d2b0ef), wf_9ce88fa3-452-1 (U4d-F, merged 2026-11-12).

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
- **The advisor's U4d-F ruling (advisor-u4df.md, R1, R4.1 and R6):**
  1. Every agent brief, builders' and reviewers' alike, carries this sentence word for word: "Write files with the
     Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing files: apply
     rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write."
  2. A builder brief's TE2-5 stop clause reads, with the unit's crate named: "a failure in <crate>, or in a test that
     reads <crate>'s files, stops your step group (report it), unless your step's plan names it as expected (a pin
     your step re-pins, or a test a later step rebuilds): then record it under TE2-5 with its cause and the step that
     ends it, and go on." B2.3's merge stop is unchanged.
  3. RM (carry 67) for any change that moves ES's synthetic recording or the World's outcome on it.

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
