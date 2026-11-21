# infra-engineer: U4r review round 2 (PR #40), notes

Snapshot under review: wip/u4r.patch sha256 21ca46be23704131c77cc20981a4bfe866811b33f9ca853279f7ddbfb5d58bce (checked 2026-11-14).
Packet u4r-r2-packet.md b1526bde... (checked), tests file 637249a5... (checked), runner u4r-run.sh 6efb9157... (checked).
Read copy: .gate/u4r-snap2 (2fbec77 + patch); compare .gate/u4r-snap1 (572c4f69).
Free space at start: 4,913,340,416 B (4.58 GiB): no builds, no runs (none needed: every claim checked from code and logs).

## Plan
My list (ruling "Who reads what"): FP-0, FP-5 (a) to (f), INF-U4R-5, INF-U4R-10 (1) and (3), FP-8's times against budgets.
Plus my round-1 ids INF-U4R-1..11: confirm fixed / deferred as ruled with file:line in snap2.
Report words to check: INF-U4R-7 (and FP-9 list incl. INF-U4R-6, INF-U4R-10 (2)).
1. FP-0: runner diffs (u4r-run.sh, main-run.sh, .diff files), bash -n, copy 6's first log MiB line.
2. FP-5 (a)-(f) in c-u4r-runs.mjs and c-u4r-r2.mjs (snap2 vs snap1 diff).
3. INF-U4R-5 det-u4.mjs; INF-U4R-10 (1)(3) check.mjs run()/wasmStep.
4. FP-8: c6 end-check logs, det-u4 and precheck regen logs, times vs 200 s / 240 s / 300 s.
5. Report section "Fix pass, round 1: report": INF-U4R-7 words, follow-up list, each run checked in its log.
6. Helper test output (FP-5 evidence) for my items.

## Blocks done
- Block 1 FP-0 DONE: committed blobs 8678438 (u4r-run.sh) and 9901cf6 (main-run.sh) at 643ddd2; `git diff b040c7f 8678438` and
  `git diff d9ce0e1 9901cf6` equal the two .diff files hunk for hunk (only the a/ b/ header names differ); bash -n rc 0 on both;
  grep: avail used only at :63/:66/:68 (u4r) and :38/:41/:43 (main); every c6-*.log line 2 reads "free <n> MiB" (first: c6-fp12-1.log
  "free 4780 MiB"). As ruled. (Header comments :5 / :4 still say "4 GB free": ruling said no other line changes; fine.)
- Block 2 FP-5 DONE (see findings). Order (a), projection (b), per run (c), (d), (e), (f) all present as ruled; npt-lab/judge argv
  unchanged vs snap1 except the marks/SERIES copies (diff of '"--' lines). Table cites u4r-build.md:739-751 (rows match :739-748);
  PAIR_BYTES 25,164 from c4-b12-3.log; r2 bytes from u4r-build.md:1734-1737 (13 sum 115,746,371; r2Projected 169,813,722 checked).
- Block 3 DONE: det-u4.mjs:13-16, :22, :31-33 (deps first; main() runs every step, exit 1 if any fails: check.mjs:2653-2683);
  c6-det-u4.log "PASS deps 1.4 s", "check: all 2 steps pass". check.mjs:1910 killSignal SIGKILL; :1898-1900 doc; :2336 performance.now;
  :2357-2359 Math.ceil (needed: Node's spawnSync refuses a non-integer timeout; c6-end1.log crashed exactly so: "Received 296513.41192700004").
  My own round-1 fix text for (3) omitted that; the builder's ceil + self-test + plant (c6-plant-ceil.log 76 of 77) completes it.
- Block 4 FP-8 DONE: c6-end2 wasm 43.1 s, c6-end3 48.7 s (vs 200 s; round 1 39.6/41.9). det-u4 235.0 s; non-cargo 220.3+2.5+0.9 = 223.7 s
  (vs 240 s escalation, 300 s kill; round 1 190.9). npt-lab hash 187.8 -> 220.3 s (+17%); precheck regen 110.1 -> 116.5 s (+6%).
  check.mjs wall under -x lock: 12:13:14-12:26:11 (13.0 min), 12:26:50-12:39:59 (13.2 min); test step 607.9/616.7 s (round 1 556.8/588.5).
  Label: recomputed the builder's fingerprint on the worktree now: `{ git diff HEAD -- spikes; git ls-files --others --exclude-standard
  -- spikes | sort | xargs sha256sum; } | sha256sum` = 63713cf3...; `diff -rq .gate/u4r-snap2/spikes <worktree>/spikes` empty, so the
  logs labelled 63713cf3 ran on patch 21ca46be's tree (builder: no edit after 12:12, checked 12:48).
- Block 5 report DONE: INF-U4R-7 words at u4r-build.md:1916-1923 (inside the FP-8 block; the report's :1997 says "item 2 below", it is
  above). Follow-ups INF-U4R-6 and INF-U4R-10 (2) at :1947-1949. end check 1 crash reported :2030-2033, :2078, :2085-2086.
- Block 6 DONE: c6-fp5-helpers.log 4 of 4; P0 3020 MiB refused at 4780 MiB free; P2 629 MiB room by 55 MiB.

## Findings log (one line each as found)
- INF-R2-1 (Medium): FP-5 (e)'s re-check compares the whole repo's HEAD and tracked status (c-u4r-runs.mjs:251, :317-321, :349, :366;
  c-u4r-r2.mjs:154, :209-211). The sets run in the main folder (header :6; .cargo/config.toml target-dir is relative, so another
  folder means a full rebuild) in the background (brief section 7) while CHECKPOINTS.md §5's saver commits .checkpoints/ every 10 min
  and §1 asks a commit at least every 30 min: any such commit or tracked checkpoint edit stops the set before the judge, after all runs
  (P0 about 31 min of runs at r5's rates, 3,020 MiB of logs that LEFTOVER-DELETE keeps; a re-run then needs a new commit and 7+ GiB).
  My own round-1 INF-U4R-9 fix proposed this whole-repo check; I missed the saver. Fix: compare `git rev-parse HEAD:spikes/netcode-proof`
  and `git status --porcelain --untracked-files=no -- spikes/netcode-proof` (start vs end), name the start commit as now.
- INF-R2-2 (Low): det-u4 margin: non-cargo 223.7 s, 16.3 s (6.8%) under the 240 s line; +32.8 s (+17%) since round 1, split unmeasured;
  U4h's TC B1 recording (carry 87) adds a regeneration (round-1 ruling: about 52.6-142.6 s per runner entry) and will cross 240 s.
- INF-R2-3 (Low): c-u4r-runs.mjs creates the set folder (:298) before the marks.json sha check (:301) and run 1's floor (:325/:359): a stop
  there leaves the folder that :266 refuses (narrow window). Same in r2: mkdirSync(out) :180 before suite 1's floor :197.
  Fix: floor(first run) and the sha check before mkdirSync (copy to bin first).
- INF-R2-4 (Low): header :6-9 now has the set held under flock -x, but timed() (:233-238, sh() :202-206) gives npt-lab no limit, unlike
  r2's step(); a hung judged run would hold every lane. Fix: run() with a limit (e.g. 3,600 s for a 610 s run), "stopped at" as a result.
- INF-R2-5 (Low): c-u4r-r2.mjs:147 `exit ${r.status ?? "by a signal"}`: run() (check.mjs:1921) returns no signal, so a record command
  killed by a signal no longer names it; snap1's step() wrote `signal ${r.signal}` (snap1 c-u4r-r2.mjs:97). Fix: run() returns
  `signal: r.signal`; step() prints `exit by ${r.signal}`.
- Verdict: APPROVE WITH FINDINGS (1 Medium, 4 Lows; none High). All INF-U4R-1..11 fixed or deferred as ruled.

DONE
