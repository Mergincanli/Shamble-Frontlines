# Plan: ADR 0006's small revision after the preview hosting check (lane 2, Builder V2)

Revision 3, 2026-11-25: the review round's fix pass. The section "Revision 3" below lists every change from revision 2.
Where it gives a final text, that text replaces revision 2's text in section 3.

Revision 2 (cbd5ada's revision 1 with the advisor agent's APPROVE and nine conditions folded in) is otherwise
unchanged. The advisor agent's notes are in `advisor-plan.md` in this folder.

## Revision 3: the fix pass after review round 1 (for the advisor agent's approval before any edit)
Round 1 returned three verdicts:
- infra-engineer: REVISE (F1 High; F3, F4 and F5 Low wording; F6 Low; F2 Medium, made a follow-up).
- test-engineer: APPROVE, with condition C1 for the final head; F2, F4, F5 and F6 are Low wording fixes; F1, F3 and
  F7 are follow-ups.
- The verifier: FAIL, for one reason only: E3 and E7 as placed at 1c45ed7 depart from revision 2 without a recorded
  approval. E7 runs 12 lines against "at most 8", its opener differs, and Q1 is not answered separately for alias and
  `workers.dev` links. E3's bracket wording differs. Every claim, number, date and byte it checked holds.

Their notes are in this folder: `infra-engineer-review.md`, `test-engineer-review.md`, `verifier.md`.

**R3-1. E7, final text.** It replaces revision 2's frame of "at most 8 lines" and its quoted opener. It is a short
paragraph of 15 lines; his words ask for "a short paragraph". The new text answers infra-engineer's F1, F3, F4 and F5
and the verifier's Q1 point. Lines 155-166 at 1c45ed7 become:
```
   *Workers static assets, weighed again* (2026-11-25, after the header mark failed; Mustafa's ONE-OFF-HEADER (a);
   Cloudflare's docs read 2026-11-25, paths under developers.cloudflare.com/workers/ unless marked): none of the
   check's marks was measured on Workers. The host adds `noindex` to `workers.dev` Preview URLs, "but not to custom
   domain Preview URLs" (previews/custom-domains); the docs are silent on production's own address, its `workers.dev`
   URL, its version links and their aliases (versions-and-deployments/version-urls; configuration/routing/workers-dev),
   so they neither show nor rule out the header the check found on Pages' production one-off links. A pull request
   gets a Preview of its branch's files (previews/configuration), at most 100 per Worker on the Free plan, the oldest
   deleted when the limit is reached, and a command that deletes it (previews; wrangler/commands/workers); on Pages,
   PREV-DELETE was met through the cleanup job, with a departure (the check's report, PM-0006-6). A wall can cover
   previews only (configuration/cloudflare-access); `_headers` takes host rules (static-assets/headers); 25 MiB a
   file, as on Pages (developers.cloudflare.com/pages/platform/limits/); no monthly cap on deploys is named
   (platform/limits; platform/pricing). Cloudflare's Pages overview says "Start new projects with Workers"
   (developers.cloudflare.com/pages/). None of this answers a failed mark: the revised header row is met by the
   readings already taken on Pages (the check's report, Appendix A), so Pages stays; a move would mean running the
   check again on Workers.
```
What changed in it, and why:
- "where Pages sets no cap" is cut (infra F1, High). The Pages limits page no longer says previews are unlimited; it
  names a retention rule (follow-up ADR6-PAGES-RETENTION). The Workers side now names its own rule instead: "the oldest
  deleted when the limit is reached" (S2: "When a limit is reached, Cloudflare automatically deletes the oldest").
- "`workers.dev` Preview URLs" now uses the source's own words (infra F5).
- "silent on production's own address, its `workers.dev` URL, its version links and their aliases" answers Q1 for
  each kind of link (infra F4; the verifier; the advisor agent's condition 7). The sources are S1 and S8, and infra-
  engineer's Q1 answers say "silent" for each.
- "on Pages, PREV-DELETE was met through the cleanup job, with a departure" (infra F3). The report's PM-0006-6 row
  reads "Met with a departure".
- "as on Pages" now names its source (the verifier: pages limits :71).

**R3-2. E3, final text** (as placed at 1c45ed7), with one clock fix from the verifier's Low. His decision is dated by
his own label in Amsterdam time, while DECISIONS heads it in UTC (2026-11-24, 23:53:15):
- "[...] and Cloudflare's docs do not say (read 2026-11-24 and 2026-11-25; PM-0006-7;
  `docs/reports/2026-11-24-preview-hosting-check.md`). If uploads count, [...] work that way anyway (Mustafa's
  STEP4-COUNT (b), 2026-11-25, 00:55 Amsterdam time). A paid plan [...]"
- It replaces revision 2's "(read 2026-11-24, and again for this revision) (PM-0006-7; ...)". The dates are now
  concrete: Q8 was read on 2026-11-25.

**R3-3. E1, test-engineer's F6.** Production's custom domain is judged too, as PM-0006-8 already does:
- "production none on its own address (the project's `pages.dev` host)" becomes "production none on its own addresses
  (the project's `pages.dev` host, and a custom domain if CF has one, as in PM-0006-8)".
- This is stricter, not looser.

**R3-4. E1b, new: PM-0006-2's "what it proves" cell** (test-engineer's F2, Low):
- "The host serves each header set as generated, on both URL forms (part 1; decides R5-PREV-OQ6)" becomes "The host
  serves each header set as generated, on both URL forms (production's one-off addresses: the host's own
  `X-Robots-Tag` recorded, not judged) (part 1; decides R5-PREV-OQ6)".

**R3-5. E4, test-engineer's F5** (one date, as in E3):
- "Cloudflare's docs do not say (2026-11-24)" becomes "Cloudflare's docs do not say (read 2026-11-24 and 2026-11-25)".

**R3-6. E11, new: the hosting checklist's intro** (:1459; test-engineer's F4, Low), to match E10's "readings":
- "or the numbers it asks for." becomes "or the readings it asks for."

**R3-7. Follow-ups, not edits** (into STATUS's follow-ups, with their text in the archive):
- ADR6-ROLLBACK (infra F2, Medium): align Rollback's trigger with the mark-fail rule; "only the deploy target
  changes" understates a move.
- ADR6-WK-ROUTE: the R5 and R6 lines on `versions upload --preview-alias` and "no deletion read" get dated pointers.
- ADR6-WK-BIC: any re-run on Workers sends a descriptive User-Agent.
- ADR6-PAGES-RETENTION: Pages keeps deployments 30 days plus the 100 newest; part 1's "no cap on previews" and the
  options row's "Unlimited previews" are re-read at the draft's next revision; R5-PREV-F13 is checked.
- PHC-PREVIEW-NOINDEX (test F1, Medium, older than this change): step 2.6's plan takes a build-output test that each
  preview mode's `_headers` carries the `noindex` rule and production's does not, with a bite; and, before any preview
  is served where the host adds no `noindex`, the preview expectation is read there.
- ADR6-PM7-PAGES (test F3) and ADR6-PM7-FIGURE (test F7) go to the draft's next revision.

**R3-8. The report**, test-engineer's C1. Whether his go covers E9 and E10 is its own entry in "Decisions needed",
PASS-WORDS, written in plain words. His word on step 4 is its own item in "For Mustafa".

**R3-9. Review round 2, on what changed only:**
- infra-engineer re-reads E7's new lines and E3.
- The verifier checks the head against this revision word for word, and spot-checks the new source claims (the
  version-urls and workers-dev pages' silence; S2's "deletes the oldest"; pages limits for 25 MiB).
- test-engineer's fixes are its own words, so the verifier's word-for-word check covers them.

Branch `claude/optimistic-albattani-mdoonz`, standing in for `cs/adr0006-header-row`; this cloud session may push only
this branch. Base: main 09faefa (#66's merge). Checkpoint: `.checkpoints/adr0006-header-row.md`.

## 1. Goal
This plan carries out three of Mustafa's instructions:
- **ONE-OFF-HEADER (a), with its two additions.** His words, 2026-11-25, 00:55 Amsterdam, in `docs/DECISIONS.md`:
  "The revision of the draft's row gives its reason in the row and weighs the other Cloudflare product (Workers static
  assets) in a short paragraph, as the draft's own rule for a failed header mark asks. [...] The revision is one small
  pull request of its own, after #66."
- **STEP4-COUNT (b).** His words: "The draft says plainly that the question is not answered. Until someone shows a
  count, step 2.6 works as if uploads count: only the newest push of a pull request is deployed, and it counts its
  deploys per month."
- **His message of 2026-11-25, 11:07:07 UTC** (his go for #66; it goes into DECISIONS), including: "The status page's
  row still calls #66 a draft: correct that in your next pull request."

The two options he chose, as the report put them to him (`docs/reports/2026-11-24-preview-hosting-check.md`):
- Option (a), :284-286: "infra-engineer revises the draft's header row to judge production's own address only (a
  one-off address works like a preview address, and the header there keeps old builds out of search engines)".
- Option (b), :298-299: "both go back to the draft with the header row: the question is re-stated as one the dashboard
  cannot answer, and step 2.6 counts its own deploys and stays far below 500 a month whatever the answer".

## 2. Files touched (all docs; risk tier 1, needs-mustafa: his go on the final head)
- `docs/adr/0006-infrastructure-operations.md` gets edits E1 to E4, E6, E7, E9 and E10 (section 3). E5 is replaced
  by E10, and E8 is dropped.
  - The draft's own rule (:1411-1416) names infra-engineer as the author of a revision. Its agent file allows writes
    only under `docs/research/` and `.checkpoints/`. So infra-engineer drafts the paragraph (E7) and reviews the rest
    as the draft's author, writing into notes only; the main session places the text with the Edit tool.
- `docs/DECISIONS.md`: his message of 2026-11-25, 11:07:07 UTC, whole.
  - Copied by script from the chat's transcript and placed by time at the top.
  - Checked byte for byte: 741 bytes, sha256 ae5eb4f6....
  - Script: the scratchpad's `dec/add_1107.py`, same asserts as before (old lines kept, quoted block present). Its
    dry run adds 16 lines.
- Docs pass, lane 2's lines only. His record rules apply:
  - RECORD-FILES (a): each pull request writes only its own lines.
  - OLD-LINES (a): lines about merged pull requests move by script.
  - REC-OLD (a): a line that records what happened is never reworded; it gets a dated pointer.
  - `docs/STATUS.md`:
    - #66's row leaves "In flight" for the archive. Following #64's precedent (STATUS_ARCHIVE :176), its state cell
      becomes "merged: PR #66, 09faefa, 2026-11-25 11:10:11 UTC (`merged_at`), by merge commit". That corrects the
      "draft" he named.
    - This pull request's row is added.
    - "Next", item 3, moves on.
    - In "Waiting on Mustafa", lane 2's bullet becomes his go on this pull request, his word on step 4, and TR-END's
      clean-up: its condition is met, and the timing is his.
    - "Open risks": PHC-HEADER points to this revision.
  - `docs/STATUS_ARCHIVE.md`: the moved lines.
  - `docs/ROADMAP.md`: #66's entry marked merged; one entry for this pull request.
  - `docs/REPO_MAP.md`: the new report's row; the map check must print OK.
  - `docs/SPIKE_PLAN.md`, entry 6: one dated pointer. It gives PM-0006-2, -7 and -9 one at a time, each "on readings
    already taken, subject to his go", and never calls the check passed.
  - docs-keeper makes these edits; the main session checks each moved line verbatim.
- `docs/reports/2026-11-25-adr0006-header-row.md`: the change-set's report (`/advisor-report`).
  - It holds the before/after table and the measured numbers, so Mustafa and his Advisor can judge the row from the
    evidence alone. The draft's rule: "the revised part returns to the advisor for review and to Mustafa and the
    Advisor with the measured numbers" (:1415-1416).
  - It never calls the whole check passed: Safari stays "not checked, never as passed".

No code, CI, gate, kit, `.claude/`, `spikes/` or lane-1 file is touched. No Brave check: nothing a player sees, hears
or feels.

## 3. Every changed sentence, before and after (the "symbols" of this change)
Line numbers are at main 09faefa. Text in [square brackets] is shortened here; the edit keeps it unchanged.

**E1. PM-0006-2, the "number and statistic" cell (:1370).**
- Before: "100 % of responses carry their expected set: every `.wasm` as `application/wasm`; previews `X-Robots-Tag:
  noindex`, production none; the preview built with [the isolation flag] on, [...]"
- After: "100 % of responses carry their expected set: every `.wasm` as `application/wasm`; previews `X-Robots-Tag:
  noindex`; production none on its own address (the project's `pages.dev` host), while on production's one-off (hash)
  addresses `X-Robots-Tag` is recorded, not judged (revised 2026-11-25, Mustafa's ONE-OFF-HEADER (a): the preview
  hosting check read `noindex` on all 7 paths of production's one-off addresses 0354374e and de1c3e5e, written by no
  rule of the production build; a one-off address works like a preview address, and the header there keeps old builds
  out of search engines); the preview built with [the isolation flag] on, [...]"
- Every other header in the set stays judged on both URL forms, production's one-off addresses included.

**E2. PM-0006-2, the bites in "runs and noise" (same row).**
- Before: "[...] a `noindex` rule added to production"
- After: "[...] a `noindex` rule added to production (fails on production's own address)"
- Evidence that the bite survives: bite-production (run 44366750977) failed both production links on all 7 paths,
  production's own address included, and on nothing else (report, Appendix A :805-811).

**E3. Part 1, the build count (:140-143).**
- Before: "Pages' Free plan allows 500 builds a month (R6-HOST-F3, r6 lines 72–79), and whether a direct upload counts
  against them is a spike item (part 14). If uploads count, only the newest push of each pull request deploys (ADR
  0002's concurrency cancel) and the month's deploys are counted; a paid plan is Mustafa's after a price read
  (proposed, not researched)."
- After: "Pages' Free plan allows 500 builds a month (R6-HOST-F3, r6 lines 72–79). Whether a direct upload counts
  against them is not answered: the preview hosting check saw no build count in the dashboard, before or after its ten
  uploads (2026-11-18), and Cloudflare's docs do not say (read 2026-11-24, and again for this revision) (PM-0006-7;
  `docs/reports/2026-11-24-preview-hosting-check.md`). If uploads count, only the newest push of each pull request
  deploys (ADR 0002's concurrency cancel) and the month's deploys are counted; until someone shows a count, the
  previews (foundation step 2.6) work that way anyway (Mustafa's STEP4-COUNT (b), 2026-11-25). A paid plan is
  Mustafa's after a price read (proposed, not researched)."
- "Cloudflare's docs do not say" is read again by infra-engineer (its Q8) and spot-checked by the verifier.

**E4. PM-0006-7, the "number and statistic" cell (:1375).**
- Before: "[...] anything else recorded as read; if uploads count, part 1's consequence follows (only the newest push
  per pull request is deployed, and the month's deploys are counted)"
- After: "[...] anything else recorded as read; if uploads count, part 1's consequence follows (only the newest push
  per pull request is deployed, and the month's deploys are counted), and until a count is shown it holds anyway. Not
  answered by the preview hosting check (revised 2026-11-25, Mustafa's STEP4-COUNT (b)): the dashboard showed no
  count before or after the ten uploads (2026-11-18), and Cloudflare's docs do not say (2026-11-24)"

**E9. PM-0006-7, the "runs and noise" cell (same row).** The advisor agent's exact words (its condition 1).
- Before: "one pair of readings; passes when both are recorded"
- After: "one pair of readings; passes when both are recorded, each as the count the page shows or as "no count shown"
  (revised 2026-11-25: the advisor agent's reading of Mustafa's STEP4-COUNT (b), never his decision; "no count shown"
  leaves the question unanswered, part 1)"

**E10. The hosting checklist, step 4 (:1451-1452).** This replaces E5. The advisor agent's exact words (condition 1).
- Before: "[...] read the count again. Expected: two numbers recorded (PM-0006-7)."
- After: "[...] read the count again. Expected: two readings recorded, each a number or "no count shown" (PM-0006-7;
  revised 2026-11-25, the advisor agent's reading of Mustafa's STEP4-COUNT (b)). Read on 2026-11-18, before and after
  the ten uploads: no count shown, so the question is not answered (part 1)."

**E6. Spike 1's list (:1214).**
- Before: "- whether a direct upload counts against Pages' 500 builds a month (R6-HOST-REC1, r6 lines 633–634);"
- After: "- whether a direct upload counts against Pages' 500 builds a month (R6-HOST-REC1, r6 lines 633–634); not
  answered by the check, which saw no count (part 1);"

**E7. Part 1, a new short paragraph after its *Need:* line (:151).** This is the weighing his addition asks for, in at
most 8 lines.
- It opens with "*Workers static assets, weighed again after the header mark failed (2026-11-25, Mustafa's
  ONE-OFF-HEADER (a)):*".
- Its text is infra-engineer's draft from Cloudflare's docs, read on 2026-11-25. Every claim has its source and date.
- It must say plainly that none of the check's marks was measured on Workers.
- It answers Q1 (`noindex`) for the production Worker's own per-deployment and version URLs, the counterpart of the
  failed reading, and separately for preview and alias URLs and for `workers.dev`. A page that says nothing about
  `noindex` is written as "silent", never inferred from another page.
- It weighs what Workers would change for the header finding, and what it would gain or lose against part 1's needs:
  a link per pull request, deletion, the wall, `_headers`, limits and price.
- It ends with the outcome under his (a). If the reading favours a move to Workers, the paragraph does not decide it:
  the move is escalated to Mustafa before any edit, and E7 waits for his answer.
- If E7's final text changes what part 2 says about the wall, the security-auditor joins the review.
- The questions (notes: `infra-engineer-workers.md`):
  - Q1 `noindex`; Q2 `_headers` and host rules; Q3 a link per pull request; Q4 deletion; Q5 the Access wall;
  - Q6 limits, price and any deploy cap; Q7 Cloudflare's own stated direction; Q8 the Pages docs on direct uploads.

**Dropped: E8.** The record's Date line stays "2026-10-31". All six drafts carry that date, the six commits to 0006
never changed it, and the dated tags inside the changed cells are enough (the advisor agent, condition 3 of its
answers).

**Left alone on purpose**, each with its reason:
- The options row Q1 (:609): it gives the options as the research read them, and the check's facts go into part 1.
- The trade-off bullet (:725-732): it argues the choice from the research. The weighing goes where the draft's rule
  sends it, "the decision part the row names", which is part 1.
- "Revisit when" (:795-797): it names a PREV-DELETE fail only, and PREV-DELETE passed.
- Rollback (:1556-1558): the revised mark is met by readings already taken, so its trigger is not reached.
- "When a mark fails" (:1404-1418): unchanged. This revision is that rule carried out.
- PM-0006-9 (:1377) and its pass words: unchanged. It waits on his word on step 4.
- GLOSSARY §5 (`docs/research/GLOSSARY.md` :652-662): the header sets are what the build ships, and the host's own
  header is outside them.

Line shift: E3 and E7 add about 10 lines before :1404, so later line numbers move by that much. Other docs cite ADR
0006 by line only in dated records (the merged report, SPIKE_PLAN's dated pointer at :467, STATUS_ARCHIVE, a 2.3
report). They read at their own dates and are not edited. The report states the shift.

## 4. The pass words (settled by the advisor agent; no question to him before the build)
The advisor agent's reading (its condition 1, report :293-299): his STEP4-COUNT (b) reaches the pass words of PM-0006-7
and step 4, because "both" in option (b) means step 4 and the build-count mark, and "both go back to the draft [...]
the question is re-stated". It does not reach PM-0006-9's. Leaving both as they were would be his STEP4-COUNT (a),
"both stay open until a count source exists", which he passed over.

So E9 and E10 join this plan with the advisor agent's exact words, each tagged as its reading, never his decision.
"For Mustafa" asks him two things:
- his word on step 4 under E10 ("achieved" or "not yet"); nothing is written in his name;
- whether his go covers E9 and E10. One push removes them if he wants both left open.

Nothing calls the whole check passed. The report and the SPIKE_PLAN pointer give PM-0006-2, -7 and -9 one at a time,
each "on readings already taken, subject to his go".

## 5. Evidence (the change type: a decision-doc revision with research claims)
- Every claim in E3 and E7 cites its source: Cloudflare's official docs with the date read, or the merged report's
  Appendix A by section. The verifier spot-checks the key claims against their sources: the E7 claims, and "the docs
  do not say" in E3.
- Every number and date in E1, E2, E4, E9 and E10 is traced to Appendix A. The verifier checks them against the
  report's lines: the dashboard readings :706-757, both on 2026-11-18; the docs reading :1384-1388, 2026-11-24.
- The before/after table of section 3, final text, goes into the report whole, with each left-alone place and its
  reason.
- DECISIONS: the script's asserts, and the copy's sha256 equal to the transcript's.
- Docs pass: every moved line checked verbatim, except the state cell of #66's row, which is changed by his words
  ("correct that"); `node .claude/scripts/repo-map-check.mjs` OK.
- `node tools/gate.mjs`: PASS (docs only, so the smoke test may skip, as on #66). CI's gate and smoke green on the
  final head.

## 6. Reviews (one round on the final diff, one fix pass; then the advisor agent on the final head)
- test-engineer: the rows and the checklist step are its own (the ADR's header: "pass marks by test-engineer").
  - Does each revised mark still fail when it should? Does the bite still bite? Are E4, E9 and E10 clear?
  - The advisor agent's note for it, not a condition: on previews the host's own `noindex` may meet the expectation by
    itself, so no bite tests a missing preview `noindex` rule. The gap is older than this pull request; it becomes a
    note or a follow-up.
- infra-engineer: as the draft's author, reads the edits for consistency with part 1 and the rest of the draft.
- The verifier: spot-checks the sources and the numbers (section 5).
- The advisor agent: the final head. The security-auditor joins only on E7's condition (section 3).
- At most 3 agents at a time. Every agent runs on Opus with effort `xhigh`, the CLAUDE.md level for ADR work, passed
  by the caller.

## 7. Risks
- **R1.** Did the row get looser than his words? E1 judges "no noindex" on production's own address only, which is
  option (a)'s wording, and the advisor agent found it not looser. Every other header stays judged on both URL forms.
- **R2.** The Workers reading could favour a move. Then it goes to him before E7 is written. Nothing is moved, and the
  draft decides no move.
- **R3.** The pass words (section 4). E9 and E10 are the advisor agent's reading, tagged as such. His go covers them
  or removes them. The check is never called passed as a whole.
- **R4.** One branch.
  - Step 2.6's plan starts after this pull request merges (the advisor agent sets that order; it fits his "after it"
    of 11:07). He is told in one line that he can name a second branch if he wants it sooner.
  - Questions that wait on him (his rule of 2026-11-25, 01:35, item 6) are pushed in a commit before the fold and cited
    at that commit.
- **R5.** Line numbers in dated records drift (section 3). They are not edited, and the report states the shift.
- **R6.** The clean-up order is: the probe off (by him, before 2 December), then the archive (an archived repository is
  read-only), then the Cloudflare test project and its Access application deleted. The timing is his TR-END (a): "I
  archive the repository, and delete the project after the report is merged". That condition is met, so he is told it
  is met and the timing is his.

## 8. What Mustafa checks
- Nothing in Brave.
- His go on this pull request's final head, which is his go on the revised row. Step 2.6's first edit waits for it
  (his words). His go also covers E9 and E10, or he says to remove them.
- His word on step 4 under E10: "achieved" or "not yet".
- TR-END's clean-up, at his own timing.

## 9. Order of work
1. Done: the plan (revision 1) and the advisor agent's APPROVE with conditions; this revision folds them in.
2. Now: E1 to E4, E6, E9 and E10 placed by the main session; the gate. Then DECISIONS by script; the gate; a
   checkpoint save after each step.
3. When infra-engineer's research is in: E7 placed, or escalated if it favours a move.
4. The docs pass (docs-keeper), checked verbatim.
5. The review round (test-engineer, infra-engineer, the verifier), then a fix pass.
6. The report, then the checkpoint folded and deleted, then a draft pull request.
7. The advisor agent on the final head; ready; the full commit id to him; push nothing after it; his go.
