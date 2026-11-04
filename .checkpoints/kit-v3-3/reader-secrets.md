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
