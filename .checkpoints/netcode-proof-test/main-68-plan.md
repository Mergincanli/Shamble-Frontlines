# MAIN-68: take main 2483e9a (#70, change-set 4a "The recording") into the lane: the plan, written 2026-11-26 12:5x UTC
MAIN-52 calls for it: #70 changes code outside spikes/ (crates/ and packages/), so it is not his LIGHT-TAKEIN recipe's
case; his word of 12:21 UTC (history.md:8068-8088): "#70 ... merges today and needs a full take-in". Form: MAIN-66's
plan (main-66-plan.md, as approved with M66-1 to M66-5, advisor-main-66.md) as MAIN-67 applied it (main-67-plan.md),
steps 1-7 whole; only the facts and numbers below differ, and every MAIN-66 value is replaced by its line here. His
interim rule holds in every command: git and gh always start their own command, with no other program in front of them.

## Facts (read 2026-11-26 12:2x to 12:4x UTC, read only)
- `git fetch origin main`: 07d673b..2483e9a741c4e552323c36523831a2f35de4002e, one first-parent merge: #70 (merged
  12:22:01 UTC; its message: "Merged on Mustafa's go of 2026-11-26 in the builder chat, for the head the advisor agent
  approved, after his Advisor's merge check; expected head 420b16b35225c41f1978b177822f2e24386a2f95; gate, smoke and
  toolchain green on it"). The merge base with this branch is 07d673b (MAIN-67's main).
- What it changes (`git diff --name-status 07d673b origin/main`): 39 paths, A 21, M 18, none deleted or renamed. Whole:
  "39 files changed, 5844 insertions(+), 158 deletions(-)"; without docs/STATUS.md: "38 files changed, 5815
  insertions(+), 129 deletions(-)"; STATUS.md's numstat 29 / 29; main's new STATUS.md has 120 lines (`git grep -c ""`).
  Code: crates/devtools (recording, replay, compare, files, scripts, sha256, wasm, two bins, four tests, and
  recordings/ with one binary, first-loop.rec, 122,487 B), crates/room/src/lib.rs, crates/sim (lib, state_hash, world,
  one test), crates/sim-wasm (lib, local_match, one test), packages/e2e (node-names.d.ts, the replay determinism
  spec), packages/sim-worker (recording.ts and its test, two boundary tests). Docs: DECISIONS, REPO_MAP, ROADMAP,
  STATUS, STATUS_ARCHIVE, UNITS and one report (docs/reports/2026-11-25-cs4a-recording.md).
- Nothing under spikes/, .claude/, .github/, tools/, legacy/, a CLAUDE.md or AGENTS.md, .gitignore, or a manifest or
  lock file (no Cargo.toml, Cargo.lock, package.json or package-lock.json: DEP-HOLD free, as its PR says); no
  docs/adr/ path (no `| PM-` row moves).
- Main's CI on 2483e9a, push runs: gate 45818982040 completed, success; toolchain 45818982052 in progress at 12:3x
  (C4 needs it completed with success, read again at step 1).
- `git merge-tree --write-tree --name-only HEAD 2483e9a` (HEAD then 3e174fe): exit 1, one CONFLICT, docs/STATUS.md;
  DECISIONS.md, REPO_MAP.md and STATUS_ARCHIVE.md merge on their own; tree 62d7e303 (not bound: re-derived from H0).
- The conflict is the "In flight" block, merged lines 24-30 (`<<<<<<< HEAD` :24, `=======` :27, `>>>>>>> 2483e9a`
  :30): ours :25 (#40, this branch's row), :26 (ci-after-3a's "none yet" row, main's 07d673b text taken in at
  MAIN-67); main's :28 (#40, main's older text), :29 (the new #70 row). Main moved ci-after-3a's row to its archive
  ("the merged #71's row and waiting bullet and 3a's follow-ups bullet moved to the archive by script").
- refs: `git grep -a -l "spikes/" 2483e9a -- . ':(exclude)spikes/' ':(exclude)docs/' ':(exclude).checkpoints/'
  ':(exclude)*.md'` lists only kept-logs.mjs and its test (§4.9's exception). Main's 21 new paths have no untracked
  file in their way (`git ls-files --others --exclude-standard -- crates packages docs` prints nothing).
- The kit: #70 touches no .claude/ file, so the guard's rules stay main's c1da1bd ones (#69's, as at MAIN-67), and
  after the merge PR #40's diff against main touches no kit file (tier 1 stays).
- The lane: no agent holds it. Builder 2's merge (on his "ready") and U7's merge (after its snapshot read) are also due
  today; the three run one at a time, never overlapping, and nothing of the others is staged during this one.
- Free disk 13,992 MiB at 11:0x UTC (the light runs need 4,096; re-read at step 1).

## The numbers (in place of MAIN-66's)
- C1 (a), (b), 5d: the base run gives every count; 5d equals them word for word, except HEAD, and refs = N0 + 21.
- C1 (c): `git diff --name-only H0 HEAD` equals `git diff --name-only 07d673b origin/main` (39 names); without
  docs/STATUS.md both pairs read "38 files changed, 5815 insertions(+), 129 deletions(-)"; the whole line is expected at
  "39 files changed, 5844 insertions(+), 158 deletions(-)" (STATUS.md's 29 / 29 as main's: the swapped #40 row is the
  same on both sides).
- C2 (a): "beyond main's 39"; the allowance as MAIN-66's: a ` M`, ` D` or `??` line under .checkpoints/netcode-proof-test/
  or the card. Expected: the card, this plan, advisor-main-68.md, the 14:50 window's verdict files, history.md (if he
  writes), msg-2026-11-27.md.
- C2 (b): `git diff --cached --shortstat H0 -- . ':(exclude)docs/STATUS.md'` prints the 38-file line; the anchor is
  re-derived at step 1 from H0 (M66-1 (a)'s form) and binds as there.
- C2 (c), C3: no builder runs on the lane. U8-F1's builder, if it runs, runs no cargo (its brief: no Rust) and works in
  its own worktree; its hand-back is only read until the push.
- C4: after `git fetch origin` in its own call, origin/main must be 2483e9a741c4e552323c36523831a2f35de4002e; main's CI
  is its own push runs: gate 45818982040 (gate and smoke) and toolchain 45818982052, all completed with success. A
  further move stops before step 2 (advisor-main-65.md:113-114).
- C5 (a): MAIN-66's resume line with "MAIN-68", advisor-main-68.md and 2483e9a741c4e552323c36523831a2f35de4002e; no
  builder on the lane.
- C5 (b): the saver running then is stopped by TaskStop at step 1 with its proof; it restarts at step 6.
- C6: keep :25 (ours #40) and :29 (main's #70 row), in that order; drop :26 (ci-after-3a's row) and :28 (main's #40).
  Then the marker grep and `wc -l docs/STATUS.md`: exactly 120, before `git add docs/STATUS.md`; after the add,
  `git diff --cached --numstat origin/main -- docs/STATUS.md` prints "1 1".
- Step 4: two parents, H0 and 2483e9a, git's own message.
- Step 5: a-c and e as MAIN-66's (5e: main's routine-guard tests, any fail stops; #70 changed no guard file, so the
  count is 1,247 again). 5d: the light run as above.
- Step 6: CI on the pushed head: gate (gate, smoke) and toolchain green, push and pull request. Toolchain now runs
  #70's additions (devtools' recording, replay, two-instance and two-process tests, sim-worker's recording and replay
  boundary tests, the e2e replay determinism spec in Chromium); none of them reads spikes/.
- Step 7: the advisor's check of the pushed head by SHA, a new agent's work: the next reset's first 30 minutes
  (19:50 UTC) unless it rides an agent already running. Its toolchain baseline: main's run 45818982052 (job read at
  step 1).
- M65-2's fallbacks stand with these values: refs at 5d = H0's tree count + 21, `git diff --name-only --diff-filter=A
  H0 HEAD` listing main's 21 added names; the staged tally `git diff --cached --name-status H0` = `git diff
  --name-status 07d673b origin/main` (39 paths, A 21, M 18); STATUS.md's 120 with no tolerance.

## Order with the rest of the day
After this plan's check: steps 1-6 when no other merge runs. Builder 2's merge goes first whenever he is at the prompt;
a "ready" from him during steps 2-6 gets "wait about 30 minutes" (steps 2-6 with CI), never a second merge beside this
one. U7's merge takes the next free turn after its snapshot read. Step 7 at 19:50 UTC, with U7's head read if ready.

## Questions for the checker
- Q1: anything in #70 (game code, tests, one binary recording, docs) that needs more than MAIN-66's steps, given that
  nothing of it touches spikes/ or the kit?
- Q2: the light run's marks step builds the spike's judge in the spike's own workspace (spikes/netcode-proof/Cargo.toml);
  #70's crates are the root workspace's, so the light run builds nothing of #70. Anything to add?
- Q3: any order constraint between this take-in, Builder 2's merge (spike files only: `git diff --stat` from their
  merge base e2c0e896 to 0e621dae, outside spikes/ and .checkpoints/, prints nothing) and U7's merge (43 spike files)?
- Q4: may step 7 ride U7's head read at 19:50 UTC?

## Change type, tier, size
A merge of main's reviewed content (#70: five reviewers in two rounds, all APPROVE in round 2; the verifier PASS; the
advisor agent's APPROVE of its head 420b16b; merged on his go in the builder chat after his Advisor's merge check; CI
green on that head, by its PR): no new code of ours; tier 1 after the merge (no kit file differs from main's). The
check: one advisor read of this plan, then step 7's read of the pushed head.
