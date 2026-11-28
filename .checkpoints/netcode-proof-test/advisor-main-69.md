# Advisor: MAIN-69 plan check (main b8f02b0, #78 ci-time, into PR #40's branch)
2026-11-27, day session. Advisor agent (the one that checked MAIN-68's rev 1, rev 3 and its pushed head), effort xhigh.
Read only: no cargo, no git write. I act by main's advisor.md (`git show origin/main:.claude/agents/advisor.md`, read
again whole in this run). The form: main-68-plan.md rev 4 as checked (advisor-main-68b.md C68B-1 to C68B-6, P-2),
and its pushed head (advisor-main-68-head.md).

**The tool's limit, applied here.** A command that exits non-zero without printing can show nothing, the same as a
pass. So every check below uses a form whose output carries the answer (`--name-only`, `--stat`, `--numstat`, sha256
lists, `ls -d`), and I quote that output. Block 0 re-reads, in such forms, the `git diff --quiet` results I relied on in
advisor-main-68b.md and advisor-main-68-head.md.

## Plan of this check
0. Re-read my earlier `--quiet` results in output-carrying forms (MAIN-68's whole tree, anchor and precondition).
1. His words: history.md:8163-8186 (the 13:55 UTC message), word for word: the window rule, its limit, TAKEIN-CODE (b)
   and its conditions.
2. The plan: main-69-plan.md whole, with its sha256, against rev 4 and C68B-1 to C68B-6 and P-2.
3. The facts by git (origin/main local, no fetch): first-parent b9aad01..b8f02b0; #78's paths and counts; the trial
   merge (clean?); the kit and CI pathspec both ways; refs; untracked files; main's CI on b8f02b0.
4. #78's toolchain.yml change, read whole, and what it changes for PR #40's CI and step 7's baseline.
5. Q1-Q5, the verdict, findings, Decisions needed, For Mustafa, Status line.

## Block 0. My earlier `--quiet` reads, re-read (HEAD 505378b, origin/main b8f02b0 = ls-remote)
- **The control.** `git diff --quiet b9aad01 origin/main` compares two trees that differ by six files. In this tool it
  printed nothing ("(Bash completed with no output)"), exactly like a pass. So every `--quiet` "exit 0" in my earlier
  files was unproven.
- **The re-reads.** Each one, in `--name-only` form, printed nothing (no path differs), so each earlier result holds:
  - advisor-main-68-head.md:
    - `b9aad01 6b4a0d3` outside spikes/, .checkpoints/ and docs/;
    - the anchor, `83d650e2… 6b4a0d3` outside .checkpoints/ and the two STATUS files;
    - `d2d55b4 6b4a0d3 -- spikes/ docs/design/ .checkpoints/`;
    - the re-run precondition, `6b4a0d3 4e3d1fb` outside spikes/, .checkpoints/ and docs/design/.
  - advisor-main-68b.md: the trial tree `b9aad01 6497e29d` outside spikes/, .checkpoints/ and docs/, and `07cb835
    6497e29d -- spikes/ docs/design/`.
  - advisor-main-68.md (rev 1): `2483e9a 079d8cdd` outside spikes/, .checkpoints/ and docs/; `9868c42 079d8cdd --
    spikes/ docs/design/`; and `2483e9a 9868c42 -- .claude/ .github/ tools/ .gitignore`.
- **Against the current main**, `origin/main 6497e29d` names .github/workflows/toolchain.yml. That is #78's change, not
  a fault of the old read.
- **Finding.** ADV-M69-0 (Low, on record): those files' "exits 0" lines read "held, re-read by `--name-only` on
  2026-11-27 (advisor-main-69.md Block 0)". No verdict changes.

## Block 1. His words of 13:55 UTC (history.md:8163-8186, word for word there)
- **Item 1** ends LEAN-MODE's window rule: "Whenever usage is there, start the next step at once, at any hour".
  - Kept: "only R1's path, at most three agents, every agent saves each finished block at once, and after a cut you
    continue from the notes and redo nothing".
  - The limit: "do not start a step that must not be cut, such as a merge's no-save stretch, unless the usage left
    covers it."
- **Item 2:** "TAKEIN-CODE: (b), with exactly the conditions you listed. You send me the one line, and the advisor still
  reads the pushed result."
  - "The conditions you listed" are msg-2026-11-27.md:71-77, as the main session reads it (history.md:8178-8181): a
    reviewed code PR merged on main, touching no kit, CI, manifest, lockfile, spikes/, design or look doc or mark row;
    main's CI green; the same conflicts.
  - So **a CI change is outside (b)**: the coordinator's "whose conditions leave CI out" means that.
- **Item 3:** #75-GO (a), his go of 2026-11-26 18:52:56 UTC for 2542b1a. ADV-M68B-4 is closed.
- **Item 5:** the interim rule and FULL-PATHS are unchanged.
- **Also read:** FULL-PATHS (history.md:8153-8161), his standing rule of 12:37 UTC, word for word there.
- **The advisor file.** main's advisor.md and CLAUDE.md are unchanged since my whole read at 2483e9a: `git diff --stat
  2483e9a origin/main -- .claude/agents/advisor.md CLAUDE.md` prints nothing.

## Block 2. The plan
- main-69-plan.md rev 0.1, sha256 3c54354c1210d7b1f500d93708ce2f74ad1df7e50bac10de8dd70c45eda47dbb (109 lines). It is
  a delta on rev 4: rev 4's steps, forms and stop rules hold, except where it differs.
- **Its reason holds.** MAIN-52 calls for the take-in, because #78 changes a CI file. The words it cites, history.md:2084-2085,
  are the main session's reading; his own are :2055-2057 ("Take main in when main changes code, CI or rules"). Low,
  M69-6.
- **Neither lighter form fits.** TAKEIN-CODE (b) leaves CI out, and the light recipe is for docs .md moves only.
- **The order holds:** before UM-4 (whose step 0 pins main) and before NB2-ONEWAY's merge.

## Block 3. The facts by git (HEAD 505378b = origin; origin/main b8f02b0 = ls-remote; no fetch)
- **The merge.** First-parent b9aad01..b8f02b0 is one merge, #78 (13:55:47 UTC), with parents b9aad01 and 312a763d.
  Its message names his go of 14:45 Amsterdam in the CI Builder's chat for head 312a763d, and the advisor agent's
  approval. #78's report: security-auditor APPROVE (no findings), infra-engineer APPROVE, and the advisor agent's plan
  approval with five conditions. The merge base with HEAD is b9aad01.
- **What #78 changes.** `--raw`: six paths, every new mode 100644. `--numstat`: toolchain.yml 1/1, DECISIONS 54/0,
  REPO_MAP 2/1, ROADMAP 5/0, STATUS 2/2, and the new report 70/0. `--stat`: "6 files changed, 134 insertions(+), 4
  deletions(-)".
- **toolchain.yml's diff, read whole:** only `timeout-minutes: 15` becomes `timeout-minutes: 25` (line 38). Nothing a
  test runs changes, and no step, permission or action pin changes.
- **The trial merge.** `git merge-tree --write-tree --name-only HEAD origin/main` prints one line, tree b27a9db1…, and
  no CONFLICT line: clean. (The plan's b231cc14 was on HEAD d765947; not bound.)
- **The trial tree:**
  - against main outside spikes/, .checkpoints/ and docs/: `--name-only` prints nothing;
  - docs/ against main: the usual six names, with the branch's own counts (DECISIONS 24/1, REPO_MAP 17/0, STATUS 1/1,
    STATUS_ARCHIVE 18/5, and the two design files);
  - against HEAD in spikes/, docs/design/ and .checkpoints/: nothing;
  - HEAD..trial tree: exactly #78's six files, "6 files changed, 134 insertions(+), 4 deletions(-)".

  So the step-2 merge should print git's summary as the plan expects.
- **The guard's view.**
  - origin/main...HEAD over the kit pathspec (with the CLAUDE.md, AGENTS.md and `.claude/` globs): nothing. So a plain
    `git merge origin/main` passes main's merge rule (routine-guard.mjs:1424-1434 at b8f02b0).
  - HEAD...origin/main over the kit: only .github/workflows/toolchain.yml.
  - `git diff --name-only origin/main HEAD -- .claude/ tools/ .gitignore CLAUDE.md AGENTS.md`: nothing.
- **Refs and untracked files.** `git grep -a -l "spikes/"` on main outside spikes/, docs/, .checkpoints/ and `*.md` names
  only kept-logs.mjs and its test, so the refs step's bar is N0 + 1. check.mjs:1880 (referenceProblems) and :2695-2700
  (refs) are cited right at HEAD. No untracked file sits under .github/ or docs/.
- **Main's CI on b8f02b0** (GitHub MCP), push runs, attempt 1, all success:
  - gate 46011351316 (gate 140547367425, smoke 140547367958), 13:55:54-13:57:01;
  - toolchain 46011355903 (job 140547396571), 13:55:57-14:03:27; build-and-test 7 min 9 s, of the new 25-minute limit;
    #74's two steps skipped.

## Block 4. What #78 changes for PR #40
- PR #40's toolchain runs get 25 minutes instead of 15. MAIN-68's runs took 7-9 minutes, so this is headroom, not a
  change in what runs.
- After the merge, `.github/` equals main's, so PR #40's diff against main holds no CI file and stays tier 1. M69-1
  makes that a recorded bar.
- Step 7's summary lines should equal MAIN-68's baseline (job 140114848934): #78 adds or removes no test.

## Verdict: APPROVE WITH CONDITIONS M69-1 to M69-6, for main's tip b8f02b0149cc928964e54e194cd9c4d68f52bd94
Every fact the plan states holds by my own reads (Blocks 2-3). The take-in is due (MAIN-52: CI), and the clean merge
keeps rev 4's rules without its conflict steps.

The conditions are all Low. They:
- make the tier claim a recorded bar;
- name step 7's baseline;
- say what "the same conflicts" means for a later move;
- write out the resume line;
- put the inherited `--quiet` bars into printing forms;
- fix one citation.

Applied word for word, they need no new review. Step 7's read of the pushed head stays. Nothing is Critical, High or
Medium. Risk tier 1 for PR #40 after the merge.

## Conditions (paste-ready)
- **M69-1 (Low): the whole-tree bars, at step 5 after the content checks, each alone, in printing forms:**
  - `git diff --name-only origin/main HEAD -- . ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/'`
    prints nothing;
  - `git diff --name-only origin/main HEAD -- docs/` prints the usual six names (DECISIONS.md, REPO_MAP.md, STATUS.md,
    STATUS_ARCHIVE.md, design/netcode-proof-messages.md, design/ws-reconnect.md);
  - `git diff --name-only H0 HEAD -- spikes/ docs/design/ .checkpoints/` prints nothing.

  These are MAIN-66's C1 (c). They prove "PR #40's diff against main holds no .github/ path".
- **M69-2 (Low): step 7's baseline.** It is main's toolchain run 46011355903, job 140547396571, compared line for line as
  M67-3 gives it (the `test result:` lines, the vitest `Test Files` and `Tests` lines, every Playwright "passed" line).
  Its lines should also equal job 140114848934's: #78 changes only the time limit. #74's two pull-request steps keep rev
  4's step-6 bar. Record the toolchain job's run time beside the 25-minute limit.
- **M69-3 (Low): a move of main found at step 1.** P-2 and TAKEIN-CODE (b) apply with "the same conflicts" read as
  none: step 1's anchor bar (one tree line, no CONFLICT line) must still hold. A move that brings any conflict stops
  before step 2 for a check.
  - Under TAKEIN-CODE (b), "reviewed and merged on main" is read from the merge's message or report: it names the
    advisor agent's approval of the merged head and his go. Without that, the move stops before step 2.
  - Each move taken in by (b) or by P-2 (a) gets one line to him naming its pull requests. The advisor's read of the
    pushed head stays.
- **M69-4 (Low): the resume line (C5 (a)), word for word.**

  "MAIN-69 approved (advisor-main-69.md), running steps 1-7 for main b8f02b0149cc928964e54e194cd9c4d68f52bd94; usage at
  step 1: <figure>, <go or no-go>. If `ls -d /home/user/Shamble-Frontlines/.git/MERGE_HEAD` lists the file, a merge
  stopped in progress: no save, no `git merge --abort`, no reset; the advisor rules. If `git rev-parse HEAD^2` prints
  b8f02b0149cc928964e54e194cd9c4d68f52bd94 and `git ls-remote origin refs/heads/claude/netcode-proof-test-plan-hcky5g`
  prints another SHA than `git rev-parse HEAD`, resume at step 5. advisor-main-69.md and this card may show unstaged:
  leave them; step 6's card save takes them. Anything else: the advisor rules."
- **M69-5 (Low): no `--quiet` bar.** Every bar this take-in inherits from rev 4 in a `--quiet` form runs as
  `--name-only` and must print nothing. This means step 6's DEP-HOLD line (C68B-3), in its own call: `git diff
  --name-only origin/main HEAD -- Cargo.lock Cargo.toml crates packages package.json package-lock.json` prints nothing.
  Its control is step 5's six-path line.
- **M69-6 (Low): "Why" cites his words.** history.md:2055-2057, word for word there: "Take main in when main changes
  code, CI or rules". The main session's reading at :2082-2086 may stand beside them.

## The plan's questions
- **Q1. Is step 5 enough without a whole check.mjs run? Yes.**
  - #78 changes no file the spike's code, manifests or tests read: no spikes/ path, no root manifest or lockfile, and
    the spike's workspace is its own.
  - The whole check.mjs builds and tests the spike with cargo. It is the bar for spike changes, never for take-ins:
    MAIN-66, MAIN-67 and MAIN-68 ran the same light set.
  - The light run's refs step is the one that reads files outside spikes/, and it stays.
  - PR #40's toolchain on the pushed head then runs the root workspace with #78's limit.
- **Q2. May 5e (main's guard tests) be left out? Yes.**
  - #78 changes no .claude/ file, and step 5's six-path bar proves the merge adds none.
  - HEAD's .claude/ equals main's (`--name-only` printed nothing, Block 3).
  - The guard's tests ran 1551/1551 on this guard at MAIN-68's step 5 and at its step 7 re-run.
- **Q3. Is the plain form right? Yes:** `git merge origin/main`, with the checks after git's commit. Not `--no-commit`.
  - Main's merge rule (routine-guard.mjs:1425-1428 at b8f02b0) passes only `git merge origin/main` and `git merge
    --ff-only origin/main`. Any other form falls to "only origin/main merges into the main folder", which asks him in a
    day session: a needless prompt.
  - Checking after the commit is safe:
    - step 1's trial tree T is the pre-commit check;
    - the anchor `git diff --name-only T HEAD` with its six-path control proves the commit equals it;
    - nothing is pushed before step 6, and a miss stops with the merge local and unpushed (the stop rules).
  - The plan's diffs name both commits in order (`H0 HEAD`, `T HEAD`), never HEAD in place of `--cached` (the recipe's
    L-4 lesson).
- **Q4. May step 7 ride B's or NB's seat? Yes, either.** Both are advisor agents (B wrote advisor-u8-f1-head.md, NB
  advisor-nb2-merge.md). C's seat is not required.
  - It must be an advisor agent's read by main's advisor.md, never D's (a builder).
  - Its brief carries the form (advisor-main-66.md Q1 (d)), M69-1's and M69-2's values, and the printing-forms rule.
  - Three agents at most.
- **Q5. Is a screenshot asked of him? No, the same as MAIN-68's Q5.**
  - The wall bullet's rule is for a kit, CI or gate change-set. MAIN-69 changes no kit, CI or gate file against main:
    it writes main's toolchain.yml into the branch byte for byte. After it, PR #40's diff against main holds none
    (M69-1).
  - Nothing a merge on this branch does can change a ruleset.
  - #78 itself was the CI change-set, and its start was its own lane's.

## Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-M69-0 | Low (on record) | My earlier files' `git diff --quiet` "exit 0" lines were unproven in this tool (the control printed nothing for differing trees). Re-read by `--name-only`, all hold. | Block 0; no verdict changes |
| ADV-M69-1 | Low | No whole-tree bar for the tier claim. | M69-1 |
| ADV-M69-2 | Low | Step 7's baseline job is not named. | M69-2 |
| ADV-M69-3 | Low | "The same conflicts" for a later move is not spelled out for a clean take-in, nor where "reviewed" is read. | M69-3 |
| ADV-M69-4 | Low | The resume line is given by pointer, and rev 4's form uses `rev-parse -q --verify`, which prints nothing when absent. | M69-4 |
| ADV-M69-5 | Low | Step 6 inherits rev 4's `--quiet` DEP-HOLD bar. The saver's own hold prints, so the reach is small. | M69-5 |
| ADV-M69-6 | Low | "Why" cites the main session's reading, not his words. | M69-6 |
| ADV-M69-7 | Low (for the recipe) | light-takein-recipe.md's anchor and step 5e bars use `--quiet` too. | When L-1 to L-12 are applied, each `--quiet` bar becomes `--name-only` printing nothing |
| ADV-M68B-4 | Closed | #75's go: his #75-GO (a), history.md:8169. | None |
| ADV-M68-5 | On record (main's) | R4-1 Critical, open on main by his R4-END (1); ADV-R4-2. | Main's next guard change-set |
| ADV-M60-7, ADV-M54-8 | Low (open) | As before. | As ruled |

## What I did not check
- I ran nothing that writes: no merge, gate, light run or test. The clean merge, its summary line and the counts come
  from the trial tree.
- How long the no-save stretch will take against the usage left: that is step 1's reading. For the record, MAIN-68's
  ran from the saver's stop to the push in about 6 minutes (20:14:24 to 20:20:00 UTC, step1-record.txt and the
  reflog), with two conflict Edits that MAIN-69 does not have.
- #78's DECISIONS.md entries against his words: main's content, taken as it is.

## Decisions needed
None. MAIN-52 is his standing rule; #78 merged on his go; the merge is clean; and his 13:55 UTC answers (the window rule
ended, TAKEIN-CODE (b), #75-GO (a)) are applied as he worded them.

## For Mustafa
Nothing. No browser check, no screenshot (Q5), and no command needs his reply (the plain merge form passes the guard).

Status: COMPLETE 2026-11-27T14:19Z
