# NPT-L2-ONEWAY's plan: on the real network, the one-way delay checks read net of half the base round trip
2026-11-26, Netcode Builder 2. For the advisor agent's review; no judge or marks edit before its APPROVE.
Base: the netcode branch's merged head 591e2cb0 (my branch fast-forwarded to it; my two parts are in it at 48efc9c5).
Paths are under `spikes/netcode-proof/`.

## 1. Goal and sources
- **Mustafa's word:** his message to this chat of 2026-11-25, item 4 (`mustafa-2026-11-25.md`, word for word), and his
  answer in the netcode chat (`.checkpoints/netcode-proof-test/history.md:7907`): "(a). No tolerance changes, the
  round-trip checks stay as they are, and the base round trip is the leg's own measured value."
- **The question he answered** (`.checkpoints/netcode-proof-test/advisor-u6c-r1.md:112-121`), option (a): "on L2 every
  one-way check (mean, both ends, the p90) reads against the profile's one-way minus `base_rtt_ns` ÷ 2 from the leg's
  header; L1 is unchanged (base 0), no tolerance changes, the round-trip checks unchanged; a marks.json notes line and
  a judge change with a reviewer's read".
- **Why:** on L2 the model adds (profile round trip − base) ÷ 2 each way (`crates/net/src/l2/config.rs:162-164`,
  `one_way_ns`), and the probe stamps only that added part, so its one-way delays sit half the base below the
  profile's. P1's tightest bounds are ±0.25 ms, so they would fail on that offset alone.
- **Change type:** logic (the judge). Risk tier 1. Fence as before: `crates/judge/`, `marks/`, these notes.

## 2. What is built
- **The rule, in `crates/judge/src/eval.rs` (`eval_check`, beside the `ws_bound` branch that already moves a bound at
  judge time):** a check is a one-way delay check when its measure is the series `second.s.probe_up_delay_ns` or
  `second.s.probe_down_delay_ns` with a statistic of place (mean, min, max or a percentile; never count, sum or
  share) and a fixed bound (`Threshold::Value`). Judged on one run whose header layer is `L2`, its bound in force is
  the written bound minus half of that run's `header.params.base_rtt_ns`, exact (a fraction when the base is odd).
  The check's `needs` text shows both, e.g. `>= 23550000 (the profile's 23750000 less half the base round trip
  400000)`. Nothing else moves: the value, the tolerance (both ends shift together), every other series (the
  round-trip `probe_rtt_ns`, the loss counts), every L1 run (never read net, whatever its header says).
- **Gaps (the run is INVALID, never a silent 0):** an L2 run with no `base_rtt_ns` in the logs the check reads; one
  that is not a whole number or is below 0; logs of the run that disagree on it; such a check pooled over runs
  (none exists today: each run has its own base).
- **Not changed:** reports print the stamped values as they are (P0's recorded part); `against` checks; the marks'
  numbers. **marks.json gains one sentence in PM-0001-3's notes and nothing else**, taken by script from his saved
  message (a new small script `oneway-note.mjs`; it refuses any file that is not `JSON.stringify(x, null, 2)` and any
  second run). No new field on the 80 checks: his answer names "a marks.json notes line and a judge change", so the
  rule lives in the judge and is keyed on the two series' names, as a figure's computation is.
- **Texts:** `marks/README.md` section 2.3 (the rule, its gaps, the `needs` text); `marks/SERIES.md` rows for the two
  series and for `header.params.base_rtt_ns` (the judge now reads it for -3's L2 one-way checks).
- **Symbols:** eval.rs: `one_way_base(check, runs) -> Result<Option<Ratio>, String>` (or folded into `eval_check`),
  the `needs` wording; no public API change.

## 3. Tests and evidence
- **Engine tests on made-up logs (a mark like PM-0001-3 with an L1 and an L2 part sharing the checks):** L2 with
  base 600,000: a stamped lower end of 19,700,000 passes (it fails the unshifted 19,750,000: the case the change is
  for); exactly at each shifted bound passes, 1 ns outside fails, for the mean and both ends; an odd base gives a
  half-ns bound; the same logs on L1 are judged against the written bounds; the round-trip check and a count on the
  same series do not move; a bite run on L2 whose mean is the full round trip each way still fails (shown); the
  `needs` text exactly. Gaps: no base, a text base, a negative base, two logs disagreeing.
- **One test with the real marks.json** (logs built in code: -3's 1,000,000-packet counts are too long for a fixture
  file): an L2 P1 probe run's eight one-way checks per direction read the shifted bounds and pass; its count checks
  fail as they must on a short log.
- **Planted defects, each failing a named test and taken back:** the shift applied on L1 too; the sign flipped; the
  whole base for half; the round-trip series shifted too; a missing base read as 0.
- **Checks on the final head, in this container:** `cargo test -p npt-judge --locked --no-fail-fast`, clippy with
  warnings as errors, `cargo fmt --check`, `--check-marks`, `node spikes/netcode-proof/check.mjs` whole; GitHub's
  gate and toolchain on the final code commit.
- **Review:** one reviewer's read (his words): the **netcode-architect** at xhigh, on the final diff: is the sign and
  the half right against `one_way_ns` and the probe's stamps (`crates/server/src/probe.rs`), do the made-up logs
  look like what the probe writes, and it runs the tests itself. Then the advisor agent's read of the head.
- **Size:** about 250 added lines. My own stop: 400. At the stop I cut and tell Mustafa in three lines.
- **Not verified here:** no real L2 probe log exists in the repository yet; R1's run gives the first.

## 4. Rulings asked of the advisor
1. The rule in the judge, keyed on the two series' names, with no new field in marks.json (my reading of "a marks.json
   notes line and a judge change"): right, or should the 80 checks carry a field?
2. The notes sentence: his own words from his message of 25 November, item 4, in the bracket ("the probe's one-way
   delay checks on the real network read against the profile's one-way delay minus half the measured base round
   trip; no tolerance changes"), tagged with his name and the date, copied by script. Right?
3. Which reviewer for the one read: netcode-architect (my lean), or test-engineer?
4. The model floors (profile − base) ÷ 2 to a whole nanosecond and saturates at 0 when the base is above the
   profile's round trip; the judge takes the exact half. The difference is under 1 ns against a 250,000 ns
   tolerance: accept, and say so in README 2.3?
