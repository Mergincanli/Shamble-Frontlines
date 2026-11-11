Task: test-engineer review of foundation step 2.1a, PR #50, pinned 3c98ad5480326ae660f11847593cae34f3b80f21 vs main bb2d817 (one round).

## Plan (blocks, in order)
B1 read the code diff dd12aeb -- tools/ (findBuild, BUILD/BASE_PREFIXES, village.py)
B2 read E3 (e3-summary.md, e3-mutations.json, e3-run.mjs, e3-results.json headline) and map mutations to new code paths
B3 carry-over: rev-parse gate files + game files + deps at 3c98ad5 vs dd12aeb
B4 rows-test.mjs: read + re-run once, judge assertions
B5 e3-bases.md: judge for 2.1b and its revert
B6 docs check: copy docs-keeper-check-decisions.py, mutate copies (byte, dropped line, rewrap, double block)
B7 gaps list (--json, village.py, guarded import failure, legacy-only read rule)
B8 write verdict + table; DONE

## Notes
B1 DONE (code diff read). Notes: findBuild picks legacy/ on ANY of 3 js there; both-places -> {error} -> die exit 2 (gate) / setup exit 2 (smoke CLI). BASE_PREFIXES ['legacy/','',OLD]; basePrefix = first prefix holding all five (cat-file -e). die = process.exit(2), so die inside the guarded import's try is not swallowed. Guarded catch path (import throws) untested by anything I've seen.
 - finding (Low): base with five files in BOTH legacy/ and root picks legacy/ silently, while a head in that state exits 2: asymmetric; and BASE_PREFIXES order is not discriminated by any recorded base (no base has two complete prefixes), so swapping the order would pass every test.
B2 DONE (E3 read). Design: plant in 3 cols, judge equal non-empty failing-id sets; controls C1-C3/PC1/PC2 exit 0 (PC1 judged on check 3). All 3 columns use base 44446a5 (root) -> E3 exercises head paths (BUILD, BUILD_PREFIX, classicParts resolve under legacy/, smoke root=BUILD, M5 + ../README.md) but never basePrefix's legacy/ or OLD branches; those are covered by e3-bases.md (real-git runs, 10 rows) not E3. Two-places stop: e3-bases + rows c/f. Good split.
 - note: E3 verdict for non-controls = "same across columns", not "expected check in failing set"; reportChecksInOld recorded separately (M19/M20 note explains the one grouping mismatch). Sound as an equivalence test (old gate is the oracle).
 - note: e3-run plant() writes mutations under c.build, so newleg mutates legacy/ files: the right files.
B3 DONE (carry-over). tools/ tree b4b5410 identical dd12aeb == pin; .github identical 44446a5==bb2d817==dd12aeb==pin; five game files identical at 44446a5, bb2d817, dd12aeb, pin; package-lock, gate-globals, battery identical. Base: default base for this head is merge-base bb2d817, whose five files == 44446a5's, so base side unchanged. Node: local v22.23.4; CI node-version: 22 (floating minor) -> 5b BUILTINS can drift with Node minor but that is pre-existing, not 2.1a's. E3 carries over: inputs are byte-identical; only the browser (1243 alias) and Node patch not pinned.
Next: B4 rows re-run.
