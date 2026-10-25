# checker3 GA (R7 legacy harvest, fixes3 checker for area GA)
- apply-ops.py GA: 14 ops, 8 fields, 3 rows, clean. fixes3-ops-GA.json = the two fix_ops verbatim (json equality True), so nothing else changed.
- verifier-game1#3: accepted, fixed correctly in GA15.where. grep -n 'rotation.x\*=0.9' shamble-game.js -> 2916 (inside en.role==='objective' && tbox.active branch at 2915) and 2928 (else branch). No problem.
- graphics-engineer#1: accepted, fixed correctly in GA15.v2_note. solveArmIK shamble-game.js:210-223 sets sh/el quaternions; SkinnedMesh 0 hits in core/game/editor. No problem.
DONE: no problems.
