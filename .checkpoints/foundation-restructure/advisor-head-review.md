Task: advisor head review of PR #50 (foundation 2.1a) at 3c98ad5480326ae660f11847593cae34f3b80f21 vs main bb2d817.

## Plan (blocks)
B1. Reconstruct C1-C8 (plan diff rev2->rev3, advisor-plan-rev2.md).
B2. Diff file list at head; risk tier; kit/CI empty check.
B3. Check each condition at head.
B4. Rev 3 for 2.1a + marker departure (gate.mjs code read).
B5. Deviations: #50 replacement proofs, Never exception record, docs pass, E3 carry-over, docs-keeper effort.
B6. Review round: infra / security / test-engineer notes.
B7. Verdict, findings, For Mustafa.

## Notes
B1 DONE. Reconstruction: my rev2 notes (advisor-plan-rev2.md) hold findings F-A..F-O and rulings Q-A1..Q-A6 but no numbered C-list (the numbers were only in the returned answer, never saved). Rev3 diff (8bbebfa..b03f4d8, .checkpoints/foundation/plan.md) shows one edit per finding: F-A->FREEZE-FDN; F-B->MODEL-RULE dropped (LANE-RULES cited); F-C->"taken by the Advisor"; F-D->Part 4 rewrite + DECISIONS.md if STATUS-DIET first + step 0 re-read; F-E->1.7 models/effort; F-F->1.7 per PR + infra on 2.1a; F-G->1.8 format; F-H->E8 main session + ask; F-I->E4 A's copies; F-J->1.1 ff never force-push; F-K->smoke shared rule, labelled "the advisor's L6"; F-L->step 4 "its report says so"; F-M->step 4 SLICE-ORDER cond 3; F-N->SCOPE adds BRIEF:20-21,:144; F-O->"C7, every command under the guard of the day". Label evidence: L6 = F-K means the Lows F-F..F-M were L1..L8 (F-K is the 6th). The Highs+Mediums in order are F-A,F-B,F-C,F-D,F-E,F-N,F-O = 7 items, and F-O lands at 7th = C7 (matches the one surviving label). The 8th condition is not recoverable; likeliest is the verdict's scope line (merges + 2.1b kit sitting wait for FREEZE-FDN/SPLIT; 2.2 own plan review). Numbers C1-C6 and C8 are therefore inference, flagged as such; I check every finding's content instead of relying on numbers.
B2 DONE. Diff bb2d817...3c98ad5 (49 files, all mode 100644, no symlink/submodule): 38 under .checkpoints/foundation-restructure*, docs/{DECISIONS,ROADMAP,SPIKE_PLAN,STATUS,STATUS_ARCHIVE}.md, docs/adr/0002, docs/reports/2026-11-11-foundation-2-1a.md, tools/{README.md,gate.mjs,smoke.mjs,village.py}. .claude/CLAUDE.md/.github diff empty. Risk tier 2 (gate.mjs, smoke.mjs); decision docs (ADR 0002, SPIKE_PLAN, DECISIONS.md his words) -> needs-mustafa; no player-visible change.
- Note: report at head holds only Appendix A ("written at the change-set's last step"); the exception line in the report body is therefore for the final head (checkpoint step 6 has it).
