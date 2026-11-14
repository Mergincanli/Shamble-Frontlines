# advisor-pr45: day-session review of docs PR #45
Head e290c970e750afdec6ab894e62fd5f5a98f9d491, base main 69ea88e. Verdict only (called from a session).
## Plan
1. Read Mustafa's decisions (checkpoint "Decisions taken"), advisor-ws-docs-r2.md, verify-ws-docs-r2.md.
2. Diff 69ea88e..e290c97; map each hunk to an r2 edit (W-ids) as corrected.
3. ws-reconnect.md vs draft vs ADR 0001 vs spike README numbers.
4. README register counts, OQ-0001-19 row; REPO_MAP row + repo-map-check; STATUS line.
5. Gate --no-smoke in worktree; CI checks.
## Notes
- N1 edits-final.json (the session's apply input) = my advisor-ws-docs-r2.md E-texts word for word, except exactly the verifier's corrections: E-PR +W24 risk line; E1 W19/W17 ("no number is ever looser than the bound times the measured slowdown, itself at most 1.10"; looser recomputed bound = changed model -> Mustafa) + W20 two reading marks; E5/E7/E6 W21 "of the link coming back". (adv45/cmp_edits.py)
- N2 Rebuilt ADR 0001 and README from main 69ea88e + edits-final: equal to e290c97 word for word (whitespace aside). Every old text once (E4, E13 normalised). (adv45/check_apply.py)
- N3 Wrap: no new prose line over 120 in the ADR; two orphan words from keeping the following line untouched ("snapshots" after E0, "cover" after E13): cosmetic, renders the same. README line 27 grew to 166 chars (E9 list, not re-wrapped): cosmetic.
- N4 E1's W19 wording: "beyond 1.10 ... no number is ever looser than the bound times the measured slowdown, itself at most 1.10" can be read as allowing bound x 1.10 after a BEYOND run's investigation; Mustafa: "beyond 10 %, investigate first and never loosen". Weigh.
- N5 PR #45 via REST: head e290c97, base main 69ea88e, not draft, same repo, label needs-mustafa, mergeable clean. Body = my E-PR + W24 line + an evidence paragraph; "For Mustafa" 3 still says "merge #<n>" (placeholder not filled).
- N6 Record (docs/design/ws-reconnect.md) = architect's draft + only W12's VISION_V2 cites (:82, :55, :58 on main: checked). Numbers vs spike README 975e72b and tables :120-131, bar :7-14: all match (today 1,121/2,686/12,068; fix 1 654/2,164/11,384; target 654/1,051/1,859/2,092/4,196; bound 0.67 s / 5 over 2.5 s / 6.9 s, 0.968).
- N7 README: counts 19/13 and 119/98 right (19 OQ entries at head). Artefact: line 23 " Total: 119 ... open  (OQ-0004-5" has a leading space and a double space (E9 split on "||" kept the spaces); line 27 166 chars. Renders the same in markdown.
- N8 Gate in the worktree (node_modules linked, then removed): checks 1-4, 5b-5d PASS, +0 -0, 5a skipped. repo-map-check OK (114 entries, 101 paths) once the link was gone (with it: "missing from the map: tools/node_modules", an artefact of the link). CI: gate 128500636641 and smoke 128500636553 success on e290c97 (and a second pair, 128500298306/128500298553, success). PR touches no .github/ or gate file.
- N9 Checkpoint cites at 95a04a1 hold (13:45 :357-362, 14:30 :349-353, U3-Q4 :391-392/:400-401, P2 :417-418). Commits bf9d248/975e72b/95a04a1 live only on the spike branch; repo allows squash: PR #40 must merge with a merge commit or the record's cites go dark (Low, info).
- N10 Decision: REVISE on N4 (E1's beyond-1.10 clause reads as a 1.10 cap, looser than Mustafa's "beyond 10 %, ... never loosen" and than my R8 and the record's own section 6.4 "nothing is loosened"); plus N7's line 23 in the same push; PR body: mirror the fix, "merge #45".
DONE
- N11 Re-review of 6ae357111887849f30fc74a619b8bef7fb4f02f7 (one commit on e290c97): diff = my change 1 word for word (new phrase + untouched 13:45 quote found once; "itself at most 1.10" gone; row still one line, 8 pipes as on main) and README line 23 now equal to main's form with the new counts. Gate --no-smoke PASS in .gate/npt-readings at 6ae3571 (link made with an absolute path, removed after); repo-map OK 114/101. CI gate 128504751905 + 128504749520, smoke 128504751754 + 128504749341: all completed success on 6ae3571. PR body: "Beyond it" bullet matches, "merge #45", REVISE recorded. Verdict: APPROVE 6ae357111887849f30fc74a619b8bef7fb4f02f7, needs-mustafa (ADR edits): waits for his go.
