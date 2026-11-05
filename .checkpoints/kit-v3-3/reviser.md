Task: A3 kit-v3-3 reviser: revise plan-rev1.md with critic.md findings (copy to plan-rev1-draft.md first, add section 11).
Plan (blocks):
B1 read mustafa-words.md + checkpoint kit-v3-3.md
B2 read critic.md, list findings
B3 read plan-rev1.md
B4 copy plan-rev1.md -> plan-rev1-draft.md
B5 verify disputed findings against worktree where needed
B6 edit plan-rev1.md, fold in findings
B7 add section 11
Notes:
- B1 done, B2 done (23 findings C-1..C-23). Next B3 read plan.
- B3 done (plan read, 396 lines). B4 done (copied to plan-rev1-draft.md). Next B5 verify C-1, C-2, C-3, C-23 etc.
- B5 checks: C-1 confirmed (protectedRel splits '.gate/x/...' p[0]='.gate' -> false, routine-guard.mjs:27-38; projectRel non-null inside project so plan's "projectRel first" never falls through). C-2 confirmed (main b6effde behind origin/main 101; ls-tree b6effde .claude/hooks = syntax-gate.mjs only). C-3 confirmed (settings.json:49-50 node .gate/*.mjs|*.js; CLAUDE.md:190 AST script). C-23 confirmed (gh api pulls: #38 open draft, head claude/sleepy-tesla-ofsxsr). C-11 lines confirmed (CHECKPOINTS.md:84-85, CLAUDE.md:141-143, :211-217, NIGHT_SHIFT.md:2). C-16 confirmed (GITHUB_READ incl search_ returns null before repo check, routine-guard.mjs:592,602; gh pr view any -R :586). C-17 confirmed (docs-pass-items.md:123-127 F-1, R-1, R-2).
- Verdicts: uphold all 23; C-3/C-4/C-8/C-9 into one decision AF-COVER; C-16 search_* as stated limit (gh -R scoped); C-22 limit + win32 case fold + P4; C-15 reason reworded.
- B5 done. Next B6 edits.
- B6 partial: header, §0, §1, §2, §3 (AF-AGENT, AF-COVER new, WT-DAY, WT-UPDATE, WT-NIGHT, READ-SCOPE) edited. Next: SECRET-DECODE, §4.1-4.8, §5-§9, §10, §11.
- B6 partial: SECRET-DECODE, 4.1, 4.3, 4.5, 4.6, 4.7 done. Next 4.8, §5-§10, §11.
- B6 partial: 4.8, §5 done. Next §6 E4/E9/E10/E11, §8, §9, §10, §11.
- B6 done (all sections edited). B7 done (§11 appended). Next: sanity check (line widths), then DONE.
- Sanity: plan-rev1.md 595 lines (draft 396); every line over 120 chars was already over in the draft (code literals,
  one STATUS line), except the ESC literal (151 -> 188, a regex literal, not wrappable). 343 diff lines.
- Summary for the answer: all 23 upheld; 2 partly not folded (C-15 return_content requirement; C-16 search_*), with reasons
  in §11. New decision AF-COVER (C-3, C-4, C-8, C-9, C-21). E10 moved to the plan stage, E11a pre-merge added.
DONE
