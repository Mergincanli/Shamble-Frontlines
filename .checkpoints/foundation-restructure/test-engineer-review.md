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
B4 DONE (rows re-run 2026-11-11 by test-engineer: `PLAYWRIGHT_BROWSERS_PATH=.gate/pw-browsers node .checkpoints/foundation-restructure/rows-test.mjs`, 1 m 40 s, 8 + 2 ALL PASS, no flake; output moved to .gate/fdn-rows-te-review). Scratch .gate/fdn-scratch: gate.mjs, smoke.mjs, battery.js, gate-globals.json and five game files cmp-equal to the pin.
 Assertions judged: exit code always; build (a '.', b/h 'legacy/'); stderr regex on c-g (anchored ^gate: for d/e/g). They test what they claim. Weak spots:
 - finding (Low): row h (stray root index.html + legacy/) runs --no-smoke, so it never proves the smoke ignores a root page; adding smoke there would discriminate (a smoke on the root would load index.html with missing scripts and fail).
 - finding (Low): smoke CLI c asserts exit 2 only, not /two places/ (exit 2 is also "Playwright missing"); it discriminates in practice because smoke CLI b passed in the same run, but the message is not asserted. Base prefix printed, never asserted (e3-bases records it).
 - note: rows are folders under .gate inside the repo; tryGit uses git -C ROOT so git discovers the parent repo for base 44446a5: valid.
Next: B5 bases.
B5 DONE (bases). e3-bases.md covers 2.1b's PR (edited gate, legacy/ head 9af08f7 vs root base 44446a5: PASS, prefix "", +0 -0) and its revert (edited gate, root head dd12aeb vs legacy/ base 9af08f7: PASS, prefix legacy/, +0 -0), plus CS-1 base and OLD's die on a legacy/ base (the reason for the split, shown). Enough for 2.1a.
 - finding (Low): the legacy/ commit 9af08f7 is a stand-in that died with the old container; 2.1b's real move commit is the proof that matters: 2.1b should re-run the two rows (its head vs main; main-at-2.1b as base vs a root head) on its own commit.
 - finding (Low): CI's smoke job runs `gate.mjs --smoke-only` (.github/workflows/gate.yml:92); no recorded run uses --smoke-only on a legacy/ layout (rows run the full gate). Same runSmoke({root: BUILD}) call, so risk is small; 2.1b's CI will be first.
 - note: CI's gate job passes origin/$BASE_REF or $BEFORE: both are commit bases read via basePrefix, so the table's rows are what CI meets.
Next: B6 docs check on copies.
B6 DONE (docs check bites). Harness .checkpoints/foundation-restructure/te-docs-check-mutate.py: a copy of docs-keeper-check-decisions.py (only its two open() paths pointed at copies in .gate/foundation-restructure/dc/, taken at the pin with git show; git reads of bb2d817 unchanged), run with python3 -I, one mutation at a time. Result 11 of 11 as expected: C0 control exit 0; M1 changed byte, M2 dropped line, M3 one line re-wrapped into two, M4 two lines joined, M5 block twice with a blank line between, M6 block twice adjacent, M7 trailing space, M8 byte changed in the appendix only, M9 byte changed in an old bb2d817 line: all exit 1 (M5 fires "each source block inserted exactly once" with A3.1 count 2, plus "unclassified" noise from difflib re-alignment); M10 same byte changed in both DECISIONS and the appendix: exit 0, as designed (the appendix is the reference; only A4 is anchored to git, A2.3 to a character count).
 Independent: the appendix's 8 non-A4 blocks equal the fenced blocks in mustafa-2026-11-10.md, -answers.md, -11-11.md byte for byte (my own reader). The notes themselves vs the chat: not checkable by me.
 - finding (Low): the check proves DECISIONS == appendix and appendix-A4 == bb2d817; it cannot catch an error made upstream in the notes or the appendix together (M10). Acceptable: the appendix is the stated reference and its writer checks it against the notes; worth one sentence in the report.
Next: B7 gaps.
B7 DONE (gaps probed on scratch copies, .checkpoints/foundation-restructure/te-gaps.sh, output under .gate/foundation-restructure/gaps/). Results:
 - G0 control (legacy/ layout, --no-smoke): exit 0.
 - G1 guarded import failure (smoke.mjs with a syntax error): exit 2, "gate: cannot load tools/smoke.mjs to find the build folder: Unexpected identifier 'is'". Path works; untested before this review.
   - finding (Low): consequence of the design: a broken smoke.mjs now stops the whole gate (exit 2) even with --no-smoke, so CI's gate job (checks 1-4, 5b-5d) depends on smoke.mjs loading; before 2.1a it failed 5a only. Fails closed; worth one line in tools/README.md or the report.
 - G2 <script src="../shamble-core-copy.js"> in legacy/index.html (inside the repo, outside legacy/): exit 1, check 2 "points outside legacy/" (+5b, 5d). The gate's static side of the legacy/-only rule, untested before this review.
 - G3 village.py: legacy/ good core + root decoy -> reads legacy/ (reaches the aabbs open); root decoy only -> IndexError (reads root); root good only -> reaches aabbs. Default path works both ways. Note: village.py has no two-places stop (prefers legacy/ silently): a tool, not the gate; Low at most, not raised.
 - --json: build '.'/'legacy/' asserted by rows a/b/h and recorded in all 31 E3 rows (new '.', newleg 'legacy/'); base.prefix '' recorded in E3, 'legacy/' and 'shamble-frontlines-repo/' in e3-bases.md. Text-mode base line with "files in legacy/" never shown in a recorded run (cosmetic).
 - legacy/-only read rule (smoke side): M5 (file:///) and ../README.md on newleg recorded; symlinks inside legacy/ pointing out are not caught (resolve, not realpath): pre-existing semantics, not 2.1a's.
Next: B8 verdict.
