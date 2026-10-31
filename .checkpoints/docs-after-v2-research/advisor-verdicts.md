# Advisor verdict on docs-after-v2-research, plan revision 2 (verbatim)

From the advisor agent in session_0173vVo5q59ameR6cnN9BENg, hand-back of 2026-10-31 (about 14:05 UTC); prompt in
`prompts/advisor-plan-review.txt`; its notes in `advisor-plan-review.md`.

## Verdict: APPROVE, with 16 conditions (exact texts below)

**Why:** the plan applies both messages of 2026-10-31 faithfully, and the script is exact (34 of 34 edits, 0 problems; its diff matches the saved preview). It also adds nothing the decisions did not decide, except four small overstatements, which are fixed below. A few gaps are fixed below too: stale "not applied" logs, sentences that still call the wildcard certificate R6's proposal, and the merge-path order. Every condition is exact text or an exact order, so no second plan review is needed. The next review is of the PR's head.

## The points you asked about
1. **Scope:** apply the six (b)-only texts and the "Answered" notes in this change-set. Step-3b condition 2 made them due "after the answer", and the drafts should start from files that carry the answers. V1 proves nothing else changed.
2. **C17.b2:** approved as written in the script. It is the approved "after" text plus step 3c's two glossary links.
   - **C23.b2:** approved, with one word fix (condition 4).
   - **C17.b1:** the plan applies it word for word, which leaves the mechanism and the policy each stated twice in a row. Use the merged text in condition 2 instead, the same treatment C17.b2 gets.
3. **The notes (16 under the shared questions, 12 in the glossary):** faithful. Each one matches the reading the files already give for that answer (for example GLOSSARY's own "16v16v16 under MODES-3 (a)", "interim positions" and "per hostname holds"). One gap: GLOSSARY:693 still says "R6-HOST-REC4 proposes one wildcard certificate" (condition 5).
4. **The decisions:** SLICE-ORDER, TICK-30 and VISION-LINE are recorded word for word, and "the foundation" stays in the Advisor's words. One addition the decisions did not make: ROADMAP's "v2 slices … after phase 1". V2_PHASE0_BRIEF §1 puts the first playable slice inside phase 1 (condition 12).
5. **Carry-forward 2's numbers are right:** at N = 48 and 60 Hz, 8.28 of 8.33 ms on the 50 % server tick budget, and 4.41 of 4.17 ms on the 25 % server-only share (GLOSSARY.md:62-66). Its phrase "written with N = 54 in view" is loose, so condition 11 gives the text.
6. **Issue #16:**
   - Item 1 is correct.
   - Item 2's "both are day work" is not a recorded rule, so condition 13 gives the text.
   - Item 3: include it. It is accurate to CLAUDE.md and the brief.
7. **KIT-ALIGN and DOCS-ALIGN:** the line references check out (CLAUDE.md:7-8 and 191-192, netcode-architect 17-19, security-auditor 14, infra-engineer 29-30; the brief's lines 22-23 and 126; LOOP_V2 19-20 and 90-91; DESIGN_NOTES 34-35). VISION_V2 has no slice list (it never mentions slices). Nothing is missing. infra-engineer 23-26 and security-auditor 46 are not wrong today; keeping them as optional is fine.
8. **NS-2:** nothing blocks tonight. The queue is empty, and the schedule as it stands still gives one run at 00:10. What is still unproven: the test run fell outside the window, so it never showed that the new routine can comment on issue #16. Tomorrow morning's comment is that proof (condition 16).
9. **Verification and merge path:** V1–V4 are right. The order has to change: the advisor must review the head that already has the checkpoint deleted (condition 15).

## Conditions
1. **Scope approved as planned:** part R is in this change-set. Regenerate `research-edits.diff` and the table after conditions 2–6, and have V1 compare against the regenerated preview.
2. **C17.b1 (R1-RR-OQ7), merged text.**
   - Replace this, from "Decides it" through "(below).": `Decides it: R5's CI research and the synthesis. R5-GATE-REC1 schedules a cargo-deny advisories check and runs it on every PR that changes `Cargo.lock` ([GLOSSARY](GLOSSARY.md#the-advisories-check): the advisories check); whether an advisory blocks a PR is R5-GATE-OQ4, left to [the security baseline](GLOSSARY.md#the-security-baseline). Shared as BASELINE-WHEN (below).`
   - With: `Decided (BASELINE-WHEN (b)): the mechanism is R5-GATE-REC1's scheduled advisories check ([GLOSSARY](GLOSSARY.md#the-advisories-check): cargo-deny's advisories check), also run on every PR that changes `Cargo.lock`; whether an advisory blocks a PR (R5-GATE-OQ4), and who owns that policy, is the security baseline's ([GLOSSARY](GLOSSARY.md#the-security-baseline); R6-SEC-OQ1). Shared as BASELINE-WHEN (below).`
   - Wrap it at the file's usual width.
3. **C17.b2:** approved exactly as in the script.
4. **C23.b2:** as in the script, with two changes:
   - "C2 replaces them only if the baseline accepts a shared key" becomes "C2 replaces the per-hostname certificates only if the baseline accepts a shared key". Without this, "them" points at nothing.
   - In R6-HOST-REC4, `*Rests on:* F24; R2-TRN-REC5 (the departure is stated above).` becomes `*Rests on:* F24; R2-TRN-REC5 (C2's departure from it is stated above).`
5. **The glossary's certificate note (G-certificate)** becomes: `**Answered 2026-10-31 (the Advisor): BASELINE-WHEN (b),** so the certificate per hostname holds and the wildcard is an option the baseline weighs; R6-HOST-REC4 now recommends the certificate per hostname (C23.b1–b3, applied 2026-10-31).`
6. **Logged changes** (step-3b condition 2: "applied as a separate, logged change"). Four step-3c logs say "not applied". Leave those lines unchanged and add one line directly after each, at the same indent:
   - after r1:3690: `Applied 2026-10-31 (docs-after-v2-research, after BASELINE-WHEN (b)): C17.b1, merged with step 3c's sentence (advisor plan review, 2026-10-31).`
   - after r2:3880 (after the end of that bullet): `Applied 2026-10-31 (docs-after-v2-research, after BASELINE-WHEN (b)): C17.b2, fitted to step 3c's wording with its two glossary links (advisor plan review, 2026-10-31). R2 has no C23.b change under (b).`
   - after r5:2245 (after the end of that item): `Applied 2026-10-31 (docs-after-v2-research, after BASELINE-WHEN (b)): C17.b3, word for word.`
   - after r6:1187 (after the end of that bullet): `Applied 2026-10-31 (docs-after-v2-research, after BASELINE-WHEN (b)): C23.b1 and C23.b3 word for word, C23.b2 fitted to step 3c's interim clause (advisor plan review, 2026-10-31). C23.b1 is a recommendation change: R6-HOST-REC4 now recommends a certificate per hostname, with the wildcard an option the baseline weighs.`
7. **`docs/research/README.md`, the "Answered since" paragraph** reads: `Answered since (2026-10-31; STATUS, "Decisions pending"): MODES-3 (a), totals: the largest mode is 24v24, N = 48; PREVIEW-WALL (a): previews that reach the staging game server are private from the first one; SEC-SPREAD (a): weapon spread stays server-only; BASELINE-WHEN (b): ADR 0006 lists the security baseline as open with interim positions, and it is researched before the first online playtest, in its own fresh session. The six changes approved on 2026-10-30 for BASELINE-WHEN (b) are applied (C17.b1–b3, C23.b1–b3; C17.b1, C17.b2 and C23.b2 fitted to step 3c's wording, as the advisor approved on 2026-10-31), so R6-HOST-REC4 now recommends a certificate per hostname; where another sentence still calls the wildcard R6's proposal, REC4 holds. ABL-PREDICT (does the client predict the player's own abilities) waits for a Feel Lab test. For the decision drafts, the Advisor set a 30 Hz server tick as the baseline for 48 players; 60 Hz only if the netcode proof test shows at least 25 % headroom on the target hardware. Each answered question carries an "Answered" note; where a file still gives values for 48 and 54, the 48 value holds; where it says "until … answers", the answer applies; the decision drafts restate those marks.` The rest of part L.3 stays as planned.
8. **STATUS, risk 1** reads: `Closed 2026-10-31 by MODES-3 (a): the largest mode is 48 players, so the three marks that failed only at 54 are no longer at risk. At 48 and 60 Hz the re-flood tick still has almost no margin (carry-forward 2), and the Advisor made 30 Hz the baseline (TICK-30).` The plan's "where the marks written for 48 hold" overstates: the server-only share is 4.41 of 4.17 ms.
9. **STATUS, Targets:** the old bullet stays, but is not marked "superseded" (that would decide between documents). It gets: `(before VISION_V2, 2026-10-24; LOOP_V2 §2 and DESIGN_NOTES still carry it: DOCS-ALIGN)`.
10. **STATUS, NS-2:** it leaves "Waiting" in every case. The schedule edit and deleting the old routine after one good night go under Routines and in "For Mustafa". Drop the "unless the schedule is still unset" clause.
11. **STATUS, carry-forwards:**
    - Item 2 reads: `R2-BUD-REC6's reason (r2-netcode.md:2402-2403) says the marks fit the 50 % server tick budget at 60 Hz on every tick but one that both encodes a snapshot and re-floods; that holds at N = 54 (8.99 of 8.33 ms), but at N = 48 that tick fits by 0.05 ms (8.28 of 8.33 ms) while its server-only share is 0.24 ms over its own 25 % (4.41 of 4.17 ms) (GLOSSARY.md:62-75). Under TICK-30 the drafts restate these marks for the 30 Hz baseline and say what the 25 % headroom for 60 Hz is measured against (the advisor's #28 review, finding 2).`
    - Add item 4: `R6-HOST-REC4 now recommends a certificate per hostname (C23.b1–b3); four sentences still describe the wildcard as R6's proposal (r2-netcode.md:2703-2705, r6-infrastructure.md:47, 938-940 and 1056, lines at b6effde): the drafts follow REC4.` List the same four under "Not touched, for the drafts".
    - Add item 5: `No doc defines SLICE-ORDER's "netcode proof test" or "the foundation" yet: the drafts' plan names the research spike the proof test is and what the foundation is, for the Advisor to confirm.`
12. **ROADMAP, Phases:** `v2 playable slices, in order (Mustafa and the Advisor, 2026-10-31): 1 move & shoot → 2 bots and three teams (played locally) → 3 online → 4 the loop`. Drop "after phase 1".
13. **Issue #16, item 2** reads: `R1–R6 research is merged ([PR #28](https://github.com/Mergincanli/Shamble-Frontlines/pull/28), main `b6effde`, 2026-10-31). Next: the decision drafts (not queued: they have no approved plan for unattended work); the security baseline is researched before the first online playtest, in its own fresh session (research runs by day only). Last updated: 2026-10-31.` Items 1 and 3 as planned.
14. **The PR body** (the report) carries both messages of 2026-10-31 word for word, from `decisions-2026-10-31.md`. That file is deleted with the checkpoint, so the body becomes the lasting record. It also carries the old and new issue #16 text. V3 checks the quotes.
15. **Merge path order:**
    1. Fold the checkpoint into the PR body and delete `.checkpoints/docs-after-v2-research*` in a commit that runs CI.
    2. Check that `git ls-tree -r --name-only HEAD -- .checkpoints` prints nothing.
    3. The advisor reviews that final head (full SHA).
    4. Mark the PR ready.
    5. Mustafa's "merge #<n>" in this session's chat; merge with that SHA as the expected head and `gate` and `smoke` green on it.

    A review before the deletion commit would approve a head that is not the one merged.
16. **"For Mustafa" adds:** tomorrow morning, check that issue #16 has the new night shift's comment. That comment is the first proof the new routine can post. If it is missing, tell the day session.

## What I checked myself
- `python3 -I …/apply_research_answers.py --check`: 34 edits in 7 files, 0 problems, exit 0; the working tree stayed clean.
- `--diff` compared with `research-edits.diff`: identical.
- The six approved texts against appendix lines 1071-1080, 1415-1428 and 1596-1599: C17.b3, C23.b1 (as REVISED) and C23.b3 match. C23.b2's TCP 80 sentence and Reason match. C17.b2 is the approved "after" plus the two links.
- The current research text at r1:2955-2961, r2:2610-2614, r5:1873-1875 and r6:684-697, and the four step-3c logs that say "not applied" (r1:3690, r2:3880, r5:2245, r6:1187).
- Every glossary note against the glossary's own (a) and (b) readings (lines 36-41, 368-412, 549-556, 653-706). Wildcard mentions grepped across `docs/research/`.
- GLOSSARY.md:49-75 and r2:2398-2403: the arithmetic behind 8.28 of 8.33, 4.41 of 4.17 and 8.99 at N = 54.
- git:
  - b6effde is PR #28's merge (parents e9f11be and 854d42f, 13:26:31Z); e9f11be is PR #29's (2026-10-27).
  - origin/main is b6effde, and the branch diff touches only `.checkpoints/`.
- GitHub REST:
  - PR #28: merged_at 13:26:31Z, head 854d42f, merge commit b6effde. PR #29 merged 2026-10-27.
  - PR #30: open, not a draft, head 5be7d37, created 2026-10-27T21:54Z.
  - Issue #16's body still reads "Last updated: 2026-10-30", with the three sentences the plan changes.
- Docs:
  - ROADMAP has 9 Recent-work entries, so 11 after the change.
  - VISION_V2:43 is the line the plan quotes, and VISION_V2 never mentions slices.
  - V2_PHASE0_BRIEF:22-23 puts the first playable slice in phase 1.
  - "foundation" and "proof test" appear in no design doc.
  - The KIT-ALIGN and DOCS-ALIGN line references, read one by one.
  - CHECKPOINTS.md §9, and the docs-after-r7 precedent plan (86ece5c).
- The script writes only the listed research files, and only with `--apply`.

## What I could not check
- The routines facts: the new routine's id, schedule, prompt and test run, and the old routine switched off at 13:54 UTC. I have no routines API access, so these rest on the session's read-back.
- Mustafa's and the Advisor's chat messages: I have only the session's verbatim file.
- NIGHT-RESEARCH (a)'s exact wording: it is not in issue #16's comments. Leaving that sentence as it is, as the plan does, is the safe choice.
- I did not run the gate. Nothing in docs/ is edited yet; I run it at the PR review.

## Decisions needed
None. Everything here was within the advisor's authority, or was already decided on 2026-10-31.

## For Mustafa
1. On the new "Shamble Frontline Night shift" routine's page, set the schedule to `CRON_TZ=Europe/Amsterdam 10 0-7 * * *`. Agents cannot edit a routine they did not create. Without it, tonight still gets one run at 00:10, and nothing is queued.
2. Tomorrow morning, check that issue #16 has the night shift's comment. It is the first proof the new routine can post.
3. After one good night, delete the old "Night shift" routine. Deleting it also deletes its session.
4. Once the advisor has approved this PR's final head, say "merge #<n>" in this session's chat. It is docs only, so there is no Brave check.

Notes: /home/user/Shamble-Frontlines/.checkpoints/docs-after-v2-research/advisor-plan-review.md (ends with DONE). Nothing was committed or pushed.
