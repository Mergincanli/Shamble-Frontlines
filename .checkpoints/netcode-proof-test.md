# Checkpoint: netcode-proof-test
- **Goal:** the netcode proof test (SPIKE_PLAN item 1), built unit by unit under its approved plan
  (`.checkpoints/netcode-proof-test/plan.md`, revision 3.3; approved by the advisor and Mustafa, 2026-11-06) · verdict:
  `.checkpoints/netcode-proof-test/advisor-verdict.md` · PR #40 (draft)
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`) ·
  **Base:** main, last merged in at 5ca77eb (main bb2d817, PR #49, the STATUS split)
- **Last update:** 2026-11-11 about 10:20 UTC (the card refreshed: U4j built and in review, U4c in round 2, U4d in its
  fix pass; night items 4 and 5 folded)

## Resume card (read this first; open the rest only when a step needs it)
**The current step: U4 wave 1** (u4-build-plan.md rev 2, approved with conditions, sha256 0d69332c… until the W1 text
batch of 2026-11-11, a81b230f… since; advisor-u4.md;
the advisor's wave-1 rulings, with the exact words for every text change: advisor-u4-wave1.md). The card's blocks of
2026-11-10 (the wave-1 list, NOW, SAVE, SAVE POINT, the next three actions of then) are in history.md, "Running work,
U4 wave 1", moved by card_refresh.py.
- **Merged:** U4s (f080551), U4b (411eeee), U4a (952f734), **U4c (f3f7c2f, 10:38 UTC)**; CI green on 411eeee and
  952f734. Main merged in: kit-v3-4 (df8975b) and PR #49, the STATUS split (5ca77eb, by merge_status_split.py, as
  Mustafa asked on 2026-11-11; CI green on 5ca77eb).
- **U4c's merge (f3f7c2f):** round 2 (wf_610486f7-cc1): rust-reviewer, test-engineer and netcode-architect APPROVE WITH
  FINDINGS, Lows only (R2-A = R2-1 = R2-N1, the jitter figure in time_threshold's doc; R2-B = R2-2, drive() hangs
  rather than fails on a model that keeps naming a past instant; R2-3, the fixer's third run never ran): follow-ups.
  check.mjs on the merged tree: measured 325 = 311 + 14, bites 366 = 342 + 24, self 52; gate PASS. wip/u4c.patch
  deleted in that commit. SIZE-CHECK: about 3,130 lines for size M (lib.rs +1,180 −323, tests.rs 1,723, tests/l1.rs
  +230): not well past.
  - Housekeeping later (approved method): `.claude/worktrees/wf_f54246ff-e0d-2`, `.gate/u4c-prefix`.
- **U4d, the stand-in sim (npt-sim), with W1-S1's npt-proto half:** **round 2 running** (workflow wf_4b5ee472-66b;
  cut by the spend limit about 10:55, resumed 11:11 as task wy6140mq8): netcode-architect, security-auditor (W1-S1
  only), rust-reviewer, engine-architect, then test-engineer, one at a time; notes
  u4d-round2-{netcode,security,rust,arch,test}.md.
  - **Done:** netcode-architect APPROVE WITH FINDINGS: every round-1 item closed; both W1-S1 halves met. Two Mediums,
    follow-ups (not before the merge): N2-M1, the estimate scene's event rate drifts down (a 10-minute measured
    window would likely read 8.5-9.3 a second, below PM-0001-2's 9.5: PM-0001-12's ES runs at R2; fix before U4d2
    pins the recordings); N2-M2, ES's 15 vehicles loop 40-82 m out, too far for "every vehicle in every client's
    view" (fix before U4d2 pins ES's hashes, or U4e shows its region covers them). Merge condition from N2-M1: U4d's
    report states T-M1's window, the per-window rates, and that a 10-minute window would likely read below 9.5; a
    U4r carry: the contract test reports the SS spread and the everyone-in-view share. Lows N2-L1 to N2-L4 (N2-L4:
    "(9 fields)" is a slip; Weapon, Scene::weapon, Scene::health belong in §4.1's npt-sim row).
  - **Done:** security-auditor APPROVE WITH FINDINGS: **SEC-U4a-1 and SEC-U4a-2 CLOSED** in writing. One Low,
    SEC-U4d-R2-1 (an effect row's owner, index / 8, has no public home; U4e's filter would copy the 8): "it keeps the
    PR needs-mustafa until it is fixed or explicitly deferred to U4e". Carries for U4e (the codec checks the kind per
    section, not the slot per client).
  - **Left:** rust-reviewer, engine-architect, test-engineer (cut; running again).
  - U4d's fix pass: done (wf_a8ff8550-67d, 10:14): every ruling item built, 29 plants failing by name, check.mjs
    twice in its copy: measured 314 = 304 + 10, bites 337 = 327 + 10. Its open points, for the advisor: (A) the
    estimate scene's knobs `aim_one_in` 11 and `grenade_fuse_ms` 1,515; (B) the estimate scene's event rate drifts
    down over a run (mean in band); (C) SOLDIER has 8 fields where the ruling says 9; new public items `Weapon`,
    `Scene::weapon`, `Scene::health`.
  - **The npt-proto half: built by the main session** in the scratch copy `.gate/u4d-s1` (detached at 21a8597, with
    U4d's final crates/sim): record s1-proto.md. Both GROUPS own-state rows gain OwnState; the codec refuses a record
    whose kind its section does not list (OutOfRange / Malformed); the GROUPS test's three checks; the -57 test sends
    the owner-only record; the generators and a1 follow the sections; one new refusal test. Eight plants, each
    failing by name. check.mjs on the combined copy: measured 351 = 311 + 39 + 1, bites 388 = 342 + 45 + 1, self 52.
    Snapshot wip/u4d-s1.patch (the whole copy).
  - SIZE-CHECK: about 7,250 lines for size M-L (code about 4,210: world.rs 2,409, replay.rs 713, scene.rs 315, seam.rs
    +308; tests about 3,030) plus the proto half's +407 −56: past its plan; three lines to Mustafa.
  - Next: settle round 2; then the merge on the main folder's HEAD (U4c merged since; expect 365 / 412 / 52): apply
    wip/u4d-s1.patch; the catalogue edit W1-S1 names (the own-state section's records); `git rm` wip/u4d.patch and
    wip/u4d-s1.patch. Housekeeping later: `.claude/worktrees/wf_f54246ff-e0d-1`, `.gate/u4d-prefix`, `.gate/u4d-s1`.
- **The plan text edits: done** (workflow wf_e1058c57-961, about 10:44 UTC; notes w1-plan-edits.md and
  w1-plan-edits-check.md; base 20702fd; +125 −47 lines in the four files). All 37 ordered texts are in, word for
  word (the checker's script took each from advisor-u4-wave1.md by line range); Mustafa's NETEM-BAR (a) is quoted
  exactly in the U8 row (plan.md). The check's two Mediums are fixed and re-checked: F1 (the W1-U4D amendment line
  moved to advisor-u4.md's end, so lines 1-475 match the base and every cite into it holds) and F2 (§5 U4d's
  generator words, "words mine"). New sha256: plan.md 4717029c… (1,518 lines), u4-build-plan.md a81b230f… (1,070),
  u4s-carries.md 19e7ca98… (272), advisor-u4.md 7b3e76ca… (479).
  - Open Lows, for the advisor request after wave 1: F10 (the ruling's §4.2 sentence quotes condition 1 as "never"
    where it says "not": a one-word change on the advisor's nod); F3 (carry 40's added sentence and its label); F5
    (a bare "§4.7" in §4.1's Why cell means plan.md §4.7); F6 (two clamp-idempotence claims left unbounded beside
    TUNE-2's bound); F7 (u4s-carries.md's header "44 entries", now 50; carry 13's moved half); F8 (U4E-Q6's two
    conditions go to the U4e and U4r briefs); F9 (a sentence the swap left reading as the manifest's); F11 (W1-PUB's
    approved public items its §4.1 list leaves out); F4 (a cite off by one line: advisor-u3q.md:142-143, not
    :143-144).
- **U4j, the judge and marks change:** **built** (agent a3b8dfeb01ed52b50, done about 10:02): all 10 steps pass twice
  (measured 304 = 286 + 18, bites 327 = 309 + 18, self 48 of 48); its four plants fail by name. Copy
  `.claude/worktrees/agent-a3b8dfeb01ed52b50` (base 411eeee); notes u4j-build.md; snapshot wip/u4j.patch (10:05).
  - SIZE-CHECK: about 2,860 lines added for its size M (code 1,330, tests 1,030, marks.json and docs 370, fixtures
    130): not well past its plan, so no stop.
  - **Review round 1 running** (workflow wf_1ea8a0dd-fee; the test-engineer cut by the spend limit, resumed 11:11 as
    task w4p9h5eio): netcode-architect, security-auditor, then test-engineer; notes
    u4j-review-{netcode,security,test}.md.
    - **netcode-architect: REVISE.** Before the merge: NC-U4J-1 (FAR BETTER uses f = max(1, s) with no band or
      tolerance check: Mustafa's 1.10 cap can be exceeded), NC-U4J-2 (a practice run added after the counted run
      turns its FAIL into INVALID), NC-U4J-3 (README never says whose slot indexes `hs`), NC-U4J-4 (the twin's role
      is not checked), NC-U4J-5 (the `hs` writer rules' four gaps), NC-U4J-7 (the practice run's heal-record count is
      never checked). Readings (a), (b), (d), (f) right; (e) partly (NC-U4J-6). Follow-ups NC-U4J-6, -8 to -14.
    - **security-auditor: REVISE.** Before the merge: **F1 High** (the same FAR BETTER hole: a log can move X, Y and Z
      past WS-BAR's cap), F2 (M_m, the band and the guard come from the log's copy of the bound line, never checked
      against marks.json), F3 (one practice run per build checked only inside the counted run's set), F4 (any log
      can supply any hash list; the Arm rule trusts the log's own aarch64 claim), F5 (the twin's identity unchecked),
      F6 (parse_bound can panic on two 30-digit decimals: an i128 overflow), F9 (a ruling is any non-empty text).
      Follow-ups: F7, F8 (Medium: no minimum coverage for the decoded-hash check), F10-F12.
    - **test-engineer: REVISE** (done about 11:25): F1 High confirmed by a probe run (band 2.0-2.0, s 1.9: X' 1.273
      s, a 1.2 s median PASSES); F9 (the 1.10 cap unpinned: a plant raising it to 1.12 passes all 65 judge tests);
      F2-F8, F10-F12 Medium before the merge, most shown by probes on the unchanged code; F13-F16 Low.
    - **Round 1 complete: three REVISE.** The advisor rules on it now (agent aef3e07087d16c503, writing
      advisor-u4j.md): the deduplicated fix list with the proofs; the calls (coverage floor F10, rulings as text F9,
      pinning the bound line F2/F6, NC-U4J-10, F16, the `hs` writer rules, the Arm rule's key); the readings (a)-(f);
      F1's severity; anything for Mustafa.
    - **The advisor's rulings: done** (advisor-u4j.md, about 11:45): REVISE; 14 fixes FX-1 to FX-14 in one pass
      (FX-1, the High, first: the band must contain 1, FAR BETTER gives f = 1, no f above the tolerance), each with
      its named test or fixture; follow-ups FU-1 to FU-6 (owners named); carries to U4e, U4r and U6. Readings (a),
      (c), (d), (f) accepted; (b) changed by FX-3, (e) by FX-5. F1 stays High (a rule loosened beyond Mustafa's
      words); a round 3 is open to F1 alone. Decisions for Mustafa: none.
      - At the merge: copy the builder's readings (a)-(f) into the notes (R3 quotes them; they are not in
        u4j-build.md); the readings list gains the orphan reading (R5's words); PR #40's report names FU-1 to FU-5 as
        security findings left open.
    - **The fix pass and round 2: running** (workflow wf_0906017a-a9b, task wgj6zbosy): the fixer in U4j's worktree
      (each named test first, failing on the unfixed code), then netcode-architect and security-auditor (read only;
      the security table of every path through `find`), then test-engineer (probe A and the advisor's three F1
      plants on the new head; counted check.mjs). Notes u4j-round2-{netcode,security,test}.md. Pre-fix copy
      `.gate/u4j-prefix` (411eeee plus wip/u4j.patch of 10:05; byte-identical to the reviewed build, checked 11:27).
  - Its fix pass, if any: by its builder (SendMessage to a3b8dfeb01ed52b50), in its copy.
  - At its merge, the main session's: u4-build-plan.md §4.1 gains U4j's public items (`TickRecord.hs`,
    `set_hashes`, `hashes`; `marks::BoundNumber`, `Check.ws_bound`, `Rule::WsPractice`; `Outcome::Empty`,
    `fails()`; `Lack::Empty`); the builder's readings (a)-(f) go to the advisor with the reviewers' views.
  - Then U4c's L-c8 judged re-run on U4j.
- **Agents (3 at most):** U4j's reviewer, U4d's round-2 reviewer (one slot free since 10:44).
- **Night work (issue #16):** all four items done: 2 and 3 on the night of 2026-11-10; 4 and 5 last night, from 02:16
  Amsterdam. The queue is empty: post tonight's items this evening.
  - **4, the U4r brief:** rev 2 at `cs/npt-u4r-brief` ed973c9 (1,007 lines). netcode-architect's check: 8 Medium and 8
    Low, all fixed in rev 2.
    - 12 open questions for the advisor; U4R-Q1 blocks the bots (npt-lab has no npt-bots dependency, and no unit owns
      that manifest edit).
    - 34 rows to recheck against U4c, U4d2, U4e and U4j; 12 day runs (D1-D12).
    - It goes to the advisor once U4r's dependencies merge.
  - **5, the U3d skeptic check:** `cs/npt-u3d-skeptic` 558fc0e. No Critical or High; 4 Mediums (NA1 = TE1, TE2, NA4,
    NA5) and 8 Lows, follow-ups for U3d's report.
    - NA1's proof, run here (na1-output-20261111.txt): at E 9,999 ns (a PASS), -3's p99.9 fails in 49.7 % of
      2,000 seeds. The finding holds.
    - NA1 (R-31's wording plus a U3D-MISS case) and NA5 (C-U8-6: read while the socket is bound; a missing row makes
      the leg INVALID) change ruled text: to the advisor with the plan text edits.
    - TE2-TE4's plants need a quiet shared target: after wave 1's merges.
  - **NS-START, the evidence (b) asks for:** get_trigger at 10:00 UTC today.
    - The routine reads `CRON_TZ=Europe/Amsterdam 10 0-7 * * *`, updated 2026-11-10 20:48 UTC.
    - Last run 06:17 UTC, SUCCEEDED; next 23:10 UTC.
    - Last night's 00:10 and 01:10 Amsterdam runs left no comment.
    - A likely cause, unproven: this session's agents hit the account's spend limit at about 21:20-21:30 UTC, and the
      night shift runs on the same (university) account; the first run that worked fired at 01:16 UTC.
    - STATUS on main still says `10 0-6`, set 18:00 UTC: one change behind (docs-keeper's, on main).

**The next three actions**
1. **When U4d's round 2 returns:** settle it within the budget; then merge the combined copy into the main folder:
   `git apply` wip/u4d-s1.patch (re-taken from `.gate/u4d-s1` first), the catalogue's own-state edit (W1-S1),
   check.mjs (expect 365 / 412 / 52) and the gate, with `git rm` of wip/u4d.patch and wip/u4d-s1.patch in the same
   commit (WIP-CLEAN). Stop the saver during the merge and restart it after.
2. **When U4j's review returns:** a fix pass within the budget, by its builder (SendMessage); round 2 on what changed
   if a finding needs it; then the merge, the same way, with §4.1's items; then U4c's L-c8 re-run on U4j.
3. **(Done 10:44)** The plan text edits; their open Lows join the advisor request below.
- **After wave 1:**
  - One advisor request: U4j's readings (a)-(f); U4d's points (A), (B), (C) and its new public items; NA1 and NA5;
    the plan text batch's open Lows (F10 first: its one-word change); and the U4e brief's rev 3 (W1-U4E's answers,
    W1-S1, W1-N2, the new carries).
  - Then wave 2 (U4d2, U4e), under W1-TARGET's lock.
- **This evening:** post tonight's night items to issue #16 (a check-in is scheduled for 19:30 UTC:
  trig_01W9mBGauhxrWnn1rSuweDic). Candidates, by what merges today: the U4e brief's 28-row recheck and rev 3 draft
  and a U4d2 builder brief carrying N2-M1 and N2-M2 (both need U4d merged); a skeptic check of U4a and U4b's merged
  code; one of U4c's.

**Open questions for Mustafa:**
- **NIGHT-BUDGET (asked 2026-11-11 about 10:25 UTC):** the day session and the night shift share the university
  account's usage. Last night this session's agents hit the limit at about 22:30 Amsterdam, and the night shift's 00:10
  and 01:10 runs left nothing (likely the same limit; unproven: the routines API shows only the last run). Options:
  (a) leave it (the queue still fit last night); (b) no new agent work in the day session after about 21:00
  Amsterdam; (c) he checks the routine's run history in claude.ai first. Recommended: (a), and (b) once a night's
  queue stops fitting.
- **Relayed 2026-11-11 about 10:45 UTC** (it had not been sent before: no earlier reply carries it): the advisor's
  answer to TCP-FREEZE's "Tell me if any later unit plans work on the model" (advisor-u4-wave1.md:546-554: none plans
  fidelity work; U6's send path and U8/R1's DelayStream self-test work on the wrapper's timing, outside the freeze),
  and its "For Mustafa" item 2 (the readings PR before R5 carries three more readings).
- **Answered 2026-11-10 about 20:40 UTC** (history.md, "Decisions taken"):
  - NETEM-BAR (a): the bar stays capped as WS-BAR says; a WebSocket row that fails because the kernel is slower than
    the model is reported to him as the kernel's, with the netem numbers, and he decides.
  - NS-START: he sets the routine to `10 0-7 * * *` himself tonight.

**Rules in force** (his words, verbatim, in history.md's "Decisions taken" unless marked)
- **Mustafa, 2026-11-11** (history.md, "Decisions taken"): STATUS's "In flight" table keeps one line for #40; his
  words move between docs by script and line ranges, never by retyping (done for PR #49 at 5ca77eb).
- **Mustafa, 2026-11-10** (history.md, "Decisions taken": his seven points and his "A"):
  - RESUME-CARD; WIP-CLEAN; SIZE-CHECK;
  - TCP-FREEZE: no more model fidelity work; if U8's netem check disagrees, netem wins;
  - SQUASH-40 with SQUASH-CITES (a): the closing checklist under Next steps;
  - PROMOTE: at U4's close; against SPIKE-SCOPE (a);
  - the second lane: merge main as usual, by the plain merge the guard allows, as at df8975b.
- **Mustafa, 2026-11-09:**
  - the night shift's role: post the next night items to issue #16 each evening and fold them in the morning;
    building and testing stay by day;
  - one shared Cargo target for every copy, `.gate/shared-target` (touch before builds, check the counts);
  - LEFTOVER-DELETE: "leave them"; no delete past the deny list.
- **Standing:**
  - write files with Write and Edit, never cd plus a redirect;
  - never switch the main folder; the guard merges only origin/main into it;
  - CHECKPOINTS.md;
  - the review budget and one severity scale (kit-v3-4);
  - Opus at xhigh for netcode;
  - the advisor reads main's advisor.md.
- **U4:**
  - the five briefs (U4e, U4r, U4f, U4g, U4h) go to the advisor before their builds;
  - carries word for word from u4s-carries.md, listed as met by number in each report;
  - the wave-1 seam freeze (u4-build-plan.md, Lanes).

**Pointers**
- The plan: plan.md (rev 3.3).
- U4: u4-build-plan.md, advisor-u4.md, u4s-carries.md, u4s-report.md, and Running work below (U4 era).
- History: history.md (Steps done U0-U3d; the old Current step; Decisions taken; Running work up to U3c; "Running
  work, U4 wave 1": the card's blocks of 2026-11-10).
- The closing checklist for PR #40: under Next steps.
- Resume commands (scripts under /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/workflows/scripts/;
  a resumed run keeps its run id, and finished agents replay from cache):
  - U4c's round 2: `Workflow({scriptPath: <scripts>/npt-u4c-fix-round2-wf_610486f7-cc1.js, resumeFromRunId:
    "wf_610486f7-cc1"})`.
  - U4d's fix pass: `Workflow({scriptPath: <scripts>/npt-u4d-fix-wf_a8ff8550-67d.js, resumeFromRunId:
    "wf_a8ff8550-67d"})`.
  - U4j's review: `Workflow({scriptPath: <scripts>/npt-u4j-review-wf_1ea8a0dd-fee.js, resumeFromRunId:
    "wf_1ea8a0dd-fee"})`.
  - A cut agent runs again with its context only if its prompt changes: add a dated RESUME line that points at its
    notes file.
  - U4j's builder: SendMessage to a3b8dfeb01ed52b50.
  - If a worktree is gone: a new one at its base, with its wip patch applied first.

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
