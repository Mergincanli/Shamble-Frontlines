# Task: advisor-cs0-final (change-set 0 final diff review, PR #56, branch claude/wonderful-hamilton-jdmlnf, base main c94c81b)

Reviewed head (git rev-parse HEAD at start): 47546e949730bcf4c75d3c24a5f33d5f4b0eb1d5

## Plan (blocks, in order)
- B1: read my earlier reviews (advisor-plan-r2.md: conditions 17-23, ROOT ruling) and plan.md points 23/24 + files table.
- B2: read Mustafa's saved messages (2020, 2058, 2133, part-15 2306) for his exact words.
- B3: diff overview: log, stat, numstat; name-only and raw (modes) for the tier.
- B4: conditions 18-23 and ROOT checked in the committed diff, each with its place.
- B5: point 2 (a) PM-0005-56/55 vs ROOT; (b) "When a mark fails"; (c) engine-architect deferred Low.
- B6: point 3, his words: DECISIONS five entries, ten pointers, drafts + register answers, report "Since the plan", STATUS.
- B7: point 4, size count (size-count.mjs, numstat 4f20643), copied-lines sentence, A7.
- B8: point 5, docs pass (STATUS 120 lines, #56 row, reports index).
- B9: point 6, merge needs: tier, who merges, go, CI checks on head (gate, smoke, toolchain), part-15 deletion.
- B10: point 7, anything else (gate run in worktree if feasible, .checkpoints in diff, links).
- B11: answer written into notes; DONE.

## Notes (findings and block status)
### B1 done: my r2 conditions (advisor-plan-r2.md l.108-113) + plan P23/P24 (plan.md l.252-318)
- 18 (N-i) pointers at DECISIONS 2026-11-10 + 2026-11-13 entries (want-count 6). 19 (N-j) every "marks stay" pointer carries COMPARE exception (PM-0004-34) + "Those tests are not built as separate tests unless the plan shows a question the real game cannot answer." 20 (N-n) mark 8 counts what PM-0005-36 counts (draws/pass, shown-hidden); unit_dark = 4 materials not 4 draws; ways out to Mustafa with cs M plan. 21 (N-o) dated "Answered by Mustafa (2026-11-17)" under OQ-0006-18 (ADR 0006) + OQ-0002-6 (ADR 0002). 22 stop told + answered (SIZE-0 (a)). 23 Lows: no pose number in a rule; joint table one owner (soldier-joint-list.md); no "under root"; research file limit 400; since-the-plan size/times/both files; go header names GATE-FOLLOW, MACHINES, COMPARE; Queued pointer not prescribing; SLICE-ORDER pointer quoted; verifier spot-checks both files; LOOK_V2 hand-over waits Part 15 answer; cs M loosening -> his word; knee rule rigid weights; coat re-rig order; "(the exported GLB only)" = plan Part 12 not his answer.
- ROOT asked: skin.joints exactly 20; hips only joint w/o joint parent; mesh node + non-joint nodes above hips identity; under client root, both backends, scale s -> s x height (0.5, 1.2; 1 mm x s), mesh + joints together; else names stand, marks 3 + 10 l.5 + recipe change (exporter setting first, else one identity weightless top node, engine-architect + advisor at M); loosening a PM-0005-47+ mark -> Mustafa.
- P23: 5 commits order; split rule his words; LOG-HISTORY (a); A10 Art lead alone 3,244 B; DECISIONS 5 entries. Files table final l.293-313 (part-15* deleted in commit 5).
- Next: B2 Mustafa's messages.
### B2 done: his messages read (go 19:10:44Z = "20:10"; 2020 = 19:59:06Z; 2058 = 20:03:57Z queued; 2133 = 20:39:36Z queued "remove"; 2306 = 22:10:34.830Z uuid c4edbe94)
- 2133: SIZE-0 (a) one PR; stop 1,200 written, copied don't count; records + plan carry in own commits; split rule (round 1 Critical/High on rules/joint list not closed by one fix pass -> cs0 + cs0b, no new word, three lines); budget unchanged; no game code before cs0 on main. LOG-HISTORY (a) "Leave the history; nothing is rebuilt. The lines hold work points only." A10 = 3,244 B Art lead alone. SMG drawing at cs8; brief numbers stand.
- 2306: Part 15 six answers (LANE-SPLIT (a) incl. "one pull request at a time per lane"; RECORD-FILES (a) each PR writes only its own lines in its docs pass after taking main in; VFX-ROUTE (a) copy pinned + seventh package; VFX-FIX (a); VFX-MOMENTS (a), hit-stop none; GUN-FILES (a) no script-written gun file enters the game, code-built guns of cs6 stand in). STOP LINE: "Lines that a finding of the verifier or of the advisor agent asks for do not count toward the 1,200. New scope still does." PR #56: "My go on the merge comes in this chat, after the Advisor's check."
- go: CHANGE 1 names three items for DECISIONS (11-14 block, addition 15:03, "this message"). COMPARE = one exception to 11-14 marks rule. MACHINES names desktop. GATE-FOLLOW sets aside 11-12 for asked edits. STAND-IN-LOOK (a) Blender model. "hang the soldier on one root so it scales with one number". "In every report, write about your own process only what a record shows, with its time."
- Note: GUN-FILES (a) (22:10 UTC) affects cs0 docs? AMK9 script-written "used only to try the checks" -> check whether asset-export-rules/ADR 0005 say AMK9 can be "the first file"/"first real export" (plan l.71-72 said it can be the first file). Check B5/B10.
- Next: B3 diff overview.
### B3 done: diff overview
- HEAD moves by saver commits ([skip ci], .checkpoints only): 47546e9 (my start) -> 7f3483a -> 96a891c; docs identical to a65d26f (git diff 47546e9 96a891c = .checkpoints only). origin/main = c94c81b = merge-base (no main movement).
- 17 files, all docs/, all mode 100644 (no symlink/submodule); docs numstat +2994 -66 (DECISIONS 240/6, appendix 454, plan report 1423, cs0 report 72, asset-export-rules 397, soldier-joint-list 224, ADR 0005 74/31, SPIKE_PLAN 46/3, ADR 0002 17, register 12/12, ROADMAP 13/4, STATUS 8/8, REPO_MAP 5, research README 2, LOOK_V2 3/1, ADR 0006 3, ADR 0004 1/1). docs/reports/README.md untouched.
- Commit order matches SIZE-0: records 3747bb3 (20:59:11Z), 4a1b0d7; carry 212f55b; rules 5f356ef, 65c8fe8, cacea55; fixes f61f43c, 94aaae8, 4f20643, d6b6b30, a65d26f (22:16:17Z).
- CI: saver commits carry [skip ci]; CI last ran on 4857f19 (per 96a891c msg: gate, toolchain green). NOTE for B9: final head must NOT carry [skip ci] (required checks stay pending on a skipped head -> no merge), and gate+smoke must be green on that exact SHA.
- Report (cs0) read: l.9 Outcome pending; l.35 verifier second look + CI pending; l.38-39 size 1,200 counted + 6 not; l.47 advisor pending; l.63 Risk 3 cs1 waits for Part 15 on main; l.69 Decisions needed Nothing. Check: GUN-FILES (a) vs rules' "first file = AMK9" wording (B5).
- Next: B4 conditions 18-23 + ROOT in the diff.
