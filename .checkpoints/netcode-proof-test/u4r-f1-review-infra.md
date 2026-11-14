# infra-engineer: U4r-F1 review (one round, final diff + evidence), 2026-11-14

## Plan
1. Read plan u4r-f1-plan.md, advisor-u4r-f1-plan-full.md (conditions 1-8, Q1-Q6), rulings r2-full INF-R2-1/-3/-4, r2run-full step 7, c62h-full step 4, findings u4r-review-r2-full.md.
2. Read the diff (git diff -- spikes/netcode-proof/ops/runs), both edited scripts whole around the changes.
3. Check each item A-E + conditions 1-8 against code file:line.
4. Read evidence logs .gate/u4r-f1/logs/ (summary, sources, edited-copies, per-staging) and harness; check notes' claims against logs.
5. Re-run doubtful parts in /dev/shm/u4r-f1-review/ (scratch copies only).
6. Risks for real sets: lane lock, process groups, disk, stopped run logs, commits during a set.
7. Size: measured diff --stat vs plan/advisor.
8. Verdict + findings.

## Progress
- step 1: plan + advisor plan review + c62h step 4 + review-r2 INF-R2-1..4 read.
- step 2-3: diff read; edited scripts read (runs :1-456, r2 :1-230). Items A-E and conditions 1,5,7 map to code:
  A runs :128-146 (spikeChanged/spikeReading), :306-307, :368-371; r2 :54, :169-171, :222-223. A5 runs :355, r2 :200.
  B runs :346-353; r2 :192. C runs :124-128, :277-283, :381, :395-400, :419, :427-445. D runs :260-270, r2 :137-147.
  E r2 :91 (word for word = c62h step 4). Cond 7: start = {commit, ...reading()} both; tracked() kept runs :296-297, r2 :164-165.
- step 4: sources.log sha256s re-checked by me: main folder runs c5cd4d9c, r2 94564f8c = f1 copies; HEAD 8ececd9d / c861da5d; check.mjs a169a286; marks 9fb5be57; edited copies' sha256s match. Spike tree at HEAD (now 4a6a145) = 0b47e881 = 7ff3823's.
  Logs read: summary (48 stagings, lim-* first run FAILed on harness liveness check then PASS), cmp-P0/r2/precheck, git-a/b/c (P0, r2, precheck; start commit vs post-save HEAD checked), lim-run/judge/pair/rec/control, realjudge, helper (9/9), redact, floor-*, ii-*.
- step 7: numstat runs 100/35, r2 25/13 = 125/48 (as card). Added: 62 comment lines + 63 code lines. node --check both OK (mine).
- checkpoint.mjs save commits only .checkpoints/ via private index + update-ref (:150-170): a saver commit passes the scoped check.
- judge on a whole-line cut (no footer): log reads with footer None, judge's complete rule (eval.rs:579) makes the run invalid, exit 0: code copies verdict.json, runs.txt keeps the stopped line. Both ends handled.
- step 5 (my scratch, /dev/shm/u4r-f1-review/parent.mjs, parent2.mjs: check.mjs:1909-1922's run() copied verbatim,
  under `flock -x /dev/shm/u4r-f1-review/lane.lock node ...`, node v22.23.4, util-linux flock 2.39.3):
  case A (limit 2000 ms): call stopped at 2.0 s, lock FREE after, no call left. case B (script node kill -9 mid-call,
  call = node sleeper, then /bin/sleep 6017): flock + script hold the lock fd (node's copy flags 02100000 = O_CLOEXEC),
  the call holds 0 lock fds (own sid/pgid); after kill -9 of the script: flock exits, lock FREE, the call still runs.
  => condition 6(b)'s mechanism ("the locked descriptor that flock passes down stays open in it") is false: node marks
  inherited fds close-on-exec (libuv uv_disable_stdio_inheritance), so npt-lab never holds the lane lock.
- disk now: 4,474 MiB free on / (helper test saw 4,780 at 17:10); main cache .gate/netcode-proof-test/target 6.1 G (release 679 M); /dev/shm/u4r-f1 75 MB left.

## Findings (one line each as found)
- INF-F1-1 (Medium): card condition 6(b) / run procedure: the lock is RELEASED (not held) when the script dies mid-call;
  the orphaned call runs unbounded beside the next lane. Fix: card wording + stop order (end the call's group first).
- (pending, Low) judge "exit by a signal": run() returns no signal name; condition 1 says "<status or signal>". r2 step() uses same form.
- (pending, Low) runner log for c-u4r-runs sets not named in header form: condition 5 needs a kept runner log. Card.
- (pending, Low) condition 6: plan's risks not updated (plan sha 774b952d unchanged); card holds 6 only inside the conditions list.
