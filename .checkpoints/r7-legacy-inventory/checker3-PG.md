# checker3-PG (R7 checker for area PG, fixes3)
Ran apply-ops.py PG: 9 ops, 7 fields, 5 rows; fixes3 ops = 5, each matches its finding's fix_op.
- verifier-edpg#0 accepted, OK: shamble-game.js:1153 switchWeapon sets reloadBar opacity '0'; grep finds 7 writes, row now lists all 7. Only the intended text changed.
- verifier-edpg#1 accepted, PROBLEM: the confirmation line is docs/AUDIT_BUGS_PERF.md:81, not :80 (:80 is blank; sed -n 78,82p). Fix: replace "(AUDIT:80)." with "(AUDIT:81).".
- verifier-edpg#4 accepted, minor PROBLEM: START_HERE.md:146 has "step 3 armory + scoreboard" OK; STATUS.md "**Paused until the greybox loop" starts on :29, ui-step-3 is on :30 -> cite STATUS.md:29-30.
- verifier-edpg#5 accepted, OK: mockup :root :8-17 has neither token; .desk rule :25-32 holds the gradient; index.html:228 (--wr-faded), :234 (--wr-desk-bg).
- graphics-engineer#10 accepted, OK: new text leaves the 3D question to the look brief; STATUS.md:107 still cited; mark port to TypeScript, look_brief_pending true.
DONE
