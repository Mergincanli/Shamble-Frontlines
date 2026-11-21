# infra-engineer: U4r-F2 review (round 1 of at most 2), notes

Started 2026-11-14 ~21:03 UTC. Target: the uncommitted diff to spikes/netcode-proof/ops/runs/c-u4r-runs.mjs (HEAD 6498d97).
(No Edit tool in this agent: the notes are rewritten whole with Write.)

## Plan
1. Read the plan (u4r-f2-plan.md), the advisor's plan review with its nine conditions (advisor-u4r-f2-plan-full.md),
   the Q2 ruling (advisor-u4r-sets-r1-full.md), context (u4r-f1-review-full.md, the card's run procedure).
2. Read the runner whole (current copy) with line numbers: order of re-checks, the two calls, results/, run() in
   check.mjs (condition 1's endings), exit lines, lock, process groups, timeouts, layout, exists-check.
3. Read the harness (.gate/u4r-f2/harness/) and its logs; re-run stagings myself; run the scratch helper test.
4. Map each of conditions 2-5 to evidence; note gaps.
5. P0 70-minute set: what is lost under each failure (judge call 1 fails, call 2 fails, both, timeout, kill, disk).
6. Findings INF-F2-n, verdict.

## Block 1 DONE (reading)
- Working copy sha256 18c1dcf6... = harness's f2 copy (sources.log). HEAD copy c5cd4d9c.... diff --numstat 52/22 (one file).
- Runner (working copy): judgeCall :445-455 (unchanged(what) first, then run() with RUN_LIMIT_MS, three endings :452,
  end words :454); calls :456-458 (split: measured then bites, array literal: a re-check throw in call 2's closure
  aborts before :459, so no results/); results/ made :459 only after both; copies :460 on c.ok; judge lines :461-463;
  runs.txt :472; exit lines :474 (no split: HEAD's words) and :475 (split: cond 3(d)); written line :476.
- --out for runs: :436 join(logs, split ? r.group : "", r.run). npt-lab create_dir_all(&s.out) scenario.rs:771.
- judge: read_logs walks .jsonl at any depth (judge/src/lib.rs:19-46); <logs>/verdict.json and marks.json are .json:
  never read. Exit 0 implies --out written (main.rs:110-121), so copyFileSync after ok cannot ENOENT.
- check.mjs:1909-1922 run(): spawnSync detached + SIGKILL at timeout + group kill; maxBuffer 256 MiB; no signal field.
- Header sentence :10-13 = advisor-u4r-f1-head-full.md:33 word for word (condition 8).
- Harness helper staging's NOTES path (.checkpoints/.../u4r-f2/) no longer exists (moved to .gate/u4r-f2/scratch/,
  sha256s equal to sources.log's): ran the test by hand.
- Condition 1: P2 results commit 1eacb60 at 20:22:31 UTC; script mtime 20:26; no spike commit after 1eacb60.

## Block 2 DONE (re-runs, ~21:06-21:13 UTC)
- harness run cmp fail lim recheck: 15 stagings, 0 FAIL, exit 0 (my run).
- helper test by hand: # tests 5, # pass 5, exit 0.
- .gate/u4r-f2-review/signal-ending.mjs: run() on a self-SIGKILLed child -> "exit by a signal (no verdict): <out>" and
  "exited by a signal" (condition 3(a)/(d)'s third ending, the OOM case): holds.
- .gate/u4r-f2-review/keys.py on .gate/u4r-f2/real/*.json: measured-alone differs from combined only in -17's
  bites+why; bites-alone only in parts/report/why; no bites-alone part counts a run. compare.py re-run: ALL EQUAL.
- Independence by code: -2/-17/-57 have no Against threshold, no ws_bound, no validity rule; their figures take `_set`
  unused (figures.rs:24, :542, :563, :571); twin figures (:156, :195) read the set but no P0 mark uses them.
- node --check OK; redact-check re-run 284 items, 0 findings. Script mtime 20:26 < redact/gate logs 20:52.
- .gate/u4r-f2-review/stop-order.mjs (F2 copy sha 18c1dcf6, shims, judge sleeps every call): kill -9 of judge call 1's
  group -> call 2 started 81 ms later, script alive; kill -9 of the script -> call 2 (pid 11551) and its sleeper alive
  0.5 s later (orphan), no results/; cleaned up (pgrep empty after).

## Findings (one line each, as found)
- INF-F2-1 (Medium): the header's stop order (:12-13, new in this diff) and card item (6) (netcode-proof-test.md:350)
  during a split set's measured call: ending call 1's group starts call 2 at once (Q2 (a)), then killing the script
  orphans call 2 (no limit, lock free). Measured (stop-order.mjs). Fix: card item (6) now; header at next change.
- INF-F2-2 (Low): header :35-36 and :443-444 claim "No run's verdict reads another run" / "both always run" without
  scope; true for -2/-17/-57 at marks 9fb5be57 (twin figures read the set); :66-69 still says "no verdict" for the
  judge's failure. Comment only, at the runner's next change.
- INF-F2-3 (Low): evidence housekeeping: condition 5's /dev/shm removal open (66 MiB tmpfs, on the card as
  LEFTOVER-DELETE); harness helper/realjudge stagings point at the moved scratch paths (cannot re-run as is).

## Verdict
APPROVE WITH FINDINGS (no code defect; one Medium in the operator procedure, card-only fix).
