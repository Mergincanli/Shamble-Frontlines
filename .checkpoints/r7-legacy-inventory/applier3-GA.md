# applier3-GA (R7 legacy harvest, fixes3 applier for area GA)
Findings: 2 (both GA15).
- verifier-game1#3 (GA15.where): accepted. grep -n 'rotation.x\*=0.9' shamble-game.js finds 2916 (objective-role bot heading to a tech box, 2915-2916) and 2928. fix_op applied as written.
- graphics-engineer#1 (GA15.v2_note): accepted. solveArmIK (shamble-game.js:210-223) sets Object3D quaternions on rigid parts; 0 SkinnedMesh hits in the engine files; GPU skinning does not solve IK. fix_op applied as written.
apply-ops.py GA: 14 ops applied to 8 fields of 3 rows, clean.
DONE
