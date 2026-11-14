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
