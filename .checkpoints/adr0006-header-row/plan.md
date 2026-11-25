# Plan: ADR 0006's small revision after the preview hosting check (lane 2, Builder V2)

Revision 1, 2026-11-25. Branch `claude/optimistic-albattani-mdoonz`, standing in for `cs/adr0006-header-row`; this
cloud session may push only this branch. Base: main 09faefa (#66's merge). Checkpoint:
`.checkpoints/adr0006-header-row.md`.

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

Option (a), as the report put it to him (`docs/reports/2026-11-24-preview-hosting-check.md` :284-289), was:
"infra-engineer revises the draft's header row to judge production's own address only (a one-off address works like a
preview address, and the header there keeps old builds out of search engines)".

Option (b): "both go back to the draft with the header row: the question is re-stated as one the dashboard cannot
answer, and step 2.6 counts its own deploys and stays far below 500 a month whatever the answer" (:298-299).

## 2. Files touched (all docs; risk tier 1)
- `docs/adr/0006-infrastructure-operations.md` gets edits E1 to E8 below.
  - The draft's own rule (:1411-1416) names infra-engineer as the author of a revision. Its agent file allows writes
    only under `docs/research/` and `.checkpoints/`. So infra-engineer drafts the paragraph (E7) and reviews the rest
    as the draft's author, writing into notes only; the main session places the text with the Edit tool.
- `docs/DECISIONS.md`: his message of 2026-11-25, 11:07:07 UTC, whole, copied by script from the chat's transcript and
  placed by time at the top, checked byte for byte with its sha256. Same script and checks as his 00:55 and 01:35
  messages (`add_0135.py`'s asserts: old lines kept, quoted block present).
- Docs pass, lane 2's lines only (RECORD-FILES (a); OLD-LINES (a) for lines about merged pull requests; REC-OLD (a):
  a line that records what happened is never reworded, it gets a dated pointer):
  - `docs/STATUS.md`:
    - The #66 row leaves "In flight": it is merged at 09faefa. Its "draft" word is corrected by this move, as he asked.
    - This pull request's row is added.
    - "Next" item 3 moves on one step.
    - In "Waiting on Mustafa", lane 2's bullet becomes his go on this pull request plus the clean-up order.
    - In "Open risks", PHC-HEADER points to this revision.
  - `docs/STATUS_ARCHIVE.md`: the moved lines, byte for byte.
  - `docs/ROADMAP.md`: #66's entry marked merged, and one entry for this pull request.
  - `docs/REPO_MAP.md`: the new report's row; the map check must print OK.
  - `docs/SPIKE_PLAN.md`, entry 6: one dated pointer to the revision.
  - docs-keeper makes these edits; the main session checks each moved line verbatim.
- `docs/reports/2026-11-25-adr0006-header-row.md`: the change-set's report, made with `/advisor-report`. It includes
  the before/after table of section 3 and the measured numbers, so Mustafa and his Advisor can judge the row from the
  evidence alone (the draft's rule: "the revised part returns to the advisor for review and to Mustafa and the Advisor
  with the measured numbers", :1415-1416).

No code, CI, gate, kit, `.claude/`, `spikes/` or lane-1 file is touched. No Brave check: nothing a player sees, hears
or feels.

## 3. Every changed sentence, before and after (the "symbols" of this change)
Line numbers are at main 09faefa. Text in [square brackets] is shortened here; the edit keeps it unchanged.

**E1. PM-0006-2, the "number and statistic" cell (:1370).**
- Before: "100 % of responses carry their expected set: every `.wasm` as `application/wasm`; previews `X-Robots-Tag:
  noindex`, production none; the preview built with [the isolation flag] on, [...]"
- After: "100 % of responses carry their expected set: every `.wasm` as `application/wasm`; previews `X-Robots-Tag:
  noindex`; on production's own address none, while on production's one-off (hash) addresses `X-Robots-Tag` is
  recorded, not judged (revised 2026-11-25, Mustafa's ONE-OFF-HEADER (a): the preview hosting check read `noindex` on
  all 7 paths of production's one-off addresses 0354374e and de1c3e5e, written by no rule of the production build, so
  the host adds it; a one-off address works like a preview address, and the header there keeps old builds out of
  search engines); the preview built with [the isolation flag] on, [...]"
- Why "recorded, not judged" and not "noindex expected": the host's own header is not ours to require. If Cloudflare
  ever drops it, the mark must not fail for that reason (the advisor tool's reading, 2026-11-25). Every other header
  in the set stays judged on both URL forms, production's one-off addresses included.

**E2. PM-0006-2, the bites in "runs and noise" (same row).**
- Before: "[...] a `noindex` rule added to production"
- After: "[...] a `noindex` rule added to production (fails on production's own address)"
- Evidence that the bite survives the revision: bite-production (run 44366750977) failed both production links on all
  7 paths, production's own address included, and on nothing else (report, Appendix A :805-811).

**E3. Part 1, the build count (:140-143).**
- Before: "Pages' Free plan allows 500 builds a month (R6-HOST-F3, r6 lines 72–79), and whether a direct upload counts
  against them is a spike item (part 14). If uploads count, only the newest push of each pull request deploys (ADR
  0002's concurrency cancel) and the month's deploys are counted; a paid plan is Mustafa's after a price read
  (proposed, not researched)."
- After: "Pages' Free plan allows 500 builds a month (R6-HOST-F3, r6 lines 72–79). Whether a direct upload counts
  against them is not answered: the preview hosting check saw no build count on any page it read, before or after its
  ten uploads, and Cloudflare's docs do not say (PM-0006-7; `docs/reports/2026-11-24-preview-hosting-check.md`). Until
  someone shows a count, the previews (foundation step 2.6) work as if uploads count: only the newest push of a pull
  request is deployed (ADR 0002's concurrency cancel), and the deploys are counted per month (Mustafa's STEP4-COUNT
  (b), 2026-11-25); a paid plan is Mustafa's after a price read (proposed, not researched)."
- "Cloudflare's docs do not say" is read again now by infra-engineer (its Q8), and the verifier spot-checks it.

**E4. PM-0006-7, the "number and statistic" cell (:1375).**
- Before: "[...] anything else recorded as read; if uploads count, part 1's consequence follows (only the newest push
  per pull request is deployed, and the month's deploys are counted)"
- After: "[...] anything else recorded as read; part 1's consequence holds until a count is shown (only the newest
  push per pull request is deployed, and the month's deploys are counted). Not answered by the preview hosting check
  (revised 2026-11-25, Mustafa's STEP4-COUNT (b)): no count was shown on any page read, before or after the ten
  uploads (2026-11-18), or on 2026-11-24"
- The "runs and noise" cell ("one pair of readings; passes when both are recorded") is NOT changed. See section 4.

**E5. The hosting checklist, step 4 (:1451-1452).**
- Before: "[...] read the count again. Expected: two numbers recorded (PM-0006-7)."
- After: "[...] read the count again. Expected: two numbers recorded (PM-0006-7). (Read on 2026-11-18 and 2026-11-24:
  no count shown, so the question is not answered: part 1.)"

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
- It weighs what Workers would change for the header finding, and what it would gain or lose against part 1's needs:
  a link per pull request, deletion, the wall, `_headers`, limits and price.
- It ends with the outcome under his (a). If the reading favours a move to Workers, the paragraph does not decide it:
  the move becomes a decision for Mustafa (escalated before any edit), and E7 waits for his answer.
- The questions infra-engineer answers (its notes: `.checkpoints/adr0006-header-row/infra-engineer-workers.md`):
  - Q1: does Workers add `noindex` to preview, version, alias or `workers.dev` links?
  - Q2: does `_headers` apply there, and can it match by host?
  - Q3: a link per pull request.
  - Q4: deletion.
  - Q5: the Access wall.
  - Q6: limits, price and any deploy cap.
  - Q7: Cloudflare's own stated direction.
  - Q8: do the Pages docs say whether direct uploads count?

**E8. The record's Date line (:4).** Proposed; the advisor agent decides whether it belongs.
- Before: "**Date:** 2026-10-31"
- After: "**Date:** 2026-10-31; revised 2026-11-25 after the preview hosting check (part 1, PM-0006-2, PM-0006-7, the
  hosting checklist's step 4, spike 1's list)"

**Left alone on purpose**, each with its reason:
- The options row Q1 (:609): it gives the options as the research read them, and the check's facts go into part 1.
- The trade-off bullet (:725-732): it argues the choice from the research. The weighing goes where the draft's rule
  sends it, "the decision part the row names", which is part 1.
- "Revisit when" (:795-797): it names a PREV-DELETE fail only, and PREV-DELETE passed.
- Rollback (:1556-1558): "move to Workers static assets" is how a move would go once one is decided. Under his (a)
  none is.
- "When a mark fails" (:1404-1418): unchanged. This revision is that rule carried out.
- PM-0006-9 (:1377) and its pass words: unchanged (section 4).
- GLOSSARY §5 (`docs/research/GLOSSARY.md` :652-662): the header sets are what the build ships. The host's own
  header is outside them.

Line shift: E3 and E7 add about 10 lines before :1404, so later line numbers move by that much. Other docs cite ADR
0006 by line only in dated records (the merged report, SPIKE_PLAN's dated pointer at :467, STATUS_ARCHIVE, a 2.3
report). They read at their own dates and are not edited. The report states the shift.

## 4. One question for the advisor agent before any edit (the pass words)
His words cover saying plainly that the question is not answered (E3 to E6), and "as if uploads count" made
unconditional (E3, E4). They do not plainly cover changing what counts as passing:
- PM-0006-7: "passes when both are recorded";
- step 4: "Expected: two numbers recorded";
- through step 4, PM-0006-9: "every step [...] "achieved"".

With those words unchanged, the check stays "not passed" on PM-0006-7 and PM-0006-9 after this revision. The pull
request must not call the check passed. Option (b), as put to him, said "the question is re-stated as one the
dashboard cannot answer". That may or may not reach the pass words.

My reading: it does not plainly reach them. So it is a "Decisions needed" entry for him, named **CHECK-CLOSE**, asked
before the build so that his answer lands in this pull request:
- (a) Re-state PM-0006-7 and step 4 as "record what the pages show: a count, or 'none shown'". The readings already
  taken close PM-0006-7, and his verdict on the re-stated step 4 is asked.
- (b) Leave both as they are, so the check stays "not passed" on the build count until someone shows a count. Step 2.6
  does not wait for it: his words make its first edit wait for his go on the revised row.

My recommendation is (a): the question stays plainly unanswered in part 1 either way, and (a) lets the check's record
close on readings he already took. If the advisor agent reads his (b) as covering the pass words, E9 and E10 (the
re-stated words, shown before and after) join this plan, and no question goes to him.

## 5. Evidence (the change type: a decision-doc revision with research claims)
- Every claim in E3 and E7 cites its source: Cloudflare's official docs with the date read, or the merged report's
  Appendix A by section. The verifier spot-checks the key claims against their sources: the E7 claims, and "the docs
  do not say" in E3.
- Every number in E1, E2, E4 and E5 is traced to Appendix A: the verifier checks them against the report's lines.
- The before/after table of section 3, final text, goes into the report whole, with each left-alone place and its
  reason.
- DECISIONS: the script's asserts (old lines kept, quoted block present), and the copy's sha256 equal to the
  transcript's.
- Docs pass: every moved line checked byte for byte; `node .claude/scripts/repo-map-check.mjs` OK.
- `node tools/gate.mjs`: PASS (docs only, so the smoke test may skip, as on #66), and CI's gate and smoke green on the
  final head.

## 6. Reviews (one round on the final diff, one fix pass; then the advisor agent on the final head)
- test-engineer: the rows and the checklist step are its own (the ADR's header: "pass marks by test-engineer").
  Checks: does each revised mark still fail when it should? Does the bite still bite? Is the plain statement in E4 and
  E5 clear?
- infra-engineer: as the draft's author, reads E1 to E8 for consistency with part 1 and the rest of the draft.
- The verifier: spot-checks the sources and the numbers (section 5).
- The advisor agent: this plan, then the final head. The security-auditor is not needed: tier 1, and nothing
  changes for network, secrets, storage or CI. The advisor may add it.
- At most 3 agents at a time. Every agent runs on Opus with effort `xhigh`, the CLAUDE.md level for ADR work, passed
  by the caller (the advisor agent included).

## 7. Risks
- **R1.** Did the row get looser than his words? E1 judges "no noindex" on production's own address only, which is
  his option (a)'s wording. Every other header stays judged on both URL forms. The report shows each changed word next
  to his words.
- **R2.** The Workers reading could favour a move. Then it goes to him before E7 is written. Nothing is moved, and
  the draft decides no move.
- **R3.** The pass words (section 4). The pull request never calls the check passed unless his CHECK-CLOSE answer and
  the readings make it so.
- **R4.** One branch. Once the final head is named, nothing may be pushed to it.
  - Waiting questions (his rule of 2026-11-25, 01:35, item 6) are pushed in a commit before the fold and cited at that
    commit.
  - Step 2.6's plan cannot be checkpointed on this branch while the revision waits for his go. He is asked where it
    is written (a second branch he names, or after this merge).
- **R5.** Line numbers in dated records drift (section 3). They are not edited, and the report states the shift.
- **R6.** The clean-up. The probe is switched off by him before 2 December (his words). My go for archiving the test
  repository and deleting the Cloudflare test project and its Access application comes with his go on the revised row,
  in case a re-reading is wanted during the review; reading needs the project. The probe goes off before the archive,
  because an archived repository is read-only.

## 8. What Mustafa checks
- Nothing in Brave.
- His go on this pull request's final head, which is his go on the revised row. Step 2.6's first edit waits for it
  (his words).
- His answers to CHECK-CLOSE (unless the advisor agent finds it covered) and to the branch question for step 2.6's
  plan. Both are saved in `.checkpoints/adr0006-header-row/questions-2026-11-25.md`, pushed, and named with the commit
  in the message.

## 9. Order of work
1. This plan, then the advisor agent's review (xhigh). infra-engineer's research runs meanwhile (notes only).
2. The questions to him, saved and pushed (section 8).
3. After APPROVE, and after his CHECK-CLOSE answer where needed: E1 to E8 (plus E9 and E10 if (a)), placed by the main
   session. Then DECISIONS by script, then the docs pass. The gate after each step; a checkpoint save after each step.
4. The review round (test-engineer, infra-engineer, the verifier), then a fix pass.
5. The report, then the checkpoint folded and deleted, then a draft pull request.
6. The advisor agent on the final head; ready; the full commit id to him; push nothing after it; his go.
