# MAIN-61: take main f5030a3 into the netcode branch (plan, 2026-11-19 15:2x UTC)

## Why
MAIN-52, Mustafa's words (history.md:2055-2057; the reading :2082-2086; TEST-FIRST's "main taken in when it changes
code, CI or rules"): main moved from b763ba3 (MAIN-60's) to f5030a3, PR #61 (change-set 2, the first page: v2 code in
crates/ and packages/, the root Cargo and npm manifests, docs). GitHub reports PR #40 un-mergeable since that push
(the merge-conflict event of 15:11:38 UTC). The advisor's round-1 ruling makes this a precondition of U4f's merge
(advisor-u4f-r1.md:327-328, M2: "If main has meanwhile changed code, CI or rules, take main in first (MAIN-52, plan
first) and re-check"). Plan first, on MAIN-60's form (main-60-plan.md) with its conditions C1-C6 (advisor-main-60.md)
built in, and its head check (advisor-main-60-head.md) after the push.

## Facts checked (15:1x-15:2x UTC, read only)
- origin/main f5030a33185b1ea1b15329174e82823f0215f993; merge base b763ba3e5f34c67fc4f875cfbd9266167fb095f3 (MAIN-60's
  second parent); HEAD = the remote branch at the plan (b25b96a and the saver's later checkpoint-only commits). H0 is
  read at step 1 (C1).
- `git diff --name-status b763ba3 origin/main`: 78 files, +5,621 / -106: 53 added, 21 modified, 3 deleted
  (packages/client/src/index.ts, its test, packages/sim-worker/src/index.test.ts), 1 renamed at 100 %
  (packages/e2e/tests/stub.spec.ts to tests/node/). So the tracked-file count after the merge is N0 + 50.
- `git diff --stat b763ba3 origin/main -- .claude/ CLAUDE.md AGENTS.md .github/ tools/ spikes/ docs/design/`: empty.
  Main changed no kit, CI, spike or design file: no settings, hook or agent file moves, and U4f's 29 files and builder
  4's worktree are untouched.
- The guard's full pathspec (MAIN-60's C3): `git diff --name-only --no-renames origin/main...HEAD -- .claude .github
  tools/gate.mjs tools/smoke.mjs tools/battery.js tools/gate-globals.json tools/package.json tools/package-lock.json
  tools/npm-shrinkwrap.json ':(glob)**/CLAUDE.md' ':(glob)**/CLAUDE.local.md' ':(glob)**/AGENTS.md'
  ':(glob)**/.npmrc' ':(glob)**/.mcp.json'` prints nothing, so a plain `git merge origin/main` in its own call passes.
- `git merge-tree --write-tree --name-only HEAD origin/main`: one conflict, docs/STATUS.md (content); DECISIONS.md,
  REPO_MAP.md and STATUS_ARCHIVE.md merge on their own (this branch changed them since b763ba3: `git diff --stat
  b763ba3 HEAD -- docs/`).
- The conflict: since b763ba3 this branch changed one STATUS.md line, the #40 row (its 2026-11-15 text, kept at
  MAIN-60); main kept its #40 row at the 2026-11-14 text, replaced the #60 row with #61's and changed five lines
  elsewhere (the date line, Last merged, Next item 2, Waiting on Mustafa, the follow-up lines).
- DEP-HOLD: `git diff --stat b763ba3 HEAD -- Cargo.toml Cargo.lock package.json package-lock.json crates packages
  .gitignore` is empty, so every manifest and lockfile comes in equal to main's and the hold holds nothing.
- refs: `git grep -n -I "spikes/" origin/main -- crates packages Cargo.toml Cargo.lock package.json package-lock.json
  .gitignore` finds nothing: main's new code names no spike path (the step's one exception stays kept-logs.mjs).
- Main's checkpoint tests: 33 (no .claude/ change since #60).
- Builder 4 runs in its own worktree and writes its notes into the main folder's
  .checkpoints/netcode-proof-test/u4f-build.md. That file, and only .checkpoints/netcode-proof-test/ files, may show as
  unstaged; no step stages them, and the merge touches none of them.

## Steps (each in its own call, in the simple command forms)
1. (C1, C2) Write the card's resume line for this take-in, by Edit. Stop the saver (TaskStop), then confirm it ended and
   that its last save is pushed (HEAD = the remote branch). If anything is left unsaved, run a foreground save, then
   repeat the pre-checks. Read H0 = HEAD. Pre-checks, each alone; any difference stops:
   - `git status --short` shows nothing outside .checkpoints/netcode-proof-test/;
   - origin/main = f5030a3, and the merge base is b763ba3;
   - the guard's pathspec above prints nothing;
   - merge-tree's conflict list is docs/STATUS.md alone.
   Then the light run on H0 for N0 and M0: `bash .checkpoints/netcode-proof-test/main-run.sh
   /home/user/Shamble-Frontlines/.gate/main-61/light-base.log node check.mjs --only refs,marks,self,redact`. Exit 0 is
   expected; its refs count is N0 and its redact item count M0. It waits for builder 4's shared lock, as the runner does.
2. (C4) `git merge origin/main`, alone. Expected: exit 1; "CONFLICT (content): Merge conflict in docs/STATUS.md" and no
   other CONFLICT line. `git status --short`: `UU docs/STATUS.md`, with main's other 77 paths staged (A 53, M 20, D 3,
   R 1). Nothing else, beyond unstaged .checkpoints/netcode-proof-test/ lines.
3. Resolve docs/STATUS.md by Edit. Main's side of every hunk is taken, except the conflict block. The block becomes this
   branch's #40 row (its text, the newer of the two), then main's #61 row, each word for word from its side; the #60
   row stays out, as main took it out. No other line changes. Grep for `^<<<<<<<|^=======|^>>>>>>>`: none. Then
   `git add docs/STATUS.md`.
4. (C3 of MAIN-60's form) `git commit --no-edit`: git's own merge message, with its "# Conflicts:" lines (cosmetic).
   It carries no attribution lines (-m and -F are not used; amend is refused); the card save at step 6 carries the
   MAIN-61 record and both attribution lines. Then, each alone:
   - `git rev-parse HEAD^1 HEAD^2` = H0 and f5030a3;
   - `git diff --stat origin/main HEAD -- .claude/ CLAUDE.md AGENTS.md .github/ tools/` is empty (the kit equals
     main's);
   - `git diff --quiet H0 HEAD -- spikes/` exits 0;
   - `git diff origin/main HEAD -- docs/STATUS.md` shows the #40 row alone;
   - `git diff --stat origin/main HEAD -- crates packages Cargo.toml Cargo.lock package.json package-lock.json` is
     empty (the code and manifests equal main's).
5. Checks on the merge commit before any push; outputs under .gate/main-61/, saved by Write where a command prints:
   a. `node tools/gate.mjs --no-smoke`: GATE PASS (smoke is CI's job, on the pushed head).
   b. `node .claude/scripts/repo-map-check.mjs`: OK.
   c. `node --test .claude/scripts/checkpoint.test.mjs`: 33 pass, 0 fail.
   d. `bash .checkpoints/netcode-proof-test/main-run.sh /home/user/Shamble-Frontlines/.gate/main-61/light.log node
      check.mjs --only refs,marks,self,redact`. Pass bar:
      - exit 0, and "check: all 4 steps pass";
      - the header's HEAD is the merge commit, and its diff and untracked hashes are e3b0c442…;
      - refs N0 + 50 tracked files, with the exception line;
      - marks 81;
      - self 78 of 78;
      - redact 10 of 10 and M0 items, 0 findings.
      Not check.mjs whole: the spike tree is unchanged (step 4).
6. Read main's CI on f5030a3 (gate, smoke, toolchain). If one is red on main, it is not this PR's failure: stop, and
   the advisor rules before the push. Then the card save pushes the merge with the card (its Base line: main f5030a3
   merged in at the merge commit). The push carries non-checkpoint changes, so CI runs: gate, smoke and toolchain
   green (toolchain also builds main's root workspace and checks packages/, as on main). Restart the saver with both
   attribution lines as trailers.
7. The advisor's check of the pushed head, as MAIN-60's (advisor-main-60-head.md).

## Stop rules
- Any of these stops the take-in: a conflict beyond docs/STATUS.md; a staged change beyond main's 78 paths and the #40
  row; any check FAIL; a guard ask or refusal. Then nothing is pushed, the card states it, and the advisor rules.
- A merge in progress stays as it is (no `git merge --abort`) until that ruling.
- No save runs from step 2 to the push, and the card is updated by Edit only. `save` builds its commit with
  commit-tree on HEAD, so a save during the merge would succeed, move the branch under MERGE_HEAD, and a foreground
  save would push it.
- A card save that prints "NOT pushed" or exits non-zero stops; nothing is pushed by hand (MAIN-60's C5).
- A call that would ask Mustafa is named in the chat before it is sent (his words of 2026-11-16 21:50).

## Risks
- Medium: from step 2 to step 4 the main folder holds a merge in progress; the steps run back to back. Builder 4 keeps
  writing its notes then; they stay unstaged and are saved by step 6's card save.
- Low: the light runs wait on builder 4's cargo runs (main-run.sh's lock and its idle wait, at most 60 minutes).
- Low: from the merge on, the lane holds main's v2 workspace. The spike's workspace and check.mjs do not build it, and
  CI's toolchain job does, as on main.

## Change type, tier, size
- A merge of main: main's reviewed and merged text (78 files, +5,621 / -106) and one hand-resolved conflict block (this
  branch's own #40 row kept, main's #61 row taken). Nothing else is edited by hand.
- #61 was reviewed on main: two rounds with seven reviewers, the verifier and the advisor agent (STATUS's #61 row), so
  this take-in adds no new review.
- Reviewers: the advisor (this plan, then the pushed head). Mustafa: nothing to check in Brave.
- Evidence: the pre-checks' outputs, the merge's output, the resolved block, the commit's parents and diffs, the gate,
  repo-map, checkpoint tests, light-base.log and light.log, CI on the pushed head, and the advisor's head check.

## After it (not this change)
- Builder 4's fix pass goes on. The advisor checks it, then U4f merges on M1-M9; M2 re-checks that spikes/netcode-proof
  and docs/design on the new head still equal 07289a4's.
