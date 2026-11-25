# Advisor report: ADR 0006's small revision after the preview hosting check (2026-11-25, branch `claude/optimistic-albattani-mdoonz` standing in for `cs/adr0006-header-row`, base main 09faefa)

Pull request: #67 (https://github.com/Mergincanli/Shamble-Frontlines/pull/67).
- Risk tier 1: an ADR draft and his words in `docs/DECISIONS.md`, both decision docs, so it waits for his go.
- No code, CI, gate, kit, `spikes/` or lane-1 file. Nothing a player sees, hears or feels.
- Written by the builder chat Mustafa calls Builder V2 (lane 2; session_019YNMUSYpJGi1xxCjKgr4e5).

**Two things he should know before his go.** Neither decides a move, and neither is escalated:
- Cloudflare's Pages page now says "Start new projects with Workers" (developers.cloudflare.com/pages/, read
  2026-11-25).
- Pages' limits page no longer says previews are unlimited. It now reads "Preview deployments older than 30 days are
  deleted, unless they are one of the project's 100 most recent deployments".

## Goal
This revision carries out his three answers on the preview hosting check (2026-11-25, 00:55 Amsterdam) in ADR 0006,
the infrastructure draft. The check itself was merged as #66.
- ONE-OFF-HEADER (a): the header mark is judged on production's own addresses, with the reason written into the row.
- His addition to (a): a short paragraph weighs Workers static assets.
- STEP4-COUNT (b): the draft says plainly that the build-count question is not answered.

## Outcome
DONE, waiting for his go. The marks, one at a time, each on readings already taken and subject to his go:
- The revised header mark (PM-0006-2) is met. It is met without the custom-domain clause, because the check had no
  custom domain.
- The build-count mark (PM-0006-7) is complete as re-stated, and the question stays unanswered.
- His verdict in Brave (PM-0006-9) waits for his word on checklist step 4.

The check as a whole is never called passed: Safari stays "not checked, never as passed".

## Plan and approval
- **The plan:** `.checkpoints/adr0006-header-row/plan.md`, revision 1 at cbd5ada, revision 2, then revision 3 for
  review round 1's fix pass. It and every note below are readable at CKPT after the notes leave the branch.
- **The advisor agent on revision 1:** APPROVE with nine conditions (`advisor-plan.md`). All nine are met:
  1. E9 and E10 use its exact words, each tagged "the advisor agent's reading of Mustafa's STEP4-COUNT (b), never his
     decision".
  2. The dates are right: the dashboard was read on 2026-11-18, Cloudflare's docs on 2026-11-24.
  3. "If uploads count, ..." is kept.
  4. E1 names production's own addresses, gives his option's reason with no quotation marks, and cites no advisor
     tool.
  5. Step 4's word is left to him.
  6. Nothing calls the check passed.
  7. E7 answers the question for production's own links, and says "silent" where the docs are.
  8. #66's row moves with its state cell set to "merged".
  9. Rollback's reason is the one the condition gave.
- **It also settled three things without a question to him:**
  - his STEP4-COUNT (b) reaches the pass words of PM-0006-7 and step 4;
  - step 2.6's plan starts after this merge;
  - his TR-END condition is met, so the clean-up timing is his.
- **The advisor agent on revision 3:** APPROVE with six conditions, all met:
  - R3-10 and R3-11 are its exact words;
  - PASS-WORDS covers E9, E10 and E11 together;
  - this report's top names the two Cloudflare changes;
  - round 2 re-reads R3-10, R3-11 and the new source (S23);
  - the follow-ups are accepted, with ADR6-ROLLBACK and PHC-PREVIEW-NOINDEX named as Medium;
  - E7 is accepted at 15 lines as "a short paragraph" (his words set no line count; "at most 8" was the plan's own
    limit).

## What changed
- `docs/adr/0006-infrastructure-operations.md` (dceb4e2, 1c45ed7, ab15d6d): 12 passages, 33 lines added and 8 removed.
  - PM-0006-2: E1, E1b, E2, R3-11.
  - Part 1: E3 (the build count), R3-10 ("no cap on previews", dated), E7 (the Workers paragraph).
  - PM-0006-7: E4, E9.
  - The hosting checklist: E11 (its opening), E10 (step 4).
  - Spike 1's list: E6.

  The file grew from 1694 to 1719 lines. Later line numbers move: +23 from the marks table on (PM-0006-2 is now
  :1393), +25 after step 4. Dated records that cite lines are not edited. Applying the Appendix's 12 entries to main's
  file gives the head's file exactly (`appendix.py`, readable at CKPT).
- `docs/DECISIONS.md` (0bead83): his message of 2026-11-25, 11:07:07 UTC, whole, copied by script. 741 bytes, sha256
  ae5eb4f6...; 16 lines added, none changed.
- The docs pass, DOCSPASS.
- This report.

## Evidence
- **The gate,** `node tools/gate.mjs --no-smoke`: checks 1-4 and 5b-5d PASS, symbols +0 -0, on 1c45ed7 and on ab15d6d.
  The smoke test (5a) was not run here: the toolkit's Playwright wants chromium_headless_shell-1194 and this machine has
  1243. CI's gate and smoke on the final head: CIRESULT.
- **The verifier:**
  - Round 1 on 1c45ed7: FAIL, on plan conformity only. Every claim, number, date and byte held.
  - Round 2 on ab15d6d: PASS WITH NOTES. The head matches plan revision 3 word for word, every new source claim holds,
    and so do PM-0006-2's 34 readings.
  - Notes: `verifier.md`, with scripts in `verifier-scratch/`.
- **The research:** infra-engineer read 22 of Cloudflare's pages on 2026-11-25 (S1-S22), plus the Pages limits page
  again (S23). Notes: `infra-engineer-workers.md`, `infra-engineer-review.md`.
- **The measured numbers behind the revised row** (#66's report, Appendix A):
  - 0354374e (round-a, :666-668) and de1c3e5e (restore-production, :813-818) are production's one-off links. Both
    carried `noindex` on all 7 paths, written by no rule of the production build.
  - Production's own address carried no `noindex`.
  - The planted `noindex` was caught on production's own address (run 44366750977, :805-811).
  - Every other header was as expected on both kinds of link.
- **The build count:** "no count shown" in the dashboard, before and after the ten uploads, on 2026-11-18 (:706-757).
  Cloudflare's docs are silent, read 2026-11-24 (:1384-1388) and 2026-11-25 (S17, S18, S23).

## Reviews (two rounds, within the review budget; the advisor agent's plan reviews are not rounds)
- **infra-engineer.**
  - Round 1: REVISE. F1 (High): "where Pages sets no cap" had no source; fixed by cutting it, with part 1's clause
    dated (R3-10). F3, F4 and F5 (Low): fixed. F6 (Low, the paragraph's length): recorded in revision 3. F2 (Medium):
    follow-up ADR6-ROLLBACK.
  - Round 2: APPROVE. Two optional Lows go to follow-ups ADR6-WORDING: G1 ("oldest" could read as "made first") and G2
    (00:55 is his label; the record's time is 00:53 Amsterdam).
- **test-engineer**, round 1: APPROVE.
  - C1: PASS-WORDS is its own entry below.
  - F2, F4, F5 and F6 (Low): adopted word for word.
  - F1 (Medium, older than this change): follow-up PHC-PREVIEW-NOINDEX, which goes to step 2.6's plan. No bite tests a
    preview build that lacks its `noindex` rule, because the host adds one anyway.
  - F3 and F7 (Low): follow-ups.
- **The verifier:** above. Its Lows: line 67's own "no cap on previews" is added to ADR6-PAGES-RETENTION. R3-10 and
  R3-11 were written into the plan before the edits but reached git in a4152ad, after the fix-pass commit ab15d6d.

## Decisions and deviations
- E8, the Date line, was dropped. E5 was replaced by E10.
- E7 is infra-engineer's draft, re-worded by this chat in plain words. At 1c45ed7 it departed from revision 2 without
  a recorded approval (the verifier's round-1 FAIL). Revision 3 recorded it and was approved before the fix pass.
- #66's row went to the archive with its state cell set to "merged". That follows his "correct that" and #64's
  precedent, rather than a byte-for-byte move.
- The local smoke test was not run (the browser version, above); CI runs it.

## Not verified
- Nothing was measured on Workers. "Silent" covers only the pages read on 2026-11-25.
- No reading tested the custom-domain clause.
- The quotes of his words were not compared again with the chat beyond his message of 11:07:07 UTC.

## Risks
- E7 rests on Cloudflare's docs alone. Cloudflare's direction ("Start new projects with Workers") is cited, not acted
  on (WORKERS-NEXT).
- Cloudflare "reserves the right to attach new headers to static asset responses at any time" (its Workers headers
  page, S11). Production's own address is still judged, so a new header there would fail the mark.
- Pages' retention rule could expire old previews of open pull requests after 30 days, unless they are among the 100
  newest. Step 2.6's plan weighs it (ADR6-PAGES-RETENTION).
- Dated records that cite ADR 0006 by line now point 23 to 25 lines early. They read at their own dates.

## Proposed next step
Step 2.6's plan: a preview link for every pull request. It starts after this merge, and its first edit waits for his
go on the revised row.

## Decisions needed
1. **PASS-WORDS: may the re-worded pass words stand?**
   - The question in plain words: his STEP4-COUNT (b) sent the build-count question back to the draft as not
     answered. The advisor agent read it as also re-wording three places so that they can be met by the readings
     already taken, while the question stays unanswered:
     - the build-count mark (PM-0006-7): "passes when both are recorded, each as the count the page shows or as "no
       count shown"";
     - checklist step 4: "two readings recorded, each a number or "no count shown"";
     - the checklist's opening: "or the readings it asks for".
   - Why it matters: under the old words ("two numbers recorded") the mark and step 4 can never be met, because no
     page shows a count.
   - Options: (a) his go on #67 covers all three; (b) one push removes all three before the merge, and both stay open
     until someone shows a count.
   - My recommendation: (a).
2. **WORKERS-NEXT (optional; #67 does not need it).**
   - The question in plain words: does Workers static assets stay only the draft's fallback, or is the preview hosting
     check run again on it now?
   - Why it matters: Cloudflare's Pages page now says "Start new projects with Workers", and Workers' way of making a
     link for each pull request has changed since our research (a feature called Previews).
   - Options: (a) keep Pages; at the draft's next revision, add Cloudflare's statement to its "Revisit when" line and
     refresh the Workers route; (b) run the check on Workers now (a new setup in his Cloudflare account, about two
     days).
   - My recommendation: (a). Nothing failed on Pages that Workers would fix.

## For Mustafa
1. Your go on this pull request's final head, with your PASS-WORDS answer. It is your go on the revised row, and step
   2.6's first edit waits for it.
2. Your word on checklist step 4 under its new words: "achieved" or "not yet". What you read on 18 November: no count
   shown, before or after the ten uploads.
3. The test repository's clean-up, at your own timing. Your TR-END (a): "after the report is merged", and it is.
   - First, switch off the hourly checker before 2 December: Mergincanli/shamble-preview-check → Actions → "probe" →
     "..." → Disable workflow.
   - Then archive the repository. An archived repository is read-only.
   - Then delete the Cloudflare test project and its Access application (Zero Trust → Access → Applications).
4. Only if you want step 2.6's plan written sooner: name a second branch I may push. Otherwise it starts after this
   merge.

## Appendix: every changed passage, before and after, and what was left alone
Line numbers: "was" at main 09faefa, "now" at this pull request's head. Each entry is checked by script: its "before"
occurs once in main's file and its "after" once in the head's.

APPENDIX

**Left alone on purpose, with the reason:**
- The options row Q1 (now :624): it gives the options as the research read them. Its "Unlimited previews" is follow-up
  ADR6-PAGES-RETENTION.
- Line 67's "with no cap on previews" ("The findings the decision turns on"): follow-up ADR6-PAGES-RETENTION.
- The trade-off bullet: it argues the choice from the research. The weighing went where the draft's rule sends it,
  part 1.
- "Revisit when": it names a PREV-DELETE fail only, and PREV-DELETE was met.
- Rollback: the revised mark is met by readings already taken, so its trigger is not reached (follow-up ADR6-ROLLBACK).
- "When a mark fails": unchanged. This revision is that rule carried out.
- PM-0006-9 and its pass words: unchanged. It waits on his word on step 4.
- GLOSSARY §5: the header sets are what the build ships, and the host's own header is outside them.
- The record's Date line: dropped from the plan (E8).
