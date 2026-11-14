# U4r-F1: the run scripts' scoped re-check, folder order, time limits and build lines (plan for the advisor's review)

Drafted 2026-11-14 on claude/netcode-proof-test-plan-hcky5g (PR #40) at HEAD 7ff3823 (spikes/netcode-proof tree
0b47e881…). Rulings: advisor-u4r-r2-full.md (INF-R2-1 Medium, INF-R2-3, INF-R2-4; next step 13); RUN-L1
(advisor-u4r-r2run-full.md, step 7); the whole-as-lost suite (advisor-u4r-c62h-full.md, step 4). Findings:
u4r-review-r2-full.md:139-165. All file:line below are HEAD's.

## 1. Goal
A checkpoint commit during a long run set must not stop it, and a hung run must not hold the lane lock for good. In plain
words: the end-of-set safety check looks only at the spike's own files (INF-R2-1); a stop before the first run leaves no
folder that blocks a re-run (INF-R2-3); every npt-lab run and the judge get a 3,600 s limit (INF-R2-4); the build's
"Compiling npt-*" lines reach the runner log (RUN-L1); r2's whole-as-lost suite names PM-0001-2 (c62h step 4). Two files
only: spikes/netcode-proof/ops/runs/c-u4r-runs.mjs and c-u4r-r2.mjs. No check.mjs change, nothing else under spikes/.
Lands before any c-u4r-runs.mjs set (r2-full step 13).

## 2. Changes, item by item (ruled words, then the change)
**A. INF-R2-1.** Ruled: "record `git rev-parse HEAD:spikes/netcode-proof` … and `git status --porcelain
--untracked-files=no -- spikes/netcode-proof`. Before the judge (runs) and before r2.txt (r2), compare both … and stop if
either changed. Drop the whole-repository HEAD and status compare … Keep the whole-repository refusal at the start and the
untracked check under crates. runs.txt and r2.txt keep naming the start commit and add the tree hash … Make the compare a
pure exported helper" (r2-full:38).
- A1 new exported pure helper `spikeChanged(start, now)`: each of {tree, status}; returns "" when both are equal, else
  the reason, e.g. `tree bc90e71ee7c3 is now 0b47e881b8d8` and/or `its tracked status now reads: <lines joined "; ">`.
- A2 runs :260-261: `start` gains `tree` and `spike` (the two git calls, through sh() whose cwd is ROOT, :202).
- A3 runs :317-321 `unchanged(what)`: replaced by the helper on a fresh reading; the error becomes
  `spikes/netcode-proof changed since the set started (<reason>): stopped before <what> (INF-R2-1)`. Both callers stay
  (:349 precheck, :366 judge). The start refusal (:251-252) and the crates check (:253-254) are unchanged.
- A4 r2 :158-159 and :209-211: the same, the two git calls with cwd ROOT passed explicitly (r2's sh() runs in SPIKE,
  :110, where the pathspec `spikes/netcode-proof` would match nothing and the status would always read empty).
- A5 runs.txt head :304 `commit <sha>, spikes/netcode-proof tree <hash>`; r2.txt :187 the same words beside the commit.
- A6 module text: runs :47-50, r2 :35-36 say the re-check is scoped to the spike's tree and tracked status.

**B. INF-R2-3.** Ruled: "In c-u4r-runs.mjs: copy marks.json into the bin folder first and check its sha256 there, call
floor() for the first run, and only then mkdirSync(logs) and copy it in as the judge's copy. In c-u4r-r2.mjs: call
floor() for SUITES[0] before mkdirSync(out)" (r2-full:108).
- B1 runs :297-302 become: copy to `<bin>/marks.json`; sha check (message now ends "stopped before the set's folder");
  `runs = planOf(<bin copy>)`; `floor(runs[0].what, runs[0].bytes)`; then `mkdirSync(logs)` and copy the bin copy to
  `<logs>/marks.json` (the judge's copy, as now). The loops' per-run floor (:325, :341, :359) stays, so run 1 is checked
  twice, seconds apart (harmless). runs.txt head :309 reads "checked in the bin folder, then copied into the raw-log
  folder". Module text :35-40, :45-47 follow.
- B2 r2: `floor(\`suite ${SUITES[0].key}\`, SUITES[0].bytes)` moves before :180 `mkdirSync(out)`; the loop's floor
  (:197) stays. Module text :30-36 follows.

**C. INF-R2-4.** Ruled: "timed()'s npt-lab calls and the judge call run through check.mjs's exported run() with a limit:
3,600 s each … A stopped run's runs.txt line reads 'stopped at <s> s'; it is a result, and the set goes on (condition 5),
never retried. The header says whether the lane lock is held" (r2-full:115).
- C1 runs :67 imports `run` too; new `const RUN_LIMIT_MS = 3_600_000` (doc: over five times r5's slowest judged run;
  re-checked against the judge's measured time before the P0 set, a step-17 task, no code).
- C2 runs :233-238 `timed()`: `run(exe, args, ROOT, process.env, RUN_LIMIT_MS)` (same cwd, env and buffer as sh());
  returns `{ out: r.stdout.trim(), secs, stopped: r.timedOut }`, so a finished run's lines stay byte-identical to today's
  (sh() returned stdout alone, :205). A non-zero exit that is not a stop throws sh()'s exact message, as today.
- C3 judged loop :361-364: a stopped run's line is `<run id>: stopped at <s> s (a result, never retried: condition 5)`,
  with no overflow flag; the loop goes on. Pairs :342-347: `## pair <p>: stopped at <s> s, expected <e> (NOT as expected:
  to the advisor)`; the next pair runs.
- C4 the judge :368: through run() with the limit, cwd ROOT, its printed line `r.stdout.trim()` as sh() gave; a
  non-zero exit that is not a stop throws as today (a stop: choice Q3).
- C5 header :6-9: says the set is run under `flock -x` on .gate/shared-target/.lane.lock (as c-u4r-r2.mjs:25-26 says
  for r2: it holds bite runs and its outputs are a record), each npt-lab call and the judge bounded at 3,600 s, the
  builds not (choice Q5).

**D. RUN-L1.** Ruled: "Both scripts print cargo's 'Compiling/Fresh npt-* (path)' lines from stderr into the runner log"
(r2run-full:17). runs :223-230 and r2 :130-137 `build()`: cargo is spawned with sh()'s options but its stderr kept; each
stderr line matching `/^\s*(Compiling|Fresh) npt-/` is printed with console.log as cargo wrote it (main-run.sh:80 greps
`^ *(Compiling|…) npt-`); a non-zero exit throws sh()'s message. The cargo command line is unchanged (choice Q6).

**E. The whole-as-lost suite** (c62h step 4). r2 :87: `marks: "PM-0001-2,PM-0001-17"` (its header and its --only, as
tick16 :84); `expect: "contract PASS; bite PM-0001-17/whole-as-lost shown: mismatches 0 pass, L1 P0 loses no group 48
FAIL; the run valid"`, word for word. The r2 record at e59ece9 is never re-run.

## 3. Risks
- A scoped check misses a change it should catch: no. The runs read only spikes/netcode-proof (bins built from it, marks
  and manifest under it); a commit touching it changes the tree hash, and an uncommitted edit shows in its status.
- r2's cwd trap (A4): a status call left in SPIKE would pass every time. The harness (5.3) runs a tracked edit through
  the real git calls to catch it.
- run() puts each call in its own process group (check.mjs:1902-1906): Ctrl-C on the script no longer reaches npt-lab;
  end it by hand (`pgrep -a npt-lab`). The cargo builds stay unbounded (outside the ruling).
- runs.txt's run lines change only for a stopped run; the judge's and the pairs' lines are unchanged (C2 keeps stdout).

## 4. Change type and tier
Tooling logic in a spike (risk tier 1). Evidence type: logic, a Node harness on the real code. No player-facing change.

## 5. Evidence (kept under .gate/u4r-f1/, the scratch scripts under .checkpoints/netcode-proof-test/u4r-f1/)
1. `node --check` on both scripts: exit 0. `node tools/gate.mjs --no-smoke` (as M7); CI green on the pushed head.
2. The scratch helper test (copy 6's u4r-fp5-helpers.test.mjs, moved into the notes folder, never committed), its 4
   tests kept and new cases: INF-R2-1 as ruled, (a) a .checkpoints/ commit (HEAD moves, tree and spike status equal)
   passes with "", (b) a tracked edit inside (status ` M spikes/netcode-proof/marks/marks.json`) stops, the reason named;
   plus (c) a commit inside (tree differs) stops. Real git, read-only, in the main folder: `git rev-parse
   71ee13d:spikes/netcode-proof e59ece9:spikes/netcode-proof` give the same hash (bc90e71e… today), and the scoped status
   is empty while the whole status shows ` M .checkpoints/…`.
3. The argv harness (scratch, about 120 lines): copies of HEAD's and F1's two scripts, HEAD's check.mjs, marks.json,
   SERIES.md and precheck-recordings.json on /dev/shm (a 16 GiB tmpfs, so every set's projection passes; the disk has
   4.4 GiB), with PATH shims: `git` (a fixed commit and tree, clean status; one mode prints a changed tree on its last
   call), `cargo` (a compiler-artifact JSON line naming a recorder script; on stderr `Compiling npt-lab v0.1.0 (…)`,
   `Fresh npt-sim v0.1.0 (…)` and `Compiling libm v0.2.0`), and the recorder (appends its argv as one JSON line; prints a
   summary line holding "rewinds refused 0" or "result: PASS"; creates --record and --out files). Run on both copies:
   precheck, judged P0, P2, P3 and B1, and r2. Pass: the recorded npt-lab and npt-judge argv lists are equal, line for
   line, except r2's whole-as-lost scenario `--marks` and judge `--only` (PM-0001-17 to PM-0001-2,PM-0001-17); runs.txt
   and r2.txt differ only by the tree words (A5), B1's head wording, and whole-as-lost's expect line and two
   command lines (E). Also on F1's copy:
   (i) the changed-tree mode: the set stops with A3's message, every run done first; (ii) a marks.json edit made by the
   cargo shim during the builds: B1's stop, and no `runs/<set>-<commit12>/` folder left; (iii) the two npt Compiling and
   Fresh lines printed, the libm line not; (iv) in a further scratch copy whose RUN_LIMIT_MS is set to 2,000 by Edit (its
   sha256 recorded), a recorder that sleeps 30 s on run 2: its line reads "stopped at 2.0 s", run 3 follows, and
   `pgrep -f` finds no sleeper left. r2's floor order (B2) is shown by the diff only: no stop can be staged between the
   after-builds projection and the first suite's floor without a mock. Said so in the report.
4. What each new refusal or line prints, quoted in the report from the harness's logs: A3's message (runs and r2), B1's
   sha message, C3's two stopped lines, C4's judge line, D's printed lines.

## 6. Who builds, reviews, checks
- The main session builds it with Edit in the main folder (r2-full step 13: "the main session's build"). The scripts
  refuse a dirty tree (runs :251-252, r2 :154-155), so no set runs during the build. No agent edits the files.
- Reviews: infra-engineer, one round, on the final diff and the evidence; then the advisor reviews the pushed head. One
  commit on the spike branch ("U4r-F1: …"), with the card; the scratch test and harness are never committed.
- Mustafa checks nothing: nothing a player sees, hears or feels changes, and it is not a merge to main.

## 7. Planned size (SIZE-CHECK)
About 100 lines changed in the two files (git diff --stat: about 100 insertions and 50 deletions): runs about 75 (A 20,
B 10, C 30, D 8, comments 7), r2 about 25 (A 10, B 2, D 8, E 2, comments 3). Estimated item by item against HEAD's
lines above. The scratch test and harness (about 200 lines) are not committed. The figure goes on the card.

## 8. Choices the rulings leave to the advisor
- Q1 the helper's home: (a) defined once in c-u4r-runs.mjs, imported by c-u4r-r2.mjs (one copy, one test); (b) one copy
  in each, as roomFor is today (runs :139, r2 :101). Recommended (a).
- Q2 an npt-lab run's non-zero exit that is not a stop: (a) throws, as today (the ruling names only the stop);
  (b) also a result line. Recommended (a).
- Q3 the judge stopped: (a) runs.txt is written with `judge: stopped at <s> s (no verdict)`, no verdict.json, exit 1;
  (b) the script stops and writes nothing to results/. Recommended (a): the run lines are kept as a result.
- Q4 the pre-check recording stopped (no pair can run without it): (a) the set stops, naming it, as carry 40's stop
  does (:333-334, which today would report "refused NaN rewinds"); (b) the three pairs run anyway. Recommended (a).
- Q5 "the header says whether the lane lock is held": (a) the module header states the set is held under `flock -x`
  (C5); (b) a runs.txt head line too, from a new required option, which changes the usage. Recommended (a).
- Q6 "Fresh" lines: no 'Fresh npt-' line appears in any kept log under .gate/u4r and .gate/u4r-merge (grep), so cargo
  writes them only with -v (not checked in cargo's source). (a) print what cargo writes now (an all-fresh build prints
  none); (b) add `-v` to the build so Fresh lines appear (more stderr, filtered). Recommended (a); r2.txt's binaries'
  sha256s already identify the build (r2run-full:17).
