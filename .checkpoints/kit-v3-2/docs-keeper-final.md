Task: docs-keeper final docs pass for A2 (kit-v3-2, PR #37) at head 0684f1b: STATUS, ROADMAP, REPO_MAP only if untrue.
Plan (blocks, in order):
B1. Read the report docs/reports/2026-11-04-kit-v3-2.md (Risks, state).
B2. Edit STATUS line 3 header.
B3. Edit STATUS Phase paragraph.
B4. Edit STATUS In-flight row kit-v3-2.
B5. Edit STATUS Open bugs and risks entry kit-v3-2.
B6. Edit ROADMAP kit-v3-2 entry.
B7. REPO_MAP: check CHECKPOINTS.md row against §4; fix only if untrue; run repo-map-check.
B8. Line-length check on the edited lines.
Notes:
- Finding: fact 4(1) says E9 ran "1 to 2 times per case on the final text"; the report's Risks 1 and its table (text E: 3 of 3 restarts, 1 of 1 DONE, 1 of 1 fresh) say 1 to 3. Use 1 to 3 (the evidence) and flag it.
- B1 done: report read (Risks 1-3, steps, For Mustafa: go to merge, no Brave check). Next B2.
- B2 done: STATUS:3 header, 'PR #37, draft' -> ready for review + report path + waits for advisor then Mustafa's go. Next B3.
- B3 done: STATUS:22-26 Phase paragraph, 'PR #37, draft; this pass' -> ready for review + report path + next (advisor final head, Mustafa's go); rewrapped to the Queue clause. Next B4.
- B3 rewrap: lines 26-31 reflowed at 120 (words unchanged); max 120. B3 done. Next B4 (In-flight row, now ~line 89).
- B4 done: STATUS:89 In-flight row: rule summary (trusts done blocks, answer from notes alone; §4, E9-1), branch col 'PR #37 (ready for review)' + report path, done col (facts 1 incl. review wave, E9, 65cd40e/49f9898, N2-1, report), next col (advisor final head; Mustafa's go, needs-mustafa, no Brave). Dropped stale 'build in 11 files' (report: 15 files vs main). Next B5.
- Finding: STATUS:278 ('in flight, PR #37, draft') and STATUS:377 ('kit-v3-2 (A2, PR #37, draft)') also say draft; not in the edit list; plan: change 'draft' -> 'ready for review' minimally in B5b and report it.
- B5 done: STATUS:512-519 risks entry: header 'ready for review' + report's Risks; old item 2 (not yet shown) replaced by items 2-4 (fact 4, with '1 to 3' per report); old items 1 and 3 kept as 1 and 5. Next B5b (lines 278, 377).
- B5b done: STATUS:278 and :377 'PR #37, draft' -> 'ready for review' (377-378 rewrapped). Checked STATUS:377's toy-box list: ROADMAP:10 still says 'bright toybox palette' (true); GAME_DESIGN:28 and :50 hold tone words (upbeat, joyful), not 'toy-box' (loose, pre-existing, not changed). Next B6 ROADMAP.
- B5b rewrap: STATUS:278-281 and :377-379 reflowed at 120 (words unchanged). B5b done. Next B6.
- B6 done: ROADMAP:92 '[..., PR #37, draft]' -> 'ready'; E9-1 clause after 'last complete block'; 'Report: docs/reports/2026-11-04-kit-v3-2.md.' at the end. Next B7 REPO_MAP.
- B7 done: REPO_MAP:74 (CHECKPOINTS row) checked against §4: simplified but not untrue (omits restart-trust; 'checked and restarted at every update' leaves out 'while agents run' and 'if stopped'); no edit. repo-map-check: 'repo map OK: 113 entries cover 100 required paths', exit 0. Next B8.
- B8 done: only over-120 added lines are STATUS:3 (unwrapped header), STATUS:89 (table row), ROADMAP:92 (unwrapped entry), all in the file's style. e9-dryrun.md diff is the main session's, not mine.
Final line numbers: STATUS:3; STATUS:22-31; STATUS:89; STATUS:278-281; STATUS:377-379; STATUS:512-519; ROADMAP:92.
DONE
