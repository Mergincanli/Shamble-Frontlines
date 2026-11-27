# Advisor: U7's pushed merge (commit 6968215 on H0 dc51f0e, pushed by 9c2fcf4) — not a round (U7-M-8)
Advisor agent, day session, effort xhigh, read only (no cargo). Acting by main's advisor.md.

## Plan
1. The head: SHAs, parents, `git diff --stat dc51f0e 6968215 -- spikes/` against the snapshot's numstat; whole tree
   against main outside spikes/.checkpoints/docs; WIP-CLEAN; later commits.
2. The records: the plan's done lines, light-base.log, m2b-header.log, c1-judge.log, c1-verdicts.json, m5-check.log;
   CI on 9c2fcf4 by run id.
3. M3's retry against M6; the form's fix.
4. The third patch; ADV-U7S-6's home.
5. UM-5's cleanup forms (Read/Glob on absolute paths, ffcheck from the main folder).
6. u7-report.md against U7-M-7.
7. Decisions needed, For Mustafa, Status.

## 1. The head: MERGE stands
- **The commit.** 6968215 is one commit on H0 dc51f0e. `git diff --stat dc51f0e 6968215 -- spikes/` gives 43 files,
  5,193 insertions and 47 deletions: the snapshot's numstat, which the M8 done line checked line for line. All 43 blob
  ids equal `git hash-object` of the builder's worktree files, and M2b tied that worktree to run 4's tree. Every file
  is 100644: 33 added, 10 modified.
- **Outside spikes/.** The commit touches only .checkpoints/. With MAIN-68 in (merge-base with main = b9aad01), the
  pushed tree equals main's outside spikes/, .checkpoints/ and docs/. The card save 9c2fcf4 is notes only.
- **The window.** The remote moved straight from H0 to 9c2fcf4 (M9's line). The six commits since are notes only.
- **The records hold:**
  - light-base.log at HEAD dc51f0e: refs 2,106, marks 81, self 78/78, redact 431 items, 0 findings.
  - m2b-header.log: HEAD 80b6748, "diff 551e9033…; untracked b4811e75…".
  - C1: c1-judge.log exits 0 with no "npt-judge:" line. c1-verdicts.json: PM-0001-36 NOT-RUN, its "ML* (rough,
    reported)" part PASS. The Wasm stamp 901a6492… is UM-3's.
  - m5-check.log, 22:15:37-22:36:58: ops 101/101; wrappers "315 ok, 0 failed"; check.mjs 12/12, test 931 and 1028
    (UM-2's figures exactly), refs 2139 = 2106 + 33, redact 464 = 431 + 33, self 80, web 70.
- **CI on 9c2fcf4 (by run id):** push gate 45906116335, push toolchain 45906116365, pull-request gate 45906121962 and
  pull-request toolchain 45906121987, each completed with success on attempt 1.
- **Findings on the head:** none.

## 2. The M3 retry: accepted; M6 need not have run (Low, a departure to record)
- **The letter.** M6 opens "a failing test or bar is never retried unnamed. It stops the merge". A refused
  `git apply --check --index` is a failing step, so by the letter M6 would have run.
- **Why the retry still stands:**
  - git changed nothing: it was a check, and nothing had been applied, so M6's undo would have been empty.
  - The cause was shown to be stale cached times, not content. Three content checks exited 0: tree = index and
    tree = H0 under spikes/, and M2. The mtime read places all of crates/ at the base run's end.
  - The retry was named in the chat before it ran, was single, and had a second failure declared as M6 in advance.
  - The later bars prove the applied tree: M4's byte-and-mode compare, then M5.
  - It is the class MAIN-61's C2 (d) ruled for a stale `.git/index.lock`: git changed nothing, so one retry.
  - `git update-index --refresh` wrote only the stat cache, inside the no-save window, with no content staged.
- **The record.** u7-report.md's M3 line adds: "accepted by the advisor's head read (advisor-u7-head.md section 2): a
  stat-only refusal, retried once by name; a departure from M6's letter, not its purpose".

## 3. The form's fix (for every merge of this form whose patch changes tracked files under crates/)
- **Why the base run's timing is not the fix.** Moving the base run before H0's save does not cure it: C1's judge and
  M5 also run main-run.sh after M4, so M6's undo would meet the same stale times. The fix is a refresh right before each
  `--index` apply. (U8's merge passed because its 43 files were all new: `git apply --index` checks only paths it
  modifies.)
- **The words to write into the form,** after M2b:
  > "**M3a** (its own call, right before M3): `git update-index --refresh`. main-run.sh:48-52 touches every file under
  > crates/, so a run through it leaves the index's cached times stale, and `git apply --index` refuses such a path
  > without reading its content. The bar: exit 0, or exit 1 whose 'needs update' lines name only paths under
  > .checkpoints/. Then each alone: `git diff --quiet -- spikes/` (tree = index) and `git diff --quiet HEAD -- spikes/`
  > (tree = H0), both exit 0. A refusal at M3 after this is a failing bar (M6)."
- **And in M6:**
  > "The undo starts with `git update-index --refresh` (the same bar) and `git diff --quiet -- spikes/`, then `git
  > apply -R --index` of the patch, then `git diff --quiet H0 -- spikes/`."

## 4. The third patch: agree
wip/u7-t-build.patch (at dc51f0e) holds 19 web/ files, every one among the 43, so its content is inside the snapshot
and superseded. WIP-CLEAN takes it by the same rule as the T5 (i) patch (23 web/ files, also all among the 43). The
plan's M8 named two. The removal was named before M8 (the done line at 22:0x) and is recorded in u7-report.md's M1
bullet and the commit message: a recorded departure, no fix.

## 5. ADV-U7S-6's home: agree, with a guard before R1
- The test goes on PR #40's closing list (U8's first change after R1), since U8's 6,282 stop is spent.
- Add one check at Stop B's fill: if `git diff --stat 9c2fcf4 <then> -- spikes/netcode-proof/ops/runs/r1.sh
  spikes/netcode-proof/crates/bots/src/load.rs` is not empty, the advisor reads the pairing again (load_settings' 24
  keys and machine's 5 against LoadSettings' fields, types and ranges) before R1.
- Until then, my read of 26 Nov stands (advisor-u7-snapshot.md section 1).

## 6. UM-5: the cleanup forms (none is a KEPT-LOGS set)
KEPT-LOGS (CHECKPOINTS.md §11) covers only the raw-log folder of a closed run set under `.gate/netcode-proof-test/runs/`
and the one old cache. U7's T5 folders are a unit's test runs, so kept-logs.mjs does not apply to them. They are kept by
moving them into the main folder's ignored `.gate/` first, and the summaries their numbers cite are copied into the
notes.
- **A. The builder's worktree** (.claude/worktrees/agent-a9873cdba07a3219e), in this order:
  1. **The record.** Each item is equal to a committed copy:
     - its 43 spike files equal 6968215's blobs (all 43 read);
     - its notes u7-build-rs.md and u7-t-build.md equal wip/u7-build-rs-notes-1055.md (a65282fa…) and
       wip/u7-t-build-notes-2050.md (d073228f…);
     - ffcheck against the lane head: the 33 ADDED present and the 10 differing are exactly U7's 43.
  2. **Kept evidence, moved first** (the main session, absolute paths, each `mv` its own call), into the main folder's
     `.gate/u7/kept-agent-a9873/`:
     - the worktree's `.gate/u7/` whole (19 MiB): T5 (i)'s run, T5 (ii) runs 1-4 and the shakedown, and t5ii-bin's
       npt-server and npt-host (the binaries runs 3 and 4 used);
     - from its `.gate/netcode-proof-test/`: `t5-dist/`, `dist-bites/` and `failed-steps/` (the dist C1 read and the
       report hashes; the plant batch's step outputs).
     Then one `find <kept> -type f -exec sha256sum {} +` (one command, no loop), its output written by Write into
     `.checkpoints/netcode-proof-test/u7-kept-list.md`. Copy run 3's and run 4's summary.json into the notes (`cp`, each
     its own call; sha256 equal after: run 4's is 100b27d1…, as the report cites), because the report's numbers cite
     them.
  3. **Not kept:** the rest of its `.gate/netcode-proof-test/` (web-test, wasm-step, wasm-target, web-build*, web-check:
     about 149 MiB of build scratch, rebuilt from 6968215 by build.mjs), and `web/node_modules`.
  4. **Clean, then remove.**
     - A plain `git worktree remove` refuses while tracked files are modified or untracked files exist, so U7's builder
       (resumed in a window: a three-line message, one command per call) runs `git add` of the 43 paths, then
       `git merge --ff-only <lane head>`. Its index then equals the new tree's entries, which git keeps. It ends with
       `git status --short`, which should show only its two notes as `??`.
     - The main session then checks their sha256 against the committed copies again and removes the two files by
       absolute path (`rm` of each file; never `-r`).
     - Then `git worktree remove /home/user/Shamble-Frontlines/.claude/worktrees/agent-a9873cdba07a3219e`, plain.
     - **Confirmed, by git's rule as I know it (not run here):** `worktree remove` checks cleanliness with `status
       --porcelain`, which does not list ignored files. So ignored files (its `.gate/`, `web/node_modules`) do not block
       the removal and are deleted with the worktree. That is why step 2 comes first.
     - Its branch ref `worktree-agent-a9873cdba07a3219e` (80b6748, merged) may stay. If it is removed, only by
       `git branch -d`; if the guard asks or refuses, leave it.
- **B. .gate/u7-amend** (detached at c21582d, the two Edits):
  - Its two files read f949f2bf…dc25a and 6e6529a8…00bc now, equal to 56b3e1b's committed content.
  - Undo the two Edits by Edit (absolute paths; the two added lines in each), check that each file's sha256 equals
    c21582d's (d8af72f7…000e and 55604a5f…9f5d), then `git worktree remove /home/user/Shamble-Frontlines/.gate/u7-amend`,
    plain.
  - Nothing in it is kept evidence: security read it, and the content is on the lane.
  - If the plain remove refuses (an untracked file), stop and bring the file's name here.
- **C. cli-target** (433 MiB, the wasm-bindgen CLI's build intermediates):
  - With no cargo or rustc running (`pgrep -x cargo` exit 1): `env CARGO_NET_OFFLINE=true cargo clean --manifest-path
    /home/user/Shamble-Frontlines/spikes/netcode-proof/Cargo.toml --target-dir
    /home/user/Shamble-Frontlines/.gate/u7-0b/cli-target`. With no package named, it removes the whole target folder.
  - Then `test ! -e` on it, and df before and after onto the card.
  - Keep `.gate/u7-0b/cli-root/` (the installed CLI that build.mjs needs for Stop B's two dist builds) and the 0b logs.

## 7. u7-report.md against U7-M-7: complete
- **Checked, each present:**
  - marks 1-8 from run 4, mark 8 under his words with both runs' numbers;
  - mark 9 NOT-RUN with the stand-ins and their limits;
  - the five wording items;
  - the five departures;
  - the carries;
  - the slips;
  - the size, 5,193 of 6,000;
  - C1's line.
- **Spot-checked against the diff, and correct:** the size table's R1-R2 row (325) and the 47 removed (clock.rs 26,
  check.mjs 6, scenario.rs 5, and 10 across six files).
- **Two additions by Edit:**
  - the M3 line cites section 2's ruling (above);
  - the ADV-U7S-6 carry adds section 5's Stop B check.
- **Optional:** one line for CI on 9c2fcf4 (the four run ids, all success).

## Decisions needed
None.

## For Mustafa
Nothing.

## Notes
- 00:56-01:0x: 6968215's parents: dc51f0e only (H0), so it is a plain commit on H0. 9c2fcf4's parent is 6968215. The
  first-parent line H0..9c2fcf4 holds those two. Remote branch 6123e85 = HEAD; main b9aad01 = merge-base(9c2fcf4, main).
- `git diff --stat dc51f0e 6968215 -- spikes/`: 43 files, 5,193 insertions, 47 deletions (the snapshot's). All 43 blob
  ids equal `git hash-object` of the builder's worktree files (5 + 38 read). `--raw`: all 100644, 33 A, 10 M.
- 6968215 touches only spikes/ (43) and .checkpoints/ (u7-merge-plan.md, u7-report.md, and three wip patches D).
  9c2fcf4 touches the card, msg-2026-11-27.md and u7-merge-plan.md. 9c2fcf4..6123e85: 6 commits, one parent each,
  .checkpoints/ only.
- Whole tree: `git diff --quiet b9aad01 9c2fcf4 -- . ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/'`
  exits 0.
- wip/u7-t-build.patch at dc51f0e: 19 web/ files, every one among the 43. wip/u7-t-build-t5i.patch: 23 web/ files,
  every one among the 43.
- Done lines read whole (u7-merge-plan.md:151-236).
- CI by run id: 45906116335 (gate, push), 45906116365 (toolchain, push), 45906121962 (gate, pull_request), 45906121987
  (toolchain, pull_request): head 9c2fcf4, attempt 1, completed, success.
- Logs read: m2b-header.log, light-base.log and c1-judge.log whole; m5-check.log's summary lines; c1-verdicts.json's
  verdict lines. u7-report.md read whole.
- ffcheck (main folder, read only) 80b6748..6123e85: 213 paths; 33 ADDED present (U7's new 33); 61 modified or deleted
  present, 10 differing (U7's 10); hash-object 61 of 61. The worktree's notes hash a65282fa… and d073228f… (= the wip
  copies). .gate/u7 is 19 MiB and .gate/netcode-proof-test 149 MiB; t5ii has runs 1-4 and the shakedown; t5ii-bin holds
  the two binaries.
- .gate/u7-amend's two files are f949f2bf… and 6e6529a8… (= 56b3e1b); cli-target 433 MiB; cli-root separate.
- CHECKPOINTS.md §11 read: KEPT-LOGS covers only `.gate/netcode-proof-test/runs/` raw-log folders and the old cache.
- Not run: no git in any worktree, no cargo, no removal. The `worktree remove` ignored-file behaviour is stated from
  git's rule as I know it, not tried.

Status: COMPLETE 01:01 UTC (sections 1-7; superseded in part by "Item 6, re-ruled" below)

## Item 6, re-ruled (01:0x UTC, 27 Nov): no delete or move to make a git step pass, in any wording
My section 6 A.4 ("removes the two files by absolute path (`rm` of each file …)") is withdrawn: it is the delete-to-pass
form the safety check stopped on 11-25 (launch-0850.md:13-17; the card's choice 7), never retried. Section 6 B's "undo
the two Edits by Edit, then … remove" is withdrawn too, for the same reason: it discards a local change by another door
to make `git worktree remove` pass. The rest of section 6 stands as amended here.

**The ruling: (a), made to work by one notes save first. Fallback (b).**
- **Why (a) as worded is not enough.** The builder's two notes, `.checkpoints/netcode-proof-test/u7-build-rs.md` and
  `u7-t-build.md`, are not on the lane at those paths: `git ls-files` prints neither. The lane holds them only as
  wip/u7-build-rs-notes-1055.md and wip/u7-t-build-notes-2050.md. So ffcheck lists neither: both lie outside the
  fast-forward's range. Staging them would leave two staged additions that the new tree lacks, which git keeps as
  staged (two-tree case 4/5, "keep index"), and the worktree would still not be clean.
- **The form (no file deleted, moved or reverted anywhere):**
  1. **A notes save puts the two notes on the lane at their worktree paths, byte for byte.** The main session runs
     `cp` of each, its own call, from the worktree's absolute path to the main folder's same path. Before the save,
     `sha256sum` must read a65282fa…a34c and d073228f…9c93, equal to the worktree's and the wip copies. The wip copies
     stay, a harmless duplicate in the notes.
  2. **ffcheck,** from the main folder, against the new lane head:
     `node .checkpoints/netcode-proof-test/ffcheck.mjs <worktree> 80b6748 <lane head>`. Expected: 35 ADDED present
     (U7's 33 and the two notes), 10 differing (U7's 10), and nothing else.
  3. **The builder, resumed in a window.** A short message, one command per call, with the two standing rules word for
     word:
     - `git add` of exactly those 45 paths (as ffcheck lists them);
     - then `git merge --ff-only <lane head>`;
     - then `git status --short`, which must print nothing;
     - if git refuses at any step: stop and report its message. Remove, restore or move nothing.
  4. **The main session:** the keep-first copy (below), then `git worktree remove
     /home/user/Shamble-Frontlines/.claude/worktrees/agent-a9873cdba07a3219e`, plain, from the main folder. If it
     refuses, leave the worktree and bring the message here.
- **Fallback (b):** if step 3 or 4 refuses, the worktree stays as it is until PR #40's close, its evidence copied
  first. It costs disk (its .gate/ 168 MiB, web/node_modules and the checkout), and the close's checklist takes it up
  again.
- **Does git's fast-forward accept a staged path equal to the target?**
  - Yes, by git's two-tree merge rules: Documentation/git-read-tree.txt, "Two Tree Merge". With I the index, H the old
    commit and M the new:
    - cases 6 and 7 (I has the path, H does not, M does, I == M): "keep index";
    - cases 18 and 19 (H != M, I == M): "keep index", clean or not.
  - Without the stage, a dirty tracked path is case 21 (I == H, H != M, not clean: "fail"), and an untracked path the
    new tree adds is refused as "would be overwritten". That is the 11-25 refusal.
  - That `git merge --ff-only` applies this table (git's builtin/merge.c fast-forward through unpack_trees'
    two-way merge) is from git's source as I recall it. **Not tried.**
- **The keep-first step may be a copy, and should be.**
  - `cp -a` (each its own call, absolute paths) of the worktree's `.gate/u7/`, `.gate/netcode-proof-test/t5-dist/`,
    `dist-bites/` and `failed-steps/` into `/home/user/Shamble-Frontlines/.gate/u7/kept-agent-a9873/`.
  - Then `diff -rq <original> <copy>` for each (read only; exit 0 = equal).
  - Then one `find /home/user/Shamble-Frontlines/.gate/u7/kept-agent-a9873 -type f -exec sha256sum {} +`, its output
    written by Write into `.checkpoints/netcode-proof-test/u7-kept-list.md`.
  - Run 3's and run 4's summary.json are copied into the notes by `cp` (sha256 equal; run 4's 100b27d1…).
  - The originals stay for the plain `git worktree remove`, which takes the ignored files with the worktree (section 6
    A.4's reading, not tried). Nothing is moved.
- **B, .gate/u7-amend: it stays.** Its two files are byte-equal to the lane's (blobs da7a5596… and 25759d8d…, the same
  as HEAD's). But it is detached at c21582d with those two files modified, so a plain remove refuses. Making it clean
  needs git inside it (stage the two, then fast-forward), and no agent works there: the main session runs no git in
  another worktree. So it stays (41 MiB) until PR #40's close, when the closing checklist names its form. Nothing in
  it is evidence: its content is 56b3e1b's.
- **C, cli-target:** unchanged (section 6 C). `cargo clean --target-dir` removes a build cache by cargo's own command,
  not to make a git step pass.

Status: COMPLETE 01:09 UTC
