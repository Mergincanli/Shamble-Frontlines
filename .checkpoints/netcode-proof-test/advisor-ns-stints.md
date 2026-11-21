# advisor: PR #54 (claude/night-shift-stints), round 1

Head checked: 20f9d59918eea1e3431ae830f72510f83f6424c7 on main 53272a11024dcec8349e67bd8c70a8db337e85bb (= origin/main,
fetched now; merge-base 53272a1). Followed main's `.claude/agents/advisor.md` (git show origin/main:...). Read only: no
build, no test, no gate run (the task forbids them); git as plain read commands; gh api reads only.

## Verdict
APPROVE WITH FINDINGS (5 Low, none needs a push, none rewords his lines). Merge conditions:
- C1: security-auditor's round-1 review is in, and each of its findings is addressed or deferred in writing.
- C2: before the merge, the day session runs the full gate (`node tools/gate.mjs`, smoke included) on 20f9d59 in a
  worktree, plus the three kit tests, with the same counts as on main. I was barred from running them; CI's gate and
  smoke are green on the head.
- C3: the head stays 20f9d59918eea1e3431ae830f72510f83f6424c7. Any push needs a new advisor review and Mustafa's go
  again, and any change to the three lines also needs his new yes (KIT-FREEZE-2's exception covers these lines only).
- C4: the PR comes out of draft after the round, then merges from a day session on Mustafa's own go in that chat,
  passing the SHA as the expected head.

## 1. Scope (done)
- `git diff --stat 53272a1 20f9d59`: `.claude/NIGHT_SHIFT.md | 3 +++`, 1 file. `git diff --raw --no-renames
  origin/main...20f9d59`: `:100644 100644 5f2d87b c1a95f7 M .claude/NIGHT_SHIFT.md` (plain file, no symlink or
  submodule). `--name-only`: that one path. The +3 sit right after step 6's last line (44), before step 7 (48).
- PR #54 (gh api, REST): open, draft, base main at 53272a1, head claude/night-shift-stints at 20f9d59, head repo
  Mergincanli/Shamble-Frontlines (not a fork), 1 file, +3 -0, no labels. No `.checkpoints/` file in the PR.
- CI check runs on 20f9d59: gate success, smoke success, toolchain success (each twice: push and PR runs). The PR
  touches neither `.github/` nor a gate file, so CI ran main's gate code on this head.

## 2. His lines and the hash (done)
- `sed -n 45,47p` of the worktree file, final newline dropped: sha256 4e3d15658c0afaf40d922cdce9559a8dc2d1079ea04a8fc96e327798b3639223
  = the hash in his record. With the newline: c4797a52..., identical to history.md lines 2194-2196 with theirs;
  `diff` of the two: IDENTICAL. The committed copy (git show 7fe4100:.checkpoints/netcode-proof-test/history.md,
  same lines) also hashes to 4e3d1565...
- His words read whole (history.md 2177-2199): NS-STATUS (a), 2026-11-16 21:50 Amsterdam (transcript 20:48:10Z); the
  NS-LINES question at 20:50:16Z; his "yes" at 21:09:27Z. I cannot read the chat transcript itself: this rests on the
  session's script copy, committed at 7fe4100.
- His conditions, each checked:
  - own pull request, only `.claude/NIGHT_SHIFT.md`, only the lines needed: yes (one file, +3 -0).
  - no guard, hook, settings or script change: yes (only the Markdown file changed).
  - after C62H-BOUND's P2 and P3 runs: P2's results daec788 21:18:08Z, P3's 8ca774a 21:36:55Z, the PR commit
    21:56:23Z. Holds.
  - the exact lines first, then his yes, then the edit: question 20:50:16Z, yes 21:09:27Z, commit 21:56:23Z. Holds.
  - then one review round (this one), then a stop for his go on the merge: pending.
- KIT-FREEZE-2: main's DECISIONS.md 27-37 gives the freeze (after kit-v3-5, until the foundation's gate step;
  exceptions a broken main and his items 2 and 3; FREEZE-FDN: the freeze stops new rules, new guard checks and kit
  redesign). This change is a new rule line, so it needs his exception, and NS-STATUS gives it in his words: "My yes
  will also be the exception to the kit freeze (KIT-FREEZE-2) for this one change." His yes of 21:09Z is that yes. The
  exception covers this one change: these three lines. Any rewording needs a new yes.

## 3. Fit with NIGHT_SHIFT.md, CHECKPOINTS.md §5 and checkpoint.mjs (done)
- checkpoint.mjs on main: parse() lines 60-80, defaults every 10, for 110; refuses unless `--every >= 0.01` and
  `--for > 0 && --for <= 115`. `--every 10 --for 25` is valid. watch() 189-211: end = start + 25 min; sleeps
  min(every, remaining): saves (if `.checkpoints/` changed) at 10 and 20, then at 25 `last('time is up')` saves and
  exits 0. So a stint ends at 25 min plus one save; the harness then reports the background task ended, which is
  what wakes an idle run.
- Step 3 (busy < 45 min) and step 4 (handoff > 45 min): a status refreshed every ~25 min stays under 45, so a later
  hourly run reads the live run as busy and stops silently instead of taking over. Agrees. If the run is cut off by a
  limit, no refresh follows, the status ages past 45 and the handoff happens as step 4 means; a stint saver left alive
  stops within 25 min, sooner than §5's 110-minute one. Agrees, and bounds the overlap.
- Step 6 ("at least every 30 minutes"): stints of 25 min with a refresh at each end keep the gap under 30 in steady
  state. Only the first stint could exceed 30 if it starts well after the last refresh (nit, not a finding).
- Step 7 ("during long agent runs the background saver, checked at every update, §5") and §5 ("stopped with `time is
  up` ...: start it again with the same command"; "one saver per task"): "then start the next stint" is §5's restart;
  agrees. §5's text still says 110 minutes in places (L3).
- Evidence the method works in a routine: the night run's checkpoint (git show 34a2e55:.checkpoints/npt-u4f-brief.md,
  "Running work", lines 97-102): "saver: run in 25-minute stints (`--every 10 --for 25`), so each stint's end wakes the
  run to refresh the status comment"; first saver stopped by TaskStop at 00:13; stint 1 00:13-00:38 time is up; next
  started 00:39. Commit ded59b7 "01:29 update ...; saver stint 4". The night report (issue #16 comment 6904274561, by
  Mergincanli, 2026-11-16T00:50:20Z) says the same and raises NS-STATUS with options (a)/(b). The PR body's "35-38
  minutes" matches the checkpoint (35 and 38 min agent runs).

## 4. Command form against step 7's rule (done)
- The backquoted command: `node .claude/scripts/checkpoint.mjs watch <slug> --every 10 --for 25`. No comment, `$`,
  backslash, here-document, brace or parenthesis inside it. `<slug>` is a placeholder, the doc's own convention
  (step 4 `git fetch origin <branch>`, step 7 `git add <files>`). The parentheses and comma around it are prose outside
  the code span. Unquoted numbers and slug: same as step 4's unquoted `<branch>`; R-PLAIN needs no quotes for plain
  characters. It does not break step 7's rule.
- Filled in (e.g. slug npt-u4f-brief), the guard on main admits it: R-PLAIN (routine-guard.mjs 592-675) letters,
  digits, space, `.`, `/`, `-` only; R-PROG program `node` (584); interpreter() 989-1013: no node flag, script tracked,
  equal to origin/main's copy, `watch` -> protectedInBranch; background: only git is refused with a background & or
  run_in_background (688-689, 753-756). Last night's run ran it (with `--trailer` options, L1).
- "in the background" means Bash run_in_background (§5 names it; last night's task ids bmctdoqdt, bmv14vtwd). A 25-min
  stint fits even the Bash tool's default 30-min background timeout.

## 5. Rules before/after (tier-2 checklist)
| Rule | Before (main) | After (head) | Looser? |
|---|---|---|---|
| Step 6 status refresh | "Update it after every commit and at least every 30 minutes." No method while waiting on agents. | Same sentence, plus: while waiting on agents, saver in stints under 30 min (`--every 10 --for 25`, background), refresh at each stint end, then the next stint. | No. Adds a method that makes the 30-min rule reachable; permits nothing new (same script and subcommand §5 already runs; the guard already admits it). |
| §5 saver default | `--for` 110, restart at time is up | unchanged (the new line uses 25, inside checkpoint.mjs's 0 < for <= 115) | No. |
No guard, hook, settings, script, CI or gate file changed.

## 6. Risk tier and merge (done)
- Risk tier 2: `.claude/NIGHT_SHIFT.md` (a `.claude/` kit file; the kit's Markdown is tier 2). Class needs-mustafa.
- Merge needs: security-auditor's review in (in progress: security-ns-stints.md, block 6 of 7 at 22:00), its findings
  addressed or deferred in writing; the advisor's APPROVE of 20f9d59918eea1e3431ae830f72510f83f6424c7 naming that SHA;
  gate and smoke green on it (they are); the PR out of draft; Mustafa's own go in the merging session's chat about #54
  and this head, given after the APPROVE; merge from a day session (the PR Advisor's allow-list excludes `.claude/`),
  passing 20f9d59... as the expected head. Any push: a new review, then his go again.
- Kit tests (routine-guard.test.mjs, settings.test.mjs, checkpoint.test.mjs): not run (forbidden here). No test reads
  NIGHT_SHIFT.md from disk (git grep NIGHT_SHIFT over *.mjs/*.js/*.json/*.yml: only comments in
  routine-guard.test.mjs 2803-2810; its KIT_LINES list is hard-coded), and every test and script file equals main's,
  so the counts cannot move; still owed by the merging session (C2).
- STATUS/ROADMAP/DECISIONS.md: not updated in this PR, rightly (his words: "Only that file"). Recorded after the merge (L5).

## 7. Findings (all Low; none needs a push)
- L1 (Low, changes his lines if fixed): the line's command has no `--trailer`. §3/§5 give the command with one
  `--trailer` per attribution line, and last night's autosaves carry both lines (34a2e55, 8bfa7ce). A run copying the
  line literally makes autosave commits without attribution. Nothing breaks: no check requires trailers. Fix now:
  none; the run keeps §5's `--trailer` options (step 7 points to §5). Only Mustafa can reword the line, later, with a
  new yes.
- L2 (Low): the method wakes the run only when the agents run in the background (Agent run_in_background or a
  workflow). A foreground agent call blocks the run, so the stint's end waits until it returns. This was already a
  gap; the change does not cause it. Last night's agents ran in the background. Follow-up after the freeze.
- L3 (Low): CHECKPOINTS.md §5's "running" test (first line "every 10 min for 110 min" less than 110 min old) and the
  "Running work" template ("stops by HH:MM (110 minutes on)") name 110 min; for a stint it is 25. The harness's
  ended notice still decides, and §5's one-saver rule means a 110-min saver is stopped (TaskStop) before the first
  stint, as last night's run did. Follow-up after the freeze.
- L4 (Low): routine-guard.test.mjs's KIT_LINES (2807-2819) does not list the stint line. By inspection it is plain
  form. Add it at the kit's next change (no script change is allowed now).
- L5 (Low): NS-STATUS and NS-LINES are recorded on the netcode branch only (history.md). STATUS, ROADMAP and
  docs/DECISIONS.md on main do not record them; docs-keeper does that after the merge.
