# STATUS_ARCHIVE: the rest of STATUS
_What this holds: Mustafa's STATUS-DIET of 2026-11-10 puts "the rest (done change-sets, old measurements, closed risks)"
here, beside the short `docs/STATUS.md` and his decisions in `docs/DECISIONS.md`. Everything below was moved here by a
script, by line ranges, from `docs/STATUS.md` at main 2d22b69 (`git show 2d22b69:docs/STATUS.md`; the script, its line
ranges and its checks: `docs/reports/2026-11-10-status-diet.md`), except what docs-keeper has added since: merged PRs'
rows in "In flight", new items in the two live sections (below), and blocks moved later from STATUS by script, each
under a dated header that says so (at the end of "Phase", 2026-11-23 and 2026-11-24, of "Routines", 2026-11-23, and of "Follow-ups", 2026-11-25). Each section keeps the old section's name, so a
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

_From STATUS's "Follow-ups", moved here on 2026-11-23 by docs-keeper's script, byte for byte (the CI-WASM docs pass, PR #64): "this chat" is lane 1's first builder chat, paused since his words of 2026-11-19, 21:44:09 UTC (`docs/DECISIONS.md`); the hold itself stays in `.claude/CHECKPOINTS.md` §3 for every session._
- **Binding this chat:** `.claude/CHECKPOINTS.md` §3's hold; the advisor agent's ruling of #58: manifest and lockfile
  edits stay uncommitted and unbuilt, other code is committed by path only, until security-auditor answers.

_From STATUS's "Phase", moved here on 2026-11-23 by docs-keeper (the CI-WASM docs pass, PR #64, to keep STATUS within 120 lines; still current: STATUS points here, and his words of 2026-11-13 stand whole in `docs/DECISIONS.md`). His quoted words below are STATUS's byte for byte; the bullet's first two lines are not: at f5030a3 (`docs/STATUS.md:14-16`) they read "KIT-FREEZE-2, what he meant by it (FREEZE-FDN) and his exceptions (two more since, for #53 and #54):" and were re-wrapped here. Corrected 2026-11-24 (change-set 3a's docs pass, from the advisor agent's read of #64's final head): this header said the whole block was moved by script, byte for byte [verified 2026-11-24: `diff` of f5030a3's STATUS lines 14-19 against these lines]._
- **The kit:** KIT-FREEZE-2, FREEZE-FDN and his exceptions (two more since, for #53 and #54): `docs/DECISIONS.md`,
  "Queued"; his two additions of 2026-11-11 for the foundation chat: `docs/DECISIONS.md`. His words of 2026-11-13 for step 2.3:
  "The kit's freeze (KIT-FREEZE-2) ends with this step. That makes no edit unasked: every edit to a kit, gate or CI file
  is asked of me in chat with its exact text and written only after my yes, with the chat in Accept edits. A guard
  prompt alone is not my yes. No such edit happens in the docs pass or while planning."

_From STATUS's "Waiting on Mustafa", moved here on 2026-11-24 by docs-keeper's script, byte for byte (change-set 3a's docs pass, to keep STATUS within 120 lines; still current: STATUS points here; the archive's "In flight" and "Phase" it names are this file's sections of those names):_
- Legacy, frozen (the archive's "In flight" and "Phase"): his Windows run ("Check a new machine", `tools/README.md`);
  the UI theme step 1 Brave check (DEV-panel FPS); `/permissions` locally (kit-v1-1); the editor inventory marks
  (docs-editor-v2); re-saving each old map once (map-dedupe-on-load). Routines: delete the old "Night shift" (below).

## In flight
| Work | Branch | State | Waiting on |
|---|---|---|---|
| the preview hosting check (ADR 0006's spike 1, SPIKE_PLAN entry 6), by lane 2 (the chat he calls Builder V2), run in his test repository Mergincanli/shamble-preview-check; report `docs/reports/2026-11-24-preview-hosting-check.md`: the runs are done; by the draft's rule not passed yet (the header mark not met as written; the build-count mark and his verdict on checklist step 4 open) | `claude/optimistic-albattani-mdoonz` (stands in for `cs/preview-hosting-check`) | merged: PR #66 (`claude/optimistic-albattani-mdoonz` (stands in for `cs/preview-hosting-check`), Builder V2), 09faefa, 2026-11-25 11:10:11 UTC (`merged_at`), by merge commit (a normal merge, his HISTORY-ID (a), on his go); risk tier 1 (the report and his seventeen messages in `docs/DECISIONS.md`) | none |
| **change-set 3a, "Rules and boundaries"** (cs3a-rules-boundaries; the first half of change-set 3, his SIZE-3 (b)), by the chat he calls Builder V3: the simulation's rules under tests (the scan of `sim`, our own ChaCha8 against the published vectors, the float vectors natively and in WebAssembly, `mul_add` refused and pinned by his MUL-ADD (a)), the panic hook's text on the page, the build's refusals; report `docs/reports/2026-11-24-cs3a-rules-boundaries.md`. Follow-ups (their text: the archive's "Follow-ups", "From change-set 3a") | merged: PR #65 (`claude/loving-hopper-6p0mg2`, standing in for `cs/3a-rules-boundaries`, Builder V3), 28254a7, 2026-11-24 22:40:39 UTC, by merge commit (parents 1ec6f6c3 and 1624f084), by his main Advisor on his tap of 22:40:00 UTC, with the expected head 1624f084924976f4bde8556509c957e1f4634341 (the merge commit's message; his words of 22:44:50 UTC, `docs/DECISIONS.md`: "That tap was my go."); gate, smoke and toolchain green on main (read 22:45 UTC, change-set 3b's card) [verified 2026-11-25: the merge commit's parents and message, `git log`] | done: risk tier 1; his GH-NEXT (a): the chat put its title and text in its notes folder, his main Advisor opened the draft, he pressed Ready and Merge; built in 8 steps; the last commit that changed what runs is 8619d539, the corrected head 8d277cf2 (comments, docs), CI green on both; two review rounds by five reviewers, then a third by security-auditor on one fix (APPROVE: the High closed); the verifier PASS and the advisor agent APPROVE on 8d277cf2; his browser check of 2026-11-24, 21:48:49 UTC: errors 0 in Brave and in Firefox (`docs/reports/2026-11-24-cs3a-browser-check.md`: the Shields setting and the page opened as a file are shown; no gap with the window covered); the notes: commit 6491b839; then the last commit 1624f084 (his browser check recorded, the report's last version, the checkpoint folded and removed), the head the chat reported as approved by its advisor agent and covered by its verifier's PASS (the merge commit's message) | none |
| **the CI-WASM pull request** (ci-wasm; planned by the first builder chat, built by the builder chat he calls Builder V3; it replaces #63, the project thread's records, and #62, the first builder chat's plan and first code; risk tier 2): before the network cut CI's `toolchain` job fetches the wasm-bindgen tool's archive by his TOOL-SOURCE values, runs GitHub's own check on it (`gh release verify-asset`) and installs its one program root-owned; inside the cut it builds the page twice (production, development), then runs the boundary tests, five type checks and the 12 browser tests in the runner image's Google Chrome (CI pins no test count: TE-COUNT); `.github/workflows/toolchain.yml` +97 −0 in two writes on his yes (cb476b1; the correction 2a654f3, after the first text stopped at its install check), three planted faults each red at its own step and taken back; the build's refusal of cargo settings from the environment moved into `build-checks.mjs` with a test; `dist-*/` in `.gitignore`; `packages/e2e/node-names.d.ts`; his words of 2026-11-19 and 2026-11-23 in `docs/DECISIONS.md` by script; his browser check of #61 (`docs/reports/2026-11-19-cs2-browser-check.md`); VISION_V2's dated note (his VISION-NOTE (a)); report `docs/reports/2026-11-19-ci-wasm.md`. Follow-ups (their text: the archive's "Follow-ups", "From the CI-WASM pull request (PR #64)") | merged: PR #64 (`claude/loving-hopper-6p0mg2` (stands in for `cs/ci-wasm`), Builder V3), 1ec6f6c, 2026-11-24 10:15:31 UTC (`merged_at`; the merge commit is dated 10:15:30), by merge commit (parents f5030a3 and 8aadfb0), by his main Advisor on his tap of 10:14:23 UTC in that thread, with the expected head 8aadfb09640c9d37a3e6ad8be078165f5074f399 (the merge commit's message; his words of 10:16:15 UTC, `docs/DECISIONS.md`), after his go MERGE-64 (a) for that head (2026-11-23, 23:44:10 UTC); `gate`, `smoke` and `toolchain` green on the head for the push and the pull request (the merge commit's message); CI green on main (gate with smoke, run 45373057216; toolchain, run 45373056953; push, on 1ec6f6c); #63 and #62 closed by GitHub itself as merged at 10:15:33 UTC, through 1ec6f6c, since their heads (21b6ea0, d064e17) are in #64's history: his CLOSE-OLD (a) had them closed once #64 was merged, and the merge did it [verified 2026-11-24, GitHub's REST API (`merged` true, the close event naming 1ec6f6c) and `git merge-base --is-ancestor`] | done: opened by him 2026-11-23 17:21:24 UTC (the report); two review rounds (infra-engineer, test-engineer and security-auditor; then test-engineer and security-auditor on what changed): no Critical or High; the verifier's PASS WITH NOTES on the code head 6d0b5b1 (the report, "Reviews"); the advisor agent's APPROVE of the final head 8aadfb0, given in the chat (recorded in `docs/DECISIONS.md`, 2026-11-23, 23:44:10 UTC, and in the merge commit's message). His go answered the chat's ask of 23:17:21 UTC, whose list of what his go accepts named the open Low F-CHANNEL, TE-TYPE-PLANTS, the two things built differently from the plan, `toolchain` still not a required check, the compiled Python file in the branch's history, and four inaccuracies in the report and this archive left for the next docs pass (corrected by change-set 3a's docs pass, 2026-11-24, with the Lows of the same read; the source: change-set 3a's card at f153ddf4, "Next steps" item 5). The notes of #64 stay readable at 5911b56 (`.checkpoints/ci-wasm/`). _Corrected 2026-11-24 (change-set 3a's docs pass): STATUS's row for #64 at 8aadfb0 said "the verifier's and the advisor agent's verdicts on the final head: the report"; the verifier's verdict is on the code head 6d0b5b1, and the advisor agent's on the final head came in the chat. STATUS's "Next" line left out his condition "If your context is above 40 % at that point, tell me first and I compact the chat." (2026-11-23, 21:24:59 UTC); by his order of 23:44:10 UTC the chat was compacted before 3a's build (his /compact, 2026-11-24, 10:50:43 UTC)._ | none |
| **change-set 2 of the build plan, "First page"** (cs2-first-page; foundation step 2.5, first half), by lane 1: the Rust simulation (a tick counter and the match seed; no random-number library yet, his RNG-CRATE (c)), built as WebAssembly with wasm-bindgen `=0.2.130`, runs in one module Web Worker at 30 ticks a second and hands the page one block per tick (so far its header: the tick and the layout's hash); the page, which makes no Wasm call, draws an empty lit ground with a 1 m grid on three.js 0.187.0's WebGPU renderer (WebGL 2 as the fallback), names its backend and has a development readout with "Copy report"; `packages/client/serve.mjs` serves a built page to this computer only, under a strict content security policy; `packages/client/scripts/build.mjs` builds the Wasm and the page and checks the tool's file (size and sha256) before every run; browser tests in `packages/e2e` (CI does not run them yet: his CI-WASM (a)); his six messages of 2026-11-18 and 2026-11-19 in `docs/DECISIONS.md` by script; report `docs/reports/2026-11-19-cs2-first-page.md` (the size: there). Follow-ups (their text: the archive's "Follow-ups", "From change-set 2, the first page (PR #61)"): for the CI pull request TE-NT6, RR-N4, RR-F5, E2E-TYPES, SA-TREE, SA-DIST; for change-set 3 NA-N1 to NA-N7, EA-F8, EA-F9, EA-F10, EA-F13, EA-N3, EA-N5, EA-N6, GE-N1, GE-N4, GE-N5, GE-N6, TE-NT3, TE-NT4, TE-NT5, TE-NT7, PA-N2, PA-L2, TE-L, SA-ENV, FAVICON; for change-sets 5 and 6 GE-F7, PA-L4; for the quality gate step SA-F6 (the npm scope `@shamble`), SA-SWAP, TE-NT2; for the drafts' next revision EA-F6, EA-F12 | merged: PR #61 (`claude/wonderful-hamilton-jdmlnf` (stands in for `cs/cs2-first-page`), lane 1's first builder chat), f5030a3, 2026-11-19 15:10:54 UTC (the merge commit's date; `merged_at` 15:10:55), by merge commit (parents b763ba3 and 4cf30ce), on his go of 14:28:25 UTC for head 4cf30ce303b16e96b3ca5d5c0b20b928a4db6b86, his words in the merge commit's message (`docs/DECISIONS.md`, 2026-11-19, 14:28:25 UTC), made after his browser check's last report (15:09:29 UTC; the record: `docs/reports/2026-11-19-cs2-browser-check.md`); `gate`, `smoke` and `toolchain` green on the head (read at 15:10 UTC); CI green on main (gate with smoke, id 44551896122; toolchain, id 44551894194; push, on f5030a3) [verified 2026-11-23, GitHub's REST API] | done: opened as a draft 2026-11-19 11:01:27 UTC; the eight build steps in (3833e3e, 784ae3e, 4c32a06, a7f785a, a68e3da, cc9fc71 (the dependency step, pushed through the hold), 6e76f30, 015a212, de63a30, c4a12eb, 74a0c24), then the fix passes 29378dd and d257bb6; round 2's fix pass: the report; two review rounds (seven reviewers, then six on what the fixes changed): no Critical, no High open; the verifier's verdict: the report; the advisor agent's: the pull request's description. **Not verified anywhere yet:** WebGPU itself (this container has no WebGPU adapter: every run here was on the WebGL 2 fallback with software rendering, in Chromium), Brave, Firefox, Safari, a real GPU (then his browser check, in Brave 1.100.58 and Firefox 160 on his Windows computer: errors 0 in all eight reports, WebGPU started in both, the forced WebGL 2 fallback and the hidden-tab pause worked in both; one reading not explained, BR-GAP: the record above and this archive's "Follow-ups", the block for #61). With the merge he accepted, in his words: "The things that went wrong, as the report writes them down", the size, "That CI does not build the WebAssembly or run the browser tests yet", the follow-ups below High and the checkpoint commits in main's history (`docs/DECISIONS.md`, 2026-11-19, 14:28:25 UTC); its merge recorded by #64 | none |
| **the kit hold and two CI edits** (kit-hold-ci), by lane 1, on his words of 2026-11-18, 15:38:13 UTC ("One small pull request with the kit hold and the two CI edits, nothing else in it."): the checkpoint script holds a push that changes a manifest or a lockfile until its security check is recorded (`.claude/scripts/checkpoint.mjs`, six new tests, one rule line in `.claude/CHECKPOINTS.md` §3; his answer DEP-HOLD (a), written on his yes to its exact text: KIT-TEXT, yes with one change, then KIT-FIX, the fix of a hole in the changed lines); the `toolchain` job runs with no cache (his CI-CUT (a), part 1), writes the generated types again and compares them, and type-checks `packages/types` with strict flags (his CI-TYPES (a)); his three messages of the day in `docs/DECISIONS.md` by script; 293 added lines, accepted by his word of 18:01:08 UTC; report `docs/reports/2026-11-18-kit-hold-ci.md`. Meanwhile, in his words: "No new library of change-set 2 is committed before that pull request is merged." Open after it, as follow-ups (below, "Follow-ups", "From the kit hold and two CI edits (PR #60)"), each below High and so a follow-up by his word of 18:01:08: security-auditor's two Mediums, F1 (in two corners the hold checks another path than the one it pushes: a root folder whose name begins with a blank character, or is not valid UTF-8) and F3 (no rule makes the review of a pull request read the hold's record; until a kit change, change-set 2's report quotes each record and the security answer it names), and its three Lows; test-engineer's three Mediums (behaviours no test pins: main's side of the "equal to main" comparison read at the exact path, a `deleted  <path>` line leaving an existing file held, `origin/main` over a local `main`) and seven Lows; infra-engineer's six Lows (two for his list for the quality gate v2's plan, four on the report's wording) | merged: PR #60 (`claude/wonderful-hamilton-jdmlnf` (stands in for `cs/kit-hold-ci`), lane 1's first builder chat), b763ba3, 2026-11-18 20:30:46 UTC (the merge commit's date; `merged_at` 20:30:47), by merge commit (parents 9b4b518 and e5ca735), on his go of 20:20:09 UTC for head e5ca7358d5a3f8cc68ee07cd59d7bbf2c45e253d, his words in the merge commit's message (`docs/DECISIONS.md`, 2026-11-18, 20:20:09 UTC), after the advisor agent's APPROVE of that head (in the pull request's description); `gate`, `smoke` and `toolchain` green on the head before the merge; CI green on main (gate run 900 with smoke, id 44391809186; toolchain run 198, id 44391809357; push, on b763ba3) | done: the four files written on his yes at 17:15 UTC, two of them again at 18:01 UTC (commits 8bf4bee and 94c197f, pushed 17:18:16 UTC; 4877599, pushed 18:03:10 UTC); the new CI lines on real runs from 17:18 to 17:23 UTC ("Cache mode: none" in the job's log; the compare step failing on a planted change, then passing on the clean tree); one review round, its notes committed at 18:08:01 (infra-engineer), 18:31:54 (security-auditor) and 18:40:04 UTC (test-engineer): nothing Critical or High; the verifier's verdict: the report; the advisor agent's: the pull request's description; with the merge he accepted the four things that went wrong as the report writes them down, the five Medium findings as follow-ups ("They are fixed with the kit changes of the quality gate step. Until then every ready report of a pull request that changes a manifest or a lockfile quotes each release record and the security answer it names."), the two wording slips left in the report, the size (293 added lines against the stop line of 300) and the checkpoint commits in main's history; his answers WRITE-MODE (a) and REC-OLD (a), with one limit; its merge recorded by #61 | none |
| **change-set 1 of the build plan, Types** (foundation step 2.4), by lane 1: the first eight message types in `crates/protocol` (plain types, no codec), the output block's layout table and its hash in `crates/client-endpoint`, their TypeScript generated into `packages/types` (ts-rs 12.0.1 as a test-only dependency, and our own small generator), the page's layout-hash check; his go on #57 and his answer OLD-LINES (who may fold old lines in the shared record files) in `docs/DECISIONS.md` by script; report `docs/reports/2026-11-18-cs1-types.md`; its follow-ups, open after it (CI-CUT; the security check's L1, L2 and L4; the lines for change-sets 2, 3 and 4, the 2.3 chat and the drafts' next revision; round 2's Lows): below, "Follow-ups", "From change-set 1, Types (PR #58)" | merged: PR #58 (`claude/wonderful-hamilton-jdmlnf` (stands in for `cs/cs1-types`), the build chat on his personal account), 9b4b518, 2026-11-18 15:39:28 UTC (the merge commit's date; `merged_at` 15:39:29), by merge commit, on his go of 15:38:13 UTC for head 1bfe462e339511cfae6b30a1015330d9ff2e718c, quoted whole in the merge commit's message (`docs/DECISIONS.md`, 2026-11-18, 15:38:13 UTC), after the advisor agent's APPROVE of that head (in the pull request's description); `gate`, `smoke` and `toolchain` green on the head before the merge; CI green on main (gate run 884 with smoke, id 44338178450; toolchain run 182, id 44338178455; push, on 9b4b518) | done: two review rounds and the verifier's PASS (15:03 UTC); with the merge he accepted the two misses as written (CI built the nine new packages before their security check, Medium, "written down, not waived"; one evidence record did not hold, and the runs were redone), the size as counted (940 hand-written lines against 600 planned, under his stop line of 1,200), round 2's Lows as follow-ups, what was not verified and the checkpoint commits in main's history; his five answers: DEP-HOLD (a) and CI-TYPES (a), and CI-CUT (a) part 1 (the kit hold and the two CI edits: PR #60), LANE-ISSUE (a) (the "Lane requests" issue: #59, opened 15:41:35 UTC) and WATCH-58 (b) (not watched); its merge recorded by #60 | none |
| **Part 15 of the build plan, "The second lane", to main** with his six answers (all (a), with additions), a docs change of its own (his go for #56): his words of 2026-11-17, 22:10:34 and 22:59:16 UTC, in `docs/DECISIONS.md` by script; Part 15 as he read it, with his answers on top: `docs/reports/2026-11-17-build-plan-slice-1-part-15.md`; dated pointers in ADR 0002 (a seventh package, VFX-ROUTE) and in the build plan report; the rules file's line on gun files (GUN-FILES, AX-R1); STATUS, ROADMAP, the archive and REPO_MAP after #56's merge. Then change-set 1, Types (foundation step 2.4), by lane 1; lane 2 (`claude/optimistic-albattani-mdoonz`) works by Part 15, each unit from the point its 15.2 names | merged: PR #57 (`claude/wonderful-hamilton-jdmlnf` (stands in for `cs/part-15-second-lane`), the build chat on his personal account), 22c6afa, 2026-11-18 11:09:31 UTC (the merge commit's date and `merged_at`), by merge commit, on his go of 11:08:25 UTC for head f26ac5b85396f4b280ba21e57f2a1ea177382d80, quoted whole in the merge commit's message (`docs/DECISIONS.md`, 2026-11-18, 11:08:25 UTC), after the advisor agent's APPROVE of that head (about 00:17 UTC, in the pull request's description); `gate`, `smoke` and `toolchain` green on the head before the merge; CI green on main (gate run 866 with smoke, id 44292101495; toolchain run 165, id 44292101705; push, on 22c6afa) | done: report `docs/reports/2026-11-17-part-15-second-lane.md`; docs-keeper's pass, the verifier's PASS (seven Lows, fixed) and the advisor agent's APPROVE at 3e4fea1 (five Lows, fixed); with the merge he accepted his go on #56 as recorded, Part 15's text as he read it (where it differs from his answers, his answers hold), the size as counted (220 written lines against the stop line of 300, and 339 copied), the lines left as found because another lane wrote them, and the checkpoint commits in main's history, and he answered OLD-LINES, who may fold old lines in the shared record files (his go, above); the advisor agent's three Lows on the final head: two in STATUS, fixed by #58's docs pass, and a size recount, left to #58's report; its merge recorded by #58 | none |
| **change-set 0 of the build plan** (foundation steps 2.4 and 2.5 and Slice 1, built now: his decision of 2026-11-14 and his go of 2026-11-17; the plan carried to main as `docs/reports/2026-11-17-build-plan-slice-1.md` with its appendix): his five messages of 2026-11-14 and 2026-11-17 in `docs/DECISIONS.md` by script (his go says: "No game code is built before change-set 0 is on main."; his CHANGE 2: "Two chats build the game on my personal account.", the second since 2026-11-15, `claude/optimistic-albattani-mdoonz`, Opus 5.5 with Fable 5.2 on its advisor tool, running the preview hosting check in a separate test repository); the model-file rules and the soldier's joint list, on paper, no export through them yet (`docs/research/asset-export-rules.md`, `docs/research/soldier-joint-list.md`; OQ-0005-AS3 and AS4 answered in part; ten new pass marks, PM-0005-47 to -56, first run at the plan's change-set M); his answers where the drafts ask (OQ-0002-6, OQ-0006-18, OQ-0003-1 further in part, the three "do soldiers block" rows, PM-0004-34's row; the register's open questions 98 to 96). Open after it, for "Follow-ups" once merged: the draw cap with a gun from its own file (the rules' O1; PM-0005-54 fails on the rifle file as reported: a question for him with change-set M's plan); the fifth material class's name (O2); the rules' proof by a first real Blender export (change-set M); whether the coat keeps the legacy motion (his call at change-set 10's pictures); the machine lines in ADR 0001, ADR 0003 and SPIKE_PLAN and ADR 0002's "one change-set per step", carried later (the plan report's "Since the plan" names each carrier) | merged: PR #56 (`claude/wonderful-hamilton-jdmlnf` (stands in for `cs/cs0-model-rules`), the build chat on his personal account), d7dad97, 2026-11-17 23:00:12 UTC (the merge commit's date; `merged_at` 23:00:13), by merge commit, on his go of 22:59:16 UTC for head cd61b28350c4407d2f19b1b3603ea4beca8b4e30, quoted whole in the merge commit's message (`docs/DECISIONS.md`, 2026-11-17, 22:59:16 UTC), after the advisor agent's APPROVE of that head; `gate`, `smoke` and `toolchain` green on the head before the merge; CI green on main (gate run 856 with smoke, id 44195404107; toolchain run 155, id 44195404068; push, on d7dad97) | done: 3747bb3, 4a1b0d7, 212f55b, 5f356ef, 65c8fe8, cacea55 (the two research files); two review rounds done (test-engineer, engine-architect, the verifier twice: PASS); the advisor agent approved its content at 47546e9, its conditions met in the last commits; report `docs/reports/2026-11-17-cs0-model-rules.md`; with the merge he accepted that PM-0005-54 stands as written (a fail with today's rifle prototype, not waived), the size as counted, the three things the verifier did not verify, and the checkpoint commits in main's history (his go, above); the open items: the Follow-ups, "From change-set 0 (PR #56) and Part 15 (PR #57)"; its merge recorded by #57 | none |
| **the night-shift lines: the saver in stints while a night run waits on agents** (his NS-STATUS (a) of 2026-11-16: `.claude/NIGHT_SHIFT.md` step 6, three lines added (+3 −0) and nothing else, their sha256 4e3d1565…; risk tier 2; his yes to the lines, NS-LINES (a), is the exception to KIT-FREEZE-2 for that one change) | merged: PR #54 (`claude/night-shift-stints`, from the netcode proof test's day session, session_0173vVo6igCNyQYBWPvhH36A), 82c605e, 2026-11-17 07:03:03 UTC (`merged_at`; the merge commit is dated 07:03:02), by merge commit (parents 53272a1 and 20f9d59), on Mustafa's go typed in that session's chat at 07:00:54 UTC (his words: `docs/DECISIONS.md`, 2026-11-17, 07:00:54 UTC), after the advisor's and the security-auditor's APPROVE WITH FINDINGS of 20f9d59 and the PR Advisor routine's APPROVE for that head (comment 6941066272); CI green on main (gate run 44035715175, toolchain run 44035715349, push, on 82c605e); its report: the pull request's description | done: his NS-STATUS (a) and NS-LINES (a) (`docs/DECISIONS.md`, 2026-11-16, 20:48:10 and 21:09:27 UTC); one review round (the advisor: five Lows, ADV-NS-L1 to -L5, merge conditions C1-C4; the security-auditor: two Lows, SA-NS-1 and SA-NS-2), the seven Lows deferred in writing by the session with no push (the Follow-ups, "From the night-shift lines"); the gate --no-smoke in a worktree, identical to main's; the kit tests 1,271 (1,270 pass, 1 skipped), as on main; smoke CI's; its merge recorded by DOCS-54 | none |
| **KEPT-LOGS: a narrow kit rule for kept raw logs and the old build cache** (`.claude/CHECKPOINTS.md` §11, `.claude/scripts/kept-logs.mjs` and its tests, two REPO_MAP rows; 4 files, +671 −0; risk tier 2; his KL-YES is the exception to KIT-FREEZE-2 for this one rule) | merged: PR #53 (`claude/kept-logs-rule`, from the netcode proof test's day session, session_0173vVo6igCNyQYBWPvhH36A), 53272a1, 2026-11-15 19:49:22 UTC (`merged_at`; the merge commit is dated 19:49:21), by merge commit (parents 4460ef5 and a637c7a), on his KL-MERGE (a), typed in that session's chat at 19:46:36 UTC (his words: `docs/DECISIONS.md`, 2026-11-15, 19:46:36 UTC), after the security-auditor's and the test-engineer's APPROVE WITH FINDINGS and the advisor's APPROVE WITH CONDITIONS of a637c7a; CI green on main (gate run 43754558567, toolchain run 43754558678, push, on 53272a1); its report: the pull request's description | done: his KEPT-LOGS (c), KL-YES with one change, KL-CHANGE-YES (a) and his go for the kit files' copy (`docs/DECISIONS.md`, 2026-11-15); the three kit files equal the text he said yes to with the diff applied, compared by script; one review round on a637c7a (SA-KL-1 to -4 and KLT-1 to -8, all Low, accepted by him as left open: the Follow-ups, "From KEPT-LOGS"); the 14 planted bugs, the GNU tar test and the dry runs for the two real folders; its merge recorded by DOCS-54 | none |
| **foundation 2.3: the first docs pass** (foundation-2-3-docs: his words for #48 and of 2026-11-13 in `docs/DECISIONS.md`, by script; dated pointers; the 2.2 report's fixes M1, L1, L2, L3 and L5; STATUS, ROADMAP and the archive true for main 4d21c1a; docs only, risk tier 1) | merged: PR #52 (`claude/sweet-mayer-grsukw` (for `cs/foundation-2-3-docs`), his personal account), 4460ef5, 2026-11-14 09:06:30 UTC (`merged_at`; the merge commit is dated 09:06:29), by merge commit (parents 4d21c1a and bfd719b), on Mustafa's go, given in foundation 2.3's day session's chat (session_019YNMUWSsyRgaYWxk4auQ5b) at 09:02:29 UTC by the merge commit's message, which quotes it whole (his words: `docs/DECISIONS.md`, 2026-11-14, 09:02:29 UTC), after the advisor agent's APPROVE of bfd719b (its day-session review); CI green on the head (gate runs 43576710810 and 43576714278, toolchain runs 43576710856 and 43576714298, by the merge commit's message) and on main (gate run 43578963868, toolchain run 43578963884, push, on 4460ef5); report `docs/reports/2026-11-13-foundation-2-3-docs.md` | done: two review rounds, the verifier (its FAIL on a1d0317, fixed in 922cadf; its re-run PASS WITH NOTES), the advisor agent's APPROVE of the final head bfd719b and his go; its merge recorded by DOCS-54 | none |
| **foundation 2.2: the toolchain** (foundation-toolchain, ADR 0002's migration plan step 2.2: the Rust and npm pins, six empty crates and five empty packages, the offline CI job `toolchain` (K1, with its release-test line K1b); the four kit lines on his yes; game symbols +0 −0) | merged: PR #48 (`claude/laughing-pasteur-fwya24` (for `cs/foundation-toolchain`)), 4d21c1a, 2026-11-13 16:20:12 UTC (`merged_at`; the merge commit is dated 16:20:10), by merge commit (parents 2aa5e9c and 9b20b67), on Mustafa's go, given in foundation 2.2's day session's chat (session_019YNMUSc7FA68ujhLgXfEJA) at 16:16:47 UTC by the merge commit's message, which quotes it whole (the chat record's event is stamped 16:16:36 UTC) (his words: `docs/DECISIONS.md`, 2026-11-13, 16:16:47 UTC), after the advisor agent's APPROVE of 9b20b67 (its day-session review); CI green on main (gate run 43461710700, its gate and smoke jobs, and toolchain run 43461710673, push, on 4d21c1a); report `docs/reports/2026-11-12-foundation-2-2.md` | done: steps 1 to 5 and the docs pass; review round 1 (five reviewers, 17 findings) and its fix pass (1de98a9 to 8a4f9b0), round 2 (four reviewers, nothing Critical or High) and its fix pass (790ac31); the verifier PASS on bd44a68; the advisor agent's APPROVE of the final head 9b20b67; his go: ADR-INDEX (a), K1-GO (a), OVERFLOW-ALL (a), four security items accepted as left open (OVERFLOW-PANIC-RULE; SETPRIV-HARDEN, SETUP-NODE-CACHE and CUT-ESCAPE for step 2.3), and the 2.2 report's fixes with his words' entry for step 2.3's first docs pass; its merge recorded by that pass (PR #52) | none |
| **foundation 2.1b: the move** (foundation-restructure, the second of step 2.1's two pull requests, his SPLIT (a): the five legacy files into `legacy/` with `git mv`, bytes unchanged, in a commit of their own (c090843); the commands in text name the `legacy` folder (1e6f84e); the four kit lines on his yes, KIT-2.1B (a) (b6b3b81); the docs: his words in `docs/DECISIONS.md`, his SCOPE (a) in ADR 0002 and the register, OQ-0002-2 answered in part (REG-TRUE (b)); game symbols +0 −0) | merged: PR #51 (`claude/clever-fermi-oudsfq` (for `cs/foundation-restructure`)), 2aa5e9c, 2026-11-12 13:22:32 UTC (`merged_at`; the merge commit is dated 13:22:31), by merge commit (parents 76b4f7a and 5d50bf2), on Mustafa's go, given in foundation 2.1b's day session's chat (session_019YNMUWdGJZoQFUuuvvBP1Y) at 13:18:42 UTC and quoted whole in the merge commit's message (his words: `docs/DECISIONS.md`, 2026-11-12, 13:18:42 UTC), after the advisor agent's APPROVE of 5d50bf2 (its day-session review); CI green on the head (push run 43130543086 and pull-request run 43130550831) and on main (push run 43226692719, on 2aa5e9c); report `docs/reports/2026-11-11-foundation-2-1b.md` | done: evidence E1-E6 and E8 PASS; review round 1 on the final diff (security-auditor APPROVE WITH LOWS, test-engineer and infra-engineer APPROVE; no Critical or High, so no round 2) and its fix pass (43733e5); the verifier PASS on 43733e5; the advisor agent's REVISE on 3860e3f (two Mediums: two earlier sends of his traced nowhere, and no `docs/DECISIONS.md` entry for his go for #50), fixed in 3828e6a; its APPROVE of the final head 5d50bf2; his REG-CELL (a) and PROMPTS-2.3 (a), his checks on 5d50bf2 on his Windows laptop (the saves backup, the Brave check of both pages, the gate) and his go; 2.1a's IE-2, IE-3, IE-4, IE-7 and SA-L1 closed (its report, "Decisions and deviations"); its merge recorded by foundation 2.2 (PR #48) | none |
| **foundation 2.1a: the legacy gate and tools read both layouts** (foundation-restructure, the first of step 2.1's two pull requests, his SPLIT (a); replaces #47, his BRANCH-47 (a); gate files and tools only, no game file, no file moved, symbols +0 −0) | merged: PR #50 (`claude/pensive-heisenberg-aazidf` (for `cs/foundation-restructure`)), 76b4f7a, 2026-11-11 18:38:28 UTC (`merged_at`), by merge commit (parents bb2d817 and f6becf5), on Mustafa's go, given in foundation 2.1a's day session's chat (session_019YNMUWLffMoE47SbvkqJ5P) and quoted word for word in the merge commit's message (with his approval of the two gate edits in dd12aeb and the five follow-ups he accepted), after the advisor agent's APPROVE of f6becf5 (its day-session review); CI green on the head (runs 43074627548 and 43074640388) and on main (push run 43079707687, on 76b4f7a); report `docs/reports/2026-11-11-foundation-2-1a.md` | done: the review rounds, the verifier and the report; then the advisor's APPROVE of the final head f6becf5 and his go; its merge recorded by foundation 2.1b (PR #51) | none |
| **status-diet: STATUS-DIET with LANE-RULES (ACCOUNTS, the Models addition)** (Mustafa's queue of 2026-11-10, items 2 and 3, with ACCOUNTS and his Models addition; docs and rules, no game code, symbols +0 −0) | merged: PR #49 (`claude/busy-turing-48e4i9` (for `cs/status-diet`)), bb2d817, 2026-11-11 09:03:11 UTC (`merged_at`; the merge commit is dated 09:03:10), by merge commit (parents 2d22b69 and 2e6e5d2), on Mustafa's go, given in the status-diet day session's chat (session_0173vVo6Vtm2C54toWvTt8sR) at about 09:02 UTC (his words, from the merge commit's message: `docs/DECISIONS.md`, 2026-11-11), after the advisor's APPROVE of 2e6e5d2 (PR comment 6779064873); CI green on main (push run 42991918886, on bb2d817); report `docs/reports/2026-11-10-status-diet.md` | done: the review round (two PASS WITH NOTES), the fix pass and the report done (its row in STATUS until the merge); then the advisor's APPROVE of the final head 2e6e5d2 and his go; its merge recorded by foundation 2.1a (PR #50), as his go says ("The next change-set records it") | none |
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

_Moved from STATUS (2026-11-25, ci-after-3a's docs pass, under his OLD-LINES of 2026-11-18): STATUS's rows for two merged pull requests, moved by script, byte for byte, so their cells read as they stood when they left STATUS. Both merged on 2026-11-25 (read with `git log --first-parent` on main): #69 as 311c048, #67 as c1da1bd._

| PR | Work | Branch | State | Waiting on |
|---|---|---|---|---|
| #67 | ADR 0006's small revision after the preview hosting check, by lane 2 (the chat he calls Builder V2), on his ONE-OFF-HEADER (a) and STEP4-COUNT (b): the header mark (PM-0006-2) judged on production's own addresses, with the reason in the row, and production's one-off links with the host's header recorded, not judged; part 1 weighs Workers static assets in a short paragraph from Cloudflare's docs read 2026-11-25 (nothing measured on Workers; in the draft, Pages stays); the build-count question said plainly to be not answered; the pass words of PM-0006-7 and checklist step 4 re-stated as the advisor agent's reading of his STEP4-COUNT (b), never as his decision; part 1's "no cap on previews" dated; report `docs/reports/2026-11-25-adr0006-header-row.md` | `claude/optimistic-albattani-mdoonz` (stands in for `cs/adr0006-header-row`) | a draft until the advisor agent approves its final head, then ready; risk tier 1 (an ADR draft, and his message of 2026-11-25, 11:07:07 UTC in `docs/DECISIONS.md`) | the advisor agent's approval of the head that takes main's #68 merge in, then his go for it (his answers of 2026-11-25, 14:55 Amsterdam, in `docs/DECISIONS.md`: PASS-WORDS (a) with his condition, step 4 achieved (recorded, the question is not answered), WORKERS-NEXT (a)) |
| #69 | guard-env-fix (SEC-U7B-1: the day guard dropped env, nice and stdbuf option words but not their values, so git and gh behind them went unchecked in a day session or subagent; routines unchanged), by the chat he calls the Guard fix chat | `claude/confident-gates-tbnq38` (stands in for `cs/guard-env-fix`) | draft; risk tier 2, needs-mustafa; code head c3827f4; guard tests 1247/1247; security-auditor APPROVE in round 2; the verifier PASS WITH NOTES | the advisor agent's review of the final head; his go for that head (a day session) |

_And STATUS's "Waiting on Mustafa" bullet for change-set 3b's merge (#68, merged 2026-11-25 as 3ab9eae), moved by the same docs pass and rule, byte for byte:_

- Change-set 3b: his go for the merge of exactly its last commit (he gets the full id after the advisor agent's approval
  and CI green; his main Advisor opens the pull request; he taps the merge). His go accepts as still open:
  3B-LATE-BOUND, OVERFLOW-PANIC-RULE (due with 3b's code in `room`), and his two press limits when due (3B-PACER-255,
  3B-OUTAGE-256).

_Moved from STATUS (2026-11-25, change-set 4a's docs pass, under his OLD-LINES of 2026-11-18): STATUS's row for the merged ci-after-3a (PR #71, 07d673b, merged 2026-11-25 20:49:09 UTC by `merged_at`), moved by script, byte for byte, so its cells read as they stood when it left STATUS:_

| PR | Work | Branch | State | Waiting on |
|---|---|---|---|---|
| none yet (title and text in the notes, `pr-title.txt`, `pr-body.md`) | ci-after-3a, the small CI pull request after change-set 3a, by the chat he calls CI Builder: every crate's release tests (E1), the two vector generators' `--check` (3A-GEN-CHECK), `devtools`' own wasm32 build line (3A-RULES-STEP's CI half), and the rule tests in the runner image's Mozilla Firefox 160.0 (FIREFOX-TESTS (b), in part: the drawing tests wait, FF-DRAW); report `docs/reports/2026-11-25-ci-after-3a.md` | `claude/keen-tesla-v807cf` (stands in for `cs/ci-after-3a`) | risk tier 2; code head 832268e, gate, smoke and toolchain green on it (runs 45676587871, 45676587814); the head for approval to be given | the advisor agent's review of the final head; his main Advisor opens the draft; his go for that head; at the merge the unused Playwright Firefox zip is deleted (his answer of 2026-11-25, 00:07:29 UTC, point 3) |

_And STATUS's "Waiting on Mustafa" bullet for ci-after-3a (#71), moved by the same docs pass and rule, byte for byte:_

- ci-after-3a: his go for the head the advisor agent approves, CI green (his main Advisor opens the draft). Answered
  2026-11-25, 20:35 Amsterdam: RULES-ONLY-READ (a); TOKEN-WORDS (a), its home the next workflow text the CI chat asks
  him for (the hand-over step for Builder V2's previews); FF-DRAW-WHEN (a), with the gate step 2.3, its own question
  (`docs/DECISIONS.md`; the report `docs/reports/2026-11-25-ci-after-3a.md`).

_Moved from STATUS (2026-11-26, ci-preview-handover's docs pass, under his OLD-LINES of 2026-11-18): STATUS's row for the merged change-set 4a (PR #70, 2483e9a, merged 2026-11-26 12:22:01 UTC by `merged_at`, read with gh's REST route), moved by script, byte for byte, so its cells read as they stood when it left STATUS:_
_Moved from STATUS (2026-11-26, guard-wrappers' docs pass, under his OLD-LINES of 2026-11-18): STATUS's row for the merged change-set 4a (PR #70, 2483e9a, merged 2026-11-26 12:22:01 UTC by `merged_at`, read with gh api), moved by script, byte for byte, so its cells read as they stood when it left STATUS:_

| PR | Work | Branch | State | Waiting on |
|---|---|---|---|---|
| #70 | change-set 4a, "The recording" (the first of Walk's four parts, his WALK-CUT (a)), by lane 1 (the chat he calls Builder V4): a recorded match replays to the same hashes, tick by tick, natively (one process, two processes, two matches in turn) and in WebAssembly on the players' file (Node and Chromium), and each planted divergence is caught at its tick with the field that differs; the one new number on the players' file is the shared state's hash (`Sim::shared_hash`), read only by a replay | `claude/cool-pasteur-oiqk1r` (stands in for `cs/4a-recording`) | risk tier 1; no browser check (nothing a player sees changes); five reviewers in two rounds (netcode-architect, rust-reviewer, test-engineer, engine-architect, security-auditor), all APPROVE in round 2, nothing Critical, High or Medium open; source final at fb5d0ff; the local checks, CI (on 21c1046, six of six) and the verifier (PASS) on it; 3,300 written lines against the stop line of 3,400 (3,050 before his STOP-4A (a), 2026-11-25, 20:48:11 UTC); report `docs/reports/2026-11-25-cs4a-recording.md`; ready for review once the advisor agent has approved its last head | the advisor agent's review of the last head; his Advisor's merge check; his go for that head |

_Moved from STATUS (2026-11-26, change-set 4b-1's docs pass, under his OLD-LINES of 2026-11-18): STATUS's rows for the merged ci-preview-handover (PR #74, merge commit 3b949c1, committed 2026-11-26 15:24:22 UTC) and guard-wrappers (PR #73, merge commit 4a62873, committed 2026-11-26 14:56:33 UTC; both times are the merge commits' own, read with `git log` on origin/main, not `merged_at`), moved by script, byte for byte, so their cells read as they stood when they left STATUS:_

| PR | Work | Branch | State | Waiting on |
|---|---|---|---|---|
| none yet (title and text in the notes) | ci-preview-handover, by the chat he calls CI Builder (his block of 2026-11-26, 13:20 Amsterdam; lane 2's request on issue #59): at the end of the `toolchain` job, on pull requests from this repository only, a check that the tested player build holds only plain files and folders with no name that begins with a dot, then the hand-over of that build to lane 2's preview workflow as the artifact `preview-site`, kept 7 days (`actions/upload-artifact` v7.0.2 at cf430e0, his UPLOAD-SOURCE (a)); TOKEN-WORDS and two comments made true; the recording's replay test joins the rule tests in Firefox (FIREFOX-REPLAY) | `claude/keen-tesla-v807cf` (stands in for `cs/preview-handover`) | risk tier 2; no browser check; written at d12e3a8 on his CI-WRITE-3 (a), GUARD-STEP (a) and HIDDEN-FILES (b) (2026-11-26, 15:05 Amsterdam), both files at his sha256 values; one review round before his yes (security-auditor: nothing stops the change, its SA-1 Medium is the accepted CUT-ESCAPE; infra-engineer: APPROVE); in the container Firefox 161 3 passed (the replay test included), Chromium 17 passed, the check on the real build and on seven planted faults; the upload step has never run, and no preview starts until lane 2's `preview.yml` is on main; CI: the push run on d12e3a8 green (the new steps skipped on a push; Firefox 3 passed, the replay test among them); the pull request run to read; report `docs/reports/2026-11-26-ci-preview-handover.md` | his Advisor opens the draft and watches the first run; the advisor agent's review of the final head; his go for that head |
| none yet | guard-wrappers (his SAME-GAP (a): nothing in front of git or gh gets through the day guard unread), by the chat he calls the Guard fix chat; what it does: "Phase", "The day guard" | `claude/confident-gates-tbnq38` (stands in for `cs/guard-wrappers`) | risk tier 2 (his go; no pull request yet); code head 83bc2417; main 2483e9a merged in (7be0a933) in a separate worktree; guard tests 1551/1551; four security-auditor rounds (the fourth by his leave, the last), R4-1 left open by his R4-END (1); 166 added lines of code and tests (stop line 200); report `docs/reports/2026-11-26-guard-wrappers.md` | the advisor agent's review of the pushed head; his Advisor's merge check; his go for that full head id |

_And, by the same docs pass and rule, byte for byte: the two "Last updated" lines those two pull requests' docs passes wrote at STATUS's top, and STATUS's "Waiting on Mustafa" bullets for the two:_

_Last updated: 2026-11-26 (ci-preview-handover's docs pass, by the CI Builder's docs-keeper, on main 2483e9a: the preview hand-over in flight; the merged #70's row and the follow-ups bullets of the merged #69 and #71 moved to the archive by script; in lane 1's lines only #70's merge and FIREFOX-REPLAY's answer; lane 2's lines left as they are)._
_Last updated: 2026-11-26 (guard-wrappers' docs pass, on the lane branch with main 2483e9a merged in: guard-wrappers waits for his go with R4-1 open; #70 merged; #70's row and the follow-ups bullets of the merged #67, #68, #70 and #71 moved to the archive by script; lane 2's #66 and #67 shown as merged where STATUS still had them waiting)._

- ci-preview-handover (CI Builder): his go for the head the advisor agent approves, after his Advisor has opened the draft and watched the first run (the push run on d12e3a8 green; the pull request run is the first to run the two new steps); a red on the new steps or on Firefox's replay test comes to him first; if the run fails at its start because the action is not allowed, the chat tells him which setting to change (his 15:05 points 5 and 7).
- guard-wrappers: his go for the head id the advisor agent approves, after his Advisor's merge check (no browser check).

_And lane 1's own lines that change-set 4b-1's docs pass replaced in STATUS (lane 1 wrote them), moved by the same script, byte for byte, so their words stay on record: "Next" item 2 as it stood, the "Waiting on Mustafa" bullet from 4a's report (its answered items now point at `docs/DECISIONS.md`), and the bullet for RELEASE-COUNT (decided on 2026-11-25; its condition 3B-RELEASE-DRAFTS stays open, listed in the archive's "Follow-ups", the bullet "From change-set 3b", which STATUS's "Merged pull requests' bullets" points at):_

2. v2: the netcode proof test (#40); step 2.3's chat stays stopped. 3b (#68), the small CI pull request (#71) and 4a (#70) merged.
   **No change-set 3c:** his ORDER-3C (c), 2026-11-25, 16:59:52 UTC (`docs/DECISIONS.md`): "Split 3c at its seam." The
   recording is change-set 4a (#70); "The link simulator follows Walk as its own small change-set and is merged before
   change-set 7 starts." Walk is four change-sets (his WALK-CUT (a)): 4a the recording, 4b content and the world, 4c the
   first soldier (he plays; an early look inside it), 4d sights; then the link simulator, then 7. Change-set 4's plan,
   section 8 (where to read it: 4a's report), places each 3c item by name. The first builder chat stays paused; Slice 3 waits for #40 and picks how commands are timed.

- From 4a's report (#70, merged 2026-11-26, 2483e9a): EARLY-LOOK-ORDER (his go's point 4: "I decide then"); before 4b's first edit
  CONTENT-SOURCE and DEPART-4; FIREFOX-REPLAY (it widens the Firefox entry he cut to the rule tests; answered by his block of 2026-11-26, 13:20 Amsterdam; carried by ci-preview-handover); LINKS-CHECK
  (4A-S3-READ). STEP-60 (a): his Export at 30 steps before 4b's step 6, then DIFF-LIST. Later: MOVE-RULES, NO-SOURCE,
  RESUME-FLASH, TUNE-HOME, SPRINT-DELAY-4; the two `docs/UNITS.md` §0 lines the Hornet's entry clashes with (his unit
  walk).
- **RELEASE-COUNT, decided by him on 2026-11-25: (a), accept as built** (`docs/DECISIONS.md`, 11:38:37 UTC), with
  two conditions: no report calls any part of the presses mark passed; and the two drafts are made to agree at
  their next revision, which comes to him before it is written (**3B-RELEASE-DRAFTS**, his).

_Moved from STATUS (2026-11-26, change-set 4b-1's docs pass, second part, to bring STATUS toward his 120 lines): lane 1's own lines (lane 1 wrote them), moved by script, byte for byte, where STATUS now has short forms: its first part's "Last updated" line, 4b-1's in-flight row as it stood before the reviews, "Next" item 2, the "Waiting on Mustafa" bullets with the long form of the builder's questions SIGHT-FLAG and DUR-1 and what he answered on 2026-11-26, the "From change-set 4b-1" follow-ups bullet and the "v2" open-risks bullet as they stood:_

| PR | Work | Branch | State | Waiting on |
|---|---|---|---|---|
| none yet | change-set 4b-1, "Content" (the first of 4b's two pull requests, his SIZE-4B (b)), by lane 1 (the chat he calls Builder V4): the content crate `crates/content` with no dependency (one reader for a cut grammar of JSON, the canonical writer, the bundle, the movement kind, the map kind with its own exact rules and the aim's reach, the time unit's conversion), the simulation's time unit, the reader's proofs and a bounded fuzz run natively, under Node and in the browsers, the old tuning's numbers and durations extracted by script, the time unit's rounding report, and the links check (his LINKS-CHECK (c)); 6,591 written lines against the stop line 7,450 | `claude/cool-pasteur-oiqk1r` (stands in for `cs/4b-content`) | risk tier 1, no browser check; built (code head ef2edec, main 3b949c1 merged in), in review round 1 (rust-reviewer, engine-architect, security-auditor; test-engineer follows) | the reviews and their fix pass, the verifier; the advisor agent's verdict on the final head; then his go for that head |

_Last updated: 2026-11-26 (change-set 4b-1's docs pass, first part, by lane 1's docs-keeper, on its branch with main 3b949c1 merged in: 4b-1 in flight, in review round 1; VISION-RULE's follow-up and pointers; 4b-1's follow-ups and the builder's two questions; the merged #73's and #74's rows, waiting bullets and top lines moved to the archive by script under OLD-LINES, with lane 1's replaced lines; the reviews' outcomes come in the main session's own pass)._

2. v2: the netcode proof test (#40); step 2.3's chat stays stopped. Walk (his WALK-CUT (a); no 3c: his ORDER-3C (c),
   `docs/DECISIONS.md`): 4a merged (#70); **4b-1 in review**; 4b-2 (steps 6 to 12: the collision library and world, his
   Export's data, the page's scene; after a short re-read by the advisor agent); 4c the first soldier; 4d sights; then
   the link simulator, before change-set 7. The first builder chat stays paused; Slice 3 waits for #40.

- Change-set 4b-1 (no browser check): his go for the head the advisor agent approves; and the builder's two questions,
  in its report. **SIGHT-FLAG**: should 4b already give each map object a "blocks sight" flag? The builder and
  engine-architect recommend not in 4b: it comes in Slice 2 with the sight check. **DUR-1**: which fire rate counts as
  the old game's? The old code never carried a timer's leftover time, so the Shabzon fired about 720 shots a minute at
  60 frames a second where its table value of 0.072 s gives 833; it binds change-set 8; game-designer's read
  (`.checkpoints/cs4b-content/game-designer-durations.md`, gone from the tree at the pull request's close, readable at
  the branch's commits) recommends the table's value now, tuned by play.
- Answered on 2026-11-26 (`docs/DECISIONS.md`, 12:21:17, 13:05:03 and 14:09:13 UTC): CONTENT-SOURCE, DEPART-4,
  EARLY-LOOK-ORDER, LINKS-CHECK, SOURCE-GO, SIZE-4B, FEATURE-COMPARE; FIREFOX-REPLAY (carried by #74). STEP-60: his
  Export at 30 steps is in (15:07:24 UTC; saved whole in the chat's notes); DIFF-LIST is asked at 4b-2's data step
  (step 8). Later, from 4a's report: MOVE-RULES, NO-SOURCE, RESUME-FLASH, TUNE-HOME, SPRINT-DELAY-4; the two
  `docs/UNITS.md` §0 lines the Hornet's entry clashes with (his unit walk).

- **From change-set 4b-1** (in review; the archive's block "From change-set 4b-1"; the 4B- ids are its docs pass's):
  4B-AIM-REACH: the loader refuses a playable map beyond the aim's reach (x and z plus or minus 327.67 m, y 0 to
  655.35 m). The aim widens in the first change-set whose map leaves that square (none in Slice 1; Slice 2's plan says
  whether its maps do), at the latest in Slice 3's change-set that sets the command codec for online play (PM-0001-51),
  beside 3B-ACKS and 3B-OUTAGE-256. No numbered change-set holds it yet. ADR 0002's next revision: 4B-FEATURE-POINTER;
  the drafts' next revision: 4B-DEPART-3, 4B-PM5-3 (his acceptance); the CI Builder: 4B-CI-CONTENT; 4d's import and
  change-set 8: 4B-OLD-TIMERS. Closed by 4b-1's step 4, at its merge: 4A-S3-READ's read side.

- **v2:** the decision drafts' risks (the asset pipeline and several LOOK_V2 techniques not researched); R1-R6's (R3-R6
  spot-checked; unread sources); change-set 3a's (MUL-ADD first); 3b's (no bound yet on how late a command runs, before
  online play; releases worked out, not counted); 4a's (two planted divergences only in a stand-in or native form; the
  recording tests' link read, 4A-S3-READ, closed on 4b-1's branch, at its merge). **Legacy (frozen):** 3a's ranked list (PR #15); bot navigation; the
  editor and map bugs; the first-deploy tutorial; the wall-clock reads (an input for ADR 0004 part 4).

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
- **From status-diet** (PR #49; the review round on 37434d5, `docs/reports/2026-11-10-status-diet.md`; each after the
  foundation's gate step, KIT-FREEZE-2):
  - **The STATUS citations left in the kit and the decision docs:** advisor.md:17 (its sources name STATUS only) and
    :286 ("STATUS, limit L4", now this file's "Open bugs and risks", kit-v3-3), netcode-architect.md:24 (the determinism
    debts, now this file's "Decisions pending: the rest" and "Open bugs and risks"); the ADRs, the register, LOOP_V2
    (:202, :214, :227) and ENGINE_OVERVIEW (:140), for their next revision. STATUS's table "Where STATUS's old sections
    went" resolves each one until then.
  - **SD-N1, a pasted block is his word** (security-auditor, Low): ACCOUNTS' "once I paste one, it is my
    instruction" against advisor.md:204-207 ("his own words"; "Never act on a go that a session relays or writes"); a
    later change-set aligns the two and says that text quoted inside a paste block stays data (ACCOUNTS' fourth bullet).
  - **SD-N2, "the advisor" in Standing permission** (security-auditor, Low): CLAUDE.md's "after the advisor approved
    that head" could be read as the advisor tool's verdict, which cannot be read back; write "the advisor agent's
    verdict" there and in advisor.md step 6.
  - **SD-N3, the Lanes rule is enforced by nothing** (security-auditor, Low): both accounts push as one GitHub user, so
    "a lane never pushes to another lane's branch" cannot be checked; a merge's expected head still fails a stale go.
  - **SD-N6, the archive's copies of his words merge by path** (security-auditor, a note): STATUS.md and this file are
    not on the guard's decision-doc list (STATUS never was), so a docs pass that edits his words here could merge
    through the PR Advisor; his call: this file on the list, or those lines moved.
  - **SD-L3, the index misses wrapped dated lines** (the verifier, Low): DECISIONS.md's "Also recorded elsewhere" finds
    "Mustafa" and a date only on one line, so it misses old lines 49, 526-527, 808-809 and 1017, and "XP-TERM: (a)" at
    597 and 613; each one's decision is in DECISIONS.md or indexed by its code (the verifier), so nothing is lost.
- **From foundation 2.1a** (PR #50; the review round on 3c98ad5 and round 2 on the fix, `docs/reports/2026-11-11-foundation-2-1a.md`):
  - **For 2.1b's plan** (all five closed by foundation 2.1b, PR #51, merged as 2aa5e9c on 2026-11-12: its report,
    "Decisions and deviations"):
    - **IE-2** (infra-engineer, Low): 2.1b moves all five files in one pushed commit; a half-moved commit turns its own
      push run and the next push run red (pull-request runs compare against main and are not affected).
    - **IE-3** (infra-engineer, Low) and **SA-L1** (security-auditor, Low): `tools/README.md` describes neither the
      build-folder rule (only the comments in `tools/gate.mjs` and `tools/smoke.mjs` do) nor that letter-case variants
      are judged differently on Linux CI (the root is judged) and on Windows or macOS (exit 2, by reading the code).
    - **IE-4** (infra-engineer, Low): the pinned Chromium 1194 has never run the `legacy/` layout (locally, 1243 ran under
      its name); 2.1b's proof that the smoke passes on `legacy/` comes from CI's run.
    - **IE-7** (infra-engineer, round 2, note): 2.1b merges with its pull-request run on the current main (pull-request
      runs do not re-run when main moves; main's ruleset does not require the branch to be up to date); read that run's
      gate header first: "(files at the root) | head = working tree (files in legacy/)".
  - **For the gate step (2.3), after KIT-FREEZE-2:**
    - **IE-1** (infra-engineer, Low; Mustafa, 2026-11-11: "Its wording Low becomes a follow-up for 2.3"):
      `tools/smoke.mjs:31` says "keep one copy" also when the files are split between the two places, not copied.
    - **TE-2** (test-engineer, Low): a base with all five files in two folders resolves to `legacy/` silently (the order
      of `BASE_PREFIXES`), where a working tree in that state exits 2; no commit in the history holds two full sets.
    - **A-1** (the advisor agent, Low): with a base in the old `shamble-frontlines-repo/` folder and a head at the root,
      the move check looks only in that old folder, so pages left in `legacy/` would go unchecked; old bases only.
    - **N2** (security-auditor, note): both folder-containment checks compare paths as text and do not resolve symlinks,
      as on main; `src="sub/%2e%2e/x.js"` stays inside the folder once decoded, so the smoke's outside check never sees
      it, while the gate reads a literal folder named `%2e%2e`.
    - **L-4** (the advisor agent, Low): CLAUDE.md's setup line `npx --no -- playwright install chromium` is refused in a
      cloud day session by the settings' `npx` deny; 2.1a's local smoke ran on Chromium 1243 aliased as 1194 instead.
    - **The v2 root page against pre-move bases** (a known limit): once v2 has a root `index.html`, any gate run against
      a base from before the move stops on the move check, including the E3 and rows scripts with their base 44446a5.
    - **IE-5** (infra-engineer, round 2, Medium): every CI run depends on live mirrors: the apt mirror on every smoke run
      (the step "Install Chromium's system libraries"), the npm registry on every run (`setup-node` without `cache:`),
      Playwright's download server on a cache miss; a failed attempt costs 10 to 15 minutes (3f29cb8 and 8d8c422,
      2026-11-11: the runner could not resolve the Ubuntu mirror). For 2.3: a short step timeout with one retry on the
      library install, `cache: npm`, or an image with the libraries in it (kit edits: his ask first).
    - **TE-7** (test-engineer, round 2, Low): the rows cannot fail if `editor\.html` or `shamble-.*\.js` were dropped from
      the move check's pattern (`tools/gate.mjs:247`); add a row with `editor.html` alone and one with a letter-case
      variant of an engine file (its own probes covered both names).
    - **R2-1** (security-auditor, round 2, Low): a name with a trailing dot or space (`index.html.`) passes the move check;
      Git for Windows refuses such paths at checkout (`core.protectNTFS`, on by default there); the suggested pattern:
      `/^(index\.html|editor\.html|shamble-.*\.js)[. ]*$/i` (a gate file: his ask first).
  - **For the drafts' next revision:** five "before the foundation" lines with no pointer to his decision of 2026-11-10
    (docs-keeper's note; the brief named only the lines it pointed): ADR 0002 :38, :724, :1129 (:1115 when 2.1a
    counted); SPIKE_PLAN :106 (the diagram's label) and :214 (:100 and :202 before this PR's pointers); also ADR 0003
    :51-52 and ADR 0006 :56-57, each wrapped across two lines; and in the other drafts: ADR 0001 :603, :1100, :1301,
    :1771; ADR 0003 :1373; ADR 0004 :1245, :1251; ADR 0005 :92; ADR 0006 :1469, :1482; the register
    (`docs/adr/README.md`) :72, :292. Nor to his decision of 2026-11-13 on step 2.3 (`docs/DECISIONS.md`, the entry
    "Taken by Mustafa, 2026-11-13, 18:47:55 UTC: his decision on step 2.3 (step 2.3 may be planned now), and step 2.3's
    task"), whose pointers stand beside 2.1a's (2026-11-13, foundation 2.3's first docs pass).
  - **Docs passes:** stale `tools/` citations in dated research docs, left as dated records: `docs/LEGACY_INVENTORY.md`
    TL1, TL6 and TL9; `docs/research/r5-toolchain-quality.md` :925 and :1737.
- **From foundation 2.2** (PR #48, merged as 4d21c1a on 2026-11-13: the report
  `docs/reports/2026-11-12-foundation-2-2.md`, its "Findings for
  2.3" table and "Follow-ups for later steps" list as review round 1's and round 2's fix passes left them (1de98a9 to
  8a4f9b0, then round 2's fix 790ac31, the report's final pass bd44a68 and the last commit before ready), and the
  advisor agent's ruling D1 of 2026-11-13 and its ruling D1-R2; his words: the report's Appendix A and
  `docs/DECISIONS.md`. The codes: S1 to S5 are the checks of the Fable advisor's early read of the built skeleton
  (through the advisor tool, 2026-11-12); K1 is the write of the new CI job, K1b its release-test line; M2 and M7 are
  planted failures and E12 a local check in the report's evidence; D1 to D17 are review round 1's findings as the
  session deduped them (head 4d78ad6, 2026-11-12 and 13; the report's "Reviews"), by rust-reviewer (RR-*),
  infra-engineer (IE-*), security-auditor (SA-*), engine-architect (EA-*) and test-engineer (TE-*), each then tested by
  a skeptic of its own; R2-D1 to R2-D4 and R2-L1 to R2-L8 are review round 2's items as the session deduped them (head
  4bccf55, 2026-11-13; the report's "Reviews"), from rust-reviewer (RR2-*), security-auditor (SA2-*), test-engineer
  (TE2-*) and engine-architect (EA2-*), R2-D1 to R2-D4 each tested by a skeptic of its own; route A is the root overflow
  setting the ruling D1 approved; PM-0002-* are ADR 0002's pass marks, OQ-0002-* its open questions, G0, G2, G3, G5, G9
  and G12 checks of its gate v2. Checked against the repository by docs-keeper [verified]: on 2026-11-12, GIT-C
  (`.claude/CHECKPOINTS.md:221`), ADVISOR-TOOL-WORDS and MODELS-2026-11-12 (their "Queued" items), G0 (PM-0002-39 and
  the register row), TSCONFIG's stubs and CI line, SAVER-SKIP-CI, PACKAGE-ENTRIES and BRIEF-STALE; on 2026-11-13, every
  repository line that the new and reworded items cite, in `toolchain.yml`, `gate.yml`, `Cargo.toml`, the root
  `package.json`, ADR 0002, CLAUDE.md, `.claude/agents/advisor.md` and the guard; and, read by the sync's fact check,
  2.2's plan, Part 3 (EMPTY-SLOTS) and the report's E12 run (its two result files). SETUP-NODE-CACHE is [verified] from
  setup-node's own source at the pinned SHA: security-auditor read it (SA-3), and its skeptic ran the shipped detection
  code (D6). The round-1 and round-2 items rest on the reviewers' and skeptics' notes (`review-r1-findings.json`,
  `review-r1-verdicts.json`, `skeptic-r1-D*.md`, `review-r2-findings.md`, `review-r2-verdicts.json`,
  `skeptic-r2-R2-D*.md`, the reviewers' `*-review-r2.md` and the advisor agent's `advisor-ruling-d1-r2.md` in
  `.checkpoints/foundation-toolchain/`, readable at the branch's last checkpoint commit,
  2f0fef3b208c10d5c38b921d24fb5f90f71b7ecb): their CI runs, API reads, source reads and scratch runs are theirs, not
  re-run by docs-keeper. Carried from the session's list and the report [assumed]: AUTO-ASK's times, MANUAL-MODE's
  documentation page and mode name, REFUSED-PUSH, PIN-GUARD's marks, SETUP-NODE-CACHE's runs, M2's and M7's runs,
  NODE-26's dates):
  - **For the gate step (2.3), when the kit unfreezes:**
    - **AUTO-ASK** (his KIT-ASK-SEEN (b): `docs/DECISIONS.md`, 2026-11-11, 00:07:48 UTC; the report's Appendix A5):
      auto mode can pass the guard's ask. K1 (2026-11-10 20:02:42 UTC): the guard's ask was answered "allow" six
      seconds later, the session log not saying by whom. Step 5's five kit writes (2026-11-12), made in the mode he
      chose as Accept edits: his count, five prompts of five (`docs/DECISIONS.md`, 2026-11-12, 16:31:45 UTC; Appendix
      A11).
    - **MANUAL-MODE**: CLAUDE.md's change-set protocol, step 3, says to make a `.claude/` edit "in Accept edits or
      Manual mode", but the cloud app offers Accept edits, Plan or Auto, with no Manual mode (the environment's
      documentation page on permissions, read 2026-11-12), and the session record called the mode he chose "default".
      _Since (#64, 2026-11-23):_ the same again on his messages of 17:22:42, 19:58:23 and 20:47:59 UTC (the record
      `default`, his screen Accept edits), which his mode rule of 17:15:07 UTC covers (`docs/DECISIONS.md`); the block
      for #64 below.
    - **GIT-C**: `.claude/CHECKPOINTS.md` §10 (:221) tells a session to run `git -C <path> status --short --ignored`,
      which the guard refuses ("no option before the subcommand"); sessions `cd` into the folder instead. The 2.1b
      report lists the same refusal in its "Findings for 2.3" [verified 2026-11-12].
    - **ADVISOR-TOOL-WORDS**: CLAUDE.md takes his words of 2026-11-11 on the advisor tool at 2.3 (`docs/DECISIONS.md`,
      "Queued": the CLAUDE.md clarification of the advisor tool).
    - **MODELS-2026-11-12**: CLAUDE.md's "Models and effort" section and the agent files change to match his words of
      2026-11-12, 16:25, on models, each edit asked of him in chat with its exact text (`docs/DECISIONS.md`, 2026-11-12,
      15:35:17 UTC, item 3, and its "Queued" item; Appendix A9).
    - **REFUSED-PUSH** (`docs/DECISIONS.md`, 2026-11-12, 16:28:28 UTC, item 4; Appendix A10): the first push of step 5's
      merge, written as a compound line (`git push -u origin <branch> 2>&1 | tail -4 && date ...`), was refused by the
      session's permission check with no reason given; the guard's own push rule allows that push; the plain `git push
      origin <branch>` then passed.
    - **PIN-GUARD** (his answer of 2026-11-10: `docs/DECISIONS.md`, 2026-11-10, 17:23:21 UTC; Appendix A1): the guard
      does not protect the new pins (`rust-toolchain.toml`, `Cargo.toml`, `Cargo.lock`, `package.json`,
      `package-lock.json`): `protectedRel` is false for them, so routines may write them on a branch and day sessions
      get no ask.
    - **G0** (the register row Q2.2-a in `docs/adr/README.md`: the advisor agent's ruling, final; his go for the row,
      asked in the report's "Decisions needed" (ADR-INDEX), came with PR #48's merge (his ADR-INDEX (a), 2026-11-13:
      `docs/DECISIONS.md`); reworded on D8, infra-engineer's IE-4, and on D17's pin-check half, test-engineer's TE-3):
      G0's exit-2 mark moves to step 2.3, where 2.3's G0 is to prove "deleting the toolchain file fails G0 with exit 2"
      under `--prove` (PM-0002-39); until then the toolchain job fails closed without the file (M2: red at the pin check
      with exit 2, no cargo step run). 2.3's pin check must keep this job's source checks, by what they do, since 2.3
      replaces `toolchain.yml`: `RUSTUP_TOOLCHAIN` unset, the active toolchain named by `rust-toolchain.toml` (`rustup
      show active-toolchain`), and `RUSTUP_AUTO_INSTALL=0`, so that only the explicit install brings a toolchain
      (`toolchain.yml:21-22` and :48-50 [verified 2026-11-13]). Why: the runner image's own stable Rust equals the pin
      today (1.99.1, by the image's software list, which infra-engineer and its skeptic read; no log prints the image
      stable's version), so comparing versions alone cannot show where the toolchain came from. Mutations beyond
      PM-0002-39's file deletion: `RUSTUP_TOOLCHAIN` set to the pin's own channel (it passes a version-only check and
      fails the source check on any image); the install step removed under `RUSTUP_AUTO_INSTALL=0` (G0 must then fail
      with exit 2 instead of installing; never run: 2.2's job would exit 1); and the check's other branches, never yet
      seen red: M2 stopped at the file
      read (:46), and in 2.2's job those branches exit 1 (:47-52 [verified 2026-11-13]), where G0 exits 2 on any setup
      failure (ADR 0002's G0 row). rustup does not self-update in CI (it turns self-update off when `CI` is set, since
      rustup 1.28.0, and GitHub sets `CI=true`), so the image's rustup is the one that runs; and `RUSTUP_AUTO_INSTALL=0`
      has never been seen working in a run, since no planted failure removed the install step (infra-engineer and its
      skeptic).
    - **OVERFLOW-XCRATE-CI** (Medium, the advisor agent's label; a STATUS follow-up; it replaces OVERFLOW-WASM. D1,
      rust-reviewer's RR-1, High, is fixed by route A, and the advisor agent's ruling D1, condition 5, split this proof
      out of it): the root `Cargo.toml` now sets `[profile.release] overflow-checks = true` and
      `[profile.release.package."*"] overflow-checks = false` (4bd9dd5; `Cargo.toml:25-29` [verified 2026-11-13]; the
      register row OVERFLOW-ALL, his go given with PR #48's merge: his OVERFLOW-ALL (a), 2026-11-13), so every workspace
      crate checks overflow in every profile. No CI step can yet show that the setting reaches sim's code compiled in
      another crate: K1b's release test and M7 (red at that test with `[profile.release]` removed) run inside sim's and
      protocol's own packages, so they cannot tell route A from the per-package blocks it replaced. That reach is shown
      locally only (the report's E12: with the old blocks, 1 of 5 of room's cross-crate overflow tests panicked under
      release, bench and an inheriting custom profile, and the wasm32 release build returned 0 in Node; with route A, 5
      of 5, and the wasm32 release build traps, as dev does). The fix: PM-0002-53's committed counter test (a planted
      counter that overflows panics in the four builds, dev and release, native and Wasm; the release builds are the
      ones that tell the two settings apart) runs from a workspace crate other than protocol that depends on sim (room
      or client-endpoint, say): part 4's two blocks check sim's and protocol's own packages, so a test in either passes
      under those blocks too (the R2-D4 skeptic's run: a counter test in protocol passed CI's release line under both
      settings). Its planted failure is Cargo.toml set back to part 4's two blocks, where the release run must go red;
      removing `[profile.release]`, as M7 did, turns the test red in any crate, protocol included. ADR 0002 places the
      test in `sim` (:1319 [verified 2026-11-13]); test-engineer, PM-0002-53's owner, rewords that at the next revision
      (the register row). Two things it needs that the tree lacks: the planted counter must be sim code other crates can
      call (a `#[cfg(test)]` item is invisible to them; part 4 makes real counters use `wrapping_*` or `checked_*`, so
      this `+` counter is test scaffolding); and a way to run Wasm: there is no runner, E12's raw `#[unsafe(no_mangle)]`
      export is refused by the workspace's `unsafe_code = "deny"` unless it takes part 4's written exception (an
      invariant, a test and an `#[allow]` with its reason), room and client-endpoint take no wasm-bindgen (part 3), and
      wasm-bindgen comes only after its by-day reading (EMPTY-SLOTS), so where the Wasm half lives is ADR 0002's owner's
      call, settled in that change's plan. CI runs only sim and protocol in release and no Wasm test, so both halves are
      CI edits, each asked of him then. Anchor: the first change that adds a dependency on sim, or step 2.3, whichever
      comes first; STATUS names the earlier trigger. If nothing depends on sim when 2.3 starts, 2.3's plan either adds
      the test with its own test-only dependency on sim, or asks the advisor agent to move the anchor to the first real
      one. 2.2's wasm32 build is dev-profile only (WASM-ARTIFACT). The overflow tests' doc comments in `sim` and
      `protocol` say `cargo test --release` "proves the release profile's setting", which M7 shows only for code
      compiled in their own crates (the wording R2-L1 fixed in the report); recorded only, reworded with that test (the
      Fable advisor's whole read, consultation 8).
    - **TSCONFIG** (S2): TypeScript is pinned but never type-checked: no `tsconfig`, and CI prints `tsc --version` only.
      ADR 0002 part 7 gives the shared strict base tsconfig and `tsc -b` to the lint step (G3). The stubs import
      `./index.ts`, a convention `tsc -b` would need `allowImportingTsExtensions` (with no emit) or
      `rewriteRelativeImportExtensions` for: 2.3 decides it.
    - **REQUIRED-CHECK** (D2: infra-engineer's IE-1, Medium, and security-auditor's SA-4; the report's Risks):
      `toolchain` is not a required check on main. Ruleset 25069670 requires `gate` and `smoke` only (infra-engineer
      read it without a token, 2026-11-12), main's guard checks only those two before a merge
      (`.claude/hooks/routine-guard.mjs:1486`), and CLAUDE.md's merge rule for a day session (:130) names only them, so
      until 2.3 a red `toolchain` stops such a merge only if the advisor or Mustafa looks at it, and it is no rollback
      trigger (CLAUDE.md:119). The PR Advisor routine waits for every check, but it cannot merge this kind of pull
      request (the report's E9); for #48 itself, the session read the two `toolchain` runs (push 43452386039,
      pull_request 43452396473) before it merged (merge commit 4d21c1a's message). Step 2.3 replaces `toolchain.yml`
      with the gate v2's workflow (the file's header says so), and ADR 0002 gives that workflow one aggregate required
      check (:642, PM-0002-57), which, by ADR 0002's foundation checklist (item 2), he names in main's branch protection
      after 2.3. With it, every kit line that names `gate` and `smoke` as the required checks changes, found by grep
      then, each edit asked of him: today CLAUDE.md:119 and :130, `.claude/agents/advisor.md` :157, :163, :224, :273,
      :294 and :323, and the guard's merge check (`routine-guard.mjs:1486`) with its comments and tests [verified
      2026-11-13].
      _Since (#64, 2026-11-23):_ `toolchain` now also holds the build players get, the boundary tests, the type checks
      and the 12 browser tests, and it is still not a required check, so a red `toolchain` blocks no merge by itself and
      is read by hand before every merge (infra-engineer, #64's review round 1; the report's Risks).
    - **SETUP-NODE-CACHE** (S4; reworded on D6, security-auditor's SA-3): setup-node v7.0.0 (pinned by SHA 8207627 in
      `toolchain.yml` and `gate.yml` [verified 2026-11-13]) turns automatic caching on when the root `package.json`
      names npm in its top-level `packageManager` field or in `devEngines.packageManager`. The root `package.json` has
      neither [verified 2026-11-13], so nothing is cached today (runs 43226692719 and 43282089533). Once either field
      names npm, setup-node would cache the whole `~/.npm` folder under one key built from the root lockfile and save it
      in its post step, after the cut, so code inside the cut could write into that folder before it is saved; `npm ci`
      re-checks every tarball's sha512, so the reach is a failed or slow install, not swapped code, and a pull request's
      run cannot write main's cache scope (security-auditor; its skeptic's poison tests on npm 10.9.9 and its reading of
      GitHub's caching reference). Fix at 2.3: `package-manager-cache: false` on every setup-node step of the workflows
      2.3 leaves, or a cache saved before the cut (CI edits: each asks him). Source: setup-node at 8207627,
      `action.yml:24` and `main.ts:144-173` [verified from setup-node's source: security-auditor read it; its skeptic
      fetched the pinned SHA and ran the shipped detection code on eight `package.json` variants].
    - **SAVER-SKIP-CI**: the checkpoint saver marks a push of `.checkpoints/` alone `[skip ci]`
      (`.claude/scripts/checkpoint.mjs:168`), so a ready head made by the saver would carry no checks; the last commit
      before ready (the notes' deletion) is made by hand.
    - **NODE-26** (the advisor agent's ruling on Q2.2-c, Node 22 for 2.2, 2026-11-10): `tools/` and v2 move to Node 26
      together in one change-set after 2.3, merged before 2027-03-31 (Node 22's end of life is 2027-04-30); the
      toolkit's `package.json` and lockfile are gate files, so it asks him.
    - From the Fable advisor's full read of the built skeleton (consultation 4, 2026-11-12, before the review round;
      the session's checks of that day, by its own commands [verified]):
    - **CRATE-NAMES**: four workspace crate names are taken on crates.io by other crates (`sim`, `protocol`, `room`,
      `devtools`; `client-endpoint` and `sim-wasm` are free): a dependency written without its `path`, or `cargo add
      sim`, would pull a stranger's crate; G5 (cargo-deny) can keep those names off the registry.
    - **CI-ENFORCED-LATER**: nothing in CI yet checks that a new crate opts into the workspace lint table, that a new
      crate has a test (cargo passes with no test, where npm fails a package without a test script), or that npm
      versions stay exact (2.2 checked that by a script in its notes).
    - **WASM-ARTIFACT**: `sim-wasm` is a plain library, so the wasm32 step compiles the crates but emits no `.wasm`
      file; its crate type and its `unsafe_code` exception come with the bindings; the wasm32 build is dev-profile only
      (it joins OVERFLOW-XCRATE-CI, whose counter test runs in wasm32 release).
      _Since (#64, 2026-11-23):_ `sim-wasm` is a `cdylib` and an `rlib` since change-set 2 (#61;
      `crates/sim-wasm/Cargo.toml:10`), and CI now builds the release Wasm through `packages/client/scripts/build.mjs`
      (`cargo build --locked --release --target wasm32-unknown-unknown -p sim-wasm`), twice, inside the cut, and runs
      the boundary tests on it; the cut's own wasm32 line of five crates stays dev-profile
      (`.github/workflows/toolchain.yml:160`) [verified 2026-11-23]. No boundary test checks overflow on the Wasm, so OVERFLOW-XCRATE-CI's Wasm half is still open
      [verified 2026-11-23: no overflow, panic or trap case in `packages/sim-worker/boundary/sim-wasm.test.mjs`].
    - **TWO-PLAYWRIGHTS**: two Playwright versions now live in the repository (the toolkit's 1.56.1, the root's
      1.65.0), so 2.3's browser cache needs a key per version; no real browser has run inside the cut's privilege drop.
      _Since (#64, 2026-11-23):_ the last clause no longer holds: a real browser has run inside the cut, the runner
      image's Google Chrome 157.0.8103.78, 12 tests, in eight clean jobs (four heads, push and pull request each); no
      browser is downloaded for them, so the two versions still need a key each only once a browser cache comes.
    - **TOOLKIT-RESOLVES-UP**: with the root installed and `tools/` installed without its optional packages (the gate
      job's form), `require('playwright')` from `tools/smoke.mjs` loads the root's Playwright 1.65.0 instead of
      reporting the toolkit's 1.56.1 missing; CI's smoke job installs `tools/` in full and never installs the root, and a
      machine set up per CLAUDE.md (`cd tools && npm ci`) finds the toolkit's own copy first. `tools/smoke.mjs` is a gate
      file, so a fix (for example checking that the resolved Playwright lies inside `tools/`) is 2.3's.
    - **CARGO-NET-OFFLINE**: `CARGO_NET_OFFLINE=true` inside the cut would give fast, clear failures once third-party
      crates exist (a CI-job edit, so it asks him).
    - **CRATE-LICENCE**: the crates carry no `license` field, which cargo-deny's licence check reads.
    - From review round 1 (head 4d78ad6, 2026-11-12 and 13; each finding tested by a skeptic of its own, all holding at
      Low; the report's "Reviews" and Risks):
    - **CUT-CARRY** (D7, infra-engineer's IE-3, with D17's probe half, test-engineer's TE-3): nothing yet carries the
      network cut into 2.3. `toolchain.yml`'s header says step 2.3 replaces the file with the gate v2's workflow (:3),
      ADR 0002 part 9 puts no cut around G9's tests (G12's outbound rule covers only the end-to-end run), and PM-0002-37
      is a one-run mark (ADR 0002 :578, :581 and :1303 [verified 2026-11-13]). So 2.3 keeps the cut around its build and
      test commands (what PM-0002-37 proved: the commands at `toolchain.yml:82-87`, from `cargo test --workspace
      --locked` to `npm test --workspaces` and `tsc --version`), or drops it by a recorded decision citing PM-0002-37's
      one-run mark. A kept cut takes SETPRIV-HARDEN and CUT-ESCAPE with it, and adds a red run with `--net` removed,
      ending in the probe's "the network is still reachable inside the cut" line (`toolchain.yml:78`), which no run has
      shown yet (test-engineer counted none in the 30 runs that had reached the cut by review round 1).
      CARGO-NET-OFFLINE and the last clause of TWO-PLAYWRIGHTS speak of the cut, so they depend on this choice
      (infra-engineer and its skeptic).
    - **CUT-ESCAPE** (D4, security-auditor's SA-1; the report's Risks): the cut proves that the tests run without the
      network; it is not a sandbox. Inside it the tests keep the runner's uid (1001) and share its filesystem and
      process table with what runs after the step, with the network, the docker group and passwordless sudo
      (`toolchain.yml:68-73`: a network namespace only, and `setpriv` back to the same uid [verified 2026-11-13]). So
      code that runs inside the cut (a compromised dev dependency, such as the native rolldown binary; lightningcss is
      installed there but loads only lazily) can reach past it. Examples of the class: files that later steps, post
      steps or the runner itself run or read, such as a `core.fsmonitor` hook in the workspace's `.git/config`, which
      actions/checkout's post step runs (shown locally with git 2.43.0 and in the source of git 2.56.0 (the runner's
      version on 2026-11-10; on 2026-11-12 the runners differed: 2.56.1 on c901187's pull_request run at 15:34 UTC and
      on a5823cf's push run at 18:39 UTC, 2.56.0 on c901187's push run and on fd14855's and 45f1bb5's runs up to 18:37
      UTC); not run on the runner), files in the runner's work folder, or the step's `GITHUB_PATH` and `GITHUB_ENV`
      files (these two by the runner's layout and documentation, not shown); host pathname sockets such as D-Bus or
      systemd-resolved (from documentation, untested on the runner); and signals to the runner's own processes
      (kill(2)). A real boundary needs a different uid with its own copy of the workspace, a container or VM, or a
      separate job whose output nothing trusted runs. Today the job has `contents: read` and no secrets, and
      `gate.yml`'s toolkit dependencies already run with the same reach (security-auditor).
      _Dated pointer, 2026-11-26 (ci-preview-handover's docs pass; his words of 2026-11-26, 15:05 Amsterdam, point 6,
      `docs/DECISIONS.md`: "CUT-ESCAPE stays on record as accepted."): on that pull request's branch a step after the
      cut uploads the build the cut's code wrote, for lane 2's preview workflow, which publishes it with a Cloudflare
      token (security-auditor's read on lane 2's branch at 9ded5a8); the acceptance now also rests on that workflow
      treating the build as data (PH-SA-2), and the later fix is
      CUT-ESCAPE-FIX (both in "From ci-preview-handover")._
    - **SETPRIV-HARDEN** (D5, security-auditor's SA-2): the cut's privilege drop (`setpriv --reuid="$(id -u)"
      --regid=nogroup --clear-groups --no-new-privs`, `toolchain.yml:69` [verified 2026-11-13]) has neither
      `--inh-caps=-all` (setpriv(1)'s own advice when dropping from root) nor `--bounding-set=-all`, and the step prints
      `id` and `ip -o link` but asserts none of the privilege properties (only the TCP probe judges the network, as the
      approved plan designed). Today the result is right: a local run as root (util-linux 2.39.3) gave effective,
      permitted and ambient sets 0, and the full bounding set is inert under no_new_privs. At 2.3: add `--inh-caps=-all
      --bounding-set=-all`; fail before the tests if `id -u` is 0 (the check that closes the root case, since it fails
      before the pull request's code runs); assert capability sets 0, NoNewPrivs 1 and groups `65534`; and check that no
      device but `lo` is up (a CI edit: it asks him).
    - **CANCEL-ON-MAIN** (D9, infra-engineer's IE-5): both workflows cancel in-progress runs in one group per ref, main
      included (`toolchain.yml:16-18` and `gate.yml:15-17` [verified 2026-11-13]; ADR 0002 part 9 keeps concurrency
      cancel), so two merges close together leave the first without a green check: main's gate run 39046286643 on
      04c086c was cancelled when df1e6d3's run started 5 s later, and its check-run annotation names a higher-priority
      waiting request in the same group (the skeptic's API read). With 2.3's `if: always()` aggregate job a cancelled
      main run gives a red verdict or none, and the Revert rule needs a red commit's first parent to be green
      (`.claude/agents/advisor.md:273-276`). At 2.3: main's runs must not share one group, or the group must keep more
      than one pending run (the skeptic's reading of GitHub's docs: the default queue keeps one pending run, so of three
      merges inside one run the middle one's run is cancelled; not tested); and planted failures wait for each run, a
      revert's included (four revert runs in this change-set were cancelled by the next plant, 2 to 4 s later). CI
      edits: each asks him.
      _Since (#64, 2026-11-23):_ the `toolchain` job now takes about 2.2 to 2.5 minutes (135 to 147 s per clean job), so
      two merges within that time leave the first without a `toolchain` result on main (infra-engineer,
      #64's review round 1; the report's Risks). The concurrency rule itself is unchanged.
    - **LINT-TABLE-PEDANTIC** (D10, rust-reviewer's RR-2): ADR 0002 part 7 asks both that every crate opt into the
      workspace lint table (`[lints] workspace = true`, itself a gate check) and that pedantic be at warn on `sim`,
      `protocol` and `content` only (:433-436 [verified 2026-11-13]); Cargo cannot hold both: cargo 1.99.1 refuses a
      member's own lint table beside `workspace = true`, and the whole workspace then stops loading (exit 101). Routes
      for 2.3's lint stage (G2): a crate-level `#![warn(clippy::pedantic)]` in those crates, repeating after it the
      table's levels for single pedantic lints (an allow or a deny); or the gate's own `cargo clippy ... --no-deps -- -W
      clippy::pedantic`, repeating them too (that flag overrides a lint the table allows and lowers a deny to warn, and
      without `--no-deps` it also lints the workspace crates the three depend on). Rewording part 7 is for the ADR's
      owner and needs his go. Today the head is pedantic-clean through the gate flag (`sim` and `protocol`, host and
      wasm32: the session's run and its skeptic's).
    - **FEATURE-UNIFY** (D11, rust-reviewer's RR-3): on the pinned 1.99.1, in edition 2024, a member's
      `default-features` overrides the `[workspace.dependencies]` entry without a warning (from 1.85 to 1.98 the
      combination was an error), and one cargo command unifies features across all the members it builds; so part 3's
      rule that the maths-relevant features of the dependencies `sim` shares with `server` are set once in
      `[workspace.dependencies]` (ADR 0002 :212-217 [verified 2026-11-13]) is not enforced by Cargo. Part 3's
      cross-build comparison of the resolved feature lists (`cargo tree -e features`, in G5) stays the guarantee. A
      manifest lint would catch it earlier, at its cause: for every crate in sim's tree, no member-level
      `default-features` or `features` keys in any dependency table, no non-inherited declarations of it, and no
      `name/feat` or `name?/feat` entries naming it in members' `[features]` tables; with the workspace entries written
      with an explicit `default-features = false` and feature list. It starts with the first third-party maths crate;
      such a lint widens G5 beyond PM-0002-44, so it would enter through 2.3's plan and the ADR's next revision (the
      skeptic).
    - **ROOT-WORKSPACE** (D12, engine-architect's EA-1): the root `[workspace]` (members `crates/*`, no `exclude`:
      `Cargo.toml:2-4` [verified 2026-11-13]) now captures every Cargo package below the root that is not a member and
      has no workspace of its own, such as a lone crate in `.gate/`, `.checkpoints/` or `spikes/`, or a crate nested
      inside a member: cargo refuses to build it as the current package (exit 101; the error names both fixes). Spikes
      keep their own `[workspace]`, as PR #40's spike and scratch crates do. Options for 2.3: `exclude = [".gate",
      ".checkpoints"]` in the root workspace (tested locally: still six members, `Cargo.lock` unchanged, and a lone
      crate in `spikes/` still needs its own table), and a note in CHECKPOINTS §4 that every such package carries its
      own `[workspace]` (a kit edit, after the freeze). For 2.4's plan: ADR 0002 part 5 puts `TS_RS_EXPORT_DIR` in a
      `.cargo/config.toml` but names no folder (:345-346 [verified 2026-11-13]); a root config's `[env]` would reach
      every nested workspace when cargo runs inside the tree, so the command line's `--config` is safer (tested
      locally).
    - **CLIENT-NET-RULE** (D16, engine-architect's EA-5, a proposal for 2.3's plan): part 3's rule list has no static
      rule for the page's side of the boundary: nothing says that `client` never imports `net` or the Wasm glue (ADR
      0002 :207-209 and :424 [verified 2026-11-13]). Part 5's rules that the page never instantiates Wasm and that the
      Worker owns the network connection are checked only at run time (PM-0002-2's and PM-0002-22's counters, which
      PM-0002-64 keeps in G12 from 2.5, in a 1-minute local match). 2.3's plan weighs a G3 `no-restricted-imports` entry
      for it, with a planted defect (PM-0002-42). Its limits: direct WebSocket, WebTransport or WebAssembly use in
      `client` (part 7's `no-restricted-globals` is the companion); imports through `sim-worker` (allow only the Worker
      entry, or check the built graph); and the glue's path, fixed at 2.4 or 2.5. Wording it into part 3 is for the
      ADR's owner (engine-architect) and needs his go.
  - **For later steps:**
    - **OVERFLOW-PER-CRATE** (S3; rewritten on the advisor agent's ruling D1, condition 6, and widened on its ruling
      D1-R2, on review round 2's R2-D1): workspace crates are covered by the root setting now
      (`[profile.release] overflow-checks = true`); `[profile.release.package."*"] overflow-checks = false` keeps every
      crate that is not a workspace member at the release default setting (a member's path dependency inside the
      repository joins the workspace and is checked, unless excluded; a `[patch]` path does not). It leaves a
      dependency's own arithmetic unchecked, concrete arithmetic inside its generic functions included; arithmetic its
      generic code does through a trait (`T: Add`, `Sum`) takes our crate's setting when our crate compiles it, but not
      when rustc reuses the dependency's own copy (an `#[inline(never)]` function the dependency already uses with the
      same types, at any opt-level; any shared generic it holds, at opt-level 0, 1, "s" or "z", where our own integer
      `sum()` and `product()`, and sim's code compiled in our crates, can then wrap too). The first third-party crate
      that any workspace crate uses decides, with its cost measured, between a named override for each such crate (a
      named package override beats `"*"`) and dropping `"*"` (which checks every dependency, the server's future network
      crates included, so security-auditor reads it then: SA2-2's point 6); while `"*"` stays, moving the release
      profile, or one inheriting it, to opt-level 0, 1, "s" or "z" reopens it (the register row OVERFLOW-ALL, his go
      given with PR #48's merge: his OVERFLOW-ALL (a), 2026-11-13). The runs: review round 2's, in the report's
      "Reviews" (rust-reviewer and the R2-D1 skeptic for the shared generics and both closing options, repeated by the
      advisor agent's ruling D1-R2; the R2-D1 skeptic and test-engineer for the dependency's generic code).
    - **OVERFLOW-PANIC-RULE** (review round 2, security-auditor's SA2-2; Medium, open; handed to the owners by the
      advisor agent's ruling D1-R2 and the register row OVERFLOW-ALL): with route A, an overflow in `room`'s own
      arithmetic panics in release instead of wrapping (local native runs; test builds already panic), and a future
      server crate's code would do the same; on the server's tick that would end the match (ADR 0004 part 20),
      repeatably if a client can drive the value. ADR 0004 part 20's first sentence already forbids any panic reachable
      from network input or content data, but its list of panic sources names no overflow (:562-566), and the other
      lines are worded for part 4's two-crate setting: ADR 0002 part 4's explicit operations name counters and tick
      numbers (:247-249); part 7 leaves out `arithmetic_side_effects`, with REC4 as what covers overflow (:471, :1617);
      PM-0004-60 and PM-0001-34 name no build setting. Handed to the owners to weigh, each at its ADR's next revision,
      any wording with his go (engine-architect: ADR 0002; rust-reviewer: part 20; test-engineer: the marks), with the
      security review's proposals: `wrapping_*` only where a wrap is meant, `checked_*` or `saturating_*` with a defined
      result for values a client or content can set or add up; part 20 names overflow, and room's and the server's tick
      code beside the sim; the fuzz builds keep overflow checks on; `arithmetic_side_effects` weighed for room's command
      handling. Anchor: the plan of the first change that gives `room` or a server crate code (2.5 by ADR 0002's
      migration plan) states the overflow rule its arithmetic follows.
    - **PACKAGE-ENTRIES** (S5): the packages have no `exports` or entry fields (the step that gives them code).
    - **EMPTY-SLOTS** (2.2's plan, Part 3, readable at the branch's last checkpoint commit, 2f0fef3; reworded on D14,
      engine-architect's EA-3): OQ-0002-7 and the wasm-bindgen pin: the pin comes with `sim-wasm`'s bindings, after the
      boundary spike fixes its exports; OQ-0002-7's decider, a by-day reading of wasm-bindgen's maintenance issue #4533
      and the guide's governance page (ADR 0002 :1082-1084 [verified 2026-11-13]), comes before the step that adds
      wasm-bindgen (2.2's plan, Part 3; by the D14 skeptic's reading, the foundation step that adds it to `crates/`,
      not the throwaway boundary spike, which adds it first). OQ-0002-3 (one Wasm instance or two), OQ-0002-4
      (isolation), OQ-0002-1 (the editor from `file://`) and the boundary spike's items 4 (generated types) and 8
      (sizes, wasm-opt, `skipLibCheck`, the three.js casts, Playwright's clock) wait for the boundary spike.
    - **OQ-0002-8-TEST** (D15, engine-architect's EA-4): the Advisor's answer (a) to OQ-0002-8 (2026-11-03, recorded in
      ADR 0002) keeps `room`, `client-endpoint` and `navbake` proposed and labelled not researched, and makes the first
      foundation milestone that builds them their test (:1095-1102 [verified 2026-11-13]). 2.2 builds `room` and
      `client-endpoint` empty (one stub test each; CI's wasm32 step compiles both) and exercises neither reason ADR 0002
      gives for them (`room`: the local server in the Worker, which cannot carry the server's async runtime and
      transport; `client-endpoint`: the endpoint's Rust code run natively and headless for ADR 0001's network marks), so
      the 2.2 report does not count 2.2 as OQ-0002-8 (a)'s test. The later marks that touch them: PM-0002-44 at 2.3 (G5
      enforces the rules behind those reasons: `tokio` added to `room`, wasm-bindgen or a transport crate added to
      `client-endpoint`; its row carries no phase, and `--prove` and G5 arrive at 2.3) and PM-0002-64 at 2.5 (their code
      runs, with ADR 0001's loopback milestone); `navbake` enters at slice 2. Which of these is the test is for the
      Advisor to say, or a dated reading from the advisor agent.
    - **BRIEF-STALE**: `docs/V2_PHASE0_BRIEF.md:20-21` (phase 0: no engine code but throwaway spikes) and `:144` (its
      "Don't" list: no engine code in phase 0) still forbid engine code in phase 0, with no word of the exception for
      foundation steps 2.1 and 2.2, nor of his decision of 2026-11-13 on step 2.3 (`docs/DECISIONS.md`, the entry "Taken
      by Mustafa, 2026-11-13, 18:47:55 UTC: his decision on step 2.3 (step 2.3 may be planned now), and step 2.3's
      task"); it is Mustafa's and the Advisor's brief (its line 3), one of DOCS-ALIGN's documents, so no session edits
      it [verified 2026-11-13].
- **From KEPT-LOGS** (PR #53, merged as 53272a1 on 2026-11-15; its one review round on a637c7a; Mustafa accepted
  these as left open for that pull request, `docs/DECISIONS.md`: the entry "Taken by Mustafa, 2026-11-15, 19:46:36
  UTC: KL-MERGE (a), merge #53, and the security and test findings accepted as left open"; for the kit's next change
  after the freeze, queued in `docs/DECISIONS.md`; copied by script from the pull request's description, its lines
  88-105):
  - **security-auditor: APPROVE WITH FINDINGS**, four Lows, each needing a second process run at the same moment by the
    same user, or deliberate misuse; the folder or the archive stays on every tested path:
    - SA-KL-1: a link swapped into a parent folder between the check and the removal;
    - SA-KL-2: the script's git calls inherit `GIT_CONFIG_*` from the environment;
    - SA-KL-3: the old cache is recognised by its path and modification times only;
    - SA-KL-4: the archive folder is not flushed after the rename, and the rename could replace an archive that
      appeared after the check.
  - **test-engineer: APPROVE WITH FINDINGS**, eight Lows. Test 5's child-only preload is sound (no leak, the real pack
    path, not flaky in four parallel runs). The eight:
    - KLT-1 (KLC-F2): two branches of the second walk have no test (a late bad name, a folder emptied late); neither
      can hide a defect that deletes data, so a follow-up is enough;
    - KLT-2: pack's routine refusal is tested only through `list` and `cache`;
    - KLT-3: some refusal and failure paths have no test or planted bug (they fail safe on reading);
    - KLT-4: no test puts a link inside the cache;
    - KLT-5: the pipe case can drop out silently where `mkfifo` is missing;
    - KLT-6: the kit tests are not run in CI;
    - KLT-7: the fingerprint wording (fixed in this report) and no pipefail in the fingerprint pipeline;
    - KLT-8: the links (fixed in this report).
- **From the night-shift lines** (PR #54, merged as 82c605e on 2026-11-17; its one review round on 20f9d59; the
  seven Lows were deferred in writing by the session, and the pull request merged on his go, `docs/DECISIONS.md`:
  the entry "Taken by Mustafa, 2026-11-17, 07:00:54 UTC: MERGE-54, his go for #54"; ADV-NS-L1 and SA-NS-1 went to
  him as NS-TRAILER and NS-FIRST, his (b) for both, queued in `docs/DECISIONS.md` for the kit's next change after
  the freeze; ADV-NS-L5 is carried out by DOCS-54, the docs pass that adds this; copied by script from the pull
  request's description, its lines 23-27):
  - **The advisor: APPROVE WITH FINDINGS** on head `20f9d59918eea1e3431ae830f72510f83f6424c7`. Five Lows (ADV-NS-L1 to L5), none needing a push; merge conditions C1-C4.
  - **The security-auditor: APPROVE WITH FINDINGS.** Two Lows (SA-NS-1, SA-NS-2), both recommended accepted as is.
  - **C1, the findings:** all seven Lows are deferred in writing, with no push (a Low never triggers one).
    - Two could be closed only by rewording Mustafa's lines, so they go to him as decisions, with the recommendation to leave the lines as they are: ADV-NS-L1 (the command shows no `--trailer`; a run takes it from CHECKPOINTS.md §5) and SA-NS-1 (the first stint is not tied to a status refresh).
    - The other five are follow-ups. ADV-NS-L2 and SA-NS-2 (the same point: a run blocked inside a foreground agent call is not woken by a stint's end, which was already true before this change), ADV-NS-L3 (CHECKPOINTS.md §5 still gives 110-minute figures) and ADV-NS-L4 (the guard test's KIT_LINES lacks the stint line) wait for the kit's next change after the freeze. ADV-NS-L5: docs-keeper records his words on main after the merge.
- **From change-set 0 (PR #56) and Part 15 (PR #57)** (PR #56, merged as d7dad97 on 2026-11-17 on his go of 22:59:16
  UTC, `docs/DECISIONS.md`; the first five items are copied by script from the end of #56's row in STATUS, each
  followed by its source; the sixth comes from his answer VFX-ROUTE (a) of 2026-11-17, recorded by #57):
  - the draw cap with a gun from its own file (the rules' O1; PM-0005-54 fails on the rifle file as reported: a question
    for him with change-set M's plan). His words of 22:59:16 UTC:
    "It is not waived. The way out is my call, with the plan for the first-model change-set." The three ways out, none
    researched further: `docs/research/asset-export-rules.md`, part 7, O1.
  - the fifth material class's name (O2). ADR 0005 and ADR 0003 name the fifth class differently: the rules' part 7, O2;
    part 8 gives O1 and O2 to Mustafa, with change-set M's plan.
  - the rules' proof by a first real Blender export (change-set M). The rules are on paper; the first real export may
    fail PM-0005-49 or -56 (how the armature is written): the change-set 0 report, risk 1.
  - whether the coat keeps the legacy motion (his call at change-set 10's pictures). Dropping it removes the four coat
    joints, a list change that should land before the Blender model is exported: `docs/research/soldier-joint-list.md`,
    section 6.
  - the machine lines in ADR 0001, ADR 0003 and SPIKE_PLAN and ADR 0002's "one change-set per step", carried later (the
    plan report's "Since the plan" names each carrier). The machine lines go with change-set 6's docs pass, when the
    machines are named (MACHINES); "one change-set per step" with change-set 2's docs pass (SPLIT-25).
    _Since (change-set 2's docs pass, 2026-11-19):_ "one change-set per step" has its dated pointer of 2026-11-19 in
    ADR 0002 (commit 2cedb9c, PR #61, in flight); the machine lines still wait for change-set 6's docs pass.
  - For the decision drafts' next revision: ADR 0002's package list and the seventh package. His answer VFX-ROUTE (a) of
    2026-11-17 is "also my word on a seventh package beside the six the structure draft names" (`docs/DECISIONS.md`,
    22:10:34 UTC); a dated pointer of 2026-11-17 marks the list (ADR 0002, part 3) and leaves where the package stands
    in it to the draft's next revision.
- **From change-set 1, Types (PR #58)** (merged as 9b4b518 on 2026-11-18 on his go of 15:38:13 UTC, `docs/DECISIONS.md`,
  with which he accepted "Round 2's Lows as follow-ups. The two doc lines that claim more than the code does are
  corrected in the next pull request that touches those files."; the items below are the report's "Follow-ups",
  `docs/reports/2026-11-18-cs1-types.md` lines 234-268, copied by script and indented by two spaces; #58's row in "In
  flight" named them for this section once merged):
  - **CI-CUT** (security, Medium, in CI, outside this change-set; found by the advisor agent, confirmed and narrowed by
    security-auditor; not tried on a runner): in the `toolchain` job the network cut separates the network, not the
    files. Build code runs as the runner's own user and can rewrite the checkout action's file, which the job runs
    again after the cut with network and the job's cache-write token. Through it a hostile dependency could plant a
    cache entry that this branch's smoke job later unpacks and runs as its browser. It reaches no secret and not
    main's cache scope. The fix is a CI edit: decision CI-CUT.
  - **From the security check:** ts-rs's packages ship no licence file (MIT is declared; no duty while it is
    test-only); the plan's draft said ts-rs needs Rust 1.88.0, its published manifest says 1.78.0 (both under our
    1.99.1); the network cut stops IP traffic only (for the gate v2's workflow).
  - **From round 2 (Lows, not fixed here; the code is as reviewed):**
    - Two doc lines claim more than holds. Both export tests say write mode "never writes outside the repository":
      true for a linked generated folder or parent (refused), not when a crate's own folder is a link (security-auditor
      ran it: write mode then wrote outside the worktree). `Input.commands`'s doc says "consecutive" sequence numbers;
      the drafts say only that sequences move forward. Each takes a committed link or is a wording matter; fix with the
      next change to those files.
    - The doc-comment check catches a comment end inside a `///` line (the only form the message types use). A
      hand-written `#[doc = "…"]` attribute or a block doc can still put a line of valid TypeScript into a generated
      file; the fix is to refuse those two forms in `messages.rs`.
    - The fix pass's new guards have no committed test of their own (the planted script is deleted with the
      checkpoint): they belong in the gate v2's planted set.
    - The build-folder check compares path strings: an honest run through a second spelling of the same folder, or a
      moved clone that kept its build folder, stops until a rebuild, and the message does not say so.
    - `generated/` may hold a link or a third folder unnoticed; the tag-byte pair has no "must differ" line of its own;
      on a file system that ignores case a link named `X.TS` would survive write mode's delete loop (reasoned, not run).
  - **Change-set 2:** an `exports` entry in `packages/types/package.json` before the first import by package name; the
    block writer takes offsets through a by-name handle that fails the build on a missing name; `Section::align()` in
    Rust only; the export tests stay native-only; run the layout check on the first block of every Worker start.
  - **Change-set 3:** the provisional list above; decoders cap every sender-sized field before allocating; `Join` gets
    a redacting `Debug` once its token is a credential.
  - **Change-set 4:** the header states each section's row count and byte offset; padding between sections has one
    owner; the page reads the hash as soon as its four bytes exist; content's export test shares the folder rules.
  - **The 2.3 chat:** its formatter and linter skip `packages/types/src/generated/**` (the files are compared byte for
    byte) while the type check still reads it; the gate's own regeneration builds into a build folder of its own.
  - **The drafts' next revision (his go):** who reads the message types in TypeScript (the page sees only the block);
    the ADR 0002 pointer on how ts-rs's folder is named.
  - _Since (the kit hold's docs pass, 2026-11-18):_ CI-CUT's part 1 (no cache in the `toolchain` job) is PR #60's, on
    his CI-CUT (a); its part 2 (no JavaScript action in the job that runs the cut) waits for the quality gate v2's
    workflow and stands on his list for that plan (next block). The two doc lines are in
    `crates/protocol/src/ts_export.rs:76` and `crates/client-endpoint/src/layout_ts.rs:58` ("never writes outside the
    repository") and `crates/protocol/src/messages.rs:56` (`Input.commands`, "consecutive"); #60 touches none of the
    three files, so both stay for the next pull request that does [verified 2026-11-18].
  - _Since (change-set 2's docs pass, 2026-11-19):_ the question whether #58's test zip saved and unpacked on his
    laptop (STATUS, "Waiting on Mustafa", until this pass) is answered, his message of 2026-11-18, 20:44:29 UTC, with
    a screenshot: "the zip saved and unpacked" (copied by script into change-set 2's notes,
    `.checkpoints/cs2-first-page/mustafa-2026-11-18-zip-saved.md`, kept at the commit change-set 2's report names). As
    the build chat read his screenshot, the SHA256 PowerShell printed for `packages\types\src\generated\block\layout.ts`
    equals the expected one. Change-set 2 (PR #61) touches none of the three files with the two doc lines either
    (`git diff --stat origin/main...HEAD -- crates`), so they still wait [verified 2026-11-19]. Of the five lines for
    change-set 2 above, PR #61's code has each: the `exports` entry; the block writer's offsets through
    `layout::section` and `layout::field`, read in constants, so a missing name stops the build; `Section::align()`
    in Rust only; the export tests still in `#[cfg(test)]` modules; the layout check on the first block after each
    Worker start (`packages/client/src/app/worker-link.ts`) [verified 2026-11-19, PR #61 not merged yet].
- **From the kit hold and two CI edits (PR #60)** (merged as b763ba3 on 2026-11-18 on his go of 20:20:09 UTC,
  `docs/DECISIONS.md`, with which he accepted "The five Medium findings as follow-ups."; one review round:
  security-auditor and test-engineer on the final kit text, 4877599, infra-engineer on the workflow edit, 94c197f; their
  notes committed at 18:31:54, 18:40:04 and 18:08:01 UTC; nothing Critical or High; by his word of 18:01:08 UTC,
  `docs/DECISIONS.md`: "A later finding on them that is not Critical or High becomes a follow-up."; the notes stay
  readable at the checkpoint commit that `docs/reports/2026-11-18-kit-hold-ci.md` names; each reviewer's codes are its
  own: security-auditor's F1 to F5, test-engineer's F1 to F11 and infra-engineer's L1 to L6 are different findings):
  - **His list for the quality gate v2's plan** (step 2.3), in his words, from `docs/DECISIONS.md`:
    - DEP-HOLD (how a dependency change is kept from being built before its security check), his answer of 15:38:13
      UTC: "The check in CI that option (c) adds is not decided here: put it on the list for the quality gate v2's
      plan." Option (c), as #58's report put it: "a check that refuses to build when the lockfile gains an entry with
      no recorded review".
    - CI-CUT (build code in the `toolchain` job can rewrite an action's file that runs again after the network cut),
      the same message: "Part 1 now, with the exact text of that same section; part 2 waits for the quality gate v2's
      workflow." Part 2: no JavaScript action in the job that runs the cut.
    - KIT-REMOVED (should a manifest that main removed also pass when a branch takes main in), his answer of 18:01:08
      UTC: "(a). A manifest that main removed stays held. Put the question on the list for the quality gate v2's plan,
      together with the line of the approved text that still reads a path beginning with ":" as a pattern." That line:
      `const entry = git('ls-tree', rev, '--', f);` (`.claude/scripts/checkpoint.mjs:128` [verified 2026-11-18]).
  - **The reviewers' lines for that list** (theirs, not his decisions):
    - infra-engineer, L1 (Low): if a commit moves `packages/types/src/generated`, the compare step checks nothing (git
      status on a path that matches nothing prints nothing): a guard for the folder. L5 (Low): an empty file list stops
      the type-check step with exit 1 and no message. Its lines for part 2 and option (c): the job that runs the cut
      has no `uses:` step at all (a plain git checkout of the exact commit; a gate test fails if it gains one), so no
      post step runs after dependency code with the network and the token; the lockfile check runs first in every job
      that fetches, on push as well as pull request, and fails unless every added or changed lockfile entry is named
      in a committed review record, reading files only; a workflow linter, if one comes, must accept `cache-mode`.
    - test-engineer, its F11 (Low): the kit's four test files run in no CI job and not in `tools/gate.mjs`.
    - security-auditor, the routes the hold does not cover, for his choice when that plan is written (no design): a
      plain `git push` or the GitHub tools, above all code pushed by hand while a held checkpoint commit sits under it;
      running the change before any push (`npm install`, `cargo build`, `npx`, `cargo install`); files outside the
      five names that change what CI fetches or runs (workflows, `rust-toolchain.toml`, `.cargo/config.toml`, `.npmrc`,
      `.gitmodules`, vendored source) and dropping `--locked` or `npm ci` in a workflow; other lockfile names and letter
      cases; a dependency added and removed within one push; a stale view of the remote branch or of main; branches
      and savers still on the old script.
  - **Open, two Mediums (security-auditor):**
    - F1: in two corners the hold checks another path than the one it pushes. The script reads the list of changed
      files through a helper that trims the text and decodes it as UTF-8 (`run()`,
      `.claude/scripts/checkpoint.mjs:60-67` [verified 2026-11-18]), so a manifest in a root folder whose name begins
      with a blank character (a space, a tab or another), when it is the first changed file, is checked under the name
      without the blank; the same for a folder name that is not valid UTF-8 when main holds its look-alike with the
      same content. Shown in a scratch repository: main holds `a/Cargo.lock`, the push adds ` a/Cargo.lock` with new
      content and no record: "saved and pushed". No such folder exists, and the workspaces are `crates/*` and
      `packages/*`, so CI would not build a manifest in such a folder unless a root manifest changed too, which is
      held. The follow-up: read that list raw (no trim, no decoding), with a test for a root folder " a" beside main's
      `a/Cargo.lock`.
    - F3: no rule makes the review of a pull request read the hold's record. The script's comment says "the review of
      the pull request reads it", but no rule names a reader, and `.claude/CHECKPOINTS.md` section 9 deletes the notes
      folder, the record with it, before a pull request is marked ready. The follow-up (a kit change, so with his
      word): the ready report of a pull request that changes a manifest or a lockfile quotes each record (with the
      commit it was pushed in) and the security answer it names, and the review checks every changed manifest at the
      head against those lines or main. Until then, with no kit change, change-set 2's report does exactly that.
  - **security-auditor's Lows:** F2, a record that is a link or a folder still releases (read the record only when it
    is a plain file); F4, with git's `push.followTags` on (unset here) a save would also push a tag on an in-between
    commit, and `toolchain` runs on tag pushes (the save's push should pass `--no-follow-tags`); F5, wording where the
    rule line or the script's header says more or less than the code (the code errs towards holding in each): the
    message for an unreadable push advises a record, which cannot release it (the way out is `git fetch origin main`);
    "as sha256sum prints it" is exactly `git show <commit>:<path> | sha256sum`; "with main for a branch not yet pushed"
    is the point where the branch left `origin/main`; "never pushes" does not hold in F1's corners; a link, a submodule
    or a file that became a folder can never be released.
  - **test-engineer's three Mediums** (tests that are missing; the behaviours hold by code reading): F1, no test pins
    main's side of the "equal to main" comparison read at the exact path (test 33's step 4 pins only the push side);
    F2, no test pins that a `deleted  <path>` line leaves an existing changed plain manifest held; F3, no test pins that
    `origin/main`, not a local `main`, is compared (the advisor agent had rated it Low).
  - **test-engineer's seven Lows:** F4, a plain file with the bytes of a link on main stays held, untested; F5, a
    mode-only change of a file equal to main's is released, untested; F6, "in any folder" untested for a manifest
    inside `.checkpoints/` and for a folder name git quotes; F7, "what cannot be read is held" untested (an unpushed
    branch with no `origin/main`); F8, test 29's look-alike `not-package.json.txt` pins neither name anchor alone (use
    `package.json.txt` and `old-package.json`); F9, for a branch not on the remote, the merge-base (not main's tip) is
    untested; F10, the `watch` tests remove their temporary folder before they stop the `watch` child (stop it first).
  - **infra-engineer's other Lows** (L2, L3, L4, L6), on the report's wording: neither CI check is a control against a
    hostile dependency (dependency code runs earlier in the same step, as the same user); the executable bit is the
    only difference that can reach the compare step today; "Cache mode: none" proves the mode the runner received, not
    that the cache service refuses anything (and until part 2 the post steps of checkout and setup-node still run after
    the cut, with the network and the read-only token); the job-log endpoint is named as the source of the quoted log
    lines.
- **From change-set 2, the first page (PR #61)** (merged as f5030a3 on 2026-11-19 on his go of 14:28:25 UTC,
  `docs/DECISIONS.md`, with which he accepted "The follow-ups below High, as the report lists them"; two review rounds, seven reviewers on its first head and
  six on what the fixes changed, each Opus 5.5 at effort max: no Critical, no High open; every finding below is Low;
  round 2's three Mediums and its fix pass: `docs/reports/2026-11-19-cs2-first-page.md`; the reviewers' notes stay
  readable at the checkpoint commit that report names. The ids are this docs pass's: a prefix for the reviewer (EA
  engine-architect, GE graphics-engineer, NA netcode-architect, PA perf-auditor, RR rust-reviewer, SA security-auditor,
  TE test-engineer) and the reviewer's own code where it has one; reviewers' codes overlap between them):
  - **For the CI pull request (CI-WASM), right after the merge** (each item marked by the CI-WASM docs pass,
    2026-11-23, PR #64; the evidence: `docs/reports/2026-11-19-ci-wasm.md`, "What changed" and "Evidence"):
    - TE-NT6 (**done in #64**: five timing points of the browser specs made robust for a slower machine without
      lowering a bound, and a readings line per test; test-engineer read six clean runner jobs: nothing thin, the
      allocation test drew 65 to 76 frames against a floor of more than 20): the browser checks that depend on the
      machine's speed (the frame count in the allocation window, the hidden-tab waits) are tuned on this container only.
    - RR-N4 (**done in #64**, with VF-L5 below: the pattern moved into `packages/client/scripts/build-checks.mjs` as
      `refusedBuildSettings`, with one test of 18 refused and 15 allowed names; test-engineer replayed it against 14
      broken copies of the pattern: 14 caught): the pattern that refuses cargo's settings from the environment in
      `packages/client/scripts/build.mjs` has no test of its own.
    - RR-F5 (**its fix moved to change-set 3**, with rust-reviewer; in #64 CI prints the Wasm's sha256 and the builder
      paths it carries, "printed, not judged": 28,991 bytes on every runner build, 28,987 in the containers,
      consistent with this finding and not shown to be only it; **done in change-set 3a**, 2026-11-24: 5c92311 remaps
      cargo's registry path on both Wasm builds, through `CARGO_ENCODED_RUSTFLAGS` since 79666ad9; on 5c92311,
      79666ad9 and 8619d539 the runner's Wasm is the container's byte for byte and CI prints "no registry path in the
      Wasm"; rust-reviewer reproduced 5c92311's sha256 in another target folder, its round 1): the Wasm carries the builder's registry path (a panic
      location inside wasm-bindgen), so one commit gives other Wasm bytes and sizes on another machine (also for
      change-set 3's size table).
    - E2E-TYPES (**done in #64**: CI type-checks the specs and `playwright.config.ts`, with
      `packages/e2e/node-names.d.ts` for the four Node names they use; seven of seven planted type errors caught, in a
      container with near-matching versions): the browser specs in `packages/e2e/tests/` are run by Playwright but
      type-checked by no tool.
    - SA-TREE (**moved to change-set 3**, as #64's plan said; **done in change-set 3a**, 2026-11-24: 5c92311, the
      build's refusal `missingOverflowOverrides` before the players' cargo build (a planted removal of an override
      from the root manifest stopped the real build); the
      security-auditor's Medium on it (any crate with a source in brackets counted as ours) fixed in 79666ad9 and
      confirmed fixed in its round 2): the list of third-party crates with an overflow override
      (root `Cargo.toml`) is checked by hand (`cargo tree`); a test would stop a new runtime crate from shipping
      without its override.
    - SA-DIST (**done in #64**: `.gitignore` has `dist-*/`): `.gitignore` covers `dist/` but not the `dist-<something>`
      folders `build.mjs --out-dir` accepts.
    - _Moved to change-set 3 as well, by his words of 2026-11-19, 14:28:25 UTC ("With change-set 3, one line in
      .gitignore for __pycache__/, so that a save cannot pick up compiled Python files again."): that `.gitignore` line
      (not in #64's `.gitignore` [verified 2026-11-23]). **Done in change-set 3a**, 2026-11-24: a1c59e8
      (`.gitignore:18` at 8619d539 [verified 2026-11-24])._
  - **For change-set 3, "The loop closes":**
    _Dated line, 2026-11-24 (change-set 3a's docs pass): his SIZE-3 (b) split change-set 3 into 3a, "Rules and
    boundaries", and 3b, "The loop closes"; change-set 3's plan (draft 3, section 8) placed each item below. Those done
    in 3a are marked with their commit and the reviewer that found them closed (the round-1 notes, readable at
    f153ddf4: `.checkpoints/cs3a-rules-boundaries/test-engineer-review1.md`, `engine-architect-review1.md`,
    `security-auditor-review1.md`); NA-N1, -N2, -N4 to -N7, EA-F9, EA-F10's bindings part, EA-F13's one-writer item,
    TE-NT7 and PA-N2 stay for 3b, NA-N3 for change-set 4, EA-N5, GE-N1, GE-N4 to -N6 and PA-L2 for change-set 5._
    - NA-N1 to NA-N7 (netcode-architect, all Low): the step gets the host's time; the pause on a hidden tab comes from
      the mode once online play lands, and held buttons are cleared on hide; who picks the blend fraction between two
      blocks (one sentence in `picture.ts` now, a field at change-set 4; events read from every block); the seed as an
      authority-only section written last, so the shared state is a prefix; the page's per-frame number is not the
      protocol's command sequence (name it the post number); a start tick in the authority's match setup; the Pacer
      throws on a due time that is not finite. Its answers for that plan: `checked_sub` or wrap-aware compares on
      decoded ticks, never a plain minus.
    - EA-F8 (**done in change-set 3a**, a1c59e8: `isWorkerToPage`, the link switches on the checked type;
      engine-architect's round 1: closed): the Worker-to-page messages are parsed by hand (an `isWorkerToPage` beside
      `isPageToWorker`).
    - EA-F9: the endpoint's state lives in `sim-wasm`; it belongs in `client-endpoint` from change-set 3.
    - EA-F10 (**its TypeScript part done in change-set 3a**, a1c59e8, engine-architect's round 1: closed; the
      bindings part, `newest_input`, stays for 3b): `newest_input`, which only tests call, ships in the player's Wasm
      (drop it when the input reaches a command), and test-only members ship in the TypeScript (`Ring.clear`,
      `BlockPool.free`, the `percentile` re-export).
    - EA-F13 (**done in change-set 3a but for its one-writer item**, the anchor, which stays for 3b: the link's state
      read through getters and the report's own pause shape in a1c59e8, `@shamble/types` among the Worker package's
      dev-dependencies in 0412814; engine-architect's round 1: closed as planned): nits on ownership and small
      couplings.
    - EA-N3 (**done in change-set 3a**: the page's half in a1c59e8, `WorkerLink.end` the one owner; the Worker's half
      in 0624557, one guard around every WebAssembly call; engine-architect's round 1: closed): "the match is over"
      has two owners, and the Worker's error event does not tell the link.
    - EA-N5: why a fallback runs is worked out in the app shell, not reported by the picture (engine-architect: or when
      `render/` changes hands, change-set 5).
    - EA-N6 (**done in change-set 3a**, a1c59e8: the Worker's chunk named by its source module, through a plugin of
      the Worker's bundle; engine-architect's round 1: closed): the build's glue check finds the Worker's chunk by a
      name prefix.
    - VF-L7 (the verifier's re-check; **done in change-set 3a with EA-N6**, a1c59e8: every other script is the
      page's whatever its name, with a lookalike plant; engine-architect's round 1: closed): the same name prefix lets
      a lazily loaded page module named `worker-...` go unscanned by the build's WebAssembly check (its second plant
      passed both builds): scan every script but the Worker's entry chunk, named by the build itself. With EA-N6.
    - VF-L4, VF-L5 (the verifier): the policy header is asserted on the 200 answer only; the build's refusal of
      environment settings has no test (with RR-N4). _VF-L5 is done in #64, with RR-N4 (above); VF-L4 stays here._
      _VF-L4 is **done in change-set 3a**, a1c59e8: the four headers are asserted on every answer, refusals too
      (`packages/client/serve.test.mjs:73-78` at 8619d539 [verified 2026-11-24]; no reviewer names it)._
    - GE-N1: the lost-device and lost-context route has no test.
    - GE-N4: each renderer error prints twice, with three's doubled prefix.
    - GE-N5: after a lost device the page says nothing outside the development readout (with ADR 0003's loss
      handling).
    - GE-N6: "because WebGPU did not start" cannot say why (no adapter, or the device request failed).
    - TE-NT3, TE-NT4, TE-NT5, TE-NT7 (**TE-NT3 to TE-NT5 done in change-set 3a**, a1c59e8, each with a plant that
      test-engineer re-ran in its round 1: proven; TE-NT7 stays for 3b): no test for the cleaning of the error text, or
      for the hand-off time read before the copy; one equality in the browser test also holds on stale reports; the
      listener's own body is outside the allocation sums.
    - PA-N2: on a fast machine the per-frame allocation window can fall partly before the browser optimises the code.
    - PA-L2: the pool's size assumes one tick per frame and one per wake (a pool of 9 once blocks grow).
    - TE-L (**done in change-set 3a**, a1c59e8; test-engineer's round 1: proven, its plants re-run; left: no plant
      for the six newly stubbed `WebAssembly` names, follow-up 3A-PLANTS): four branches of the small server without a
      test case of their own; `skipIf` for the link cases; more `WebAssembly` names in the test stand-ins; one Worker
      only; the scratch-buffer test.
    - SA-ENV (**done in change-set 3a**, a1c59e8: the spec restores both; test-engineer's round 1: no test can see the
      restore): the isolation spec sets two variables in the test process and never restores them (no later spec
      reads them today).
    - FAVICON (**done in change-set 3a**, a1c59e8: `/favicon.ico` gets "no content", 204, with the common headers;
      security-auditor's round 1 read it: not a finding): the browser's own request for `/favicon.ico` gets a 404.
  - **For change-sets 5 and 6 (the picture and the look):** GE-F7, the memory cost of multisampling (an estimate of
    about 125 MB at 1920x1080; three's memory count leaves the samples out); PA-L4, the readout's frame time is the
    interval between animation frames, not the frame callback's own work time (both change-set 6).
  - **For the quality gate step (2.3):**
    - SA-F6: the npm scope `@shamble` is unclaimed: someone who claims it could publish `@shamble/types@0.0.0` for an
      install that resolves without the lockfile (ours follow it); the options: claim the scope (an act of his own
      account) or a check that every `@shamble/*` lockfile entry is a link into `packages/`.
    - SA-SWAP: in the root container a process left behind could swap the wasm-bindgen tool between its check and its
      run (residual).
    - TE-NT2: the planted-defect script matches a failure by its text, not by the failing line's mark; the planted set
      belongs in the gate v2's.
  - **For the decision drafts' next revision (his go):** EA-F6, the block has one fixed size for a Sim's life, which
    ADR 0002 part 5 does not decide (record it there); EA-F12, decide and record whether the Worker's counters ship in
    production as monitoring candidates.
  - **From his words of 2026-11-19, 13:36:37 UTC** (whole in `docs/DECISIONS.md`, the entry of that time; the ids
    SLICE-FEEL and PROJ-DRAW are his, LANE2-LIST and FIREFOX this pass's):
    - SLICE-FEEL, for change-set 4's plan: aiming down sights and the sprint-to-fire delay are game rules of Slice 1.
      That plan prices and places both: the input, the rule, the fields, what lands in change-set 8, and the lines
      they add. The starting values come from his Testlines export (his sandbox: look at it, copy no code).
    - LANE2-LIST, for change-set 4's plan: lane 2's first list of needs on the "Lane requests" issue #59 (issue comment
      6994614989 of 2026-11-18, 23:24:14 UTC: 16 fields, 6 events, 5 wishes, 10 gaps or questions, by its own count)
      is answered there row by row; with SLICE-FEEL its wishes W1 and W4 are needs. A comment there is never his word.
    - PROJ-DRAW, for change-set 9's plan: lane 1 draws grenades and arrows in flight plainly; they pass to lane 2 with
      the plain tracers and flash when change-set 9 is merged.
    - FIREFOX, from now on: the game has to work in Chrome, the Chromium-based browsers and Firefox (Safari and Mac
      come later). The browser tests run in Chromium only (one browser project; the allocation test uses Chromium's
      own profiler). A Firefox run needs a Firefox build from the internet: the question goes to him with the CI pull
      request's text. His check of #61 in Firefox is the page's first Firefox run; what it finds is a fix before the
      merge or a follow-up here.
      _Dated pointer, 2026-11-23 (the CI-WASM docs pass, PR #64): what his Firefox check found (Firefox 160 on his
      Windows computer, reports of 2026-11-19, 14:53:34 and 15:09:11 UTC, and a screenshot; the record:
      `docs/reports/2026-11-19-cs2-browser-check.md`): errors 0 in both reports; three.js's WebGPU backend starts in his
      Firefox (adapter "(not given)"); the forced WebGL 2 fallback runs (its name: "ANGLE (NVIDIA, NVIDIA GeForce GTX 980
      Direct3D11 vs_5_0 ps_5_0), or similar"); the hidden-tab pause holds (0 ticks while hidden); the page opened as a
      file keeps its own line of text. What it found beyond that is the group below (FPS-LINE, FF-WAKES, ADAPTER, TQ-FF;
      none an error count above 0); #61 merged with no fix for Firefox. The question went to him as FIREFOX-TESTS: his
      answer (c) of ANSWERS-7 (2026-11-23, 14:31:06 UTC) is now (b), by his words of 2026-11-23, 21:24:59 UTC: "Firefox
      in CI joins the small CI pull request after 3a (E1's)." (`docs/DECISIONS.md`)._
      _Dated pointer, 2026-11-25 (ci-after-3a's docs pass): on that pull request's branch, closed when it merges, CI
      runs the rule tests (`tests/rules/`) in the runner image's own Mozilla Firefox 160.0 on Linux (his
      FIREFOX-SOURCE (c), 2026-11-25, 00:07:29 UTC); the browser tests that draw stay Chromium-only in CI, since the
      runner's Firefox gives the page no WebGL (FF-DRAW, in "From ci-after-3a"); WebKit and Safari not._
      _Dated pointer, 2026-11-26 (ci-preview-handover's docs pass): on that pull request's branch, closed when it
      merges, the `firefox` project also runs the recording's replay test (`tests/replay/`), his FIREFOX-REPLAY
      (FIREFOX-REPLAY in "From ci-preview-handover")._
  - **From his browser check** (2026-11-19, 14:46:25 to 15:09:29 UTC: eight "Copy report" texts from Brave 1.100.58 and
    Firefox 160 on his Windows computer; the record, with his four messages whole and the readings below:
    `docs/reports/2026-11-19-cs2-browser-check.md`, "Observations for the follow-ups"; the ids are that record's; none
    is an error count above 0):
    - BR-GAP (**the one reading not explained**; to read again with change-set 3's browser check, whose README asks him
      to keep the window in front and uncovered during the first run and has one step with it covered on purpose, by
      his words of 2026-11-19, 16:08:04
      UTC, `docs/DECISIONS.md`): Brave's first run drew 46,382 frames while it ticked for 410 s (113 a second, where
      every other Brave run averages 180), so for about 150 s the page drew nothing while the simulation ran on, and no
      pause was recorded. The window may have been covered or the screen off; the pause on a tab switch works in Brave.
    - FPS-LINE (**done in change-set 3a**, a1c59e8: frames a second is the frames kept over the time they took,
      `packages/client/src/harness/frame-times.ts` at 8619d539 [verified 2026-11-24]; his Firefox step of 3a's browser
      check reads it): the readout's "frames a second" comes from the usual gap between two frames; in Firefox it said
      239.2 where the run's average was 179.9. The line should be a count over time (no change-set named).
    - NEG-ZERO (**done in change-set 3a**, a1c59e8: a value that rounds to zero prints without its minus sign,
      `packages/client/src/overlay/report.ts` at 8619d539 [verified 2026-11-24]): "deliveryP50: -0.00 ms" (Brave,
      `?pad=65536`): the page's and the Worker's clocks, each rounded by the browser, can give a difference just under 0.
    - FF-WAKES: in Firefox 4 % to 10 % of the Worker's wakes come before a tick is due (6982 wakes for 6717 ticks; 1126
      for 1018), with no dropped tick and lateness p99 4.667 ms; a reading for the pacing marks in change-set 3.
    - BR-LATE: Brave's first run has lateness p99 13.2 ms and max 15.833 ms, its other runs p99 4.0 to 6.3 ms; it reads
      like the system's coarse timer (about 15.6 ms) while the window was not in front; with BR-GAP.
    - ADAPTER: Brave names no adapter on WebGPU and "Brave" on WebGL; Firefox none on WebGPU and a rounded name on
      WebGL. A report from Brave cannot say which GPU ran (his machine's, from Firefox: NVIDIA GeForce GTX 980); Brave
      reports 6 logical processors where Firefox reports 24.
    - TQ-FF: no timestamp query on Firefox's WebGL 2 (yes on Brave's WebGL 2 and on WebGPU in both).
    - NOT-SHOWN: a single hidden stretch of 6 minutes; Brave's double-click on `index.html`; his Shields setting.
- **From the CI-WASM pull request (PR #64)** (merged as 1ec6f6c on 2026-11-24, by his main Advisor on his tap, after
  his go MERGE-64 (a) of 2026-11-23, 23:44:10 UTC, `docs/DECISIONS.md`; risk tier 2; two review rounds,
  infra-engineer, test-engineer and security-auditor, then test-engineer and security-auditor on what changed: no
  Critical or High; the items are the report's "Follow-ups", `docs/reports/2026-11-19-ci-wasm.md`, with each owner
  and time as the report and the reviewers' notes give them; "not named" where they give none. The reviewers' notes
  stay readable at the checkpoint commit 5911b56 (`.checkpoints/ci-wasm/`). The ids are the report's. _Corrected
  2026-11-24 (change-set 3a's docs pass): this header said "in flight, waiting for his go", and that the notes stay
  readable "at the checkpoint commit that report names"; the report names none, and 5911b56 is the notes' last
  commit (the advisor agent's read of #64's final head) [verified 2026-11-24: `git ls-tree 5911b56` holds the notes]._):
  - TE-COUNT (test-engineer; the gate v2, step 2.3): CI pins no test count (a skipped or removed test keeps it green,
    only zero tests turns it red); the gate v2 fails a run with a skipped or missing test. Today each clean run printed
    "Running 12 tests using 1 worker", "12 passed" and the boundary block's "Tests  7 passed (7)".
  - TE-TYPE-PLANTS (test-engineer; when: not named): a planted type error in a Worker file and in a page file has never
    been shown failing CI's type checks (the specs' check caught seven of seven, in a container only).
  - TE-TSC-LINES (test-engineer; the next CI edit, its exact text asked of him): each type check prints its own result
    line (today `tsc` prints nothing on success, and a pass is read from the next echo under `bash -e`).
  - TE-104 (test-engineer; with the next change to that spec; **done in change-set 3a**, a1c59e8, with TE-NT5's
    change: a plant of 90 extra ticks on resume with no drop counted is caught at that bound; test-engineer re-ran it
    in its round 1: proven): the hidden-tab bound at
    `packages/e2e/tests/skeleton/skeleton.spec.ts:104` has never been seen hit (it holds by arithmetic; the plant that
    would reach it is caught two lines earlier): a plant that runs extra ticks on resume without counting drops.
  - TE-MONO (test-engineer; with the next change to that spec; **done in change-set 3a**, a1c59e8, a monotonic clock
    in the hidden-tab test; test-engineer's round 1: proven): `performance.now()` in place of `Date.now()` at `skeleton.spec.ts:92` and
    `:98`.
  - TE-WATCH (every session that reads a red `toolchain`; on every runner-image or Chrome change): read the allocation
    test's byte windows and the default backend first.
  - TE-CPU (test-engineer; the next CI edit, its exact text asked of him): the job prints `nproc` and the CPU model
    once, so a timing reading on a runner comes with its machine (no run prints them today).
  - TE-HIDDEN-FRAMES (test-engineer; when the page's pause behaviour changes): the hidden-tab test's four-frame waits
    need the page to keep drawing while it is faked hidden (`packages/client/src/app/boot.ts:73-76`); revisit them if that
    changes.
  - IMAGE-CHROME (infra-engineer; on a red): the browser tests run the image's Chrome, which moved 157.0.8103.50 to
    157.0.8103.78 in five days; a later Chrome can outrun Playwright 1.65.0 or move the allocation test's byte
    windows: read the printed line, and the fix is a Playwright update (a dependency, with its security answer) or
    another browser source (his CHROME-SOURCE (a) accepts a moving browser). A red is read, not re-run into green.
  - CUT-PATH (infra-engineer, Low after its own correction; with CI-CUT part 2, the gate v2): inside the cut PATH is
    the runner's, whose first folders the cut's user can write on this image, so a build script can shadow node,
    npx, git, sha256sum or Chrome for every later line; the root-owned tool protects its own file only.
    _Pointer, 2026-11-25: new evidence from ci-after-3a's review (CUT-PATH-N1 in "From ci-after-3a")._
  - TOKEN-WORDS (security-auditor and infra-engineer, Low; the next CI edit, his yes): the comment "The only step with
    the token" (`.github/workflows/toolchain.yml:91`) is true of run steps; checkout and setup-node, both pinned by
    SHA, receive the job's token as their default input, as on main ("the only run step with the token").
    _Pointer, 2026-11-25: not done in ci-after-3a, the next CI edit; that chat deferred it again (the chat's choice,
    not his), to keep its pull request to his four parts; still due at the next CI edit ("From ci-after-3a")._
    _Dated pointer, 2026-11-26 (ci-preview-handover's docs pass): **done** on that pull request's branch, closed when
    it merges: `.github/workflows/toolchain.yml:94` reads "The only run step with the token" [verified 2026-11-26 at
    d12e3a8]._
  - F-CHANNEL (security-auditor, Low; with change-set 3's build work; named in the list of what his go for #64
    accepts; **done in change-set 3a**, 5c92311: `CARGO_OVERRIDES` refuses every `RUSTC_` name, `__CARGO_` and
    `CARGO_UNSTABLE_`, CI's cut names still allowed, and names compared in upper case since 79666ad9;
    security-auditor's round 1 found them "refused as meant"): the refusal pattern in
    `packages/client/scripts/build-checks.mjs` also refuses `__CARGO_*`, `CARGO_UNSTABLE_*` and `RUSTC_*`: with
    `__CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS` and `CARGO_UNSTABLE_*`, stable cargo takes nightly-only features
    from the environment; local builds only (CI's cut starts from `env -i`).
  - GH-TOOLS for later pull requests (his; open as GH-NEXT, the report's "Decisions needed"): his GH-TOOLS (a) of
    2026-11-23, 17:15:07 UTC was "for this pull request"; Builder V3 cannot write to GitHub (no GitHub tool, the
    platform refuses `gh pr create`, the guard keeps `gh api` read-only). _Answered for change-set 3a by his GH-NEXT
    (a) with one change (2026-11-23, 23:44:10 UTC, `docs/DECISIONS.md`): the chat puts each pull request's title and
    text in its notes folder, his main Advisor opens the draft and sets them, he presses Ready and Merge; his (c), a
    kit change, "is planned with the gate step, not now". After 3a: open._
  - SAVER (the kit; when: not named, a kit change that asks him): the checkpoint saver pushes to the local branch's
    name, and Builder V3's worktree branch is named `cs/ci-wasm` (the main folder holds the `claude/` name and is never
    switched), so no saver ran in that chat: card and notes were committed by path and pushed by hand after every
    step (the advisor agent ruled it acceptable; a departure from `.claude/CHECKPOINTS.md` §5).
  - MANUAL-MODE's record value (beside MANUAL-MODE, in "From foundation 2.2" above): on his messages of 2026-11-23,
    17:22:42, 19:58:23 and 20:47:59 UTC the record read `default` while his screen showed Accept edits, as his rule of
    17:15:07 UTC foresaw ("If it reads default: my screen shows Accept edits"); on 17:15:07 UTC it read `auto`, and
    nothing was written on it.
  - Firefox in CI (the chat he calls Builder V3; the small CI pull request after change-set 3a, his FIREFOX-TESTS (b),
    2026-11-23, 21:24:59 UTC).
    _Pointer, 2026-11-25: carried by the chat he calls CI Builder (ci-after-3a), in part: the rule tests in Firefox in
    CI; the drawing tests are FF-DRAW ("From ci-after-3a")._
    _Dated pointer, 2026-11-26: the replay test joins them on ci-preview-handover's branch, closed when it merges
    (FIREFOX-REPLAY in "From ci-preview-handover")._
  - The line for CLAUDE.md on the third account (ACCT-3; his to give).
  - RR-F5's fix, SA-TREE and the `__pycache__/` line: change-set 3 (the block for #61 above). _All three **done in
    change-set 3a** (marked there, 2026-11-24)._
  - Known ones it touches: REQUIRED-CHECK, CANCEL-ON-MAIN, TWO-PLAYWRIGHTS and WASM-ARTIFACT (a dated line under each,
    in "From foundation 2.2" above); CI-CUT and NODE-26 (#64 changes nothing they say: checkout's post step still
    runs after the cut, and the job still names `node-version: 22`).
- **From change-set 3a, "Rules and boundaries"** (merged as 28254a7 on 2026-11-24 by his main Advisor on his tap, PR #65;
  _corrected 2026-11-25 (change-set 3b's docs pass): this header said "in flight, no pull request yet" until the merge_;
  branch `claude/loving-hopper-6p0mg2` standing in for `cs/3a-rules-boundaries`, code head 8619d539; risk tier 1; two review rounds, five reviewers
  (rust-reviewer, engine-architect, test-engineer, security-auditor, perf-auditor) on 5c92311 and again on what changed
  at 79666ad9, then security-auditor alone on one fix (APPROVE, the High closed); the verifier PASS on 8619d539,
  nothing above Low; report `docs/reports/2026-11-24-cs3a-rules-boundaries.md`. The reviewers' notes stay readable at
  f153ddf4 (`.checkpoints/cs3a-rules-boundaries/`: `<reviewer>-review1.md` and `-review2.md`, the advisor agent's
  `advisor-mul-add.md`), round 3's and the verifier's at 1c668e95 (`security-auditor-review3.md`,
  `verifier-final.md`), and all of them in their last state at 6491b839, the last commit before the notes were
  removed (with the advisor agent's `advisor-head.md` and his browser check's message). The ids are this docs pass's; each item names its source):
  - **His two, from his words of 2026-11-24, 15:58:25 UTC** (MUL-ADD (a), with his Advisor's two conditions "for the
    report and STATUS's follow-ups"; whole in `docs/DECISIONS.md`):
    - 3A-FMA-CHECK (change-set 4): "The check on the built file (no fused multiply-add inside the players'
      WebAssembly) is built in the same change-set that brings the first outside crate into the simulation (the
      collision library, planned for change-set 4), not later. That change-set's plan names it." Until then only our
      own code is held to the ban: the scan of `crates/sim/src` refuses `mul_add` (its one-name list `HELD` in
      `crates/sim/tests/rules_scan.rs`); an outside crate's code is not scanned, and in WebAssembly such a call would
      run the toolchain's software `fmaf`.
    - 3A-RUST-REPORT (when: not named): reporting the defect to the Rust project. In WebAssembly `f32::mul_add` gives
      the value of two roundings wherever the result lies below the smallest normal `f32` (the builder's measurement on
      the 48 constructed cases; rust-reviewer's on 22,048 of its own: the whole class of such results, not 16 points),
      where std 1.99.1's page says "The result of this operation is guaranteed to be the rounded infinite-precision
      result" (the builder's read of 2026-11-24, 15:25 UTC: change-set 3a's card at f153ddf4, step 11). His words:
      "Nothing is posted anywhere without my go."
  - 3A-PART7-POINTER (the drafts' next revision; the same message): ADR 0004 part 7 names "explicit `mul_add`, which
    std specifies exactly" (`docs/adr/0004-simulation-core.md:263-264`); it gets a dated pointer for his MUL-ADD (a) at
    its next revision, "with its exact sentence shown to me first". No word of ADR 0004 changes in 3a.
  - 3A-LINT-MULADD (the quality gate step, 2.3): the step's lints carry his MUL-ADD (a), so that `mul_add` cannot come
    back silently when they replace the interim scan: part 7's list still names it, and only the scan's `HELD` refuses
    it meanwhile (the card at f153ddf4, "Follow-ups this pass leaves").
  - 3A-GEN-CHECK (the small CI pull request; a CI file: his yes on its exact text): CI runs neither vector generator's
    `--check` (`crates/devtools/vectors/make_f32_vectors.py`, `make_chacha8_vectors.py`); a `pin` line added by hand
    would let WebAssembly accept a new wrong value while the native test ignores pins, and only review and a local
    `--check` would see it (test-engineer's round 2, L-CHECK-CI).
    _Pointer, 2026-11-25 (ci-after-3a's docs pass): **done** on that pull request's branch, closed when it merges: the
    `toolchain` job runs both generators' `--check` inside the network cut (`python3 -I -B ... --check`,
    `.github/workflows/toolchain.yml`) [verified 2026-11-25 at 832268e], green there on Python 3.12.3 (run
    45676587814); two planted faults fail the checks in the container, none is planted in CI (its report)._
  - 3A-RULES-STEP (the small CI pull request; and `devtools`' first native-only dependency): `devtools` is both the
    native tools crate and the test-only WebAssembly module every development build makes, so its first native-only
    dependency must be target-gated, or the development build and CI break; and the test-only module gets a build step
    of its own in CI (engine-architect's round 1, F5).
    _Pointer, 2026-11-25 (ci-after-3a's docs pass): its CI half **done** on that pull request's branch, closed when it
    merges: `cargo build --locked --target wasm32-unknown-unknown -p devtools --lib` on a line of its own inside the
    cut [verified 2026-11-25 at 832268e]; it proves that `devtools` builds for wasm32, not the module `build.mjs` makes
    for the tests. Still open: the rule that `devtools`' first native-only dependency is target-gated._
  - _Built in change-set 3b, on its branch (closed when 3b merges; the source: 3b's card, steps 3, 7 and 9):_
    3A-MANIFEST-COMMENT in 1ee3e918, 3A-HOOK-TEST in 4eeec33a (`crates/sim-wasm/tests/panic_hook.rs`), 3A-FROM-F64
    in f8db0de3 (20 cases) and round 1's fix pass (4 more: 24 appended, 10,072 in all) [verified 2026-11-25 at 6abf3d2a: the root `Cargo.toml`'s comment,
    the test file, `CASES` in `crates/devtools/src/rules/f32_ops.rs`].
  - 3A-FROM-F64 (3b): the cast from `f64` to `f32` is now tested on real `f64` values, but no input is an exact tie
    and none lies between the largest `f32` and the overflow point (test-engineer's round 2, its Low a; rust-reviewer's
    round 2).
  - 3A-HOOK-TEST (3b): the panic hook's test installs a process-wide hook inside `sim-wasm`'s shared unit-test binary,
    where a later panicking test would race it; it moves to a test binary of its own (engine-architect's round 1, F6;
    rust-reviewer's round 1, L5).
  - 3A-FEATURE-VERSION (when `sim` first gets a dependency): the feature compare matches packages by name, not by name
    and version, so another version of a crate `sim` uses, with its features, counts as `sim`'s wherever it sits in
    the tree; the builds then differ and every build stops: it fails closed (engine-architect's round 2, NEW-A,
    measured on a scratch workspace).
  - 3A-SCAN-HELPERS (with the change-set that adds part 7's three helpers, or the quality gate step): the scan
    refuses those helpers when imported by name or called by a module path, and `max` or `min` on our own ordered
    types (`Tick::max`; only `Ord::max` passes): false refusals, so it fails closed (engine-architect's round 2,
    NEW-B).
  - 3A-PART5-CLASH (the step that first nests generated types): the feature compare would refuse the feature ADR 0002
    part 5 plans for nested generated types, one that only dev-dependency entries turn on
    (`docs/adr/0002-code-structure-build-toolchain.md:353-357`); that step decides (engine-architect's round 2,
    NEW-C).
  - 3A-SCAN-NAMES (the quality gate step's type lint): hashed maps not named after std's pass the scan (`DashMap`,
    hashbrown's `HashTable`); the scan's doc says so, and the type lint is the real net (rust-reviewer's round 2,
    N4).
  - 3A-NAME-SECTION (with the table of build options: the build plan's change-set 3 card names it,
    `docs/reports/2026-11-17-build-plan-slice-1.md:330`, and defines it nowhere; change-set 3's plan, section 8, made
    it a follow-up of its own): a quarter of the players' Wasm is its name section (7,997 of 31,805 bytes at 5c92311)
    (rust-reviewer's round 1, L3).
  - 3A-RNG-COPY (with PM-0004-27's cost, change-set 8): the generator copies its cached 64-byte block to the stack on
    every draw (`next_u32` in `crates/sim/src/rng.rs`) (rust-reviewer's round 1, L4).
  - 3A-STATS-ROWS (when: not named): the Worker's statistics check lets extra keys through and the copy report copies
    every key, so a Worker of another build could add uncleaned lines to the report; the report's rows should come
    from the known list (security-auditor's round 1, L-STATS; narrower than main's check, which took any object).
  - 3A-PLANTS (when: not named): no planted defect for the six newly stubbed `WebAssembly` names (`Table`, `Global`,
    `Tag`, `Exception`, `Suspending`, `promising`) or for the Worker's start path; two plants (`module-no-hook`,
    `tel-not-a-file`) are caught at their named test, but for a neighbouring reason (test-engineer's round 1, R20 and
    R26); and the planted-defect script leaves one plant's build on disk for the next (`room-drift` meets
    `hook-start-exported`'s build and gets one extra failure; its own test still fails for its own reason) (the
    verifier's L2). With TE-NT2, the planted set's place in the gate v2.
  - 3A-CONFIG-FILE (the quality gate's flag check): a cargo config file outside the repository can still set a
    profile, a linker, a compiler wrapper or an `[env]` table; the build shuts out only its `rustflags`, by giving the
    remap through `CARGO_ENCODED_RUSTFLAGS` in cargo's environment, and the comments in `build.mjs` and
    `build-checks.mjs` say so (rust-reviewer's round 1, L1; security-auditor's round 2, N2). Those comments still
    leave out two more such settings, source replacement (`[source]`, a crate's code swapped with no diff) and
    `build.rustc` (security-auditor's round 3, N3: name both, or say "among others").
  - 3A-PEDANTIC (the quality gate step): six warnings of clippy's stricter `pedantic` group left in
    `crates/sim/src/rng.rs` and `crates/sim/tests/rules_scan.rs` (among them a long function and similar names)
    (engine-architect's round 1, F4: nine; its round 2: seven; the seventh stood inside words that were wrongly in
    quotation marks as his and went with that correction, the advisor agent's read of c5f6a85a).
  - 3A-FMA-LOG (when: not named): the native float test prints whether the processor has FMA from inside a passing
    test, which cargo hides, so no log shows that line; only the pass itself shows that the exact path ran
    (test-engineer's round 2, L-FMA-LINE; its fix: `--show-output` where the test runs, or the line said to show on a
    failure only).
  - 3A-NO-FMA (when: not named): on a processor without the fused multiply-add instruction (and without AMD's FMA4)
    the native `fmaf` takes the same software path as WebAssembly, and the three native float tests then fail; shown
    by emulation only (rust-reviewer's round 2: run under a debugger with the processor's FMA flags cleared), never on
    such a processor.
  - 3A-OTHER-BROWSERS (with Firefox in CI, then later): the vectors ran in Node and Chromium only, not in Firefox,
    WebKit or Safari (test-engineer's round 2, "Not verified").
    _Pointer, 2026-11-25 (ci-after-3a's docs pass): its Firefox half **done in CI on Linux** on that pull request's
    branch, closed when it merges: the `firefox` project runs `tests/rules/` (the float and stream vectors and the
    planted counter) in the runner image's Mozilla Firefox 160.0 (2 passed, run 45676587814) [verified 2026-11-25];
    the Firefox drawing tests are not in CI (FF-DRAW). WebKit and Safari stay open._
    _Dated pointer, 2026-11-26 (ci-preview-handover's docs pass): on that pull request's branch, closed when it
    merges, the `firefox` project also runs the recording's replay test (FIREFOX-REPLAY in "From
    ci-preview-handover")._
  - 3A-MANIFEST-COMMENT (the next manifest batch, 3b's): the root `Cargo.toml`'s comment "in sim-wasm only" (line 14)
    is no longer true; and "The later cargo-deny wasm-bindgen rule must allow devtools too" (security-auditor's
    answer on the dependency batch, `security-auditor-deps.md`).
  - 3A-DEPARTURES (the drafts' next revision, as dated pointers, never as rewording now): readings that go beyond the
    list his DEPART-3 (a) accepted (change-set 3's plan, section 12, question 4), so not covered by it: PM-0004-74 ("One
    feature set builds the sim on every target", `docs/adr/0004-simulation-core.md:1184`) read as three builds, the
    players' Wasm, the test-only module and the native tests, with dev-dependency edges for the tests; ADR 0002 part
    8's row "Rust in Wasm | wasm-bindgen-test in Node" (`docs/adr/0002-code-structure-build-toolchain.md:494`), met
    through our own test-only module instead; PM-0004-27's "each stream" read as four edge stream ids (no stream exists
    yet); ADR 0002 part 5's planned dev-only feature (3A-PART5-CLASH) (engine-architect's round 1, its departures line,
    and NEW-C). Inside DEPART-3 (a), said here so it is not lost: PM-0002-53's counter in WebAssembly is the tools
    crate's own function (test-engineer's round 1, its Low (c): "accepted by DEPART-3 (a); say so").
- **From change-set 3b, "The loop closes"** (reviewed, his browser check in, no pull request yet; branch `claude/loving-hopper-6p0mg2`
  standing in for `cs/3b-loop-closes`, final code 6abf3d2a; risk tier 1; two review rounds by six reviewers
  (engine-architect, netcode-architect, rust-reviewer, test-engineer, security-auditor, perf-auditor): round 1 on
  bbbf09e4 (2026-11-25; a container restart cut three of them short, who finished in round 2), round 2 on 0dfca749
  and what changed after it, all six PASS, nothing Critical or High open; the verifier PASS; the advisor agent
  approved the code as it stands; report `docs/reports/2026-11-25-cs3b-the-loop-closes.md`. Source of every item:
  3b's card, "Open findings" and "Round 2 so far", and the reviewers' notes
  `.checkpoints/cs3b-loop-closes/<reviewer>-review1.md` and `-review2.md`, readable at the notes' last commit
  80a51e4299d2a9abb0c08adc76dc1a31716f036a. The ids are this docs pass's):
  _Pointer, 2026-11-25 (change-set 4a's docs pass, which carries 3b's owed one): **merged** as PR #68, 3ab9eae,
  2026-11-25 13:57:03 UTC by `merged_at`, by merge commit, from his main Advisor thread on his tap of 13:56:26 UTC, with
  the expected head 5e02ee73 (the merge commit's message) [verified 2026-11-25 with git and GitHub's REST API]. There is
  no change-set 3c any more (his ORDER-3C (c) of 2026-11-25, 16:59:52 UTC, `docs/DECISIONS.md`): read each "3c" below
  as change-set 4a for the recording's items and as the link simulator's own change-set (after 4d, merged before
  change-set 7 starts) for the simulator's, as change-set 4's plan, section 8, places them by name (the plan:
  `.checkpoints/cs4-walk/plan.md` at c17a054, the last commit of 4a's branch that holds its notes)._
  - **RELEASE-COUNT, decided by him on 2026-11-25: (a), accept as built** (his words: `docs/DECISIONS.md`, the
    entry of 11:38:37 UTC; no release counter in change-set 3c). What it was about (test-engineer's round 2): the
    presses mark's row (PM-0002-12, `docs/adr/0002-code-structure-build-toolchain.md:1283`) says "the Worker counts
    the presses and releases it applies through the edge counters"; the boundary draft (ADR 0002 part 5, line 319)
    has the post carry "press and release edge counters", while the netcode draft (ADR 0001 part 5, line 226)
    defines a press counter per button only, and that is what 3b built. The releases are worked out from the
    presses and the held keys, with two checks on the page: a button held again with no press
    (`EchoTally.heldWithoutPress`) and each tick's held keys against the post it used (`WorkerLink.wrongKeys`), both
    asserted 0 in both browser specs. The option not taken, (b): a release counter per button in the messages, 16
    bytes more per command, up to 128 more per input message (292 to 420) until delta coding. **His two
    conditions:** no report calls any part of that mark passed, and the mark's numbers stay as the row has them;
    and the next item.
  - **3B-RELEASE-DRAFTS (his; at the drafts' next revision, which comes to him before it is written):** the two
    drafts are made to agree on how releases are counted (ADR 0002 part 5's sentence about press and release edge
    counters, and the method cell of the row PM-0002-12), with the reason written there. Neither draft is edited
    by change-set 3b.
  - 3B-RESUME-FLASH (**his, still open: he named no letter**; it comes back with change-set 4, when a soldier is
    on screen and his browser check will see it): what the first picture after a pause shows. In
    his browser check he once saw, in Brave, a key's box still lit for a split second after coming back to the
    tab; not in Firefox and not on a second try. From the code: the match stands still while the tab is hidden,
    the box shows the server's last echo, and the first tick after the return lets the key go
    (`docs/reports/2026-11-25-cs3b-browser-check.md`). He wrote that it is not an issue; nothing is changed.
  - 3B-LATE-BOUND (change-set 7; before online play: no room goes behind a network transport without it): the room has
    no bound on how late a command may run, so a client that sends nothing for N ticks and then its backlog runs two
    commands a tick for N ticks (security-auditor's round 1, M1, Medium; deferred in writing in its round 2, not
    closed). Built in change-set 7 with each of the two ways of timing commands, each with a long-hold test (idle,
    then the backlog: at most the repeat window of two-command ticks; honest outages 0 refused). netcode-architect's
    round 2 gives the two forms: running on arrival, a per-client budget, +1 per room tick, banked to at most the
    repeat window (8), one spent per command run, a command over it dropped and counted outside `refused()`;
    buffered, a command whose tick lies more than 8 ticks before the tick being run dropped as late; one mark for
    both (idle 30, 300 or 3,000 ticks, then at most 8 extra commands; honest outages refused 0 and at most 7
    two-command ticks). The report must not read as if running on arrival had no bound.
  - 3B-LONG-HOLD (for his decision at ADR 0001's next edit): PM-0001-21's lag-switch case holds for only 200 ms (6
    ticks, inside the repeat window), so the row passes a room with no bound; a long-hold case (60 s) is proposed
    (netcode-architect's round 2).
  - 3B-PLAUSIBLE (change-set 7): the client endpoint refuses and counts a snapshot more than the clock estimate plus
    30 ticks ahead, and a confirmation newer than the newest sequence it sent (netcode-architect's round 2; the same
    gap as rust-reviewer's round 1, RR-3, whose link-simulator half is 3c's).
  - 3B-ACKS (before online play): a snapshot acknowledgement bounded from below, and one per entity group
    (netcode-architect's round 1, NA-5).
  - 3B-OUTAGE-256 (**his decision when due**, before online play; a stated limit): during an outage presses are lost in
    steps of 256 once one button is pressed more than 255 times unconfirmed (netcode-architect's round 1, NA-5, and
    round 2: its 3,000-tick outage lost 1,024 of 2,520 presses per button, exactly 4 times 256). The option of
    PRESS-PACE he said yes to (the plan's words, not his own: `.checkpoints/ci-wasm/cs3-plan.md` at fcf6def, line 303,
    question 3, option (a)) reads "It is applied one tick (33 ms) later; nothing is ever lost."; the question was about
    a second tap within one tick. No person can reach this limit in 3b (it needs a network), so nothing is needed before
    3b's merge (the advisor agent, 2026-11-25).
  - 3B-PACER-255 (**his decision when due**, once a press fires a weapon): at most 255 presses of one button wait
    their turn in the press pacer; one beyond that is dropped and counted (netcode-architect's round 1, NA-4;
    `crates/client-endpoint/src/press.rs`). Against the same option's "nothing is ever lost"; only a machine gets
    there in 3b (more than 30 presses a second on one key for over 8 seconds).
  - 3B-ONE-PER-TICK (change-sets 4, 8 and 9, each with a test): a system that acts on presses applies at most its own
    rule per tick (one action, or its cooldown's worth), since an honest client after an outage brings up to 255
    presses of one button in one command (security-auditor's round 2; written on `sim::PlayerCommand`).
  - 3B-KEY-POST (before PM-0002-13, key to tick, is judged): post the keys from the key handler too, not only once
    per frame; in perf-auditor's model the 95th percentile is 34.8 to 48.0 ms, 2 of 16 phases over the mark's 50 ms at
    a 4 ms delay and 6 of 16 at 8 ms, and 32.9 to 40.1 ms in every phase when posted from the handler
    (perf-auditor's round 2).
  - 3B-POOL-16 (once the block grows): the Worker's buffer pool from 4 to 16 (perf-auditor's round 2).
  - 3B-SATURATED (for PM-0002-8): under key input on a saturated main thread Chrome held blocks back 2 to 4.5 s
    (perf-auditor's round 2).
  - 3B-ALLOC-METHOD (change-set 5): the allocation method of PM-0002-7 runs with the optimising compilers off or sums
    under the link's own function, since the browser charges compiled code to the listener's frame (perf-auditor's
    round 2); PA-N2 (change-set 2's follow-up) carries over, never "done" (engine-architect's round 1, EA-8).
  - 3B-CSS-PROD (when: not named): the key diagram's CSS (`#key-diagram` in `packages/client/src/styles.css`) reaches a
    production build, though the diagram itself is cut out of it (engine-architect's round 1, EA-9).
  - 3B-PUB-MODS (a nit; when: not named): `pub mod queue` (`room`) and `pub mod wire` (`protocol`) could be private
    (rust-reviewer's round 1, RR-6).
  - 3B-REFUSED-FIELD (Slice 3): the block's `refused` gets a field of its own for what online play has no room count
    for (engine-architect's round 2).
  - 3B-OTHER-BROWSERS (with the small CI pull request another chat writes): PM-0002-12 is shown in Chromium only;
    its row asks for Chromium, Firefox and WebKit (the row, `docs/adr/0002-code-structure-build-toolchain.md:1283`).
    _Pointer, 2026-11-25 (ci-after-3a's docs pass): still open after that pull request: the mark's test
    (`packages/e2e/tests/skeleton/presses.spec.ts`) opens the page, which the runner's Firefox cannot start drawing,
    and lies outside `tests/rules/`, the only folder the `firefox` project runs [verified 2026-11-25 at 832268e]; it
    waits for FF-DRAW, and WebKit for its own route._
  - 3B-OQ6-POINTER (at ADR 0001's next edit; NOT edited now): three places in the netcode draft still say the proof
    test decides how commands are timed (`docs/adr/0001-multiplayer-netcode.md` lines 252, 1156 and 1699 to 1700); each gets a
    dated pointer to his decision of 2026-11-24 (`docs/DECISIONS.md`, 22:44:50 and 23:24:50 UTC; the build plan's three
    sentences were changed by 3b's docs pass) (the advisor agent's read of 2026-11-24, 23:12 UTC, condition C3).
  - The board add-on is active for the builder chat he calls Builder V3 (since 2026-11-24): a pointer of the
    chat's own, no word of his; nothing of the board is in the repository (he asked for that in the chat).
  - 3B-BLUR-TEST (the verifier, 2026-11-25; with the next change to the page's key handling): nothing fails when
    the listener that lets go of every key as the window loses the focus is removed (its own plant: 85 unit tests
    and 16 browser tests still pass). A browser test holds W, sends `blur` and expects W let go.
  - 3B-HOSTILE-ALLOC (the verifier, 2026-11-25; with 3c's tests): the room's "allocates nothing per tick" is
    counted for honest ticks only (its own plant: an allocation on every datagram the room cannot decode passes all
    114 tests). The two reviewers' probes counted 0 after warm-up; a counting allocator in the two random-message
    tests makes it a test.
    _Pointer, 2026-11-25 (change-set 4a's docs pass): moved by 4a's size ladder (rung 1, at 2,450 written lines) to
    the link simulator's own change-set, its other named home; not written in 4a; told to him before it moved, in
    the question STOP-4A, which he answered (a) at 20:48:11 UTC._
  - **For change-set 3c's plan** (3c: the link simulator and the first recording, moved out of 3b by the plan's size
    ladder, he told before anything moved; with them the one new read-only number on the players' file, the state's
    hash, by his DEPART-3, and `devtools`' three path edges that 3b declares and does not use yet, security-auditor's
    L2 = engine-architect's EA-6): the simulator's clock reaches it through `LocalMatch` from recorded times
    (netcode-architect's round 1, NA-7; engine-architect's round 2); its reliable channel delivers exactly once (NA-7);
    loss and the rate limit as silent drops, duplicated datagrams only (engine-architect's round 2); corruption as loss
    only, what an authenticated transport delivers (netcode-architect's round 2, citing RFC 9001 section 6.6 and RFC
    8446 section 5.2; rust-reviewer's round 1, RR-3); a `link()` accessor on `LocalMatch`; the host's time recorded
    with every tick; the state-hash export returns the shared hash only, as plain numbers, read only during a replay,
    its buffer private to `Room` (all three engine-architect's round 2); `late` checked against the simulator's own
    reorder count (netcode-architect's round 2).
    _Pointer, 2026-11-25 (change-set 4a's docs pass): placed by change-set 4's plan, section 8 (rows 1-27;
    `.checkpoints/cs4-walk/plan.md` at c17a054). **Met by
    4a, closed at its merge (#70):** the replay runner, the comparer and the field-by-field difference, the first
    recording natively and in WebAssembly and in CI (rows 1-4); the state's hash on the players' file, shared hash
    only, as one plain number, read only during a replay, with its pin, and the host's time recorded with every tick
    (rows 5-8; since 4a's fix pass the hash has no buffer at all: `shared_hash(&self)` [verified 2026-11-25 at
    fb5d0ff]); the hash report (PM-0004-21: keep FNV-1a 64) before the first recording was committed, and the begun
    parts of PM-0004-17, -18, -19 with the three planted divergences (rows 9-10, run again on movement in 4c). Row 11,
    3B-HOSTILE-ALLOC, moved (above); row 12, RELEASE-COUNT (a): nothing built. **To the link simulator's change-set:**
    the simulator itself, its clock, its reliable channel, loss and corruption, `link()`, `late` and its marks (rows
    13-21, 23-27). The three path edges (row 22): still unused after 4a [verified at fb5d0ff]; dropped in 4b's
    manifest batch if still unused there, each back with its first user in the link simulator's change-set._
- **From the preview hosting check** (lane 2, the chat he calls Builder V2; in flight, its pull request #66 (2026-11-25: merged, 09faefa);
  branch `claude/optimistic-albattani-mdoonz` standing in for `cs/preview-hosting-check`; risk tier 1; the items of its
  report's "Follow-ups (for STATUS)", `docs/reports/2026-11-24-preview-hosting-check.md`, their text kept; the ids are
  this docs pass's):
  - For step 2.6's plan:
    - PHC-CLICKLESS: previews without his click per preview, and what keeps the token safe then (his words of
      2026-11-19, 00:01:25 UTC).
    - PHC-LOCKFILE: a locked install (a committed lockfile, install scripts off; his point 3 of 2026-11-16).
    - PHC-TOKEN-SETUP: token setup ends with Cloudflare's verify answer, not a screenshot of the form; the form's
      single-date chip sets a one-day window.
    - PHC-PROBES: every probe gets a named User-Agent.
    - PHC-DELETE: delete with `--force` or the API; a weekly sweep on `main`.
    - PHC-ACTIONS-READ: `actions: read` reconsidered (SA-R2).
    - PHC-ERROR-PATH: the deploy step's error path and the account printout (the rename answered the name; the ID is
      masked when it equals the secret).
  - PHC-TR-END (after the merge): the test repository: archive it (his TR-END (a)) once this report is merged; then
    delete the test project and the Access application (Zero Trust, Access, Applications).
  - PHC-SCRIPT-LOWS: Round 2's Lows (infra L1, L2, L4; test R2-1 to R2-6) and the deferred SA-R5, SA-R6, F5, F6 stay
    with the throwaway scripts; none carries into step 2.6 unless its plan picks it up.
- **From ADR 0006's revision** (lane 2, the chat he calls Builder V2; PR #67, a draft until the advisor agent
  approves its final head, then ready; branch `claude/optimistic-albattani-mdoonz` standing in for
  `cs/adr0006-header-row`; risk tier 1; review round 1's follow-ups as plan revision 3 lists them (its item R3-7),
  F3's and F7's short texts from test-engineer's notes; round 2's two optional Lows from infra-engineer (G1, G2);
  the report `docs/reports/2026-11-25-adr0006-header-row.md`; the ids are the chat's, not his):
  - For the draft's next revision:
    - ADR6-ROLLBACK (Medium; infra-engineer's F2): align Rollback's trigger with the mark-fail rule ("When a mark
      fails"); "only the deploy target changes" understates a move.
    - ADR6-PAGES-RETENTION (infra-engineer; with the verifier's round 2 Low): Pages keeps deployments 30 days plus the
      100 newest. Every place that still reads as unlimited previews is re-read against Pages' limits page as read on
      2026-11-25 ("Preview deployments older than 30 days are deleted, unless they are one of the project's 100 most
      recent deployments", developers.cloudflare.com/pages/platform/limits/):
      - ADR 0006 line 67 (at ab15d6d), in "The findings the decision turns on": "with no cap on previews", with no
        dated pointer;
      - part 1's *Why:* "no cap on previews", dated by this pull request;
      - the options row's "Unlimited previews";
      - R6-HOST-F3 (`docs/research/r6-infrastructure.md` lines 72-79).
      R5-PREV-F13 is checked at the same time.
    - ADR6-PM7-PAGES (test-engineer's F3): PM-0006-7's method cell names the dashboard pages each reading read.
    - ADR6-PM7-FIGURE (test-engineer's F7): PM-0006-7's "does not if by 0" names the figure the page shows as the
      Free plan's build count.
    - ADR6-WORDING (infra-engineer's round 2, G1 and G2, Low, optional; if the lines are touched again): G1, the
      Workers paragraph's "the oldest deleted when the limit is reached" reads better as "the least recently deployed
      one deleted when the limit is reached" (Cloudflare's Previews page deletes the Preview deployed to least
      recently); G2, part 1's build-count text gives "00:55 Amsterdam time", which is his message's own label (the
      record's time, 23:53:15 UTC, is 00:53 Amsterdam): call it "his label".
    - ADR6-REVISIT-WORKERS (his WORKERS-NEXT (a), 2026-11-25, 14:55 Amsterdam): Cloudflare's "Start new projects with
      Workers" goes into the draft's "Revisit when" line.
  - ADR6-WK-ROUTE (infra-engineer; at R5's and R6's next revision): the R5 and R6 lines on `versions upload
    --preview-alias` and "no deletion read" get dated pointers.
  - ADR6-WK-BIC (infra-engineer): any re-run on Workers sends a descriptive User-Agent.
  - PHC-PREVIEW-NOINDEX (Medium; test-engineer's F1, older than this change): step 2.6's plan takes a build-output test
    that each preview mode's `_headers` carries the `noindex` rule and production's does not, with a bite; and, before
    any preview is served where the host adds no `noindex`, the preview expectation is read there.
  - WORKERS-NEXT, decided by him on 2026-11-25, 14:55 Amsterdam (`docs/DECISIONS.md`): (a), keep Pages. The draft's
    part is ADR6-REVISIT-WORKERS above; step 2.6's part is STEP26-WORKERS.
  - STEP26-WORKERS (his, for step 2.6's plan): keep the deploy step small enough to move to Workers later, say in two
    lines what such a move would cost, and weigh the 30-day rule for old previews.
- **From ci-after-3a, the small CI pull request after change-set 3a** (the chat he calls CI Builder,
  session_019YNMUUemXge56777MiUWFz; no pull request number yet, its title and text in the chat's notes; branch
  `claude/keen-tesla-v807cf` standing in for `cs/ci-after-3a`, code head 832268e; risk tier 2; one review round by
  security-auditor and infra-engineer on the tried texts before his yes: `security-auditor-ci.md` and
  `infra-engineer-ci.md` in `.checkpoints/ci-after-3a/`, readable at 832268e until the notes are deleted; report
  `docs/reports/2026-11-25-ci-after-3a.md`; the ids are the chat's and this docs pass's, not his):
  - E1 (change-set 3's plan, section 6 at 6491b839; his opening block of 2026-11-24, part a): **done** on the branch,
    closed when it merges: the release test line names all six crates (`-p sim -p protocol -p room -p client-endpoint
    -p sim-wasm -p devtools`) [verified 2026-11-25 at 832268e].
  - FF-DRAW (his question first: his words of 2026-11-25, 14:55 Amsterdam, point 3; when: with the gate step 2.3, as
    its own question, his FF-DRAW-WHEN (a) of 20:35 Amsterdam): the Firefox drawing tests in CI
    need a graphics route for Firefox on the runner, an install line or a Firefox graphics setting (such as
    `webgl.force-enabled`), each his question first. On GitHub the runner's Firefox 160.0 gave the page no WebGL ("This
    page could not start drawing: ... this.gl is null", "libEGL.so.1: not in the loader cache of this runner"; run
    45670972698 on 52ad5d3, its log read 2026-11-25); in the container a blank page got the same refusal
    (`question-2026-11-25-firefox-gl.md`). Until then the `firefox` project runs `tests/rules/` only.
  - FF-SPEC-COMMENTS (outside that pull request's fence; with FF-DRAW, when Firefox runs these specs): the comments at
    `packages/e2e/tests/skeleton/isolation.spec.ts:1-3` ("Chromium's 4 of the 12 cases; Firefox and WebKit run in CI
    later") and `file-url.spec.ts:1-3` ("here for Chromium") [verified 2026-11-25 at 832268e] still read true while
    the `firefox` project runs neither spec; they are to be updated when it does (security-auditor's F1 follow-up;
    the plan's section 7).
  - FF-FILEURL-PREF (security-auditor's F1, Medium; its comment fix was taken; matters only when Firefox draws):
    Playwright starts Firefox with its own test preferences, one of them `security.fileuri.strict_origin_policy`
    false, so what `file-url.spec.ts` records there may differ from a player's Firefox (inferred, not measured). His
    FF-FILEURL (a) kept the spec in Firefox with that comment; the rules-only change took both out of Firefox's run.
    Revisit with FF-DRAW.
  - FF-STOP (infra-engineer's F6, Medium; his FF-STOP (a): "keep the 5-minute stop for the first run"): the Firefox run
    keeps Playwright's 5-minute stop. The 15 tests Firefox would run took about 187 s in Chromium in the container, two
    of them 2.4 minutes together, so a slower Firefox may reach the stop; revisit when the drawing tests return (a CI
    text change: his yes first).
  - FF-SANDBOX (infra-engineer's F5, Low): the log prints "Sandbox: CanCreateUserNamespace() unshare(CLONE_NEWPID):
    EPERM" just before Firefox's version line (run 45676587814, read 2026-11-25), so Firefox starts with a weaker
    sandbox of its own (the report's reading); harmless inside the cut (security-auditor's threat model: a Firefox exploit gains only the cut's user,
    which planted code already is). A launch failure would show as the 2 rule tests failing.
  - CUT-PATH-N1 (security-auditor's N1; new evidence for CUT-PATH in the #64 block above): the new lines call
    `python3` and `grep` through the runner's PATH (`ldconfig` by its full path, `/usr/sbin/ldconfig`; Firefox by
    `/usr/lib/firefox/firefox`); GitHub's image build runs `chmod -R 777` on `/opt` and `/usr/share` (security-auditor's
    E10: actions/runner-images' scripts on their main branch, not the image of 2026-11-16, read 2026-11-25 through a
    fetched summary, the scripts' order not read), so Chrome's folder under `/opt` is probably writable inside the cut
    (inferred, not measured); Firefox's code under `/usr/lib/firefox` is root-owned in the 161.0 package the trial
    read, not checked on the runner's 160.0.
  - TOKEN-WORDS (the #64 block above; due "at the next CI edit"): deferred again, by the chat, not by him.
    _Dated pointer, 2026-11-25 (his TOKEN-WORDS (a) with one change, 20:35 Amsterdam, `docs/DECISIONS.md`): it rides in
    the next workflow text the chat asks him for, the hand-over step for Builder V2's previews, at no yes of its own._
    _Dated pointer, 2026-11-26 (ci-preview-handover's docs pass): **done** there, on that pull request's branch, closed
    when it merges (`toolchain.yml:94`, "The only run step with the token" [verified 2026-11-26 at d12e3a8])._
  - FF-ORDER (infra-engineer's F3, Low): the Firefox run starts only if the Chromium run passes (`set -e`), so a red
    Chromium run gives no Firefox evidence; accepted for now.
- **From change-set 4a, "The recording"** (PR #70, a draft; lane 1, the chat he calls Builder V4; branch
  `claude/cool-pasteur-oiqk1r` standing in for `cs/4a-recording`, source final at fb5d0ff unless the verifier finds
  something; risk tier 1, no browser check; two review rounds by five reviewers (netcode-architect, rust-reviewer,
  test-engineer, engine-architect, security-auditor): round 1 on eb14739, round 2 on what the fix pass changed, all
  five APPROVE in round 2, nothing Critical, High or Medium open; the stop line 3,400 by his STOP-4A (a). Source of
  every item: the chat's `.checkpoints/cs4a-recording/round1-fold.md` and `hash-report.md` (at c17a054), folded into 4a's report
  before the pull request is marked ready. N4, N5, C1-C4, E2-2, E2-3, 4A-BYTES-LENGTH and 4A-FIREFOX-REPLAY are the
  reviewers' and the chat's ids; the other 4A- ids are this docs pass's):
  - For 4b:
    - 4A-NA-4B (netcode-architect's round 1, for 4b's re-read): what `first-loop` replays on once content reaches the
      constructor.
    - 4A-EA-4B (engine-architect's round 1): content enters through the constructor only; if the players' file must
      compute the content's SHA-256, `sha256.rs` moves whole into the content crate, never a second copy; first-loop's
      header is re-made with its frozen content set and every hash line stays byte-identical, which proves that
      loading content changed no state.
  - For 4c:
    - N5 (netcode-architect's round 1, Low): `recording.ts` stays out of the production build only because nothing
      imports it: a development marker when the paced replay imports it.
    - 4A-BYTES-LENGTH (rust-reviewer, a watch, no defect today): `StateSink::bytes` writes no length, which is right
      for the 16 press counters; a run of variable length (a soldier list, events) needs a fixed-width count before
      it, or two different states can give the same bytes (4c's step 1). Since 4a's fix pass (E3) `bytes` and `secret`
      take runs of a fixed length only, so a run of varying length cannot be handed to them [verified 2026-11-25 at
      fb5d0ff: `crates/sim/src/world.rs`, `StateSink`].
    - E2-2 (engine-architect's round 2): `SHARED_STATE_BYTES` lost its last user outside the crate; 4c turns it into a
      length per match: drop the re-export then.
    - 4A-NA-4C (netcode-architect's rounds 1 and 2): the frozen content copies are listed inputs of the folder check,
      not unlisted files; the shared length comes from the world, not a constant; `StateSink::f32` with a finite
      check; every slot of the soldier table is handed over with its occupied flag and generation, so that line i of
      a state print is always slot i and the field comparison never misaligns. For Slice 2: the recording's header
      has no mode.
    - 4A-EA-4C (engine-architect's round 1): `u8`, `f32` (through `to_bits`) and `row()` join `StateSink`; the paced
      replay reads the hash through a development-only driver, never through the live host's interface; the button
      meanings get one home in Rust.
    - 4A-TE-4C (test-engineer's round 1): one known answer with every field non-zero, read by both readers; the real
      search for a sine input where the two targets differ, and the three injections on movement fields; the replay
      loop pinned to the live host's call order and its rebase on resume; `scripts.rs`'s button numbers pinned to
      `keyboard.ts` (its fifth item, the two-process test's line filter, N6, was fixed in 4a).
    - C4 (security-auditor, the paced replay in 4c): behind a development marker the production message check
      refuses; its own `Sim` from the recording's test seed, never the live match; `replayOn` keeps calling
      `checkReplayable`.
  - E2-3 (engine-architect's round 2; 4c, or change-set 8 at the latest): with `write_state` test-only (done in 4a),
    `secret` is the only public way seed bytes leave the crate; a wrapper that outside readers can only count would
    make the seed rule one the types enforce (about 10 lines).
  - C1 (security-auditor; decided at change-set 7 and binding from Slice 3's first build that talks to a real server):
    before `Sim::shared_hash` is in a build whose authority is elsewhere, it hashes only the client's own world; no
    message carries a hash over state outside the recipient's interest set, and the server never answers a client's
    hash with match or mismatch over such state; tests pin both. Why: FNV-1a can be run backwards (its script got 4
    hidden bytes back from one hash in 0.06 seconds).
  - For change-set 8: N4 (netcode-architect's round 1, Low, about 4 lines): `files.rs` and `bin/replay.rs` report the
    shared column's step first; once random streams join the authority's part, the whole column can differ earlier.
    C2 (security-auditor): the random streams' key and buffered output go only through `secret`, with a test that the
    dumps of two worlds that differ only in their seed are identical.
  - For Slice 3: C3 (security-auditor): the server refuses to start a live match on a seed that begins with `TESTSEED`.
  - 4A-S3-READ (security-auditor's round 2, left open, to be named to him in 4a's report; for the quality gate step
    2.3): S3's read side. The tests that read the recordings still follow a link (with `first-loop.rec` linked to an
    endless device a test reads until memory runs out); the folder check refuses such a link, so CI cannot go green
    with one, and the exposure is a test run on a hostile branch. Its fix is a repository-wide "no tracked links"
    check: a gate change, risk tier 2.
    _Dated pointer, 2026-11-26 (4b-1's docs pass): the read side is **done** on 4b-1's branch by its step 4, his
    LINKS-CHECK (c) ("about 15 lines in 4b", the entry of 2026-11-26, 12:21:17 UTC), closed when 4b-1 merges: the Rust
    tests read a recording only through `devtools::files::read_plain`, the Node tests and the browser spec only through
    `packages/sim-worker/boundary/plain-file.ts`, both refusing anything that is not a plain file in a plain folder, with
    planted links on both sides and a text scan in `tests/recordings.rs` [verified 2026-11-26 at ef2edec]. The
    repository-wide "no tracked links" check stays for the gate step 2.3 (the chat's reading of LINKS-CHECK (c) in its checkpoint, not his words)._
  - 4A-FIREFOX-REPLAY (a wish for the CI Builder; his word first, engine-architect's E2-4, for 4a's report's
    "Decisions needed" as FIREFOX-REPLAY): the replay spec needs no WebGL, so one line in the browser tests' setup
    would run the recording in Firefox too, a second engine; that widens the Firefox entry he accepted as cut to the
    rule tests (RULES-ONLY-READ (a), 2026-11-25, 19:36:36 UTC).
    _Dated pointer, 2026-11-26 (ci-preview-handover's docs pass): his word came as FIREFOX-REPLAY in his block of
    2026-11-26, 13:20 Amsterdam (`docs/DECISIONS.md`); **done** on that pull request's branch, closed when it merges
    (FIREFOX-REPLAY in "From ci-preview-handover")._
  - 4A-HASH-REVISIT (PM-0004-21's report, by rust-reviewer: keep FNV-1a 64): revisit when the hash first runs on a live
    tick (server or Worker) or the shared state passes 64 KiB; the replacement then is XXH64, in our own code (no
    crate). A change costs one change-set: `make_recordings` re-makes every hashes file; world.rs's 8 known answers and
    the vector file's label and digest by hand; the old hashes must still replay green before the swap. Not measured
    by the report: Chromium, Firefox and Safari (WebAssembly in Node only), the real soldier table, a server running
    many rooms, ARM.
- **From ci-preview-handover, the preview hand-over step** (the chat he calls CI Builder,
  session_019YNMUUemXge56777MiUWFz, on his block of 2026-11-26, 13:20 Amsterdam; lane 2's request: issue #59, its
  comment of 2026-11-25, 19:39 UTC; no pull request number yet, its title and text in the chat's notes; local branch
  `cs/preview-handover` pushed to `claude/keen-tesla-v807cf`, code head d12e3a8; risk tier 2; the advisor agent's plan
  review `advisor-plan.md` and one review round by security-auditor and infra-engineer on the tried texts before his
  yes, `security-auditor-ci.md` and `infra-engineer-ci.md`, all in `.checkpoints/ci-preview-handover/`, readable at
  d12e3a8 until the notes are deleted; report `docs/reports/2026-11-26-ci-preview-handover.md`; CI on d12e3a8: to be
  read; the ids are this docs pass's, the reviewers' own ids in brackets, not his):
  - The hand-over (his UPLOAD-SOURCE (a), GUARD-STEP (a) and HIDDEN-FILES (b)): at the end of job `toolchain`, on pull
    requests from this repository only, a check with absolute program paths that `.gate/dist-production` holds only
    plain files and folders, none named with a leading dot, and is not empty (it prints the file count and size), then
    `actions/upload-artifact` at cf430e030ddbb5b0abf93d22962f4752f3646cd9 (v7.0.2) uploads it as `preview-site`, kept 7
    days, an empty upload an error, hidden files off (`.github/workflows/toolchain.yml:215-243` [verified 2026-11-26 at
    d12e3a8]). In the container the check passed on the real production build (5 files, 892,209 bytes) and failed on
    each of seven planted faults (`trials.md`, `question-2026-11-26-ask2.md`). The upload step has never run; its proof
    is the pull request run's log: its file count against the check's (counts, not bytes: infra-engineer's IE-5), the
    digest and the artifact id. CI: the push run on d12e3a8 skipped both steps, as meant; the pull request run is their first. No consumer yet: lane 2's `preview.yml` is not on main, and a
    `workflow_run` starts only from main's copy, so no preview starts until it is (infra-engineer; "Open bugs and
    risks", the ci-preview-handover entry).
  - TOKEN-WORDS (the #64 block above): **done** on the branch, closed when it merges (`toolchain.yml:94`), with two
    comments the new steps would have made false (`:4`, `:130`) [verified 2026-11-26 at d12e3a8].
  - FIREFOX-REPLAY (his block of 2026-11-26, point 2; 4A-FIREFOX-REPLAY in the 4a block above): **done** on the
    branch, closed when it merges: the `firefox` project's `testDir` is `./tests` with `testMatch`
    `["rules/**/*.spec.ts", "replay/**/*.spec.ts"]` (`packages/e2e/playwright.config.ts:38-40` [verified 2026-11-26 at
    d12e3a8]). In the container's Firefox 161.0, 3 passed, the replay test "first-loop.rec: 1800 steps, every shared
    hash as in the file" (`trials.md`); in the runner image's Firefox 160 on d12e3a8: "first-loop.rec: 1800 steps, every shared hash as in the file" (toolchain 45836151624). A red, a time-out or a hash
    difference there comes to him before any change and is never re-run for a green (his point 2; plan R2-4).
  - CUT-ESCAPE-FIX (his words of 2026-11-26, 15:05 Amsterdam, point 6: "CUT-ESCAPE stays on record as accepted. Put
    its later fix (the cut's code as its own throwaway user) into STATUS as a follow-up."; when: not named by him;
    CUT-ESCAPE's own home is step 2.3, by his go for #48): the cut's code as its own throwaway user.
    security-auditor's fuller form of the fix (its SA-1, Medium, CUT-ESCAPE itself, not new with this change): its own
    uid on its own copy of the workspace, the output copied out by root and checked, and `unshare --pid --fork` so that
    no process of the cut outlives the step. Why: the cut's code keeps the runner's uid and can change what later steps
    run (the action's code in the work folder and the step's `GITHUB_ENV` and `GITHUB_PATH` files, inferred from the
    runner's layout; `BASH_ENV` set that way reaches the check's absolute shell, shown locally), and the upload holds
    the runner's upload key with the network, as checkout's and setup-node's post steps already do (from the runner's
    source, read through a fetched summary: inferred).
  - PH-SA-2 (security-auditor's SA-2, Low): CUT-ESCAPE's acceptance rested on "contents: read and no secrets"; after
    this change a workflow holding the Cloudflare token publishes what the cut wrote, so the acceptance now also rests
    on lane 2's preview treating the build as data. security-auditor's read at lane 2's 9ded5a8 (its branch, not main;
    [assumed] here, not read by docs-keeper): `hosting/preview.mjs` walks with lstat (no link followed), refuses dot
    names, names outside a safe set, host-acted files and non-plain items, caps size and count, needs `index.html`,
    and never runs the build; `preview.yml`'s job `check` needs the run's workflow name `toolchain`, its path, the
    event `pull_request`, success and a run from this repository. The sentence is on CUT-ESCAPE's record above.
  - PH-IE-4 (infra-engineer's IE-4, Low, likely benign): the upload keeps `overwrite` false; if artifact names are
    scoped to the run, not the attempt, a re-run of a pull request run whose first attempt uploaded `preview-site`
    fails the step, and lane 2's runbook prescribes that re-run. GitHub's docs are silent on it; the action's own client
    expects several artifacts of one name in a run (consistent with names per attempt, not shown). Read it on the first
    re-run; `overwrite: true` would be a CI text change (his yes first).
  - PH-KEEP-NAMES (for the gate step 2.3; lane 2's note 1 in its request; plan R2-10): step 2.3 replaces
    `toolchain.yml`; keep the workflow name `toolchain`, the path `.github/workflows/toolchain.yml` and the artifact
    name `preview-site`, or change lane 2's preview with them: its `check` job matches the name and the path, and
    `preview.mjs` finds the hand-over on main by the line `name: preview-site` (infra-engineer's read at lane 2's
    dc00c79).
  - PH-MERGE-REF (for lane 2; the advisor agent's F9, plan R2-10): a pull request run builds the pull request merged
    into main, so `preview-site` is the build of that merge, not of the pull request's head commit [assumed: from
    the plan and the review, not read in GitHub's docs by docs-keeper].
  - PH-TIME (plan R2-10): the job's time on the first run, against its 15-minute limit (`toolchain.yml:38`); main's
    last run took 6.0 minutes (infra-engineer). On d12e3a8 the job took 5 min 45 s (the push run).
  - PH-ACTIONS-SETTING (his 15:05 point 5; infra-engineer's one unknown): if the repository's Actions settings allow
    only chosen actions without `actions/upload-artifact`, every run fails at its start; the first run on the draft is
    the test, and such a failure goes to him with the setting to change.
  - PH-DELETE (at this pull request's merge; his FF-REFETCH (a), the advisor agent's N2): the Firefox 161 folder under
    `.gate/` and the read-only copy of the action's code in the scratchpad (a download not used) are deleted.
- **From guard-wrappers** (the chat he calls the Guard fix chat, session_019YNMUUc6oiiNVygR9H56Q7; branch
  `claude/confident-gates-tbnq38` standing in for `cs/guard-wrappers`, code head 83bc2417, main 2483e9a merged in at
  7be0a933 in a separate worktree; risk tier 2; no pull request yet; four security-auditor rounds, the fourth by his
  leave (R3-WAY, 2026-11-25, 23:01:07 UTC) and the last; by his R4-END (1) what is approved merges, with R4-1 open, on
  his go; report `docs/reports/2026-11-26-guard-wrappers.md`; the notes in `.checkpoints/guard-wrappers/`
  (readable at commit 68b91211f6336a9308a7b420d205854026eac1dd). Class words only: every form stays out of the repository (his words of 2026-11-25,
  16:34:18 UTC, item 3e, and of 2026-11-26, 14:09:28 UTC, item 1). The ids are security-auditor's (SA-, R2-, R3-, R4-),
  the advisor agent's (ADV-) and the chat's (INPUT-RUN)):
  - Left open as security findings: R4-1 (Critical) and INPUT-RUN (a named limit): "Open bugs and risks",
    "guard-wrappers".
  - For the next kit change-set (his R3-WAY of 2026-11-25, 23:01:07 UTC, item 4: the minor points stay follow-ups, with
    no push for them):
    - R2-2 (Low, round 2): false asks on the git-… words (his item 4's words).
    - R2-3 (Low, round 2, older than this change-set): in the area of kit-v3-3's stated limit L9.
    - R3-3 (Low, round 3): false refusals in one shape, behind a program that is not read-only.
    - R3-4 (Low, round 3): a few of git's options are missing from a table; they fall to an ask, never a pass.
    - R3-5 (Low, round 3): one value form has no test row.
    - R3-6 (Low, round 3): false asks in one shape.
    - R4-2 (Low, round 4): a cost class in the new check; it fails closed (a guard that runs out of time blocks the
      call: SA-R2-2's answer, below).
    - ADV-R4-2 (Low, the advisor agent): the plan's "no deny or ask became a pass" against main does not hold on the
      wider sweep: 91 lines are looser than main, all older than round 4, by kind as his item 2 of 2026-11-26,
      14:09:28 UTC has them:
      "(the 6 that now pass and cannot start their program; the 85 that were refused and now ask me)"; subagents are
      still refused on the 85; the report lists them by kind.
    - ADV-R4-4 (Low, the advisor agent): one check holds today only because another rule asks first; no gap today.
    - SA-c (Low, round 1): a reading case, recorded, not fixed.
    - SA-e (Low, round 1): lines that main refused now ask instead in a session (never a pass; 121 in round 1's sweep).
    - Shell function definitions (round 1): not inspected; recorded, not fixed.
  - Closed at this change-set's merge, from guard-env-fix (#69): GEF-SAME (what is left of its class: R4-1) and SA-R1-3
    (a line at the turn limit asks); CAP decided, his (a) of 2026-11-25, 16:34:18 UTC. SA-R2-2 answered (the plan,
    section 4, from Claude Code's hooks reference): a guard hook that runs out of time blocks the call, because the
    guard's three hooks set `"onFailure": "block"` (Claude Code 2.1.295 or later; this container 2.1.332; an older
    Claude Code would let the call run); nothing changed, his word first.
  - Closed in this change-set for every form the reviews ran (ids only; what round 4 left open is R4-1): SA-d (Critical,
    older) with SA-D-DIRECT (his (a)), SA-a and SA-g (Medium), SA-b, SA-f and SA-h (Low), the plugin review's High, R2-1
    (raised to Critical by the advisor agent), R3-1 and R3-2 (Critical, by his R3-WAY (a)), ADV-R4-1 (Critical, older),
    ADV-R4-3 (Low).
- **Moved from STATUS (2026-11-24)** (by lane 2's docs pass for the preview hosting check, under his OLD-LINES: lines
  about merged pull requests, moved by script, byte for byte): STATUS's Follow-ups lines 78-93 at main 28254a7, the
  four bullets of the merged pull requests #49, #50, #51 and #48 for the gate step (2.3), are the four bullets below.
  Their pointers to "the archive's" blocks lead to the blocks above.
  (Change-set 3b's docs pass moved the same four bullets by script on 2026-11-25, before it had main's copy; the
  merge of main into its branch keeps them once, under this header.)
- **From status-diet (PR #49), after the gate step:** the STATUS citations left in advisor.md (:17, :286),
  netcode-architect.md (:24) and the decision docs (one hop through the table below until then); SD-N1, SD-N2, SD-N3
  and SD-N6 (security-auditor) and SD-L3 (the verifier): the archive's "Follow-ups", "From status-diet".
- **From foundation 2.1a (PR #50), for the gate step (2.3):** IE-1, IE-5 (Medium: CI's live mirrors), TE-2, TE-7,
  R2-1, A-1, N2, L-4 and the v2 root page against pre-move bases; for the drafts' next revision, five "before the
  foundation" lines left without a pointer; the archive's "Follow-ups", "From foundation 2.1a".
- **From foundation 2.1b (PR #51), for the gate step (2.3):** the Node 24 installer hang (his words: DECISIONS.md,
  2026-11-11, 20:44:29 UTC); the gate's stray-page check stops after the merge (security-auditor's F4, Low, open);
  PROMPTS-2.3 and the guard's `git -C` refusals (the 2.1b report, "Findings for 2.3").
- **From foundation 2.2 (PR #48), for 2.3:** AUTO-ASK, MANUAL-MODE, GIT-C, ADVISOR-TOOL-WORDS, MODELS-2026-11-12,
  REFUSED-PUSH, PIN-GUARD, G0, OVERFLOW-XCRATE-CI (or the first crate to depend on sim), TSCONFIG, REQUIRED-CHECK,
  SAVER-SKIP-CI, CRATE-NAMES, CI-ENFORCED-LATER, WASM-ARTIFACT, TWO-PLAYWRIGHTS, TOOLKIT-RESOLVES-UP, CARGO-NET-OFFLINE,
  CRATE-LICENCE, CUT-CARRY, CANCEL-ON-MAIN, LINT-TABLE-PEDANTIC, FEATURE-UNIFY, ROOT-WORKSPACE, CLIENT-NET-RULE,
  NODE-26; later: OVERFLOW-PER-CRATE (with the first third-party crate), PACKAGE-ENTRIES, EMPTY-SLOTS, OQ-0002-8-TEST,
  BRIEF-STALE. Accepted as left open (his go for #48, item 5): SETPRIV-HARDEN, SETUP-NODE-CACHE, CUT-ESCAPE (step 2.3,
  each a CI edit that asks him); OVERFLOW-PANIC-RULE (with the first code in the room crate or a server crate).
- **Moved from STATUS (2026-11-25)** (by change-set 3b's merge of main 09faefa7, under his OLD-LINES: lines about merged
  pull requests, moved by script, byte for byte; 3b's report lists each line): STATUS's Follow-ups bullets for the
  merged pull requests #60, #61 and #64 are the three bullets below, moved so that STATUS holds both lanes' rows within
  its 120 lines. Their pointers to "the archive's" blocks lead to the blocks above.
- **From #60 (Mediums: with the quality gate step's kit changes, his go):** security-auditor's F1-F5, test-engineer's
  F1-F11, infra-engineer's L1-L6; his list for that plan: a CI lockfile check, the cut's part 2, KIT-REMOVED, ":" paths.
- **From change-set 2 (#61)** (3a's marked in the archive): for 3b NA-N1, -N2, -N4 to -N7, EA-F9, EA-F10's bindings
  part, EA-F13's one-writer item, TE-NT7, PA-N2; for 4 NA-N3; for 5 EA-N5, GE-N1, -N4 to -N6, PA-L2; for 5 and 6 GE-F7,
  PA-L4; for the gate step SA-F6, SA-SWAP, TE-NT2; drafts EA-F6, EA-F12; his four things (SLICE-FEEL, LANE2-LIST,
  PROJ-DRAW, FIREFOX); his check's readings (BR-GAP, not explained; BR-LATE, NOT-SHOWN, FF-WAKES, ADAPTER, TQ-FF).
- **From CI-WASM (#64)** (the archive's block for #64): TE-COUNT, TE-TYPE-PLANTS, TE-TSC-LINES, TE-WATCH, TE-CPU,
  TE-HIDDEN-FRAMES, IMAGE-CHROME, CUT-PATH, TOKEN-WORDS, GH-TOOLS after 3a, SAVER, MANUAL-MODE's record value.
- **Moved from STATUS (2026-11-25, lane 2)** (by lane 2's merge of main 3ab9eaec, the merge of #68, into #67's branch,
  under his OLD-LINES: lines about merged pull requests, moved by script, byte for byte; #67's report lists the lines):
  STATUS's Follow-ups bullet for the merged pull request #66 is the bullet below, moved so that STATUS holds both
  lanes' rows within its 120 lines. Its pointer to "the archive's block" leads to the block "From the preview hosting
  check" above.
- **From the preview hosting check** (lane 2; the archive's block): for step 2.6's plan PHC-CLICKLESS, PHC-LOCKFILE,
  PHC-TOKEN-SETUP, PHC-PROBES, PHC-DELETE, PHC-ACTIONS-READ, PHC-ERROR-PATH; at the merge PHC-TR-END; PHC-SCRIPT-LOWS.
- **Moved from STATUS (2026-11-25, change-set 4a's docs pass)** (under his OLD-LINES: lines about merged pull requests,
  moved by script, byte for byte): STATUS's Follow-ups bullet for change-set 3a (PR #65, merged 2026-11-24 as 28254a7)
  is the bullet below. Its pointer to "the archive's block" leads to the block "From change-set 3a" above; since #71's
  merge (07d673b) 3A-GEN-CHECK and 3A-RULES-STEP's CI half are done (the block "From ci-after-3a" above), and change-set
  4's plan (section 15) places 3A-FMA-CHECK and 3A-FEATURE-VERSION in 4a and 4b, 3A-SCAN-HELPERS in 4c, and
  3A-PART5-CLASH in 4b's step 3 if the content's types are the first that nest.
- **From change-set 3a** (the archive's block): 3A-FMA-CHECK (his: first outside crate), 3A-RUST-REPORT (his),
  3A-PART7-POINTER; for ci-after-3a 3A-GEN-CHECK, 3A-RULES-STEP (below); built in 3b, closed by its merge
  3A-FROM-F64, 3A-HOOK-TEST, 3A-MANIFEST-COMMENT; for the gate step 3A-LINT-MULADD, 3A-SCAN-NAMES, 3A-CONFIG-FILE,
  3A-PEDANTIC; later 3A-FEATURE-VERSION, 3A-SCAN-HELPERS, 3A-PART5-CLASH, 3A-NAME-SECTION, 3A-RNG-COPY, 3A-STATS-ROWS,
  3A-PLANTS, 3A-FMA-LOG, 3A-NO-FMA, 3A-OTHER-BROWSERS; for the drafts 3A-DEPARTURES.
- **Moved from STATUS (2026-11-26, ci-preview-handover's docs pass)** (under his OLD-LINES: lines about merged pull
  requests, moved by script, byte for byte): STATUS's Follow-ups bullets for guard-env-fix (PR #69, merged 2026-11-25
  as 311c048) and ci-after-3a (PR #71, merged 2026-11-25 as 07d673b) are the two bullets below. The second one's
  pointer leads to the block "From ci-after-3a" above; its "TOKEN-WORDS deferred again" is done on ci-preview-handover's
  branch (the block "From ci-preview-handover" above).
- **From guard-env-fix (#69):** GEF-SAME (the same kind of gap in other wrappers and keywords the day guard strips; the forms are with Mustafa, kept out of the repository), SA-R1-3 (the wrapper loop's turn cap), SA-R2-2 (very long lines against the hook timeout), V-2 (the folder tracking after a split-string value), the reason-text nit.
- **Moved from STATUS (2026-11-26, guard-wrappers' docs pass)** (under his OLD-LINES: lines about merged pull requests,
  moved by script, byte for byte, to keep STATUS within its 120 lines): STATUS's Follow-ups bullets for change-set 3b
  (PR #68, merged 2026-11-25 as 3ab9eae), change-set 4a (PR #70, merged 2026-11-26 as 2483e9a) and ci-after-3a (PR
  #71, merged 2026-11-25 as 07d673b) are the three bullets below. Their pointers to "the archive's" blocks lead to the
  blocks above; where those blocks say "a draft" or "no pull request yet", the merges named here came after.
- **From change-set 3b** (#68; the archive's block): for 7 3B-LATE-BOUND (no room on a network without it), 3B-PLAUSIBLE;
  before online play 3B-ACKS, 3B-OUTAGE-256; for 4c, 8 and 9 3B-ONE-PER-TICK; for 5 3B-ALLOC-METHOD (PA-N2 carries over);
  for PM-0002-13 and -8 3B-KEY-POST, 3B-SATURATED; for Slice 3 3B-REFUSED-FIELD; his: 3B-LONG-HOLD with 3B-OQ6-POINTER at
  ADR 0001's next edit, 3B-RELEASE-DRAFTS; design: 3B-PACER-255; for 4c 3B-RESUME-FLASH, 3B-POOL-16, 3B-CSS-PROD,
  3B-BLUR-TEST; 3B-OTHER-BROWSERS (FF-DRAW); later 3B-PUB-MODS; for the link simulator's change-set 3B-HOSTILE-ALLOC (by
  4a's size ladder, told to him in STOP-4A) and 3c's link items; 3c's recording items met by 4a, closed at its merge.
- **From change-set 4a** (#70; the archive's block): for 4b 4A-NA-4B, 4A-EA-4B; for 4c N5, 4A-BYTES-LENGTH, E2-2,
  4A-TE-4C, 4A-NA-4C, 4A-EA-4C, security-auditor's C4; for 4c or 8 E2-3; for 7 C1 (binding from Slice 3's first build that
  talks to a real server); for 8 N4, C2; Slice 3 C3; the gate step 4A-S3-READ; 4A-FIREFOX-REPLAY (his block of 2026-11-26, 13:20 Amsterdam; done on ci-preview-handover's branch); 4A-HASH-REVISIT.
- **From ci-after-3a** (the small CI pull request; the archive's block "From ci-after-3a"; each closed at its merge): done on its branch E1, 3A-GEN-CHECK, 3A-RULES-STEP's CI half (its target-gating rule stays open), the Firefox half of FIREFOX and 3A-OTHER-BROWSERS in CI on Linux (the rule tests only; WebKit and Safari open); still open 3B-OTHER-BROWSERS (its mark is a drawing test); new: FF-DRAW (his question first), FF-SPEC-COMMENTS, FF-FILEURL-PREF, FF-STOP, FF-SANDBOX, FF-ORDER; CUT-PATH's new evidence; TOKEN-WORDS deferred again.
- **Moved from STATUS (2026-11-26, guard-wrappers' docs pass, the same rule)**: STATUS's Follow-ups bullet for ADR
  0006's revision (PR #67, lane 2, merged 2026-11-25 as c1da1bd) is the bullet below, moved by script, byte for byte.
  Its pointers lead to the block "From ADR 0006's revision" and to "Moved from STATUS (2026-11-25, lane 2)" above.
- **From ADR 0006's revision** (#67, lane 2; the archive's block): ADR6-ROLLBACK (Medium), -WK-ROUTE, -WK-BIC,
  -PAGES-RETENTION, -PM7-PAGES, -PM7-FIGURE, -WORDING, -REVISIT-WORKERS; step 2.6: STEP26-WORKERS, PHC-PREVIEW-NOINDEX.
  #66's follow-ups (PHC-CLICKLESS and the rest, for step 2.6): the archive's "Moved from STATUS (2026-11-25, lane 2)".
- **From change-set 4b-1, "Content"** (lane 1, the chat he calls Builder V4; branch `claude/cool-pasteur-oiqk1r`
  standing in for `cs/4b-content`; no pull request yet; risk tier 1, no browser check; reviewed, two rounds, no
  Critical or High (rust-reviewer, engine-architect and security-auditor in both, test-engineer's read in round 2);
  final code 2039e3c; the planted defects' whole run on the final code caught 35 of 35, the 1 control held
  (`.checkpoints/cs4b-content/plants-4b1.txt`); report `docs/reports/2026-11-26-cs4b1-content.md`. Written by 4b-1's
  docs pass, first part (before the reviews) and second part (after them), 2026-11-26. The 4B- ids are this docs
  pass's, not his; each item names its source and who found it. The notes files named here leave the tree at the pull
  request's close and stay readable at the branch's commits):
  - 4B-AIM-REACH (fixed at the plan's re-read, its text as given to this docs pass; his DEPART-4 (a), entry 6's note, the
    entry of 2026-11-26, 12:21:17 UTC: "STATUS names the change-set that widens it"): the loader refuses a playable map
    beyond the aim's reach (x and z plus or minus 327.67 m, y 0 to 655.35 m). The aim widens in the first change-set
    whose map leaves that square (none in Slice 1; Slice 2's plan says whether its maps do), at the latest in Slice 3's
    change-set that sets the command codec for online play (PM-0001-51), beside 3B-ACKS and 3B-OUTAGE-256. No numbered
    change-set holds it yet. Built in 4b-1: `crates/content/src/map.rs` refuses bounds beyond `AIM_REACH_CM` (32,767)
    on x and z or above `AIM_HEIGHT_MAX_CM` (65,535) (`BeyondAim`), and `crates/devtools/tests/aim_reach.rs` fails by
    name when the aim point's field widens [verified 2026-11-26 at 2039e3c: `map.rs:35-38`, `aim_reach.rs`'s header].
  - 4B-FEATURE-POINTER (his FEATURE-COMPARE (a), the entry of 2026-11-26, 14:09:13 UTC, point 4, which ends "Recorded as
    a dated pointer for ADR 0002's next revision."): the two named exceptions to the build's check that the simulation's
    features are the same in every build, with his conditions in that entry, are a dated pointer for ADR 0002's next
    revision. The exceptions themselves are built in 4b-2 (step 6), not in 4b-1.
  - 4B-DEPART-3 (the re-read sheet's section H, `.checkpoints/cs4b-content/reread-sheet.md` lines 185 to 190): the dated
    pointer for the plan's departure DEPART-4 entry 3, at the drafts' next revision, in this order. (1) Entry 3 as he
    accepted it (DEPART-4 (a), the entry of 2026-11-26, 12:21:17 UTC), the plan's own text
    (`.checkpoints/cs4-walk/plan.md` lines 694 to 697): “DEPART-3, which he accepted for change-set 3, said of the
    network draft (its line 221): "three limits are code, not data, until the content crate exists in change-set 4".
    The crate exists from 4b, and the three limits stay code until change-set 7, which rebuilds the room's command
    scheduling behind one seam, so that they move once (engine-architect's proposal; netcode-architect confirms it at
    4b's re-read)”. (2) netcode-architect confirmed it at 4b's re-read (`.checkpoints/cs4b-content/netcode-architect-read.md`,
    its CONFIRM of change-set 7). (3) netcode-architect's sharper sentence, netcode-architect's and not his, as the
    sheet sets it out: “...stay code until change-set 7, which rebuilds the room's command scheduling behind one seam
    and the client endpoint's sending; there they become content values within capacities the code keeps, with
    defaults equal to today's 8, 8 and 2”. If that sentence should become the departure itself, it goes to him as a
    question (the sheet's own rule).
  - 4B-PM5-3 (the builder's reading, for his acceptance at the drafts' next revision): ADR 0005's PM-0005-3 asks for the
    corpus "written back as the same text as its input"; 4b-1 reads it with two stated exceptions: -0, written "0"; and
    a corpus input that is not already canonical, written back as the engine writes it [verified 2026-11-26 at 2039e3c:
    `crates/content/src/number.rs:263`, the pair `(-0.0, "0")`].
  - 4B-CI-CONTENT (two requests for the CI Builder; this chat writes no CI file; found by rust-reviewer, RR-2, and
    security-auditor, SA-5, in round 1, and test-engineer, TE-8): (1) a clippy step in the `toolchain` job, so that
    the content crate's lint guard is held by CI and not only by the source scan and by hand (security-auditor: not
    later than 4b-2's merge); (2) `-p content` in the job's release test line. The million-input fuzz run DOES run in
    CI: the release line holds `-p devtools`; only clippy and the content crate's own release tests run by hand
    (test-engineer's TE-8, correcting the first part of this docs pass, which said otherwise) [verified 2026-11-26 at
    2039e3c: `.github/workflows/toolchain.yml:162` names `sim`, `protocol`, `room`, `client-endpoint`, `sim-wasm` and
    `devtools`, not `content`; no `clippy` in the file].
  - 4B-OLD-TIMERS (game-designer's note for the builder, `.checkpoints/cs4b-content/game-designer-durations.md` lines 91
    to 93): some of the old game's timers are typed into its functions and are in no table (a bot's lunge cooldown
    2.5 s, fire locks of 0.1 to 0.2 s, among others), so the time unit's report cannot see them; 4d's import and
    change-set 8 meet them.
  - 4B-TIME-REEXPORT (engine-architect, EA-4, round 1; for 4b-2's step 6): the simulation re-exports the content
    crate's time unit when `sim` first depends on `content`; until then the number 3,000 stands twice, held equal by
    `crates/devtools/tests/time_unit.rs` [verified 2026-11-26 at 2039e3c: `crates/sim/src/time.rs`'s header says so].
  - 4B-FACING-ZERO (engine-architect, EA-6, rounds 1 and 2; for 4b-2's step 8, before the first map file is written):
    which way a spawn's `facing_deg` of 0 points is pinned then; a spawn's `at` is pinned now (the point on the ground
    between the feet: `map.rs`'s `Spawn`). By its round 2 the line must state that 0 faces +z and degrees grow toward
    +x (the old game's own convention), and how the simulation reads an angle without a std sine is chosen before the
    first map file, since a tighter grid later is a version step
    (`.checkpoints/cs4b-content/engine-architect-review-4b1-round2.md`).
  - 4B-SPAWN-SLOTS (security-auditor, round 1, its condition on the plan's departure 9, the loader asking for at least
    one spawn only; for 4c): 4c's match setup refuses, by name, a map with fewer spawns than the match has slots.
  - 4B-PLAN-4B2 (engine-architect's notes, no findings; for 4b-2's plan): step 9's TypeScript cap of 8,388,608 bytes
    needs its own tie to the Rust cap; the bundle's outer shape has no Rust struct for the type writer; `f64::round`
    (`map.rs`, `time.rs`) enters the players' file at step 9, so step 7's libm rule must allow it or the code rounds
    in whole numbers [verified 2026-11-26 at 2039e3c: `map.rs:225`, `time.rs:33`].
  - 4B-FUZZ-REACH (security-auditor, round 1; a stated limit): the fuzz run cannot reach the caps (its inputs are a few
    kilobytes), full map lists, more than one map in a bundle, or long strings, and it measures no coverage; the caps
    are held by the seven limit cases instead.
  - 4B-REFUSAL-TABLE (test-engineer, TE-7, Low): one sentence per refusal pinned in a table; today one refusal's
    sentence is pinned, the others only by the digests, which the by-hand rewrite replaces without showing what
    changed.
  - 4B-SCAN-CFG-TEST (engine-architect, round 2; for 4b-2's steps 6, 7 and 9): the source scan does not see a file
    under a file-level `#![cfg(test)]` as test code (it fails closed), and step 9 plans one in the content crate
    (`ts_export.rs`); when the build lints of steps 6 and 7 land, their hashed-map and clock half must cover the
    content crate too.
  - 4B-OLD-READERS (security-auditor, round 2; outside this change-set): two older readers of committed vectors go
    around the plain read, `packages/sim-worker/boundary/rules.test.mjs` and
    `packages/e2e/tests/rules/wasm-rules.spec.ts` [verified 2026-11-26 at 2039e3c: `rules.test.mjs:15` reads the vectors with `readFileSync`; `wasm-rules.spec.ts:33` hands Playwright a vector by its path with `route.fulfill`].
  - 4B-MEMORY-UNIT (the verifier, on the final code; for 4b-2's first commit): the comment of
    `packages/sim-worker/boundary/content.test.mjs` gives the test module's memory as "49 MiB"; it is 49.2 million
    bytes, 46.9 MiB. The asserted ceiling (64 MiB) is right; only the comment's unit is wrong, and it stays until
    4b-2 so that 4b-1's reviewed code does not move for a comment.
  - Closed by 4b-1, at its merge: 4A-S3-READ's read side (the block "From change-set 4a" above, its dated pointer).

## Decisions pending: the rest
_The advisor agent's rulings (not Mustafa's), the five carry-forwards, and the old ADR and design lines, moved from
STATUS's "Decisions pending"; his decisions and the queue of his words are in `docs/DECISIONS.md`._
- **Taken by the advisor agent for guard-wrappers** (not his; its notes `.checkpoints/guard-wrappers/advisor-plan.md`
  (readable at commit 68b91211f6336a9308a7b420d205854026eac1dd); no form named): the plan APPROVE with 9 conditions (plan section 7); round 1's fix
  pass APPROVE with conditions, SA-D-DIRECT approved in advance if he said yes (he did, (a), 2026-11-25, 19:36:42 UTC);
  R2-1 raised from Medium to Critical (an agent's kit write passed with no refusal, the class of SEC-U7B-1 and SA-d), so
  a third round on what changed, with SA-g's refinement riding and R2-2 not; round 4's plan change: the round-4 build
  approved only together with ADV-R4-1 (Critical, older), closed in the reviewed build as his line (a) says (asks,
  agents refused, silent only when a read-only program follows) for the forms the reviews ran; INPUT-RUN a named limit
  (the class of stated limits L1-L4), a follow-up listed in the report as a security finding left open, so his go covers
  it knowingly; ADV-R4-2 and ADV-R4-4 Low follow-ups; ADV-R4-3 rides; the order of the edits (the guard in three Edits,
  the tests in four, each asked of him, one commit); main brought in from one separate detached worktree, the main
  folder not moved (a fast-forward of the main folder to a commit built in a worktree is not one of the three moves
  CLAUDE.md's Never line allows, and the advisor cannot widen it); the final build APPROVE (a plan-change review, not a
  round); for the notes, the report names the full SHA of the last commit that holds them, STATUS gets one follow-up
  pointer, and the pull request merges with a merge commit.
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

_From STATUS's "Routines", moved here on 2026-11-23 by docs-keeper's script, byte for byte (the CI-WASM docs pass, PR #64): the two routines as read on 2026-11-10 at 20:10 UTC; the "next" runs they name have passed, and no read of the routines has been recorded since. STATUS keeps their state and schedules._
- **Night shift** (trig_01QpV6y6UaG1Na2q3jumRDxh): enabled; `CRON_TZ=Europe/Amsterdam 10 0-6 * * *` (hourly, 00:10 to
  06:10 Amsterdam), set 2026-11-10 18:00 UTC (daily at 06:00 from 2026-11-07 00:02 UTC until then); Opus 5.5; last run
  2026-11-10 05:04 UTC, succeeded; next 2026-11-10 23:10 UTC.
- **PR Advisor** (trig_01YPyWAsQmsaQoydhMGG3CkJ): enabled again 2026-11-10 16:07:30 UTC, after #46's merge (paused
  2026-11-09 18:22 UTC by Mustafa); `CRON_TZ=Europe/Amsterdam 45 7,13,19 * * *`, a schedule only, never pull-request
  events; Opus 5.5; runs 16:07 UTC (succeeded) and 18:46 UTC (failed: the usage limit); next 2026-11-11 06:45 UTC.

## Open bugs and risks
- **guard-wrappers** (the day guard, `.claude/hooks/routine-guard.mjs`; waiting for his go; by his R4-END (1) what is
  approved merges with these open, on his go; class words only, every form out of the repository by his words of
  2026-11-25, 16:34:18 UTC, item 3e, and 2026-11-26, 14:09:28 UTC, item 1):
  - R4-1 (Critical, open; security-auditor's round 4 on 83bc2417, the last round): one more spelling hides git, gh or
    xargs from the guard's reading, for a session and a subagent; part is this change-set's class, part older; open on
    main too; its fix is the next guard change-set (his order of 2026-11-26: after MAIN-WALL, together with GUARD-KIT;
    its plan says whether it can be a rule that fails closed). Until that fix merges, his interim rule:
    "git and gh always start their own command, with no other program in front of them."
  - INPUT-RUN (a named limit; the advisor agent's ruling of 2026-11-26): programs outside the guard's wrapper table that
    run their input are not inspected, the class of kit-v3-3's stated limits L1-L4; listed in the report as a security
    finding left open.
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
  3. The inventory's line numbers hold after phase 1's move: `git mv` kept the bytes and the line numbers (foundation
     2.1b, PR #51: five pure renames, the blobs equal at 76b4f7a and at the move commit c090843), so its citations hold
     with the prefix `legacy/` (`shamble-game.js:N` is `legacy/shamble-game.js:N`) [verified 2026-11-11].
- **ci-preview-handover, the preview hand-over step** (2026-11-26, no pull request number yet; the evidence: `plan.md`,
  section 6, and `security-auditor-ci.md` in `.checkpoints/ci-preview-handover/`; the follow-ups: the block "From
  ci-preview-handover" in "Follow-ups"):
  1. The upload runs after the cut on the build the cut's code wrote: CUT-ESCAPE, accepted (his words of 2026-11-26,
     15:05 Amsterdam, point 6), its acceptance now also resting on lane 2's preview treating the build as data
     (PH-SA-2); its fix is CUT-ESCAPE-FIX.
  2. The artifact `preview-site` can be downloaded by any signed-in GitHub user for 7 days: the production page only,
     which the preview publishes anyway (the plan's risks; security-auditor: no finding).
  3. No consumer yet: lane 2's `.github/workflows/preview.yml` is on its own branch, not in this branch's
     `.github/workflows/` (gate.yml and toolchain.yml only [verified 2026-11-26]); a `workflow_run` starts only from
     main's copy, so no preview starts until lane 2's file is on main (infra-engineer).
- **Change-set 4a of the v2 build plan, "The recording"** (2026-11-25, PR #70, a draft; the evidence: the chat's card
  and `round1-fold.md` in `.checkpoints/cs4a-recording/`, folded into 4a's report before the pull request is ready):
  1. The tests that read the recordings still follow a link (4A-S3-READ): CI cannot go green with one, since the folder
     check refuses it; the repository-wide check is a gate change for step 2.3.
     _Dated pointer, 2026-11-26 (4b-1's docs pass): closed on 4b-1's branch by its step 4, at its merge; the
     repository-wide check stays for step 2.3 (the follow-up 4A-S3-READ's dated pointer, "Follow-ups")._
  2. Two planted divergences are shown in a weaker form: plant E (a WebAssembly-only difference) is a stand-in, and
     the search for a sine input where the two targets differ was not run; plant D (an outside draw) was planted
     natively only. Their real forms run again on movement in 4c (4A-TE-4C).
  3. In CI a red replay names the step; the differing field comes from the `replay` tool's two state prints, by hand.
  4. Size: 4a ran at 2.2 times its designs' count before review fixes (3,030 written lines against 1,365); by the
     chat's reckoning a part that grows as 4a did ends 2 to 4 % under its stop line before review fixes and 5 to 7 %
     over it after them (4a's fixes added 9 %), so 4b, 4c and 4d are each likely
     to come back to him; their stop lines stay as they are (his STOP-4A, point 3).
- **Change-set 3b of the v2 build plan, "The loop closes"** (2026-11-25; the evidence: its card and reviewers'
  notes at the notes' last commit 80a51e42, and its report `docs/reports/2026-11-25-cs3b-the-loop-closes.md`):
  1. The room has no bound on how late a command may run (3B-LATE-BOUND): harmless while nothing moves and play is
     local, but no room goes behind a network transport without it (change-set 7).
  2. Releases are not counted on the wire: the page works them out from presses and held keys (RELEASE-COUNT,
     decided (a) by him on 2026-11-25); no part of the presses mark is called passed (3B-RELEASE-DRAFTS).
  3. The presses mark and the loop are shown in Chromium only (3B-OTHER-BROWSERS); the decoder mark PM-0001-34 has
     started, never passed (its row names a fuzzing tool that has not joined).
- **Change-set 3a of the v2 build plan, "Rules and boundaries"** (merged 2026-11-24 as 28254a7, PR #65; written while
  in flight; not the legacy 3a below; the evidence: its reviewers' notes at f153ddf4 and its report
  `docs/reports/2026-11-24-cs3a-rules-boundaries.md`):
  1. MUL-ADD: on the pinned toolchain `f32::mul_add` gives other bits in WebAssembly than natively wherever the result
     lies below the smallest normal `f32`. By his MUL-ADD (a) (2026-11-24, 15:58:25 UTC) it stays banned in the
     simulation, and its 16 WebAssembly values are pinned in the float vectors as a known defect. A call from an
     outside crate is not caught until 3A-FMA-CHECK (change-set 4), and a native server without FMA would give the
     WebAssembly values (3A-NO-FMA).
  2. The scan of `crates/sim/src` reads text, not types: it refuses some valid code (3A-SCAN-HELPERS) and misses a
     macro that builds a call itself (its doc says so); it is interim until the quality gate step's lints.
  3. CI does not check that the vector files match their generators (3A-GEN-CHECK, a CI edit).
     _Pointer, 2026-11-25: CI checks them on ci-after-3a's branch (3A-GEN-CHECK, marked done there); closed when that
     pull request merges._
- **The preview hosting check (ADR 0006's spike 1, SPIKE_PLAN entry 6)** (lane 2, in flight, 2026-11-24; its report
  `docs/reports/2026-11-24-preview-hosting-check.md`, "Risks"; the ids are this docs pass's):
  1. PHC-HEADER: the header mark (PM-0006-2) is not met as written: production's one-off links (0354374e and
     de1c3e5e) carry the "keep out of search engines" header (`X-Robots-Tag: noindex`), which the host adds to every
     one-off address; production's own address stays clean. By ADR 0006's "When a mark fails" (ADR 0006 :1404-1416)
     a failed mark goes back to the draft: infra-engineer revises the part the row names by the draft's own routes,
     and the revised part returns to the advisor and to Mustafa and the Advisor with the measured numbers. No word of
     ADR 0006 changes in this pull request. Whether step 2.6 waits for the revision is his decision (ONE-OFF-HEADER).
     _Pointer (2026-11-25): the header row is revised by #67 (ADR 0006's small revision,
     `docs/reports/2026-11-25-adr0006-header-row.md`), waiting for his go; by his ONE-OFF-HEADER (a) (2026-11-24,
     23:53:15 UTC) step 2.6's first edit waits for his go on the revised row._
  2. PHC-PROBE-MAIL: the test repository's hourly probe reads upload records kept 14 days; the last expires on
     2026-12-02 (about 18:35 UTC). After that every scheduled run ends red and GitHub mails the failure to Mustafa's
     account; GitHub's docs do not say whether archiving a repository stops its scheduled runs, while switching the
     workflow off does. So the probe is switched off before 2 December (his to do).
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
- Deferred, low (kit-v2-1): `tools/gate.mjs:333` and `tools/README.md:34` still call `gate-globals.json` "reviewed data, not a gate tool", which contradicts kit-v2-1's category 3 (gate files need Mustafa's go). It errs on the safe side; fix the wording when gate.mjs is next touched. **Fixed** in dd12aeb (foundation 2.1a, PR #50; on main once #50 merges): both now call it a gate file whose change needs Mustafa's go [verified: `tools/gate.mjs:355`, `tools/README.md:35`]
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
| v2, the first page (change-set 2, PR #61, merged 2026-11-19; his Brave and Firefox readings: `docs/reports/2026-11-19-cs2-browser-check.md`): the Wasm's size, the Worker's tick pacing, the hand-off's cost per block size, the page's allocation per frame and per block, the backend | in its report, from the container's headless Chromium on the WebGL 2 fallback with software rendering (no WebGPU adapter, no real GPU here): readings, not judged; the final numbers are the ready head's | 2026-11-19 | docs/reports/2026-11-19-cs2-first-page.md |
| CI on main, latest (the rows below are older) | `gate`, `smoke` and `toolchain` success on 2483e9a, PR #70's merge commit (push; gate run 45818982040 with its gate and smoke jobs, completed 12:23 UTC; toolchain run 45818982052, 12:28 UTC) | 2026-11-26 | GitHub's REST API, the commit's workflow runs and the gate run's jobs (read by ci-preview-handover's docs-keeper) |
| CI on main, latest (the rows below are older) | `gate`, `smoke` and `toolchain` success on 2483e9a, PR #70's merge commit (gate 12:22:25, smoke 12:23:01, toolchain 12:28:06 UTC by `completed_at`) | 2026-11-26 | GitHub's REST API, the commit's check runs (read 14:24 UTC by docs-keeper) [verified] |
| guard-wrappers (no pull request yet): the kit's three test files | `routine-guard.test.mjs` 1551/1551 (1247 on main c1da1bd by the chat's count; `.claude/` unchanged from c1da1bd to 2483e9a, read with git diff), `settings.test.mjs` 127/127, `checkpoint.test.mjs` 33/33, on the docs pass's worktree at 54bbf38c (main 2483e9a merged in) | 2026-11-26 | `node --test`, run by docs-keeper [verified] |
| CI on main (older) | `gate`, `smoke` and `toolchain` success on 07d673b, PR #71's merge commit (push; gate and smoke completed 20:49 UTC, toolchain 20:54 UTC) | 2026-11-25 | GitHub's REST API, the commit's check runs (read 22:12 UTC) |
| Change-set 4a (PR #70, a draft): the players' Wasm, the hash's cost, the tests | the players' Wasm 54,514 bytes on the final source fb5d0ff (53,813 before 4a), sha256 a4d8184c...d094, the same on the verifier's rebuild and on GitHub's runner (CI on 21c1046); after round 1's fix pass it was 19847796...f708, one byte other (a line number in a panic message), and 55,607 bytes before that pass; the state hash: native release 4,096 bytes 5.7 us and 16,384 bytes 20.7 us, WebAssembly release in Node 22 5.2 and 19.9 us; 17 browser tests in 195 s in the container; the release tests of all six crates, 139 passed, on the tree with #71 taken in | 2026-11-25 | the chat's card and `round1-fold.md` (`.checkpoints/cs4a-recording/`); not re-measured by docs-keeper, not re-read at fb5d0ff |
| CI on main (older) | `gate`, `smoke` and `toolchain` success on 28254a7, PR #65's merge commit (run ids not recorded here) | 2026-11-24 | change-set 3b's card, step 0 (read 22:45 UTC) [assumed: not read again by this docs pass] |
| ci-after-3a (no pull request yet): the `toolchain` job with every crate's release tests, the two generator checks, `devtools`' wasm32 line and two browser runs | on 832268e: the job 18:34:50 to 18:40:12 UTC (5 min 22 s by the job's start and end; the report gives 5 min 20 s); Chromium "16 passed (3.4m)"; Firefox 160.0 "2 passed (15.0s)" (the rule tests only); Python 3.12.3; gate and smoke green (run 45676587871). The red first run on 52ad5d3 (45670972698): Chromium "16 passed (3.5m)", Firefox "4 failed", "9 did not run", "2 passed (5.0m)" at Playwright's 300 s stop | 2026-11-25 | GitHub Actions, run 45676587814 and 45670972698 logs (gh, read by ci-after-3a's docs pass) |
| Change-set 3b (PR #68, merged 2026-11-25 as 3ab9eae; written while in review): the `toolchain` job on a runner, the players' Wasm and the browser tests | the job 4 min 47 s (0dfca749, run 45529670690) to 4 min 58 s (the final code 6abf3d2a, run 45537884772) of its 15 minutes; CI green on d117fca5 and eb0d2314, and gate, smoke and toolchain green on 0dfca749, 6380df45, a85b9dbb and 6abf3d2a; the players' Wasm 53,813 bytes (21,994 gzip, 18,464 brotli; sha256 5ebae154...b41262) as written before the push of the commit that closes round 2's fixes (31,718 on main); 16 browser tests in 3.3 to 3.4 of CI's 5 minutes; on 0dfca749 in CI: the one-minute loop 1,816 ticks in 60.5 s, key to tick median 167.7 ms and 95th percentile 185.0 ms at about 10 frames a second; the press test 10,000 presses and releases made and echoed, 0 refused; allocation 90 bytes a frame and 597 a block; on 6abf3d2a in CI: the loop 1,814 ticks in 60.6 s, key to tick median 238.1 ms and 95th percentile 257.8 ms at about 7 frames a second, the press test 10,000 presses and releases made and echoed, 0 lost, 0 doubled, allocation 91 bytes a frame and 590 a block; 53 of 53 planted defects caught on 6abf3d2a | 2026-11-25 | change-set 3b's card, "Current step" (CI read line by line by the builder) |
| CI on main (older) | `gate` with `smoke` (run id 45373057216) and `toolchain` (run id 45373056953) success on 1ec6f6c, PR #64's merge commit (push) | 2026-11-24 | GitHub Actions check runs (REST API, read 2026-11-24) |
| Change-set 3a (PR #65, merged 2026-11-24 as 28254a7): the `toolchain` job on a runner, and the players' Wasm | 138 s (a1c59e8) to 185 s (8619d539) per job, from the job's start and end, on the six heads whose times the card records; the players' Wasm with the panic hook 31,718 bytes, sha256 383f5303...3963, the same bytes on the runner and in the container since the remap (RR-F5; 28,991 on the runners before 3a); 14 browser tests in Google Chrome 157.0.8103.78 in about 1.1 min; boundary tests 2 files, 17 tests; the client's unit tests 70; the allocation test 61 bytes a frame and 529 a block (75 frames) on 8619d539, whose runner image moved to 20261116.133.1 (20261116.376.1 before): the verifier's TE-WATCH read found the allocation inside its windows (25 to 170, 430 to 610) and the default backend unchanged (the WebGL 2 fallback); one red, on a1b2e802 (an assertion of the builder's, fixed forward, not re-run) | 2026-11-24 | change-set 3a's card at f153ddf4 (steps 4 to 12); the job log of 8619d539 (job 138592422504) |
| CI on main (older) | `gate` with `smoke` (run 934, id 44551896122) and `toolchain` (run 232, id 44551894194) success on f5030a3, PR #61's merge commit (push) | 2026-11-19 | GitHub Actions runs (REST API, read 2026-11-23) |
| CI-WASM (PR #64, merged 2026-11-24 as 1ec6f6c): the `toolchain` job on a runner | 135 to 147 s per clean job (eight jobs on four heads, push and pull request); the Wasm 28,991 bytes, sha256 22dad2eb...018b, on every runner build (28,987 bytes in the containers: RR-F5, not judged); 12 browser tests in Google Chrome 157.0.8103.78 in about 1.1 min; boundary tests 7, the client's unit tests 49; the allocation test 65 to 76 frames (floor: more than 20) | 2026-11-23 | docs/reports/2026-11-19-ci-wasm.md |
| CI on main (older) | `gate` with `smoke` (run 900, id 44391809186) and `toolchain` (run 198, id 44391809357) success on b763ba3, PR #60's merge commit (push) | 2026-11-18 | GitHub Actions runs (API) |
| CI on main | `gate` and `smoke` success on 44446a5 (PR #44's merge commit) after one re-run: run 42673290304's first attempt lost its `gate` job on GitHub's side (only `smoke` ran, green); attempt 2 green, 06:37 UTC | 2026-11-10 | GitHub Actions runs (API) |
| Gate (official battery, older) | 0 flagged, 44 editor refs all guarded | 2026-10-10 | battery.js |

## Where STATUS's old sections went (STATUS-DIET, 2026-11-10; old line numbers: `git show 2d22b69:docs/STATUS.md`)
| STATUS at 2d22b69 (lines) | Now |
|---|---|
| The header (2-27), Phase (30-139), In flight (142-185), Targets (188-195), Routines (949-1008), Open bugs and risks (1011-1217), Health (1220-1233) | `docs/STATUS_ARCHIVE.md`, under the same names |
| Decisions pending: his blocks, the relayed Advisor's, and the queue of his words (198-272, 281-420, 674-799, 813-846, 880-928) | `docs/DECISIONS.md` |
| Decisions pending: Follow-ups, KIT-ALIGN and DOCS-ALIGN (421-673, 862-879) | `docs/STATUS_ARCHIVE.md`, "Follow-ups" |
| Decisions pending: the advisor agent's rulings, the carry-forwards, the old ADR lines (273-280, 800-812, 850-861, 929-946) | `docs/STATUS_ARCHIVE.md`, "Decisions pending: the rest" |
| Decisions pending: Waiting (847-849) | this file, "Waiting on Mustafa" |

_The section above was moved here from `docs/STATUS.md` (its last section) on 2026-11-18 by a script, byte for byte,
under his OLD-LINES of that day (change-set 1, PR #58, docs pass); in its table "this file" is `docs/STATUS.md`._
