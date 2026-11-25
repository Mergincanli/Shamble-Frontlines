# docs-keeper: the docs pass for #67 (ADR 0006's small revision; lane 2)

Branch `claude/optimistic-albattani-mdoonz`, HEAD a4152ad, main 09faefa merged in (checked: `git merge-base
--is-ancestor 09faefa HEAD`). Facts checked on GitHub's REST API, 2026-11-25: #66 `merged_at` 2026-11-25T11:10:11Z,
merge commit 09faefa7f7cd1bca939bd2f98237e65e622d4737 (parents 28254a7 and d7937de), head d7937de; #67 open, draft.

## Rules that bound this pass (his words: `docs/DECISIONS.md`)
- RECORD-FILES (a): only lane 2's own lines (the preview hosting check #66, this PR #67).
- OLD-LINES (a): lines about merged PRs may move by script, byte for byte; every moved line listed below.
- REC-OLD (a): a line that records what happened or what he decided gets a dated pointer, never a rewording.
- STATUS at most 120 lines (STATUS-DIET).
- Left alone on purpose: lane 1's lines (the rows for #40 and change-set 3a, "Next" items 1-2, the "Waiting on" bullets
  for 3a and the netcode test, every lane-1 follow-up and risk); the shared lines: STATUS :5 "_Last updated" and :13
  "Last merged".

## Plan (edits in order; all by one Python script with asserts, in the scratchpad)
1. STATUS "In flight": #66's row (line 26) out; archive "## In flight" gets it at the top (before #64's row), the PR
   cell dropped, Work and Branch cells byte for byte, State cell and Waiting cell replaced (his "correct that", #64's
   precedent). #67's row in at the same place.
2. STATUS "Next" item 3: #66 merged (09faefa); #67 waiting for his go; step 2.6's plan after #67's merge.
3. STATUS "Waiting on Mustafa", lane 2's bullet: #67's go with PASS-WORDS; his word on step 4; the clean-up at his own
   timing.
4. STATUS "Follow-ups": new bullet "From ADR 0006's revision"; archive block after "From the preview hosting check"
   (texts from plan R3-7 and the review notes they cite); dated pointer on "in flight, its pull request #66".
5. STATUS "Open risks": PHC-HEADER revised by #67; archive PHC-HEADER gets a dated pointer line.
6. ROADMAP: #66's entry gets a dated pointer; new top entry for #67.
7. REPO_MAP: report row; `node .claude/scripts/repo-map-check.mjs` must print OK.
8. SPIKE_PLAN entry 6: one dated pointer after the existing one.

## Log (filled as each block lands)

### Block 1 (done, 2026-11-25): the script's run
Script: the session scratchpad's dk/pass.py (python3 -I; exact-string anchors, each asserted to occur once; bytes read and written as UTF-8; asserts: #66 row 5 cells in, 4 out; Work and Branch cells taken from the file; STATUS <= 120 and lines 5 and 13 unchanged; each in-line pointer is insertion only; ROADMAP Recent work 8-12 entries). Its log, verbatim:
```
STATUS In flight: removed (moved to the archive) the #66 row, sha 7eca4d8f9f5db5e2 len 778
  old state cell: draft; risk tier 1 (the report and his seventeen messages in `docs/DECISIONS.md`); main 28254a7 merged in
  old waiting cell: his answers of 2026-11-25 recorded, the last two at 01:35 Amsterdam (DEPARTURES item 5 (a), HISTORY-ID (a): a normal merge); the advisor agent's review of the final head; his go for that head
STATUS In flight: added the #67 row: | #67 | ADR 0006's small revision after the preview hosting check, by lane 2 (the chat he calls Builder V2), on his ONE-OFF-HEADER (a) and STEP4-COUNT (b): the header mark (PM-0006-2) judged on production's own addresses, with the reason in the row, and production's one-off links with the host's header recorded, not judged; part 1 weighs Workers static assets in a short paragraph from Cloudflare's docs read 2026-11-25 (nothing measured on Workers; in the draft, Pages stays); the build-count question said plainly to be not answered; the pass words of PM-0006-7 and checklist step 4 re-stated as the advisor agent's reading of his STEP4-COUNT (b), never as his decision; part 1's "no cap on previews" dated; report `docs/reports/2026-11-25-adr0006-header-row.md` | `claude/optimistic-albattani-mdoonz` (stands in for `cs/adr0006-header-row`) | a draft until the advisor agent approves its final head, then ready; risk tier 1 (an ADR draft, and his message of 2026-11-25, 11:07:07 UTC in `docs/DECISIONS.md`) | the advisor agent's review of the final head; his go, which is his go on the revised row and comes with his answer PASS-WORDS (does his go cover the re-stated words?); his word on checklist step 4 |
STATUS Next item 3: OLD
3. Lane 2 (Builder V2), in his order of 2026-11-25: #66; ADR 0006's small revision of the header row (its own pull
   request); step 2.6's plan (its first edit after his go on the revised row); the carry of his look decisions when his
   block comes; then one plan for the look test page and the model-check tool (his EARLY-LOOK (a)); nothing built
   before his go on that plan.
NEW
3. Lane 2 (Builder V2), in his order of 2026-11-25: #66 (merged, 09faefa); ADR 0006's small revision of the header row
   (#67, waiting for his go); step 2.6's plan (after #67's merge; its first edit after his go on the revised row); the
   carry of his look decisions when his block comes; then one plan for the look test page and the model-check tool (his
   EARLY-LOOK (a)); nothing built before his go on that plan.

STATUS Waiting on Mustafa, lane 2: OLD
- Lane 2's preview hosting check (#66): all his answers of 2026-11-25 are in DECISIONS (HISTORY-ID (a): a normal
  merge); his go for the final head after his main Advisor checks it. He switches off the test repository's hourly checker himself before 2 December. After the
  merge: archive the test repository, delete the test project and its Access application.
NEW
- Lane 2's ADR 0006 revision (#67): his go on the final head (his go on the revised row) with PASS-WORDS (does it
  cover the re-stated pass words?); his word on checklist step 4. At his own timing (TR-END met): the test repository's
  hourly checker off before 2 December, then its archive, then the test project and its Access application deleted.

STATUS Follow-ups: added after the preview hosting check bullet
- **From ADR 0006's revision** (#67, lane 2; the archive's block): ADR6-ROLLBACK (Medium), -WK-ROUTE, -WK-BIC,
  -PAGES-RETENTION, -PM7-PAGES, -PM7-FIGURE, -WORDING; for step 2.6 PHC-PREVIEW-NOINDEX (Medium); his: WORKERS-NEXT.

STATUS Open risks, lane 2: OLD
- **Preview hosting (lane 2):** PHC-HEADER, the header mark not met as written (back to the draft); PHC-PROBE-MAIL.
NEW
- **Preview hosting (lane 2):** PHC-HEADER, the header mark not met as written (back to the draft; revised by #67,
  waiting for his go); PHC-PROBE-MAIL.

STATUS lines: 117 -> 120
ARCHIVE In flight: inserted at the top the moved #66 row; Work cell sha e92ad100d6430581 and Branch cell byte for byte from STATUS; State and Waiting cells replaced:
  new state cell: merged: PR #66 (`claude/optimistic-albattani-mdoonz` (stands in for `cs/preview-hosting-check`), Builder V2), 09faefa, 2026-11-25 11:10:11 UTC (`merged_at`), by merge commit (a normal merge, his HISTORY-ID (a), on his go); risk tier 1 (the report and his seventeen messages in `docs/DECISIONS.md`)
  new waiting cell: none
ARCHIVE Follow-ups, dated pointer (insertion only): OLD
- **From the preview hosting check** (lane 2, the chat he calls Builder V2; in flight, its pull request #66;
NEW
- **From the preview hosting check** (lane 2, the chat he calls Builder V2; in flight, its pull request #66 (2026-11-25: merged, 09faefa);

ARCHIVE Follow-ups: new block after "From the preview hosting check"
- **From ADR 0006's revision** (lane 2, the chat he calls Builder V2; PR #67, a draft until the advisor agent
  approves its final head, then ready; branch `claude/optimistic-albattani-mdoonz` standing in for
  `cs/adr0006-header-row`; risk tier 1; review round 1's follow-ups as plan revision 3 lists them (its item R3-7),
  F3's and F7's short texts from test-engineer's notes; round 2's two optional Lows from infra-engineer (G1, G2);
  the report `docs/reports/2026-11-25-adr0006-header-row.md`; the ids are plan revision 3's and the report's):
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
  - ADR6-WK-ROUTE (infra-engineer; at R5's and R6's next revision): the R5 and R6 lines on `versions upload
    --preview-alias` and "no deletion read" get dated pointers.
  - ADR6-WK-BIC (infra-engineer): any re-run on Workers sends a descriptive User-Agent.
  - PHC-PREVIEW-NOINDEX (Medium; test-engineer's F1, older than this change): step 2.6's plan takes a build-output test
    that each preview mode's `_headers` carries the `noindex` rule and production's does not, with a bite; and, before
    any preview is served where the host adds no `noindex`, the preview expectation is read there.
  - WORKERS-NEXT (his, optional; the report's "Decisions needed"): keep Pages and refresh the Workers route at the
    draft's next revision, or run the check on Workers now.

ARCHIVE Open bugs and risks, PHC-HEADER: added line(s)
     _Pointer (2026-11-25): the header row is revised by #67 (ADR 0006's small revision,
     `docs/reports/2026-11-25-adr0006-header-row.md`), waiting for his go; by his ONE-OFF-HEADER (a) (2026-11-24,
     23:53:15 UTC) step 2.6's plan starts after #66, and its first edit waits for his go on the revised row._

ARCHIVE lines: 1877 -> 1915
ROADMAP fold (OLD-LINES, by script, byte for byte): PR #52 entry, ROADMAP lines 329-337 at HEAD (and the blank line 328), sha bab19135cab26fe9 bytes 960 moved to Build history after #48 entry (after line 173) with one blank line before it
  first line: **The foundation, 2.3: the first docs pass [2026-11-13 to 11-14, MERGED]** — foundation-2-3-docs (PR #52, 4460ef5,
  last line: `docs/reports/2026-11-13-foundation-2-3-docs.md`. Step 2.3's plan followed on its branch and waits for his go.
ROADMAP #66 entry: added pointer line(s)
_Pointer (2026-11-25): merged as PR #66, 09faefa, 2026-11-25 11:10:11 UTC (`merged_at`), by merge commit, a normal
merge on his go of 11:07:07 UTC (his HISTORY-ID (a)), with the expected head d7937de._

ROADMAP Recent work: new top entry
**ADR 0006's small revision after the preview hosting check [2026-11-25, in flight]** — adr0006-header-row, by
lane 2 (the chat he calls Builder V2), on his ONE-OFF-HEADER (a) and STEP4-COUNT (b) (docs only; an ADR draft, so his
go): the header mark (PM-0006-2) is judged on production's own addresses, with the reason in the row; production's
one-off links have the host's header recorded, not judged. Part 1 weighs Workers static assets in a short paragraph
from Cloudflare's docs read 2026-11-25: nothing was measured on Workers, and in the draft Pages stays. The build-count
question is said plainly to be not answered; the pass words of PM-0006-7 and checklist step 4 are re-stated as the
advisor agent's reading of his STEP4-COUNT (b), never as his decision; part 1's "no cap on previews" is dated; his
message of 2026-11-25, 11:07:07 UTC in `docs/DECISIONS.md`. Report `docs/reports/2026-11-25-adr0006-header-row.md`.
PR #67, a draft until the advisor agent approves its final head, then ready; waits for that review, his go (with
PASS-WORDS) and his word on checklist step 4.


ROADMAP Recent work entries: 12
ROADMAP lines: 337 -> 350
REPO_MAP: added row
| `docs/reports/2026-11-25-adr0006-header-row.md` | The report of ADR 0006's small revision after the preview hosting check (PR #67; lane 2): the header mark (PM-0006-2) judged on production's own addresses, with its reason in the row; the Workers static assets paragraph from Cloudflare's docs (nothing measured on Workers); the build-count question said to be not answered; the pass words of PM-0006-7 and checklist step 4 re-stated as the advisor agent's reading of his STEP4-COUNT (b), never his decision; the before/after table and the measured numbers behind the revised row |
SPIKE_PLAN entry 6: added pointer after the existing one
  _Pointer (2026-11-25, ADR 0006's small revision, PR #67; `docs/reports/2026-11-25-adr0006-header-row.md`), one mark
  at a time: PM-0006-2 is met under the revised row, on readings already taken, subject to his go; PM-0006-7 is
  complete as re-stated (the advisor agent's reading of his STEP4-COUNT (b), never his decision), with the question
  unanswered, on readings already taken, subject to his go; PM-0006-9 waits for his word on checklist step 4, on
  readings already taken, subject to his go. The check is not called passed: Safari stays "not checked, never as
  passed"._

WIDE 138 - **From the preview hosting check** (lane 2, the chat he ca
```

### Block 2 (done): after the script
- One Edit after the run: the archive block's header, "the ids are plan revision 3's and the report's" -> "the ids are the chat's, not his" (ADR6-WORDING came from the main session's message, not from R3-7).
- Coordinator additions folded in before the run: ADR6-PAGES-RETENTION names ADR 0006 line 67 (verified at ab15d6d and at HEAD: "with no cap on
  previews", section "The findings the decision turns on"), part 1's *Why:*, the options row (:631 at HEAD), R6-HOST-F3 (r6 :72-79, verified); #67's state written as the process ("a draft until the advisor agent approves its final head, then ready") in STATUS, the archive and ROADMAP; ADR6-WORDING (G1, G2: infra-engineer-review.md :145-152) in the archive and in STATUS's bullet.
- Verified after the run: #66's Work and Branch cells identical in the archive row (4 cells, last "none"); the folded #52 entry (ROADMAP :329-337 at HEAD, sha256 bab19135cab26fe9...) occurs once, now at :175, after #48's entry; STATUS 120 lines; git diff deletions only in STATUS's lane-2 lines, the ROADMAP fold, and the archive line that took the in-line pointer.
- repo-map-check: one line, "in the map but not in the repository: docs/reports/2026-11-25-adr0006-header-row.md" (exit 1); baseline at HEAD was OK (178 entries, 109 paths). Clears when the main session writes the report. No stub made.
- Left alone, by name: STATUS :5 and :13 (shared); lane 1's rows and lines; the archive's risk block header "(lane 2, in flight, 2026-11-24; ...)" for the check (not in the brief; a dated record); ROADMAP #66 header "[2026-11-15 to 11-24, in flight]" (record; the pointer line below says merged); REPO_MAP's #66 report row (describes the report, still true); the archive pointer line now 138 characters wide (insertion only, never re-wrapped).

### Block 3 (done): moved lines, verbatim (by command), and the last fixes
Moved 1: STATUS.md line 26 at HEAD a4152ad (the #66 row), out of STATUS:
```
| #66 | the preview hosting check (ADR 0006's spike 1, SPIKE_PLAN entry 6), by lane 2 (the chat he calls Builder V2), run in his test repository Mergincanli/shamble-preview-check; report `docs/reports/2026-11-24-preview-hosting-check.md`: the runs are done; by the draft's rule not passed yet (the header mark not met as written; the build-count mark and his verdict on checklist step 4 open) | `claude/optimistic-albattani-mdoonz` (stands in for `cs/preview-hosting-check`) | draft; risk tier 1 (the report and his seventeen messages in `docs/DECISIONS.md`); main 28254a7 merged in | his answers of 2026-11-25 recorded, the last two at 01:35 Amsterdam (DEPARTURES item 5 (a), HISTORY-ID (a): a normal merge); the advisor agent's review of the final head; his go for that head |
```
Into STATUS_ARCHIVE.md "## In flight", its top row (line 176 now), PR cell dropped, Work and Branch cells byte for byte, State and Waiting cells replaced (his "correct that"):
```
| the preview hosting check (ADR 0006's spike 1, SPIKE_PLAN entry 6), by lane 2 (the chat he calls Builder V2), run in his test repository Mergincanli/shamble-preview-check; report `docs/reports/2026-11-24-preview-hosting-check.md`: the runs are done; by the draft's rule not passed yet (the header mark not met as written; the build-count mark and his verdict on checklist step 4 open) | `claude/optimistic-albattani-mdoonz` (stands in for `cs/preview-hosting-check`) | merged: PR #66 (`claude/optimistic-albattani-mdoonz` (stands in for `cs/preview-hosting-check`), Builder V2), 09faefa, 2026-11-25 11:10:11 UTC (`merged_at`), by merge commit (a normal merge, his HISTORY-ID (a), on his go); risk tier 1 (the report and his seventeen messages in `docs/DECISIONS.md`) | none |
```
Moved 2 (OLD-LINES, beyond the brief: Recent work would hold 13 entries, docs-keeper's rule is 8-12): ROADMAP.md lines 328-337 at HEAD (a blank line and PR #52's entry, a merged PR, not a lane-1 line: it predates the lanes), byte for byte, to Build history after #48's entry (now lines 174-183):
```

**The foundation, 2.3: the first docs pass [2026-11-13 to 11-14, MERGED]** — foundation-2-3-docs (PR #52, 4460ef5,
2026-11-14 09:06:30 UTC by `merged_at`, by merge commit, on Mustafa's go of 09:02:29 UTC, quoted whole in its message,
after the advisor agent's APPROVE of bfd719b; CI green on main), by
Mustafa's words of 2026-11-13 (docs only, no kit, gate or CI file): his go for #48 and his decision on step 2.3 go into
`docs/DECISIONS.md` word for word, by script; dated pointers of 2026-11-13 in SPIKE_PLAN, ADR 0002, DECISIONS
(SLICE-ORDER condition 3) and ROADMAP, his decision quoted in STATUS; the 2.2 report's fixes M1, L1, L2 (worded as the
CI logs read), L3 and L5, the archive's lines on the same claims, on #48's merge and on his go for it, and two pointing
clauses; STATUS, ROADMAP and the archive made true for main 4d21c1a. Report
`docs/reports/2026-11-13-foundation-2-3-docs.md`. Step 2.3's plan followed on its branch and waits for his go.
```
- Edit after Block 2: the archive PHC-HEADER pointer, "23:53:15 UTC) step 2.6's plan starts after #66, and its first edit waits for his go on the revised row._" -> "23:53:15 UTC) step 2.6's first edit waits for his go on the revised row._" (the advisor tool: avoids a clash with STATUS's "after #67's merge"; both were true to different messages).
- ADR6-PAGES-RETENTION's line-67 item: the verifier's round 2 notes say so (verifier.md :156-157, :198).
- Grep for stale open-state words about #66 in the five files: the archive's risk block header :1837 "(lane 2, in flight, 2026-11-24; ...)" and ROADMAP :236's header "[2026-11-15 to 11-24, in flight]" (records; left, the latter has the merged pointer below it). ROADMAP :197 (the spike order, 3f98c0b) is not about #66's state.
- Not confirmed by me: the REPO_MAP row describes a report not yet written (from the plan and report-draft.md); the Cloudflare quotations in the archive block come from the plan (R3-10), infra-engineer's notes and the main session's messages, not read by me.
