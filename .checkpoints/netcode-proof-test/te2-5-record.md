# TE2-5 (b): the bites build's workspace tests, ten times (the main session, 2026-11-11)

**What the ruling asks** (advisor-u4j-r2.md B2.3 (b)): before the next merge into the spike branch after U4j's, "the
bites build's workspace tests, as check.mjs's test step runs them but with `--no-fail-fast` (`cargo test --locked
--workspace --all-features --no-fail-fast`), ten times through the shared-target runner, each run's whole output saved
under .gate/te2-5/. Any failure's name and panic go into the notes, and that test's cause is found and fixed in a
planned change of its own. If none fails in ten, that is recorded, and the rule below stays until a name is caught."

**The runs.** te2-5-runs.sh: ten calls of main-run.sh in turn (each holding the shared target's lane lock exclusively,
idle at its start, 9 GB free, the 2 s touch loop on crates/, the shared target, offline), from the main folder's
spikes/netcode-proof at 9e0f103 (spikes/ equal to 9cd7c39's: U4j merged at d50396a, plus L-c8's results). Nothing else
built or ran tests meanwhile (no builder; the one agent running, the advisor, was told to run nothing). 20:20:26-20:41:12
UTC; logs .gate/te2-5/run1.log to run10.log.

**Results** (each run's exit code, and the sums of its "test result:" lines, the test runner's own summary per binary):

| Run | Ended (UTC) | Exit | Test binaries | Passed | Failed | Ignored |
|---|---|---|---|---|---|---|
| 1 | 20:22:32 | 0 | 47 | 443 | 0 | 12 |
| 2 | 20:24:37 | 0 | 47 | 443 | 0 | 12 |
| 3 | 20:26:39 | 0 | 47 | 443 | 0 | 12 |
| 4 | 20:28:42 | 0 | 47 | 443 | 0 | 12 |
| 5 | 20:30:45 | 0 | 47 | 443 | 0 | 12 |
| 6 | 20:32:51 | 0 | 47 | 443 | 0 | 12 |
| 7 | 20:34:57 | 0 | 47 | 443 | 0 | 12 |
| 8 | 20:37:04 | 0 | 47 | 443 | 0 | 12 |
| 9 | 20:39:08 | 0 | 47 | 443 | 0 | 12 |
| 10 | 20:41:12 | 0 | 47 | 443 | 0 | 12 |

443 passed is check.mjs's bites count at d50396a (u4j-build.md, the merge's M5). The 12 ignored are `#[ignore]` tests,
which check.mjs's "reported a skip" does not count.

**Recorded, as the ruling asks:** none failed in ten, on a quiet machine. The failure U4j's fixer saw once (its check.mjs
run 2, 12:39 UTC, an npt-net library test on the bites build, unnamed) did not recur here. What else ran on the machine
at 12:39 UTC is not recorded in these notes, so the runs do not tell whether load matters. The cause stays unknown, and
TE2-5's rule stays in force until carry 65 is met (check.mjs names every failing test) or a failure is caught by name.
