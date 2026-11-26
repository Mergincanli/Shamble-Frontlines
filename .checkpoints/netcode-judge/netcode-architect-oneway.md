# netcode-architect: review of NPT-L2-ONEWAY (591e2cb0..1a637408), 2026-11-26
## Plan
1. Read the diff (eval.rs, marks.rs, tests/engine.rs, README 2.3, SERIES.md, marks.json).
2. Point 1: config.rs one_way_ns, probe.rs stamps, ImpairSocket stamps in crates/net/src/l2/.
3. Point 2: how probe_rtt_ns is measured on L2.
4. Point 3: test logs vs the probe's real output (process, layer, params.base_rtt_ns per leg, run/log count).
5. Point 4: scratch copy in .gate/na-oneway/, cargo test, plants (a)-(e), two of my own.
6. Also: PM-0001-3's checks by stat, P0 and B1 parts, other marks reading the series, texts vs code, marks.json diff.
## Progress
