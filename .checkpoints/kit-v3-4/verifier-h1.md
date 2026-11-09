# Verifier re-check of H1's fix: kit-v3-4 head 3d39527 (2026-11-09)

Role read from origin/main (verifier.md, CLAUDE.md). Scope: advisor-h1.md §2. Not a review round.
Scratch: `.gate/verifier-h1/` (test repos there, each its own `git init`, outside the worktree's index).

## Plan
1. advisor.md:285-286 and :76-77 vs the ruling's §1/§3 texts (byte compare); word diff 198c075..3d39527.
2. pid tests with `--verbatim` on both sides plus the non-empty guard: whitespace fake, honest revert, revert after a
   later STATUS merge, empty pair.
3. kit-draft.md §8 rows 66, 68, 76, 80, 82 vs ruling §1, §3, §4, §6.
4. STATUS F5 under F4 vs ruling §5; the source line STATUS:560-561 (file exists at 20b4be5?).
5. Kit tests: 1034 / 127 / 27.
6. gate vs origin/main --no-smoke; repo-map check.
7. Scope of 20b4be5..3d39527 and origin/main...3d39527.
8. Added lines over 120 characters outside table rows.

## Results
1. **PASS.** advisor.md:285-286 and :76-77 at 3d39527 are byte-identical (`cmp`) to the ruling's §1 and §3 blocks;
   lengths 117/87 and 116/41. Word diff 198c075..3d39527 on advisor.md: only `path`→`path, CLAUDE.local.md`,
   `CLAUDE.local.md,`→`AGENTS.md,`, and `--stable` → `--verbatim` twice plus ", and neither is empty". Nothing else.
2. **PASS.** New script `.gate/verifier-h1/pid3.sh` (the rule as written: `--verbatim` both sides, pass only if equal
   and neither empty), run in `.gate/verifier-h1/t2/` (own `git init` repos; real `--no-ff` merges, `git revert -m 1`);
   git 2.43.0. Ids are first 12 hex.
   - 1a whitespace fake (`rm -rf` → `rm-rf`): inverse 9fd898640078, fake cd817a1e7ecf → FAIL (correct). Control: the
     old `--stable` gives 555b2f60ec9e for both (the H1 hole, now closed).
   - 1b honest revert: 9fd898640078 both → PASS. 1c fake with a trailing space on another line → FAIL.
   - 1d mistyped merge SHA (A empty) → FAIL; 1e mistyped head ref (B empty) → FAIL; 1f unknown option on both sides
     (as on a git without `--verbatim`): A and B both empty → FAIL, where bare equality would have said "equal".
   - 2a honest revert after a later two-line STATUS merge (offsets): bfb4ad1eacae both → PASS (plain diffs differ);
     2b the same plus a whitespace edit → FAIL.
   - The earlier `pid.sh` / `pid2.sh`, re-run in new folders `t3a`/`t3b`: same ids as the session's `session-h1.md`
     items 2-3 (3ddce8883e42 stable both tampers; a2e97545dfc8 / 67360025d966 verbatim; 8178535f5d4a / bfb4ad1eacae).
     The 50afd389efe7 in my fix-pass notes is pid.sh's `--stable` *inverse*, not a tamper id: my earlier "both" line
     misquoted it, as the advisor's §7 says. Corrected here.
3. **PASS.** kit-draft.md §8 at 3d39527 (lines 524, 526, 534, 538, 540), checked by script (whitespace-joined
   substring, verdict cells exact): row 66 After and verdict "stricter" = §3; row 68 Why carries §6's sentence, verdict
   "looser" = §6; row 76 After, verdict and Why are exactly §1's cells; row 80 After, verdict "same" and the added Why
   sentence = §4; row 82 After addition and "stricter" = §4. Both quoted After texts are present word for word in
   CHECKPOINTS.md:216-218 and :225-228.
4. **PASS.** STATUS.md:581-585 (F5) is byte-identical (`cmp`) to the ruling's §5 block and sits directly under F4
   (:577-580). The ungiven change at STATUS.md:560-561 ("and for F5 its ruling on H1, `.checkpoints/kit-v3-4/
   advisor-h1.md`, readable at 20b4be5"): true (`git cat-file -e 20b4be5:...advisor-h1.md` succeeds; the file is
   unchanged 20b4be5..3d39527; 20b4be5 is an ancestor of the head) and acceptable: it follows the line's own pattern
   for c3cf647, and without it F5 would cite a ruling that does not contain it. Note only: F4 and F5 both say "the
   verifier's L3"; they are different rounds' L3s (`verifier.md:126` and `verifier-fixpass.md:81`), told apart only
   by the source line. Not a finding.
5. **PASS.** `node --test`: routine-guard.test.mjs 1034/1034, settings.test.mjs 127/127, checkpoint.test.mjs 27/27,
   0 fail. `.claude/hooks` and `.claude/scripts` are unchanged vs origin/main (empty diff), so these are main's counts.
6. **PASS.** `node tools/gate.mjs origin/main --no-smoke` (base 626df67, head = working tree at 3d39527, clean but for
   this notes file): [1]-[4] and [5b]-[5d] PASS, symbols core 138 / game 714 / editor 353, +0 −0, 5a skipped by flag.
   `repo-map-check.mjs`: "repo map OK: 114 entries cover 101 required paths".
7. **PASS.** 20b4be5..3d39527 touches `.checkpoints/kit-v3-4.md`, `.checkpoints/kit-v3-4/kit-draft.md`,
   `.claude/agents/advisor.md`, `docs/STATUS.md` only. `git diff --name-only origin/main...` outside `.checkpoints/`:
   14 files at 3d39527, identical list to 198c075 (`diff` empty; both merge-bases 626df67).
8. **PASS.** Python character count: this push adds 10 lines to advisor.md and STATUS.md, none over 120. Whole branch
   vs main (259 added lines in the two files): only STATUS.md:3 ("_Last updated", the standing exemption). (awk's
   byte count flags two STATUS register lines with en dashes at 127/128 bytes; they are 120 characters or fewer.)

## Verdict: PASS
No finding (no Critical, High, Medium or Low). H1 is closed: the whitespace fake now fails, the honest revert and the
offset revert pass, and an empty pair fails. Correction to my own fix-pass notes recorded in item 2.
Not verified: GitHub-side state of PR #44 (CI runs on the pushed head); git versions other than 2.43.0 (a git without
`--verbatim` prints nothing, which item 1f shows the new "neither is empty" now refuses).
