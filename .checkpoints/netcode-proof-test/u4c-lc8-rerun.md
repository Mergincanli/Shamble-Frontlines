# U4c's L-c8, its judged half: PM-0001-3's L1 set re-run on the merged build (the main session, 2026-11-11)

**The test** (u4-build-plan.md §5 U4c, :591-593): "L-c8 -3's regression: its five L1 parts re-run on the new build and
re-judged PASS (after U4j merges, so C4 applies), its four bites failing as before (advisor-u2.md:20-22); and for P2, B1
and P4 seed 1, SHA-256 of (the v1 tag + the v2 log's datagram entries) equals U2's committed 913db8e9, feabf2f0 and
b8a04f2f (advisor-u2.md:31-32)." Its unit half is U4c's test
`v1s_tag_and_the_v2_logs_datagram_entries_hash_as_v1s_log_did` (merged at f3f7c2f). This half was due after U4j's merge
(advisor-u4j-r2.md M9).

**The run.** U2's own script, unchanged: `node spikes/netcode-proof/ops/runs/c-u2-pm-0001-3.mjs --date 2026-11-11`, from
the main folder at 6586991 (spikes/ equal to d50396a: U4c's model and U4j's judge, both merged), with tracked files
clean (the script refuses otherwise). It builds into the main folder's own cache (.gate/netcode-proof-test/target, from
the spike's .cargo/config.toml; release builds); no other cargo ran. 20:15:19-20:17:11 UTC: 38 runs (15 measured, their
15 twins, the 4 bites and their twins), then the judge on PM-0001-3. Results: spikes/netcode-proof/results/
c-u2-pm-0001-3/65869918809f/ (verdict.json; runs.txt with each binary's sha256: npt-lab 4c765111…, its bites build
5d272592…, npt-judge 01066078…). Raw logs: .gate/netcode-proof-test/runs/c-u2-pm-0001-3-65869918809f/ (not
committed: plan §10.6). Log of the run: .gate/u4j-merge/lc8.log.

**Compared with U2's set (81c6d01e371b), by script** (read only; the scratchpad's lc8_compare.py and a bite pass):
- The five L1 parts PASS, as at U2: P1 to P4 75 checks each (3 runs of 25), B1 81 (3 runs of 27); every check's outcome
  and value equal to U2's. The L2 parts and MP are NOT-RUN and the mark NOT-RUN, with the same reasons, as at U2.
- The four bites are shown, each failing exactly U2's checks with U2's values: rtt-each-way 9 (mean, the 0.1st and
  99.9th percentiles and p90 high, both ways, and the round trip's mean high); loss-per-round-trip 2 (loss low, both
  ways); normal-jitter 2 (p90 low, both ways); clock-seed 1 (the twin check only; its numbers differ from run to run by
  design, 22 of its 25 values). not-net-of-base: not run (an L2 bite), as at U2. These are advisor-u2.md:20-22's
  outcomes.
- runs.txt: every run's summary (sent, lost each way, the mean one-way delays, the round trip's mean and its sample
  count, the reorder counts) equals U2's, but for the fate log's hash and the clock-seed bite's numbers. The hash
  differs because the log is v2 now (U4c's C1': the acknowledgements' own stream and entries), and npt-lab prints the
  SHA-256 of the whole canonical log (lab/src/probe.rs:210-241).

**The hash half.** npt-lab still records only the whole log's hash: U4c's netcode round-1 finding F2 (Medium) stays open
("No code path writes SHA-256(v1 tag + datagram entries) from an npt-lab run; crates/lab is outside U4c's lane":
u4c-review-netcode.md:53-54; in wave 2 crates/lab is U4d2's alone: u4-build-plan.md :842, :862). What these runs show:
P2, B1 and P4 seed 1's whole v2 logs hash to 43745683ab36d67b, 722f20713e18352e and 432693efb9698c05, the values U4c's
builder recorded for its scratch logs of the same runs (u4c-build.md:79-84). On those logs' bytes the builder computed
SHA-256 of (the v1 tag + the datagram entries) equal to U2's 913db8e9…, feabf2f0… and b8a04f2f…, and U4c's test reviewer
reproduced that in its own build (u4c-review-test.md:95-99). Equal whole-log hashes mean equal logs, so the hash half
holds through the builder's scratch, not through an npt-lab line of its own. Proposed route for F2, for the advisor to
rule: a carry for U4d2 (crates/lab's lane in wave 2), so that a later L1 re-run shows it directly.

**Result:** L-c8's judged half PASS; its hash half holds through the scratch (F2 open, route proposed).
