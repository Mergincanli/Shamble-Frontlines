Task: A3 (kit-v3-3) reader-secrets: Mustafa's item 2 (sk- key after %20/%2C/%3A) plus the advisor's deferred F-1 (trust rule vs lost container output).

## Plan (blocks, in order)
B1. Read checkpoint.mjs SECRETS and where it is applied (save, watch).
B2. Read checkpoint.test.mjs secret tests.
B3. Reproduce the gap: scratch harness over every SECRETS pattern after %20 %2C %3A %22 %26 %2F %0A %3D.
B4. Design the fix; test it with the harness (positives and negatives).
B5. False-positive scan of new patterns over every tracked file.
B6. Exact test cases to add.
B7. Read CHECKPOINTS.md §4 trust rule (lines ~76-79, 110-111, 118-119); write F-1 wording option and fit advice.
B8. Answer, then DONE.

## Notes
- B1 DONE. checkpoint.mjs:27-38 SECRETS (10 patterns, no g flag so .test is stateless). Applied only in stagedSecrets()
  checkpoint.mjs:102-113: for every added/modified (diff-filter=d) note under .checkpoints/ in the private index, the
  WHOLE staged blob (git show :f) is tested with each pattern (line 109) and with exact ENV_SECRETS values (line 110).
  save() calls it at line 154; watch() reaches it via save(..., 'autosave', true) at lines 196 (last) and 204. So one
  scan point covers save and watch; no decoding of any kind happens before the scan.
  Finding F-a: line 30 API key boundary is (?:^|[^A-Za-z0-9-]|%3[Dd]) : only %3D is special-cased. After %20/%2C/%3A etc.
  the char before "sk-" is a hex digit/letter (0, C, A), which the class excludes, so no match (verify in B3).
  Finding F-b: line 34 AWS uses \b..\b; "%2CAKIA" has C before A, both word chars, no \b, so no match (verify in B3).
  Finding F-c: line 36 Slack uses \bxox; same gap ("%2Cxoxb-" -> C then x, no \b).
  Other patterns (GitHub, sk-ant-, JWT, npm, Google, private key, URL creds) have no left boundary: substring match,
  so a %XX prefix cannot hide them (verify in B3).
- B2 DONE. checkpoint.test.mjs: secret tests at 181-198 (raw ghp_, URL creds; secret never printed), 282-294 (env value),
  296-313 (%3Dghp_, JWT, npm, Google), 315+ (flagged note left out), 329+ (renamed note), 360-374 (%3D JWT, %3Dsk-proj-,
  and the negative: "cs/task-legacy-inventory-and-tuning-tables-for-v2-research" must save, exit 0). No test for
  %20/%2C/%3A, for AWS, or for Slack at all. Next: B3.
- B3 DONE. Harness .gate/kit-v3-3/repro.mjs (evals the real SECRETS literal from checkpoint.mjs; synthetic tokens built at
  run time, prints only names and prefixes). Command: `node .gate/kit-v3-3/repro.mjs .claude/scripts/checkpoint.mjs`.
  Result (miss = no pattern of that kind fires, tested both mid-text and at start of text):
  API key (sk- and sk-proj-): missed after %20 %2C %2c %3A %3a %22 %26 %2F %0A %0a %09 %253A %25252C \n \t \x2C "
    (literal backslash escapes, as in JSON); caught after "" space , = %3D %3d &#44; +.
  AWS key (AKIA and ASIA): missed after all of the above AND %3D %3d (no %3D special case on this pattern).
  Slack token: same misses as AWS, %3D included.
  GitHub, Anthropic, JWT, npm, Google, private key, URL creds: no miss (no left boundary: substring match).
  Finding F-d: a whole URL percent-encoded inside a query value ("?next=" + encodeURIComponent of a URL that carries a user and a password)
  is NOT caught by 'credentials in a URL' (prints false): the ":" "//" "@" are %3A %2F %40. Different gap, same family.
  Next: B4.
- B4 DONE. Harness .gate/kit-v3-3/fix.mjs (`node .gate/kit-v3-3/fix.mjs .claude/scripts/checkpoint.mjs`).
  Fix (A), in the patterns: one shared fragment before SECRETS (TDZ: must be declared above it):
    const ESC = String.raw`%(?:25)*[0-9A-Fa-f]{2}|\\(?:[nrt]|x[0-9A-Fa-f]{2}|u[0-9A-Fa-f]{4})`;
    ['API key', new RegExp(String.raw`(?:^|[^A-Za-z0-9-]|${ESC})sk-(proj-|svcacct-)?[A-Za-z0-9_-]{32,}`)],
    ['AWS key', new RegExp(String.raw`(?:^|[^A-Za-z0-9_]|${ESC})(AKIA|ASIA)[0-9A-Z]{16}\b`)],
    ['Slack token', new RegExp(String.raw`(?:^|[^A-Za-z0-9_]|${ESC})xox[abprs]-[A-Za-z0-9-]{10,}`)],
  (%3[Dd] special case is subsumed by ESC; [^A-Za-z0-9_] is exactly what the old left \b allowed before A/x.)
  (%25)* covers double/triple encoding (%253A, %25252C); \n \t \r \xHH \uHHHH cover JSON/JS-escaped text.
  Fix (B), optional, at the scan site (checkpoint.mjs:109): also test pctDecode(blob.out), where
    pctDecode = s => up to 3 rounds of s.replace(/%([0-9A-Fa-f]{2})/g, (_, h) => String.fromCharCode(parseInt(h, 16))).
  (B) is what closes F-d (encoded URL creds: A clean, A+B fires). A alone does not.
  Positives: every token (sk-, sk-proj-, sk-svcacct-, AKIA, ASIA, xoxb-, xoxp-) after "" space , = " %20 %2C %2c %3A %3a
  %22 %26 %2F %0A %0a %3D %3d %09 %253A %25252C \n \t \r \x2C " &#44; + now fires. The only "misses" printed are
  harness artifacts, expected and unchanged from today: "xsk-..." / "xAKIA..." (letter before), "-sk-..." (hyphen is
  excluded on purpose by the old class too).
  Negatives, all clean under A and A+B: cs/task-legacy-inventory-and-tuning-tables-for-v2-research and with %2C before,
  %20risk-/%3Arisk-/risk-/\ndisk-/%2Cdesk- + 40 chars, %2C+sk-+31 chars (too short), %2C+AKIA+15 and +17, XAKIA+16,
  myAKIA+16, %2Cxoxb-+9, fooxoxb-+20, %2Cxoxz-+20, an encoded plain URL (https%3A%2F%2Fexample.invalid%2Fpath%3Fa%3D1),
  "100%25 done, %2Fnot-a-key", %ZZsk-+40 (not hex).
  Next: B5 false-positive scan over git ls-files.
- B5 DONE. Harness .gate/kit-v3-3/fpscan.mjs (`node .gate/kit-v3-3/fpscan.mjs .claude/scripts/checkpoint.mjs <worktree>`):
  runs OLD, A, and A over the percent-decoded text (B) over every file `git ls-files` lists (146 tracked, 146 scanned
  as text, 0 binary, 0 skipped; working-tree contents). Result: A and A+B add ZERO hits over OLD. The only hits, the same
  under all three, are 'credentials in a URL' at .claude/scripts/checkpoint.test.mjs:187 and :320: the deliberate fake
  fixtures of the URL-credentials tests (a proxy URL with a fake user and password; a postgres URL with the password
  "changeme" at localhost). The guard never scans them anyway: it scans only staged notes under .checkpoints/
  (checkpoint.mjs:104, the `-- DIR` pathspec).
  Finding F-e: the first scan also hit this notes file (my F-d note quoted a URL with user and password). The saver
  would have refused the note. Reworded it; the rescan is clean. Lesson for the test list: notes must describe such
  shapes, never write them.
  Next: B6 exact test cases.
- B6 DONE. Verified with .gate/kit-v3-3/cases.mjs (`node .gate/kit-v3-3/cases.mjs .claude/scripts/checkpoint.mjs`):
  of the 17 "refused" strings below, 16 PASS SAVE TODAY (OLD clean) and are refused under A; k%3Dsk- is already refused
  (kept as a regression guard). All 10 "clean" strings are clean under OLD, A and A+B. The B-only string is clean under
  OLD and A, refused under A+B. Tests are end to end through `save` (SECRETS is not exported); watch reaches the same
  scan via save() (B1), so no separate watch test is needed. Tokens are built by concatenation, so the test source
  never holds a whole token shape (push protection is on).
  New test, after the "URL-encoded JWTs and sk- keys" test (checkpoint.test.mjs ~360-374), same style:
    test('keys after a %XX or backslash escape are refused (sk-, AWS, Slack); risk- and other words are not', (t) => {
      const { repo } = fixture(t);
      note(repo);
      assert.equal(run(repo, 'save', 'test', '-m', 'clean').status, 0);
      const head = sh(repo, 'rev-parse', 'HEAD');
      const sk = 'sk-' + 'K'.repeat(40), aws = 'AKIA' + 'Q'.repeat(16), slack = 'xoxb-' + 'S'.repeat(20);
      const refused = [`q=%20${sk}`, `a%2C${sk}`, `a%2c${sk}`, `k%3A${sk}`, `v=%22${sk}`, `a%26${sk}`, `p%2F${sk}`,
        `l%0A${sk}`, `k%3D${sk}`, `k%253A${sk}`, `"line\\n${sk}"`, `"\\u0022${sk}`, `a%2C${aws}`, `k%3D${aws}`,
        `l%0A${'ASIA' + 'Q'.repeat(16)}`, `x%20${slack}`, `k%3D${slack}`];
      for (const [i, text] of refused.entries()) {
        note(repo, `e${i}.md`, text + '\n');
        assert.equal(run(repo, 'save', 'test', '-m', 'escaped').status, 1, text.slice(0, 8));
        assert.equal(sh(repo, 'rev-parse', 'HEAD'), head);
        rmSync(join(repo, '.checkpoints', `e${i}.md`));
      }
      const clean = ['risk-' + 'r'.repeat(40), '%20risk-' + 'r'.repeat(40),
        '%2Ctask-legacy-inventory-and-tuning-tables-for-v2-research', '\\ndisk-' + 'd'.repeat(40),
        '%2Csk-' + 'K'.repeat(31), '%2CAKIA' + 'Q'.repeat(15), 'XAKIA' + 'Q'.repeat(16), 'fooxoxb-' + 'S'.repeat(20),
        'https%3A%2F%2Fexample.invalid%2Fpath%3Fa%3D1', '100%25 done'];
      for (const [i, text] of clean.entries()) note(repo, `w${i}.md`, text + '\n');
      assert.equal(run(repo, 'save', 'test', '-m', 'words').status, 0);
    });
  (assert message uses text.slice(0, 8): never more than the prefix, as the existing tests do with 12-16.)
  If (B) is adopted, add to `refused`:
    `next=${encodeURIComponent('https://' + 'user' + ':' + 'p'.repeat(8) + '@example.invalid')}`
  and rename the test "...; an encoded URL with a password is refused".
  Base-vs-head proof for the evidence: run the suite on main (the new test fails: 16 saves exit 0) and on the branch.
  Next: B7 F-1.
- B7 DONE. The trust rule is in three places only, all in .claude/CHECKPOINTS.md: :76-79 (§4 "a restart continues"),
  :110-111 ("Look first"), :118-119 (the paste block) (`git grep -n 'may still read a file it needs'`, .checkpoints/,
  STATUS and reports aside; `git grep -i recheck` finds no other restatement of the rule).
  Finding F-f (F-1 is real, and this run shows it): B3-B6's harnesses live in .gate/kit-v3-3/ (gitignored: .gitignore:2),
  B6's cases.mjs imports B4's fix.mjs. A lost container keeps these notes (the saver pushes them) but loses fix.mjs; the
  notes say B4 DONE, the rule says never redo it, and B6 could not run. Nothing today tells B6 to redo B4.
  Wording option (smallest, reactive: it adds no check of done blocks, which was E9-1's failure):
    :76-79 "...(a later block may still read a file it needs; only if that file is then missing, for example a build
            under `.gate/` lost with its container, the later block notes it and redoes the done block that made it,
            written over)."
    :110-111 "(a later block may still read a file it needs; only if that file is then missing, note it and redo the
            block that made it)."
    :118-119 paste block: "Trust the blocks they mark done: never reread, recheck or redo them (a later block may still
            read a file it needs; only if that file is then missing, note it and redo the block that made it)."
  Optional second line, not needed for F-1: "A done line names any file the block left outside the notes folder", so a
  restart knows which block made it (these notes already do this).
  Fit: put it in A3 if A3 runs a small E9-style dry run of the changed paste block (3 restarts after a cut: one with the
  .gate output present, which must not recheck; one with it deleted, which must note it and redo only that block; one
  DONE case). The paste-block text was validated by E9 (25 runs, STATUS.md:89), so changing it without a rerun would
  drop evidence. If A3 stays a guard-code pass with no agent dry run, defer F-1 to the next kit pass and record it in
  STATUS's follow-ups (Mustafa's 22:45 addition 2). Advice: A3, with the 3-run check; it is one file and about 15 words.
  Next: B8 answer.
- B8 DONE.
DONE
