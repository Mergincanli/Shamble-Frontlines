Task: security-auditor tier-2 review of change-set status-diet, PR #49, head 37434d5, base 2d22b69.

Plan (blocks, in order):
B1 diff stat + confirm no guard/settings/hook/CI/gate change
B2 his words: git show 2d22b69:docs/STATUS.md 680-726 + kit-v3-5 report Appendix A
B3 CLAUDE.md diff vs his words; run e3.mjs; e5-rule-diff
B4 docs-keeper.md diff vs STATUS-DIET exception (freeze)
B5 ACCOUNTS / advisor-tool / Lanes security effect vs Standing permission + advisor.md step 6 + guard
B6 split: security follow-ups (PX-1, W1, W2, W3, Windows-name limits) findable in STATUS; run check.mjs --rev 8ead8f1
B7 merge path: routine-guard mergeAllowed decision-doc list vs DECISIONS/STATUS/ARCHIVE
B8 notes folder secret scan
B9 write findings + verdict

Notes:
- B1 done: 23 files; outside docs/CLAUDE.md/.checkpoints only .claude/agents/docs-keeper.md; no guard/settings/hook/CI/gate file changed (git diff --name-only 2d22b69 37434d5). Next B2.
- B2 done: his words read (2d22b69:docs/STATUS.md 674-726: KIT-FREEZE-2 + what he meant, STATUS-DIET, LANE-RULES + Models addition + ACCOUNTS). Next B3.
- B3 note: CLAUDE.md quotes read against 2d22b69 STATUS 680-726 by eye: Lanes, 5 ACCOUNTS bullets, Models line, addition, look line all match; paraphrase 'No other model, except Fable 5.2 ... personal account' within his scope. Candidate F: STATUS_ARCHIVE.md:802 holds an 'Open, medium' Mergincanli-comment risk; check it is still in STATUS (B6).
- B3 e3: read e3.mjs (read-only: git show + fs reads), ran 'node .checkpoints/status-diet/e3.mjs CLAUDE.md' -> E3 PASS, 9 atoms + six-bullet check. Limit: E3 proves containment only, not the paraphrase lines (checked by eye: OK).
- B3 done: e5-rule-diff read; 8 rows; one looser (row 4, his Models line). Next B4 docs-keeper.
- B4 note: docs-keeper item 1 old text listed 'open risks and the health table' as STATUS duties; new item 1 quotes only his six STATUS parts + archive 'closed risks'; no line says where OPEN risks / security follow-ups live. Weigh vs his #46 go ('stay in STATUS until the foundation's gate step').
- B4/B6 note: STATUS.md@37434d5:64-69 keeps the security follow-ups bullet (PX-1, W1, W2, W3, Windows-name limits) and :86-89 Open risks (kit-v2-2 finding 4 Medium open); text lives in archive 'live sections'. docs-keeper item 1 does not name the Follow-ups/Open risks sections, header STATUS:2-4 names only six parts -> Low drift risk.
