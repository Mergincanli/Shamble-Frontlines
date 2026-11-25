# Advisor report: ADR 0006's small revision after the preview hosting check (2026-11-25, branch `claude/optimistic-albattani-mdoonz` standing in for `cs/adr0006-header-row`, base main 09faefa)

Pull request: #PRNUM (https://github.com/Mergincanli/Shamble-Frontlines/pull/PRNUM).
- Risk tier 1: an ADR draft and his words in `docs/DECISIONS.md`, both decision docs, so it waits for his go.
- No code, CI, gate, kit, `spikes/` or lane-1 file. Nothing a player sees, hears or feels.
- Written by the builder chat Mustafa calls Builder V2 (lane 2; session_019YNMUSYpJGi1xxCjKgr4e5).

## Goal
This revision carries out his three answers on the preview hosting check (2026-11-25, 00:55 Amsterdam) in ADR 0006,
the infrastructure draft. The check was merged as #66.
- ONE-OFF-HEADER (a): the header mark is judged on production's own address, with the reason written into the row.
- His addition to (a): a short paragraph weighs the other Cloudflare product, Workers static assets.
- STEP4-COUNT (b): the draft says plainly that the build-count question is not answered.

## Outcome
DONE, waiting for his go. The marks, one at a time, each on readings already taken and subject to his go:
- The revised header mark (PM-0006-2) is met.
- The build-count mark (PM-0006-7) is complete as re-stated, and the question stays unanswered.
- His verdict in Brave (PM-0006-9) waits for his word on checklist step 4.

The check as a whole is not called passed: Safari stays "not checked, never as passed".

## Plan and approval
- **The plan:** `.checkpoints/adr0006-header-row/plan.md`, revision 1 at cbd5ada, revision 2 folding in the conditions.
  It is readable at CKPT after the notes leave the branch.
- **The advisor agent:** APPROVE with nine conditions (notes: `advisor-plan.md`, same commit). All nine are met:
  1. E9 and E10 use its exact words, each tagged "the advisor agent's reading of Mustafa's STEP4-COUNT (b), never his
     decision".
  2. The dates are right: the dashboard was read on 2026-11-18, Cloudflare's docs on 2026-11-24.
  3. "If uploads count, ..." is kept beside "until a count is shown, ... anyway".
  4. E1 names production's own address as the project's `pages.dev` host, gives his option's reason with no quotation
     marks, and cites no advisor tool.
  5. Step 4's word is left to him.
  6. Nothing calls the check passed.
  7. E7 answers the question for production's own version links, and says "silent" where the docs are.
  8. #66's row moves with its state cell set to "merged".
  9. Rollback's reason is re-worded, as the condition gave it.
- **It also settled three things without a question to him:**
  - his STEP4-COUNT (b) reaches the pass words of PM-0006-7 and step 4, but not those of PM-0006-9;
  - step 2.6's plan starts after this merge;
  - his TR-END condition is met, so the clean-up timing is his.

## What changed
- `docs/adr/0006-infrastructure-operations.md` (dceb4e2, 1c45ed7, FIXES):
  - PM-0006-2 (E1, E2);
  - part 1, the build count (E3) and the Workers paragraph (E7);
  - PM-0006-7 (E4, E9);
  - checklist step 4 (E10);
  - spike 1's list (E6).

  The file grew from 1694 to 1712 lines. Later line numbers move by 16 from part 1 on, and by 18 after step 4. Dated
  records that cite lines are not edited. Every changed sentence is in the Appendix.
- `docs/DECISIONS.md` (0bead83): his message of 2026-11-25, 11:07:07 UTC, whole, copied by script. 741 bytes, sha256
  ae5eb4f6...; 16 lines added, none changed.
- Docs pass (DOCSPASS), lane 2's lines only: STATUS (#66's row to the archive as merged; this pull request's row),
  STATUS_ARCHIVE, ROADMAP, REPO_MAP and SPIKE_PLAN entry 6.
- This report.

## Evidence
- **The gate,** `node tools/gate.mjs --no-smoke`: checks 1-4 and 5b-5d PASS, symbols +0 -0. The smoke test (5a) was
  not run here: the toolkit's Playwright wants chromium_headless_shell-1194 and this machine has 1243. CI's gate and
  smoke on the head: CIRESULT.
- **The verifier:** VERIFIER.
- **The research:** infra-engineer read 22 of Cloudflare's pages (S1-S22), all on 2026-11-25. Its notes are readable
  at CKPT.
- **The measured numbers behind the revised row** (#66's report, Appendix A):
  - production's one-off links 0354374e (round-a, :666-668) and de1c3e5e (restore-production, :813-818) carried
    `noindex` on all 7 paths, and no rule of the production build writes it;
  - production's own address carried no `noindex` (round-a, restore-production);
  - the planted `noindex` was caught on production's own address (bite-production, run 44366750977, :805-811);
  - every other header was as expected on both kinds of link.
- **The build count:** "no count shown" in the dashboard, before and after the ten uploads, both on 2026-11-18
  (:706-757). Cloudflare's docs are silent on 2026-11-24 (:1384-1388) and again on 2026-11-25 (S17, S18).

## Reviews
REVIEWS

## Decisions and deviations
- E8, the Date line, was dropped (the advisor agent). E5 was replaced by E10.
- E7 is infra-engineer's draft, re-worded by this chat in plain words (11 lines). Every clause keeps its source, and
  the read date is given once for all of them.
- #66's row went to the archive with its state cell set to "merged". That follows his "correct that" and #64's
  precedent, rather than a byte-for-byte move.
- The smoke test was not run locally (the browser version, above); CI runs it.

## Risks
- E7 rests on Cloudflare's docs alone: nothing was measured on Workers. Cloudflare's own words, "Start new projects
  with Workers", are cited and not acted on (WORKERS-NEXT, below).
- Cloudflare "reserves the right to attach new headers to static asset responses at any time" (its Workers headers
  page, S11). The revised row still judges production's own address, so a new header there would fail the mark.
- Dated records that cite ADR 0006 by line now point 16 to 18 lines early. They read at their own dates.

## Proposed next step
Step 2.6's plan (a preview link for every pull request). It starts after this merge, and its first edit waits for his
go on the revised row.

## Decisions needed
1. **WORKERS-NEXT (optional; this pull request does not need it).**
   - The question: does Workers static assets stay only the draft's fallback, or is the preview hosting check run
     again on it now?
   - Why it matters: Cloudflare's Pages page now says "Start new projects with Workers", and the way Workers makes a
     link for each pull request has changed since our research (a feature called Previews).
   - Options: (a) keep Pages; at the draft's next revision, add Cloudflare's statement to its "Revisit when" line and
     refresh the Workers route; (b) run the check on Workers now (new setup in his Cloudflare account, about two days).
   - My recommendation: (a). Nothing failed on Pages that Workers would fix, and the revised row is met on Pages.

## For Mustafa
1. Your go on this pull request's final head. It is your go on the revised row, and step 2.6's first edit waits for
   it.
   - It also covers E9 and E10 (the advisor agent's reading of your STEP4-COUNT (b)).
   - If you want both left open, say so, and one push removes them.
2. Your word on checklist step 4 under its new words: "achieved" or "not yet". What you read on 18 November: no
   count shown, before or after the ten uploads.
3. The test repository's clean-up, at your own timing. Your TR-END (a) said "after the report is merged", and it is.
   - First, switch off the hourly checker before 2 December: Mergincanli/shamble-preview-check → Actions → "probe" →
     "..." → Disable workflow.
   - Then archive the repository. An archived repository is read-only.
   - Then delete the Cloudflare test project and its Access application (Zero Trust → Access → Applications).
4. Only if you want step 2.6's plan written sooner: name a second branch I may push. Otherwise it starts after this
   merge.

## Appendix: every changed sentence, and what was left alone
APPENDIX
