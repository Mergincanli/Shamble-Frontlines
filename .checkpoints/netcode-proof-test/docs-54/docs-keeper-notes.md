# docs-keeper notes: DOCS-54 plan draft

Task: draft the change-set plan for DOCS-54 (a) into docs-54-plan-draft.md. Read-only repo; write only these two files.

## Plan of work
1. Read main's STATUS, ROADMAP, DECISIONS, STATUS_ARCHIVE, REPO_MAP (git show origin/main:...), log -15.
2. List stale claims (file:line on origin/main) from #52, #53, #54.
3. Find Mustafa's words in history.md since 2026-11-14; classify main vs PR #40; line ranges.
4. Draft method, risks, evidence.

## Findings (a line each)
- main log: 82c605e #54 merge, 53272a1 #53 merge, 4460ef5 #52 merge. history.md 2227 lines.
- STATUS (main) stale: :5 Last updated (2026-11-14, #52, 4d21c1a); :13-14 Last merged #48; :15-19 kit (freeze ended
  "with this step"? his 2026-11-13 words; plus his two one-change exceptions #53/#54 missing); :21-23 CI green on
  4d21c1a; :28 #40 row stale (d0d1cfd); :29 #52 row (merged); :35-36 Next "#52 then its plan after #52 merges";
  :43 waiting: his go for #52; :56-65 Routines read 2026-11-10 (can't re-read; read-only -> [assumed]/leave dated).
  STATUS is 120 lines exactly: adding needs cuts.
- DECISIONS format: "- **Taken by Mustafa, <date>, <time> UTC: <title>** (source...). Copied by script from ..." then
  "> " lines. Newest first under "## Decisions, newest first" (:109). Queued (:20-107): KIT-FREEZE-2 exceptions live there.
- DISCREPANCY: the task's quoted DOCS-54 (a) text "(a) Yes. This chat opens one branch (claude/docs-after-54)..." is the
  advisor's draft (advisor-main-54-full.json:129 for_mustafa), NOT what history.md:2215 records as posed to him
  ("Options: (a) yes; ..."). His word is "I agree with all your recommendations, continue" (:2205). The branch name and
  method are the advisor's / session's, not his words. Open question.
- history.md decision blocks since 2026-11-14: 2039-2086 (11-14 SIZE-U4R...), 2088-2111 KEPT-LOGS, 2113-2129 C62H-REVIEW,
  2131-2146 KL-YES, 2148-2161 KL-CHANGE-YES, 2163-2175 KL-MERGE, 2177-2187 NS-STATUS, 2189-2197 NS-LINES,
  2199-2202 MERGE-54, 2204-2227 RECS-1043.
- ROADMAP Recent work: 12 entries (:117 #52 "in review" ... :217 kit-v3-2). Adding #53 and #54 -> fold kit-v3-2 (:217)
  and kit-v3-3 (:215) into Build history by line-range script. :97 "step 2.3 planned after ... (PR #52) merges" stale.
- ARCHIVE: In flight table :156-204, newest row :159 (#48). Add #54, #53, #52 rows above it. Follow-ups :216-870
  live; Open bugs :988-1198 live; Routines :926; Health :1199.
- ADV-M54-9 (Medium): STATUS:5,:12,:15,:29; DECISIONS:27-31 (KIT-FREEZE-2 exceptions), :110. ADV-NS-L1..L5 Lows
  (L1=NS-TRAILER, NS-FIRST from another finding?); RECS-1043 put NS-TRAILER/NS-FIRST (b) = follow-ups for kit after freeze.
- advisor docs54 ruling: opening the branch is his call; risk tier 1 decision doc, needs-mustafa.
- PR #40 branch edits main's STATUS :28 (#40 row), DECISIONS ~:723, archive :106, :289, top of "Open bugs" (:988) and
  top of Health (:1211), REPO_MAP. DOCS-54 must avoid those hunks (no insert at the top of Open bugs or Health).
- #52 merge commit 4460ef5 quotes his go for #52 (2026-11-14 09:02:29 UTC, foundation chat) whole with "> ": not on
  main's DECISIONS -> copy by script from the commit message (as #52 did for #48).
- #53 PR body lines 90-110: SA-KL-1..4, KLT-1..8 (accepted left open, KL-MERGE). #54 PR body :22-27: ADV-NS-L1..L5,
  SA-NS-1/2; L1 and SA-NS-1 became NS-TRAILER/NS-FIRST (his (b) at RECS-1043); L2=SA-NS-2, L3, L4 follow-ups; L5 = DOCS-54.
- Times: history.md headers use his own label times; the transcript stamps differ (KEPT-LOGS label 08:20 Amsterdam,
  typed 12:01:41 UTC). DECISIONS headers should use the transcript stamp and quote his label.
- Drift: START_HERE, MODULE_MAP, TECHNICAL, ART_DIRECTION, ANIMATION_PLAN: no hit for night shift/kept-logs/checkpoints
  [verified grep on origin/main]. REPO_MAP :80 CHECKPOINTS row lists up to §10, misses §11 (no path change).
- Foundation lane: claude/sweet-mayer-grsukw head 5f7f682 (2026-11-14 12:31 UTC) "plan revision 2.4 ... stopped for his
  go"; no open PR (REST: only #40 open).
- Transcript for a second check: /root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-...jsonl (local only).
- Classification done (see plan §2). Plan drafted next.
- DONE: plan written, docs-54-plan-draft.md, 134 lines (2026-11-17). Open questions Q1-Q4 in its §5, plus the DOCS-54
  wording discrepancy (§0). Nothing edited outside these two files.
