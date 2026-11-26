# Advisor: MAIN-68 plan check (main 2483e9a, #70, into PR #40's branch)
2026-11-26, 14:5x-15:2x UTC window. Advisor agent (general-purpose, opus, effort xhigh), day session, read only.
I acted by main's advisor.md (`git show origin/main:.claude/agents/advisor.md`, read whole).
- Plan: main-68-plan.md, sha256 e0bb8c75e1c2bc760ddca3d9f013f5ebd8d056e3d6ca34f75e821ed90e2eca24.
- Binding form: main-66-plan.md as approved (M66-1 to M66-5, advisor-main-66.md), as MAIN-67 applied it
  (main-67-plan.md, advisor-main-67.md M67-1 to M67-3, advisor-main-67-head.md).
- At the start: HEAD 9868c42 (equal to the remote branch by ls-remote); local origin/main 2483e9a. I made no fetch
  (the brief's rule).

## 1. Verdict: REVISE. Main moved after the plan was written.
**Main is now 4a62873e0385ad8f8bf0bb3bfd45d3f41489d34f.** `git ls-remote origin refs/heads/main` shows it. The GitHub
tools show one new first-parent merge after 2483e9a:
- #73, "guard-wrappers (SAME-GAP (a))", merged 14:56:33 UTC.
- Its merge message: "Merged on Mustafa's go of 2026-11-26 15:45 Amsterdam for head 7bff799f…, after the advisor
  agent's APPROVE of that head and his Advisor's merge check. R4-1 (Critical) stays open by his R4-END (1)."
- Its 8 files, by GitHub's list for the merge commit:
  - .claude/hooks/routine-guard.mjs (+55/-14) and .claude/hooks/routine-guard.test.mjs (+111/-3);
  - docs DECISIONS, REPO_MAP, ROADMAP, STATUS (+32/-32) and STATUS_ARCHIVE;
  - one new report, docs/reports/2026-11-26-guard-wrappers.md.

**Main's CI on 4a62873 is green.** These are main's push runs, attempt 1, read with the GitHub MCP tools:
- gate run 45844058153: gate job 139992978242 and smoke job 139992978857, both success (14:56:40-14:57:50);
- toolchain run 45844058166: job 139992977903, success (14:56:41-15:02:17). It was still running at my first read.

**Why this is REVISE:**
- The plan cannot run as written. Step 1's `git fetch origin` makes origin/main 4a62873, and C4 then stops the take-in
  before step 2. That stop is right: advisor-main-65.md:113-114 says "no tolerance … a new take-in needs a new plan",
  and M66-5 binds it.
- The guard passes only a plain `git merge origin/main` in the main folder (advisor-main-66.md section 5). That form
  takes main's tip, so the take-in now carries #73 as well as #70.
- #73 changes two kit files, the guard and its tests (risk tier 2 on main). That is MAIN-66's case (#69), not MAIN-67's.
- It is also outside his LIGHT-TAKEIN limits.
- Three of the plan's statements no longer hold for the merge the guard would run: "#70 touches no .claude/ file, so the
  guard's rules stay main's c1da1bd ones" (Facts :35), "the count is 1,247 again" (step 5), and every count and number
  in "The numbers".
- I cannot write conditions that apply word for word. #73's objects are not in the local store
  (`git cat-file -t 4a62873…` fails), and I made no fetch, so no new count, trial merge or block line is mine.

**What still holds.** Every fact the plan states about 2483e9a holds when I re-run it (section 3). So the revision is a
bring-up to main's tip. The next check reads only #73's half and the new numbers.

Severity: ADV-M68-1, High (the plan's facts no longer hold for the take-in it would run). C4 makes it stop safely.
Nothing is Critical. Risk tier 1 for PR #40 after the merge: its diff against main holds no kit file.

## 2. The exact changes (R68-1 to R68-8; then one advisor check of the brought-up plan)
- **R68-1: bring the plan up to main's tip.** After `git fetch origin` in its own call, re-derive every fact by git:
  - the first-parent list 07d673b..tip (expected: #70, #73);
  - `git diff --name-status 07d673b origin/main` and its counts. Expected about 42 paths (A 22, M 20). That is my union
    of git's 39 and GitHub's 8; it is not a git count;
  - the whole shortstat, the one without docs/STATUS.md, and STATUS.md's numstat;
  - main's STATUS.md line count (`git grep -c ""`). My hand count of the GitHub copy is 120;
  - `git merge-tree --write-tree --name-only HEAD origin/main`: its exit, its CONFLICT and auto-merge lines, and the
    block's markers and rows;
  - the trial tree's checks, as MAIN-66 ran them: the whole tree outside spikes/, .checkpoints/ and docs/; spikes/ and
    docs/design/; the auto-merged docs' numstat both ways;
  - the refs grep, the untracked check and df.

  If main moves again before the brought-up plan's check, bring it up again to the new tip (no tolerance).
- **R68-2: the reason and the record.**
  - MAIN-52 calls for #73 as well, because it changes rules (two kit files). The plain merge takes it with the tip
    anyway.
  - His 12:21 word (history.md:8075) names #70 only. Say so, as MAIN-66 did for #67.
  - #73 merged on his go, by its merge message (quoted in section 1).
  - The "Choices taken without him" line and the one line to him name both pull requests, #70 and #73.
- **R68-3: MAIN-66's kit lines and steps come back whole** (main-66-plan.md: Facts :63-80, step 2's last sentence, 5e,
  and the risk at :223-225). Concretely:
  - **Facts.** Main changes .claude/hooks/routine-guard.mjs and routine-guard.test.mjs and no other `.claude/`,
    settings.json, guard.mjs, `.github/`, tools/, CLAUDE.md or AGENTS.md file. Re-derive this by git.
  - **The guard's pathspec, both ways.** origin/main...HEAD prints nothing (I read it empty on 2483e9a; re-derive it).
    HEAD...origin/main prints exactly the two guard files.
  - **The merge rule.** Quote its lines, read on the new tip's routine-guard.mjs by git. GitHub's patch puts #73's
    three hunks at :298 (`WRAP_OPTS` and `unwrap()`), :787 (`bash()`: rule U and xargs) and :847 (constants). None of
    them is in the merge rule. That is a read through GitHub, not git; confirm it by git.
  - **M66-2 applies whole:** name step 2's command first, and follow its refusal branch. The merge writes into
    `.claude/hooks/` (CLAUDE.md step 3: the auto-mode classifier may refuse a `.claude/` edit).
  - **5e in M66-3's form.** The bar is 0 fail, with the count recorded. Expect 1,551: #73's report ("Evidence":
    "routine-guard 1247 on main → 1437 → 1490 → 1499 → 1551/1551") and main's STATUS row agree. M65-2's fallback for
    another total stands. Step 7 re-runs it.
  - **The risk line.** From the first call after step 2's `git merge` writes main's routine-guard.mjs, #73's rules check
    this session's calls. The calls of any agent whose hook resolves to the main folder may come under them too; that
    is untested (ADV-M66-7). Each later step's command stays a plain form. If one asks or is refused, the step stops,
    the merge stays, and the advisor rules.
- **R68-4: the commands under #73's rules.** List every command of steps 2-6. For each, say that no program stands in
  front of git or gh, and that no xargs runs at the session level. Under #73:
  - rule U asks in a session (and refuses an agent) for a program the guard does not strip with git or gh after it;
  - xargs asks unless a read-only program follows it;
  - timeout, xvfb-run, command, time and exec read their options from the table.

  main-run.sh's own xargs (:42) and git calls run inside the script, which the guard does not read (#73's report,
  Risks 2, INPUT-RUN). For Builder 2's and U7's merges after the take-in, see Q3 (b).
- **R68-5: R4-1 on record** (Facts and Risks).
  - #73 left R4-1 (Critical) open by his R4-END (1). Its report: "one more spelling hides git, gh or xargs from the
    guard's reading, for a session and a subagent … open on main too". Its fix is the next guard change-set.
  - Its report ("Outcome"): "His interim rule stays until R4-1's fix merges".
  - No security read for the take-in. Only part of MAIN-66 Q3's reasoning carries over. These three reasons carry:
    - main's files come in byte for byte;
    - after the merge, PR #40's diff against main holds no kit file;
    - #73 merged on his go with R4-1 left open by his own R4-END (1), so the open finding is main's, not this lane's.

    Two of MAIN-66's reasons do not carry; say so in one line:
    - #73's last security round was REVISE, not APPROVE;
    - #73's report names ADV-R4-2: 91 swept lines looser than the guard before it. Six now pass where timeout gets a
      lone dash, and timeout then runs nothing. 85 ask in a session instead of being refused; subagents are still
      refused. So unlike #69's, #73's rules do not only add asks or denies.

    Put ADV-R4-2 on record next to R4-1, for the brought-up plan's check.
- **R68-6: C4's and step 7's values.**
  - C4: origin/main = 4a62873e0385ad8f8bf0bb3bfd45d3f41489d34f (unless it moved again).
  - Main's push runs on it: gate 45844058153 (jobs 139992978242 and 139992978857) and toolchain 45844058166 (job
    139992977903). All completed with success at my 15:1x UTC read; read them again at step 1.
  - Step 7's toolchain baseline is job 139992977903. By GitHub's list, #73 changes no crates/, packages/ or `.github/`
    path, so its summary lines should equal 2483e9a's job 139905981337. Step 7 compares with 139992977903.
- **R68-7: write these out, not by pointer** (ADV-M67-1's point, which M67-1 to M67-3 fixed last time):
  - C5 (a)'s resume line: M67-1's text, with "MAIN-68", the brought-up plan's check file and the tip's full SHA;
  - C2 (b)'s anchor, in M67-2's form (from H0, after the fetch);
  - C6's kept and dropped lines, with their numbers.

  The expected shape, from main's file (the GitHub copy at 4a62873):
  - ours: #40 (the branch's row), then the ci-after-3a "none yet" row;
  - theirs: main's #40 (the 2026-11-14 text), then the guard-wrappers "none yet" row (#73's);
  - keep ours #40, then main's guard-wrappers row;
  - the line numbers move: main's file gains a 7-line "The day guard" bullet above the table.
- **R68-8: Facts :24-25.** Toolchain 45818982052 was not "in progress at 12:3x": it completed at 12:28:06 UTC (job
  139905981337, success).

Applied, the brought-up plan goes to one advisor check. That check reads #73's half and the re-derived numbers, and it
stands on section 3 for #70's half.

## 3. What I checked on 2483e9a (read only; each holds as the plan says)
- **SHAs.** HEAD 9868c42 equals the remote branch. The merge base is 07d673b. 07d673b..2483e9a holds one first-parent
  merge, #70 (parents 07d673b and 420b16b; author date 12:22:00 UTC), and its message is as the plan quotes it.
- **What #70 changes.** 39 paths: A 21, M 18; `--no-renames --diff-filter=DRCTUXB` prints nothing; `--raw` shows every
  file as 100644 (no link, no submodule).
  - "39 files changed, 5844 insertions(+), 158 deletions(-)"; without STATUS.md, "38 files changed, 5815 insertions(+),
    129 deletions(-)"; STATUS.md 29/29.
  - Main's STATUS.md has 120 lines. first-loop.rec is 122,487 B (binary in numstat).
  - The plan's per-crate list matches the name-status.
- **No kit, manifest, spike, ADR or design path.** The diff names nothing under .claude, .github, tools, legacy, spikes,
  docs/adr or docs/design. It names no .gitignore, Cargo.toml (any depth), Cargo.lock, package.json, package-lock.json,
  .npmrc, CLAUDE.md, CLAUDE.local.md or AGENTS.md.
  - origin/main...HEAD over the kit pathspec prints nothing.
  - `.claude/ .github/ tools/ .gitignore` equal main's now (exit 0).
- **The trial merge.** `git merge-tree --write-tree --name-only HEAD origin/main` (at 9868c42) exits 1 with tree
  079d8cdd. Its one CONFLICT is docs/STATUS.md; it auto-merges DECISIONS, REPO_MAP, STATUS and STATUS_ARCHIVE.
  - The markers sit at :24, :27 and :30. Ours: :25 (#40, the 2026-11-15 text) and :26 (ci-after-3a). Theirs: :28
    (#40, the 2026-11-14 text) and :29 (#70).
  - `-U0 origin/main..tree` shows only the block. Keeping :25 and :29 gives main's file with line 24 swapped: 120
    lines, "1 1".
  - The branch's own STATUS.md change since 07d673b is 1/1.
- **The rest of the trial tree.** It equals main outside spikes/, .checkpoints/ and docs/ (exit 0). Its spikes/ and
  docs/design/ equal HEAD's (exit 0).
  - HEAD..tree without STATUS.md and .checkpoints/: the 38-file line.
  - The auto-merged docs' numstat is the same both ways (DECISIONS 24/1, REPO_MAP 17/0, STATUS_ARCHIVE 18/5), and
    docs/ differs from main in the usual six names.
- **Refs, untracked files and disk.** The refs grep (`-a -l`, outside spikes/, docs/, .checkpoints/ and `*.md`) names
  only kept-logs.mjs and its test. No untracked file sits under crates, packages or docs. 13,975 MiB free at 15:0x.
- **Main's CI on 2483e9a.**
  - Gate run 45818982040: push, main, attempt 1, success, 12:22:04-12:23:01; gate 139905982025 and smoke 139905982614.
  - Toolchain run 45818982052: success, 12:22:04-12:28:06; job 139905981337, its build-and-test step 12:22:29-12:28:03
    (6 of the job's 15 minutes).

## 4. The plan's questions (answered for #70; they carry into the brought-up plan)
- **Q1. Does #70 need more than MAIN-66's steps? No, not for #70 itself. #73 now does: R68-3 to R68-5.**
  - No kit, CI, manifest, spikes/ or docs/adr/ path, so DEP-HOLD is free and no `| PM-` row moves.
  - Every file is 100644.
  - The refs step reads every tracked file as UTF-8 text (check.mjs:2625-2630, :1852-1866). The `-a` grep shows
    "spikes/" in none of #70's files, the 122,487 B recording included.
  - docs/UNITS.md is a decision doc (advisor.md's list), and #70 changed it on main on his go. After the merge, PR #40
    has no UNITS.md difference against main, so its tier stays 1.
  - Toolchain on #40 now also runs #70's tests. On main they took 6 of the job's 15 minutes.
- **Q2. Does the light run build any of #70? No. Nothing to add for #70.**
  - main-run.sh runs in spikes/netcode-proof (:14, :26) and touches only $S/crates (:48-52).
  - The marks step is `cargo run -q --locked -p npt-judge` there (check.mjs:2633).
  - The spike's Cargo.toml has its own 17 members, and no member manifest has a `path = "../..` dependency.
  - #70 changes no Cargo.toml or Cargo.lock.
  - Only refs reads #70's files (as text). The self tests read no root file but .gate/ paths (check.test.mjs:2564).
  - One addition, for Q3: Builder 2's head changes crates/judge and marks/marks.json. If its merge lands before the
    take-in's H0, the first light run rebuilds the judge (a longer exclusive hold) and prints the new marks line. The
    base run on H0 gives it, as written.
- **Q3. Is there an order constraint? Only "one at a time, never inside another's no-save window", with two points.**
  - Checked: `git merge-base HEAD 0e621dae` is e2c0e896, and `git diff --stat e2c0e896 0e621dae -- .
    ':(exclude)spikes/' ':(exclude).checkpoints/'` prints nothing. U7's wip/u7-build-rs.patch has 43 `diff --git`
    entries, all under spikes/.
  - (a) Builder 2 changes the judge and marks.json, and the take-in adds main's new files to refs. So each of the three
    takes its bars from its own base run on its own H0, never another's numbers (MAIN-66 Q2 (b)).
  - (b) After the take-in, Builder 2's and U7's merges run under #73's guard. Before naming each command, check its
    list:
    - no program in front of git or gh, and no xargs at the session level;
    - Builder 2's merge prompt stays a prompt: #73's hunks leave the git merge rule's text alone. Confirm it by git on
      the then guard.
  - Builder 2 goes first whenever he is at the prompt (his "ready").
- **Q4. May step 7 ride U7's head read at 19:50 UTC? Yes,** on advisor-main-66.md Q1 (d)'s terms:
  - three agents at most;
  - step 7 never waits for U7's head;
  - the baseline is R68-6's job.

  The REVISE changes the timing. The brought-up plan's check is new agent work: it waits for 19:50 unless the main
  session's LEAN-MODE reading lets it ride an agent already running. Steps 1-6 come after that check, and step 7 then
  goes to the next window unless it rides an agent then running. Builder 2's and U7's merges are not held by this
  (Q3).

## 5. Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-M68-1 | High | Main moved to 4a62873 (#73: two guard files and docs) after the plan was written. C4 stops the plan at step 1. The only merge form the guard passes takes the tip, so the take-in now carries #73's kit files. The plan's kit line, 5e's count and every number no longer hold for that merge. | R68-1 to R68-6; one new check |
| ADV-M68-2 | Low | The resume line, the anchor and step 7's job are given by pointer (as ADV-M67-1). | R68-7 |
| ADV-M68-3 | Low | Facts :24-25: toolchain 45818982052 is called "in progress at 12:3x". It completed at 12:28:06 UTC. | R68-8 |
| ADV-M68-4 | Low (on record) | Main's STATUS.md at 4a62873 (GitHub copy) still lists guard-wrappers as "none yet … no pull request yet", though #73 merged. It is main's text, taken as it is. | Main's next docs pass |
| ADV-M68-5 | On record (main's) | R4-1, Critical, is open on main by his R4-END (1). ADV-R4-2: 91 swept lines are looser than the guard before #73 (6 pass, harmlessly; 85 ask in a session instead of being refused). The take-in brings main's guard as it is. | R68-5; R4-1's fix is main's next guard change-set |
| ADV-M67-2 | Closed | Main's ci-after-3a row: #70's docs pass moved it to the archive (main's STATUS.md at 2483e9a, line 5). | None |
| ADV-M60-7 | Low (open) | The #40 row says "main 53272a1 merged in". | PR #40's closing docs pass |
| ADV-M54-8 | Low (open) | CLAUDE.md's Never line lists no plain `git merge origin/main`. | The kit follow-up after the freeze |

## 6. What I did not check
- **Nothing on 4a62873 by git.** Its objects are not local, and a fetch would move the ref that C4, the guard's merge
  rule and the saves read. What I know of #73 comes from the GitHub tools:
  - its file list and patch;
  - its report;
  - main's STATUS.md at 4a62873;
  - the CI runs;
  - ls-remote.
- **No runs.** I ran no merge, gate, light run or test (read only). The 120 lines and the "1 1" come from the trial
  tree.
- **Not read:** U7's snapshot contents (A's read) and Builder 2's merge plan beyond its xargs and timeout lines. The
  "timeout 7200000" there is the tool's timeout, not a shell wrapper.
- **The patch-ids** were not run (they need a pipe). The numstat both ways stands for them.

## Notes (one line per check, as run)
- Read: advisor.md (main), main-68-plan.md, main-66-plan.md, advisor-main-66.md, main-67-plan.md, advisor-main-67.md,
  advisor-main-67-head.md, launch-0850.md:100-117, launch-1450-msgs.md, history.md:8020-8106.
- `git rev-parse HEAD origin/main`: 9868c42…, 2483e9a…; merge-base 07d673b…; first-parent: #70 only.
- name-status 39 (A 21, M 18); shortstat and numstat as section 3 says; DRCTUXB none; raw all 100644; .rec 122487 B.
- merge-tree: exit 1, tree 079d8cdd49f7…; markers :24 :27 :30; rows as stated; -U0 vs main: only the block.
- The kit, manifest and spike pathspecs: empty; origin/main...HEAD kit: empty; kit folders vs main: exit 0.
- refs `-a -l`: kept-logs.mjs and its test; untracked: none; df 13,975 MiB.
- CI on 2483e9a by run and job id: all success (section 3).
- GitHub runs list (push, main): newer runs for 4a62873 (#73). ls-remote main = 4a62873…; `git cat-file -t` fails
  (not local).
- get_commit 4a62873: 8 files (section 1), +666/-60. The patch's three routine-guard.mjs hunks are in unwrap(), bash()
  and the constants, not the merge rule. list_commits: 2483e9a, then #73's branch commits, then 4a62873.
- #73's report: R4-1 Critical open; 1551/1551; his interim rule until R4-1's fix merges; INPUT-RUN.
- Main's STATUS.md at 4a62873 (GitHub): In flight = main's #40 (2026-11-14 text) + guard-wrappers "none yet"; a 7-line
  "The day guard" bullet in Phase; 120 lines by hand count.
- CI on 4a62873: gate 45844058153 (139992978242, 139992978857) and toolchain 45844058166 (139992977903), all success
  by 15:02:17.
- Builder 2: merge base e2c0e896; outside spikes/ and .checkpoints/ empty; it changes crates/judge (src, tests,
  fixtures) and marks/ (marks.json, README, SERIES). U7's patch: 43 entries, all spikes/.
- The spike's Cargo.toml: own members, no `path = "../..`; main-run.sh :14, :26, :48-52; check.mjs refs :2625-2630,
  marks :2632-2641; check.test.mjs reads only .gate/ paths at the root.
- Closing read (15:2x, ls-remote): main still 4a62873; the remote branch moved to f18d8f3 (the main session's save; I
  did not read what it holds).

## Decisions needed
None. MAIN-52 is his standing rule. The bring-up takes #73 by the same merge form, and #73 is main's merge on his go. If
the bring-up's facts show something else (a second conflict, a guard merge-rule change, a red run), it stops for the
check, as the plan's stop rules say.

## For Mustafa
Nothing.

DONE
