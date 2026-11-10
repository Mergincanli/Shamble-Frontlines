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
- F7 MISSED: STATUS.md:1009 deferred low: "fix the wording when gate.mjs is next touched" - gate.mjs:333 and tools/README.md:35 (STATUS says :34) call gate-globals.json "reviewed data, not a gate tool", contradicting kit-v2-1 category 3. 2.1 touches gate.mjs and tools/README.md: fix or re-defer explicitly.
- F8 2.2 note: rule globs `**/index.html`, `**/editor.html` (pages.md:3-4) and engine-*.md `**/shamble-*.js` match legacy/ now, but `**/index.html` will also match a v2 index.html (root or packages/*) in 2.2 and load the legacy "edit both pages identically" rule on it. Narrow to `legacy/**` (kit edit; could ride with 2.1's kit edits). Same ambiguity for CLAUDE.md:46 basename once v2 has an index.html.
- check: no backslash/case variants; no markdown/HTML relative links to the five files; pages have no nav/href to siblings.
- check: settings deny has no git mv; `git checkout`, `git restore`, `rm -rf` denied (inv-code upheld).
- B1 DONE. next B2 spot-checks.
- B2 spot-checks (all read at the cited lines):
  inv-code: village.py:10 upheld; gate.mjs:92-100,:150-152,:190 upheld; smoke.mjs:118,:147-149,:201 upheld; index.html:1255 / editor.html:1256 CDN upheld; shamble-game.js:3189 location.search upheld; settings.json:74 ln rule upheld; tools/README :22,:24-28,:56,:58 '.' commands upheld; KVStore keys shamble-game.js:1954,:2335 shamble-editor.js:444,:697,:706 upheld; editor.html:1248 #edJson readonly, shamble-editor.js:1100,:1104 upheld; routine-guard.mjs:971-978 upheld.
  inv-kit: verifier.md:28 upheld; CLAUDE.md:19,:41-42,:46 upheld; advisor.md:313 upheld; NIGHT_SHIFT.md:27-29,:60-61 upheld; advisor.md:150-152 WRONG reading (F5); design-4 battery.js WRONG (F6); advisor.md:54,76-77 cites off (actual :53 and :73-75), substance upheld.
  inv-docs: START_HERE:21,:104,:114-116 upheld; STATUS:987 upheld; REPO_MAP:14-18,:49 upheld; README:12-22 upheld; ADR quotes 0001:1067,0002:951,0004:840,0006:806 upheld; PM rows 63/75/32, 0 legacy-path hits upheld; r5:1000,:1234,:1245 upheld; CS-1 saves claim WRONG (F4); TECHNICAL:204 is a doc claim, never tested.
- B2 DONE. next B3 design check.
- B3 design check:
  - inv-code 2(b): link `ln -s ../../../../tools/node_modules` at .gate/fdn-oldgate/legacy/tools/ resolves right, but does not match the allow rule (settings.json:74, `../../../` and `.gate/*/tools/`): it prompts. Writing main's gate into legacy/tools/ of a worktree is not a kit path (kitPath judges rel to worktree top: legacy/tools/gate.mjs not in GATE_FILES, routine-guard.mjs:26-27,:96-104): no ask. Logic of (b) holds: main's gate takes ROOT=legacy/ (gate.mjs:32-33), `git show sha:f` paths are tree-root paths.
  - inv-code 1: per-file base fallback holds; F1 (revert) is the gap. gate.yml push/pull_request base reasoning upheld (gate.yml:42-50).
  - inv-code 4 / done-when: F3.
  - inv-kit design 4: battery.js (F6); "move alone is tier 1 (:67)" conflicts with literal :63-64, already raised as Q-BRACKET.
  - inv-docs saves: F4.
- B3 DONE. next B4 answer.
- Corrections after review: grep -n puts advisor.md --no-renames at :54 and :76-77: inv-kit's cites UPHELD (drop my "off" note). Revert "CI green on its head" is advisor.md:289 (F1 cite fixed). F5 moved to upheld-with-note + design issue.
- Extra sweep: .claude/agent-memory not tracked (git ls-files empty). Layout prose with no file name ("repo root", "at the root", "five files") in kit + current docs: only CLAUDE.md:19, START_HERE:21 (both already rows), "run from the repo root" lines (tools/README:4,:52; START_HERE:112; TECHNICAL:227) stay true under (a). Nothing new.
- F2 conditional: ADR 0002 names no package.json "type" (grep empty): label as 2.2 risk only if root package.json is type module.
DONE
