Task: docs-keeper, A3 (kit-v3-3, PR #38) final docs pass after fix pass 4 (head 0c481e1). Edit STATUS/ROADMAP only (REPO_MAP if untrue).

Plan (blocks, in order):
B1 read sources (advisor-pass4 §5/§6, mustafa-words last entry, queue a4-b1, pass-4 reports, builder-pass4)
B2 read STATUS + ROADMAP current A3 text
B3 edits: R-PLAIN replace; In flight row (fix pass 4 summary + state); header; Phase In flight lines
B4 Follow-ups FU-P4a/b/c
B5 Queued in Mustafa's words: A4-FREEZE, B.1 list; fix queue-order line
B6 ROADMAP A3 entry
B7 line refs re-verify (routine-guard, settings.json, checkpoint.mjs, NIGHT_SHIFT)
B8 REPO_MAP check
B9 word-for-word check script, results

Notes:
- B1 done: sources read. advisor-pass4 s5 (R-PLAIN line :200-210, fix-pass-4 summary :212-220), s6 FU-P4a :235-239, FU-P4b :240-246, FU-P4c :247-252; queue a4-b1 text block :8-17; SA pass4 FAIL (S4 crit; S1,S2,S3,S5 high; S4,S1,S3,S5 on main 12d7961; S6 med, S7/S8 low); verifier PASS WITH NOTES (V1 obs, V2-V4 low); auto-6 #14 MEDIUM day (of 9589bfb). Builder: suites 1188/1188 (1034/127/27); role decisions 1348 looser 0 stricter 59; row decisions 983 kind looser 0, 110 moved; main's file 179/192 (one new #181); 6 rows moved from main MUTANTS (4 to PLAIN_BEHIND as R-PROG, 2 to new KIT_BEHIND_MUTANTS, named outside the ruling's literal permission); teeth 4/4; gate PASS with smoke; PASS4_ROUTINE_CASES 98 rows, 12 PASS4 mutants.
- B2 done: STATUS A3 places: header :3; Phase In flight :25-28, queue :28-38 (A4 "after A3" at :30); In flight row :96; WT-NIGHT ref :157 (NIGHT_SHIFT.md:27-31) and :149 (:25-36); Follow-ups :324-392 (FU-P3a at :389-391); Queued :393-432 (A4 "Queued after A3" :395; B.1/B.2 :417-432); Open bugs A3 :681-781, R-PLAIN :757-762. Line refs to check: guard :694 :818 :892 :1323-1346 :306 :329 :333 :906 :141-153 :202-249 :625-634 (+:446 at :222, main's :594); settings :243-246 :50-51 :272; checkpoint :30-40, main's :30; NIGHT_SHIFT :25-36, :27-31.
- B7 done (line refs at 0c481e1, checked by reading the lines): guard :141-153, :202-249, :306, :329, :333 unchanged (right); :625-634 D-ASK -> :670-679; :694 KIT_AGENT -> :741; :818 shell-write ask -> :865; :892 dayEdit ask -> :939; :906 python refusal -> :953; :1323-1346 ACTIONS_READ..dispatch -> :1373-1396; main's :594 GITHUB_READ right. settings.json :50-51, :243-246, :272 right (file unchanged). checkpoint.mjs :30-40 right, main's :30 right. guard.mjs:46-48 right. NIGHT_SHIFT.md: step 4 :25-36 right; switch text :27-30 ("Switch to" :27 ... "needed)" :30) -> STATUS:157 fix to :27-30. :222's routine-guard.mjs:446 is in the 2026-11-04 #35 item (main), not A3's entry: left.
- Edits still to make for B7: STATUS:157, :683 ("at fix pass 3" -> fix pass 4), :686, :692, :696, :766-767.
- B3a done: R-PLAIN line replaced whole (STATUS Open bugs, A3 entry, was :757-762).
- B4 done: FU-P4a/b/c added under a new 'From A3's pass-4 ruling' line after FU-P3a in STATUS Follow-ups.
- B5 done: Queued: A4 line 'Queued after A3' -> waits for the freeze; A4-FREEZE quote added under A4; B.1's addition quote (9 lines, wrapped, '> ' prefixed) after B.1/B.2. Phase In flight lines + Queue order rewritten (after A3: B.1 with his editor decisions, then B.2; A4 waits).
- B3 done: header :3 (pass 4 failed -> fix pass 4 0c481e1; next verifier check); In flight row: pass 4, ruling + A4-1, fix-pass-4 summary verbatim with commit 0c481e1, builder counts [assumed], Waiting on rewritten. Finding: builder moved 6 main mutation rows (2 to KIT_BEHIND, outside ruling's literal permission) - noted in row; report/advisor should weigh.
- B7 edits done: STATUS WT-NIGHT ref -> NIGHT_SHIFT.md:27-30 [verified at 0c481e1] + "fix pass 4 leaves the switch check as it is" (diff has no switchTargetCheck hunk, :1154); A3 open-bugs header adds fix pass 4 and "line numbers at fix pass 4, 0c481e1"; :694/:818/:892 -> :741/:865/:939; :1323-1346 -> :1373-1396; :906 -> :953; :625-634 -> :670-679.
- FINDING (drift, not edited: advisor says the final pass writes only its lines; earlier follow-ups stand): STATUS Follow-ups "Worktrees under `.gate/`: a routine's write, commit and push checks all miss kit files there (follow-up)" and "SA-3's code fix ... first in the next kit change-set" are now closed for routines by fix pass 4's worktree path rule (kitName at commit check + protectedHit, kitPath in writePath/writeTarget). Also "Stated limits" intro does not name the pass-4 ruling. Report to caller.
- B6 done: ROADMAP A3 entry (:93) title + pass 4/ruling/fix pass 4/waits; :73 queue order (B.1, B.2; A4 waits). 11 entries, no fold needed.
- B8 done: REPO_MAP rows still true (routine-guard row covers R-PROG as the deny-by-default shell); repo-map-check: OK, 113 entries cover 100 required paths. No REPO_MAP edit.
- B9 done: dk-final-check.py (read-only) on docs/STATUS.md: FOUND x8, each 1 occurrence (R-PLAIN 1238 chars; fix-pass-4 summary 889; FU-P4a 499; FU-P4b 672; FU-P4c 659; A4-FREEZE 67; B.1 addition 1993; scope stop 697, unchanged); GONE: old R-PLAIN text, old "Queued after A3, plan first", old NIGHT_SHIFT.md:27-31. All new wrapped lines <=120 chars (long single lines :3 and :96 are the file's style).
- Final edit locations (after edits): STATUS :3 header; :25-38 Phase In flight + Queue; :96 In flight row (State + Waiting on); :157-161 WT-NIGHT ref; :394-414 FU-P4a/b/c; :418-419 A4 line; :436-439 A4-FREEZE quote; :460-484 B.1 addition quote; :733-738 A3 open-bugs header + refs :741/:865/:939; :744 :1373-1396; :748 :953; :809-819 R-PLAIN; :824 :670-679. ROADMAP :73 queue order; :93 A3 entry. REPO_MAP: none.
DONE
