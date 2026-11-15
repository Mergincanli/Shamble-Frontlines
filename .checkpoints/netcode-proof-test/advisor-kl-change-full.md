# Advisor: the review of KL-CHANGE, pack's second walk (workflow wf_d7fbca53-76d), 2026-11-15

Written by script, word for word, from the workflow's journal. Nothing below is the main session's. Its notes: advisor-kl-change.md.

- **Verdict:** APPROVE WITH CONDITIONS

## Summary

I reviewed KL-CHANGE against Mustafa's words of 16:35 (history.md:2131-2146, which match the relayed request word for word), using main's advisor.md (main at 4460ef5).

Inputs are as described:
- The before/ copies equal the three posted blocks byte for byte (kept-logs-final-text.md, sha256 97c8165655869d9a…).
- kl-change.diff (c67c5322…) has the same body as `git diff --no-index -U1 before after`. The one difference is how the inserted test lines up against the repeated `});` + blank line at its seam; both diffs describe the same change.
- Sizes after the change: §11 is 19 lines (was 18), the script 323 (was 313), the tests 323 (was 292). The widest lines are 118, 120 and 120 characters.

My own re-runs, on scratch copies in the system temp folder only (Node v22.23.4, GNU tar 1.35):
- Changed suite: 9 pass, 1 skipped (the dry run). The GNU tar test is "ok 2" and was not skipped.
- Posted suite: 8 pass, 1 skipped.
- M12 (second walk removed): only test 5 fails, actual 0, expected 1. The scratch folder holding the late file was removed and the script exited 0, which is the gap his change closes.
- M13 (second walk refuses): only test 5 fails, actual 2, expected 1, on the late link.
- M4, retargeted to `if (differs(now, rows))` → `if (false)`: only test 4 fails, actual 1, expected 2. With M4 planted, the second walk now keeps the folder too.
- The new test passed 3 of 3 repeat runs.
The TAP files agree with my runs.

The code:
- The second walk sits inside pack's try, after the read-back check and before the rename and the removal.
- A difference throws Mismatch into the same catch as a read-back mismatch: the folder and the .part stay, exit 1.
- An odd entry also exits 1, through walk()'s new `fail` parameter. It refuses by default, so `list` and the first walk are unchanged.
- `differs` is the old inline check, moved into a helper.
- No other behaviour changes. All three narrowings are intact.

The test:
- The child-only `--import` preload is sound and deterministic. It fires synchronously when pack opens the .part to read it back, so it also pins the walk's place.
- It is test-only: no seam in the script, constants only, in the temp folder. No security concern.
- The late-link case is the only case M13 fails on, so it belongs.

One Medium gap: no case changes a size after the read-back, and his words say "files or sizes". Fix: a third case (a file that grows) and planted bug M14, with exact text in the conditions.

One Low: two branches of the second walk are not exercised. Left as is; the folder stays on every branch.

I wrote nothing but my notes file (/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-kl-change.md). One slip: one command used `> /dev/stdout`, which wrote no file.

## Against his words

- **history.md:2136: yes to the kit lines as posted (kept-logs-final-text.md, sha256 97c81656), with one change** · **met:** yes · kl_change_apply.py:20 asserts the posted file's sha256 prefix 97c8165655869d9a. I recomputed it: 97c8165655869d9a7747…. A node byte compare shows before/checkpoints-11.txt, before/kept-logs.mjs and before/kept-logs.test.mjs equal the posted BEGIN/END blocks exactly. The diff changes only what the change needs.
- **history.md:2136: his yes confirms KIT-FREEZE-2 for this one rule** · **met:** yes · Recorded in the session checkpoint (netcode-proof-test.md:496, history.md:2131-2146). It needs no kit line; the PR report must quote history.md:2136 (build plan).
- **history.md:2137: in pack, after the read-back equals the list and before the folder is removed, walk the folder once more** · **met:** yes · after/kept-logs.mjs:256-263: the second walk follows the read-back compare (the `bad` Mismatch throw). It comes before renameSync(part, archive) at :264 and rmSync(dir) at :273, inside pack's try. The test's hook fires when the .part is opened for read-back. A walk placed earlier would miss the change and exit 0, so the test pins the place.
- **history.md:2137: if its files or sizes differ from the list, the folder stays and the script exits 1, as for a mismatch** · **met:** yes · differs() compares the count, each path and each size. A difference, or an odd entry through walk's new `fail` thrower, throws Mismatch into the same catch as a read-back mismatch: it prints "MISMATCH … the folder stays … tell Mustafa", leaves the folder and the .part, and exits 1. My re-run: test 5 passes (exit 1, folder kept, no finished archive, .part kept).
- **history.md:2137: add a test and a planted bug for it** · **met:** yes · Test 5 (a late file; a late link). M12 fails only test 5 (actual 0, expected 1). M13 fails only test 5 (actual 2, expected 1). Re-run by me with KEPT_LOGS_SCRIPT. Gap: his 'or sizes' half has no case and no planted bug (F1); condition C1 adds both.
- **history.md:2137: post the changed lines here and wait for my yes to them; then build** · **met:** yes · Nothing is built. git status shows only notes files changed, nothing under .claude/. This review comes before the post, per his 13:35 rule (history.md:2129). Conditions C2-C3 cover the post.
- **history.md:2138: keep all three narrowings** · **met:** yes · (1) Pack nothing more until he answers: §11 after lines 14-15, and the .part refusal at after:226. A second-walk mismatch keeps the .part, so the next pack refuses. (2) An unfinished archive is never deleted: no rmSync or unlink of a .part anywhere; the only removals are rmSync of the set folder at :273 and of the cache at :303. (3) The cache date bound: OLD_BEFORE at :44 and the newest() check at :299, untouched by the diff.
- **history.md:2139: in the PR's evidence, the GNU tar unpack test ran and was not skipped** · **met:** NOT YET · On the scratch copies it ran: after.tap and my re-run show 'ok 2 - GNU tar unpacks…' with no SKIP, and '# skipped 1' is the dry run alone. The PR evidence is not made yet; build_plan_changes item 4 makes it exact.
- **history.md:2139: in the PR's evidence, the dry run's result for the two real folders** · **met:** NOT YET · Not run yet; it belongs to the build. Both real folders exist: .gate/netcode-proof-test/runs/u4r-precheck-30-faf42aeb034c and u4r-P3-fd4612050cd6. build_plan_changes item 4 names what each must show.

## Findings

### KLC-F1 (Medium)

- **Where:** kl-change.diff, kept-logs.test.mjs new test 5 (scratch after/kept-logs.test.mjs:162-191)
- **What:** The new test changes the folder's file count (a late file) and adds an odd entry (a late link), but never a size. His words say 'files or sizes'. By reading: a planted bug that makes the second walk compare only the count (`walk(...).length !== rows.length`) passes all 9 tests. Case a changes the count, case b throws before any compare, and test 4 goes through the first walk only. Under that bug, a listed file that grows after the read-back is deleted with the folder while the archive holds the old bytes.
- **Fix:** Condition C1: a third case in test 5, a listed file grown after the read-back, which must exit 1 with the folder and the .part kept. Plus planted bug M14 (the second walk compares only the count), which must fail only test 5 (actual 0, expected 1). Exact text in C1.

### KLC-F2 (Low)

- **Where:** scratch after/kept-logs.mjs walk(): the PATH_OK branch's and the 'holds no files' branch's fail() on the second walk
- **What:** Only the not-a-plain-file branch of `fail` is exercised on the second walk. A planted bug turning one of the other two calls back to refuse would exit 2 instead of 1. The folder and the .part would still be kept, because refuse exits before any removal.
- **Fix:** No change: the safety property (the folder stays) holds on every branch. Named for test-engineer's read at build; a Low never triggers a push on its own.

## Conditions

1. C1 (KLC-F1, Medium), applied to the test text before posting. In kept-logs.test.mjs's new test, the name's two lines become exactly `test('a folder that changes after the read-back, before it would go: the second walk fails (1) on a late file, a ' +` and `  'late link and a grown file; the folder and the unfinished archive stay', (t) => {` (116 and 84 characters). After the line `  assert.ok(lstatSync(join(b.set, 'late-link')).isSymbolicLink(), 'the link stays, never followed');` add exactly three lines: `  const c = packWith(`fs.appendFileSync(${JSON.stringify(`${RUNS}/${SET}/marks.json`)}, 'late')`);` / `  assert.match(c.p.stderr, /MISMATCH[\s\S]*changed while it was packed[\s\S]*the folder stays/);` / `  assert.equal(readFileSync(join(c.set, 'marks.json'), 'utf8'), '{}\nlate', 'the grown file stays as it is');`. Planted bug M14, in the script, replaces exactly `    if (differs(walk(dir, set, (msg) => { throw new Mismatch(msg); }), rows)) {` with `    if (walk(dir, set, (msg) => { throw new Mismatch(msg); }).length !== rows.length) {` (it must match once).
2. C1's check, on scratch copies in the system temp folder only: the changed suite gives 9 pass and 1 skipped (the dry run); M14 fails only test 5 (actual 0, expected 1); M12 and M13 still fail only test 5 (actual 0 and 2); M4 still fails only test 4 (actual 1, expected 2). The tests file is 326 lines with the widest line at most 120 characters. Record the TAP outputs beside the existing ones.
3. C2: rebuild after/ and the diff by script from the posted blocks (sha256 97c81656…) with every replacement matching once. Check that the new diff differs from kl-change.diff (c67c5322…) only by C1's lines; record that check and the new diff's sha256 in the checkpoint. Post that diff whole. If anything else differs, it comes back to the advisor before it is posted (his 13:35 rule, history.md:2129).
4. C3: the post that carries the diff states the new numbers: §11 19 lines (was 18), the script 323 (was 313), the tests 326 (was 292); tests 9 plus the dry run (was 8); planted bugs M1-M14 (was 11). It also states the two limits named in to_mustafa: it does not catch a same-size rewrite, and a moment remains between the last walk and the removal. Then WAIT for his yes; nothing is built before it.
5. C4: the build follows build_plan_changes exactly, on top of my earlier C7-C9. Nothing real is packed or deleted before the PR is on main and this branch has taken main in.

## Build plan changes

These change the build plan (kept-logs-rule-draft.md (c) and my C7-C9) once he says yes to the changed lines.

1. Test count. A full run gives 10 TAP entries: 9 pass and 1 skipped, the dry run alone. The draft (c) step 1 said "8 pass, 1 skipped".
   - The tests renumber from 5 on:
     1. list then pack
     2. GNU tar
     3. not committed or not pushed
     4. changed after its list
     5. NEW: the second walk (late file, late link, grown file)
     6. refused before any work
     7. odd entries
     8. cache
     9. readBack
     10. the dry run

2. Planted bugs M1-M14, each made by one exact replacement that must match once in the changed script. Each must fail at least its named test; record every failing test name and the assert's actual and expected values.
   - test 4: M1, M2, M9, M10, and M4. M4's exact replacement is now `  if (differs(now, rows)) {` → `  if (false) {`; it fails the extra-file case, actual 1, expected 2, and with the second walk the folder now stays.
   - test 6: M3.
   - tests 7 and 8: M5.
   - test 3: M6.
   - test 8: M7 and M11.
   - test 9: M8.
   - test 5: M12 (the second walk removed; late file, actual 0), M13 (the second walk refuses; late link, actual 2) and M14 (the second walk compares only the count; grown file, actual 0).
   - M1-M11's exact replacement strings are checked to match once against the changed script, not the posted one.

3. The extraction (C7). The three kit files are rebuilt by script from kept-logs-final-text.md (sha256 97c81656…) plus the KL-CHANGE diff he says yes to (named by its sha256), applied as exact replacements. They are compared byte for byte (cmp) with the worktree's files. The sizes are §11 19 lines, kept-logs.mjs 323, kept-logs.test.mjs 326.

4. His evidence lines in the PR (history.md:2139):
   - (a) The full run's TAP line for test 2, verbatim: 'ok 2 - GNU tar unpacks the archive to the same bytes, and readBack reads an archive GNU tar wrote', with no '# SKIP'. Next to it: the run's '# skipped 1' (the dry run alone), plus `tar --version` (GNU tar) and `node --version`.
   - (b) For each real folder, u4r-precheck-30-faf42aeb034c and u4r-P3-fd4612050cd6:
     - the dry run's TAP line with no SKIP;
     - its two diagnostics: the file count, list ms and pack ms, and the script's stdout (free before and after, archive bytes from the total, 'read back: N of N files equal the list', removed);
     - the real folder's fingerprint (`find <real> -type f -printf '%P %s %T@\n' | sort | sha256sum`) before and after, equal.
   - (c) Test 5's TAP line.

5. The PR report:
   - It quotes history.md:2136-2139 by line and records KIT-FREEZE-2 as confirmed for this one rule.
   - C8's tier-2 table stays at three narrowings. The second walk is his requirement, not a narrowing.

6. Reviews (C9): unchanged, security-auditor and test-engineer in one round of one batch. test-engineer is asked about two things: test 5's child-only `--import` preload, and KLC-F2 (two untested branches of the second walk). The advisor reviews every pushed head. Then STOP for his go on the merge.

7. Nothing else changes. That covers the worktree from origin/main (never switching the main folder), the `.claude/` writes made in Accept edits mode after asking him, and the kit tests' counts, which must equal main's (routine-guard, settings, checkpoint). It also covers repo-map-check, the gate, and nothing real packed or deleted before the PR is on main and this branch has taken main in.

## To Mustafa

**What the change does.** pack reads the archive back, and every hash must equal the list. Then it lists the folder one more time. It stops if any file was added, removed or changed size since the start, or if anything became something other than a plain file (a link, for example). When it stops, the folder stays, the unfinished archive stays, and the script exits 1 with "MISMATCH … tell Mustafa", exactly as for a hash that differs. Nothing else in the script changes. All three narrowings are kept as you saw them.

**Why it matters.** Shown on scratch copies, not argued. Without the last walk, a file that appears while the folder is being packed is deleted with the folder, and it is not in the archive. The planted bug with the walk taken out (M12) shows exactly that: the folder with the late file was removed, and the script reported success. With your change, the same case stops with exit 1 and keeps everything.

**What it does not catch.** Your words ask for files and sizes, and it stops there:
- A file rewritten after the read-back to the same size with other bytes. Catching it would mean hashing every file a second time.
- A change in the brief moment between the last walk and the removal (renaming the archive, printing one line).
- An empty new folder: it holds no file, so it is not counted, and it goes with the folder.

§11's "never while a run set or a build runs" covers these.

**One addition from the advisor before you see it.** The test now also grows a listed file after the read-back, for your "or sizes". A third planted bug (M14) must make that case fail.

**New numbers:**
- §11: 19 lines (was 18).
- The script: 323 lines (was 313).
- The tests: 326 lines (was 292).
- Tests: 9 plus the dry run (was 8).
- Planted bugs: 14 (was 11).

The pull request's evidence will show, as you asked:
- the GNU tar test's own result line, proving it ran and was not skipped;
- the dry run's result for both real folders (u4r-precheck-30-faf42aeb034c and u4r-P3-fd4612050cd6), with each real folder's fingerprint equal before and after.

So far everything ran on scratch copies in a temp folder. Nothing real was packed or deleted, and nothing under .claude/ was written.

**Decisions needed:**
- **KL-CHANGE-YES: do the changed lines go in exactly as shown?**
  - Why it matters: you asked to see them and say yes before anything is built.
  - Options: (a) yes; (b) yes, with changes you name; (c) no.
  - Recommendation: (a).

**For Mustafa:**
1. Read the diff: the §11 lines, the script lines, and the new test with its three cases.
2. Say "yes" (or what to change) in this chat. Nothing is built before that. Nothing real is packed or deleted before the pull request is on main and this branch has taken main in.
