Task: docs-keeper A3 kit-v3-3 fix pass 2 doc update (STATUS.md, ROADMAP.md; REPO_MAP only if untrue). Never commit.
Plan (blocks):
B1 read advisor-fix-pass2.md (D-1, Stated limits, Follow-ups)
B2 read verifier-delta.md B8
B3 read builder-pass2-routine.md, builder-pass2-day.md, builder-step9.md
B4 locate stale lines in STATUS/ROADMAP at current head
B5 edit STATUS (header, in-flight, decisions pending, recent work A3 entry, limits, follow-ups, 715-717)
B6 edit ROADMAP:93
B7 re-verify line refs (routine-guard.mjs, settings.json, checkpoint.mjs, NIGHT_SHIFT.md)
B8 REPO_MAP check
Notes:
- B1 done: ruling PROCEED; L9/L11 widen texts, L16-L18 texts, A-4 follow-up text in advisor-fix-pass2.md Stated limits/Follow-ups sections. Next B2.
- B2 done: B8 list read (STATUS :3,:26-27,:92,:145,:152-153,:658; ROADMAP :93; line refs at 442fbf7). Next B3.
- B3 part: pass-2 counts: suites 864/864 = routine-guard 710 + settings 127 + checkpoint 27 (was 768 = 614/127/27); main's file 180/192 (3 pre-existing + 8 rev-parse scaffold + 1 SD-2 key scaffold); gate with smoke PASS (base 12d7961; core 138/game 714/editor 353 +0 -0; 51 guarded 0 dead; index 8/8 editor 5/5). Fixes: SD-1 (+commit/push alone, night only), V-3 exact show-ref, SD-2 -z --no-renames --no-relative (named C6 edit), SD-3, SD-4, SD-5 (--revision), SD-6 (worktree list), V-1 (function/trap/alias). Mutants all killed (PASS2 15, NIGHT_SWITCH 17, real-git routine 9, DAY +22, real-git day 4). No existing expected value moved. NIGHT_SHIFT.md:27-36 step 4.
- B3 done (step9: 614/614 +52 rows; ed3ea8c). Next B4 locate lines.
- finding: STATUS has no "Recent work" section; the A3 entry = Open bugs :658-728 (and ROADMAP Recent work :93). Treat "STATUS's Recent work A3 entry" as the Open bugs A3 entry; say so in answer.
- B4 done: stale lines at current head same numbers as verifier's (:3, :25-28, :92, :145, :152-153, :658-660; limits :674-717; follow-ups :357-368; ROADMAP :93). Next B7 line refs (before edits), then B5.
- B7 done (working tree, pass 2 uncommitted): routine-guard.mjs :525->:564 (KIT_AGENT), :600->:679 (shell-write ask), :655->:753 (dayEdit ask), :920-943->:1162-1185 (ACTIONS_READ..mcp dispatch), :259->:283, :282->:306, :286->:310, :669->:767; main's :594 GITHUB_READ unchanged (12d7961); checkpoint.mjs:29-40 -> :30-40 (ESC :30, Slack :40); main's checkpoint.mjs:30 ok; settings.json:243-246, :50-51, :272 ok; guard.mjs:46-48 ok; NIGHT_SHIFT.md:26-27 -> step 4 :25-36, switch text :27-30. Next B5 edits.
- B5 part: STATUS :3 edited (holds "fix pass 2 (commit pending)"); :25-27 Phase In flight edited (holds it); In flight row :92 edited (step 7 "(step 9 adds them)"; step 9, delta review, auto 4-5, ruling, fix pass 2 (commit pending) with counts 864/864; Waiting on replaced). Next: Decisions pending :145, :152-153.
- B5 part: Decisions pending :145 (NIGHT_SHIFT.md step 4, :25-36), :152-154 (answered; built step 9; tightened by fix pass 2 (commit pending); under review) done; Follow-ups A-4 added after fix-round block (~:369-371); A3 entry header :659-661 (holds "fix pass 2 (commit pending)") and refs :564 :679 :753 :1162-1185 :283 :306 :310 :767, checkpoint :30-40, main's :594; caveat dropped. Next: limits header, L9, L11, L16-18, closed line.
- B5 done: limits header names fix-pass-2 ruling; L9, L11 widened; L16-L18 added (ruling's words); Closed line rewritten (cd after failed cd -> fix pass 2 (commit pending), SD-4; other three 2acfbbc). L2 unchanged. Widths ok. Next B6 ROADMAP:93.
- B6 done: ROADMAP :93 title + body (step 9, delta review, fix pass 2 (commit pending), counts, Waits). 11 entries, no fold. Next B8 REPO_MAP.
- B8 done: REPO_MAP no untrue row (guard row incomplete re night switch, not untrue); repo-map-check OK 113/100. Next: list pending lines, DONE.
- Final: STATUS :27 rewrapped (was 127 wide). "fix pass 2 (commit pending)" lines: STATUS :3; :26-27; :92 (In flight row, "Fix pass 2 (commit pending; ..."); :153-154 (WT-NIGHT); :662-663 (A3 entry header); :733-734 (Closed line); ROADMAP :93 (title bracket and body, 2x).
- Edits: STATUS :3, :25-28, :92, :145, :152-154, :369-371 (A-4), :660-663, :665, :667, :672-673, :677-678, :680-683 (limits header), :714-715 (L9), :718-720 (L11), :725-734 (L16-L18, Closed); ROADMAP :93. REPO_MAP unchanged.
DONE
