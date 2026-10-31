# Advisor report: docs-after-v2-research (2026-10-31, branch cs/docs-after-v2-research, base main b6effde)

## Goal
The docs catch up with the merge of the R1–R6 research ([#28](https://github.com/Mergincanli/Shamble-Frontlines/pull/28), b6effde) and record the decisions of 2026-10-31, so the decision drafts start from them.

## Outcome
DONE pending verification: docs only, symbols +0 −0; the four checks (V1–V4) and the advisor's review of the final head are still to come.

## Plan and approval
- Plan revision 2 (STATUS, ROADMAP, the research folder, one line of VISION_V2, the Night queue), built from a read-only sweep (two agents: 56 + 18 findings).
- Advisor: **APPROVE with 16 conditions** (2026-10-31, about 14:05 UTC); each condition gave exact text or an exact order:
  - 1–6 (the research edits: C17.b1 merged, C17.b2 as written, C23.b2 with "the per-hostname certificates", the certificate note, four logged-change lines): applied by script in af2560c.
  - 7–12 (the README paragraph, STATUS risk 1, the Targets note, NS-2 out of "Waiting", carry-forwards 2, 4 and 5, the ROADMAP slices line): docs-keeper, word for word, in 05b2d3a.
  - 13 (Night queue item 2): applied to issue #16 at 14:12 UTC. 14 (this body): below. 15 (the merge-path order) and 16 (the Night queue check): pending, in "For Mustafa".

## What changed
- `docs/research/` (r1–r6, GLOSSARY): 39 exact edits, +75 −17 lines: an "Answered" note under each answered shared question (16) and glossary dependency (12 edits); the six changes approved on 2026-10-30 for BASELINE-WHEN (b), so R6-HOST-REC4 now recommends a certificate per game-server hostname (a recommendation change); four logged-change lines.
- `docs/VISION_V2.md` §3, line 43: "(in total: 1v1 to 4v4, and 2v2v2 to 6v6v6; MODES-3 (a), Mustafa, 2026-10-31)" (the Advisor's VISION-LINE (a)).
- `docs/STATUS.md`: #28 merged; the decisions in their words; "Next: the decision drafts" with five carry-forwards; KIT-ALIGN and DOCS-ALIGN; the new night routine; risk 1 closed.
- `docs/ROADMAP.md`: v2 phase 0 and the slice order under Phases; a "v2, next" bullet; Recent work (#28 merged, docs-after-r7 added, this PR).
- `docs/research/README.md`: "Answered since"; the R6 owner cell; "R1–R6 merged in PR #28".
- `docs/REPO_MAP.md`: lines 57 and 79 say where the security baseline goes.
- Issue #16 (the Night queue): three sentences (old and new text below).

## Evidence
- Gate at 05b2d3a: **PASS**, all 8 checks: syntax 5/5 · concatenation · symbols +0 −0 · player build (51 refs, all guarded) · smoke (index.html 8/8, editor.html 5/5, 0 errors) · names · layering · script order. Repo-map check: "repo map OK: 103 entries cover 93 required paths".
- The research edits: a dry run matched every anchor exactly once (39 of 39); the applied diff equals the reviewed preview line for line (75 added, 17 removed).
- Verification workflow (V1 exact edits, V2 facts, V3 decision wording, V4 completeness): pending.
- Not verified here: the routines facts come from the session's routines API reads (no other reviewer can repeat them); the new routine has not yet posted on issue #16.

## Reviews
- Advisor, plan: APPROVE with 16 conditions (all applied above). Advisor, final head: pending (after the checkpoint is deleted, condition 15).
- Verification V1–V4: pending.

## Decisions and deviations
- The report is this PR body, not a file, as the approved plan and the earlier docs-only change-sets have it.
- NS-2: Mustafa created the fresh-session routine; the API refused the session's schedule edit (agents may only edit routines they created), so the schedule is his step; the session switched the old routine off at 13:54 UTC (reversible).
- Left for the decision drafts: R2-BUD-REC6's reason and GLOSSARY §1's 60 Hz sentence (rewording needs judgment), and the sentences listed below.
- docs-keeper's prompt gave two wrong times (14:25 and 14:20 UTC); it was corrected to 14:06 UTC before it wrote the Routines facts.

## Risks
1. The new night routine is unproven at posting: tomorrow morning's comment on issue #16 is the first proof; until Mustafa edits it, it runs once a night.
2. The research files changed after their verification: only decision notes and texts the advisor approved (V1 checks nothing else changed).
3. CLAUDE.md:7-8 and the netcode agent's file still read VISION_V2 §3 per team (KIT-ALIGN) until a kit change-set after PR #30.

## Proposed next step
The decision drafts (`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006`), planned from the merged research, the 2026-10-31 decisions and STATUS's five carry-forwards.

## Decisions needed
- **FOUNDATION (the Advisor):** does "the foundation" in the slice order's condition 3 ("the netcode proof test … still runs before the foundation") mean phase 1, that is, the repository restructure, toolchain and CI gate v2 (V2_PHASE0_BRIEF §1)? Why it matters: the decision drafts schedule the proof test against it, and no document defines it. Options: (a) yes, phase 1; (b) something else (say what). Recommendation: (a).

## For Mustafa
1. On the new night routine's page (https://claude.ai/code/routines/trig_01QpV6y6UaG1Na2q3jumRDxh), set the schedule to `CRON_TZ=Europe/Amsterdam 10 0-7 * * *` (it now runs once a night, at 00:10).
2. Tomorrow morning, check that issue #16 has the new night shift's comment; if it is missing, tell the day session.
3. After one good night, delete the old "Night shift" routine (deleting it also deletes its session).
4. Once the advisor has approved this PR's final head, say "merge #31" in the session's chat (docs only: no Brave check).

<details><summary>The decisions of 2026-10-31, word for word (condition 14)</summary>

First message (in the v2 day session's chat, before the merge of #28 at 13:26:30 UTC):

> Mustafa: merge #28.
> - MODES-3: (a), totals; the biggest match is 24v24 = 48.
> - PREVIEW-WALL: (a), private from the first preview that reaches the test game server.
> - SEC-SPREAD: (a), server-only spread.
> - ABL-PREDICT: later, after the Feel Lab test.
> Advisor:
> - BASELINE-WHEN: (b). List it in ADR 0006 as open, with temporary positions. Research the
>   security baseline before the first online playtest, in its own fresh session.

Second message (the same chat, between 13:35 and 13:50 UTC):

> Advisor:
> - VISION-LINE: (a). I wrote that line; the clarification is right. Add it as proposed.
> - The 60 Hz margin (8.28 of 8.33 ms) is effectively zero. For the decision drafts: a 30 Hz
>   server tick is the baseline for 48 players; 60 Hz only if the netcode proof test shows at
>   least 25 % headroom on the target hardware.
>
> Mustafa + Advisor: new slice order: 1 move & shoot → 2 bots and three teams (played locally)
> → 3 online → 4 the loop. Conditions, for the decision drafts:
> 1. From slice 1, the game runs as a client plus a local server (the Rust simulation in a Web
>    Worker), talking through the same messages the network will carry. No offline-only
>    shortcuts.
> 2. Player movement and shooting are driven by inputs and can be replayed from day one, so
>    prediction slots in later without rework.
> 3. The netcode proof test (48 simulated players over WebTransport) still runs before the
>    foundation.
> Record the new order in ROADMAP and STATUS.

"As proposed" (VISION-LINE): add, in brackets, "in total: 1v1 to 4v4, and 2v2v2 to 6v6v6; MODES-3 (a), Mustafa, 2026-10-31" to VISION_V2 §3's line, and change nothing else.
</details>

<details><summary>Issue #16 (the Night queue): the old and new text (condition 14)</summary>

Edited 2026-10-31 at 14:12 UTC; only these three sentences of the body's fourth paragraph changed.
1. OLD: "No game-code changes until the v2 brief, so the old queue is paused below ([Advisor verdict, 2026-10-17](https://github.com/Mergincanli/Shamble-Frontlines/issues/16#issuecomment-6238312490))."
   NEW: "The legacy game is frozen (critical fixes only, CLAUDE.md) and phase 0 writes no engine code, so the old queue stays paused below ([Advisor verdict, 2026-10-17](https://github.com/Mergincanli/Shamble-Frontlines/issues/16#issuecomment-6238312490))."
2. OLD: "kit-v3-1 is being finished in the R7 session, and its PR goes to Mustafa ([Advisor verdicts, 2026-10-27, second message](https://github.com/Mergincanli/Shamble-Frontlines/issues/16#issuecomment-6454223695))."
   NEW: "kit-v3-1 is being finished in the R7 session, in [PR #30](https://github.com/Mergincanli/Shamble-Frontlines/pull/30) (open since 2026-10-27), which goes to Mustafa ([Advisor verdicts, 2026-10-27, second message](https://github.com/Mergincanli/Shamble-Frontlines/issues/16#issuecomment-6454223695))."
3. OLD: "Last updated: 2026-10-30."
   NEW: "R1–R6 research is merged ([PR #28](https://github.com/Mergincanli/Shamble-Frontlines/pull/28), main `b6effde`, 2026-10-31). Next: the decision drafts (not queued: they have no approved plan for unattended work); the security baseline is researched before the first online playtest, in its own fresh session (research runs by day only). Last updated: 2026-10-31."

A comment posted at 13:55 UTC (6544279732, by another session under the Mergincanli account) also announces the night shift's move; this session did not post it.
</details>

<details><summary>Not touched, for the decision drafts (condition 11)</summary>

- R2-BUD-REC6's reason (`r2-netcode.md:2402-2403`) and GLOSSARY §1's 60 Hz sentence (`GLOSSARY.md:73-74`): at N = 48 a 60 Hz tick that encodes and re-floods fits the 50 % budget by 0.05 ms (8.28 of 8.33 ms) while its server-only share is 0.24 ms over its 25 % (4.41 of 4.17 ms); TICK-30 makes 30 Hz the baseline.
- Four sentences that still describe the wildcard certificate as R6's proposal (lines at b6effde): `r2-netcode.md:2703-2705`, `r6-infrastructure.md:47`, `938-940` and `1056`; R6-HOST-REC4 holds.
- The "until Mustafa answers PREVIEW-WALL" sentences inside R5-PREV-REC5, R6-HOST-REC1 and R6-HOST-REC3; R6-HOST-OQ5's "after the security baseline"; the marks that give values for 48 and 54 (48 holds); TICK-30's consequences for the tick-rate marks.
- Rules files (KIT-ALIGN) and the Advisor's documents (DOCS-ALIGN): listed in STATUS, not edited.
</details>

<details><summary>Steps with their commits (the checkpoint, folded in)</summary>

1. PR #28 merged on Mustafa's "merge #28" after the advisor's APPROVE of 854d42f121ab67744040da41a4b2e974e200020c (PR #28 comment 6542571633), gate and smoke green: b6effde, 13:26:30 UTC.
2. The decisions saved word for word; a read-only sweep (workflow, two agents); plan revision 1, then 2: 1bd662d, 9f2c3f5, 83cb459.
3. NS-2: Mustafa's new routine checked (fresh session per run, repository, Opus 5.5, its 13:51 UTC test run); the old routine switched off at 13:54 UTC; the schedule edit refused by the API (Mustafa's step): 83cb459.
4. Advisor plan review: APPROVE with 16 conditions: a5405d2 (prompt), the verdict folded in af2560c.
5. The research answers (39 edits) and the VISION_V2 line: af2560c.
6. Draft PR #31; docs-keeper's part L: 1a1327b, ca4fffd, f180f6e (a work-in-progress snapshot), 05b2d3a.
7. Issue #16 edited (14:12 UTC). Gate PASS at 05b2d3a.
</details>

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_0173vVo5q59ameR6cnN9BENg
