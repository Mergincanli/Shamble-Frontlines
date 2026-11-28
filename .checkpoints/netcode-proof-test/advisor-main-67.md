# Advisor: MAIN-67 plan check (main 07d673b, #71 alone, into PR #40's branch)
2026-11-26, 00:0x UTC window. Advisor agent, day session, effort xhigh. Acting by main's advisor.md (read whole in this
run). Plan: main-67-plan.md (72 lines, sha256 48c7309c1d8ff124597946adfcbffa59d434b710c8d2e14f36c7320fc30ce7ad at the
start). Binding form: main-66-plan.md as approved, advisor-main-66.md (M66-1 to M66-5), and through them MAIN-65's,
MAIN-64's and MAIN-61's conditions.

## Plan of this check
1. Every fact by my own read-only commands: SHAs, merge base, the 8 paths and their counts, main's CI, the merge-tree
   conflict and the block's lines, the kit, the manifests, docs/adr/, the auto-merged docs, refs, the main-folder
   state.
2. #71's two non-doc files (toolchain.yml, playwright.config.ts): what they change for PR #40's CI.
3. The numbers against MAIN-66's list (nothing of MAIN-66's left in place).
4. Q1, Q2, the verdict, DONE.

## 1. Verdict: APPROVE WITH CONDITIONS M67-1 to M67-3 (all Low; paste-ready below)
The take-in is due:
- MAIN-52 calls for it: #71 changes CI (.github/workflows/toolchain.yml) and code outside spikes/
  (packages/e2e/playwright.config.ts).
- His word of 20:48 UTC (history.md:8048-8057, item 3) says #71 does not fit the lighter recipe: "Neither fits the
  lighter recipe".

Every fact and number in the plan holds when I re-run it myself (section 4). The plan keeps MAIN-66's steps 1-7 and
M66-1 to M66-5 whole, and leaves no MAIN-66 value in place.

The three conditions only spell out three lines the plan gives by pointer:
- the resume line;
- the anchor;
- step 7's baseline job.

Applied word for word, they need no new review, and step 7 stays. There is nothing Critical, High or Medium. Risk
tier 1 after the merge: no kit file differs from main's.

## 2. Conditions
- **M67-1 (Low): the resume line (C5 (a)), word for word, replacing the card's MAIN-66 resume text.**
  "MAIN-67 approved (advisor-main-67.md), running steps 1-7. If `git rev-parse -q --verify MERGE_HEAD` prints
  07d673b0ee509d36ce1b976ed8dfae002c7d7b4a, the merge is in progress: no save, no `git merge --abort`, no reset. If `git
  diff --name-only --diff-filter=U` names docs/STATUS.md, resume at step 3; otherwise resume at step 3's checks before
  the commit (C2 (b)), then step 4. If HEAD^2 is 07d673b and HEAD is not on the remote yet, resume at step 5.
  advisor-main-67.md and this card may show unstaged: leave them; step 6's card save takes them. U7's builder works in
  its own worktree: nothing of it shows here. Anything else: the advisor rules."
- **M67-2 (Low): the anchor (C2 (b)), as M66-1 (a) form.** At step 1, after H0 is fixed and after `git fetch origin`,
  run alone: `git merge-tree --write-tree --name-only H0 origin/main`.
  - The bar: exit 1, one CONFLICT line (docs/STATUS.md), and the auto-merge lines for DECISIONS.md, REPO_MAP.md,
    STATUS.md and STATUS_ARCHIVE.md. Anything else stops before step 2.
  - Its tree T goes into the plan's numbers by Edit before step 2.
  - Then, after `git add docs/STATUS.md`: `git diff --cached --quiet T -- . ':(exclude).checkpoints/'
    ':(exclude)docs/STATUS.md'` exits 0. After the commit, the same command with `HEAD` in place of `--cached`.
  - The other C2 (b) lines stand: `--diff-filter=U` prints nothing; the 7-file line;
    `git diff --cached --name-only H0 -- .checkpoints/ spikes/` prints nothing.
- **M67-3 (Low): step 7's toolchain baseline is main's run 45698565943, job 139502338223** (read now: push, main,
  07d673b, attempt 1, success).
  - Step 7 compares, line for line with that job: the `test result:` lines, the vitest `Test Files` lines, and every
    Playwright "passed" line (Chromium and Firefox).
  - It is not MAIN-66's job 139375347198: #71 adds the release tests and the Firefox step, so the old line list no
    longer applies.
  - C6's content bar after the add: the line out is main's #40 row (its 2026-11-14 text), and the line in is H0's #40
    row, byte for byte.

## 3. The plan's questions
- **Q1. Does #71 need more than MAIN-66's steps? No.**
  - The merge: the guard's merge rule reads only the branch's side (`origin/main...HEAD` over the kit pathspec: empty),
    so a plain `git merge origin/main` passes. MAIN-64 took the same file, toolchain.yml, by the same merge.
  - M66-2 still applies: step 2 writes a `.github/` file. Name it first; a refusal before git runs leaves nothing
    changed, and it goes to him as one named command.
  - No security read for the take-in, for MAIN-66's Q3 reason:
    - toolchain.yml comes in byte for byte (C1 (c)'s whole-tree check proves it);
    - #71 was reviewed and merged on main on his tap of 20:48:23 UTC (its merge message);
    - after the merge, PR #40's diff against main holds no `.github/` file.
  - Nothing of #71 reads spikes/:
    - toolchain.yml's new lines run the root workspace's release tests, the two generators in crates/devtools/vectors,
      devtools' wasm32 build and the Firefox project, which is packages/e2e/tests/rules only;
    - neither file names "spikes/" (`git grep`).
    - So check.mjs's refs step stays clean, and the light runs are unchanged but for refs (+1).
  - What changes is step 6's CI on #40. Its toolchain job now also runs the release tests and the Firefox step from
    the runner image's /usr/lib/firefox. A red there is main's or the runner's (MAIN-66's risk line stands): the lane
    takes no step on that head until the advisor rules.
  - Step 7's baseline: M67-3.
- **Q2. May step 7 ride U8's head read in the same window? Yes,** on MAIN-66's Q1 (d) terms with MAIN-67's values:
  - it reads H0, the merge and the pushed card save by SHA;
  - `git rev-list --parents <pushed>..HEAD`, with every later commit touching only spikes/, .checkpoints/ and
    docs/design/ (U8's merge is spikes/netcode-proof/ops/ and notes);
  - it re-runs the gate, the repo map, the checkpoint tests and the guard tests in the main folder when
    `git diff --quiet <merge> HEAD -- . ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/design/'` exits
    0, and otherwise in a worktree of the merge under .gate/;
  - it compares toolchain with M67-3's job;
  - three agents in all, U7's builder included.
  - If U8's head is not ready by that window, step 7 runs alone in it and never waits for U8.
  - U8's M3-M9 may start once MAIN-67's step 6 CI is green, on MAIN-66's Q1 terms (a), (b), (c), (e) and (f).
  - A step-7 finding above Low stops new lane steps until the advisor rules.

## 4. What I checked (read only; notes in section 6)
All of the plan's facts hold:
- origin/main 07d673b (= ls-remote), with one first-parent merge since c1da1bd: #71, committed 20:49:08 UTC on his tap
  of 20:48:23 UTC.
- The merge base with HEAD is c1da1bd.
- 8 paths (A 1, M 7), +555/-29; without STATUS.md "7 files changed, 543 insertions(+), 17 deletions(-)";
  STATUS.md 12/12.
- No path under spikes/, docs/adr/, docs/design/, .claude/, a CLAUDE.md or AGENTS.md, or a manifest or lockfile.
- Main's CI on 07d673b: gate run 45698566018 (gate 139502337281, smoke 139502337674) and toolchain 45698565943
  (139502338223), push, main, attempt 1, all success, 20:49:12-20:54:35 UTC.
- The trial merge (HEAD 4672461): tree e732d5a9, one conflict, docs/STATUS.md, with markers at :24, :28 and :31:
  - ours: :25 (#40, the 2026-11-15 text), :26 (#67), :27 (#69);
  - theirs: :29 (main's #40, the 2026-11-14 text), :30 (the ci-after-3a row).
  - Outside the block, the merged file equals main's.
  - Main's file has 120 lines and moved #67 and #69 to its archive (its "Last updated" line).
  - Since c1da1bd the branch has changed only the #40 row. Keeping :25 and :30 gives main's file with line 24
    swapped: 120 lines, "1 1".
- The trial tree equals main's outside spikes/, .checkpoints/ and docs/; docs/ differs in the usual six names; spikes/
  and docs/design/ equal HEAD's.
- HEAD..e732d5a9 without STATUS.md and .checkpoints/: the 7-file line.
- Trees: 1,930 files, then 1,931 (+1).
- The three auto-merged docs keep the branch's lines: numstat both ways (24/1, 17/0, 18/5), and HEAD..tree equals
  main's own (256/0, 5/4, 88/0).
- No untracked file in main's new paths.
- U7's builder worktree branch is at 9177fb5.
- The saver (pgrep) runs, so step 1 uses TaskStop and C5 (b).
- git status is clean.

## 5. Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-M67-1 | Low | The plan gives the resume line, the anchor and step 7's baseline job by pointer only. | M67-1, M67-2, M67-3 |
| ADV-M67-2 | Low (on record) | Main's own STATUS.md row for ci-after-3a still reads "none yet … risk tier 2 … the head for approval to be given", although #71 has merged. It is main's text, taken as it is. | Main's next docs pass |
| ADV-M60-7 | Low (open) | The #40 row says "main 53272a1 merged in" (the branch's own row). | PR #40's closing docs pass |
| ADV-M54-8 | Low (open) | CLAUDE.md's Never line lists no plain `git merge origin/main`. | The kit follow-up after the freeze |

Not checked: I ran no merge, light run or test for this plan. The 120 lines, "1 1" and the 8-file whole line come from
the trees.

## Decisions needed
None. MAIN-52 is his standing rule, and his 20:48 word sends #71 to the full plan.

## For Mustafa
Nothing.

## 6. Notes (one line after each check)
- Plan sha256 48c7309c…ce7ad (72 lines). HEAD 4672461 = ls-remote branch; origin/main = ls-remote main 07d673b.
- merge-base HEAD origin/main = c1da1bd. First-parent c1da1bd..07d673b: 07d673b only (parents c1da1bd, c9460ca),
  21:49:08+01:00. Its message: his tap 20:48:23 UTC, expected head c9460ca, main c1da1bd, checks green.
- name-status: M .github/workflows/toolchain.yml, M DECISIONS, M REPO_MAP, M ROADMAP, M STATUS, M STATUS_ARCHIVE,
  A docs/reports/2026-11-25-ci-after-3a.md, M packages/e2e/playwright.config.ts. numstat 18/2, 256/0, 5/4, 19/10,
  12/12, 88/0, 138/0, 19/1.
- toolchain.yml's diff: the header comment; `cargo test --release` over six crates; python3 -I -B the two generators'
  `--check`; `cargo build --locked --target wasm32-unknown-unknown -p devtools --lib`; a Firefox step (ldconfig print,
  /usr/lib/firefox plain-file check, `playwright test --project=firefox`). playwright.config.ts: a third project
  "firefox", testDir ./tests/rules, channel moz-firefox. Neither names spikes/.
- The guard pathspec: origin/main...HEAD empty; HEAD...origin/main = .github/workflows/toolchain.yml.
  c1da1bd..HEAD outside .checkpoints/ and spikes/: the six usual docs names only.
- merge-tree HEAD origin/main: tree e732d5a9, exit 1, CONFLICT docs/STATUS.md; auto-merge DECISIONS, REPO_MAP,
  STATUS, STATUS_ARCHIVE. Markers :24, :28, :31; the rows as section 4 says; `-U0 origin/main..e732d5a9` shows only
  the block. Main's STATUS.md 120 lines.
- CI by run id: 45698566018 (gate, push, main, 07d673b, attempt 1, success; jobs 139502337281, 139502337674) and
  45698565943 (toolchain, same; job 139502338223), 20:49:12-20:54:35 UTC.
- The whole tree exits 0; docs six names; spikes/ and docs/design/ unchanged (exit 0); the 7-file line; trees 1,930
  and 1,931; refs `-a -l` on main: only kept-logs.mjs and its test.
- U7's worktree: .git/worktrees/agent-a9873cdba07a3219e/HEAD -> refs/heads/worktree-agent-a9873cdba07a3219e =
  9177fb5. Saver: pgrep shows the watch (b2zyx9eey per the plan). git status: clean.
- history.md:8048-8057: his 20:48 word, item 3: "#71 (the workflow and docs) and later #70 … Neither fits the lighter
  recipe."

DONE
