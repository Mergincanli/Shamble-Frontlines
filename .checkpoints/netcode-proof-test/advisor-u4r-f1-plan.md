# Advisor: plan review of U4r-F1 (PR #40, claude/netcode-proof-test-plan-hcky5g, plan at HEAD 7ff3823)

## Plan of this review
1. Read advisor.md from main (done), the plan (done).
2. Read the rulings: r2-full (INF-R2-1, -3, -4, step 13), r2run-full (step 7, RUN-L1), c62h-full (step 4, JDG-ONLY-1
   interim), the findings u4r-review-r2-full:139-165.
3. Read c-u4r-runs.mjs and c-u4r-r2.mjs at HEAD; check.mjs run(); the scratch helper test.
4. Item by item A-E against rulings and code; risks; evidence; size; panel; Q1-Q6; verdict.

## Notes (a line per item as I go)
- Read: rulings r2-full (INF-R2-1 :38-41, -3 :108, -4 :115, step 13 :19), r2run-full step 7 (RUN-L1 :17), c62h-full
  step 4 (:45) and JDG-ONLY-1 interim (:20); findings :139-165; both scripts at HEAD 7ff3823 (no diff under spikes/).
- Line refs in the plan check out against HEAD (runs :202 sh cwd ROOT, :233-238 timed, :251-254, :260-261, :297-302,
  :304, :309, :317-321, :325, :341, :349, :359, :366, :368; r2 :110 sh cwd SPIKE, :130-137, :154-159, :180, :197,
  :209-211, :87).
- check.mjs run() :1909-1922 returns {ok,out,stdout,status,timedOut}; detached + SIGKILL group when timeout. builtArtifact
  :2213 skips non-'{' lines, so -v's stderr cannot reach it (stdout only anyway).
- npt-judge main.rs:52-124 exits 0 whatever the verdicts: C4's "non-zero = throw" only hits real errors. Good.
- main-run.sh:47-51 touches every crates/ file first and every 2 s: under it npt-* always Compile, never Fresh. So the
  plan's Q6 grep (no Fresh line in kept logs) proves nothing about -v; r2 runs through main-run.sh (Compiling lines
  always), runs.mjs does not (header :6-7), so a fresh runs.mjs build prints NO provenance line under Q6 (a).
- git: HEAD:<path> is root-relative from any cwd (no ./); the status pathspec is cwd-relative: plan's A4 note correct.
- Tree hashes the plan cites verified: 71ee13d and e59ece9 spikes/netcode-proof = bc90e71e…; HEAD and 5e563b9 =
  0b47e881…; (b57488ad… in r2run-full is spikes/'s tree, also right).
- A (INF-R2-1): as ruled. Gaps: (1) plan risk says "the harness (5.3) runs a tracked edit through the real git calls",
  but 5.3's git is a SHIM: nothing then catches r2's cwd trap or shows the headline case (a .checkpoints/ commit mid-set
  passes) end to end. Needs real git in a scratch repo for the A stagings. (2) start.status (whole) left as dead data:
  drop it. (3) The tree hash covers results/: committing an earlier set's results/ while a set runs stops it (as ruled,
  but the plan's risks must say it and the card's procedure: commit results before the next set).
- B (INF-R2-3): runs as ruled; (ii) stages the sha stop. r2 "diff only" is less than ruled ("a helper-test case or a
  by-hand stop that leaves no set folder"): stage it in a scratch copy whose floor() refuses "suite p0" (Edit, sha256
  recorded, as (iv)); run the stagings on HEAD's copy too (folder left = window reached).
- C (INF-R2-4): HIGH-class gap. npt-lab's logs are BufWriter, flushed only at the footer (logs.rs:82, log lib.rs:507-514);
  a SIGKILLed run leaves cut or empty .jsonl files; read_logs refuses any malformed log (judge lib.rs:36-45; read_log
  :580 "not one JSON record", :645-648 "empty log"); judge exits 1 (main.rs:99-104). Under C4 that throws: no runs.txt,
  the ruled 'stopped at' line never reaches runs.txt, the set's verdict lost. The plan's judge shim exits 0, so the
  evidence would hide it. Fix: once the runs are done runs.txt is always written; judge line = summary / stopped /
  exit <n> (no verdict); verdict.json only on exit 0; exit 1 otherwise. Stage it.
- npt-judge exits 0 on any verdict (main.rs:52-124), so the extension only touches real refusals.
- Q6: (a) leaves c-u4r-runs.mjs (outside main-run.sh, no toucher) with no provenance line on a fresh build: less than
  ruled. Rule (b) -v; confirm at the pre-check set's log.
- E: word for word as c62h step 4. JDG-ONLY-1 interim rule: after E every r2 suite's --only holds PM-0001-2; runs'
  listedOf and marksOf hold it.
- redact-check scans the spike folder (check.mjs:2630-2633), these two files included: run it alone on the edited tree.
- Size ~100 changed lines: my own count A~30, B~15, C~43, D~20, E~1 -> ~110; honest. Panel infra-engineer one round:
  as ruled (step 13). Tier 1; nothing for Mustafa.

## Verdict: APPROVE WITH CONDITIONS (1-8, as returned)
Q1 (a) one copy in runs, imported by r2 · Q2 (a) throw as today · Q3 (a) extended by condition 1 to a non-zero judge
exit · Q4 (a) with an exact message, no runs.txt (as carry 40) · Q5 (a) with the flock form in the header · Q6 (b) -v.
Conditions: 1 judge end always writes runs.txt (stopped/exit lines; verdict only on exit 0; exit 1) + shim mode + one
real-judge check on a cut copy; 2 real git in a scratch repo for INF-R2-1's three stagings (HEAD's copy stops on the
.checkpoints/ commit, F1's passes); 3 INF-R2-3 floor stop staged on both scripts, HEAD's copies too; 4 every new stop
line staged in the 2,000 ms copy; 5 -v and a first-use provenance check at the pre-check set; 6 risks/procedure: results/
commit mid-set stops it, pgrep -a npt-lab after a stopped set; 7 drop start.status; 8 redact-check alone + JDG-ONLY-1
assert in the scratch test. Nothing to Mustafa.
