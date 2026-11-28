# Advisor: MAIN-64 pushed-head check (pushed 807e92a; merge 72b0dc3; branch head now 204e6f4)
2026-11-24, about 13:2x UTC. Read only. Acting by main's advisor.md, read whole. Measured against main-64-plan.md,
advisor-main-64.md (M64-1 to M64-4), MAIN-61's C1-C6 and advisor-main-61-head.md's form. Kept short (NPT-TRIM).

## 1. Verdict: APPROVE (heads checked: pushed 807e92ad7a9de22a2a7d032b7339d4a4d5049fd0; now 204e6f476ef6…)
The take-in matches the plan, and M64-1 to M64-4 are met. Merge commit 72b0dc3 has parents dd6a03b (H0) and 1ec6f6c.
Outside spikes/, .checkpoints/ and docs/, its tree equals main's. spikes/ equals H0's. docs/ differs from main in the
branch's six docs only. STATUS.md has 120 lines and differs from main in the #40 row alone. CI is green on 807e92a:
6 of 6 check runs. Toolchain's test lines equal main's. **The lane may go on from this head.**
No new finding. Two Lows from earlier stay open, as ruled (section 2).

## 2. Findings (CLAUDE.md's scale)
- None new.
- Still open, as ruled:
  - ADV-M60-7 (Low): the #40 row says "main 53272a1 merged in" (807e92a:docs/STATUS.md:24). This is the branch's own
    row, kept word for word, which is right for a merge. It is fixed in PR #40's closing docs pass.
  - ADV-M54-8 (Low): the kit follow-up after the freeze.
- On record: the STATUS slip (history.md:7840-7842). The newline was lost and found by `wc -l` (119), then fixed by
  Edit before `git add`. The committed file shows no trace of it (section 3).

## 3. What I checked
- **The merge.** `git cat-file -p 72b0dc3`: parents dd6a03b and 1ec6f6c; git's own message with "# Conflicts:"; no
  attribution lines; an SSH signature is present. 807e92a's message cites MAIN-52 and ADV-M60H-5 and carries both
  trailers. After the merge, `git rev-list --parents 72b0dc3..HEAD` gives a straight line: 807e92a, a17efa4, 579f38f,
  204e6f4. None touches anything outside .checkpoints/. ls-remote: branch 579f38f at 13:2x (204e6f4 local, saver).
- **The whole tree.** `git diff --quiet origin/main 807e92a -- . ':(exclude)spikes/' ':(exclude).checkpoints/'
  ':(exclude)docs/'` exits 0. docs/ differs from main in six names: DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE and
  the two design docs. `git diff --quiet dd6a03b 72b0dc3 -- spikes/` exits 0.
- **H0 to the merge.** The 19 names of `git diff --name-only f5030a3 origin/main`, and "19 files changed, 2174
  insertions(+), 99 deletions(-)". STATUS.md is 38/38. H0 differs from the head I reviewed for the plan (1a89aca)
  in .checkpoints/ only.
- **72b0dc3 against merge-tree's tree f4dce49.** It is equal outside STATUS.md and .checkpoints/, so the auto-merged
  DECISIONS, REPO_MAP and STATUS_ARCHIVE are the ones I checked at the plan. DECISIONS.md's patch-id against main
  (f0822ab) equals the branch's own change.
- **STATUS.md at 807e92a.** It has 120 lines. `git diff origin/main 807e92a` changes one line, the #40 row (the
  2026-11-15 text). The #64 and "#63, #62" rows are main's, and #61 is gone. There is no conflict marker in docs,
  crates, packages, .github or .gitignore.
- **The kit.** The guard's full pathspec on `origin/main 807e92a` prints nothing. `git diff --raw --no-renames`
  shows only the two old 100755 files (u4d-fix1-run.sh, u3c-kernel-check/run.sh): no symlink, no submodule.
- **Step 1 and step 5 records** (.gate/main-64/; history.md:7821-7852):
  - light-base.log: HEAD dd6a03b; both hashes e3b0c442…; refs 1,811; the full marks line; self 78/78; redact 11/11
    with 359 items and 0 findings; exit 0.
  - light.log: HEAD 72b0dc3, the same hashes; refs 1,814 = N0 + 3; marks, self and redact word for word the base's;
    exit 0.
  - Both runs took the lock with "idle after 0 s".
  - step5-outputs.txt: GATE PASS (base 1ec6f6c, symbols +0 -0), repo map 173/110, checkpoint tests 33/33.
- **Reproduced myself in the main folder** (tree equal to 72b0dc3 outside .checkpoints/): `node tools/gate.mjs
  --no-smoke` GATE PASS; repo map "173 entries cover 110 required paths"; `checkpoint.test.mjs` 33 tests, 33 pass,
  0 fail. `git status --short` was clean.
- **CI on 807e92a** (gh api check-runs): 6 runs, all completed with success, 13:07:45-13:10:20 UTC.
  - gate 138466569052 and 138466595486, smoke 138466569735 and 138466595991 (runs 45400232353 push, 45400238799 PR);
  - toolchain 138466570653 (push, 45400232653) and 138466594409 (PR, 45400238548).
- **Toolchain against main's run 45373056953** (job 138371964348). From the job logs (data), with timings removed:
  - the 28 `test result:` lines, the 5 `Test Files` lines, the "1 passed" line and the Playwright "12 passed" line
    are equal in order and count;
  - the PR job has the same 34 summary lines.
- **M64-2:** the card is 197 lines. Its Base line (:5-7) names 72b0dc3, and "My step" is a single block (:156-).
- **M64-3:** the saver had ended at 12:31 ("time is up"); the card and history.md:7830-7831 give that proof.
- **PR #40:** head 204e6f4, draft, open, mergeable true, state "blocked", base 1ec6f6c. No conflict.

## 4. What I did not check
- **Steps recorded only in the chat:** the pre-checks' printed output, the merge's output and exit code, the Edits
  of STATUS.md, the background calls' timeouts, and whether any call asked. The committed results match each one.
- I ran no light run, check.mjs or cargo myself. spikes/ is unchanged since H0, and the logs are whole.
- I did not verify the commit signatures' keys, and read no CI artifacts.

DONE
