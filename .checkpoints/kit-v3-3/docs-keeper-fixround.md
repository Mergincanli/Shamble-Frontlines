Task: docs-keeper, A3 (kit-v3-3, PR #38) fix-round docs: STATUS.md + ROADMAP.md per advisor-fix-round.md.
Plan (blocks, in order):
B1 read advisor-fix-round.md, builder-fix1.md, builder-fix2.md (source read)
B2 read STATUS.md A3 sections (risks, SA-3 ~:340, follow-ups, decisions pending, in-flight row)
B3 edit STATUS risks entry (L1,L4,L9 widened; L12-L15 new; remove R-a..R-e-closed limits)
B4 edit STATUS SA-3 text fix
B5 edit STATUS follow-ups
B6 edit STATUS decisions pending (WT-NIGHT, PC-SETUP q4)
B7 edit STATUS in-flight A3 row
B8 ROADMAP A3 entry (if one exists / needs update); REPO_MAP only if a row untrue
Notes:
- B1 done: ruling (verdict PROCEED, R-a..R-e, L1/L4/L9 widened, L12-L15 new, removed: if/while cd, failed cd, profile CDPATH, env/nohup/timeout cd), fix1 (cd redesign; 620/620; 7 movers; 28/28 mutants), fix2 (SA-2 routine half, F-b, SA-5, SA-6, SA-8; 716/716 = 562+127+27; mover T9 only; 33/33 scratch mutants, gate PASS). Next B2.
- finding: mustafa-words.md (commit 40c3f2a, 2026-11-05 ~10:30 UTC) records Mustafa's answers: WT-NIGHT (a) "holds only when the branch's rules, CI and gate files match main's"; PC-SETUP 1 Auto, 2 no mods, 3 Enterprise (university), 4 Windows itself (PowerShell), not WSL. So the decisions are answered: write the ruling's wording AND mark answered with his words (else STATUS untrue); flag to caller.
- B2 done: STATUS in-flight :92, phase :3/:25-26, Decisions pending :141-150, Follow-ups :299-345 (SA-3 text :338-341), A3 risks :635-683 (L1 :659, L4 :666, L9 :682, L11 :685). The four removed limits are NOT in STATUS (grep CDPATH/if-while/failed cd/nohup: only :92's step-4c history): nothing to remove. ROADMAP A3 entry :93. REPO_MAP rows 107-109 still true. Next B3.
- B3 done: risks entry: header cites the ruling; L1 (kept settings facts, :49-50 corrected to :50-51 [verified], :243-246 [verified]), L4, L9 widened; L12-L15 added after L11; a 'closed by R-a..R-e' note (the four were never separate lines). Next B4 SA-3.
- B4 done: SA-3 text at Follow-ups (:338-341 area), ruling's words; settings :49-50 -> :50-51 [verified]; dropped critic's routine-guard line refs (would shift with the fix round). B5 follow-ups list inserted before 'kit-v3-1 Windows test'. Next: check 'L1-L4, if E11' exists, then B6.
- finding: 'L1-L4, if E11 or use shows a need' follow-up is only in plan-rev2.md:633-634, not in STATUS; wrote it out in the L1 runners line. B5 done. Next B6 decisions pending.
- B6 done: Decisions pending A3 entry rewritten: ruling's WT-NIGHT and PC-SETUP (4) wording + his answers word for word (mustafa-words.md). Next B7 in-flight row.
- B7 done: in-flight row :92 state (+ reviews, ruling, fix round not yet committed) and waiting cell (fix commit + re-reviews; WT-NIGHT answered (a), step 9). Next: :3, :26, :635 state lines, then B8 ROADMAP.
- :3, :25-26, :635 state lines updated (answered (a); fix round not yet committed). Next: drift check of risks entry's routine-guard line refs.
- finding: risks entry's routine-guard.mjs [verified] line refs are at 442fbf7; working tree moved them (actions_get :921 -> :1048). Added a header note: re-verify after the fix commit. Next B8 ROADMAP.
- B8 done: ROADMAP :93 title state + reviews/fix round/waits. REPO_MAP unchanged (rows true). Next: wrap check, line list, repo-map-check.
- Final: wraps <=122 on all new non-table lines; repo-map-check OK (113 entries, 100 paths). Edits (new numbering): STATUS :3, :25-27, :92, :141-160 (Decisions pending), :349-352 (SA-3 text), :357-368 (follow-ups), :658-660 (entry header), :674-681 (limits header + L1), :687-688 (L4), :702-703 (L9), :707-717 (L12-L15 + closed note); ROADMAP :93. Two in-flight phrase fixes were made with a python exact replace (not Edit).
DONE
