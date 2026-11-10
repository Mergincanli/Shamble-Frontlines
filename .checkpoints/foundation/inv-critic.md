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
- F3 MISSED (inv-code design 4, inv-docs): PM-0002-36's own words (adr/0002:1288, done-when :1429-1430, r5:1245) say `node tools/preview.js legacy eval tools/tuning-dump.js` output is cmp'd with docs/legacy-tuning.json. As written it cannot pass: the JSON is the builder's output (facts.md), and without --pre=tools/tuning-dump.js the eval only snapshots and returns undefined (tuning-dump.js:3-5,:50). inv-code's dump+builder route is an equivalent, not the mark: the plan must say so and get it accepted (advisor; ADR wording is tier 1, Mustafa).
- F4 WRONG (inv-docs saves finding): "CS-1's earlier move checked that rank, tutorial and maps were still there" - repo-root.md:55 lists saves surviving the move as UNTESTED, :58 is only the proposed Brave step; no recorded result (grep STATUS/reports).
- F5 WRONG-ish (inv-kit indirect advisor.md:150-152): advisor.md:149-150 already runs the PR head's own gate in a worktree; "use main's copy" (:152) is for reproduction tools (preview.js, sim.js). Real gap: for a gate-changing PR the head's gate is not independent - fix is the differential evidence, not a reading of :152.
- F6 WRONG minor (inv-kit design 4): lists battery.js among changed gate files; inv-code: battery.js no edit (battery.js:64-68,:210-213). Tier 2 stands on gate.mjs, smoke.mjs, CLAUDE.md, verifier.md.
- check: TESTDATA command strings `preview.js . eval` at routine-guard.test.mjs:207 and settings.test.mjs:41 (allow-rule tests): no change needed, settings rules `node tools/preview.js *` still match `legacy`.
- B1 sweep mostly done; next: guard.mjs, checkpoint.mjs, then B2.
