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
