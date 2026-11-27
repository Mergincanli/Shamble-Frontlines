# docs-keeper: step 2.6's docs pass (lane 2), on head 8c8ad0d0

## Plan
1. STATUS (120 lines now, cap 120): lane 2's lines only.
   - In flight: one row for #72 (draft; rounds 1 and 2 closed; stopped until GUARD-OFF; token sitting waits for the
     Guard fix chat's protection for main; first live link needs the CI Builder's hand-over step, asked on #59;
     token end date at the token sitting).
   - Next item 3 (lane 2): #67 merged; step 2.6 in flight; compressed to free lines.
   - Waiting on Mustafa, lane 2's line (#67 stale): his five open questions (GUARD-OFF, MAIN-WALL-AGAIN, GUARD-KIT,
     GIT-IN-NODE, TAG-CHECK), open, not decided; TR-END sentence kept byte for byte.
   - Follow-ups: one bullet "From step 2.6 (#72)" and the guard kit item.
   - Open risks: lane 2's preview line takes the report's three risks.
   - Keep the file at <= 120 lines.
2. ROADMAP: one entry at the top of "Recent work" (built, reviewed, not merged); 13 entries then: move the oldest
   (cs0) byte for byte into "Build history" by script.
3. REPO_MAP: the report's row (docs/reports/2026-11-26-step-2-6-previews.md is not listed); wall-test.yml's row says
   "removed before the pull request's final review", untrue now (stays until the token sitting): fix if in scope.
   Then run repo-map-check.

## Notes
- Breach of the interim git rule: one call `git -v 2>/dev/null; git log --oneline -5` chained two git calls in one
  Bash command. Every later git call alone.
- DONE STATUS (120 lines): :5 Last updated; :26 #72 row; :38 Next 3 (3 lines to 1); :50 Waiting (first sentence replaced, TR-END kept byte for byte on :51); :104 From step 2.6 follow-ups + kit guard item; :118 risks. :102-104 #67 follow-ups untouched.
- Also: two Bash calls used `cd /home/user/Shamble-Frontlines && ...` (repo-map-check, a REPO_MAP grep), against the brief's never-cd; no git in them.
- DONE ROADMAP: cs0 (PR #56, merged) moved byte for byte by scripts/fold_cs0.py into Build history (:202-208; OLD-LINES (a): the report must list it); step 2.6 entry :255-264; #67 pointer :290-291; Recent work 12 entries.
- DONE REPO_MAP: report row :156 (after ci-after-3a's); wall-test row left (not a missing path); map check: OK, 204 entries cover 114 required paths.
- DONE wording fixes (advisor): STATUS :104 kit item 'who takes it is his GUARD-KIT, open', 'lane 2's chat (Builder V2)'; :118 risk 2 as a pointer to the report; ROADMAP :255/:257 'no live link yet', 'is to get'. STATUS still 120 lines. c1da1bd confirmed with git show: #67's merge, 2026-11-25 18:12:59 +01:00.
