# Verifier r2: the advisor's revised docs-PR words (revision 2) and the revised reconnect record (2026-11-07)

Branch claude/netcode-proof-test-plan-hcky5g, HEAD 7c8bbe2 at start (docs/ and spikes/ as at 975e72b; the record draft
and the architect's notes are uncommitted edits in the working tree). Inputs: the advisor's revision 2 (relayed in the
task; saved by the session as advisor-ws-docs-r2.md), the record draft ws-reconnect-record-draft.md, verify-ws-docs.md
(V1-V31, must-correct 1-7), ADR 0001, docs/adr/README.md (branch and origin/main), plan.md, marks.json, npt-judge
(marks.rs), the committed outputs spikes/netcode-proof/results/u3-ws-fallback/. I edit no tracked file and commit
nothing; this file is my only write.

## Plan
1. Read Mustafa's 13:45 and 14:30 words in the checkpoint; the first verifier's must-correct list.
2. Old texts: every E0-E13 old_text against ADR 0001 (branch = main?) and main's README; exact or normalised counts.
3. Numbers: every figure in E-PR, E0, E1, E6, E13 and the rulings against tables-1e020a7.txt / bar-fix1-bf9d248.txt.
4. Tolerance statistics: is s = M_p/M_m measurable from one run with noise well under 10 %? Check G8's arithmetic
   (median SE at n=480, clustering, tick phase), G10 (5 %), G12 (budget x2 at 1.187), and the guard's sense.
5. Direction of moves: never looser than the model for a correct build; up to 10 % only; beyond: investigate and never
   loosen. Hunt for any path that loosens beyond 10 % or tightens/loosens otherwise (FAR BETTER, chance ruling, model
   input recompute, tolerance guard, rounding).
6. Additions or losses vs Mustafa's words (13:45, 14:30): anything added that is his decision; anything dropped.
7. marks.json schema and npt-judge's stats: can the checks (c)-(e) with f be expressed; validity rule; select keys.
8. Record draft vs E11 (1)-(11) and vs the ADR words (target wording, slice 3, cites).
9. Must-correct list 1-7 of verify-ws-docs.md: each met or not.
10. Verdicts, bugs, must-corrects. DONE.

## Findings (one line each, in order found)
- W1 (setup) my notes file did not exist at resume; started fresh. HEAD 7c8bbe2; the checkpoint's decisions at HEAD :349-356 (14:30) and :357-362 (13:45), same lines at 95a04a1 (git show, first 80 columns compared). bar-fix1-bf9d248.txt:11 holds "x1.10 0.000": a correct build 10 % slower fails the first bound; :10-15 every bar row as E-PR quotes.
- W2 (slip, recorded) I appended W1 once with a shell heredoc (`cat >>`) instead of the Write tool; same file, same content. From here on only the Write tool.
- W3 judge schema (spikes/netcode-proof/crates/judge/src/marks.rs at HEAD): checks carry `parts` (:116), parts carry `validity` added to the mark's (:59-68), so E-PR's per-part rules and per-check parts are expressible. A limit "X x f" with f read from another run's record is NOT expressible today: Threshold is a fixed Value or `against` = factor x the same statistic of role runs (:247-251; README.md:111-113). `against` cannot give X x max(1, M_p/M_m). `where` takes a fixed value (:71-75). So checks (c)-(e) need new judge code (the advisor calls it preferred, with a run-script fallback): plan work, not a contradiction. Rule::parse (:304-317) refuses "ws-practice" today (as V16).
- W4 README.md:76-78: a part's `select` has `role` default `measured`, so a role "practice" run never enters "L2 B1 ws"; README.md:152 step 1 keeps per mark only runs listing it, so the practice run must list PM-0001-18 (the advisor says so). Fine.
- W5 the results-ledger rule R9 cites exists: advisor-u1.md:129-132 (a run superseded only by a later build fixing its found cause).
- W6 BUG (spike-branch changes, the ws-practice rule vs the WebSocket bite): a bite run takes the validity rules of the parts it matches, role aside (marks/README.md:204-205; eval.rs:1279-1281, the advisor's B7 ruling), and an invalid bite run makes the whole mark INVALID (README.md §3 step 6, :160-161; eval.rs:1515). The ws-budget-x2 bite run (L2, B1, LG, params.transport ws) matches "L2 B1 ws" and so inherits "ws-practice", which needs a practice run "with the same build hash" whose outcome says IN TOLERANCE. The bite's build is the bites build (README.md:22), so it has none; and if it got its own practice run (R11: "lands beyond the tolerance in its own practice run"), that run is BEYOND (s about 1.19), so still not IN TOLERANCE. Either way the bite run is INVALID and PM-0001-18 comes out INVALID, never PASS. The rule must say how a bite run is held: against the measured build's practice run and its f (build equal but for the planted defect), with no practice run of its own; R11's sentence must go. R3's hours (0.15 + 0.15 + 0.1) also have no room for a bite practice run.
