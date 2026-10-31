# Plan: docs-after-v2-research (docs only), revision 2

Revision 1 (commit 9f2c3f5) was a draft written before the read-only sweep came back. Revision 2 folds in the sweep
(`sweep-a-decisions.md`, `sweep-b-merge-state.md`: 56 + 18 findings) and the second message of 2026-10-31. Both
messages are word for word in `decisions-2026-10-31.md`.

## Goal
The docs catch up with PR #28's merge (R1–R6 research, merge commit b6effde, 2026-10-31) and record the decisions of
2026-10-31 where the next step, the decision drafts, starts from: STATUS, ROADMAP, the research folder, VISION_V2 §3
(one line) and the Night queue. The decisions:
- **Mustafa:** MODES-3 (a), totals, the biggest match 24v24 = 48 · PREVIEW-WALL (a), private from the first preview
  that reaches the test game server · SEC-SPREAD (a), server-only spread · ABL-PREDICT later, after the Feel Lab test.
- **The Advisor:** BASELINE-WHEN (b): ADR 0006 lists the security baseline as open, with temporary positions; the
  baseline is researched before the first online playtest, in its own fresh session · VISION-LINE (a): add the totals
  reading to VISION_V2 §3 as proposed · TICK-30: "a 30 Hz server tick is the baseline for 48 players; 60 Hz only if
  the netcode proof test shows at least 25 % headroom on the target hardware".
- **Mustafa and the Advisor:** SLICE-ORDER: "1 move & shoot → 2 bots and three teams (played locally) → 3 online → 4
  the loop", with three conditions for the decision drafts: (1) from slice 1 the game runs as a client plus a local
  server (the Rust simulation in a Web Worker), talking through the same messages the network will carry, no
  offline-only shortcuts; (2) player movement and shooting are driven by inputs and can be replayed from day one, so
  prediction slots in later without rework; (3) the netcode proof test (48 simulated players over WebTransport) still
  runs before the foundation. "Record the new order in ROADMAP and STATUS."

## Branch
`cs/docs-after-v2-research`, from origin/main b6effde, in the main checkout; commits so far are checkpoint-only
(1bd662d, 9f2c3f5).

## Change type and scope
Docs only, symbols +0 −0. Four parts:
- **L. Living docs** (docs-keeper): `docs/STATUS.md`, `docs/ROADMAP.md`, `docs/research/README.md`, `docs/REPO_MAP.md`
  (two descriptions).
- **V. `docs/VISION_V2.md` line 43** (the session): one bracket, as the Advisor decided (VISION-LINE (a)).
- **R. The research files** (the session, by script): 34 exact edits in 7 files, +65 −12 lines.
- **I. Issue #16's body** (the session): three sentences, after this plan is approved.
Out of scope, listed for their owners: the rules files (KIT-ALIGN, below), design inputs other than VISION_V2 line 43
(DOCS-ALIGN, below), and research sentences whose rewording needs judgment (left to the drafts, below).

## L. Living docs (docs-keeper; every fact checked against git, GitHub or the decisions file first; every decision code
explained in plain words on its line; plain words first, internal names in brackets after)
1. **`docs/STATUS.md`:**
   - **Header line 3:** "Last updated: 2026-10-31"; the R1–R6 research merged (PR #28, b6effde); "(PR #28, not merged
     yet)" goes.
   - **Phase, the v2 paragraph:** R1–R6 joins "Done" (PR #28, b6effde, 2026-10-31; docs only); "Done, waiting to
     merge" goes. "Next": the decision drafts (`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006`), starting from the
     2026-10-31 decisions; then a stop for Mustafa and the Advisor; then phase 1 and the playable slices in the new
     order (SLICE-ORDER, below), with the netcode proof test before the foundation.
   - **In flight:** the R1–R6 row becomes "merged: PR #28, b6effde, 2026-10-31, on Mustafa's 'merge #28' after the
     advisor's APPROVE of 854d42f" | done; report `docs/reports/2026-10-31-v2-research.md` | none. A new first row:
     docs-after-v2-research (this PR) | waiting on the advisor's review of its head, then Mustafa's "merge #<n>". The
     kit-v3-1 row names its pull request: "PR #30 (open since 2026-10-27)", waiting on Mustafa's go (sweep B: PR #30,
     head `cs/kit-v3-1` 5be7d37, not a draft, last updated 2026-10-28; the session does not open the branch).
   - **Targets:** a v2 bullet first: VISION_V2 §3 read with MODES-3 (a): start with two teams with 2–8 players in total
     (1v1 to 4v4) and three teams with 6–18 in total (2v2v2 to 6v6v6); the goal 12v12v12 or 24v24; the largest mode
     24v24, 48 players; one faction per team, squads of 4, bots fill empty slots; design for 48+ players (CLAUDE.md).
     The old bullet stays, marked "legacy target (before v2), superseded".
   - **Decisions pending:**
     - New first group, "Taken by Mustafa and the Advisor 2026-10-31": SLICE-ORDER, the order and its three conditions
       in their words (above).
     - New group, "Taken by Mustafa 2026-10-31 (in the v2 day session's chat, with his 'merge #28')": MODES-3 (a),
       PREVIEW-WALL (a), SEC-SPREAD (a) (spread stays known only to the server; tracers use a similar-looking spread
       that the hit corrects), ABL-PREDICT later, after the Feel Lab test.
     - New group, "Taken by the Advisor 2026-10-31 (relayed by Mustafa)": BASELINE-WHEN (b) and its timing;
       VISION-LINE (a); TICK-30 (in its words).
     - **Waiting:** ABL-PREDICT (after the Feel Lab test) stays; NS-2 leaves (carried out, see Routines) unless the new
       routine's schedule is still unset when docs-keeper edits; the answered entries leave. The line "Next
       after PR #28 …" becomes "Next: the decision drafts", with what they carry forward:
       1. The synthesis verifies every ADR claim and number, R3–R6's included (research plan condition 9; the
          advisor's #28 review, finding 4).
       2. R2-BUD-REC6's reason (`r2-netcode.md:2402-2403`) and GLOSSARY §1's 60 Hz sentence (`GLOSSARY.md:73-74`) were
          written with N = 54 in view. At N = 48 a 60 Hz tick that both encodes a snapshot and re-floods fits the 50 %
          server tick budget by 0.05 ms (8.28 of 8.33 ms), while its server-only share is 0.24 ms over its own 25 %
          (4.41 of 4.17 ms) (`GLOSSARY.md:62-66`). Under TICK-30 the drafts restate these marks for the 30 Hz baseline
          and say what the 25 % headroom for 60 Hz is measured against (the advisor's #28 review, finding 2).
       3. Where a research mark gives values for 48 and 54, the drafts use 48; where a research sentence says "until
          Mustafa answers …", the answer applies.
     - New open item **KIT-ALIGN** (rules files the answers left stale; a kit change-set after PR #30 aligns them, with
       Mustafa's go): `CLAUDE.md:7-8` and `.claude/agents/netcode-architect.md:17-19` (the per-team reading of
       VISION_V2 §3); `CLAUDE.md:192`, `.claude/agents/security-auditor.md:14` and `.claude/agents/infra-engineer.md:29-30`
       (the security baseline as part of R6); `.claude/agents/infra-engineer.md:23-26` and
       `.claude/agents/security-auditor.md:46` (preview privacy, costs at the starting scale); optional:
       `.claude/agents/netcode-architect.md:37` and `.claude/agents/security-auditor.md:30` (server-only spread as a rule).
     - New open item **DOCS-ALIGN** (the Advisor's documents; the session does not edit them): `docs/V2_PHASE0_BRIEF.md`
       §1 ("Then: playable slices, as in VISION_V2"; VISION_V2 has no slice list) and line 126 (R6's row still holds the
       security baseline); `docs/LOOP_V2.md:90-91` (tracers and the Feel Lab test); `docs/LOOP_V2.md:19-20` and
       `docs/DESIGN_NOTES.md:34-35` (team sizes written before VISION_V2: "12v12v12 to 16v16v16, possibly more").
   - **Routines** (NS-2 carried out on 2026-10-31; facts from the routines API, read back by the session): the night
     shift is now "Shamble Frontline Night shift" (trig_01QpV6y6UaG1Na2q3jumRDxh), created by Mustafa on the Routines
     page at 13:50 UTC: a fresh session on every run (no persistent session), the repository attached, Opus 5.5, the
     same one-line prompt; its 13:51 UTC test run read `.claude/NIGHT_SHIFT.md` from main and stopped silently outside
     the window. Its schedule reads `CRON_TZ=Europe/Amsterdam 10 0 * * *` until Mustafa sets `10 0-7 * * *` on its page
     (the API refuses agents' edits to routines they did not create). The old "Night shift"
     (trig_019LtRqmAdzddyk6K3e3kDJz, one persistent session) was switched off by the session at 13:54 UTC under NS-2
     (c); deleting it, after one good night, is Mustafa's step (deleting it also deletes its session). docs-keeper
     writes the state as it stands when it edits, read again from the API by the session.
   - **Open bugs and risks, R1–R6 research:** risk 1 becomes "Closed 2026-10-31 by MODES-3 (a): the largest mode is 48
     players, where the marks written for 48 hold." Risks 2–4 stay.
2. **`docs/ROADMAP.md`:**
   - **Phases:** two v2 lines after the legacy list: "v2 phase 0 (V2_PHASE0_BRIEF): IN PROGRESS: kit-v3-0, R7 and R1–R6
     done (PRs #26, #27, #28); next: the decision drafts" and "v2 slices (Mustafa and the Advisor, 2026-10-31), after
     phase 1: 1 move & shoot → 2 bots and three teams (played locally) → 3 online → 4 the loop". Phase 6 ("Multiplayer —
     NOT STARTED") gets "(legacy plan; v2 replaces it)".
   - **Next / open:** a new first bullet: "v2, next: the decision drafts from the merged research and the 2026-10-31
     decisions (STATUS); then phase 1 and the slices in the new order, with the netcode proof test (48 simulated
     players over WebTransport) before the foundation; the security baseline is researched before the first online
     playtest, in its own fresh session."
   - **Recent work:** the R1–R6 entry becomes MERGED (PR #28, b6effde, 2026-10-31, on Mustafa's "merge #28" after the
     advisor's APPROVE of 854d42f), its future-tense sentence goes; a missing entry for docs-after-r7 (PR #29, e9f11be,
     2026-10-27) and a new one for this change-set are added: 11 entries, inside the file's 8–12 rule.
3. **`docs/research/README.md`:** after the opening paragraph, "Answered since (2026-10-31; STATUS, 'Decisions
   pending'): MODES-3 (a), the largest mode is 24v24, N = 48; PREVIEW-WALL (a); SEC-SPREAD (a); BASELINE-WHEN (b), and
   the six changes approved for (b) are applied (C17.b1–b3, C23.b1–b3). ABL-PREDICT waits for a Feel Lab test. The
   Advisor set a 30 Hz server tick as the baseline for 48 players (TICK-30). Each answered question carries an
   'Answered' note; where a file still gives values for 48 and 54, or says 'until … answers', the answer applies, and
   the decision drafts restate those marks." Under the table: "R1–R6 merged in PR #28 (b6effde, 2026-10-31)." The R6
   owner cell: "infra-engineer (the security baseline: security-auditor, in its own session before the first online
   playtest; BASELINE-WHEN (b))".
4. **`docs/REPO_MAP.md`:** line 57 (r6: "the security baseline deferred") and line 79 (security-auditor: "R6 security
   baseline") say where the baseline now goes (BASELINE-WHEN (b)). The repo-map check is re-run.

## V. `docs/VISION_V2.md` line 43 (the session; VISION-LINE (a): "I wrote that line; the clarification is right. Add it
as proposed.")
- before: ``- `[DECIDED]` **Start small:** two teams with 2–8 players, and three teams with 6–18 players.``
- after (wrapped at the file's width, nothing else changed):
  ``- `[DECIDED]` **Start small:** two teams with 2–8 players, and three teams with 6–18 players (in total: 1v1 to 4v4,``
  ``  and 2v2v2 to 6v6v6; MODES-3 (a), Mustafa, 2026-10-31).``

## R. The research files (the session, by `scripts/apply_research_answers.py`; exact texts in the script, the preview
diff in `research-edits.diff`, the table in `research-edits-table.md`)
- **16 notes under the shared questions**, the same words in every topic: MODES-3 ×6 ("Answered 2026-10-31 (Mustafa):
  (a), totals. The largest mode is the goal 24v24, N = 48; where a mark gives values for 48 and 54, the 48 value
  holds."), BASELINE-WHEN ×4, SEC-SPREAD ×2, PREVIEW-WALL ×2, ABL-PREDICT ×2 ("answered later, after the Feel Lab test").
- **12 glossary edits:** a note under each "Depends on" tied to an answered question (10), one summary line under the
  list of decision codes, and one sentence after the Preview definition's "Until PREVIEW-WALL is answered there are no
  multiplayer previews."
- **The six changes approved on 2026-10-30 for BASELINE-WHEN (b)** (appendix §2: C17.b1–b3 at 1071-1080; C23.b1–b3 at
  1415-1428, as REVISED at 1596-1599). The step-3b verdict, condition 2: "C17.b1–b3 and C23.b1–b3 wait for
  BASELINE-WHEN"; its "before the PR is ready" case did not happen, so they apply now:
  - **Verbatim (4):** C17.b1 (R1-RR-OQ7; the whole line is replaced so the approved words wrap; step 3c's interim
    sentence after it stays, so the mechanism is named twice), C17.b3 (R5-GATE-OQ4), C23.b1 with C23.b2's first part
    (R6-HOST-REC4's heading, "This departs" → "C2 would depart"), C23.b3 (the spike).
  - **Adapted, for the advisor to rule (2)** (the step-3b rule: a quote that no longer matches is an OBJECT, never
    improvised):
    - **C17.b2** (R2-TRN-REC1 mark 5): the approved "before" ("An audit gate that keeps these current belongs to R6's
      security baseline (dependency audits), deferred by the Advisor.") no longer occurs, because step 3c reworded the
      sentence. Proposed: the approved "after" text, keeping step 3c's two glossary links at their terms.
    - **C23.b2** (R6-HOST-REC4's body): the approved quote "… stay the alternative if the baseline rejects a shared
      key." now ends "… rejects a shared key; when the baseline is researched is BASELINE-WHEN (below)." (step 3c's
      interim clause). Proposed: replace only the approved part ("C2 replaces them only if the baseline accepts a shared
      key") and keep that clause; the TCP 80 sentence and the Reason as approved.
- **Dry run:** every anchor and every "before" occurs exactly once (34 of 34); the research files are untouched until
  this plan is approved.
- **Not touched, for the drafts** (rewording needs judgment): R2-BUD-REC6's reason and GLOSSARY §1's 60 Hz sentence
  (carry-forward 2), the "until Mustafa answers PREVIEW-WALL" sentences inside R5-PREV-REC5, R6-HOST-REC1 and
  R6-HOST-REC3, R6-HOST-OQ5's "after the security baseline", the marks that give values for 48 and 54, and TICK-30's
  consequences for the tick-rate marks.

## I. Issue #16, the Night queue (the session, after approval; the PR body keeps the old and new text)
1. "kit-v3-1 is being finished in the R7 session, and its PR goes to Mustafa ([Advisor verdicts, 2026-10-27, second
   message](…#issuecomment-6454223695))." → "kit-v3-1 is being finished in the R7 session, in
   [PR #30](https://github.com/Mergincanli/Shamble-Frontlines/pull/30) (open since 2026-10-27), which goes to Mustafa
   ([Advisor verdicts, 2026-10-27, second message](…#issuecomment-6454223695))."
2. "Last updated: 2026-10-30." → "R1–R6 research is merged ([PR #28](https://github.com/Mergincanli/Shamble-Frontlines/pull/28),
   main `b6effde`, 2026-10-31). The decision drafts come next, and the security baseline is researched before the first
   online playtest; both are day work, so nothing new is queued. Last updated: 2026-10-31."
3. "No game-code changes until the v2 brief, so the old queue is paused below ([Advisor verdict,
   2026-10-17](…#issuecomment-6238312490))." → "The legacy game is frozen (critical fixes only, CLAUDE.md) and phase 0
   writes no engine code, so the old queue stays paused below ([Advisor verdict, 2026-10-17](…#issuecomment-6238312490))."
   (The brief exists since 2026-10-24, so "until the v2 brief" reads as if the pause had ended.)
The research-schedule sentence stays: NIGHT-RESEARCH (a) (2026-10-28) was "keep the rule (research by day only)".

## Evidence and report
- No report file (docs-only change-sets have none): the PR body is the report: what changed, what each fact was
  checked against, the gate result, the issue #16 diff, Decisions needed, For Mustafa.
- Gate: PASS on the branch (`node tools/gate.mjs`), and CI's `gate` and `smoke` on the PR head. Repo-map check.
- A read-only verification workflow over the final diff (four agents, effort high, ultracode): (V1) the research
  diff equals `research-edits.diff` hunk for hunk, and nothing else in `docs/research/r*.md` or `GLOSSARY.md` changed;
  (V2) every new fact in STATUS, ROADMAP, README, REPO_MAP, VISION_V2 and issue #16 against git, GitHub and the
  decisions file; (V3) each recorded decision says what Mustafa and the Advisor said, nothing added, every code
  explained on its line; (V4) a completeness critic: anything this plan lists but the diff lacks, anything still stale.
  Findings are fixed (docs-keeper for its files, the session for the rest) before the advisor's PR review.

## Reviewers
The advisor agent: this plan, then the PR's head, in this session. docs-keeper writes part L.

## Merge path
A draft PR while work is in progress; the advisor reviews its head; the checkpoint is folded into the PR body and
deleted in the last commit (CHECKPOINTS.md §9; that commit runs CI); the PR is marked ready; on Mustafa's "merge #<n>"
(docs only: no Brave check) the session merges with the approved full SHA as the expected head, with `gate` and `smoke`
green on it. A push after the APPROVE needs a new review and a new go.

## Risks
1. The research files change after their verification. Only decision notes and the six approved texts change, and V1
   proves nothing else did.
2. If the advisor rejects the two adapted texts, those two stay unapplied and STATUS lists them for the drafts.
3. kit-v3-1 (PR #30) may merge with the per-team wording; KIT-ALIGN stays open until a kit change-set fixes it.
4. "The foundation" (condition 3) is recorded in the Advisor's words. The session reads it as phase 1 (V2_PHASE0_BRIEF
   §1: the repository restructure, toolchain and CI gate v2) but writes no reading into the docs.
5. STATUS is long and its line numbers drift: docs-keeper greps before each edit.

## For the advisor's plan review
- The two adapted texts (C17.b2, C23.b2): approve, or give the text.
- Issue #16 item 3: include or leave.
- The scope: the research edits in this change-set (recommended: the six approved texts are due, and the drafts should
  start from files that carry the answers), or leave the research files to the drafts.

## For Mustafa
Nothing to check in Brave. On the new night-shift routine's page, set the schedule to `CRON_TZ=Europe/Amsterdam 10 0-7 *
* *`; after one good night, delete the old "Night shift" routine. Say "merge #<n>" once the advisor has approved this
PR's head.

## Conditions (advisor APPROVE of revision 2, 2026-10-31 about 14:05 UTC; verbatim in `advisor-verdicts.md`)
1. Part R in this change-set; the preview and table regenerated after conditions 2–6; V1 compares against them.
2. C17.b1: the advisor's merged text (replaces "Decides it … (below)." in R1-RR-OQ7), wrapped at the file's width.
3. C17.b2 exactly as in the script.
4. C23.b2: "them" → "the per-hostname certificates"; REC4's "Rests on" → "(C2's departure from it is stated above)".
5. The glossary's certificate note: the advisor's exact text (R6-HOST-REC4 now recommends the certificate per hostname).
6. Four logged-change lines after the step-3c "not applied" logs (r1:3690, r2:3880-3881, r5:2245, r6:1187-1188).
7. The README's "Answered since" paragraph: the advisor's exact text.
8. STATUS risk 1: the advisor's exact text (closed; the 60 Hz margin; TICK-30).
9. STATUS Targets: the old bullet is not marked superseded; it gets "(before VISION_V2, 2026-10-24; LOOP_V2 §2 and
   DESIGN_NOTES still carry it: DOCS-ALIGN)".
10. NS-2 leaves "Waiting" in every case; the schedule edit and deleting the old routine go under Routines and in "For
    Mustafa".
11. Carry-forwards: item 2 in the advisor's exact text; item 4 (the four sentences that still call the wildcard R6's
    proposal: r2-netcode.md:2703-2705, r6-infrastructure.md:47, 938-940 and 1056, lines at b6effde); item 5 (no doc
    defines "netcode proof test" or "the foundation" yet: the drafts' plan names them for the Advisor to confirm).
12. ROADMAP Phases: "v2 playable slices, in order (Mustafa and the Advisor, 2026-10-31): 1 move & shoot → 2 bots and
    three teams (played locally) → 3 online → 4 the loop" (no "after phase 1").
13. Issue #16 item 2: the advisor's exact text; items 1 and 3 as planned.
14. The PR body carries both messages word for word and the issue #16 old and new text; V3 checks the quotes.
15. Merge path: fold and delete the checkpoint in a CI-running commit → `git ls-tree -r --name-only HEAD -- .checkpoints`
    prints nothing → the advisor reviews that final head → ready → Mustafa's "merge #<n>" → merge with that SHA.
16. "For Mustafa" adds: tomorrow morning, check issue #16 has the new night shift's comment (the first proof it can post).
Applied so far: 1–6 (research edits, commit below), and V (VISION_V2 line 43).
