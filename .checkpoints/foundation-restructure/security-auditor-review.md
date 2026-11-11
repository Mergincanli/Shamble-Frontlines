Task: security-auditor review of foundation step 2.1a, PR #50, pinned 3c98ad5480326ae660f11847593cae34f3b80f21 vs main bb2d817 (risk tier 2).

Plan (blocks, in order):
B1 kit diff empty check (.claude CLAUDE.md CLAUDE.local.md AGENTS.md .github)
B2 read tools/smoke.mjs diff + findBuild + containment check
B3 read tools/gate.mjs diff (BUILD, BASE_PREFIXES, basePrefix, guarded import)
B4 probe folder rule cases in scratch copies under .gate/foundation-restructure/ (stray legacy file, symlinks, case, partial, both)
B5 containment probes (legacy-x, .., %2e%2e, symlinks, file URL, case)
B6 his words: run docs-keeper-check-decisions.py; read report-appendix.py; judge #49 labelling
B7 process: exception line in checkpoint, merge commit d6a3fee
B8 write findings + coverage table; DONE

Notes:
- B1 done: `git diff bb2d817...3c98ad5 -- .claude CLAUDE.md CLAUDE.local.md AGENTS.md .github | wc -c` = 0. Kit untouched.
- Read tools diff (gate.mjs +48, smoke.mjs +26, village.py, README). Candidate concern: decoy copy (legacy/ with clean 3 engine files + html redirects the gate away from root pages that load renamed scripts). To verify by probe in B4.
- B2/B3 done (read): die() = exit 2 (gate.mjs:40); tryGit = execFileSync('git', [...]) no shell (gate.mjs:77-80); BASE_PREFIXES fixed literals ['legacy/','',OLD_PREFIX] (gate.mjs ~104), sha from rev-parse; smoke.mjs top-level imports node: builtins only (smoke.mjs:13-16), playwright required lazily inside runSmoke -> guarded import works under --omit=optional; import failure -> die (exit 2), fail closed, no root fallback. Plan.md:156-160 states the intent: root pages are not judged once legacy/ holds an engine file (for a future v2 index.html).
- Next B4: probes in worktree .gate/foundation-restructure/sa-probe.
