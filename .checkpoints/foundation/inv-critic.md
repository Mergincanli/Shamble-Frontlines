Task: foundation 2.1 completeness critic (find missed/wrong legacy-path refs; spot-check >=12 rows; check design answers)
Plan (blocks):
B1 sweep: relative paths, runtime-built names, readdir/glob over root, gate check-1 lists, .gitattributes/.gitignore, CI, settings, NIGHT_SHIFT/advisor commands, hooks (syntax-gate)
B2 spot-checks (>=12, >=3/agent)
B3 design answers check
B4 write answer
Notes:
- F1 MISSED: Rollback of 2.1 breaks CI. A revert PR's head carries the pre-2.1 gate (baseFile tries f, shamble-frontlines-repo/f only, gate.mjs:92-95) while BASE=origin/main has legacy/ only -> die exit 2 (gate.yml:42-50). advisor.md Revert check needs "CI green on its head" (advisor.md:290-291). Option: split 2.1 (gate learns both layouts first, then the move) or record the expected red.
- F2 2.2 note: syntax-gate.mjs:3,21-23 runs `node --check` in place; module type comes from nearest package.json. A root package.json with "type":"module" (2.2) would check legacy/shamble-*.js as ESM. gate.mjs nodeCheck uses .cjs temp (gate.mjs:115) so gate is safe.
- check: gate check 1 has no glob: FILES list (gate.mjs:34) + inline scripts (:227-231). .gitattributes `* text=auto eol=lf` covers legacy/. .gitignore no root patterns hitting legacy/.
- check: settings.test.mjs:232 join(root,'shamble-game.js') -> upheld (realOf routine-guard.mjs:45-51).
- check: repo-map-check claims upheld (:22-35).
