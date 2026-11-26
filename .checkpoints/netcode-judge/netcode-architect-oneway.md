# netcode-architect: review of NPT-L2-ONEWAY (591e2cb0..1a637408), 2026-11-26
## Plan
1. Read the diff (eval.rs, marks.rs, tests/engine.rs, README 2.3, SERIES.md, marks.json).
2. Point 1: config.rs one_way_ns, probe.rs stamps, ImpairSocket stamps in crates/net/src/l2/.
3. Point 2: how probe_rtt_ns is measured on L2.
4. Point 3: test logs vs the probe's real output (process, layer, params.base_rtt_ns per leg, run/log count).
5. Point 4: scratch copy in .gate/na-oneway/, cargo test, plants (a)-(e), two of my own.
6. Also: PM-0001-3's checks by stat, P0 and B1 parts, other marks reading the series, texts vs code, marks.json diff.
## Progress
- Block 1 (diff read): eval.rs one_way_bound + eval_check; marks.rs ONE_WAY_SERIES, one_way_place, loader refusal; 4 tests; texts.
- Block 2 (point 1): stamp = out - t_in on RS's own ImpairSocket clock (net/l2/probe.rs:780, socket.rs:10-21); up t_in at the
  reader after the real path, down out at try_send: real path NOT in the stamp, no inter-machine clock. Model:
  due = t + one_way + U[-a,a] (impair.rs:121-123, :553), one_way = (rtt - base)/2 floored (config.rs:162-164). Every
  statistic of place shifts by base/2: rule right, sign minus, half. Ignored: release lateness L (+, tiny, same as before).
- Block 3 (point 2): probe_rtt_ns = up release - down t_in - LG hold (net/l2/probe.rs:788): spans the real path both ways. Yes.
- Block 4 (point 3): one log per leg, run <run>-<leg>, process probe, layer L2 (server/probe.rs:576-580); params.base_rtt_ns
  in every leg's header via header_numbers (socket.rs:338); P0's = 0 (base set after P0, probe.rs:389-409); P1-P4,B1 = P0's
  median. LG's log (process load) carries no probe series. Fixtures match in shape.
- Block 5 (point 4): baseline 19+4+87+26 = 136 pass in scratch copy.
- Block 6: plants a,b,c each fail >=1 oneway_ test (c: the real-marks test alone survives; needs only ends_with)
- Block 7: plants d,e fail >=1 oneway_ test; my plant f (drop the carries-the-series filter) SURVIVES all 136
- Block 8: own plants g (gap loosened), i (no disagree check), j (against allowed), k (down dropped) each fail >=1 oneway_ test; h (Min|Max dropped) SURVIVES (no check or test uses min/max). marks.json: only -3's notes changed, prefix kept, quotes verbatim, stringify-2 exact.
- Block 9: fmt --check clean, clippy -D warnings clean (scratch). Head 1a637408 pushed, no skip marker. No mark names
  pre:PM-0001-3; no other mark reads the two series; reports go through report_values (eval.rs:1658), never eval_check.
## Findings (all Low; none changes a verdict a real run would get)
- L1 (Low, test gap) tests/engine.rs:5067-5084: plant f (drop the "carries the series" filter, eval.rs:485-486) and
  plant h (drop Stat::Min|Max, marks.rs:308) survive all 136. Fix: add to one_way_mark a row (UP, "min", ">=", 19_750_000)
  and assert its needs is net; add to the first test a second L2 log in run p1 (process "load", no probe series, no
  base) and assert nothing changes.
- L2 (Low, test strength) tests/engine.rs:5286-5288: the real-marks test asserts only needs.ends_with(...400000)"), so
  plant c (whole base) passes it (0.2 ms below sits inside both windows). Fix: pin one needs exactly, e.g.
  ">= 19550000 (the profile's 19750000 less half the base round trip 400000)".
- L3 (Low, wording) eval.rs:471,515 and README.md:198: "the model adds no delay there" is false for a bound below
  the profile's one-way d (the low ends d-a-5%a): at base 40 ms on P1 the model still adds 5 ms +- 5 ms. Fix: "the bound
  in force would be below 0 ns, so the check has no bound to read".
- L4 (Low, wording) eval.rs:584 and README.md:194: "the profile's 23750000" names the written bound (the profile's
  25 ms less 5 %), not the profile's value. Fix: "the written 23750000 less half the base round trip 400000".
- L5 (Low, pre-existing, outside the diff) marks.json PM-0001-3 part "L2 P0 (the real path, recorded)" and its four P0
  reports: on P0 the stamps hold only the pass-through socket's own hold (out - t_in on RS), never the real path, and
  P0's header base_rtt_ns is 0 (probe.rs:389-409). Unshifted is right; the label misleads. Follow-up: rename, or point
  readers to the base RTT report. Also: the release lateness L (out - due) sits in every L2 stamp, unnetted, as before.
## Verdict: APPROVE WITH FINDINGS
