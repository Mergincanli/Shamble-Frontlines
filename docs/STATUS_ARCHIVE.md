# STATUS_ARCHIVE: the rest of STATUS
_What this holds: Mustafa's STATUS-DIET of 2026-11-10 puts "the rest (done change-sets, old measurements, closed risks)"
here, beside the short `docs/STATUS.md` and his decisions in `docs/DECISIONS.md`. Everything below was moved here by a
script, by line ranges, from `docs/STATUS.md` at main 2d22b69 (`git show 2d22b69:docs/STATUS.md`; the script, its line
ranges and its checks: `docs/reports/2026-11-10-status-diet.md`). Each section keeps the old section's name, so a
citation such as "STATUS, Routines" finds its text here; line numbers in older citations read at 2d22b69, and words such
as "below" and "above" read in STATUS's old order. The text is the record as it stood then; the current state is in
`docs/STATUS.md`._
_Two sections are live, kept current by docs-keeper (a reading of his "the rest", since they do not fit in 120 lines):
"Follow-ups" and "Open bugs and risks". The open items in them are listed by id in STATUS ("Follow-ups", "Open risks").
"In flight" is the table of done change-sets: a merged PR's row is added there. The advisor agent's rulings are here,
under "Decisions pending: the rest"; his own decisions are in `docs/DECISIONS.md`. A3's notes, cited here as
`.checkpoints/kit-v3-3/…`, were deleted in 8b7cfb3: read them at its parent 6c6c91a (`git show 6c6c91a:<path>`)._

## STATUS's header at 2d22b69
_Kept current by docs-keeper after every change-set. Mustafa and the Advisor read this first._
_Last updated: 2026-11-10; v2 phase 0 in progress. Since 2026-10-17: kit-v3-0 (PR #26), docs-after-kit-v2-2 (PR #25),
R7, the legacy inventory (PR #27), docs-after-r7 (PR #29), the R1–R6 research (PR #28, b6effde, 2026-10-31),
docs-after-v2-research (PR #31, 2f5b52e, 2026-10-31), docs-after-31 (PR #32, 9e7db28, 2026-10-31), the v2 decision
drafts (PR #33, e4d8b22, 2026-11-03), kit-v3-1 (PR #30, e2b5799, 2026-11-04 10:21 UTC), part-b-test (PR #34, 868e165,
2026-11-04 13:35 UTC, merged by the PR Advisor routine itself: TEST-RUN B2) docs-after-kit-v3-1 (PR #35, 4893ab6,
2026-11-04 15:50 UTC), B3, Mustafa's design walk (PR #36, 8bcc65e, 2026-11-04 19:58 UTC), kit-v3-2, A2 on his list (PR
#37, 12d7961, 2026-11-04 23:06 UTC, by merge commit, on Mustafa's go after the advisor's APPROVE of 342ff49) and
kit-v3-3, A3 on his list, the guard's follow-ups (PR #38, af8393e, 2026-11-05 21:25 UTC, by merge commit, on Mustafa's
go after the advisor's APPROVE of 8b7cfb3; CI green on main), the netcode proof test's readings (PR #41, d8c485a,
2026-11-06 13:35 UTC, by merge commit, on Mustafa's go after the advisor's APPROVE of 1b4094a; docs only) and B.1 on his
list, EDITOR_V2's plan view and his editor decisions (PR #39, ee736ec, 2026-11-06 17:09 UTC, by merge commit, on
Mustafa's go after the advisor's APPROVE of 4f5976b; CI green on main), B.2 on his list, VISION_V2's community-servers
note and B3's hand-over (PR #42, 4c6db5b, 2026-11-06 23:57 UTC, by merge commit, on Mustafa's go after the advisor's
APPROVE WITH NOTES of e223ecc; docs only; CI green on main), vfx-xp-notes, his VFX editor item in EDITOR_V2 §7 and his
private-match XP rule in VISION_V2 §5, both of 2026-11-06 (PR #43, 69ea88e, 2026-11-07 11:28 UTC, by merge commit, on
Mustafa's go ("merge #43") after the advisor's APPROVE WITH NOTES of 4e03bb5; docs only; CI green on main), ADR
0001's WebSocket send budget, the netcode session's (PR #45, 626df67, 2026-11-07 20:52 UTC, by merge commit, on
Mustafa's go after the advisor's APPROVE of 6ae3571; CI green on main) and kit-v3-4, A4 on his list, the review policy
(PR #44, 44446a5, 2026-11-09 18:21 UTC, by merge commit, on Mustafa's go ("merge #44") after the advisor's APPROVE WITH
NOTES of 7c36d5a; CI green on main after one re-run) merged. In flight: kit-v3-5, F1, the guard's rename gap, agent
memory included (PR #46; his GUARD-RENAME (c) of 2026-11-09 and his allow-list direction of 2026-11-10; the kit
commit 9bacf47; the review round and its fix pass, 08423ce; the report; next, the advisor's final-head review and his
go). `main` is protected: merging needs a pull request with
the `gate` and `smoke` checks green (Mustafa, 2026-10-17; rules read with the GitHub API). Approved unattended work:
[Night queue, issue #16](https://github.com/Mergincanli/Shamble-Frontlines/issues/16)._

## Phase
**v2 phase 0 (V2_PHASE0_BRIEF):** the v2 rebuild, a game-specific engine (`docs/VISION_V2.md` and
`docs/V2_PHASE0_BRIEF.md`, added 2026-10-24). Done: kit-v3-0, the process upgrades (PR #26, f2ccc13, 2026-10-25);
docs-after-kit-v2-2 (PR #25, 4c24d07, 2026-10-27; docs only); R7, the legacy inventory (PR #27, 07ab240, 2026-10-27);
docs-after-r7 (PR #29, e9f11be, 2026-10-27; docs only); R1–R6 research (PR #28, b6effde, 2026-10-31; docs only);
docs-after-v2-research (PR #31, 2f5b52e, 2026-10-31; docs only); docs-after-31 (PR #32, 9e7db28, 2026-10-31; docs
only; the v2 day session's handover); **the decision drafts** (PR #33, e4d8b22, 2026-11-03 21:57 UTC, by squash):
`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006` (Status: Proposed), `docs/SPIKE_PLAN.md`, `docs/adr/README.md` (the
index and the loose-ends register), GLOSSARY §8 (the phase 1 terms) and `docs/LOOK_V2.md` (the look brief, Mustafa and
the Advisor, added 2026-10-31); the six drafts are signed off as the basis for the spikes, not yet accepted (Status
stays Proposed; a draft becomes Accepted only when Mustafa signs it off after its spikes report; the stop's answers:
Decisions pending, below); kit-v3-1, the routine guard (PR #30, e2b5799, 2026-11-04 10:21:56 UTC, `merged_at`; the
merge commit is dated 10:21:55; on Mustafa's "merge #30" after the advisor's APPROVE of 7a23132); part-b-test
(PR #34, 868e165, 2026-11-04 13:35 UTC; docs only; merged by the PR Advisor routine itself: TEST-RUN B2);
docs-after-kit-v3-1 (PR #35, 4893ab6, 2026-11-04 15:50 UTC; rules and docs; on Mustafa's go after the advisor's APPROVE
of 76c35bf); B3 (the item on Mustafa's list, not the SharedArrayBuffer ring or ADR 0006 part 13's B3 row): his
2026-11-04 design decisions into LOOP_V2 and UNITS.md (PR #36, 8bcc65e, 2026-11-04 19:58 UTC, by merge commit; docs
only; on Mustafa's go after the advisor's APPROVE of a32e4c0); kit-v3-2 (A2 on his list: the CHECKPOINTS.md rules
for every session and the TOY-BOX rewording; PR #37, 12d7961, 2026-11-04 23:06:24 UTC, `merged_at`, by merge commit;
rules and docs; on Mustafa's go after the advisor's APPROVE of 342ff49; report `docs/reports/2026-11-04-kit-v3-2.md`);
kit-v3-3 (A3 on his list: the guard's follow-ups and his two additions of 2026-11-04, the worktree rule and the queue
into the repo; PR #38, af8393e, 2026-11-05 21:25:34 UTC, `merged_at`, by merge commit (parents 12d7961 and 8b7cfb3);
rules and kit code; on Mustafa's go ("merge #38") after the advisor's APPROVE of 8b7cfb3, PR comment 6669362242; CI
green on main after the merge; report `docs/reports/2026-11-05-kit-v3-3.md`; its limits: Open bugs and risks); B.1
(his EDITOR_V2 plan view of 2026-11-03 and his editor decisions of 2026-11-05; PR #39, ee736ec, 2026-11-06 17:09:30
UTC, `merged_at`, by merge commit (parents d8c485a and 4f5976b); docs only; on Mustafa's go ("merge #39") after the
advisor's APPROVE of 4f5976b, PR comment 6682184695; CI green on main after the merge; report
`docs/reports/2026-11-06-b1-plan-view.md`); B.2 (his VISION_V2 "Community servers (later)" note of 2026-11-03, plus
B3's hand-over; PR #42, 4c6db5b, 2026-11-06 23:57:35 UTC, `merged_at`, by merge commit (parents ee736ec and e223ecc);
docs only; on Mustafa's go ("merge #42") after the advisor's APPROVE WITH NOTES of e223ecc, PR comment 6698888516; CI
green on main after the merge (the gate workflow, run 42315225217, on 4c6db5b); report
`docs/reports/2026-11-06-b2-community-servers.md`); vfx-xp-notes (his VFX editor item of 2026-11-06 in EDITOR_V2 §7
and his private-match XP rule of 2026-11-06 in VISION_V2 §5; PR #43, 69ea88e, 2026-11-07 11:28:59 UTC, `merged_at`, by
merge commit (parents 4c6db5b and 4e03bb5); docs only; on Mustafa's go ("merge #43") after the advisor's APPROVE WITH
NOTES of 4e03bb5, PR comment 6701970962; CI green on main after the merge (run 42368356165); report
`docs/reports/2026-11-07-vfx-xp-notes.md`); kit-v3-4 (A4 on his list, the review policy: risk tiers 0-3, the review
budget, one severity scale, the rollback rule and disk hygiene; PR #44, 44446a5, 2026-11-09 18:21:13 UTC, `merged_at`
(the merge commit is dated 18:21:12), by merge commit (parents 626df67 and 7c36d5a); rules and docs; on Mustafa's go
("merge #44", reconfirming the rule changes at 7c36d5a) after the advisor's APPROVE WITH NOTES of 7c36d5a, PR comment
6740050583, and the PR Advisor routine's APPROVE, comment 6740560231; main's first CI run on 44446a5 (run 42673290304)
lost its `gate` job on GitHub's side, and its one re-run (attempt 2, 2026-11-10 06:37 UTC) was green; report
`docs/reports/2026-11-08-kit-v3-4.md`, with its correction of 2026-11-10).
**In flight:** kit-v3-5 (F1 from kit-v3-4, the guard's rename gap, agent memory included: his GUARD-RENAME (c) of
2026-11-09; PR #46; rules and kit code, risk tier 2): the PR Advisor merges only from an allow-list (the
guard's `mergeAllowed`), on both names of a renamed file and with whole lists; the kit commit 9bacf47 and the fix pass
08423ce, the report; next, the advisor's final-head review and his go (In flight, below).
**Queue** (Mustafa's words still to carry out, word for word, under Decisions pending, "Queued, in Mustafa's words"; one
change-set at a time, plan first), in his order (ORDER (b), 2026-11-10): KIT-FREEZE-2 (after kit-v3-5 merges, the kit
freezes again until the foundation's gate step; exceptions: a broken main, STATUS-DIET and LANE-RULES; what he meant,
his FREEZE-FDN, beside it), STATUS-DIET (docs), LANE-RULES (rules, with his Models addition and ACCOUNTS), then his unit
walk of 2026-11-09 (docs), then the Slice 1 note. A4's freeze lifted under his UNDER-WAY (a) of 2026-11-06 (the netcode
plan approved and its first build step committed, f6886bf, in PR #40).
**The WebSocket fallback (Mustafa, 2026-11-07):** the netcode proof test measures the send budget alone (the newest
snapshot only, about 1.5 round trips of snapshots, ADR 0001 part 10); the client's fast reconnect is a requirement for
the real client, built and measured before the first online playtest against the model's target (ADR 0001 OQ-0001-19,
`docs/design/ws-reconnect.md`), RESUME-CRED deferred with it; PM-0001-18's WebSocket bar takes the three-part form,
its numbers from the model's bound and one practice run with a 10 % tolerance fixed before it.
**Next (v2):** the netcode proof test, its own change-set, branched from main, plan first (no longer waiting on PR
#33). Wave 1 runs
one heavy spike at a time: the netcode proof test, the sim core spike (then the 60 Hz run), the boundary spike, the look
proof test once its models exist; the preview hosting check alongside any. The proof test still waits on: his OK of each
run's cost cap, in its plan (OQ-0006-20, answered); the GitHub macOS runner check for its Safari leg, then the Mac only
if that fails (MACHINES-TIMING, answered); and, for its own plan with security-auditor, the certificate method, how a
session holds its SSH key, and how it checks the new VMs' host keys. "Decisions needed" from PR #33: none open
(MACHINES-TIMING and LG-COUNT answered); OQ-0006-22 is a later item (before slice 3). For Mustafa: borrow and name the
laptop and desktop (rough checks first, then a short re-run), the Mac as the runner check decides; OK each run's cost
cap; the `.github/` workflow for the Safari check, if the proof test's plan puts it in this repository. Two later steps:
LOOK_V2 §8 reads "Spline or Blender" once the asset research's export part reports (that part's change-set makes the
edit, and it goes to Mustafa for the merge); the drafts take up the design answers at their next revision. Then phase 1
and the playable slices (SLICE-ORDER, below). The legacy game is frozen (critical fixes only).
**Kit-v3-1's setup steps, done 2026-11-04** (Mustafa's steps from `docs/reports/2026-10-27-kit-v3-1.md`, "For
Mustafa" 2–5; details in Routines, below): 1. both role environments exist (`SHAMBLE_ROLE=night-shift` and
`SHAMBLE_ROLE=pr-advisor`) and both routines point at them (Mustafa); 2. PR-HOME replaced, with no setup script
(PR-HOME-REPLACE (a)): the PR Advisor has a schedule trigger only, `CRON_TZ=Europe/Amsterdam 45 7,13,19 * * *`, can also
be started by hand ("Run now", as B2 was) or through the API, and is never started by pull-request events; its GitHub
pull-request trigger and its connectors are removed; 3. TEST-RUN passed (Mustafa accepted parts A and B): part A, B1
(13:20 UTC, second try) and B2 (the routine reviewed and merged PR #34 on its own); 4. the PR Advisor is back on:
Mustafa un-paused it, and the routines API (list_triggers, first read 14:03 UTC, again at 14:36 and 14:47 UTC) shows it
`enabled: true`, updated 14:01:31 UTC; its first scheduled run (18:45 UTC) failed on the usage limit (Routines). Paused
again by Mustafa, 2026-11-09 18:22 UTC, until kit-v3-5 merges (Routines).
V2_PHASE0_BRIEF §3 (step 2, the
routines: "effort high", lines 103 and 108; "Keep its triggers", line 108) is outdated by these facts and rulings; the
brief itself is not edited.
**Legacy phase 0 (environment): environment + green baseline**: merged (PR #2, 2d30e4a, 2026-10-14, Advisor APPROVE; CS-1 / PR #1 approved
after the fact). Status stays **PARTIAL** until Mustafa's Windows run ("Check a new machine" in tools/README.md: npm ci,
gate, headed RTX renders, headless-gl, `/permissions`, syntax-hook test); Windows-only fixes become a small follow-up
change-set. Gate check 5 is now on main, so that machine needs a fresh `npm ci` in tools/ (eslint-scope) and Node 22.13+.
**Merged 2026-10-14:** editor-prefab-list (PR #4, c458fe7), gate-check-5 (PR #5, 8675367), kit-v1-2 (PR #6, ef019e0),
bot-poplar-wedge (PR #7, 159bec4).
**Merged 2026-10-15/16:** editor-undo-ground (PR #8, b5f4ecf), docs-loop-v2 (PR #9, 67cf55b), map-dedupe-on-load
(PR #10), team-prep-fixes (PR #11, 513f2fc), docs-editor-v2 (PR #12, fec5e67), editor-hotfix (PR #13, 8f4c3b6), map-dedupe-on-load merged as 7ca2cc4.
**Merged 2026-10-17** (each with the Advisor's APPROVE; Mustafa's go where the rules need it): kit-v1-3 (PR #14, 9614ecf; reporting rule),
kit-v1-4 (PR #17, bd27eeb; CLAUDE.md "Night shift" section), kit-v1-5 (PR #18, 2303073; `.claude/NIGHT_SHIFT.md`),
kit-v1-6 (PR #19, cbd79e5; cut-off run pick-up, report kept current), **3a three-teams-core** (PR #15, 5ffe679),
docs-after-3a (PR #20, df1e6d3; docs only), kit-v2-0 (PR #21, 04c086c; the `advisor` agent), kit-v2-1 (PR #22, 211ddcb; automated merging), docs-after-kit-v2-1 (PR #24, e727ce4; docs only, merged by the PR Advisor), kit-v2-2 (PR #23, 48ffb51; the merge go comes after the APPROVE).
_Superseded by v2 (paused in issue #16); the legacy lines below, as of 2026-10-17, are kept for the record:_
**In flight:** none; next game work is N1 from the Night queue.
Editor v2 comes after the playable greybox loop (Advisor's queue; LOOP_V2 §11 defines the loop but has no editor step).
**Greybox phase (LOOP_V2 §11; superseded by v2):** the new core loop (three teams, harvest money, uplink tech, points race) is built as a
greybox prototype with bots, one change-set at a time; each plan says how it meets the [DECIDED] and [DEFAULT] items and
flags any [OPEN] item it needs decided; all numbers live in one tunable data table; [LATER] items are not built.
**Queue (Advisor, 2026-10-15; one change-set at a time; superseded by v2, paused in issue #16):** greybox phase: 1. three-teams, split by the approved plan into
0. team-prep-fixes (merged) → 3a three-teams-core (merged, PR #15) → **N1** (next; editor Play never rebuilds the nav
grid, a one-line change-set; Night queue item 1, APPROVED) → R19 (navBuild building-cell fix; Night queue item 2, NEEDS PLAN;
Advisor 2026-10-17: right after N1, before 3b) → 3b three-teams-hud → 3c spawn-and-buy → "ramps as slopes in SIM" (lets `_botFireWorld` go) →
2. octopon-3-greybox (NC-7 blast reach by line of sight comes before its playtests) →
3. harvest-economy → 4. bot-replan-storm (the navSteer replan storm, bot-poplar-wedge PERF-2) → 5. uplinks →
6. win-and-scoring → then the scale test on the Octopon-3 greybox (N = 12, 24, 36, 48). **Paused until the greybox loop
is playable:** tutorial-field-manual, ui-step-2, ui-step-3, build-spike and the ADR run (Prompt 3). house-textures-seeded
stays low priority and unslotted (seeded house textures also make before/after renders exact). The old order (client
visual/UI work first, gameplay after the foundation refactor) is paused by LOOP_V2 §12.

## In flight
| Work | Branch | State | Waiting on |
|---|---|---|---|
| **kit-v3-5, the guard's rename gap** (F1 from kit-v3-4, agent memory included: Mustafa's GUARD-RENAME (c) of 2026-11-09; his allow-list direction and SA5-RIDE (a) of 2026-11-10; rules and kit code, no game code, symbols +0 −0. The kit (9bacf47): the PR Advisor's merge check takes an allow-list, `mergeAllowed`: plain names only (letters, digits, `_`, `-`, `.`, `/`; no empty segment and none that starts with a dot; no `node_modules`), only `docs/`, `tools/`, `README.md` and `FIRST_PROMPTS.md`, and no rules name (`CLAUDE.md`, `CLAUDE.local.md`, `AGENTS.md`), no gate file's name under `tools/` and no decision doc (`docs/adr/**` and SA5's fixed list) in any letter case; it reads both names of a renamed or copied file, the whole file list (refused at 3000 files or more, counted against `changed_files`) and the whole check list (`total_count`). Part A: `protectedRel` and `CANON` gain `CLAUDE.local.md` and `AGENTS.md`, so writes and day-session asks cover them; `GATE_FILES` gains `tools/npm-shrinkwrap.json` (the advisor's E1). advisor.md's tier texts and gate-file lists, and CLAUDE.md's step 3, rows 0 and 1 and the line under the table, match the new checks (F4 folded in). Known limits: Open bugs and risks) | merged: PR #46 (`claude/sleepy-tesla-ofsxsr`, standing in for `cs/kit-v3-5`; local branch `kit-v3-5` from main 44446a5), 2d22b69, 2026-11-10 16:05:52 UTC (`merged_at`; the merge commit is dated 16:05:51), by merge commit (parents 44446a5 and 0573db3), on Mustafa's go ("merge #46", reconfirming the rule changes at 0573db3; his words: `docs/DECISIONS.md`, 2026-11-10) after the advisor's APPROVE WITH NOTES of 0573db3 (comment 6772168492); CI green on main (the gate workflow, run 42851159265, on 2d22b69); report `docs/reports/2026-11-10-kit-v3-5.md` | done: the facts; plan revision 1 and the advisor's REVISE (R1-R14, conditions 1-14; its rulings: F4-RIDE fold in, MERGE-ONLY approved); revision 2 and its APPROVE with D1-D6; his answers of 2026-11-10 (ORDER (b), SA5-RIDE (a), the allow-list direction, the known limits) and his second message (ACCOUNTS, FREEZE-FDN, SPLIT (a), SCOPE (a), the Slice 1 note); revision 3, the allow-list, and the advisor's APPROVE with E1-E5; the four kit writes, one ask each, approved by Mustafa: the tests (red on main's guard: 47 of 1086 fail), the guard (green: routine-guard 1086/1086, main 1034; settings 127/127; checkpoint 27/27), advisor.md, CLAUDE.md (9bacf47); gate PASS, symbols +0 −0; repo map OK; docs-keeper's pass (STATUS, ROADMAP, his queue word for word, the known limits, the kit-v3-4 report's correction); the review round on f9728c9: security-auditor, test-engineer and the verifier, each PASS WITH NOTES, nothing Critical or High (test-engineer's four Medium test gaps; Lows); the advisor's fix-pass ruling, no second round; the fix pass on his approval (08423ce: the test gaps closed, the empty-list and nameless-line refusals, the shrinkwrap in the night install check, advisor.md's tier-2 bracket; routine-guard 1107/1107, settings 127/127, checkpoint 27/27; gate PASS); the verifier's re-check of the fix pass, PASS WITH NOTES (three Lows: two test gaps, follow-ups that ride PX-1's change-set; one wording point the report explains); his PX1-ROUTE (a); the report `docs/reports/2026-11-10-kit-v3-5.md`; the notes deleted in the last commit (readable at 501c446); the advisor's APPROVE WITH NOTES of the final head 0573db3 (comment 6772168492), its Low notes: merge by merge commit so the 501c446 pointers stay in main's history, the verifier's two test gaps ride PX-1's change-set, the Windows reserved-name line right as written; his go of 2026-11-10 16:04:59 UTC | none (he re-enabled the PR Advisor routine at 16:07:30 UTC, after the merge; recorded by status-diet) |
| **kit-v3-4, the review policy** (A4 on Mustafa's list and A4's addition, both 2026-11-04; rules and docs, no game code, symbols +0 −0. The kit: CLAUDE.md's protocol takes risk tiers 0-3 (a table: a PR takes the highest tier any changed file or effect reaches, and a tier lifts no `needs-mustafa` trigger or Never line), a review budget (at most two rounds, a third only for an open Critical or High; below that a STATUS follow-up; a Low never triggers a push; the gate and the verifier are never cut), one severity scale (Critical, High, Medium, Low) and a rollback rule (when main breaks, a day session reverts that merge first, in a revert PR with no plan step, merged by today's classes; the fix is a new PR, plan first); advisor.md takes his words, the tiers mapped onto today's `ready` and `needs-mustafa` classes, per-tier checklists (what to check, what to ignore), the review budget and the revert steps; the changeset skill and security-auditor.md point at them (security-auditor reviews every risk-tier-2 change); `.claude/CHECKPOINTS.md` §10, disk hygiene (his item of 2026-11-08; guidance, no rule change). The docs (d8147cd): his foundation notes (items 4 and 6) in SPIKE_PLAN's foundation entry and ADR 0006, register clauses (R2-ROOM-OQ7, OQ-0006-14, I-18), his XP-TERM (a) and LOOK (a) answers of 2026-11-07, #43's four Low notes. No guard, settings, CI or gate file changes) | merged: PR #44 (`claude/sleepy-tesla-ofsxsr`, standing in for `cs/kit-v3-4`; local branch `kit-v3-4` from main 69ea88e; main 626df67, PR #45, merged in at 202951f), 44446a5, 2026-11-09 18:21:13 UTC (`merged_at`; the merge commit is dated 18:21:12), by merge commit (parents 626df67 and 7c36d5a), on Mustafa's go ("merge #44", reconfirming the rule changes at 7c36d5a) after the advisor's APPROVE WITH NOTES of 7c36d5a (comment 6740050583) and the PR Advisor routine's APPROVE (comment 6740560231); CI green on main after one re-run (run 42673290304: its first attempt lost the `gate` job on GitHub's side; attempt 2 green); report `docs/reports/2026-11-08-kit-v3-4.md` | done: plan revision 1 (parts A-F) and the advisor's APPROVE WITH CONDITIONS (13), with its rulings T0-MERGE (a) (his "auto-merge" is the PR Advisor's own merge, pinned to the head it approved: GitHub auto-merge skips the expected-head check), DOCS-RIDE (a) and SCALE-HOME (a) (`.checkpoints/kit-v3-4/advisor-plan-rev1.md`, readable at d96e215); the docs parts (d8147cd); Mustafa's answers of 2026-11-08 (ebbdaa2): DISK-WHEN (a), TIER0-SCOPE (a) (docs that record his words or decisions, the ADRs, the register, the design and look docs keep his go), REVERT-EXCEPTION (a) (day sessions only); his kit sitting of 2026-11-08, five kit edits one at a time: CLAUDE.md (252 to 277 lines), advisor.md (172 to 301), the changeset skill (32 to 36) and security-auditor.md (54 to 56) in 21d2b56, and CHECKPOINTS.md §10 (+12) in b85b2cf after the advisor's addendum check (REVISE with two text changes, approved once applied: `advisor-addendum-disk.md`); the kit tests unchanged and passing with main's counts (routine-guard 1034, settings 127, checkpoint 27); gate PASS; docs-keeper's pass (STATUS, ROADMAP, the queue; ad1f9bd); the review round on ad1f9bd: infra-engineer, the verifier and security-auditor, each PASS WITH NOTES (IE-1 and SA-2 High, fixed; SA-1 Critical, on main and outside the diff: his GUARD-RENAME); the advisor's fix-pass ruling (c3cf647) and the fix pass (9f0c6a2, 198c075: advisor.md to 324 lines, CHECKPOINTS.md §10 +9); the verifier's check of it FAIL on H1 (the revert check ignored whitespace), fixed at 3d39527 as the advisor ruled (`--verbatim`, and neither id empty) and re-checked PASS; follow-ups F1-F5 (below); the report `docs/reports/2026-11-08-kit-v3-4.md` (his words in its appendices A and B); the notes deleted in the last commit (readable at d96e215); the advisor's APPROVE WITH NOTES of the final head 7c36d5a: one Critical on main and outside the PR (the merge check missed force-added agent memory: joined F1, kit-v3-5's scope) and three Lows (F4 to name agent memory: taken by kit-v3-5; the report's mixed verdict rows and Appendix D: its correction of 2026-11-10); the PR Advisor routine's APPROVE of the same head, with one Low (two STATUS lines still called #44 a draft: fixed by kit-v3-5's docs pass); his GUARD-RENAME (c) and his go of 2026-11-09 | none |
| **vfx-xp-notes, the VFX editor and private-match XP** (vfx-xp-notes; Mustafa's two items of 2026-11-06, both "For the next docs pass", next by his VFX-ORDER (a). Docs only, symbols +0 −0: his VFX editor item, word for word, as EDITOR_V2 §7, "A future tool: the VFX editor (after the map features)", the tool `[LATER]` and each of his ten principles `[DECIDED]`, with a dated header line (EDITOR_V2:7); his private-match XP rule in VISION_V2's community-servers note, after the trust rule (VISION_V2:71-75): `[DECIDED]` 25 % of normal XP with bots in the match, 100 % humans only, and the Advisor's two suggestions `[OPEN]`, as he marked them; "bears on it" clauses in five register rows (`docs/adr/README.md:104`, `:105`, `:123`, `:161` and `:184`: OQ-0003-15, OQ-0003-16, OQ-0005-CD7, OQ-0006-21 and R2-ROOM-OQ7), no ADR draft or LOOK_V2 edited; STATUS and ROADMAP after PR #42; the B.2 report's corrections after the merge) | merged: PR #43 (`claude/sleepy-tesla-ofsxsr`, standing in for `cs/vfx-xp-notes`; local branch `vfx-xp-notes` from main 4c6db5b), 69ea88e, 2026-11-07 11:28:59 UTC (`merged_at`), by merge commit (parents 4c6db5b and 4e03bb5), on Mustafa's go ("merge #43") after the advisor's APPROVE WITH NOTES of 4e03bb5 (comment 6701970962); CI green on main (run 42368356165); report `docs/reports/2026-11-07-vfx-xp-notes.md` | done: plan revision 1 and the advisor's APPROVE with 10 conditions (`.checkpoints/vfx-xp-notes/advisor-plan-rev1.md`, readable at ced21da); the build (d5e2c62): EDITOR_V2 §7 and its header line, VISION_V2's XP block, the four register clauses; E1-E3 PASS (`e-checks.py`, 30 checks: his words byte for byte; EDITOR_V2 `[DECIDED]` 11 to 21 and `[LATER]` 2 to 3, VISION_V2 `[DECIDED]` 12 to 13 and `[OPEN]` 3 to 4); `gate.mjs 4c6db5b --no-smoke` PASS, symbols +0 −0 (`gate-build.txt`); docs-keeper's pass (STATUS, ROADMAP, the B.2 report's corrections; the follow-ups "From vfx-xp-notes"); the review round on e967196 (`verifier.md`, `game-designer.md`, `art-director.md`): each PASS WITH NOTES, nothing Critical or High (two Medium: the OQ-0003-15 link, and the fact that tells XP-TERM's options apart; the rest Low); the one fix pass: a fifth register clause (OQ-0003-15), VISION_V2's header line, the XP-TERM and LOOK follow-ups sharpened, two follow-ups added, STATUS:3's verdict wording (5912c68); the report `docs/reports/2026-11-07-vfx-xp-notes.md` (ced21da); the notes deleted in the last commit (4e03bb5; readable at ced21da); the advisor's APPROVE WITH NOTES of the final head 4e03bb5, with four Low notes, taken by kit-v3-4 (PR #44, d8147cd): the report's STATUS line number, why three register rows got no clause, "five register rows" in STATUS and ROADMAP, OQ-0003-15 in the LOOK entry; his XP-TERM (a) and LOOK (a) of 2026-11-07 recorded by kit-v3-4 too | none |
| **B.2, VISION_V2's community-servers note and B3's hand-over** (b2-community-servers; B.2 on Mustafa's list. Docs only, symbols +0 −0: his "Community servers (later)" note of 2026-11-03, word for word, at the end of VISION_V2 §5, tagged `[LATER]`: the why and the precedents in his words, the trust rule tagged `[DECIDED]` (progression counts on official servers only), and a pointer to its open question in the drafts (ADR 0006, OQ-0006-21); B3's hand-over: pillar 5's `[OPEN]` replaced by his Factions line of 2026-11-04, `[DECIDED]` at the level of principle (which units are shared or per faction still waits on his unit list: VISION_V2 §7, the register's OQ-0005-CD5), pointers in brackets on §5's account-progression line and §7's faction question, each old line's words unchanged; a dated header line; one clause in the register's OQ-0006-21 row (`docs/adr/README.md:161`), no ADR draft edited; STATUS and ROADMAP after PRs #41 and #39; the B.1 report's correction after the merge) | merged: PR #42 (`claude/sleepy-tesla-ofsxsr`, standing in for `cs/b2-community-servers`; local branch `b2-community-servers` from main ee736ec), 4c6db5b, 2026-11-06 23:57:35 UTC (`merged_at`), by merge commit (parents ee736ec and e223ecc), on Mustafa's go ("merge #42") after the advisor's APPROVE WITH NOTES of e223ecc (comment 6698888516); CI green on main (the gate workflow, run 42315225217, on 4c6db5b); report `docs/reports/2026-11-06-b2-community-servers.md` | done: plan revision 1 and the advisor's APPROVE with 9 conditions (`.checkpoints/b2-community-servers/advisor-plan-rev1.md`, readable at 5c0999f); the build (67978c2): VISION_V2's note, pillar 5's line, the two pointers and the header line, and the register clause; E1-E4 PASS (`e-checks.py`: his pieces byte for byte; nothing else changed in VISION_V2; tags `[DECIDED]` 10 to 12, `[OPEN]` 4 to 3, `[LATER]` 4 to 5, the rest unchanged; every `VISION_V2 §N` citation still resolves); `gate.mjs ee736ec --no-smoke` PASS, symbols +0 −0 (`gate-build.txt`); CI gate and smoke green on 67978c2; docs-keeper's pass (STATUS, ROADMAP, the B.1 report's correction; 26a5455, CI green), with Mustafa's VFX editor item of 2026-11-06 queued word for word; the review round on 26a5455 (`verifier.md`, `game-designer.md`, `infra-engineer.md`): the verifier PASS WITH NOTES (one Medium: B.2's added lines move ADR 0005's VISION_V2 line ranges, four found by the verifier and a fifth by the widened E4; five Low), game-designer PASS WITH NOTES (two Low), infra-engineer PASS WITH NOTES (two Low), nothing Critical or High; the one fix pass: STATUS and ROADMAP wording (A4's freeze in the past tense, the queue's order marked as the session's reading, his 2026-11-03 list as pointers, B3's line numbers dated), four follow-ups ("From B.2"), E4 widened to line ranges (8d1540f); the report `docs/reports/2026-11-06-b2-community-servers.md` (5c0999f); the notes deleted in the last commit (readable at 5c0999f); the advisor's APPROVE WITH NOTES of the final head e223ecc, with four Low notes, taken by vfx-xp-notes (PR #43): the queue entry's "Next after B.2", #40's stale lines (its STATUS :3, :31 and :173 at 0588a17), For Mustafa 1(b)'s wording, and the check script's FAIL (1) on the dated Appendix B title; its pull_request CI run 42313665674 stalled once installing Chromium's system libraries (Open bugs and risks) | none |
| **B.1, EDITOR_V2's plan view** (b1-plan-view; B.1 on Mustafa's list. Docs only, symbols +0 −0: his "Plan view: draw to build" of 2026-11-03 and the eight lines of his B.1 addition of 2026-11-05 (seven "Decided", one "Principle"), word for word, in a dated block in EDITOR_V2 §3, "Decided 2026-11-05", in his order, each tagged `[DECIDED]` with his own marks kept ("Principle", "lower priority"; P-TAG (a): no new tag), the plan view after the views line with its own date; a dated header line and a pointer to his Principle; pointers in brackets on §2 rule 4, "Select and transform", the "Validation panel", "Test", the `[LATER]` line and §6, each old line's words unchanged; a "bears on it" clause in the register's OQ-0005-CD4 row (`docs/adr/README.md:120`), no ADR draft edited; the A3 report's corrections after the merge (the advisor's two Low notes); STATUS and ROADMAP after PR #38's merge) | merged: PR #39 (`claude/sleepy-tesla-ofsxsr`, standing in for `cs/b1-plan-view`; local branch `b1-plan-view` from main af8393e), ee736ec, 2026-11-06 17:09:30 UTC (`merged_at`), by merge commit (parents d8c485a and 4f5976b), on Mustafa's go ("merge #39") after the advisor's APPROVE of 4f5976b (comment 6682184695); CI green on main (gate and smoke on ee736ec); report `docs/reports/2026-11-06-b1-plan-view.md` | done: plan revision 1 and the advisor's APPROVE with 14 conditions (`.checkpoints/b1-plan-view/advisor-plan-rev1.md`, readable at a3ee6f8); the build (6ddbdd9): EDITOR_V2 §3's block dated 2026-11-05, the plan view, the pointers, the OQ-0005-CD4 register clause, the A3 report's corrections; E1 (each of his nine pieces found exactly once in EDITOR_V2; in STATUS, B.2's words and his two lead lines once each), E2 (every old bullet unchanged but for at most one pointer; headings and §1-§6 unchanged) and E3 (`[DECIDED]` 2 to 11, every other tag count unchanged, no new tag) PASS, rerun by docs-keeper at 6ddbdd9 [verified]; `gate.mjs --no-smoke` PASS, symbols +0 −0 (the builder's run, `.checkpoints/b1-plan-view/builder-gate.txt`, readable at a3ee6f8); docs-keeper's pass (STATUS, ROADMAP; 2f7988f, 4f57d26); the review round: the verifier PASS WITH NOTES (B1-V1 to B1-V4, Low), game-designer PASS WITH NOTES (B1-GD1 to B1-GD16, 6 Medium and 10 Low, none in his words), nothing Critical or High; the one fix pass (1d79b57: clearer and three more pointers, the slice-5 note, the CD4 clause, six follow-ups; E1-E3 and the gate PASS again); the report `docs/reports/2026-11-06-b1-plan-view.md` (E1 PASS on its appendix); the notes deleted in the last commit (readable at a3ee6f8); the advisor's APPROVE of the final head 4f5976b, with two Low notes: STATUS still called #39 a draft (taken by B.2), and the report's risk 3 left out one point of plan condition 8, that Test is the editor's own mode and ADR 0002 keeps the editor out of the player build (it stands in the plan, readable at a3ee6f8) | none |
| **kit-v3-3** (A3 on Mustafa's list: the guard's follow-ups; rules and kit code, no game code. His items: an ask before kit and gate edits in day sessions; the `sk-` key after `%20`, `%2C` or `%3A`; read-only GitHub Actions tools such as `actions_get` for pr-advisor; and his two additions of 2026-11-04: the worktree rule from the freeze, and the queue files into STATUS's follow-ups. security-auditor reviews it) | merged: PR #38, af8393e, 2026-11-05 21:25:34 UTC (`merged_at`), by merge commit (parents 12d7961 and 8b7cfb3), on Mustafa's go ("merge #38") after the advisor's APPROVE of 8b7cfb3 (comment 6669362242); CI green on main (gate and smoke on af8393e); report `docs/reports/2026-11-05-kit-v3-3.md` | done: plan revision 1 (a critic's points folded in) and the advisor's APPROVE with conditions C1-C12; E10, the doc quotes: five wording contradictions; the advisor's E10 ruling: PROCEED with changes E10-1 to E10-9; plan revision 2. Built (plan revision 2 §8): step 2, these Follow-ups and the queued words (93b9a07); steps 3-5 (f22c4df): `--output` refused for `git rev-list`, `diff`, `log` and `show`, `rev-list` for routines; in a day session a shell write to a kit or gate file asks and an agent's is refused; switching, merging, pulling or bisecting the main folder asks unless it brings in only reviewed main; the read-only Actions methods for pr-advisor, on this repository only; step 4b (2e82216, the advisor's step-4 ruling R1-R4): the Edit and Write tools' ask, and four fixes from an automated security review of f22c4df; step 6 (2e82216): the saver's secret check finds `sk-`, AWS and Slack keys behind `%XX` and other escapes; step 7 (2e82216): the rule texts, without the WT-NIGHT parts (step 9 adds them); step 4c (bbcb96b; `.checkpoints/kit-v3-3/builder-step4c.md`): the two findings of a second automated review of 2e82216 (a `CDPATH` set on the line; `git switch` to a name a tag shares); tests 555/555 (routine-guard 401, settings 127, checkpoint 27); E11a (`.checkpoints/kit-v3-3/e11a.md`): the ask stops a kit-file Write in default, acceptEdits and auto modes; the full gate with smoke PASS (5a: index 8/8, editor 5/5, 0 errors; step 8's run, the main session); step 8's docs (442fbf7, the head the reviews read). The reviews of 12d7961..442fbf7 (`.checkpoints/kit-v3-3/security-auditor-review.md`, `verifier-review.md`): security-auditor FAIL on SA-1 (this diff's regression: after a cd the guard could not follow, a relative write was checked against the project folder, so a kit write could get no ask), with SA-2 to SA-10 (SA-2 major and pre-existing: a routine could forge origin/main); the verifier PASS WITH NOTES (F-a, F-b, F-c, N-1 to N-3); a third automated security review of bbcb96b (its findings 7 and 8: a quoted `CDPATH`, `cdable_vars`); skeptics on six findings: five upheld, one refuted (F-a). The advisor's fix-round ruling (`advisor-fix-round.md`): PROCEED. The fix round (2acfbbc; `builder-fix1.md`, `builder-fix2.md`): the cd rules R-a to R-e (the folder is known only where the line's text fixes it; an unknown folder makes a relative write ask and an agent's refused; step 4c's `CDPATH` code deleted as dead); SA-2's routine half (every main check compares with `refs/remotes/origin/main`'s commit; routines refuse a fetch or pull refspec with `:` or a leading `+`, new branch names such as `origin/…` or `main`, and `git remote` and `git symbolic-ref` beyond their read forms); F-b (`git worktree add` or `git clone` into a kit folder asks, an agent's refused); SA-5 (`sed --in…` abbreviations); SA-6 (a kit folder's ancestors and nested kit files); SA-8 (`GIT_DIR` and its kin anywhere in a day line ask); tests 716/716 (routine-guard 562, settings 127, checkpoint 27); one mutant per new rule, 28/28 and 33/33 killed; the moved cases listed (seven by the cd rules, six of them from no prompt to an ask; T9 by F-b); the full gate with smoke PASS (5a: index 8/8, editor 5/5, 0 errors). Step 9, WT-NIGHT (a) (ed3ea8c; `builder-step9.md`): the night shift's `git switch` is refused unless the branch's rules, CI and gate files equal main's (compared with `refs/remotes/origin/main`'s commit), and the texts in CLAUDE.md's Never bullet, NIGHT_SHIFT.md step 4 and CHECKPOINTS.md §8; tests 768/768 (routine-guard 614, settings 127, checkpoint 27); its mutants 13/13 killed; the full gate with smoke PASS. The delta review of 442fbf7..ed3ea8c (`security-auditor-delta.md`, `verifier-delta.md`): security-auditor FAIL on SD-1 (the night switch check read the refs before the line ran, so a fetch on the same line could move the branch after the check) and SD-2 (the night push and commit checks and the main-folder check read plain name lists, so a kit file renamed away or with a non-ASCII name was missed), with SD-3 to SD-9; the verifier FAIL on V-2 (the same as SD-1) and V-3 (the switch check found a plain branch name through git's guessing, so a tag named `refs/heads/<name>` could answer it), with V-1, V-4, V-5 and N-a; skeptics upheld every finding; two more automated security reviews, of 2acfbbc and ed3ea8c (findings 9 to 13; 13b refuted). The advisor's fix-pass-2 ruling (`advisor-fix-pass2.md`): PROCEED. Fix pass 2 (c8cbb1a; `builder-pass2-routine.md`, `builder-pass2-day.md`): routines: a night `git switch`, `git commit` or `git push` must be the only command on its line (SD-1; not for pr-advisor, whose step-0 line stays allowed), the switch target looked up by exact ref names (V-3), and the name lists read with `-z --no-renames --no-relative` (SD-2, a named C6 edit); day sessions: a main-folder switch or merge after an earlier git command on the line asks unless its target is `origin/main` (SD-3. So `git stash && git switch main` asks too: a fetch earlier on the line can move local main.), a cd joined by `&&` after a cd the guard could not follow stays unknown (SD-4), clone targets and `--revision` (SD-5), a folder delete or move over a worktree asks (SD-6), and `function`, `trap` and `alias` count as command words that can move the folder (V-1); NIGHT_SHIFT.md step 4: fetch first in its own call; tests 864/864 (routine-guard 710, settings 127, checkpoint 27); every new rule's and flag's mutant killed, real-git rows included; no existing case's expected value moved; main's test file 180/192 on the new guard (3 pre-existing, 8 rev-parse scaffold, 1 SD-2 key scaffold); the full gate with smoke PASS (5a: index 8/8, editor 5/5, 0 errors). Pass 3, the review of ed3ea8c..01b158c (`security-auditor-pass3.md`, `verifier-pass3.md`, `skeptic-pass3-*.md`): security-auditor FAIL on P3-1 to P3-3 (a shell comment, a here-document delimiter with a quoted or escaped space, and ANSI-C `$'…'` quoting each hid whole commands from the guard's reader: on real bash, a night switch to a branch with other settings, and a CLAUDE.md write by the night shift or pr-advisor) and P3-5 (the night commit check missed paths named on the commit line, `.` and globs included; the push check still stopped the push), all four on routine paths and all on main 12d7961 too, with P3-4 and P3-6 (minor), P3-7 (minor, a day false ask on a clone into `.gate/`) and P3-8 (a nit); the verifier PASS WITH NOTES (864/864, mutants 48/48, main's test file 180/192 with the named 12, the full gate with smoke PASS; its two notes are P3-5 and P3-7); skeptics upheld all four majors. The advisor's pass-3 ruling (`advisor-pass3.md`): PROCEED inside A3, with its own P3-9 (`>&` before a file name, read by bash as a write into the file: on real bash a night or pr-advisor line overwrote CLAUDE.md or `.claude/settings.json`; on main too). Mustafa's scope stop (2026-11-05, Decisions pending): the day parser rules dropped, D-ASK instead, and the review budget from pass 4 on. Fix pass 3 (9589bfb; `builder-pass3-routine-a.md`, `builder-pass3-routine-b.md`, `builder-pass3-day2.md`): routines: R-PLAIN (a routine's line must be in a plain form the guard reads exactly as bash does, or it is refused; it closes P3-1 to P3-3 and P3-9 for routines), git in the background refused (P3-4), a `date +FORMAT` of plain characters only, the night commit allowlist (P3-5). A night `git commit` takes only `-m`/`--message`, `-a`, `-q`, `-s` and `--allow-empty`; no paths or pathspec options, so the guard checks exactly the staged list (and the unstaged list with `-a`). The name lists read with `--ignore-submodules=none` too (P3-6); day sessions: D-ASK (a line in one of five forms the guard does not read asks, an agent's is refused; no new parsing), the clone fix (P3-7), and P3-8 named; NIGHT_SHIFT.md step 7: write commands plainly, commit with `git add` first, git in the foreground; tests 1084/1084 (routine-guard 930, settings 127, checkpoint 27); C-PLAIN, the corpus test against real bash: PASS (212 corpus lines and 2000 random ones, 10426 commands); every new mutant killed (the routine side 31, plus 4 of main's mutants rerun behind R-PLAIN; the commit allowlist 11, plus 6 on real git; D-ASK and the clone fix 20); no routine line looser (1260 role decisions against 01b158c: 0 looser); the moved cases named (eight refusals that now give R-PLAIN's or P3-4's reason, one day row that now asks by D-ASK; five new clone rows that 01b158c asked on now pass, P3-7); main's test file 180/192, the same 12 as at 01b158c; the full gate with smoke PASS (5a: index 8/8, editor 5/5, 0 errors). Pass 4, the one final review, of 01b158c..60ae124 (`security-auditor-pass4.md`, `verifier-pass4.md`): security-auditor FAIL on P4-S4 (Critical: a worktree under `.gate/` or the scratchpad let a routine write, commit and push a `.claude/` or `.github/` change) and P4-S1, P4-S2, P4-S3 and P4-S5 (High: a wrapper or keyword given by a path, a quoted program after a keyword, a command with no program, xvfb-run's file options), all on routine paths, with P4-S4, P4-S1, P4-S3 and P4-S5 on main 12d7961 too; and P4-S6 (Medium), P4-S7 and P4-S8 (Low); the verifier PASS WITH NOTES (1084/1084, the full gate with smoke PASS; its notes P4-V1 to P4-V4, Low or an observation); the automated review of 9589bfb (`security-review-auto-6.md`): its finding 14, Medium, day (the clone folder after unknown options). The advisor's pass-4 ruling (`advisor-pass4.md`): PROCEED with a narrow fix pass 4 inside A3, checked by the verifier alone; every finding at High or above fixed now, everything below High a STATUS follow-up (FU-P4a to FU-P4c, Decisions pending; P4-S7, P4-V2 and P4-V3 fixed in the final docs pass); with its own A4-1 (High, routine, on main too: a failed cd followed by `;` let a night or pr-advisor redirect overwrite the live `.claude/settings.json` on real bash). Fix pass 4 is commit 0c481e1 (`builder-pass4.md`): "Fix pass 4 (pass 4's one Critical and four High findings, all on the routine side, plus one High the Advisor found; five of the six were on main too): R-PROG narrows R-PLAIN so every routine command starts with git, gh, npm, node, python, python3 or date by name, which closes wrappers and keywords given by a path, a quoted program after a keyword, a command with no program, xvfb-run's file options and a failed cd followed by `;`. And a routine's kit-path checks also read a path by its place in its own worktree: the night commit and push checks (and checkpoint save and watch) test the name git prints, the path the commit or push publishes, and a routine's Write, Edit and redirect checks use the worktree-aware kit-path test, so a worktree under `.gate/` or the scratchpad cannot carry a rules or CI change into a commit or a push. Checked by the verifier alone (the review budget)." The builder's counts: tests 1188/1188 (routine-guard 1034, settings 127, checkpoint 27); PASS4_ROUTINE_CASES 98 rows and 12 new mutants, all killed, plus the real-git worktree rows and their 4 real mutants; C-PLAIN's teeth 4/4; no routine line looser (1348 role decisions against 60ae124: 0 looser, 59 stricter, none of them in the routine texts; 983 row decisions: 0 looser by kind, 110 moved, all named); six of main's mutation rows moved behind R-PLAIN or the kit-path checks, byte for byte (four for R-PROG, under the advisor's C6 conditions; two for the kit-path test, which the builder names as outside the ruling's literal permission); main's test file 179/192 (the same 12 as before, plus #181, whose refusal text is now R-PROG's); NIGHT_SHIFT.md step 7 takes the ruling's sentence; the full gate with smoke PASS (5a: index 8/8, editor 5/5, 0 errors) [assumed: the builder's numbers, not re-run by docs-keeper]. The verifier's check of fix pass 4 (`verifier-fixpass4.md`), alone under his review budget: PASS WITH NOTES, nothing Critical or High (1188/1188; every probe line that did harm on 60ae124 refused; no routine line looser, 1348 role decisions; every command the routine texts give still passes; main's test file 179/192 as named; the full gate with smoke PASS; its one Low, P4F-V1, is follow-up FU-P4d). The report: `docs/reports/2026-11-05-kit-v3-3.md`; the notes and the checkpoint deleted in the last commit (8b7cfb3; readable at its parent 6c6c91a); the advisor's APPROVE of the final head 8b7cfb3, its two Low notes taken by B.1 as the report's corrections after the merge | none |
| **kit-v3-2** (A2 on Mustafa's list; rules, no code, symbols +0 −0. `.claude/CHECKPOINTS.md` takes his four anti-waste rules of 2026-11-03 for every session (day sessions, the night shift, the PR Advisor): at most 3 agents at a time, workflow agents included (§4, §7); each agent writes its plan into its notes first, then saves each finished block at once and a notes line after each finding, and a restart continues after the last complete block and trusts the blocks its notes mark done: it never rereads, rechecks or redoes them, though a later block may still read a file it needs, and its answer comes from the notes alone, opening no other file (§4; E9-1); scratch text and small scripts in the notes folder, builds and binaries under `.gate/<task-slug>/` (§4); the background saver checked and restarted at every checkpoint update while agents or a workflow run (§3, §5). Restated in CLAUDE.md ("Checkpoints", "Agents", "Ultracode and workflows"), NIGHT_SHIFT.md rule 7, the `changeset` skill, advisor.md (its plan review checks the cap), verifier.md, perf-auditor.md, rust-reviewer.md and tools/README.md:4. TOY-BOX with LOOK-LINES (B): CLAUDE.md:4-6 and :23-24, art-director.md (its first line, its v2 paragraph, its tone line) and graphics-engineer.md point at `docs/LOOK_V2.md` and never decide the look. Docs: STATUS, ROADMAP, REPO_MAP, START_HERE:112) | merged: PR #37, 12d7961, 2026-11-04 23:06:24 UTC (`merged_at`), by merge commit (parents 8bcc65e and 342ff49), on Mustafa's go after the advisor's APPROVE of 342ff49 (comment 6643739289); report `docs/reports/2026-11-04-kit-v3-2.md` | done: plan revision 1 and the advisor's APPROVE with 11 conditions; Mustafa's four answers (Decisions pending); plan revision 2; the build and docs-keeper's first pass (2c12306; notes 8c2abd6; the builder's E1–E8 and E10 PASS, `node --test` 339/339); the review wave (art-director, security-auditor, the verifier, 3 at a time): each PASS WITH NOTES, of 12 findings 9 upheld and fixed (acfc17e) and 3 refuted; E9, the dry run of the new paste block (25 runs on five versions of its text): its finding E9-1 (restarted agents re-checked the blocks their notes marked done) fixed in §4 (65cd40e, then 49f9898 after the verifier's fix check: PASS WITH NOTES, its note N2-1); the final text passed 3 of 3 restarts after a cut, the DONE case and the fresh case; 15 files vs main, `gate.mjs --no-smoke` PASS at the final head, symbols +0 −0 (the report) [assumed: not re-run by docs-keeper]; the report; the final docs pass; the advisor's APPROVE of the final head 342ff49, its deferred points F-1, R-1 and R-2 in Follow-ups (Decisions pending) | none |
| **B3, Mustafa's design walk** (b3-design-walk; B3 is the item on his list. Docs only, symbols +0 −0: his 2026-11-04 decisions and ideas, word for word, tagged `[DECIDED]`, `[IDEA]` (a new tag) or, for the war around the matches, `[OPEN]`, plus one Advisor `[DEFAULT]` reading in §13 (no learn-by-doing layer), which Mustafa may veto, in LOOP_V2 (§2 spawn protection, §4–§5 factions and infiltrators, §7 active reload and sprint-to-fire, §8 account progression, §10 bots, new §13 amps, loot and ammo) and UNITS.md §0 (factions, the tier-one amp ladders, infiltrators, faction ideas); his answers B3-Q1 to B3-Q3; pointers in DESIGN_NOTES; "bears on" clauses in `docs/adr/README.md`'s register, no ADR edited, no new open question; plus #35's final-review follow-ups in STATUS and ROADMAP; B3's hand-over: VISION_V2 lines (as numbered before B.2) 36-37 (pillar 5's `[OPEN]`, which quotes LOOP_V2's old "infantry shared, tanks per faction"), 59 (account progression) and 86 (faction asymmetry), against LOOP_V2 §5 and §8 of 2026-11-04, for B2, taken by B.2 (PR #42). Report: `docs/reports/2026-11-04-b3-design-walk.md`) | merged: PR #36, 8bcc65e, 2026-11-04 19:58:50 UTC (`merged_at`; the merge commit is dated 19:58:49), by merge commit, on Mustafa's go after the advisor's APPROVE of a32e4c0 | done: plan revision 1, the advisor's APPROVE with 11 conditions; plan revision 2 adds Mustafa's answers, the advisor's APPROVE with 10 conditions; the build and docs-keeper pass (6a09137; the builder's checks PASS, `gate.mjs --no-smoke` PASS, symbols +0 −0); the review round on the final diff (game-designer, the verifier, a completeness critic: each PASS WITH NOTES; 18 findings: a skeptic upheld 15, 13 of them distinct, all handled in the fix commit, and refuted 3); the report written (`docs/reports/2026-11-04-b3-design-walk.md`); the advisor's APPROVE of the final head a32e4c0. Its final-review follow-ups: STATUS and ROADMAP taken by A2 (PR #37); the report's deviations line and length nit wait for a later pass (Follow-ups) | none |
| **docs-after-kit-v3-1** (rules and docs, symbols +0 −0: CLAUDE.md's routine lines, advisor.md's PR review, STATUS, ROADMAP and REPO_MAP record PRs #33, #30 and #34, the routine setup and TEST-RUN; PORTAL-EMBED as open questions OQ-0002-9 and OQ-0006-23; two small fixes in the kit-v3-1 report and LEGACY_INVENTORY KT14. It takes over the docs-after-33 row, which the advisor's ruling Q2 kept out of kit-v3-1's A1) | merged: PR #35, 4893ab6, 2026-11-04 15:50:40 UTC (`merged_at`; the merge commit is dated 15:50:39), on Mustafa's go ("Merge $35", read as #35) after the advisor's APPROVE of 76c35bf | done: plan revision 1 (381471d), the advisor's APPROVE with 16 conditions; revision 2 (0099b3c) adds Mustafa's answers; reviewed once on the final diff (five reviewers, a skeptic per finding: 30 of 31 upheld and fixed); report `docs/reports/2026-11-04-docs-after-kit-v3-1.md`. Its final-review follow-ups are taken by B3 (PR #36) | none |
| part-b-test (docs only: the kit-v3-1 report's corrections, the advisor's #30 findings 1 and 2, 822aaa4; the PR that TEST-RUN B2 reviewed and merged) | merged: PR #34, 868e165, 2026-11-04 13:35:00 UTC (`merged_at`; the merge commit is dated 13:34:59), by the PR Advisor routine itself with `expectedHeadSha` a432692ff3800b23dc186051b8b1590778183bed | done: TEST-RUN B2, the routine's first unattended merge under the guard (Routines) | none |
| **v2-decision-drafts** (docs only, symbols +0 −0: ENGINE_OVERVIEW, ADR 0001–0006 Proposed, SPIKE_PLAN, `docs/adr/README.md`, GLOSSARY §8, LOOK_V2.md; plan revisions 3 to 6, each with the advisor's APPROVE with conditions) | merged: PR #33, e4d8b22, 2026-11-03 21:57 UTC, by squash | done: verified against the research, ten domain reviewers on the final diff; the stop answered (2026-11-03): signed off as the basis for the spikes, Status Proposed (Decisions pending). Report `docs/reports/2026-11-01-v2-decision-drafts.md` and its appendix (§9: where each answer landed) | none |
| docs-after-31 (docs only: STATUS and ROADMAP after #31's merge, and the v2 day session's handover) | merged: PR #32, 9e7db28, 2026-10-31 | done | none |
| **docs-after-v2-research** (docs only, symbols +0 −0: STATUS, ROADMAP, the research folder and VISION_V2 §3 catch up with PR #28's merge and the 2026-10-31 decisions; the research files carry the answers, 39 exact edits in af2560c; VISION_V2 §3 gets the totals reading, VISION-LINE (a)) | merged: PR #31, 2f5b52e, 2026-10-31, on Mustafa's "merge #31" after the advisor's APPROVE of 1c687d1 | done; the report is PR #31's body | none |
| **R1–R6 research** (six topic files in `docs/research/`, docs only, symbols +0 −0. Labels: R1 and R2 "verified (every recommendation claim)"; R3 "spot-checked (45 of 77 key claims)", R4 "spot-checked (50 of 52 key claims)", R5 "spot-checked (61 of 115 key claims)", R6 "spot-checked (21 of 42 key claims)". Shared: `GLOSSARY.md` (34 terms, starting with the tick budgets) and `UNREAD_SOURCES.md` (180 sources not read, with what each was needed for). The consistency critic's 12 contradictions: 7 resolved, 1 aligned into one shared question, 4 escalated (see Decisions pending). Report `docs/reports/2026-10-31-v2-research.md` and its appendix) | merged: PR #28, b6effde, 2026-10-31, on Mustafa's "merge #28" after the advisor's APPROVE of 854d42f | done; report `docs/reports/2026-10-31-v2-research.md` | none |
| docs-after-r7 (docs only: STATUS and ROADMAP catch up with v2 phase 0 and the Advisor's 2026-10-27 decisions) | merged: PR #29, e9f11be, 2026-10-27, on Mustafa's go after the advisor's APPROVE of d973b3f | done | none |
| **kit-v3-1** (routine guard for the two cloud routines. New `.claude/hooks/`: `routine-guard.mjs` (rules: `decide`), `guard.mjs` (launcher, fails closed for routines), `routine-guard.test.mjs`, `settings.test.mjs`. In a routine the shell runs only git, gh, `npm ci` and node on main's scripts (reading/searching via the Read/Grep/Glob tools) [verified 2026-11-04 in `routine-guard.mjs`: also `date`, and python on main's tracked scripts (:445–446); the PR Advisor runs no scripts (:458)]; routines never read the web, can't push to main or force-push or edit `.claude/`/`CLAUDE.md`, merges fail closed while the guard loads, nothing is ask-first. Day sessions unchanged. Also `.claude/settings.json` (empty ask, allow/deny, `effortLevel: "high"`, guard on PreToolUse/PermissionRequest/ConfigChange), procedures in NIGHT_SHIFT.md / CHECKPOINTS.md / advisor.md / CLAUDE.md / the 11 agent files (memory: local). Plus the GUARD-FILE (a) fix (Mustafa, 2026-11-03, after a day session froze when `guard.mjs` vanished under loaded hook settings): the three guard handlers start `guard.mjs` through a one-line `node -e` bootstrap; when `guard.mjs` is missing or can't load, a day session goes on and a routine is blocked; a routine is also blocked when `guard.mjs` ends without its own exit (amendment A1-AM1). No game code, symbols +0 −0) | merged: PR #30, e2b5799, 2026-11-04 10:21:56 UTC (`merged_at`; the merge commit is dated 10:21:55), on Mustafa's "merge #30" after the advisor's APPROVE of 7a23132 | done. A1 (Mustafa, 2026-11-03; plan revision 2, the advisor's APPROVE with conditions C1–C9): updated onto main e4d8b22 (af0c5dc, 2026-11-04) and the GUARD-FILE (a) fix (537c7a8) with its amendment A1-AM1 (deded41, the advisor's APPROVE; `settings.test.mjs` 122/122, 40 new, including the hook command run with `guard.mjs` removed, broken, unfinished and present; `routine-guard.test.mjs` 192/192 on Linux, unchanged); security-auditor PASS WITH NOTES on the fix and on the amendment. Report `docs/reports/2026-10-27-kit-v3-1.md`. His setup steps and TEST-RUN: done (Phase; Routines) | none |
| **R7 legacy inventory** (`docs/LEGACY_INVENTORY.md`: 247 systems, each with a recommended mark: rewrite in Rust 45 · port to TypeScript 74 · redesign 96 · drop 32; 118 rows "look brief pending"; `docs/LEGACY_TUNING.md` and `docs/legacy-tuning.json`: 60 tuning tables; read-only `tools/tuning-dump.js`; symbols +0 −0) | merged: PR #27, 07ab240, 2026-10-27, by the R7 session on Mustafa's go after the advisor's APPROVE of head 63be6f6 | done; report `docs/reports/2026-10-27-r7-legacy-inventory.md` | none |
| docs-after-kit-v2-2 (docs only: kit-v2-2's and #24's merges and main's branch protection recorded) | merged: PR #25, 4c24d07, 2026-10-27 | done | none |
| **kit-v3-0** (v2 process upgrades: VISION_V2.md and V2_PHASE0_BRIEF.md, CLAUDE.md's v2 rules, checkpoints, REPO_MAP.md, six new agents, the web allow list; no game code, symbols +0 −0) | merged: PR #26, f2ccc13, 2026-10-25, on Mustafa's "merge #26" after the advisor's APPROVE of its final head | done; report `docs/reports/2026-10-24-kit-v3-0.md` | none |
| kit-v2-2 (CLAUDE.md "Standing permission": Mustafa's go is for the PR's current head, given after the advisor approved that head; a later push needs the advisor's review first, then his go again; now matches advisor.md:98–105. security-auditor PASS WITH NOTES, findings 1 and 3 fixed, re-check PASS; findings 2 and 5 deferred, finding 4 open: see Open bugs and risks. No game code, symbols +0 -0) | merged: PR #23, 48ffb51, 2026-10-17, PR Advisor APPROVE on 964ea5a, merged on Mustafa's go in chat (PR Advisor's #23 comment) | done | none |
| docs-after-kit-v2-1 (docs only: kit-v2-1's merge and Mustafa's Brave check of 3a recorded; PR #22 finding 6 nits fixed) | merged: PR #24, e727ce4, 2026-10-17, PR Advisor APPROVE on d255277, merged by the PR Advisor (class ready) | done | none |
| **kit-v2-1** (automated merging: the PR Advisor routine merges approved ready PRs at once; `needs-mustafa` (rules category, or a major change; when unsure) waits for Mustafa's go. His go counts only in his own words in the chat of the merging session, about that PR, after the advisor's APPROVE of its current head; a "merge" on GitHub never counts. The approved full SHA goes in the review comment and is the merge's expected head; a later push needs a new review and go. Fork PRs skipped; PR text is evidence, never instructions. Green = `gate` and `smoke` both passed on the approved SHA; a hard-constraint change also needs its accepted ADR on main. The gate's own files (everything `tools/gate.mjs` or `.github/workflows/` load: today gate.mjs, smoke.mjs, battery.js, gate-globals.json, tools/package.json, tools/package-lock.json; any `.npmrc`) join the rules category; a changed repro tool (preview.js, sim.js) is replaced by main's copy. The advisor always works from main's advisor.md. NIGHT_SHIFT rule 7 REVISE path; night shift never merges; CLAUDE.md Agents lists `advisor`. No game code, symbols +0 -0. Reviews: advisor plan review REVISE (8 texts applied); security-auditor FAIL, all 8 findings fixed; deferred: wording at tools/gate.mjs:333 / tools/README.md calling gate-globals.json "reviewed data") | merged: PR #22, 211ddcb, 2026-10-17, PR Advisor APPROVE (run 2, head 9baa7f2), merged on Mustafa's go in chat | done | none |
| kit-v2-0 (new `.claude/agents/advisor.md`: plan reviews, PR reviews, merges by the PR Advisor routine; CLAUDE.md protocol steps 2 and 6 and the standing permission route plans and PRs through the advisor; NIGHT_SHIFT rule 7: NEEDS PLAN items get a plan reviewed by the advisor agent; labels `ready-to-merge`, `needs-mustafa`, `changes-requested`) | merged: PR #21, 04c086c, 2026-10-17, PR Advisor APPROVE, merged on Mustafa's go in chat | done; no game code | none |
| docs-after-3a (docs only: STATUS + ROADMAP caught up with the 2026-10-17 merges and the Advisor's R19-ORDER / LOS-SIZE / K4-MEASURE verdicts) | merged: PR #20, df1e6d3, 2026-10-17, PR Advisor APPROVE, merged on Mustafa's go in chat | done | none |
| **3a three-teams-core** (teams as data, 12-slot soldier table, team-filtered shots, two-layer damage, per-team objectives, bot targeting with a LOS budget, DM_Tri arena at `?map=tri`, interim end rule; symbols +22 -0 vs main) | merged: [PR #15](https://github.com/Mergincanli/Shamble-Frontlines/pull/15), 5ffe679, 2026-10-17, Advisor APPROVE, merged on Mustafa's go | done; gate PASS on the merged main (symbols +22 -0, smoke 8/8 + 5/5). Mustafa's first Brave check ran the main build by mistake (Tab showed You alone + Hostile 01-06); renders showed the PR #15 build lists You + Ally 01-03 vs Hostile 01-04. Mustafa checked main (three teams, 3a) in Brave on 2026-10-17: "it looks good" (his words, in the implementer session's chat) | none (3b is next in the greybox queue, after N1 and R19) |
| kit-v1-3 (reporting rule: "Reporting to Mustafa" in CLAUDE.md + advisor-report template) | merged: PR #14, 9614ecf, 2026-10-17, Advisor APPROVE | done | none |
| kit-v1-4 (CLAUDE.md "Night shift": approved unattended work goes into the Night queue, issue #16, the day it is approved; anything needing Mustafa never does) | merged: PR #17, bd27eeb, 2026-10-17, Advisor APPROVE | done | none |
| kit-v1-5 (`.claude/NIGHT_SHIFT.md`: the night shift's rules; the three routines read it from main) | merged: PR #18, 2303073, 2026-10-17, Advisor APPROVE | done | none |
| kit-v1-6 (NIGHT_SHIFT.md: an earlier night's cut-off run is picked up; report comment updated after every finished item) | merged: PR #19, cbd79e5, 2026-10-17, Advisor APPROVE | done | none |
| UI theme transplant, step 1 (menu → dossier, pause → clipboard, field manual) | main (baseline) | delivered, battery PASS | Mustafa's Brave check incl. DEV-panel FPS on the menu |
| kit-v1-1 (settings + 19 deny rules) | merged: PR #3, 5858864, 2026-10-14, Advisor APPROVE | done | Mustafa runs `/permissions` locally (Windows check) |
| kit-v1-2 (narrowed the playwright allow rule) | merged: PR #6, ef019e0, 2026-10-14, Advisor APPROVE | done | none |
| bot-poplar-wedge (`shamble-game.js` only: navBuild clearance 0.7 m + half a cell, navPath start from nearest open cell, 3-call warm-up at end of bootGame; symbols +0 -0) | merged: [PR #7](https://github.com/Mergincanli/Shamble-Frontlines/pull/7), 159bec4, 2026-10-14, Advisor APPROVE | done; report `docs/reports/2026-10-14-bot-poplar-wedge.md` | none |
| editor-undo-ground (`shamble-game.js` only: `addBox`, `addPath`, `makeBuilding`, `makeBarracks` and the `buildMap` terminal now call `stylizeOne()`; `clearMap` now clears `trees[]` and `props[]`; symbols +0 -0) | merged: [PR #8](https://github.com/Mergincanli/Shamble-Frontlines/pull/8), b5f4ecf, 2026-10-15, Advisor APPROVE | done: Mustafa's Brave check passed; report `docs/reports/2026-10-14-editor-undo-ground.md` | none |
| team-prep-fixes (B1 bots hit through the SIM hit world; B7 allocation-light LOS and shot checks via `rayStatics` (103 B / 648-750 B per call); SIM tick in headless runs; B2 bar width; B3 enemy armory; B5 scoreboard churn; B6 map names; B-labels are the three-teams plan's, not AUDIT_BUGS_PERF B1-B3; symbols +5 -0) | merged: [PR #11](https://github.com/Mergincanli/Shamble-Frontlines/pull/11), 513f2fc, 2026-10-16, Advisor APPROVE | done: Mustafa's Brave feel check passed; report `docs/reports/2026-10-15-team-prep-fixes.md` | none |
| docs-editor-v2 (docs only: EDITOR_V2.md + map-editor feature inventory with recommended keep / drop / improve marks) | merged: [PR #12](https://github.com/Mergincanli/Shamble-Frontlines/pull/12), fec5e67, 2026-10-16, Advisor APPROVE | done | Mustafa reviews the inventory marks |
| docs-loop-v2 (docs only: docs/LOOP_V2.md added; pointers in GAME_DESIGN §5–6, DESIGN_NOTES (death tax → Recruit, Spy target → harvester building, Momentum largely answered, three-team question partly answered) and START_HERE's doc list; STATUS / ROADMAP queue) | merged: [PR #9](https://github.com/Mergincanli/Shamble-Frontlines/pull/9), 67cf55b, 2026-10-15, Advisor APPROVE | done | none |
| map-dedupe-on-load (`dedupeMapDef` in game + `edFlashHint` in editor, symbols +2; drop exact duplicate trees / props when a saved map loads; status-bar note; no auto-save) | merged: [PR #10](https://github.com/Mergincanli/Shamble-Frontlines/pull/10), 2026-10-16, Advisor APPROVE | done: Mustafa's Brave check passed; report `docs/reports/2026-10-14-map-dedupe-on-load.md` | Mustafa loads and re-saves each old map once |
| Diagnosis (read-only): poplar wedge + editor Ctrl+Z re-rolls ground textures | n/a | done, both plans approved (wedge: bot-poplar-wedge; Ctrl+Z: editor-undo-ground) | none |
| editor-prefab-list | merged: PR #4, c458fe7, 2026-10-14 | done: Mustafa's Brave check of editor.html passed (no problems), Advisor APPROVE | none |
| gate-check-5 (5a smoke, 5b names, 5c layering, 5d script order; CI jobs gate + smoke) | merged: PR #5, 8675367, 2026-10-14 | done: Advisor APPROVE after his own full-gate run (PASS, 26 s) and mutations that failed correctly | none |
| Phase 0 setup | merged (2d30e4a) | PARTIAL | Mustafa's Windows run |

## Targets
- **v2 (VISION_V2 §3, read with MODES-3 (a): the starting sizes are totals; Mustafa, 2026-10-31):** start with two teams
  with 2–8 players in total (1v1 to 4v4) and three teams with 6–18 in total (2v2v2 to 6v6v6); the goal 12v12v12 or
  24v24; the largest mode 24v24, 48 players. One faction per team, squads of 4, bots fill empty slots; design for 48+
  players (CLAUDE.md).
- **Multiplayer (before VISION_V2, 2026-10-24; LOOP_V2 §2 and DESIGN_NOTES still carry it: DOCS-ALIGN):** three teams,
  one per faction, of 12–16 players each in squads of 4 (12v12v12 to 16v16v16, possibly larger): design for 48+
  players. Bots fill empty slots. Milestones: 2–8 players → 16 → full scale, with load tests at
  each step.

## Follow-ups
- **Follow-ups** (each with its source; recorded in the repo by kit-v3-3, A3, PR #38, on Mustafa's addition of
  2026-11-04: "Record them in the repo, in STATUS's follow-ups, alongside A3's plan."):
  - TOY-BOX's small kit change-set: done, with kit-v3-2 (A2 on his list; PR #37, 12d7961, 2026-11-04 23:06 UTC).
  - A3's own items and the points its plan weighs: the day-session ask before kit and gate edits, the `sk-` key
    after `%20`, `%2C` or `%3A`, read-only Actions tools for pr-advisor, `git rev-list`, python in the docs, the
    advisor's Write tool, `node .checkpoints/*`: done, with kit-v3-3 (PR #38, af8393e, 2026-11-05 21:25 UTC; its
    limits: Open bugs and risks, A3's and kit-v3-2's entries).
  - LATER's design session after the netcode proof test, with the twelve LATER questions and the 38 design questions
    on its agenda (the advisor's ruling).
  - The B3 report (`docs/reports/2026-11-04-b3-design-walk.md`), a later report edit: its "Changes to his words"
    (report:56-57) lacks plan rev 1 §3's approved deviations (a capital first letter at LOOP_V2:147 and :169, labels
    before older quoted lines, the splits and repeats; recorded at c5d86cb, plan-rev1-folded.md:88-91), and its body
    runs 83 lines (skill: ~60); its "fixed in the last commit" (report:37) means c5d86cb (PR #36's final review, the
    advisor's APPROVE of a32e4c0, items 1-2).
  - The kit-v3-2 report (`docs/reports/2026-11-04-kit-v3-2.md`), a later report edit: R-1, it runs 69 lines (skill:
    ~60); R-2, its Risks omit "nothing enforces the cap or the scratch rule" and the notes-folder script prompt (Open
    bugs and risks, kit-v3-2 items 1 and 5, carry both) (the advisor's APPROVE of PR #37's final head 342ff49, comment
    6643739289).
  - F-1, for a kit change-set after A3 (cut from A3 by its plan's condition C3): CHECKPOINTS.md §4's trust rule gives
    no way back when a done block's output outside the notes folder (a build under `.gate/`) is lost with a container
    while the notes survive; the fix, about 15 words ("if a later block finds a done block's output missing, note it
    and redo that block") at CHECKPOINTS.md:77-78, :110-111 and :119, with a 3-run dry run of the paste block (the
    same APPROVE of 342ff49).
  - "B3" means three things (his list's item, the SharedArrayBuffer ring, ADR 0006 part 13's CSP row): still open in
    `docs/adr/README.md` row 86 and ADR 0006's OQ-0006-23 (both inside his quoted words) and the merged #35 report,
    for the drafts' next revision (PR #35's final review, the advisor's APPROVE of 76c35bf, item 5; its STATUS and
    ROADMAP part was taken by B3, PR #36).
  - Local smoke needs Playwright's Chromium build 1194 (Playwright 1.56.1); the cloud container has 1243 in
    `/opt/pw-browsers` [verified 2026-11-05: `ls /opt/pw-browsers`]: a candidate for the environment's setup or
    tools/README's "Check a new machine" (PR #36's final review, item 5).
  - Routine proofs to record: what the night shift's first guarded night (2026-11-05 from 00:10) and the PR
    Advisor's next scheduled runs show; its first, 2026-11-04 18:45 UTC, failed on the usage limit (Routines) (the
    docs-pass queue, with PR #35's final review).
  - CHECKPOINTS.md §7's cap line could say a workflow script must still wave its calls in 3s: this container's
    runner allows CPUs - 2 agents at once (2 here), a bigger machine's up to 16 (A2's later-pass item 2; not taken by
    A4, so a later kit pass).
  - E9's untested cases: the "first line names another task" case (notes moved into `old/`) and worktree agents; a
    small dry run in a later kit pass (A2's later-pass item 1; Open bugs and risks, kit-v3-2 item 4).
  - The toy-box tone outside A2's files (`README.md:4`, `docs/GAME_DESIGN.md:14`, `:17`, `:28`, `:50-51`,
    `docs/ROADMAP.md:10`), for a later docs pass against LOOK_V2 (A2's later-pass item 3; KIT-ALIGN, below).
  - From A3's plan readers, for security-auditor to weigh: code.claude.com and docs.anthropic.com are not on the
    WebFetch list; the allow entries `node .gate/*.mjs` and `node .gate/*.js` (`.claude/settings.json:50-51`
    [verified]) also match paths with `..`. Scratchpad worktrees: a routine's kit write is caught only at commit and
    push. Worktrees under `.gate/`: a routine's write, commit and push checks all miss kit files there (follow-up)
    (A3's critic; corrected by security-auditor's SA-3 and the advisor's fix-round ruling). Both closed for routines
    by A3's fix pass 4 (0c481e1, P4-S4): a routine's Write, Edit and redirect checks use the worktree-aware kit-path
    test, and the night commit and push checks also read the path git prints.
  - A3's cuts (its plan's condition C3), for later: the secret check re-testing notes after up to 3 rounds of
    percent-decoding, and environment values on decoded text; URL credentials with an encoded `@` (`\u0040`,
    `%40`); routine reads on other repositories (`get_job_logs` on another repository, `gh … -R/--repo`,
    `search_*`), for security-auditor.
  - From A3's fix-round ruling (`.checkpoints/kit-v3-3/advisor-fix-round.md`), with where each goes ("next kit
    change-set" meant the one after kit-v3-4, A4: that is kit-v3-5, which carries F1 and F4 only, so by his
    KIT-FREEZE-2 the items below that are still open wait for the foundation's gate step):
    - SA-3's code fix (worktrees under `.gate/` seen by their own top when their common dir is the project's): first
      in the next kit change-set, or first in WT-NIGHT (b)'s change-set, whichever comes first; before any night item
      works in a worktree. Done in A3's fix pass 4 (0c481e1): P4-S4's worktree path rule is this fix, for the
      routines' write, commit and push checks.
    - L12, a day fetch whose destination is `refs/remotes/origin/*` or `refs/heads/main` asks: next kit change-set.
    - L14, day git aliases (`git config --get alias.<sub>`: re-check a plain expansion, ask for a `!` one): next kit
      change-set.
    - L9 / SA-9, always fold case (or detect it at run time): after PC-SETUP (4) and E11 P4.
    - L13, the ask in a cloud day session and in bypassPermissions: E11 after merge.
    - L1's runners (sudo, doas, shell -c variants, eval): join plan revision 2 §10's follow-up "the day ask's stated
      limits L1-L4 (scripts, other writers, user settings, patch-applying commands), if E11 or use shows a need"
      (`.checkpoints/kit-v3-3/plan-rev2.md:633-634`; it had no line here until now).
  - From A3's fix-pass-2 ruling (`.checkpoints/kit-v3-3/advisor-fix-pass2.md`; the follow-ups above stand as ruled):
    - A-4: the day bootstrap (settings.json:272, guard.mjs) passes when routine-guard.mjs is missing; consider asking
      instead: next kit change-set (SD-2's fix closes today's path to it).
  - From A3's pass-3 ruling (`.checkpoints/kit-v3-3/advisor-pass3.md`) under Mustafa's scope stop of 2026-11-05 (A-4
    and the earlier follow-ups stand):
    - FU-P3a, the day reader against real bash: a differential corpus of day lines comparing the guard's reading with
      bash's, to find syntax it neither reads exactly nor flags; each form found joins D-ASK's list (an ask, not a
      parse: Mustafa, 2026-11-05). Not before the kit freeze lifts.
  - From A3's pass-4 ruling (`.checkpoints/kit-v3-3/advisor-pass4.md`, section 6), under Mustafa's review budget (below
    Critical or High: a follow-up, not code). Each waits for the kit freeze to lift (A4-FREEZE (a): after the netcode
    proof test is under way); none is built in A3; FU-P3a and the earlier follow-ups stand:
    - **FU-P4a, here-document bodies in D-ASK** (pass 4, P4-S6, Medium, day): D-ASK does not look inside a
      here-document's body. So in a day session, a body with an unquoted delimiter can run a command unseen: a `$(...)`
      or a backtick in the body, or a backslash-newline that moves the end line so that the next lines run as commands.
      Fix: such a body (unquoted delimiter, and the body holds a `\`, a `$` or a backtick) asks too, and an agent's is
      refused; FU-P3a's corpus covers it. After the kit freeze.
    - **FU-P4b, D-ASK's prompt cost** (pass 4, P4-S8 and P4-V4, Low, day): D-ASK asks on every day line that holds a
      comment, a `\` line continuation, `$'…'`, an unusual here-document delimiter or `>&file`, and refuses an agent's,
      even far from kit files (`npm test # run` asks; an agent's `ls docs # list` is refused). This is the cost of
      Mustafa's rule ("any line the guard can't fully read is an ask"). Watch it in day sessions. If it is noisy, a
      later kit change-set may let a comment that holds no quote or backslash pass (it hides nothing from the reader),
      or tell agents in their agent files to write shell lines without these forms. Mustafa's call. After the kit
      freeze.
    - **FU-P4c, the day clone folder after unknown options** (automated review 6, finding 14, Medium, day): when `git
      clone` carries an option the guard does not know, the guard checks the folder git makes from the URL only at the
      second positional word. So two unknown options that take values, or a short bundle with a value letter such as
      `-qb`, can shift the URL further, and a clone into a kit folder named by the URL's last part does not ask. Fix:
      with an unknown option present, check the folder made from every positional word (pass 2's shape), and read a
      short bundle that holds a value letter (b, o, u, c, j) as taking the next word. After the kit freeze.
  - From the verifier's check of A3's fix pass 4 (`.checkpoints/kit-v3-3/verifier-fixpass4.md`, P4F-V1, Low,
    routines), under the same review budget:
    - **FU-P4d, R-PROG's reason drops the hints:** R-PROG answers first for every program that is not a routine
      program (cat, curl, kill, sudo and the rest), so the refusal no longer says what to use instead (the Read, Grep
      and Glob tools; research is a daytime task; TaskStop for the saver), and main's test "a refusal names what to
      use instead" fails on A3's guard for that reason only. Fix: progReason adds routineProgram's hint for a program
      it knows (one table for both). After the kit freeze.
  - From B.1 (b1-plan-view, PR #39; the advisor's plan conditions 8 and 10,
    `.checkpoints/b1-plan-view/advisor-plan-rev1.md`, readable at a3ee6f8), for the decision drafts' next revision
    (B.1 edits no draft):
    - **Scale, "1 unit = 1 metre":** Mustafa's "one consistent scale across the engine", with "1 unit = 1 metre" and
      "textures tile in world space (they repeat per metre and never stretch)" (2026-11-05; EDITOR_V2 §3, Decided
      2026-11-05). No ADR draft or design doc stated the unit, but the drafts already work in metres (ADR 0005:401,
      top speed in metres per second; ADR 0004:261, "a map 2 km across") [verified], so the revision confirms them and
      changes nothing in them. Owners: ADR 0004 and ADR 0005 for the unit; ADR 0003 for textures that tile in world
      space.
    - **Editing while the match runs:** his tool order (2), "you can keep editing while the match runs", meets
      OQ-0005-CD4 (ADR 0005:670-681): under both of its options, map geometry and navigation data apply at the next
      match (R4-PHYS-REC2 keeps the static world for the match), and EDITOR_V2 §2 rule 4 merges static objects only
      when you press Test [verified]. Two readings: edits go into the document and apply at the next Test or match (no
      conflict), or they reach the running match; then CD4's "an engine feature no §6 item names" no longer holds,
      since EDITOR_V2 counts as a need source (`docs/adr/README.md:295-299`). For whoever decides OQ-0005-CD4 (Mustafa
      and the Advisor); the register's row notes it (`docs/adr/README.md:120`).
  - From B.1's review round (game-designer, `.checkpoints/b1-plan-view/game-designer.md`, readable at a3ee6f8), for
    the editor's
    architecture plan (EDITOR_V2 §6 step 2), which puts each question to Mustafa; none changes his words:
    - **B1-MODES:** do the editor's symmetry tool and fairness panel also serve two-team maps? VISION_V2 §3 has
      two-team and three-team battles both tested and maps designed per mode, and §6.1 makes modes data; both tools
      work only under 120° today. The game-designer recommends: yes, the symmetry order as data (2 teams at 180°, 3
      at 120°). His call.
    - **B1-LEAP:** what are "leapers" in the travel-time ruler? No doc defines them (the nearest is UNITS.md:49, the
      Soviet walker's leap, `[IDEA]`); the ruler needs their movement data. Open, for the unit list to answer; the
      ruler is lower priority.
    - **The objectives list:** EDITOR_V2 §3's "Objectives as data" (the Advisor's `[DEFAULT]`) leaves out refineries
      (LOOP_V2:32), power plants (:42), the HQ (:96, :130) and supply points (:259), which his heatmaps ("each
      objective") and fairness panel ("every objective") read. For the Advisor's next pass on that line.
    - **The fairness panel's pass rule:** at any moment the travel times to a pair uplink site are unequal by design
      (LOOP_V2 §4, §9) and even out only over a full rotation; cache spawn points need rotated sets of three. What the
      panel counts as fair (per site, per active pair, per cycle) is a design rule for Mustafa with the game-designer.
    - **Heatmaps and sightlines meet open points:** heatmaps meet LOOP_V2 §11's three runs per build (:188), the bots'
      role mix (§10, :170), VISION_V2 §6.6's telemetry and R6-TEL-OQ2 (`docs/adr/README.md:203`); sightline checks
      meet R4-LOS-OQ1 and OQ2 (README:193-194) and LOOP_V2 §9's aiming at height (`[OPEN]`, :162).
    - **Quality bars:** EDITOR_V2 §4 has no bar for "instantly" (his tool order (2)) or for smooth editing while a
      match runs; for the architecture plan's test plan.
  - From B.2 (b2-community-servers, PR #42; the advisor's plan conditions 8 and 9,
    `.checkpoints/b2-community-servers/advisor-plan-rev1.md`, readable at e0b0687):
    - **OQ-0006-21's pointer**, for the decision drafts' next revision (B.2 edits no ADR): ADR 0006's OQ-0006-21
      entry may point at VISION_V2 §5, his community-servers note (B.2, PR #42); the register's row already does
      (`docs/adr/README.md:161`). The note's `[DECIDED]` trust rule adds no field or check now: part 12's "No field
      marks a match as official or community ... adding one is design for later" stands (infra-engineer, Low). His
      private-match XP rule (2026-11-06, vfx-xp-notes) adds no field now either: the note is `[LATER]` (decision map
      J3, `docs/adr/README.md:409`), and part 12's match record already has a "private flag" and "humans and bots per
      team" (ADR 0006:482-483; :474-475 before kit-v3-4's notes in parts 9 and 10).
    - **VISION_V2 line ranges in ADR 0005**, for the drafts' next revision (the verifier's Medium finding; the widened
      E4 found a fifth): B.2's added lines moved them, vfx-xp-notes' XP block moved §6 again, and kit-v3-4's XP-TERM
      pointer (three lines added at VISION_V2:72; the block is now :71-78) moves it once more. At kit-v3-4's head:
      ADR 0005:27 "§6 (lines 62–81)" is 83–102; :48 "§2 (lines 25–40)" is 27–43; :49 "§3 (lines 42–49)" is 45–52;
      :50 "§4 (lines 51–55)" is 54–58; :2550 "§2, §3, §4 and §6 (lines 25–81)" is 27–102. No ADR is edited in B.2,
      vfx-xp-notes or kit-v3-4 (kit-v3-4 adds notes to ADR 0006 only).
    - **Not decided by the note** (game-designer and infra-engineer, Low): whether rented "official private
      servers" count as official for progression, beside R2-ROOM-OQ7 (private matches and Service points) and
      R6-TEL-OQ2 (bot and private matches in balance rates). His trust rule says only that progression counts on
      official servers. His XP rule of 2026-11-06 for private (rented) matches (VISION_V2:71-78: 25 % of normal XP
      with bots in the match, 100 % humans only) bears on it and does not answer it. Which progression his "XP"
      means is answered: "all account progression (the account level and Service points), never the in-match weapon
      XP" (Mustafa, 2026-11-07, XP-TERM (a); "From vfx-xp-notes", below). Still open: whether rented official private
      servers count as official, and whether unrented private matches count; the design session for community
      servers decides the rest.
    - **"The war around the matches"** (LOOP_V2:147, `[OPEN]`, Mustafa, 2026-11-04) is not among VISION_V2 §7's
      open design questions (game-designer, Low): for a later VISION_V2 pass, not B.2's scope.
    - **A stale research citation, left unedited:** `docs/research/r1-simulation-core.md:3301` and `:3535` cite
      `docs/VISION_V2.md:54` for the 100 ms line, already stale before B.2 (that line was :55 at ee736ec and is :58
      after B.2, at d5e2c62 and at kit-v3-4's head). The research file is a dated record, so it stays as written.
  - From vfx-xp-notes (PR #43; the advisor's plan conditions 2, 3, 5 and 7, and the review round's findings,
    `.checkpoints/vfx-xp-notes/advisor-plan-rev1.md`, readable at ced21da):
    - **XP-TERM, answered:** his private-match rule says "XP", and "XP" already names other things in the docs: the
      in-match weapon XP of LOOP_V2:242 (`[DECIDED]`, LOOP_V2 §13: it levels a unit's or weapon's track, and tracks
      reset each match, LOOP_V2:237-238); the register's :69 (OQ-0005-AB3), which quotes it; DESIGN_NOTES:23's
      "account XP"; and the legacy career's XP (LEGACY_INVENTORY:302, GE15; LOOP_V2:140 calls the account level "the
      existing career rank"); LOOP_V2:235 names the same weapon XP. vfx-xp-notes mapped his "XP" to none of them and
      asked him (its report's "Decisions needed", XP-TERM). Mustafa answered on 2026-11-07 (his message that gave
      "merge #43"), word for word: "XP-TERM: (a), all account progression (the account level and Service points),
      never the in-match weapon XP: amps reset every match, so there's nothing to farm, and cutting them would only
      make practice with bots feel worse." Recorded by kit-v3-4 (PR #44): a pointer beside his rule (VISION_V2:72-75)
      and the register's R2-ROOM-OQ7 row (`docs/adr/README.md:184`); it settles the "XP" part of B.2's
      rented-servers follow-up above. Facts kept for the community-servers design session (nothing decided here):
      Service points come only from a share of the team's uplink tech, never from kills (LOOP_V2:45-52, :132-133), so
      with the weapon XP left out, the Advisor's open suggestion (1), XP weighed per action, fits only an account-level
      XP that v2 has not defined (LEGACY_INVENTORY:302; game-designer, Medium, on vfx-xp-notes); the rule covers
      rented private matches only, not unrented ones (VISION_V2 §3); one bot in an otherwise human match (a bot taking
      a leaver's slot, R2-ROOM-OQ4, OQ5) sets everyone to 25 %.
    - **LOOK, answered:** his "Comic flavour: ink-outlined smoke, impact frames, halftone flashes, onomatopoeia"
      (EDITOR_V2 §7) against LOOK_V2:63's `[DEFAULT]` "Comic impact touches for big moments," "used sparingly"
      (LOOK_V2 §7). The lists differ too: only halftone flashes are in both, and §7's ink "line boil" is not in his.
      One reading finds no conflict: his line says what the tool can make, §7 how often to use it. It also bears on
      OQ-0003-15 (the legacy look touches, comic words first; README:104, which now points at it). vfx-xp-notes asked
      him (its report's "Decisions needed", LOOK). Mustafa answered on 2026-11-07 (his message that gave "merge
      #43"), word for word: "LOOK: (a)." His word is "(a)"; the option's text is the report's: "no, §7 stands until
      his VFX style guide and the look proof test". So LOOK_V2 §7 stands, and no LOOK_V2 line is edited. His "A VFX
      style guide comes first, from a separate reference study": a follow-up only, not queued; its "ink-outlined
      smoke" bears on OQ-0003-7, the particle technique, not yet researched (art-director, Low).
    - **The need rule:** under decision map J3 (`docs/adr/README.md:409`, "no feature for `[LATER]` or `[OPEN]`
      items"), the `[LATER]` EDITOR_V2 §7 gives the drafts no engine feature now; its principles are inputs for the
      tool's design session. VISION_V2 §6.3's "effect" means gameplay effects (the ability and effect system), not
      visual effects. Today's visual effects already trace to LOOP_V2 §7, LOOK_V2 §3/§7 and §6.9 (ADR 0003 part 5.k).
    - **For the VFX tool's design session** (game-designer and art-director, Low; nothing decided): one "whole impact
      moment" asset must split into a part at the shot and a part at the confirmed hit (LOOP_V2:119), or its hit-stop
      comes up to about 100 ms late; the feel kit (ADR 0005 CD-14) and his effect assets both hold muzzle flash, impact
      and sound, so which references which is open; camera shake, impact frames and halftone flashes add to the
      reduced-motion question (OQ-0003-18); a Low-quality effect must not show what other tiers hide (the Shinobi's
      stealth, UNITS:44); "role colour slots" recolour per faction, which cannot tell two teams of one faction apart if
      a mode allows that (VISION_V2:85, :82 before kit-v3-4; ADR 0005:381), and ADR 0003 5.h (:247) puts faction
      colours and visual effects in separate material classes.
    - **Stale citations, older than this pass** (left unedited): `docs/research/r1-simulation-core.md:3631` cites
      `docs/EDITOR_V2.md:39` (the "Validation panel" line), which is :48 at this head (the verifier, Low); VISION_V2:66
      cites DESIGN_NOTES for "team colours never change", which no version of DESIGN_NOTES says (art-director, Low).
  - From kit-v3-4 (PR #44; the advisor's fix-pass ruling, `.checkpoints/kit-v3-4/advisor-fix-pass.md`, readable at
    c3cf647, and for F5 its ruling on H1, `.checkpoints/kit-v3-4/advisor-h1.md`, readable at 20b4be5):
    - **F1, the guard's rename gap** (risk tier 2; security-auditor SA-1, Critical, and SA-8, Low; the advisor's
      agent-memory Critical of 2026-11-09): carried out by kit-v3-5 (PR #46, in flight; 9bacf47) on his GUARD-RENAME
      (c): the PR Advisor merges only from an allow-list, read on both names of a renamed file, below 3000 files and
      with whole lists; `CLAUDE.local.md` and `AGENTS.md` are protected anywhere; SA-5 rides it (his SA5-RIDE (a));
      advisor.md's tier-2 bracket now describes the allow-list. Its known limits: Open bugs and risks, kit-v3-5.
    - **F2, CI and a pure revert's dropped symbols** (risk tier 2, its own change-set; infra-engineer IE-1, High,
      fixed in advisor.md's text by kit-v3-4): CI passes no `--allow-drop`, so CI's `gate` fails on a revert of a
      merge that added game symbols. A way for CI to accept a pure revert's drops, with security-auditor SA-3's note:
      only for a first-parent merge of main named in "This reverts commit <sha>", and exactly that merge's added
      symbols. Until then advisor.md's "Revert" sends such a revert to Mustafa. Waits for the foundation's gate step
      (KIT-FREEZE-2, 2026-11-10).
    - **F3, register line drift from #45** (docs-keeper; the verifier's L1, Low, second half): fixed by kit-v3-5's docs
      pass. #45 added a register row at README:88, so the nine STATUS citations of `docs/adr/README.md` below it now
      name the line one down (:120, :161, :193-194, :203, :295-299, :409), each checked against the register at 9bacf47.
    - **F4, CLAUDE.md's short risk-tier table** (the verifier's L3, Low, and the advisor's final-head Low on #44: name
      agent memory too): carried out by kit-v3-5 (9bacf47). Row 0 reads "docs that add or change no word or decision of
      Mustafa's" and leaves out a `CLAUDE.md`, `CLAUDE.local.md`, `AGENTS.md`, `.claude/` folder, agent memory or other
      gitignored path, and a file renamed or moved from outside tier 0 (CLAUDE.md:98); the line under the table reads
      "Steps 4 and 5 apply at every tier; decision docs always need his go." (CLAUDE.md:103).
    - **F5, the third "main breaks" case in the revert check** (left out of kit-v3-5; the verifier's L3, Low):
      advisor.md's check of a revert PR accepts a red `gate` or `smoke` run or Mustafa's own words, so "a merged kit
      change stops sessions working" qualifies only through his report (the strict side). To decide then: accept it
      when the advisor reproduces the refusal or error on main itself (a loosening: needs his words), or drop the
      case from "Main breaks" and CLAUDE.md's Rollback line, leaving it to his report. Waits for the foundation's gate
      step (KIT-FREEZE-2, 2026-11-10).
  - From kit-v3-5 (PR #46; the advisor's fix-pass ruling, `.checkpoints/kit-v3-5/advisor-fix-pass.md`, readable at
    501c446; the report `docs/reports/2026-11-10-kit-v3-5.md`):
    - **PX-1, check runs matched by name only** (security-auditor; outside kit-v3-5's diff and untested; Critical on
      main before kit-v3-5 if confirmed, Medium after it by the advisor, since the allow-list leaves the PR Advisor
      nothing a faked green would hide; GitHub's required checks trust the same names): the merge check reads the
      `gate` and `smoke` check runs by name, not by app or workflow, so a workflow on another branch might post green
      runs on a head. The fix, a proof test first and then binding both to the gate workflow's run on that head SHA
      and to the GitHub Actions app, is its own kit change-set. His PX1-ROUTE (2026-11-10, 15:00 UTC; saved word for
      word in `.checkpoints/kit-v3-5/mustafa-2026-11-10-third.md`, readable at 501c446): "(a). A STATUS follow-up, as
      its own change-set after the foundation's gate step."
    - **Two test gaps on the fix pass's own Low checks** (the verifier's re-check, L1 and L2, Low): the nameless-line
      row has the empty name only on the last line (a line such as `\tdocs/old.md` in the middle of a list is not
      pinned), and the fake git answers any `ls-files --others` alike (the untracked-shrinkwrap check's arguments and
      its git failure are not pinned). Rows to add; they ride PX-1's change-set, which edits the guard and its tests.
  - The kit-v3-1 Windows test follow-up stays in Open bugs and risks (kit-v3-1 item 1; its own change-set).
- **Open, KIT-ALIGN** (rules files the 2026-10-31 answers left stale; a kit change-set after PR #30 aligns them, with
  Mustafa's go): `CLAUDE.md:7-8` and `.claude/agents/netcode-architect.md:17-19` (the per-team reading of VISION_V2 §3);
  `CLAUDE.md:208` [verified on A2's head; was cited as :192], `.claude/agents/security-auditor.md:14` and
  `.claude/agents/infra-engineer.md:29-30` (the security baseline as part of R6);
  `.claude/agents/infra-engineer.md:23-26` and `.claude/agents/security-auditor.md:46` (preview privacy, costs at the
  starting scale); optional: `.claude/agents/netcode-architect.md:37` and `.claude/agents/security-auditor.md:30`
  (server-only spread as a rule). The look lines added 2026-11-01 (the four "look is not decided" lines and the two
  "toy-box" lines, against LOOK_V2 §3: "The audience is adults. It must not look like a children's game.") are taken by
  kit-v3-2 (A2, PR #37, merged 12d7961): TOY-BOX and LOOK-LINES (B), Decisions pending. Left for a later docs pass:
  the toy-box tone in `README.md:4`, `docs/GAME_DESIGN.md:14` ("toy-like"), `:17` and `:51` ("toy-box"), `:28`
  ("upbeat") and `:50` ("joyful"), and `docs/ROADMAP.md:10` ("bright toybox palette") [verified] (outside A2's files).
- **Open, DOCS-ALIGN** (the Advisor's documents; the session does not edit them): `docs/V2_PHASE0_BRIEF.md:113`
  ("running in parallel", against kit-v3-2's cap of 3 agents at a time); `docs/V2_PHASE0_BRIEF.md` §1 ("Then:
  playable slices, as in VISION_V2"; VISION_V2 has no slice list) and line 126 (R6's row still holds the security
  baseline); `docs/LOOP_V2.md:90-91` (tracers and the Feel Lab test); `docs/LOOP_V2.md:19-20` and
  `docs/DESIGN_NOTES.md:34-35` (team sizes written before VISION_V2: "12v12v12 to 16v16v16, possibly more" in LOOP_V2, "possibly larger" in DESIGN_NOTES);
  `docs/EDITOR_V2.md:94` (§5: "It still runs by double-clicking editor.html") and `docs/EDITOR_V2.md:95` (§5: "if
  ADR-002 moves the project to modules"): ADR 0002 now decides the structure (added 2026-11-01).

## Decisions pending: the rest
_The advisor agent's rulings (not Mustafa's), the five carry-forwards, and the old ADR and design lines, moved from
STATUS's "Decisions pending"; his decisions and the queue of his words are in `docs/DECISIONS.md`._
- **Taken by the advisor for A3** (inside his words; he can overrule any at merge; plan revision 2 §3.2, from
  `advisor-plan-rev1.md` and `advisor-e10.md`): AF-MECH (b), the day guard answers "ask", the settings ask list stays
  empty · AF-AGENT (b), an agent's kit edit is refused, keyed on `agent_id` only · AF-COVER, a trimmed (b): file tools
  and the named shell writers (limits 1-3) · WT-DAY (ii), trimmed to what switches the main folder (limit 4) ·
  WT-UPDATE (a), the fast-forward forms to `origin/main` stay free · SECRET-DECODE (A) only, one widened escape
  fragment (follow-ups F2-F3) · F1-FIT (b), F-1 recorded in Follow-ups, no CHECKPOINTS.md edit · READ-SCOPE (b), the
  owner/repo check on the new Actions reads only (limit 10) · NODE-ALLOW (a), no allow entry for `node .checkpoints/*`
  · QUEUE-HOME (a), "Queued, in Mustafa's words" under Follow-ups · keep the advisor's `memory: local` (E10-2).
- **Taken by the advisor agent 2026-10-31, about 18:35 (its plan review of v2-decision-drafts, APPROVE with 19 conditions;
  verbatim in `docs/reports/2026-11-01-v2-decision-drafts-appendix.md` §3):**
  - ASSET-RESEARCH (a): ADR 0005's asset part takes LOOK_V2 §8's flow as given and proposes only what R1-DATA, R3-CHAR
    and R3-BUDGET support; the rest are "not researched" open questions, grouped as one "asset-pipeline research" unit
    (graphics-engineer, its own daytime session) that SPIKE_PLAN places before the asset spike.
  - TICK-60-HEADROOM: TICK-30's 25 % is the 60 Hz margin, measured against the server tick budget at 60 Hz (8.33 ms): a
    60 Hz tick needs a p99 server tick time of at most 6.25 ms in the netcode proof test (one vCPU of the reference
    server, the stress scene at N = 48 with a snapshot-receiving client on every soldier, R2-BUD-REC6's 30-minute soak,
    re-flood ticks included) plus every 60 Hz mark the research sets; otherwise 30 Hz. Confirmed by Mustafa at the stop
    (2026-11-03, OQ-0001-1), with his addition and TICK-60-SETUP's early-signal reading (above).
  - GLOSSARY-8: yes, an append-only §8 with three entries (the foundation, the netcode proof test, the 60 Hz margin) and
    one note.
  - CHECKPOINT-AT-STOP: keep; the report folds in the checkpoint, which is deleted before the PR is marked ready.
- **The five carry-forwards into the decision drafts** (final state 2026-11-02; each line's row in
  `docs/adr/README.md`'s loose-ends register):
  1. Verify every ADR claim and number, R3–R6's included. Done: every draft and the synthesis verified against the
     research, 0006's FAIL fixed, the fixes re-checked; claims on unreadable sources stay "unverified" (row CF-1).
  2. R2-BUD-REC6's reason holds at N = 54, not 48. Done: ADR 0001 parts 2 and 3 (restated at 30 Hz, "(at N = 54,
     MODES-3 (b))", the 60 Hz margin per GLOSSARY §8); its reading, OQ-0001-1, is confirmed at the stop (row CF-2).
  3. Use 48 where a mark gives 48 and 54; "until … answers" takes the answer. Done: every draft's Appendix A; checked in
     the consistency pass (row CF-3).
  4. A certificate per hostname over the wildcard sentences. Done: ADR 0006 part 7 lists the four as superseded;
     issuance is OQ-0006-5 (row CF-4).
  5. Define "the netcode proof test" and "the foundation". Done: GLOSSARY §8 (appended); ADR 0001 owns the proof test,
     ADR 0002's migration plan is the foundation (row CF-5).
- ADR-001..003 below: the v2 drafts `docs/adr/0001`–`0006` replace them (CLAUDE.md, "Decisions pending"). Since
  2026-11-01 each is covered by a Proposed draft (`docs/adr/README.md`'s register: ADR-001 → 0001, 0004, 0005, 0006;
  ADR-002 → 0002; ADR-003 → 0003); they count as replaced once Mustafa accepts the drafts.
- **ADR-001 Multiplayer architecture** (authority, tick, transport, hosting, offline play): not started. Inputs logged so far: the 11 wall-clock reads (performance.now / Date.now) that make same-seed matches differ; no SRI/CSP on the three.js CDN tag; server-side map-def validation (SEC-MDL-2: `__proto__` key through `envMerge` / `rulesMerge`). Added by 3a: one hit world for every shooter is a multiplayer invariant (K1: bots still use `_botFireWorld`, a view of SIM without ramp slots, until ramps become slopes in SIM); `update()` must become phased (today's real order is in TECHNICAL §4a, K4); the server must repeat the team allow-list (`teamOf`); boot-time `Math.random` draws per slot (`bootGame` soldier loop, `enemyFire` spread, strafe and respawn type); three.js UUIDs draw from the shared `Math.random`; the bot-shot FX cull makes the shared `Math.random` sequence camera-dependent (cosmetic effects need their own RNG stream); `applyDamage` runs `addTech`, whose HUD writes and era toasts should become an event; humans take over bot slots; the LOS budget is sized for 12 soldiers (480 checks/s caps demand from about 24 soldiers in combat; fair since the review fix); map data needs a full schema check server-side (`placeObjectives` checks only the objective keys; `version:1` is written but never read) [verified in code: no `.version` read in shamble-*.js]
- **ADR-002 Code structure & build** (modules, bundler, TypeScript, double-click build): under discussion with Mustafa;
  evidence first via the build spike
- **ADR-003 Three.js version** (stay on r128 vs upgrade): deferred until after ADR-002
- **Design: progression:** LOOP_V2 §8 now settles the shape: team tech tiers in the match; Service points plus
  WARDOGS-style three-gate unlocks on the account; the unlock catalogue is [LATER]; since 2026-11-04 the account gives
  class variants and cosmetics only (B3-Q2 (a)) [verified: LOOP_V2 §8]. Affects what the server must persist
- **Three-teams plan: APPROVED** with D1-D16 plus three tweaks: D6 (the 200 HP Empire elite leaves the bot roster, done in 3a [verified: `spawnEnemyMesh(_fac,false)`, `elite:false`]; its kit code
  stays for a possible Tier 3 or hero unit), D11 (your own HQ down = you lose; the win and loss rules stay placeholders until
  win-and-scoring) and DM_Tri (`?map=tri` follows LOOP_V2 §9's 6 m gap rule and gets the bot reachability check)
- **LOOP_V2 [OPEN] items:** the infiltrators' cost (tech or money) and their rules (what breaks the Spy's disguise,
  what reveals the Shinobi's stealth, what its sabotage does, hack time, steal amount, warning) [verified: LOOP_V2 §5];
  what spawn protection covers and whether an ability press counts as firing (LOOP_V2 §2, OQ-0005-AB14); the war
  around the matches, in its own design session (LOOP_V2 §8);
  aiming at height with ledges (decide in octopon-3-greybox); Octopon-3 map size (start near Cabana, adjust with the scale test)

## Routines
_Read with the routines API (list_triggers) on 2026-11-04 at 14:03 UTC (the first read after the un-pause) by the
docs-after-kit-v3-1 session (session_0173vVo7FcYUpbWgRW8oQyHU), and again at 14:36:38 and 14:47:25 UTC, the last right
before PR #35's final commit: unchanged. Both routines run on the role environments (`SHAMBLE_ROLE=night-shift`,
`SHAMBLE_ROLE=pr-advisor`) under the routine guard (kit-v3-1); NS-2 (c), a fresh session on every run, since
2026-10-31. Each routine: one repository, not in a project (A3's E10-5, `.checkpoints/kit-v3-3/advisor-e10.md`: a
cloud thread in a project, or a cloud session with several repositories, loads no repository hooks, so no guard;
CLAUDE.md:140; TEST-RUN B2's refusals were the guard's). The next proof of the routine rules: the PR Advisor's first
scheduled run that gets through (its 2026-11-04 18:45 UTC run failed on the usage limit; next 2026-11-05 06:45 UTC)
and the night shift's first guarded night (2026-11-05 from 00:10 Amsterdam); neither has recorded a run yet._
- **Night shift: "Shamble Frontline Night shift"** (trig_01QpV6y6UaG1Na2q3jumRDxh): `CRON_TZ=Europe/Amsterdam 10 0-7
  * * *` (hourly, 00:10 to 07:10 Amsterdam; window to 07:30), enabled, Opus 5.5, the one-line prompt; one
  repository, not in a project (E10-5). Its first night under the guard starts 2026-11-05 00:10 Amsterdam. Under the
  guard it never reads the web; research runs in the daytime only (NIGHT-RESEARCH (a)).
- **PR Advisor** (trig_01YPyWAsQmsaQoydhMGG3CkJ): one repository, not in a project (E10-5); no setup script; it has
  a schedule trigger only, `CRON_TZ=Europe/Amsterdam 45 7,13,19 * * *` (07:45, 13:45, 19:45 Amsterdam), can also be
  started by hand ("Run now", as B2 was) or through the API, and is never started by pull-request events
  (PR-HOME-REPLACE (a), Mustafa, 2026-11-04: its GitHub pull-request trigger and its connectors are removed).
  `enabled: false`: Mustafa paused it on 2026-11-09 at 18:22 UTC until kit-v3-5 merges (his GUARD-RENAME (c)), and it
  did not run at 18:45 UTC; re-enabling it is his (kit-v3-5's checkpoint; before that `enabled: true` from his un-pause,
  updated 2026-11-04 14:01:31 UTC). Model `claude-opus-5-5`; its trigger has no MCP connections (its GitHub tools come
  from the environment). Its first
  scheduled run, 2026-11-04 18:45:37 UTC (session cse_0173vVo8Ju2Jb2eN5ZDzJdtK), failed:
  `ROUTINE_RUN_FAILURE_REASON_USAGE_LIMIT_REACHED`, the account's usage limit, which routines share with day sessions
  (it reset at 18:50 UTC). Next run: 2026-11-05 06:45 UTC (07:45 Amsterdam), so the scheduled-start proof is still
  open (get_trigger, about 18:55 UTC). Its stored
  prompt, verbatim: "You are the PR Advisor for SHAMBLE FRONTLINES. Read .claude/agents/advisor.md from the default
  branch and follow its PR review procedure exactly, together with CLAUDE.md, for every open pull request in this
  repository, oldest first. Stay on the default branch: read each pull request only through git, naming its head SHA,
  and the GitHub tools."
- **TEST-RUN, passed** (Mustafa accepted parts A and B, 2026-11-04):
  - Part A: the "Guard test" routine on the night-shift environment: `git status` and the Read tool ran; `echo`, `cat`,
    `curl` and `git push --dry-run origin HEAD:main` were refused, each with the right message.
  - B1: 13:20 UTC, second try (the first failed on a typo, `SHAMBLE_ROLE=pr-adviso`, an unknown role, refused as
    designed); steps 1–7 ran with HEAD = origin/main = e2b5799, steps 8–14 were refused with exactly the expected
    messages, and the PR Advisor can read CI's gate job log (session_0173vVo6XGr8xapun6WUNTpM).
  - B2: "Run now", 13:26:48–13:35:54 UTC (session_0173vVo7uY6BfxLBnYjEBJLV; 57 turns, 3.36 USD, effort max), checked
    row by row from its transcript: it started on main (HEAD e2b5799 on the harness's own container-only branch
    `claude/quirky-bohr-k7scly`, clean, then `git switch --detach origin/main`); every guard refusal names
    `routine (pr-advisor)`; it read #34 only through git and the GitHub tools and ran no programs; CI from the 4 checks
    and the gate job logs; review comment 6632017016 naming the full head, label `ready-to-merge`, head and checks
    re-checked at 13:34:46, merged at 13:35:00 as 868e165 with `expectedHeadSha` a432692…; no permission prompt; all 7
    `permission_denials` are guard refusals (`echo`, `ls`, 4 × `actions_get`, `git rev-list`).
- **"Guard test B1"** (trig_01XmLByUmTibCu5XXQR7iN26): deleted by Mustafa (gone from the 14:03, 14:36 and 14:47
  listings).
- **Old "Night shift"** (trig_019LtRqmAdzddyk6K3e3kDJz, one persistent session): stays disabled until it is deleted;
  Mustafa deletes it after one good night of the new routine (deleting it also deletes its session).
- **The old PR Advisor session from the PR #26 event** (stuck since 2026-10-25): stopped at step 0 with the role
  unset, changed nothing; Mustafa archived it.
- **Effort (EFFORT-SOURCE, answered 2026-11-04):** a routine runs at the effort set on the routine, not at
  `.claude/settings.json`'s `effortLevel: "high"`: the night shift's last run (06:11 UTC,
  session_0173vVo8bhZGeK4bVFXQQEtM) records `session_context.effort_level: "xhigh"` (its page's "Extra"), and B2
  records `effort_level: "max"` (Mustafa's choice for the PR Advisor). A cloud day session's launch records
  (session_0173vVo7FcYUpbWgRW8oQyHU: `/tmp/env-manager.log` "Added gateway arg to command"; `/tmp/claude-command` and
  `/tmp/claude-code.log` read `--effort max`; `CLAUDE_EFFORT=max`) show the harness passing the effort as `--effort`,
  the likely route for routines too. The next record to read: tonight's first night-shift run (2026-11-05 00:10).
- **Known noise (PROBE-FILE (c), Mustafa, 2026-11-04):** the Stop hook is Claude Code on the web's own
  (`~/.claude/stop-hook-git-check.sh`, registered by the harness; the repository defines none). It asks a session to
  commit and push uncommitted changes, untracked files that are not ignored, and unpushed or unsigned commits. B1's
  probe (`.checkpoints/part-b/probe.md`) set it off; the guard refused the push, and the hook asks only once per stop.
  B2 left no files and did not set it off.

## Open bugs and risks
- **CI, the smoke job's Chromium step** (for infra-engineer; PR #42's checks, read with the GitHub API by the main
  session on 2026-11-07): on #42's head e223ecc, the pull_request run 42313665674 stalled in the step "Install
  Chromium's system libraries (browser restored from the cache)", which comes before "Smoke (check 5a)": attempt 1
  was cancelled after 6 min 34 s in that step, and attempt 2 passed after 6 min 49 s in it; the push run 42313658977
  did the same step in 11 s. The cause is not investigated; vfx-xp-notes changes nothing in `.github/`.
- **kit-v3-5** (PR #46, the PR Advisor's allow-list, 9bacf47; known limits of the new merge check, from plan revision 3
  §2 as the advisor's E4 worded them; the first three not built, by his words of 2026-11-10):
  - Windows short names (`CLAUDE~1.MD`): not handled; the plain-name rule happens to refuse `~`.
  - NTFS streams (`::$DATA`): not handled; the plain-name rule happens to refuse `:` and `$`.
  - Trailing dots and spaces: not handled; spaces are refused, but a trailing dot passes (`docs/CLAUDE.md.`); whether
    Windows git checks such a name out, and loads it as `docs/CLAUDE.md`, was not verified.
  - Gitignored names inside the places are not refused: today `.gitignore`'s `dist/` and `*.log` match at any depth
    (`tools/dist/x.js`); a `.gitignore` change needs a look at the list.
  - Case-only twins inside the places (`docs/status.md` beside `docs/STATUS.md`): Windows and macOS overwrite one with
    the other.
  - Symlinks: the file list carries no file mode, so a symlink at an allowed path merges.
  - The foundation's restructure: a foundation PR that moves a place or a gate file updates `MERGE_PLACES` or
    `GATE_FILES` in the same PR (a keep-the-kit-true edit, his second message of 2026-11-10, item 2, asking him); a new
    place (`legacy/`, the v2 folders) widens the PR Advisor's merges and is his decision, after the gate step or by his
    explicit word.
  - More than 100 files: such a PR never merges through the PR Advisor here, since the proxy refuses the next-page links
    gh follows (PR #40, 2026-11-10), and the merge check fails closed.
  - The write-side limits (deferred under KIT-FREEZE-2; severities from kit-v3-5's security review): a day subagent or
    a routine may write a nested `<dir>/.claude/` file (W1, Medium) or a gitignored path other than the protected names
    (W2, Medium) without an ask; `kitDir` checks a folder for `CLAUDE.md` only (W3, Low; routine-guard.mjs:147).
  - Windows reserved names inside the places (`CON`, `NUL`, `AUX`, `COM1`, such as `docs/aux.md`): not handled; Git for
    Windows refuses to check them out (`core.protectNTFS`), so his pull on Windows would fail (security-auditor SA5-3).
  - `docs/ENGINE_OVERVIEW.md` is a decision draft (CLAUDE.md:20-22) but not on the guard's decision-doc list (his
    SA5-RIDE (a): "the fixed list in your plan and nothing more"); the advisor still treats it as one that needs his go.
- **kit-v3-2** (PR #37, merged 12d7961; from its plan, the advisor's plan review and the report's Risks): 1. the agent
  cap and the scratch rule are written rules, enforced by nothing (a size or binary check in the saver would be code: a
  possible later item); 2. the rules hold only as far as agents follow their prompts: E9 ran toys, 1 to 3 times per
  case on the final text, and real long tasks may behave differently; 3. a workflow holding the cap is shown only in
  part: E9's final text ran in a workflow in waves of 3 and 2, but this container's runner allows only 2 agents at once
  (4 CPUs), so no wave of 3 ran together; 4. not tested: the "first line names another task" case, and worktree agents;
  5. scripts in the notes folder ask before they run in day sessions, and routines refuse them: an allow-list entry for
  `node .checkpoints/*` was weighed by A3 and decided against (NODE-ALLOW (a), Decisions pending).
- **kit-v3-1** (PR #30): 1. The Q3 follow-up (advisor condition C6, its own change-set): `routine-guard.test.mjs` fails
  9 of 192 on Windows (pre-existing: the credential checks match forward-slash paths only; 192/192 on Linux, where the
  routines run), and CI does not run the hook tests (`.github/workflows/gate.yml` runs only `tools/gate.mjs` and
  smoke); the fix: separator-agnostic path handling in `routine-guard.mjs`, and the hook tests in CI. 2. By GUARD-FILE
  (a), a day session whose `guard.mjs` is missing or can't load runs with no guard checks at all (git option forms and
  the merge check included), so "merges fail closed" holds while the guard loads. (Both stand after the merge, PR #30,
  2026-11-04.)
- **kit-v3-3** (A3, PR #38, merged 2026-11-05 as af8393e; built, WT-NIGHT (a) included (step 9, ed3ea8c); the fix
  round in 2acfbbc; fix pass 2 (c8cbb1a); fix pass 3 (9589bfb); fix pass 4 (0c481e1); KIT-FOLLOWUPS (a), Decisions
  pending; security-auditor reviewed it). What it closes, now merged (In flight has the steps and commits; the line
  numbers below are at fix pass 4, 0c481e1, rechecked in its working tree, and the guard and saver files are the same
  on main af8393e [verified: `git diff 0c481e1 af8393e -- .claude/hooks/ .claude/scripts/` is empty]; A3's notes,
  `.checkpoints/kit-v3-3/`, and its checkpoint were deleted in its last commit, 8b7cfb3, as CHECKPOINTS.md §9 says,
  and every notes path cited here stays in the branch history, last at 6c6c91a, the same files as at 4f268da):
  1. Asking before kit and gate edits: routines had it since #30; day sessions now ask too (file tools and shell
     writes), and an agent's or teammate's kit edit is refused [verified: `routine-guard.mjs:741`, `:865`, `:939`].
  2. The URL-escaped secret gap: the saver's `sk-`, AWS and Slack patterns now accept a key behind `%XX` and other
     escapes [verified: `checkpoint.mjs:30-40`]; before A3 only `%3D` worked (main's `checkpoint.mjs:30`). Anthropic
     `sk-ant-` keys were already caught. GitHub secret scanning and push protection are on (Mustafa).
  3. Read-only GitHub Actions tools for pr-advisor: `actions_get` (get_workflow, get_workflow_run, get_workflow_job,
     get_workflow_run_usage) and `actions_list` (its four list methods), this repository only; the night shift and
     every other method stay refused [verified: `routine-guard.mjs:1373-1396`]. Until A3 merged (af8393e), routines
     ran main's guard, whose read pattern `^(get_|list_|search_)|_read$` matches no `actions_*` tool (12d7961's
     `routine-guard.mjs:594`), so the PR Advisor read CI from the PR's check runs and the jobs' logs.
  Also: `git rev-list` for routines and `--output` refused for rev-list, diff, log and show [verified:
  `routine-guard.mjs:306`, `:329`, `:333`]; the docs and the guard's refusals name python beside node (`:953`); the
  advisor's tools reworded (E10-2: `memory: local` also turns on Read, Write and Edit, `advisor.md:8`, `:155`).
  **Stated limits** (plan revision 2 §2, with L1, L4 and L9 widened and L12-L15 added by the advisor's fix-round
  ruling, `.checkpoints/kit-v3-3/advisor-fix-round.md`, and L9 and L11 widened and L16-L18 added by its fix-pass-2
  ruling, `.checkpoints/kit-v3-3/advisor-fix-pass2.md`, and L16 and L18 rewritten and R-PLAIN and D-ASK added by its
  pass-3 ruling, `.checkpoints/kit-v3-3/advisor-pass3.md`, and Mustafa's scope stop of 2026-11-05, and R-PLAIN
  narrowed (R-PROG) by its pass-4 ruling, `.checkpoints/kit-v3-3/advisor-pass4.md`; the ask is a
  seatbelt in a watched session, not a wall: the PR review and Mustafa's go remain the wall):
  - L1, scripts and other runners: a `node`/`python` script, inline code (`node -e`, `python -c`), a shell given a
    command string (`bash -lc`, `bash -e -c`, `/bin/sh -c`, `dash -c`, `ksh -c`), `eval` after other words, and
    `sudo`/`doas` are not inspected: a kit edit made through them gets no ask. (`bash -c`, `sh -c`, `zsh -c` and `eval`
    at the start of a command are refused by settings.json [verified: `settings.json:243-246`]; `node .gate/*.mjs|*.js`
    runs without a prompt [verified: `settings.json:50-51`].)
  - L2, other writers: `install`, `dd of=`, `git config --file`, `git format-patch -o`, `git archive -o`,
    `find -exec`/`-delete`, `xargs`, `patch`, `tar`/`unzip` extraction, `perl -pi` with other bundled flags; paths
    built at run time.
  - L3, user settings: `~/.claude/settings.json` and `settings.local.json` (where `disableAllHooks` would switch the
    guard off) are not kit paths for the day ask.
  - L4, patch-applying commands and HEAD moves: `git cherry-pick`, `am`, `apply`, `revert`, `read-tree -u`, `stash
    apply|pop`, in any folder, the main folder included; and `git symbolic-ref HEAD <ref>` (moves HEAD, not files).
  - L5, auto mode and `.claude/` (E10-1): Claude Code's own classifier may refuse a `.claude/` edit before the prompt;
    if it approves, the call "normally still goes to the prompt" (hooks page). Never a silent edit; make such edits in
    Accept edits or Manual mode.
  - L6, the advisor's memory tools (E10-2): `memory: local` gives the advisor Write and Edit, which the page does not
    limit to its memory folder; the guard holds it in routines (pr-advisor writes only `.checkpoints/`, `.gate/` and
    agent memory) and refuses its kit edits in day sessions, but in a cloud day session its Write to a game file runs
    without a prompt (cloud sessions pre-approve file edits): only its own rules stop it (`advisor.md:168`).
  - L7, mods (E10-4): a mod that handles `tool.check` answers after the PreToolUse hooks and, without managed settings
    or a Team or Enterprise login, can approve a call a hook blocked or would ask for (permissions page). The guard
    cannot see or stop this, day ask and routine guard alike; mainly the PC (PC-SETUP, Decisions pending).
  - L8, sessions without repository hooks (E10-5): a cloud thread in a project, or a cloud session with several
    repositories, loads none of the repository's hooks (settings page, "Settings in cloud sessions"): no day ask, no
    routine guard. So each routine is one repository outside a project (Routines; CLAUDE.md:140).
  - L9, case and Windows spellings: Windows Git Bash `/c/…` spellings of a path; and, under WSL, Windows drives
    (`/mnt/c/…`) are case-insensitive while the guard folds case only on Windows and macOS. The night switch check
    does not fold case either; the night shift runs only on Linux.
  - L10, routines may still read other repositories through `search_*`, `get_job_logs` and `gh … -R/--repo`; only the
    new Actions reads are scoped to this repository (follow-up F4).
  - L11, worktrees other than the main folder: the main-folder checks do not apply there; the kit-edit ask does. A
    new clone or worktree made in a folder that is not a kit folder (for example `git clone <url> tools/x`) brings its
    own kit files, such as a nested CLAUDE.md, with no ask.
  - L12, a forged origin/main in a day session: a day session's `git fetch origin <x>:refs/remotes/origin/main`
    repoints origin/main, so the main-folder checks compare with the wrong commit. (Routines refuse it.)
  - L13, where the ask is untested: a hook's ask is not yet shown to prompt in a cloud day session or in
    bypassPermissions mode (E11 P1/P1b after merge, plus a bypassPermissions run).
  - L14, git aliases in day sessions: an alias (`sw = switch`, `co = checkout`, a `!` alias) runs past the
    main-folder checks and the git write checks.
  - L15, the shell profile: a cd function or alias defined in the user's shell profile, or a profile that sets
    physical mode (`set -P`) with a symlinked folder, can move the folder where the guard believes it knows it.
  - L16, deliberately disguised shell text: a day line written to hide what it does from a text reader (a variable
    name built by expansion, such as `export "${n}IR=…"`, `declare -n`, `printf -v`, `read` or `mapfile` into a git
    variable, a cd reached through tricks the cd rules do not list, or shell syntax the guard neither reads nor flags,
    such as brace expansion) can pass the day checks. The day ask is a seatbelt against accidents, not a wall. Lines
    the guard knows it can't fully read ask (D-ASK). Routines are held to a plain shell form that the guard reads
    exactly as bash does (R-PLAIN), so they are refused such lines.
  - L17, folder deletes and moves: a folder target asks when git tracks a kit file in it, when it holds a worktree of
    this repository, or when it is the project folder or an ancestor of it; kit files git does not track, and nested
    repositories that are not worktrees of this one, are not seen.
  - L18, parallel tool calls: a `git fetch` running in one tool call can move a ref after the guard checked a night
    `git switch` in another call made at the same time. NIGHT_SHIFT.md step 4 says to fetch first and wait. A routine
    never runs git in the background (a background `&` or run_in_background on a line with git is refused), so a fetch
    made in an earlier call has finished before the next call is checked.
  - **R-PLAIN, routines' plain shell (fix pass 3, narrowed in fix pass 4):** before the guard reads a routine's Bash
    line, the line must be in a plain form that the guard splits exactly as bash does, or it is refused. Outside quotes
    it allows letters, digits, spaces, tabs, newlines, the characters `_ - . / : = @ % + , ^`, a `~` inside a word, and
    the operators `&&`, `||`, `;`, `|`, `|&`, `&`, `>`, `>>`, `>|`, `<`, `&>`, `&>>`, and `>&` followed by a number or
    `-` that ends the word. It refuses `<&`, `;;`, `;&` and a quoted word right before `<` or `>`. Inside double quotes
    it allows no `$`, backtick or backslash. Every quote must be closed. Every command starts with its program, unquoted
    and by its bare name: git, gh, npm, node, python, python3 or date, after only `TZ=`, `LC_ALL=`, `LANG=`, `LANGUAGE=`
    or `NO_COLOR=` settings, and its redirects come after the program (R-PROG, fix pass 4). So routines use no comments,
    expansions, here-documents, braces, brackets or parentheses; no cd, no wrapper (env, timeout, nice, stdbuf,
    xvfb-run, nohup, time, command, exec), no shell keyword (if, then, do, while and the rest), no program given by a
    path, and no command without a program. A corpus test against real bash checks the claim.
  - **D-ASK, the day's unreadable lines (fix pass 3, Mustafa's scope stop of 2026-11-05):** a day line holding any of
    five forms the guard does not read (a comment, a line continuation, ANSI-C quoting `$'…'`, a here-document
    delimiter that is not a plain word, or `>&` or `<&` before a file name) asks the main session, and an agent's is
    refused; the scan goes on, so a later refusal on the line still wins [verified: `routine-guard.mjs:141-153`,
    `:202-249`, `:670-679`]. The guard does not parse these forms: "any line the guard can't fully read is an ask,
    not a parse" (Mustafa).
  - Closed, so no longer limits: by the fix round's cd rules R-a to R-e (2acfbbc), a cd inside `if`/`while`, a
    `CDPATH` set in the shell profile, and a cd behind `env`/`nohup`/`timeout` followed as a move; by fix pass 2
    (c8cbb1a; SD-4: a cd joined by `&&` after a cd the guard could not follow now stays unknown), a cd after a
    failed cd; by fix pass 3 (9589bfb), for routines, a command or a write hidden behind a comment, a
    here-document, ANSI-C quoting or `>&` before a file name (P3-1 to P3-3, P3-9: R-PLAIN), and paths named on a night
    commit (P3-5: the allowlist). (None had its own line here.)
  - E11a (`.checkpoints/kit-v3-3/e11a.md`, a nested `claude -p` in the worktree): the ask stopped a Write to a kit file
    outside Claude Code's protected paths (`docs/probe/CLAUDE.md`) in default, acceptEdits and auto modes; an
    untrusted workspace ignored the project's 145 allow entries but still ran its hooks. Not shown: a real prompt
    reaching Mustafa, a cloud session, his PC (E11 after merge).
  - Also (plan revision 2 §9): a day session reloads changed settings, so a `.claude/settings.json` write that slips
    past the ask (L1-L4) applies at once (E10-3); a guard error in a day session is "no decision", so no ask
    (`guard.mjs:46-48`); kit change-sets now prompt at every kit edit in day sessions.
- B2, not the guard: the GitHub tool that reads labels by number fails for a PR ("Could not resolve to an Issue with
  the number of 34"); labelling still worked.
- **v2-decision-drafts** (PR #33; the report `docs/reports/2026-11-01-v2-decision-drafts.md` has the details):
  1. Ten spend-limit cuts (2026-10-31 about 19:18; 2026-11-01 about 05:30, 09:27, 14:20, 19:35; 2026-11-02 about
     09:50, 12:15, 17:50, 21:20; 2026-11-03 about 13:55), each resumed from the agents' notes, no work lost (every agent writes as it goes; Mustafa's
     save-as-you-go rules of 2026-11-02); the report has the details.
  2. The asset pipeline is not researched (the research has one glTF mention, R3-CHAR-F8): ADR 0005 lists its parts as
     "not researched" open questions for the asset-pipeline research unit (ASSET-RESEARCH (a)).
  3. Several LOOK_V2 items rest on techniques not researched (in ADR 0003: the ink's inner lines and per-class weight, the
     sun shadow setup, the lighting model, the roof fade, particles, line boil, vegetation; OQ-0003-2 and -4 to -9): the look proof test decides them.
  4. PR #33's history holds 31 MB of a skeptic's build output (commit 294b3cf); the squash merge keeps it out of main.
- **R1–R6 research** (its report, "Risks"):
  1. Closed 2026-10-31 by MODES-3 (a): the largest mode is 48 players, so the three marks that failed only at 54 are no
     longer at risk. At 48 and 60 Hz the re-flood tick still has almost no margin (carry-forward 2), and the Advisor
     made 30 Hz the baseline (TICK-30).
  2. R3–R6 are spot-checked only: the drafts must verify every ADR claim and number, R3–R6's included.
  3. Unread sources: 67 rows where a recommendation hinges on one wait for other copies (`docs/research/UNREAD_SOURCES.md`).
  4. The org spend limit cut the research five times; notes-first kept every cut recoverable.
- **R7 legacy inventory** (its report, "Risks"):
  1. R1–R6 use the marks before Mustafa confirms them: answered by R7-MARKS (a), the marks are working recommendations.
  2. Some `ANIMATION_PLAN.md` §6 numbers come from headless snippets that are not in `tools/`, so they can't be re-run
     as is.
  3. The inventory's line numbers go stale once phase 1 moves the legacy files to `legacy/`.
- **3a, ranked (all from the change-set's evidence; not re-measured by docs-keeper):**
  1. Cabana: buildings are not NAV cells, so 27.5 % of objective routes and 25 % of siege legs cross a building roof bots can't climb (`tools/reach.js`; R19: the `navBuild` building-cell fix with bot replanning; queued as Night queue item 2, NEEDS PLAN, right after N1 and before 3b)
  2. Allocation grows with team fights (garbage-included inspector profile, MB per game second inside update(), seeded): default Cabana natural match head 4.6-4.8 vs main 2.4-3.1 (head's ally and Soviet bots fight, main's bots don't), Cabana ring combat head 1.72-2.09 vs main 1.94-1.98; DM_Tri 12 soldiers 2.9-3.0 and 48 soldiers 7.2-8.2 after the review fixes (main Cabana at 48: 6.2-6.3). Driver: `updateSoldierAnim` hit/death states (the CS11 anim split); `syncSimWorld` runs twice per step since K4 (0.39-0.43 after the house-array fix, was 0.56-0.65; main 0.16 for its one call)
  3. Draw calls: about 120 per on-screen soldier (pre-existing). Merge per-material parts, prune shadow casters, add LOD before team size rises
  4. DM_Tri: Soviet and Empire enterable barracks doors face south (rot 0 for the nav grid); the HUD shows two teams, two silo dots and "Cabana Republic" (3b)
  5. Colour (3b review, Brave decides): ally teal bars and tracers have little value contrast on grass; unit-type accents tint bot faces red / blue on any team; the 'SUPPLY CACHE' world label clips (pre-existing)
  6. Truck lanes are map data, so moving a refinery in the editor leaves its dock (editor change-set)
  7. `teamSpawnPoint` at team size 16 can put a soldier inside a building, and the spawn line runs along world x (3c)
- First-deploy tutorial lists controls that no longer exist (V melee, B shield, 1–5 switch): every new player sees it (fix paused with tutorial-field-manual until the greybox loop is playable)
- team-prep-fixes follow-ups (low): NC-7 the grenade-arc preview still loops `rayStatic` per static (about 77.7 KB per call while aiming; small perf change-set); PERF-1 `resolveShot` about 112 B per live player (5.4 KB per shot at 48) and PERF-2 the 7 boxed `rayStatics` arguments (the 103 B left per LOS call), both deferred to multiplayer prep; player shots and LOS still see ramp boxes (needs a sloped-ramp primitive in SIM: queued as "ramps as slopes in SIM"); NC-1 (bot sight gate flat at 1.45 m): fixed in 3a, `acquireTarget` now uses each soldier's own height [verified: shamble-game.js:1588]. Label note: "NC-7" here is the grenade-arc preview; the 3a queue's NC-7 is blast reach by line of sight (ROADMAP)
- Editor: Ctrl+Z (undo) changes the ground textures (old, reported by Mustafa): fixed and merged (PR #8, b5f4ecf; Mustafa confirmed Ctrl+Z in Brave). Root cause: `stylizeScene` runs only at boot, so rebuilt roads / crates / buildings / barracks / terminal lost toon + ink; also `clearMap` never cleared trees / props, so every undo / redo / reset / load stacked another copy of all 70 trees and 36 props, also into snapshots and saved maps
- Maps saved on older builds after an undo may contain duplicate trees and props: map-dedupe-on-load drops exact duplicates on Load (PR #10, merged, Brave check passed). Limit: on an old stacked save, Cabana trees and props that the map had deleted or moved come back once. They are single entries, so they stay; Mustafa removes them by hand. Measured on the edited fixture: 69 / 35 intended, 71 / 37 after dedupe
- Deferred (map-dedupe-on-load): prefabs saved by marquee over a stacked area can stamp exact stacks again. Exposure is small: prefabs could only be saved since dbe24f8 (2026-10-14). The next map Save and Load drops them. Follow-up: dedupe on prefab load or stamp with the same key
- Deferred, info, older (SEC-MDL-2): `envMerge` / `rulesMerge` use `Object.assign`, so a stored `__proto__` key swaps the prototype of that local env/rules sub-object (`Object.prototype` stays clean). Belongs with the server-side map-def validation (ADR-001; in v2: ADR 0005 CD-2 and CD-9, README register row SEC-MDL-2)
- Editor bugs reported by Mustafa (2026-10-15, editor.html on main): placed trees can't be deleted; rotation does nothing when the slider changes; the editor lags after a few placements. Fixed by editor-hotfix (PR #13, 8f4c3b6, merged after Mustafa's Brave check): delete, rotation, inspector selection capture; the lag needed no code fix, PR #8 and PR #10 removed its causes
- N1 (found in 3a): the editor's Play never rebuilds the nav grid; a one-line change-set, Night queue item 1 (APPROVED), next after the 3a merge [verified: `editorPlay` sets mode='game' itself and never calls `deploy()`, where `navBuild()` runs]
- Deferred, low (editor-undo-ground PERF-2): `_disposeObj3D` disposes the shared cached BK textures (houses / covers already did on base; trees / props add the well roof `tile`). r128 re-uploads them next frame, nothing breaks. Follow-up: skip `BK._tex` textures in `_disposeObj3D`
- Pre-existing, low (art review): `serializeMap` rounds to 0.1 m, so the resupply terminal snaps about 2 cm on a rebuild; the editor playtest camera clips into the barracks roof (no roof cutaway in playtest)
- Deferred from gate-check-5 reviews: no SRI/CSP on the three.js CDN tag (belongs with hosting, ADR-001; in v2: ADR 0002, three.js bundled with no CDN tag, and ADR 0006 part 13 for the CSP, README register row CDN-SRI)
- GitHub branch protection on `main` is on (Mustafa, 2026-10-17, in the implementer session's chat: "PROTECT-MAIN is done, main is protected"). Rules, read with the GitHub REST API on 2026-10-17 (`gh api repos/Mergincanli/Shamble-Frontlines/rules/branches/main`): "Shamble Ruleset 1" (active, targets the default branch `main`) blocks deleting and force-pushing `main`, requires a pull request (0 approvals) and requires the `gate` and `smoke` checks; a branch behind `main` may still merge (not strict); no classic protection rule. The `.claude/settings.json` deny rules stay a second layer (fragile by design). Decision PROTECT-MAIN: done (recommended option (a), plus a required pull request)
- Fixed (PR #22 review, finding 5): CLAUDE.md's merge sentence now says the go comes after the APPROVE of the PR's current head and a later push needs a new review and go, as advisor.md:98–105 does (kit-v2-2, PR #23, 48ffb51)
- Deferred, low (kit-v2-1): `tools/gate.mjs:333` and `tools/README.md:34` still call `gate-globals.json` "reviewed data, not a gate tool", which contradicts kit-v2-1's category 3 (gate files need Mustafa's go). It errs on the safe side; fix the wording when gate.mjs is next touched
- Open, for a later security review (kit-v2-1): night-shift rules 3-4 (`.claude/NIGHT_SHIFT.md`) trust any "Night shift running" comment on the public issue #16, whoever wrote it, so an outside comment could make a night run stop silently or take over a "cut-off" item
- Open, medium (security review of kit-v2-2, finding 4; existed before kit-v2-2): CLAUDE.md "Standing permission" lets a session accept an advisor approval read from a PR comment by the `Mergincanli` account, but every session posts as that account (`advisor.md`:103–104), so a confused or injected session could post "APPROVE <SHA>" on its own head. Mustafa's own go in chat is still required. The fix is its own change-set and still needs a plan; that plan picks the option (accept only the merging session's own advisor verdict, or a check run sessions can't write) as a decision for Mustafa
- Deferred, low (security review of kit-v2-2, findings 2 and 5): "the advisor's review" can be read as any verdict, not only an APPROVE, and smaller wording gaps remain between CLAUDE.md and advisor.md (listed in reports/2026-10-17-kit-v2-2.md, Reviews); one later rules PR edits both files together (needs Mustafa's go)
- The 3D scene renders every frame behind the opaque main menu (wasted GPU)
- Bot nav, editor-made layouts (R1, tracked per the Advisor): rotated colliders at odd angles keep up to 0.34 m of solid inside free nav cells, and wall gaps of about 3-5.4 m can close for bots depending on grid alignment (built-in maps unaffected). Recommendation: an editor-side reachability check; new maps keep walkable gaps at least 6 m wide
- Bot nav, navSteer "replan storm" (old; bot-poplar-wedge PERF-2): a bot whose straight line flickers clear/blocked replans every few frames; the wedge fix hits it more on Romania (budget-limited frames 0.9% -> 1.8%). Advisor: its own change-set in the greybox phase (bot-replan-storm)
- Bot nav, A* gives up after 8000 nodes (old): a few far-corner routes fail and fall back to a straight line
- The game reads performance.now()/Date.now() in 11 places, so matches with the same seed differ run to run. Tracked as an input for ADR-001 (deterministic shared simulation; in v2: ADR 0004 part 4 and ADR 0002 part 7, README register row WALL-CLOCK)

## Health (latest measurements)
| Metric | Value | When | Source |
|---|---|---|---|
| Combat allocation (GC churn) | ~4.9 MB/s (audit harness, scripted combat; not comparable to the 2026-10-15 headless match) | 2026-09-26 | audit CS11 |
| Perf baselines, team-prep-fixes head (headless): `hasLOS` | 103 B per call (was 20-26 KB) | 2026-10-15 | reports/2026-10-15-team-prep-fixes.md |
| `resolveShot` / `enemyFire` / `syncSimWorld` | `resolveShot` 648-750 B at 6 players, about 5.4 KB at 48 (was 18.1 KB) · `enemyFire` 0.88-1.0 KB per call (was 3.7-7.0 KB) · `syncSimWorld` 561 B per call (pre-existing, the houses fallback array) | 2026-10-15 | reports/2026-10-15-team-prep-fixes.md |
| Allocation method (the audit row is stated to use it; the team-prep-fixes rows do not name their method [assumed the same]) | Inspector sampling profile with `includeObjectsCollectedByMajorGC` / `MinorGC` (garbage included). `node --heap-prof` counts only objects alive at exit and is NOT used for rates. 3a rows are in Open bugs and risks (item 2); they come from the 3a commit messages, not re-run here | 2026-10-16 | tools/perfbots.js header |
| Combat garbage per game second | close-combat ring 6 / 12 bots 2.5-3.2 / 2.0-2.5 MB (was 15.5-16.1 / 28.3-28.5) · natural 90 s match 1.79-1.97 MB (was 2.80-3.05) | 2026-10-15 | reports/2026-10-15-team-prep-fixes.md |
| Village draw calls | 202 (was 398) | 2026-09-26 | audit CS12 |
| Headless match stall % (tools/sim.js, 40 unseeded runs each) | base mean 8.82 / median 7.5 · bot-poplar-wedge mean 7.73 / median 6.5 | 2026-10-14 | sim.js |
| Seeded match pairs (240) | tree-face stall windows 1885 -> 0; mean stall 9.12% -> 8.50% | 2026-10-14 | scratch harness (report) |
| Editor rebuild (Cabana): full-map draw calls boot / 1 / 2 rebuilds | base 1992 / 2406 / 3502 · head 1992 / 1992 / 1992; heap after GC boot / 10 rebuilds: base 42.9 / 119.5 MB · head 42.7 / 44.5 MB | 2026-10-14 | preview.js harness + perf-auditor |
| Gate | on main 8675367: full gate PASS in 26 s (Advisor's independent run), all 8 checks (1-4, 5a-5d); core 134 / game 691 / editor 349, 44 editor refs guarded (re-measured on main + kit-v1-2), smoke 8/8 + 5/5 steps | 2026-10-14 | gate.mjs |
| CI on main | `gate` and `smoke` success on 44446a5 (PR #44's merge commit) after one re-run: run 42673290304's first attempt lost its `gate` job on GitHub's side (only `smoke` ran, green); attempt 2 green, 06:37 UTC | 2026-11-10 | GitHub Actions runs (API) |
| Gate (official battery, older) | 0 flagged, 44 editor refs all guarded | 2026-10-10 | battery.js |
