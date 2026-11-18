# MAIN-60: take main b763ba3 into the netcode branch (plan, 2026-11-18 23:2x UTC)

## As approved
The advisor's review (advisor-main-60.md, 162 lines, read in full; 23:25-23:40 UTC): **APPROVE WITH CONDITIONS C1-C6**
on this plan's first text (sha256 dc8a1e73…). Applied word for word, they need no new review. In short: C1 H0 read in
step 1 (HEAD and the remote branch equal, after the saver's last save), and step 4 checks HEAD^1 = H0 and `git diff
--quiet H0 HEAD -- spikes/`; C2 the card's resume line before TaskStop, the saver's end confirmed, and a foreground save
with the pre-checks again if its last save left anything; C3 the guard's full pathspec on origin/main...HEAD in the
pre-checks; C4 the merge exits 1 with STATUS.md its one CONFLICT and 49 status lines (UU 1, A 24, M 24), git's message
carries "# Conflicts:" lines (cosmetic), three whole-tree checks after the commit, and the base run held to the light
run's bar; C5 a save that does not push stops (nothing pushed by hand), and no save from step 2 to the push; C6 these
fixes. The conditions' own words govern where this summary is shorter.

## Why
MAIN-52, Mustafa's words (history.md:2055-2057; the reading :2082-2086): take main in when main changes code, CI or
rules. Main moved from 82c605e (MAIN-54's) to b763ba3: #55-#57 (docs), #58 (cs1-types: crates/protocol,
crates/client-endpoint, packages/types, Cargo.toml, Cargo.lock: code) and #60 (kit-hold-ci: .claude/CHECKPOINTS.md's
DEP-HOLD, .claude/scripts/checkpoint.mjs and its tests: rules; .github/workflows/toolchain.yml: CI). The advisor's stop
ruling R4 (advisor-u4f-stop.md:167-190): now, before his answer on SIZE-U4F and before U4f's panel, plan first as
MAIN-54 (main-54-plan.md; its conditions C1-C7, history.md:2533-2541, kept here).

## Facts checked (23:2x-23:4x UTC, read only)
- origin/main b763ba3e5f34c67fc4f875cfbd9266167fb095f3; HEAD = the remote branch = 4ec4a9e at the plan, 20b3b33 at
  the review (cf4eee5 and 20b3b33 are checkpoint-only commits); H0 is read at step 1 (C1); merge base
  82c605e2e8a7f284e6925718368d3fe569f89ae9.
- F15 (the review's): main edits ADR 0004 (the PM-0004-34 row) and ADR 0006 (+3 lines) and none of the 81 marks' rows
  (each hashed on HEAD and on main, equal to marks.json's).
- `git diff --shortstat 82c605e origin/main`: 49 files, +6,835 / -137; 24 added (`--diff-filter=A`), 0 deleted.
- `git diff --name-only 82c605e origin/main -- spikes/ docs/design/`: empty (U4f's 29 files untouched).
- `git diff --stat 82c605e HEAD -- .claude/ CLAUDE.md AGENTS.md .github/ tools/`: empty. So the guard lets a plain
  `git merge origin/main` through in the main folder (routine-guard.mjs:1342-1352, R2; any other merge form asks), and
  the merged kit equals main's. Main changed no `.claude/settings.json`, hook or agent file: the session's settings
  stay as they are.
- `git merge-tree --write-tree --name-only HEAD origin/main`: one conflict, docs/STATUS.md (content); DECISIONS.md,
  REPO_MAP.md and STATUS_ARCHIVE.md merge on their own.
- The conflict: this branch changed one line of STATUS.md since 82c605e, the #40 row (:28, its 2026-11-15 text); main
  kept its #40 row at the 2026-11-14 text and replaced the #52 row with #60's (:29).
- DEP-HOLD (main's CHECKPOINTS.md): main's Cargo.toml, Cargo.lock, crates/protocol/Cargo.toml and
  packages/types/package.json come in equal to main's, so main's checkpoint.mjs holds nothing; U4f's patch has no
  manifest or lockfile.
- refs: outside spikes/, docs/, .checkpoints/ and .md files, only kept-logs.mjs and its test name spikes/ on main (the
  step's standing exception, unchanged since 82c605e; check.mjs:1840-1863).
- Main's checkpoint tests: 33 pass (docs/reports/2026-11-18-kit-hold-ci.md:178).

## Steps (each in its own call, in the simple command forms)
1. (C1) Stop the saver b4p5xbtmj (TaskStop; its last save pushed). Pre-checks, each alone; any difference stops:
   `git status --short` empty; HEAD = the remote branch's; origin/main = b763ba3; the merge base 82c605e; the kit-path
   stat above empty; merge-tree's conflict list = docs/STATUS.md alone. Then the light run on this pre-merge head for
   N0 and M0: `bash .checkpoints/netcode-proof-test/main-run.sh /home/user/Shamble-Frontlines/.gate/main-60/light-base.log
   node check.mjs --only refs,marks,self,redact` (exit 0 expected; its refs count is N0, its redact item count M0).
2. (C2) `git merge origin/main` alone. Expected: "CONFLICT (content): Merge conflict in docs/STATUS.md" and nothing
   else; `git status --short` shows `UU docs/STATUS.md` and main's other files staged.
3. Resolve docs/STATUS.md by Edit: the conflict block becomes this branch's #40 row (its text, the newer of the two)
   then main's #60 row, each word for word from its side; the #52 row stays out, as main took it out; no other line
   changes. Grep the file for `^<<<<<<<|^=======|^>>>>>>>`: none. Then `git add docs/STATUS.md`.
4. (C3) `git commit --no-edit`: git's own merge message, no attribution lines (-m and -F are not used; amend is
   refused); the card save's commit at step 6 carries the MAIN-52 record and both attribution lines. Then, each
   alone: `git rev-parse HEAD^1 HEAD^2` = H0 and b763ba3 (C1); `git diff --stat origin/main HEAD -- .claude/ CLAUDE.md
   AGENTS.md .github/ tools/` empty (the kit equals main's); `git diff --quiet H0 HEAD -- spikes/` exit 0;
   `git diff origin/main HEAD -- docs/STATUS.md` shows the #40 row alone.
5. Checks on the merge commit before any push; outputs under .gate/main-60/, saved by Write where a command prints:
   a. (C5) `node tools/gate.mjs --no-smoke`: GATE PASS (smoke: CI's, on the pushed head).
   b. (C5) `node .claude/scripts/repo-map-check.mjs`: OK.
   c. `node --test .claude/scripts/checkpoint.test.mjs`: 33 pass, 0 fail (main's count, its new script).
   d. (C4) `bash .checkpoints/netcode-proof-test/main-run.sh /home/user/Shamble-Frontlines/.gate/main-60/light.log node
      check.mjs --only refs,marks,self,redact`. Pass bar: exit 0; "check: all 4 steps pass"; the header's HEAD the merge
      commit, its diff and untracked hashes e3b0c442…; refs N0 + 24 tracked files with the exception line; marks 81;
      self 78 of 78; redact 10 of 10 and M0 items, 0 findings. Not check.mjs whole: the spike tree is unchanged (4).
6. (C6) `subscribe_pr_activity` for #40 first (its rules in context for the merge-conflict event of 20:40:46 UTC). Then
   the card save pushes the merge with the card (its Base line: main b763ba3 merged in at the merge commit). The push
   carries non-checkpoint changes, so CI runs: gate, smoke and toolchain green (toolchain now also builds main's root
   workspace and checks packages/types, as on main). The saver restarted (main's checkpoint.mjs) before step 7.
7. The advisor's check of the pushed head, as at 20048a5.

## Stop rules
- Any conflict beyond docs/STATUS.md, any staged change beyond main's 49 files and the #40 row, any check FAIL, or a
  guard ask or refusal: stop; nothing pushed; the card states it; the advisor rules. A merge in progress stays as it is
  (no `git merge --abort`) until that ruling; no save runs from step 2 to the push, the card updated by Edit only
  (C5 (b): `save` builds its commit with commit-tree on HEAD, so a save during the merge would succeed, move the
  branch under MERGE_HEAD, and a foreground save would push it). A card save that prints "NOT pushed" or exits
  non-zero stops; nothing is pushed by hand (C5 (a)).
- A call that would ask Mustafa is named in the chat before it is sent (his words of 2026-11-16 21:50).

## Risks
- Medium: from step 2 to step 4 the main folder holds a merge in progress; the steps run back to back.
- Low: main's kit text lands in the main folder at once (CHECKPOINTS.md, checkpoint.mjs and its tests); no settings,
  hook or agent file changes, so the running session's rules do not move.
- Low (C6): from the merge on, main's checkpoint.mjs holds any push of this lane that changes a spike `Cargo.toml`,
  `Cargo.lock` or `package.json` until security-auditor's answer and `.checkpoints/netcode-proof-test/dependency-check.md`
  exist (DEP-HOLD; on the card's Rules in force). U4f's snapshot has none (its 29 files).

## Change type, tier, size
- A merge of main: main's reviewed and merged text (49 files, +6,835 / -137) and one hand-resolved conflict line (this
  branch's own #40 row). Nothing else is edited by hand. #60's kit and CI lines were reviewed on main (security-auditor,
  infra-engineer, test-engineer: nothing Critical or High, report :140-207), so no new security run (ADV-M54-7's
  reasoning). Reviewers: the advisor (this plan, then the pushed head). Mustafa: nothing to check in Brave.
- Evidence: the pre-checks' outputs, the merge's output, the resolved block, the commit's parents and diffs, gate,
  repo-map, checkpoint tests, light-base.log and light.log, CI on the pushed head, the advisor's head check.

## After it (not this change)
- R3 (iii)'s four hand runs (p3, p9, p16, p20) on builder 3's tree, then his answer on SIZE-U4F; U4f's snapshot
  (74ae26aa) applied onto the new head with `git apply --check` first (its base 07289a4 equals 5b29296b in spikes/
  and docs/, which this merge leaves alone).
