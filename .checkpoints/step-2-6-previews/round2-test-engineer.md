# Round 2, test-engineer (PR #72, fix pass de6e4b58 + 6fd0375d; on what changed only)

## Plan
1. Read round-1 notes, B13, `git diff 8fef0f6 6fd0375d -- hosting/`.
2. Re-run the suite; scratch copy .gate/step26-r2-test/.
3. Re-run my lint probe (27 variants) and new probes on the new lint; compare with the round-1 lint.
4. Re-run mutate_te.py; CLI-entry mutants.
5. Per-finding verdict; new findings.

## Block 1 (done before 23:49 UTC, `date -u` 23:49:32 at block 4)
- Working tree hosting/ and .github/ = 6fd0375d (git diff --quiet exit 0); preview.yml sha256 31119c06 (unchanged).
- `node --test hosting/preview.test.mjs`: 148 tests, 13 suites, 148 pass, 0 fail (~5.8 s; the two 20,000-file tests
  take 4-8 s each, no timing in any assertion; harness sleep is a no-op, test:80).
- mutate_te.py (copy = round-1 script, identical) on the scratch copy: 22 of 25 caught; survivors T11, T20, T23 (the
  three I judged equivalent in round 1). Reproduces B13.
- CLI mutants (te/cli_mut.py): entry guard off, exit code dropped, root one level wrong, out to stderr, argv index
  off: 5 of 5 caught. TE-D closed.

## Block 2: the lint (te/lintprobe.mjs, round-1 27 variants, + te/probe2.mjs)
- Real file: 0 problems; reader sees jobs [tests, check, deploy, after, reconcile]; positive control 26 (item-form
  `- run: |` without `${{`) clean; a comment line mentioning "secrets" not flagged. No false alarm.
- Now caught: 02 checkout ref (L5), 03 toJSON(secrets) job-wide, 04 secrets['X'] job-wide (L2), 05/06 token on a third
  step (L2 token-places), 09 --ignore-scripts=false, 10 npm rebuild own line, 11 npm --prefix . install (L3),
  12/13/14 quoted key, `run :`, flow-mapping step (L0), 22 4-space re-indent without secrets (token-places).
- Still missed (all outside G9's list, round-1 Lows TE-H/TE-I): 07 WRANGLER_LOG=debug, 08 metrics "true",
  15 `run: | # c` header then `# ${{ }}` line, 16 github-script `${{` in script:, 17 job permissions widened,
  23 download path into the workspace, 24 wrangler without working-directory.
- NEW, regression vs round-1 lint (round-1 lint extracted from 8fef0f6, same probe):
  `npm ci --ignore-scripts --no-audit --no-fund; npm install left-pad` and `... && npm exec wrangler -- --version` on
  the npm ci line: round-1 lint CAUGHT (L3 installer regex), new lint MISSES. Cause: test L3 now flags `npm` only when
  the line lacks `npm ci ... --ignore-scripts`; the separate `npm install|i|exec|x|add` check was dropped.
  `... && npm rebuild` on the same line: missed by both.
- NEW, gap in the token-places check: it keys on `secrets.CLOUDFLARE_API_TOKEN` only; L2 allows any secrets spelling in
  a step env, so `T: ${{ secrets['CLOUDFLARE_API_TOKEN'] }}` or `ALL: ${{ toJSON(secrets) }}` in a THIRD step's env
  (the npm ci step) passes. Low (unusual spelling).
- Also missed (Low, outside G9 bullets as written): `--ignore-scripts --ignore-scripts=false`; step env
  `npm_config_ignore_scripts: 'false'`.

## Block 3: extra probes (te/probe3.mjs), 23:49 UTC
- Token MOVED (not added) from deploy's verify step to deploy's npm ci step: MISSED. tokenAt joins job names only
  (test:859-860), so "deploy,deploy,reconcile" still holds. B13's "the token must sit in exactly deploy, deploy,
  reconcile" is true at job level only; "its three planned places" (steps) is not what is checked. -> N2 Medium.
- `run: | # c` header then a plain `echo "${{ }}"` line: CAUGHT (L1). Then a `# ${{ }}` shell-comment line: MISSED
  (plain `run: |` + `# ${{ }}` line: caught). TE-I residue is only that combination: Low.

## Block 4: verdict (final)
REVISE, narrow (no Critical/High). The fix is two lint lines + three planted variants; per the budget no third review
round: suite + te/probe2.mjs + te/probe3.mjs re-run suffices. If deferred: STATUS follow-up, and B13/the report must
not claim either property.
- TE-A PARTLY closed: toJSON/secrets[...] job-wide, =false, own-line rebuild/--prefix install caught; REGRESSION N1
  (test:865): `npm ci --ignore-scripts ...; npm install x` / `&& npm exec ...` caught by 8fef0f6's lint, missed now;
  `&& npm rebuild` on the same line missed by both. Severity Medium, not High: the real file is clean (current
  evidence holds), preview.yml runs from main only and every edit is a tier-2 reviewed change; but B13's
  "L3 allows npm only as `npm ci --ignore-scripts`" does not hold as written.
- TE-B closed (variant 02 caught, L5). TE-C closed (T1-T5, T12 caught). TE-D closed (T26 + 5 CLI mutants caught).
- TE-E moved (comment + runbook :82 worded as upper bound): CONDITION, the report must correct plan 2.3's "exact".
- TE-F moved: (a) token sitting condition (c) screenshots; (b) tag case a decision in the report: CONDITION.
- TE-G closed. TE-H partly (job-level token places); rest (WRANGLER_LOG, metrics, permissions, github-script, download
  path, working-directory: variants 07, 08, 16, 17, 23, 24) needs a STATUS follow-up: not recorded in B13 (N3 Low).
- TE-I closed except the `| # c` + `# ${{` combination (Low, fold into N3). TE-J closed (20/20). TE-K moved: CONDITION.
- N2 Medium (test:859-860), token-places at job level and `secrets.` spelling only (secrets['CLOUDFLARE_API_TOKEN'] or
  toJSON(secrets) in a third step env passes).
Not verified by me: scripts/mutate.py 32/32 (not re-run), CI on 6fd0375d, the report (not written yet).
Rule disclosure: several read-only git calls this round had `cd ... &&` in front or `; echo`/`> file` after.
