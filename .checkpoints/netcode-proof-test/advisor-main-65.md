# Advisor: MAIN-65 plan check (main 28254a7 into PR #40's branch)
2026-11-24, 22:56-23:2x UTC. Read only, effort xhigh. Acting by main's advisor.md (`git show
origin/main:.claude/agents/advisor.md`, read whole). Plan: main-65-plan.md (107 lines, sha256 fd6e8c4c…952eb2 on disk,
equal to the brief's). Binding form: main-64-plan.md; advisor-main-64.md M64-1 to M64-4 (:15-48); main-61-plan.md;
advisor-main-61.md C1-C6 (:19-104); advisor-main-64-head.md. Notes of every check: section 7.

## 1. Verdict: APPROVE WITH CONDITIONS M65-1 to M65-5
The take-in is due under MAIN-52 (history.md:2055-2057: "Take main in when main changes code, CI or rules") and is on
R1's path (LEAN-MODE item 1, history.md:7862). #65 changes code in crates/ and packages/, two manifests, both lockfiles
and .gitignore. Every fact the plan states holds when I re-run it, on today's head 1fb3a1b as on the plan's 40a3481
(the three commits between touch .checkpoints/ only). The STATUS.md rule is right.
One line is wrong: the merge-window line (plan :96-97) allows a save after step 4's commit. A save pushes the merge
before step 5's checks (M65-1). The rest binds this take-in's numbers, as M64-1 did for MAIN-64. Applied word for word,
the conditions need no new review, and step 7 stays. No Critical or High. Risk tier 1.

## 2. Conditions (each with where it is met)
- **M65-1 (Medium): no save from step 2 to step 6's card save.** Read plan :96-97 as main-61-plan.md:110-112 and
  C2 (c): no save, commit or push by hand from step 2 until step 6's card save. Why: `save` pushes every local commit
  on the branch (checkpoint.mjs:2-6). A save after step 4 would publish the merge before 5a's gate and 5d's light run.
  What arrives in that window is read, and gets no save until step 6:
  - the round-2 ruling advisor's handback (a0031a451c8991809): read only; no save, apply or run on it;
  - the check-in trig_01EKqNqwZrQTVJ6caAyRrug5 and the day-message trigger trig_016pV7v16ktVwMuzygofJhjJ: the card
    and msg-2026-11-25.md by Edit only;
  - the web Stop hook's ask: ignored.
  Met: steps 2-6. Step 6's card save takes all of them.
- **M65-2 (Medium): the numbers C1-C6 and M64-1 bind with, for this take-in** (in place of MAIN-64's):
  - C1 (a), (b), 5d: the base run gives every count (refs N0, the full marks line, self S0 of S0, redact R0 of R0 with
    M0 items, 0 findings). 5d equals them word for word, except HEAD, and refs = N0 + 23.
  - C1 (c): docs/ prints the same six names. `git diff --name-only H0 HEAD` equals `git diff --name-only 1ec6f6c
    origin/main` (64 names). Without docs/STATUS.md, both pairs' shortstat reads "63 files changed, 4644
    insertions(+), 284 deletions(-)". The whole line is expected at "64 files changed, 4677 insertions(+), 317
    deletions(-)".
  - C2 (a): "beyond main's 64". The allowance covers the card, advisor-u6c-r2.md, this file and msg-2026-11-25.md, as
    ` M` or `??` lines only.
  - C2 (b): after `git add docs/STATUS.md`, each alone:
    - `git diff --name-only --diff-filter=U` prints nothing;
    - `git diff --cached --shortstat H0 -- . ':(exclude)docs/STATUS.md'` prints the 63-file line;
    - `git diff --cached --name-only H0 -- .checkpoints/ spikes/` prints nothing;
    - the anchor in section 3 exits 0.
  - C2 (c), C3: "builder 4" is the round-2 ruling's advisor a0031a451c8991809. It reads only and takes no lane lock, so
    the light runs need not wait for it (main-run.sh still takes the lock: expect "idle after 0 s").
  - C4: after `git fetch origin` in its own call, origin/main must be 28254a73ab226379373e380eeee64336e58ef916.
    Main's CI is main's own push runs: gate run 45503161434 (gate and smoke) and toolchain run 45503161333, three check
    runs, all completed with success. A read by SHA shows six. The other three come from a push of the same commit to
    #65's branch claude/loving-hopper-6p0mg2 (runs 45503920083 and 45503920426): extra, not main's.
  - C5 (a): the resume line, word for word: "MAIN-65 approved (advisor-main-65.md), running steps 1-7. If `git
    rev-parse -q --verify MERGE_HEAD` prints 28254a73ab226379373e380eeee64336e58ef916, the merge is in progress: no
    save, no `git merge --abort`, no reset. If `git diff --name-only --diff-filter=U` names docs/STATUS.md, resume at
    step 3; otherwise resume at step 3's checks before the commit (C2 (b)), then step 4. If HEAD^2 is 28254a7 and HEAD
    is not on the remote yet, resume at step 5. The ruling advisor's notes (advisor-u6c-r2.md), advisor-main-65.md and
    this card may show unstaged: leave them; step 6's card save takes them. Anything else: the advisor rules."
  - C5 (b): M64-3's form. The saver ended at 19:47 on its own, so the proof is its notice ("watch stopped: time is
    up") plus `pgrep -af "checkpoint.mjs watch"` exit 1, with no TaskStop. pgrep was exit 1 at 22:5x.
  - C6: step 3's block is merged lines 24-31 (`<<<<<<< HEAD` :24, `=======` :28, `>>>>>>> origin/main` :31).
    - Keep :25 (ours #40, the 2026-11-15 text) and :30 (main's "none yet" row, change-set 3a).
    - Drop :26 (#64), :27 (#63, #62) and :29 (main's #40, the 2026-11-14 text).
    - Step 5b's expected line is "repo map OK: 186 entries cover 110 required paths". This is worked out, not run, so
      the bar is "repo map OK" (section 3).
  - Step 7's toolchain baseline (M64-4's fourth bullet): main's run 45503161333, job 138831415745.
  Met: steps 1-7, by these values.
- **M65-3 (Low): the card stays at or under 200 lines (M64-2).** It is at 200 now, so the resume line replaces lines
  rather than adding them. The main session chooses which; the MAIN-64 DONE line (:157) can simply go, since its
  record is already in history.md. M64-2's "step details go to history.md" is superseded by LEAN-MODE item 5 and its
  reading (history.md:7866, :7875-7877: history.md takes his words only; notes go on the card and the day's message).
  `wc -l` on the card before step 6's save shows 200 or fewer. After the push, one done line remains (merge SHA,
  pushed SHA, CI, and the bar numbers of section 3). Met: steps 1 and 6.
- **M65-4 (Low): text.**
  - Name the risk tier: 1 (a spike PR). After the merge, .github/ and .gitignore equal main's; C1 (c)'s first check
    proves it.
  - Step 3: run `wc -l docs/STATUS.md` (120) before `git add`, as MAIN-64 in fact did (history.md:7840-7842). After
    the add, `git diff --cached --numstat origin/main -- docs/STATUS.md` prints "1 1" (not before the add: the path is
    unmerged then). A slip found after the add is fixed by Edit and added again, before the commit.
  - Keep `.gate/main-65/` (light-base.log, light.log, step5-outputs.txt) until step 7 is DONE. The bar lines go word
    for word on the card's done line and in the day's message (LEAN-MODE item 5), not in history.md.
  - The saver's restart moves from step 6 to U6c's merge (plan :80): accepted. Until then, a hand save follows each
    step from step 6 on, and comes at least every 30 minutes while an agent runs (CLAUDE.md, Checkpoints item 1).
  Met: the plan's text, steps 3, 6 and 7.
- **M65-5 (Low): U6c's order.** U6c's M3-M9 may start once step 6's CI is green, on the terms in section 4. Step 7
  reads MAIN-65 by its SHAs as section 4 says. Met: after step 6, and at 03:50 UTC.

## 3. Fallbacks for the numbers the plan binds (so a small mismatch needs no new ruling)
- **N0** (the base run's refs count): not bound; record what the run prints. 1,825 files are tracked now, not 1,822:
  three notes files were tracked after 40a3481. Step 1's save may add more. If N0 differs from
  `git ls-tree -r --name-only H0 | wc -l` (the run read a different index), use H0's tree count as N0 and record why.
- **refs at 5d = N0 + 23:** the bar is really the merge commit's own count. `git ls-files | wc -l` equals
  `git ls-tree -r --name-only HEAD | wc -l`, which is H0's tree count + 23, and `git diff --name-only --diff-filter=A
  H0 HEAD` lists main's 23 names. If that holds, record it and go on. Any other path in the index: stop (C2).
- **The 63-file line:** one anchor covers it. Before the commit:
  `git diff --cached --quiet 9475d37ae94cbaa24ea6cfa5bad5f692e0f881cc -- . ':(exclude).checkpoints/' ':(exclude)docs/STATUS.md'`
  exits 0. After the commit, the same command with `HEAD` in place of `--cached` exits 0. (It accepts the tree: it
  exits 1 today, before the merge. 5f57b5ae and 81aa7f1e equal 9475d37 outside .checkpoints/.) If the anchor holds and
  STATUS.md passes its content check, a different line is a counting artifact: record it and go on. If the anchor
  fails: stop.
- **The 64-file line** (STATUS.md's own 33/33 included): only STATUS.md's numbers can move. The content check governs:
  - `git diff --numstat origin/main HEAD -- docs/STATUS.md` is "1 1";
  - the line out is main's #40 row, and the line in is H0's #40 row, byte for byte;
  - the file has 120 lines.
  With the anchor holding, record the line and go on.
- **STATUS.md's 120 lines:** no tolerance. Main's file has 120 lines and one row is swapped, and his STATUS-DIET cap is
  120. A 119 or 121 is a resolution slip: fix it by Edit (and add again) before the commit, as with MAIN-64's 119.
- **The staged tally** (`UU docs/STATUS.md` plus 63 staged: A 23, M 40): count only lines with A or M in the first
  column; ` M` and `??` notes lines belong to C2 (a). If the tally still reads differently, the list after `git add`
  decides: `git diff --cached --name-status H0` must equal `git diff --name-status 1ec6f6c origin/main` (64 paths,
  23 A, 41 M). Equal: record and go on. Any other path or letter: stop.
- **The checkpoint tests' 33:** any fail stops. If the total differs with 0 fail, and `git diff --quiet origin/main
  HEAD -- .claude/` exits 0 (the kit is main's), record the line (pass and skipped) and go on; step 7 reads it.
- **The repo map's two numbers** (186 entries, 110 required): the bar is exit 0 and "repo map OK"; record both numbers.
  Other numbers with exit 0: record them. Entries are the merged REPO_MAP's rows, and required paths move only with an
  untracked file outside .checkpoints/. Exit 1 stops (the merge is committed and unpushed; the advisor rules).
- **Main's CI** (three check runs from main's push, all success): a read by SHA shows six. The three from
  claude/loving-hopper-6p0mg2's push are extra. Green: note it. Red or pending: note it on the card, but it does not
  stop the take-in (they are not main's). A red or pending run from main's own push stops (C4).
- **origin/main and the base** (28254a7…, 1ec6f6c…): no tolerance. If main has moved, stop before step 2 (C4); a new
  take-in needs a new plan.
- **The block's lines** (markers :24, :28, :31): fixed by both sides' STATUS.md, which have not changed since the plan.
  If they land elsewhere, resolve by content (ours #40 row, then main's "none yet" row) and record the lines; the 120
  and one-row checks govern.
- **merge-tree's tree** (5f57b5ae at the plan, 9475d37 at 23:0x): not bound, since it changes with every notes save.
  What is bound: the one conflict, docs/STATUS.md, and the anchor above.

## 4. The plan's question: may U6c's M3-M9 run once step 6's CI is green? Yes, on the terms below.
Why yes:
- Nothing in U6c's merge needs what only step 7 adds:
  - U6c's 24 files (wip/u6c-fix.patch) are all under spikes/netcode-proof/. They share no path with main's 64.
  - MAIN-65 leaves spikes/ equal to H0's (C1 (c), step 4).
  - What U6c builds on is covered by the main session's own steps 4-5 (the whole-tree checks, the gate, and the light
    run's refs and marks) and by CI at step 6.
- The approved form gates further lane steps on CI of the pushed head, not on the head check: "if gate, smoke or
  toolchain is red on the pushed head, the lane takes no further step on that head (no U4f merge) until the advisor
  rules" (main-61-plan.md:122-123, from my C6). MAIN-61 did run step 7 first (17:46-17:55, U4f's M3 at 18:27), but
  that was before LEAN-MODE item 3 limited agent starts to a reset's first 30 minutes.
- Step 7 loses nothing by reading later: the commits it checks are fixed by SHA. A finding in MAIN-65's part is fixed
  by a forward commit, which cannot touch U6c's files.
- Waiting costs about four hours on R1's path.

The terms (if one cannot be met, U6c waits for step 7):
- (a) **"Green" means:** every check run on the pushed SHA has completed with success, at least gate, smoke and
  toolchain from the push event; none is pending or failed; and PR #40 reads mergeable true. GitHub computes that field
  late and may read it as unknown: then read it again after a minute. Pull-request check runs on the pushed SHA also
  prove the conflict cleared, since none run while the PR is in conflict. Pull-request runs that appear later count
  too: one red there stops the lane, as a red head does.
- (b) **This lifts only MAIN-65's hold.** advisor-u6c-r2.md's own conditions govern U6c's steps, its M3's `git apply
  --check` on the new head included.
- (c) **LEAN-MODE item 3 holds inside U6c's merge:** no new agent outside a reset's first 30 minutes (next:
  03:50-04:20 UTC). Where U4f's form used an agent (M8's papers), the main session writes them itself, or that step
  waits for the reset.
- (d) **Step 7 at 03:50 reads MAIN-65 by SHA:**
  - H0, the merge commit and the pushed card-save commit;
  - `git rev-list --parents <pushed>..HEAD`, with proof that every later commit touches only spikes/, .checkpoints/
    and docs/design/ (U6b's merge 8efb9b2 touched docs/design/ too);
  - it reproduces the gate, the repo map and the checkpoint tests in the main folder when `git diff --quiet <merge>
    HEAD -- . ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/design/'` exits 0, and otherwise in a
    worktree of the merge commit under .gate/;
  - it compares toolchain's summary lines with main's job 138831415745.
  U6c's head read may ride in the same agent.
- (e) **A step-7 finding above Low** means the lane starts no new step (no U8 merge, no U7 R-half) until the advisor
  rules. The fix is a forward commit: no reset, no force push, no revert of U6c.
- (f) **A red CI on U6c's pushed head,** after MAIN-65's was green, is U6c's or the runner's, and U6c's ruling decides.
- One line on record: U7's Block 0b binding check "runs the hour U6c merges" (advisor-u7-brief.md:219-220). It is a list
  of commands, not an agent, so it moves with U6c.

## 5. Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-M65-1 | Medium | Plan :96-97 lets a save run after step 4's commit. `save` pushes every local commit, so the merge would go out before 5a and 5d; this conflicts with main-61-plan.md:110-112, which the plan says stand. | M65-1 |
| ADV-M65-2 | Medium | MAIN-64's values are left in C1-C6 and M64-1: main's CI as "three check runs" (a read by SHA shows six), the resume line, "builder 4", the block's lines, the repo map line, step 7's baseline run. | M65-2 |
| ADV-M65-3 | Low | The card is at its 200-line cap, so the resume line must replace lines. | M65-3 |
| ADV-M65-4 | Low | No risk tier named; step 3 runs `wc -l` after the add; .gate/main-65/ not kept for step 7; the saver's move not tied to hand saves. | M65-4 |
| ADV-M65-5 | Low (on record) | Main's own 3a row still reads "none yet … no pull request yet" although #65 has merged. It is main's text, taken word for word, which is right for a merge. | Main's next docs pass; nothing here |
| ADV-M60-7 | Low (open) | The #40 row says "main 53272a1 merged in" (the branch's own row, kept word for word). | PR #40's closing docs pass, as ruled |
| ADV-M54-8 | Low (open) | CLAUDE.md's Never line lists no plain `git merge origin/main`. | The kit follow-up after the freeze, as ruled |

## 6. What I did not check
- I ran no merge, gate, light run, checkpoint test or repo-map-check (the brief is read only). The repo map's 186/110
  and STATUS.md's 33/33 after the swap are worked out from the code and the trees, not run.
- The saver's task output (its 19:47 "time is up" notice): read only through the card. pgrep confirmed no saver.
- The 22:41 UTC merge-conflict event: not found in the timeline read. PR #40's "dirty" state agrees with it.
- Who pressed Merge on #65: GitHub shows only the Mergincanli account (22:40:39 UTC).
- advisor-u6c-r2.md: not read (it is being written; left alone). U6c's merge steps come from U4f's form (history.md:
  7022-7028) and the card, not from its ruling.
- CI on the pushed head: that is steps 6 and 7.

## 7. Notes (one line per check, kept as written)
- Plan sha256 fd6e8c4c...952eb2: matches.
- Read: main-64-plan.md, advisor-main-64.md, advisor-main-64-head.md, main-61-plan.md, advisor-main-61.md,
  history.md:7815-7900 (MAIN-64's record; LEAN-MODE word for word at :7857-7879), :2050-2090 (MAIN-52).
- SHAs: origin/main 28254a73ab22... = ls-remote main. Merge base 1ec6f6c3852d... HEAD 1d36067 = remote branch
  (ls-remote). HEAD moved 40a3481 -> 92fdf65 -> 1d36067: only .checkpoints/ (card, advisor-u6c-r2.md, the plan,
  u6c-r2-ruling-request.txt). Main: one merge on 1ec6f6c (parents 1ec6f6c, 1624f08), 22:40:38 UTC.
- name-status 1ec6f6c..origin/main: 64 paths, 23 A, 41 M (STATUS.md among the M), 0 D, 0 R; shortstat 64 files
  +4677/-317; without STATUS.md 63 files +4644/-284; STATUS.md numstat 33/33. Step 2's "63 others (A 23, M 40)" holds.
- Guard pathspec (main-61-plan.md:37-40) on origin/main...HEAD: empty. Kit/CI/tools pathspec on HEAD...origin/main:
  empty (main changes no .claude/, CLAUDE.md, AGENTS.md, .github/, tools/). Main's checkpoint.test.mjs: 33 test() calls.
- .gitignore: +packages/sim-worker/pkg-rules/ and +__pycache__/ (as stated). Tracked under pkg-rules/: none. Tracked
  under __pycache__/: the three .pyc files the plan names, all in .checkpoints/. spikes/ has no untracked file (only
  ignored node_modules), so the light run's untracked hash (main-run.sh:42, spikes/netcode-proof only) is not moved.
- check.mjs: refs counts `git ls-files` at ROOT (:2626), the whole index; redact scans the spike folder only (:2643-2647),
  so M0 and R0 do not move with main's files; self is check.test.mjs (spike only).
- HEAD moved again during this check: e8e276f (22:57:18, a save; it took this file's first text). Index 1,825 =
  tree 1,825 (1,822 at 40a3481 + main-65-plan.md + advisor-u6c-r2.md + this file). git status clean. No saver running
  (pgrep exit 1).
- merge-tree HEAD origin/main now: tree 9475d37 (5f57b5ae differs from it in .checkpoints/ only, six names), one
  conflict docs/STATUS.md (content); DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE auto-merge lines as stated. 1,848
  files = 1,825 + 23.
- STATUS.md in 9475d37: 126 lines; markers :24, :28, :31; ours :25 (#40, 2026-11-15 text), :26 (#64), :27 (#63, #62);
  theirs :29 (#40 = base text), :30 (none yet, 3a). Outside the block it equals main's. Main's file: 120 lines.
  Branch changed only the #40 row since 1ec6f6c (1/1). Keep :25, :30 -> 120 lines, #40 row alone differs from main.
- `git diff --shortstat HEAD 9475d37 -- . ':(exclude)docs/STATUS.md'`: "63 files changed, 4644 insertions(+), 284
  deletions(-)", as the plan's preview. Names HEAD..9475d37: main's 64. 81aa7f1e exists (tree).
- C1 (c) on 9475d37: equal to main outside spikes/, .checkpoints/, docs/ (exit 0); docs/ differs in the six names
  (DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE, design/netcode-proof-messages, design/ws-reconnect); spikes/ = HEAD's.
- Auto-merged docs: DECISIONS.md patch-id f0822ab both ways (branch's change = merged vs main), STATUS_ARCHIVE 348c3a6
  both ways; REPO_MAP differs with context, equal with -U0 (a75358f): same lines, moved context, as at MAIN-61/64.
- Repo map (rules of repo-map-check.mjs on main, worked out, not run): entry rows HEAD 173, main 177, merged 186
  (main +13: 9 crates rows, rules.test.mjs, tests/rules/, two reports; the two "built, not in git" rows do not match
  the pattern); no duplicates. Required: main's new files at mapped depths are two docs/reports/ files only, and
  nothing outside .checkpoints/ changed on the branch since 72b0dc3 (110 there) -> 110. Expected "repo map OK: 186
  entries cover 110 required paths".
- DEP-HOLD: Cargo.lock, package-lock.json, crates/devtools/Cargo.toml, packages/sim-worker/package.json, Cargo.toml,
  package.json: `git diff --stat origin/main 9475d37` empty. checkpoint.mjs:23-24 says it as cited; the code
  (:115-141) diffs against the remote branch and frees a file whose blob equals refs/remotes/origin/main's. Not held,
  as long as origin/main is still 28254a7 locally at the save.
- Marks: main changes nothing under docs/adr/, spikes/, docs/design/, .checkpoints/.
- refs: the plan's `git grep -I` prints nothing. check.mjs's referenceProblems (:1852-1867) also reads binaries
  (utf8, outside spikes/, docs/, .checkpoints/, *.md): `git grep -a -l "spikes/"` on main over that set prints only
  kept-logs.mjs and its test (the exception), the two new .bin vectors hold no such text. So refs passes.
- Main's CI on 28254a7 (gh api): SIX check runs, all completed success. Main's push (head_branch main): gate run
  45503161434 (gate job 138831416058, smoke 138831416551) and toolchain run 45503161333 (job 138831415745),
  22:40:42-22:42:53 UTC, as the plan says. The other three: a push of the same commit to claude/loving-hopper-6p0mg2
  (gate 45503920083, toolchain 45503920426, 22:45:42-22:48:27): not main's; a by-SHA read will see them.
- PR #65: merged 22:40:39 UTC by the Mergincanli account (whose hand: not provable from GitHub), merge 28254a7, head
  1624f08. PR #40: head 1fb3a1b (= local HEAD now; 1d36067 -> e8e276f -> 1fb3a1b are notes saves), draft, open,
  mergeable false, "dirty", base 1ec6f6c. The 22:41 conflict event: not found in the timeline read (not material).
- toolchain.yml (unchanged by main): `cargo test --workspace`, `npm test --workspaces`, the boundary dir, playwright
  with the e2e config: 3a's new tests run with no workflow change, as the plan says.
- Card: 200 lines now (M64-2's cap reached). Saver: none (pgrep exit 1); card :9 and :151 say it ended 19:47.
- U6c's fix patch (wip/u6c-fix.patch): 24 files, all under spikes/netcode-proof/ (marks/SERIES.md among them). Main's
  64 paths and U6c's are disjoint. U4f's merge form (history.md:7022-7028): M3/M4 apply, M5 counted run, M6, M7 gate,
  M8 one commit (U4f's papers were drafted by an agent), M9 push, saver, CI, card.
- Precedent: MAIN-61's step 7 ran 17:46-17:55, U4f's M3 at 18:27 (step 7 first, before LEAN-MODE). MAIN-61's own
  risk line (C6, main-61-plan.md:122-123) gates further lane steps on CI of the pushed head, not on step 7.
- Plan's merge-window line (:96-97): "a save only after step 4's commit" conflicts with main-61-plan.md:110-112 ("No
  save runs from step 2 to the push"): checkpoint.mjs save pushes every local commit (:2-6), so a save after step 4
  publishes the merge before step 5's gate and light run. Needs a condition.
- Citations: MAIN-52 history.md:2055-2057; LEAN-MODE items 1 and 3 at :7862, :7864; advisor-main-64.md:15-48;
  advisor-main-61.md:19-104; main-61-plan.md:106-114, :116-128; checkpoint.mjs:23-24: all as cited. Preview 81aa7f1e
  and 5f57b5ae exist; 92fdf65..81aa7f1e without STATUS.md is the 63-file line.
- Main's 23 new paths: no untracked file in their way (`git ls-files --others --exclude-standard` over crates,
  packages, docs: empty; the new folders do not exist yet). Free disk 10,480 MiB (the light runs need 4,096).
- U6b's merge 8efb9b2 touched docs/design/netcode-proof-messages.md besides spikes/: so step 7's later-commit proof
  allows docs/design/ too.
- The anchor's form: `git diff --cached --quiet 9475d37 -- . ':(exclude).checkpoints/' ':(exclude)docs/STATUS.md'`
  takes a tree id (exit 1 today, before the merge, as expected).

## Decisions needed
None. MAIN-52 is his standing rule, and LEAN-MODE item 1 names "main taken in". The one conflict is resolved by the
plan's mechanical rule. No pass mark, judge, marks.json, money, main, stop or run day is touched. #65 was reviewed and
merged on main, and its manifests arrive equal to main's (DEP-HOLD's own exemption).

## For Mustafa
Nothing.

DONE
