# Advisor: MAIN-60 plan review (take main b763ba3 into PR #40's branch)
2026-11-18, 23:25-23:40 UTC. Read only. Acting by main's advisor.md (`git show origin/main:.claude/agents/advisor.md`,
335 lines). Reviewed: `.checkpoints/netcode-proof-test/main-60-plan.md` (83 lines, sha256 dc8a1e73a948cd70…), against
R4 of my stop ruling (advisor-u4f-stop.md:167-190) and MAIN-54's conditions C1-C7 and build record (history.md:2533-2564).

## Verdict: APPROVE WITH CONDITIONS C1-C6
The take-in is due under his MAIN-52 rule (history.md:2055-2057: "Take main in when main changes code, CI or rules"):
#58 adds code, #60 changes CI and rules. The plan follows MAIN-54's approved model. Every fact it states that I could
check holds, with one exception: the HEAD it names is out of date, and step 4's parent check is pinned to it (C1).
Nothing in it changes a pass mark, the judge, marks.json or main: I hashed all 81 marks' rows on main and on HEAD, and
each matches marks.json (F15). The conditions below are exact. Applied word for word, they need no new plan review.
Step 7 (my check of the pushed head) stays as planned.

## Conditions (each can be met by number)
- **C1 (ADV-M60-1, Medium): use H0, not 4ec4a9e.** HEAD has moved since the plan was written. It is at 20b3b33 now
  (cf4eee5 and 20b3b33 are checkpoint-only commits), and the saver's last save will move it again. Step 1: once the
  saver has ended (C2), read `git rev-parse HEAD` and `git rev-parse origin/claude/netcode-proof-test-plan-hcky5g`
  in separate calls. They must be equal; that SHA is H0, named in the chat. Step 4 then reads
  `git rev-parse HEAD^1 HEAD^2` = H0 and b763ba3, and `git diff --quiet H0 HEAD -- spikes/` exit 0. As written,
  step 4 would fail its own check against 4ec4a9e and stop with the merge committed and unpushed, the one state the
  guard gives no way back from (reset and amend are refused).
- **C2 (ADV-M60-2, Low): the saver's stop and a resume line.**
  (a) Before TaskStop, add one line to the card by Edit, so the saver's last save carries it: "MAIN-60 approved
  (advisor-main-60.md), running steps 1-7. If `git rev-parse -q --verify MERGE_HEAD` prints b763ba3, the merge is in
  progress: no save, no `git merge --abort`, no reset. If `git diff --name-only --diff-filter=U` names
  docs/STATUS.md, resume at step 3; otherwise resume at step 4. If HEAD^2 is b763ba3 and HEAD is not on the remote
  yet, resume at step 5. Anything else: the advisor rules."
  (b) After TaskStop, confirm before step 2 that the saver has ended: its output ends with "watch stopped:
  terminated", or, if it doesn't, the reflog shows the last save (ADV-M54H-2), and no `checkpoint.mjs watch`
  process is left. A saver still running during the merge would save on top of the pre-merge HEAD and push (F13).
  (c) If the last save left notes unsaved or unpushed, run one foreground `save` (it pushes), then start the
  pre-checks again from the beginning.
- **C3 (ADV-M60-3, Low): check what the guard itself checks.** Keep the plan's `82c605e HEAD` stat, and add MAIN-54's
  C1 pathspec, which is what the guard compares (protectedRel):
  `git diff --name-only --no-renames origin/main...HEAD -- .claude .github tools/gate.mjs tools/smoke.mjs tools/battery.js tools/gate-globals.json tools/package.json tools/package-lock.json tools/npm-shrinkwrap.json ':(glob)**/CLAUDE.md' ':(glob)**/CLAUDE.local.md' ':(glob)**/AGENTS.md' ':(glob)**/.npmrc' ':(glob)**/.mcp.json'`
  must print nothing. I ran it: it prints nothing. The same pathspec on `HEAD...origin/main` prints main's four kit
  and CI files, so it does catch them. The plan's pathspec misses a nested CLAUDE.md, AGENTS.md, .npmrc or .mcp.json.
  If one existed, step 2 would ask Mustafa without the chat naming the ask first.
- **C4 (ADV-M60-4, Low): the expected outputs, and one check over the whole tree.**
  Step 2: the merge exits 1, prints "CONFLICT (content): Merge conflict in docs/STATUS.md" as its only CONFLICT
  line, then "Automatic merge failed…". That is expected, not a stop. `git status --short` must then show 49 lines:
  `UU docs/STATUS.md`, 24 lines `A ` and 24 lines `M `, nothing untracked or of any other kind.
  Step 4: git's message will read "Merge remote-tracking branch 'origin/main' into
  claude/netcode-proof-test-plan-hcky5g", followed by "# Conflicts:" and "#\tdocs/STATUS.md". With --no-edit, git
  keeps comment lines, as the repository's merges 1022f4d and 41266a7 show. This is cosmetic and not a stop.
  Add these checks, each in its own call:
  - `git diff --quiet origin/main HEAD -- . ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/'` exits 0
    (code, kit, CI, legacy and tools all equal main's);
  - `git diff --name-only origin/main HEAD -- docs/` prints exactly the branch's six docs: DECISIONS.md,
    REPO_MAP.md, STATUS.md, STATUS_ARCHIVE.md, design/netcode-proof-messages.md and design/ws-reconnect.md;
  - `git diff --name-only H0 HEAD` prints exactly main's 49 names.

  I ran the first two against merge-tree's predicted tree: exit 0, and the six names.
  Steps 1 and 5d: the light run's marks line must read "81 marks read and checked …; rows in this branch's
  docs/adr/: all unchanged". The base run in step 1 must meet the same bar except N0 and M0 ("check: all 4 steps
  pass", marks 81, self 78 of 78, redact 10 of 10, 0 findings). A FAIL on the base run stops before the merge.
- **C5 (ADV-M60-5, Low): two more stop rules.**
  (a) If step 6's card save prints "NOT pushed" or exits non-zero (the DEP-HOLD or anything else), stop. Push
  nothing by hand: CHECKPOINTS.md §3 says "push nothing by hand: a plain `git push` is not held". The advisor rules.
  (b) From step 2 until the push, update the card by Edit only, never with a save. Replace the plan's reason ("a
  partial commit fails") with the real one: `save` builds its commit with commit-tree on HEAD, so a save during the
  merge would succeed. It would move the branch under MERGE_HEAD, and a foreground save would push that commit (F13).
- **C6 (ADV-M60-6, Low): fixes to the plan's text, with an "As approved" header as at MAIN-54.**
  - The facts' HEAD line: 20b3b33 at the review, not 4ec4a9e, and the times when the facts were checked.
  - Add F15: main edits ADR 0004 (the PM-0004-34 row) and ADR 0006 (+3 lines), and none of the 81 marks' rows.
  - Add to the Risks: from the merge on, main's checkpoint.mjs holds any push of this lane that changes a spike
    `Cargo.toml`, `Cargo.lock` or `package.json` until security-auditor's answer and
    `.checkpoints/netcode-proof-test/dependency-check.md` exist (DEP-HOLD), written on the card's Rules in force.
    U4f's snapshot has no such file (I checked: its 29 files, none a manifest or lockfile, all under spikes/ or
    docs/design/).

## Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-M60-1 | Medium | Step 4's parent check and spike diff are pinned to 4ec4a9e; HEAD is 20b3b33 now and moves again at the saver's last save (F2). Followed literally, the check fails after the merge is committed. | C1 |
| ADV-M60-2 | Low | Nothing confirms the saver has ended, there is no fix if its last save does not push, and the card gets no resume line for a spend-limit cut between steps 2 and 5 (two cuts today). | C2 |
| ADV-M60-3 | Low | The pre-check's pathspec is narrower than the guard's (C1 of MAIN-54 used the full set). | C3 |
| ADV-M60-4 | Low | The plan doesn't name the expected merge exit code (1) or the message's "# Conflicts" lines; it has no check over the whole tree; the base run has no stated bar. | C4 |
| ADV-M60-5 | Low | No stop rule for a save that does not push; the reason "a partial commit fails" is wrong (F13). | C5 |
| ADV-M60-6 | Low | Stale facts and times; the marks-row check (F15) and DEP-HOLD's reach over the lane's later work are not stated. | C6 |
| ADV-M60-7 | Low | After the merge, STATUS's #40 row still says "main 53272a1 merged in" (this branch's row, kept word for word, which is right for a merge). | PR #40's closing docs pass; no change now |
| ADV-M60-8 | Low | The saver's commits (cf4eee5, 20b3b33) carry one attribution line, while CHECKPOINTS.md §3 asks for one `--trailer` per line the session's instructions give. 20048a5's card save carried both. | Restart the saver with both if that form needs no ask; not a condition |
| ADV-M54-8 | Low (open) | CLAUDE.md's Never line still lists no plain `git merge origin/main`, which the guard allows (R2). | The kit follow-up after the freeze, as ruled |

## Facts checked (one line each, read-only git and file reads)
- F1 origin/main = b763ba3e5f34c67fc4f875cfbd9266167fb095f3; merge base 82c605e2e8a7f284e6925718368d3fe569f89ae9.
  Main's first-parent merges: #55, #56 and #57 (docs only), #58 (Cargo.toml, Cargo.lock, 7 crates files,
  14 packages files, 6 docs), #60 (3 .claude, 1 .github, 6 docs). Matches the plan.
- F2 HEAD is not 4ec4a9e: cf4eee5 (23:25:11, plan and card) and 20b3b33 (23:25:54, this review's first block), both
  [skip ci], .checkpoints/ only. origin branch = 20b3b33. PR #40: head 20b3b33, draft, mergeable_state dirty.
- F3 `git diff --shortstat 82c605e origin/main`: 49 files, +6,835/-137; 24 added, 0 deleted. Matches.
- F4 `git diff --name-only 82c605e origin/main -- spikes/ docs/design/`: empty. Matches.
- F5 `git diff --stat 82c605e HEAD -- .claude/ CLAUDE.md AGENTS.md .github/ tools/`: empty. Matches. The guard's
  wider set (C3) on origin/main...HEAD (1,292 names) also gives nothing.
- F6 The guard (routine-guard.mjs:1307-1363, mainFolderMove) handles `git merge origin/main` as follows: one word, in
  its own call, no earlier git command in the line, origin/main is the real ref, protectedHit(origin/main...HEAD) is
  null, so it returns null (no decision). Any other merge form asks (a subagent is refused). In settings.json,
  `git merge` is in no allow or deny list, so it takes the normal permission flow; at 3f2fa21 it ran with no prompt
  (history.md:2546-2547). `git commit *` and `git add *` are allowed, and `--amend` is denied. Commit rules
  (:319-340): `--amend` is commit's only dangerous long option; `--no-edit` counts as exact (`--no-` plus `--edit`,
  :373); the other commit checks are for the night shift only (:1067, :1118). The plan's reading holds.
- F7 merge-tree (now): one conflict, docs/STATUS.md (content); DECISIONS, REPO_MAP and STATUS_ARCHIVE merge cleanly.
  The conflict block is at merged lines 28-34: HEAD's side has #40 (the 2026-11-15 text) and #52; main's side has #40
  (the base text) and #60.
- F8 The planned resolution (HEAD's #40, then main's #60), diffed against origin/main's STATUS.md: only line 28 (the
  #40 row) differs. Neither side has a line that starts like a conflict marker, so the plan's grep cannot hit a real
  line.
- F9 The automatic merges: DECISIONS and STATUS_ARCHIVE have the same patch-id against main as the branch's own patch
  against the base; REPO_MAP has the same +/- lines, with different context. Outside spikes/ and .checkpoints/, the
  merged tree differs from main only in the six docs the branch owns.
- F10 git config: no merge.conflictStyle, no rerere, no editor; commit.gpgsign true (ssh); no local ref named
  origin/main.
- F11 1022f4d and 41266a7 (conflicted merges with git's own message) carry "# Conflicts:" and "#\t<path>" lines.
- F12 DEP-HOLD (main's checkpoint.mjs:115-141): a file is held when it is a manifest or lockfile changed between
  refs/remotes/origin/<branch> and the save commit, unless its blob equals origin/main's. Main's four changed
  manifests arrive equal to main's, and the merge changes no spike manifest, so nothing is held. This needs
  origin/main still at b763ba3 at the save.
- F13 The saver (PID 424, started 22:39) runs the branch's old checkpoint.mjs with one `--trailer`. On SIGTERM it makes
  a last save. A background save pushes only when nothing outside .checkpoints/ is unpushed. `save` builds its commit
  with commit-tree -p HEAD and update-ref, so during a merge in progress it would succeed and move the branch.
- F14 checkpoint.test.mjs on main has 33 test() calls (27 on HEAD); the report (:178) says 33 pass. Its fixtures turn
  commit signing off. Matches.
- F15 marks: main changes docs/adr/0004 (the PM-0004-34 row) and 0006 (+3 lines). I hash-checked all 81 marks' rows
  on HEAD and on origin/main (63 in ADR 0001, 1 in 0004, 17 in 0006): each appears exactly once and each hash equals
  marks.json's. PM-0001 rows: 63 on both. The judge and marks.json are under spikes/, which the merge does not touch.
- F16 refs (check.mjs:1847-1866) skips spikes/, docs/, .checkpoints/ and *.md files. On main only kept-logs.mjs and
  its test name spikes/, and only as spikes/netcode-proof/results/ (no other hits). None of main's 24 new files
  names it.
- F17 The counts: HEAD tracks 1,473 files now; merge-tree's tree (on cf4eee5's 1,472) has 1,496, so +24. N0 + 24
  holds while nothing else enters the index between the two runs.
- F18 main-run.sh writes a header (HEAD, the spike folder's diff and untracked hashes) and stops below 4 GB free
  (exit 3). Free space now: 5,079 MiB.
- F19 self (check.test.mjs) reads ADR rows from fixtures only; redact reads spike results only. Main changes
  neither. Spike tracked files: 307 (so M0 is read, not taken as 305).
- F20 CI on main b763ba3: gate, smoke and toolchain all success (two runs each, 20:31-20:38 UTC). toolchain.yml now
  has cache-mode none, the SHAMBLE_WRITE_TYPES compare and strict tsc on packages/types. The root workspace's
  members are crates/* only (the spike keeps its own workspace).
- F21 repo-map-check run by hand on merge-tree's tree: no entry missing, none stale (149 entries, 110 required
  paths).
- F22 main's SPIKE_PLAN and ADR edits: dated pointers recording his decisions of 2026-11-14 and 2026-11-17. "Slice 3,
  online, still waits for the netcode proof test's result" is unchanged; no netcode mark is touched.
- F23 The citations check out: R4 at advisor-u4f-stop.md:167-190; C1-C7 at history.md:2533-2541; the build record at
  :2542-2564; MAIN-52 his words at :2055-2057, the session's reading at :2082-2086.

## What I did not check
- I did not run the merge, the gate, the light runs, checkpoint.test.mjs or repo-map-check in the main folder (the
  brief is read-only). repo-map was checked against merge-tree's tree instead; the gate's legacy inputs are
  unchanged by main, and CI is green on main.
- I did not test `git commit --no-edit`'s message in a scratch repository (no commits, per the brief). F11 is inferred
  from git's history in this repository.
- I did not check how TaskStop signals the saver; C2 (b) checks the result instead.
- Whether the permission flow asks on `git merge origin/main` comes from the precedent at 3f2fa21, not from a test
  of my own. The guard's part is read from the code (F6).
- CI on the pushed head: that is step 7.

## Decisions needed
None. MAIN-52 is his standing rule; the conflict is resolved by the plan's mechanical rule; no pass mark, judge,
marks.json, money or main is touched.

## For Mustafa
Nothing.
