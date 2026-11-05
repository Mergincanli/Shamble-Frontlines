Task: kit-v3-3 builder, plan rev2 section 8 step 6: the secret check (4.3 SECRET-DECODE (A)), its tests (section 5) and E3.
Plan (blocks): B1 read reader-secrets.md + plan 4.3/5/8 + rulings; B2 read harnesses + checkpoint.mjs secret check; B3 baseline test counts; B4 edit checkpoint.mjs; B5 add tests to checkpoint.test.mjs; B6 run tests; B7 E3 fp scan; B8 mutation rows; B9 evidence copy + self-scan; DONE.
## Notes
- B1 DONE: reader-secrets.md read (its fix A with ESC, 17 refused + 10 clean cases, fpscan found 0 new hits on draft A);
  plan-rev2 4.3 ESC fragment is wider than reader's: adds %uXXXX, =XX (QP, decided by E3), \b \f \v \0. 5 also lists
  critic's misses (\b \f \v \0 %u002C %252C =2C) and "save works from a linked worktree". (B) not built (C3).
- B2 DONE: checkpoint.mjs:27-38 SECRETS; API key :30, AWS :34, Slack :36; scan in stagedSecrets :102-113; refusal
  log :156 names file and kind only ("NOT committed: <f> looks like a <kind>").
- B3 DONE: baseline before edits: combined 3 suites 526/526 pass; checkpoint.test.mjs alone 25/25 (so guard+settings 501).
  Next: B4 edit checkpoint.mjs.
- B4 DONE: checkpoint.mjs edited: :27-30 ESC (comment :27-29, const :30, plan 4.3 fragment exactly, =XX included),
  :34 API key, :38 AWS key, :40 Slack token now new RegExp(String.raw`(?:^|<old class>|${ESC})...`). Refusal text
  unchanged (:160 now; names file and kind only). Base copy of main's file: .gate/kit-v3-3/step6/base-checkpoint.mjs.
- E3 harness written: .checkpoints/kit-v3-3/e3-scan.mjs (loads ESC+SECRETS from each copy, scans git ls-files + extra
  paths, prints kind, file:line, length only). First E3 run (before test edits; 170 tracked + 6 untracked notes files,
  176 scanned, 0 binary): old 2 hits, no-=XX variant 2, head 2; the same 2 (checkpoint.test.mjs:187 and :320, the
  existing fake URL-credential fixtures). New hits: 0 (noqp) and 0 (head) -> =XX KEPT (C8). Final E3 rerun after B5.
  Next: B5 tests.
- B5 DONE: tests appended to .claude/scripts/checkpoint.test.mjs (existing cases unedited): :376-378 shared split keys
  SK/AWS/ASIA/SLACK; :380-417 test "keys after a %XX or backslash escape are refused (sk-, AWS, Slack); risk- and other
  words are not" (26 refused strings, each asserts exit 1, the line "NOT committed: <file> looks like a <kind>", no key
  body in the output, HEAD unchanged; 12 clean strings saved in one commit, exit 0, no "NOT committed"); :419-434 test
  "save works from a linked worktree" (save from `git worktree add` folder commits only the note and pushes cs/wt; the
  main folder's HEAD and branch untouched; a raw key note there is refused; leak uses "key "+SK so base catches it too).
- B6 DONE: checkpoint.test.mjs head 27/27; with CHECKPOINT_SCRIPT=base copy 26/27 (fails only the new secret test; the
  worktree test passes on base: existing behaviour, newly tested). All three suites on head: 528/528 (before 526/526).
  Next: B7 per-case counts + final E3.
- B7a DONE: per-case harness .checkpoints/kit-v3-3/step6-cases.mjs (`node step6-cases.mjs old=<base> noqp=<copy> head=<file>`):
  refused caught: old 1/26 (only k%3D+sk), no-=XX 24/26 (misses a=2C+sk, k=3D+AWS), head 26/26; clean strings with a
  hit: 0/12 under all three. So 25 of the 26 refused strings save today on main.
- B8 DONE: mutation rows .checkpoints/kit-v3-3/mut-step6.mjs (`node mut-step6.mjs <worktree>`), scratch copies in
  .gate/kit-v3-3/step6/mut-0..8.mjs, all parse (node --check): 9/9 killed (API/AWS/Slack without ESC, no =XX, no %25
  chain, no %uXXXX, no backslash escapes, no JSON \b \f, ESC too wide with any letter -> fails the clean "risk-" case).
  Control (unmutated copy, same runner): 1/1 pass. Failure messages show only an 8-char prefix.
  Next: B9 final E3 over the tree with the new tests and every file I wrote.
- Line fix for B5: shared keys :376-378 (comment :376), secret test :380-413, worktree test :415-433. checkpoint.test.mjs
  diff: 59 added, 0 removed (C6: no existing case edited). checkpoint.mjs diff: ESC block :27-30, :34, :38, :40.
- B9 DONE: final E3 (tree with the new tests; 170 tracked + 8 untracked notes files, 178 scanned, 0 binary): old 2,
  no-=XX 2, head 2, the same 2 pre-existing fixtures (checkpoint.test.mjs:187, :320); NEW hits under head: 0 -> =XX kept.
  Env-value check over the 6 files I wrote/edited: 0 hits. Nothing committed or pushed.
DONE
