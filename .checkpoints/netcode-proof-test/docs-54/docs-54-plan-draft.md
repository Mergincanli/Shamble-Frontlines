# DOCS-54: main's docs after #52, #53 and #54 (plan draft, docs-keeper, 2026-11-17)
Branch `claude/docs-after-54` from origin/main 82c605e, in a worktree; his DOCS-54 (a) (history.md:2204-2227).
Docs only, risk tier 1 (it records his words): `needs-mustafa`, his go to merge. No kit, gate, CI, spike or code file.

## 0. What his word covers (read this first)
- His words (history.md:2205): "I agree with all your recommendations, continue". DOCS-54 as last posed to him
  (:2215): "may this chat open one docs-only branch and pull request to update main's dashboard and decision record for
  #52, #53 and #54? ... Options: (a) yes; (b) ...; (c) ... I recommend (a)." So: one docs-only branch and PR.
- The branch name, "plans first with the advisor", "one review round" and "waits for your go" come from the advisor's
  draft (advisor-main-54-full.json, docs54.for_mustafa) and the card (netcode-proof-test.md:204), not from the question
  he answered (:2215 has the short form). This plan keeps all four, since each is the protocol anyway; it never quotes
  the long form as his words.

## 1. What is out of date on main (origin/main 82c605e)
docs/STATUS.md (120 lines now: the cap; every addition below is paid for by a cut in the same file):
- :5 "Last updated: 2026-11-14 (step 2.3's first docs pass, PR #52; main 4d21c1a)" -> 2026-11-17, DOCS-54, main 82c605e.
- :13-14 "Last merged: foundation 2.2 ... (PR #48, 4d21c1a ...)" -> #54 (82c605e, 2026-11-17 07:03 UTC, merge commit, on
  his "Merge #54" after the advisor's and security-auditor's APPROVE WITH FINDINGS and the PR Advisor's APPROVE of
  20f9d59); before it #53 (53272a1, 2026-11-15 19:49 UTC, KL-MERGE (a)) and #52 (4460ef5, 2026-11-14 09:06 UTC).
  Times: GitHub `merged_at` [verified: REST, read 2026-11-17].
- :15-16 "The kit: KIT-FREEZE-2 ... his exceptions: DECISIONS.md, Queued" -> add his two one-change exceptions:
  KEPT-LOGS (#53, CHECKPOINTS.md §11, his KL-YES) and NS-STATUS (#54, NIGHT_SHIFT.md step 6). :17-19 (his 2026-11-13
  words on the freeze's end) stay: step 2.3 is planned, not built, so the freeze still stands.
- :21-23 "CI green on 4d21c1a (gate run 43461710700, toolchain run 43461710673)" -> CI on 82c605e (gate 44035715175,
  toolchain 44035715349 by history.md:2202; re-read with the REST check-runs API at build time, [verified] or left out).
- :29 the #52 row ("final (marked ready ...)", waiting on his go) -> removed (merged); its row goes to the archive.
  New row: this PR (DOCS-54). :28, the #40 row, is NOT touched: PR #40's branch edits that line (conflict at its next
  merge of main); #40's row stays #40's (open question Q2).
- :35-39 Next item 2: "step 2.3: its first docs pass (#52), then its plan after #52 merges, by his decision ..." -> #52
  merged; step 2.3's plan on `claude/sweet-mayer-grsukw` (its last commit 5f7f682, 2026-11-14 12:31 UTC: "plan
  revision 2.4 ... stopped for his go" [verified: git log, the commit's subject only]). His quoted 2026-11-13 decision
  (:36-39, 4 lines) becomes a pointer to DECISIONS.md, where #52 recorded it: this pays for the lines added above.
- :43 "His go for #52 ..." -> his go for this PR; step 2.3's plan waits on his go (that lane's word, by its commit).
- :73-75 Follow-ups, "Kit, after the foundation's gate step": add SA-KL-1 to -4 and KLT-1 to -8 (#53, his KL-MERGE (a):
  "the kit's next change after the freeze takes them"); ADV-NS-L2 (= SA-NS-2), -L3, -L4 (#54); NS-TRAILER and NS-FIRST
  (his (b): the kit's next change after the freeze, each with his yes to the exact text). ADV-NS-L5 closes with this PR.
- :56-65 Routines (read 2026-11-10, dated): left as they are unless the build session can read get_trigger [assumed
  stale in its "last run/next" figures; dated, so not false].
docs/ROADMAP.md (header :5: terse entries, 8-12 in "Recent work", fold the oldest into "Build history"):
- :117-124 the #52 entry "PR #52: in review ... Next: the advisor agent's review ..." -> MERGED, with 4460ef5, merged_at,
  his go of 2026-11-14 09:02:29 UTC after the advisor agent's APPROVE of bfd719b (merge commit 4460ef5's message).
- :97 "step 2.3 planned after its first docs pass (PR #52) merges" -> "#52 merged; step 2.3's plan waits for his go".
- New entries, newest first, above :117: DOCS-54 (in review), #54 (NIGHT_SHIFT.md step 6, MERGED), #53 (KEPT-LOGS,
  MERGED). 12 + 3 = 15, so the three oldest (:213 b1-plan-view, :215 kit-v3-3, :217 kit-v3-2, with their blank lines
  :212-217) move by script to the end of "Build history" (after :84), each line kept, "- " put in front of each.
docs/STATUS_ARCHIVE.md: "In flight" (:156-204): rows for #54, #53, #52 above :159 (newest first). "Follow-ups" (live):
  two subsections after :869, "From KEPT-LOGS (PR #53)" and "From the night-shift lines (PR #54)", their item lines
  copied by script from the PRs' descriptions (#53 body :90-105, #54 body :22-27, read with the REST API). Nothing goes
  at the top of "Open bugs and risks" (:988) or "Health" (:1211): PR #40 inserts there.
docs/DECISIONS.md: :110 no entry after 2026-11-13; :27-37 KIT-FREEZE-2's record names only "a broken main, and items 2
  and 3 below" (his words, kept) -> a dated pointer line under it to his two one-change exceptions (below).
docs/REPO_MAP.md: no path is added, moved or removed. One claim made untrue by #53: :80 (CHECKPOINTS.md's row) lists
  sections up to §10, not §11 (kept raw logs and the old build cache) -> add that clause [verified: CHECKPOINTS.md:231
  on main]; "Last checked" -> 2026-11-17 after `repo-map-check.mjs` passes (open question Q4).
Drift docs: START_HERE, MODULE_MAP, TECHNICAL, ART_DIRECTION, ANIMATION_PLAN: no claim on the night shift, the saver,
  kept logs or CHECKPOINTS [verified: grep on origin/main, 0 hits]; no edit.

## 2. His words: which go to main now, which stay with PR #40
Each entry joins DECISIONS.md "Decisions, newest first", above :110, in this order (newest first, by the transcript's
stamp; his own label time is quoted in the header, since history.md's headers use his labels: KEPT-LOGS is labelled
"08:20 (Amsterdam time)" but was typed 12:01:41 UTC). Ranges: history.md at this branch's 110c303 (or later, appended).
| # | Header time (UTC, transcript) | What | history.md lines copied | Lane lines left out (stay with #40) |
|---|---|---|---|---|
| E1 | 2026-11-17 10:43:48 | RECS-1043: DOCS-54 (a), NS-TRAILER (b), NS-FIRST (b) | 2205 (his); 2215, 2217, 2223, 2225 (the questions answered, the assistant's, marked so) | 2209-2214, 2216, 2222, 2224 (PACK-READ, C62HB-Q4); 2227 (the session's reading) |
| E2 | 2026-11-17 07:00:54 | MERGE-54, "Merge #54" | 2200 (his); 2201 (what it answered, the assistant's) | 2202 is the session's merge record: to the archive's #54 row, not DECISIONS |
| E3 | 2026-11-16 21:09:27 | NS-LINES (a), "yes" | 2190-2191, 2193-2197 (the three lines he said yes to, fenced) | none |
| E4 | 2026-11-16 20:48:10 | NS-STATUS (a), the freeze exception for #54, the approval-free command forms | 2180-2187 | none |
| E5 | 2026-11-15 19:46:36 | KL-MERGE (a), merge #53, SA-KL and KLT accepted as left open | 2166, 2168-2170 | 2172-2175 (C62H-BOUND-YES) |
| E6 | 2026-11-15 16:21:11 | KL-CHANGE-YES (a) | 2151-2161 | none |
| E7 | 2026-11-15 15:34:42 | KL-YES, the freeze exception for #53 | 2134, 2136-2139 | 2141-2146 (C62H-KEEP, C62H-BOUND) |
| E8 | 2026-11-15 12:34:01 | the kit rule's lines go to his Advisor first | 2116, 2129 | 2118-2127 (C62H-REVIEW, the marks.json read) |
| E9 | 2026-11-15 12:01:41 | KEPT-LOGS (c) and the kit-rule route | 2091-2107 | 2109 (the bot fix, its 1,400-line stop), 2111 (5e563b95) |
| E10 | 2026-11-14 09:02:29 | his go for #52 (foundation chat) | none: the "> " lines of merge commit 4460ef5's message, as #52 did for #48 | none |
Stay with PR #40 whole: 2039-2086 (2026-11-14 11:07: SIZE-U4R, SIZE-CHECK, COMPACT-WAIT, MAIN-52). SIZE-CHECK applies
his lane rule of 2026-11-10 (history.md:398-399) to the netcode units; COMPACT-WAIT is how this chat waits for his
/compact; MAIN-52 is when #40 takes main in. None is on main yet, so none has a main entry to extend (Q1).
Also: a dated pointer line under KIT-FREEZE-2 in "Queued" (:37) to E7 and E4 (a pointer, not his words); NS-TRAILER and
NS-FIRST join "Queued" as one pointer item to E1 (to carry out: the kit's next change after the freeze); KL-YES's
text and his three lines already stand in CHECKPOINTS.md §11 and NIGHT_SHIFT.md:45-47 (pointers in E7 and E3).
Each header follows the file's form: "- **Taken by Mustafa, <date>, <time> UTC: <title>** (the netcode proof test's
day session, session_0173vVo6igCNyQYBWPvhH36A, his message typed into that chat; copied by script from
`.checkpoints/netcode-proof-test/history.md` lines a-b at <sha> on PR #40's branch, checked byte for byte ...)", then
his lines with "> ". Lines left out are marked "[...]" in the header text, never inside his quote.

## 3. Method (one command per call, in forms that need no approval; a call that must ask is named first)
1. `git fetch origin main`, then `git worktree add -b claude/docs-after-54 /home/user/Shamble-Frontlines/.gate/docs-after-54-wt origin/main`
   (the main folder never switches). Checkpoint: `.checkpoints/docs-after-54.md` on that branch, saved after each step
   (`git add` + `git commit -F` with `[skip ci]` in the worktree; the saver's `save` if it works from a worktree
   [assumed]); deleted in the last commit before ready (CHECKPOINTS.md §9), checked with `git ls-tree`.
2. The copy script `.checkpoints/netcode-proof-test/docs-54/copy_words_54.py` (run `python3 -I`), written after the
   advisor's APPROVE of this plan. It: refuses to run unless the worktree's DECISIONS.md equals origin/main's blob
   (sha256) and holds no E1 header (so a second run exits 1); reads history.md with `git show <sha>:<path>`; for each
   block takes the exact line ranges above, removes only the two-space list indent (his paragraphs), adds "> ", and
   writes the headers and blocks above :110; reads the file back and, per block, strips "> " and checks the sha256
   equals the source's; checks E3's fenced lines against `git show origin/main:.claude/NIGHT_SHIFT.md` :45-47 byte for
   byte and the recorded 4e3d1565...; for E10 re-quotes the commit's lines and checks them byte for byte; second check
   (local evidence): each of his blocks equals the text of his message in the session's transcript at its stamp. It
   prints a table (block, range, sha256 source, sha256 copy, match) into the notes and the PR description.
3. The same script (or a second one) moves ROADMAP :212-217 by line range and copies the PR-body follow-up lines into
   the archive; STATUS, ROADMAP's new entries and the archive rows by Edit in the worktree.
4. Checks in the worktree: `node tools/gate.mjs --no-smoke` with NODE_PATH set to the main folder's `tools/node_modules`
   (the lockfiles byte-equal first, as for #54); `node .claude/scripts/repo-map-check.mjs`; `wc -l docs/STATUS.md` <=
   120; no file outside docs/ in `git diff --stat origin/main`.
5. `git commit -F <file>` (both attribution lines), `git push -u origin claude/docs-after-54`; the PR by REST
   (`gh api repos/Mergincanli/Shamble-Frontlines/pulls`, draft), label `needs-mustafa`.
6. One review round on the final diff: the verifier (his words byte for byte against history.md and the commit, the
   hash table re-run, the other claims) and the advisor agent (every pushed head). No type reviewer fits a docs-only
   record better; security-auditor is for tier 2. One fix pass; a Low becomes a follow-up.
7. Report into the PR description (/advisor-report form); the checkpoint deleted (last commit, CI runs); ready by the
   CCR route; the PR Advisor routine reviews it; his go in this chat for its head, after the advisor's APPROVE of that
   head, gate and smoke green, that SHA as the expected head; merge commit. Then the worktree removed
   (`git worktree remove`), and #40's card notes the merge (main is merged into #40 only by MAIN-52's rule).

## 4. Risks, size, evidence
- R1, his words misquoted or cut wrong (High if it happens): the hash table and the verifier; partial blocks name the
  lines left out in the header. R2, the lane split is a judgement (Q1): the plan lists every range on both sides.
- R3, the source's lasting proof: history.md is deleted before #40 is ready and #40 merges by squash (SQUASH-40), so a
  branch SHA may not stay reachable; each header also names the transcript stamp, and the hash table goes in the PR
  description and the commit message.
- R4, conflicts with PR #40: avoided by leaving STATUS :28, DECISIONS ~:723, the archive's :106, :289, :988 and :1211
  alone; this PR's DECISIONS hunks (above :110, under :37) are lines #40 does not edit, so git merges them cleanly
  [assumed until #40 next takes main in; checked then with `git merge-tree`].
- R5, STATUS's cap: draft text counted with `wc -l` before the commit; the cut is the :36-39 quote, now a pointer.
- R6, times: header times are the transcript's, his labels quoted; `merged_at` from REST.
- Size: about 230 changed lines (DECISIONS about +120, ROADMAP about +30 with 6 lines moved, the archive about +45,
  STATUS about ±20, REPO_MAP 1-2). Change type: docs, decision record. Evidence: the copy script's hash table (his words
  exact), its refusal on a second run (shown once), the gate and repo-map-check output, `wc -l`, the verifier's PASS.
- Mustafa: nothing to check in Brave (no player-facing change). His go to merge.

## 5. Open questions
- Q1: the 2026-11-14 message (SIZE-CHECK, COMPACT-WAIT, MAIN-52) stays with #40 whole? Recommended: yes.
- Q2: STATUS's #40 row on main stays stale (d0d1cfd) until #40 updates it? Recommended: yes, to avoid the conflict.
- Q3: a report file in docs/reports/ (adds a path) or the PR description only? Recommended: the PR description only.
- Q4: REPO_MAP :80's §11 clause is a description fix, not a path change: in scope? Recommended: yes, one clause.
