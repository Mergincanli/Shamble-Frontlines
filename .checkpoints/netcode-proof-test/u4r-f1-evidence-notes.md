# U4r-F1 evidence: notes (agent; plan first, then one line per staging with its log)

Started 2026-11-14 on claude/netcode-proof-test-plan-hcky5g at HEAD c18c36c (F1 uncommitted in the main folder:
2 files, 125+/48-). What to prove: u4r-f1-plan.md section 5 items 2-4, as extended by advisor-u4r-f1-plan-full.md
conditions 1-4 and 8. On a restart: read this file, check /dev/shm/u4r-f1/ and .gate/u4r-f1/logs/, continue at the
first staging without a "done" line.

## Plan
- Files (Write only): .gate/u4r-f1/harness/harness.mjs (prep, run, realjudge, helper, collect), shims/{cargo,git,
  npt-lab,npt-judge}.cjs, u4r-f1-helpers.test.mjs (copy 6's 4 tests kept + new ones).
- prep: /dev/shm/u4r-f1/src/{head,f1,f1-limit,f1-floor,head-floor}/ from `git show HEAD:` (harness spawn, cwd main
  folder) and the main folder's F1 files (read); check.mjs, crates/hash-wasm/loader.mjs (check.mjs imports it),
  marks.json, SERIES.md, precheck-recordings.json from HEAD. Then Edit by hand: f1-limit RUN_LIMIT_MS = 2_000;
  f1-floor and head-floor: floor() refuses "l1-p0-s1", "the recording SS-P0-s4" and "suite p0". sha256s recorded.
- Stagings (each its own tree under /dev/shm/u4r-f1/stage/<name>/, its log logs/<name>.log):
  - cmp-<set>-{head,f1} for precheck, P0, P2, P3, B1, r2: git shim (fixed commit, tree, clean status), then the argv
    compare (npt-lab, npt-judge) and runs.txt / r2.txt line compare, normalized (root, wall, uptime, free MiB).
    Also (iii) build lines on F1's stdout, cargo's argv (-v), and the git shim's argv+cwd.
  - git-{a,b,c}-{P0,r2,precheck}-{head,f1}: real git scratch repo (GIT_CONFIG_GLOBAL=/dev/null): (a) .checkpoints/
    commit on call 2, (b) tracked edit of marks/marks.json on call 2, (c) commit under spikes/netcode-proof on call 2.
  - ii-{P0,precheck}-{head,f1}: cargo shim edits marks.json on build 2; then a re-run with the file restored.
  - floor-{P0,precheck,r2}-{head,f1}: the floor-edited copies.
  - lim-run (P0, sleep on run 2: (iv), condition 1 (a), condition 4 judged run + exit line), lim-pair (precheck, sleep
    on the wallclock pair), lim-rec (precheck, sleep on the recording), lim-judge (P2, judge sleeps); pgrep -f after.
  - realjudge: r2's npt-judge (sha256 65acacfe...) on a /dev/shm copy of r2-e59ece9ecafc/whole-as-lost: uncut (control,
    exit 0), then client.jsonl cut mid-line: exit 1 naming file and line.
  - helper: node --test on the extended helper test (INF-R2-1 (a)-(c) pure and on real git in /dev/shm; main folder
    read-only rev-parse; JDG-ONLY-1; E's words).
  - extra: a /dev/shm copy of HEAD's redact-check.mjs on copies of F1's two scripts.
- collect logs to .gate/u4r-f1/logs/, then rm -rf /dev/shm/u4r-f1.

## Progress
- prep done (main folder HEAD then 24d795f; spikes/ unchanged since 7ff3823). Sources and edited copies' sha256s:
  /dev/shm/u4r-f1/logs/sources.log (kept as .gate/u4r-f1/logs/sources.log at collect). f1-limit runs 3c7bc1e4...,
  f1-floor runs 3c2eb8f5... r2 4c0ed845..., head-floor runs e79746ad... r2 c9c2a3a3...
- cmp-{precheck,P0,P2,P3,B1,r2}: all PASS (logs cmp-<set>.log). r2: only calls 40 and 42 differ (--marks, --only).
- git-{a,b,c}-{P0,r2,precheck}-{head,f1}: all 18 PASS (logs git-*.log). Note (Low, cosmetic): sh() trims the porcelain
  line, so (b) quotes "M spikes/netcode-proof/marks/marks.json" (raw " M ...").
- ii-{P0,precheck}-{head,f1} (with re-runs) and floor-{P0,precheck,r2}-{head-floor,f1-floor}: all 10 PASS (logs ii-*.log,
  floor-*.log): F1 leaves no set folder and its re-run completes; HEAD leaves it and its re-run is refused.
- lim-{run,pair,rec,judge}: first run FAILed one check each through the HARNESS's own liveness test (kill(pid, 0)
  succeeds on a zombie: the killed sleeper waits for PID 1 to reap it); fixed to read /proc/<pid>/stat; re-run: all
  PASS, every "stopped at 2.0 s". Added lim-control (no group kill: the sleeper outlives its parent; then the group
  kill ends it): PASS. Logs lim-*.log; summary.log keeps both runs.
- realjudge PASS (r2's npt-judge 65acacfe...: uncut copy exit 0; client.jsonl cut mid-line 451: exit 1 "npt-judge:
  .../client.jsonl: line 451: not one JSON record: EOF while parsing a string at line 1 column 548"); helper PASS
  (# tests 9, # pass 9, # fail 0); redact (extra, F1's two scripts copied) PASS: 0 findings.
- collect done: 44 files in .gate/u4r-f1/logs/ (42 staging logs, sources.log, summary.log; the edited copies in
  logs/edited-copies/). The `rm -rf
  /dev/shm/u4r-f1` was DENIED by the permission system: the 75 MB scratch tree is still there, not worked around; for
  the main session. DONE otherwise; the report went to the main session.
