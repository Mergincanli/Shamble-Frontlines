# The 00:50 UTC window, 2026-11-27: the messages (written 23:1x UTC on 11-26, sent only inside the window)
**Status: NOT SENT.** Each message below gets a "Sent HH:MM UTC" line when it goes; the window has run only for the
messages that carry one. (This file existing does not mean the window ran.)
Step 0 at the window, each its own call: the reset seen (resetsAt 1795740600, 00:50 UTC); `git ls-remote origin
refs/heads/main` still b9aad01 (else: MAIN-52 first, nothing below); HEAD = origin, clean; ListAgents; df. Then the
advisor tool once on this file; then B, A and C at once; D only after B's ruling is on disk. At most 3 agents at
once; new work only 00:50-01:20 (LEAN-MODE). A ruling after 01:20 moves D to the next window (05:50).
Every message below carries the two standing sentences word for word.

## B: a3de649cd25e474df (resumed; it wrote advisor-u8-f1-brief.md): the ruling on U8-F1's security round
Rule on U8-F1's security round as the advisor (main's .claude/agents/advisor.md, by `git show
origin/main:.claude/agents/advisor.md`; read only, no cargo). Read .checkpoints/netcode-proof-test/security-u8-f1.md
whole (SEC-U8F1-1 Medium and its fixes (a), (b) and (c); the three Lows; "Decisions needed", S14-VMS at :182-187),
u8-f1-build.md, and his S14-PUSH (a) at history.md:8090-8106 (his words: "through the same redaction check").
1. Rule each fix: (a) and (b) as written or changed; (c) S14-VMS, dropping `--vms` for the s14 folder's own check:
   say whether it departs from his words. If it departs, it is his: write its entry for "Decisions needed" (a code, the
   question in plain words, why, the options, your lean) and rule what the fix pass does meanwhile.
2. The fix pass's size: U8 is at 6,282 of its 6,282 stop (U8-F1 +93 of 93). Name each fix's net lines; the fix pass
   must stay at or under the stop, else its builder stops and reports (U8F1-SIZE (a)'s rule).
3. Write the fix pass's brief items for U8-F1's builder (a7fe1b9f436f36bb1): what it changes, the reds and plants
   that prove each fix, the end checks (ops tests, wrappers, redact on the s14 fixture), the hand-back and snapshot.
4. The Lows (-2 to -4): fix with the Medium where they ride its files, else carry with a taker.
Write .checkpoints/netcode-proof-test/advisor-u8-f1-sec.md (plan first, then each block as you finish it); end with
"Decisions needed" and "For Mustafa". Write files with the Write and Edit tools only (no sed, no shell redirect, no
python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its
notes file whole with Write. git and gh always start their own command, with no other program in front of them.

## A: a0530735048f1620a (resumed; it wrote advisor-u7-snapshot.md and advisor-u7-merge.md): U7-M-8, the head read
Read U7's pushed merge as the advisor (main's advisor.md, as before; read only, no cargo): commit 6968215 on H0
dc51f0e, pushed by 9c2fcf4. Not a round (U7-M-8). Sources: u7-merge-plan.md's done lines (M0-M9), u7-report.md,
.gate/u7-merge/ (light-base.log, c1-judge.log, c1-verdicts.json, m5-check.log), .gate/u7/m2b-header.log, CI on
9c2fcf4 (push gate 45906116335 with gate and smoke, toolchain 45906116365; pull request gate 45906121962, toolchain
45906121987: all success). Rule on:
1. The head: `git diff --stat dc51f0e 6968215 -- spikes/` against the snapshot's numstat; MERGE stands, or a fix.
2. The M3 retry (the plan's done lines, 22:1x): the first `git apply --check --index` was refused on stale cached
   times (main-run.sh:48-52 touches crates/), content equal by three checks; `git update-index --refresh` and one
   re-run by name, after the advisor tool's read. Was that inside M6's "never retried unnamed", or should M6 have run?
3. The form's fix for later merges of this form: the base run before H0's save, or a refresh before M3 and before
   M6's undo, or another; the words to write into the form.
4. The third patch: wip/u7-t-build.patch removed at M8 under WIP-CLEAN (inside the snapshot; the plan named two).
5. ADV-U7S-6's new home: carried to PR #40's closing list (U8's stop 6,282 is spent by U8-F1): agree or rehome.
6. UM-5: name the cleanup forms for U7's builder worktree (agent-a9873cdba07a3219e: U7's uncommitted files and its
   notes; a copy of each is now committed), .gate/u7-amend (a detached scratch worktree at c21582d with two Edits), and
   cli-target (433 MiB, `cargo clean --target-dir`). Never `git worktree remove --force`, never `rm -rf` (the guard
   refuses both); only forms that keep every file a copy is not yet committed for.
7. u7-report.md: anything wrong or missing against U7-M-7's list.
Write .checkpoints/netcode-proof-test/advisor-u7-head.md (plan first, then each block); end with "Decisions needed"
and "For Mustafa". Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never
`cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with
Write. git and gh always start their own command, with no other program in front of them.

## C: a2b7636a57dff4206 (resumed; it wrote advisor-main-68b.md): MAIN-68's step 7, the read of the pushed head
Read MAIN-68's pushed head as the advisor (main's advisor.md, as before; read only, no cargo): merge 6b4a0d3 (H0
d2d55b4 + main b9aad01), pushed by 7971da0 at 20:20 UTC. Sources: main-68-plan.md (rev 4, C68B-1 to -6 applied),
.gate/main-68/step5-outputs.txt (steps 2-6 and every bar), CI on 7971da0 (push gate 45890271684, toolchain
45890271634; pull request gate 45890273559, toolchain 45890273394: all success). Rule: the merge as recorded (the two
STATUS conflicts resolved as C68B-4 says, nothing else changed against main and H0, every bar met), APPROVE or a fix;
and anything owed to him. Since then the lane took U7 (6968215); that is A's read, not yours. Write
.checkpoints/netcode-proof-test/advisor-main-68-head.md (plan first, then each block); end with "Decisions needed" and
"For Mustafa". Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never
`cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with
Write. git and gh always start their own command, with no other program in front of them.

## D: a7fe1b9f436f36bb1 (resumed; U8-F1's builder): the fix pass, only after B's ruling is on disk
Base: stay on 77ffc95101e186f75c57a1e8770b3ab7a9442820, no merge. Checked 23:1x: `git diff --stat 77ffc95 HEAD --
spikes/netcode-proof/ops/` is empty (your six files unchanged on the lane through 4743561); ffcheck shows six of your
own notes files ADDED on the lane (the main session's copies), so a fast-forward would be refused, and none is needed.
Do not delete or move any file to make a git step pass. Your brief: advisor-u8-f1-sec.md's fix-pass items, word for
word (fill from B's file at sending). The stop: 6,282 (U8 is there now); past it you stop and report what grew, why and
the cheaper option. First calls in your own worktree: show-toplevel, HEAD (77ffc95), status. Every cargo call
`--locked`; no shell loops; one command per call; waits in the background. Notes: append to u8-f1-build.md under a new
"Fix pass" heading, a line after each finding. Hand-back: the snapshot by worktree_patch.py, added, removed and net per
file, the end checks' counts. Write files with the Write and Edit tools only (no sed, no shell redirect, no python
writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes
file whole with Write. git and gh always start their own command, with no other program in front of them.
