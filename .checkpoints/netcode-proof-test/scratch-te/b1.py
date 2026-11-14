import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

marks = []

# ---------------- PM-0001-1 ----------------
marks.append(M(
    "PM-0001-1", "The rig is named and the measuring unit holds", "pass",
    ["C"] + ALL_VMS + ["ML*", "MAC-r"],
    [P("RS, single-room runs", {"machine.name": "RS", "process": "room"}, 1),
     P("LG", {"machine.role": "LG"}, 1),
     P("ML (the named laptop)", {"machine.name": "ML"}, 1),
     P("RD", {"machine.role": "RD"}, 1),
     P("MAC", {"machine.role": "MAC"}, 1)],
    [],
    [F("every machine named (README §1's machine fields filled; a VM's day address set)", "machine_named", "==", 1),
     C("tick thread alone on its vCPU, at start and end", "event.affinity.alone", "min", "==", 1,
       window="all", **{"from": ROOM, "parts": ["RS, single-room runs"]}),
     C("both affinity reads present (start and end)", "event.affinity.alone", "count", ">=", 2,
       window="all", **{"from": ROOM, "parts": ["RS, single-room runs"]}),
     C("tick thread's SMT sibling free of the transport runtime", "event.affinity.sibling_free", "min", "==", 1,
       window="all", **{"from": ROOM, "parts": ["RS, single-room runs"]}),
     C("steal recorded every second", "second.v.steal_permille", "count", ">=", 1,
       window="all", **{"from": {"machine.role": "RS"}, "parts": ["RS, single-room runs"]})],
    [R("steal per second, max", "second.v.steal_permille", "max", **{"from": {"machine.role": "RS"}}),
     ],
    [B("PM-0001-1/shared-vcpu", "a single-room run with a second room's tick thread on the same vCPU: the affinity read must say not alone")],
    ["R-21"],
    "A precondition of every server mark; its checks run in every single-room run's log. -27's and -63's unpinned runs do not list it (they record each process's affinity under PM-0006-13). Not V-S: the bite must FAIL this mark's own affinity check, and V-S would make it INVALID instead. The same bite run is the judge fixture that shows V-S making a -4 run INVALID. ML, RD and MAC parts wait for the named machines (plan §6.3: ML* is a rough check, RD has no stand-in), so -1 stays NOT-RUN until R8. The clock resolution is a machine field (`machine.clock_res_ns`, README §1), which no series form reads: the judge's verdict detail should print it (question)."))

# ---------------- PM-0001-2 ----------------
ss = ["SS runs"]
es = ["ES runs"]
marks.append(M(
    "PM-0001-2", "The load is the stress scene", "pass",
    ALL_VMS,
    [P("SS runs", {"scene": "SS"}, 3), P("ES runs", {"scene": "ES"}, 3)],
    [],
    [C("48 soldiers on every measured tick", "tick.v.soldiers_count", "min", ">=", 48, **{"from": ROOM, "parts": ss + es}),
     C("three teams of 16", "tick.v.smallest_team_count", "min", ">=", 16, **{"from": ROOM, "parts": ss}),
     C("a connected snapshot-receiving client on every soldier", "tick.v.snapshot_clients_count", "min", ">=", 48, **{"from": ROOM, "parts": ss + es}),
     C("15 big vehicles", "tick.v.big_vehicles_count", "min", ">=", 15, **{"from": ROOM, "parts": ss + es}),
     C("4 very large units of at least 20 parts", "tick.v.very_large_units_count", "min", ">=", 4, **{"from": ROOM, "parts": ss}),
     C("300 live projectiles", "tick.v.projectiles_count", "min", ">=", 300, **{"from": ROOM, "parts": ss}),
     C("700 shots per second, run mean at least -5 %", "second.v.shots_count", "mean", ">=", "665", **{"from": ROOM, "parts": ss}),
     C("700 shots per second, run mean at most +5 %", "second.v.shots_count", "mean", "<=", "735", **{"from": ROOM, "parts": ss}),
     C("portal changes 1 per second, mean at least -5 % (proposed band)", "second.v.portal_changes_count", "mean", ">=", "0.95", **{"from": ROOM, "parts": ss}),
     C("portal changes 1 per second, mean at most +5 % (proposed band)", "second.v.portal_changes_count", "mean", "<=", "1.05", **{"from": ROOM, "parts": ss}),
     F("everyone in view on at least 99 % of each client's snapshots", "everyone_in_view_min_share", ">=", "0.99"),
     C("ES: 72 shots per second, mean at least -5 %", "second.v.shots_count", "mean", ">=", "68.4", **{"from": ROOM, "parts": es}),
     C("ES: 72 shots per second, mean at most +5 %", "second.v.shots_count", "mean", "<=", "75.6", **{"from": ROOM, "parts": es}),
     C("ES: 10 reliable events per second, mean at least -5 %", "second.v.reliable_events_count", "mean", ">=", "9.5", **{"from": ROOM, "parts": es}),
     C("ES: 10 reliable events per second, mean at most +5 %", "second.v.reliable_events_count", "mean", "<=", "10.5", **{"from": ROOM, "parts": es}),
     C("ES: 32 live projectiles, mean at least -5 %", "tick.v.projectiles_count", "mean", ">=", "30.4", **{"from": ROOM, "parts": es}),
     C("ES: 32 live projectiles, mean at most +5 %", "tick.v.projectiles_count", "mean", "<=", "33.6", **{"from": ROOM, "parts": es}),
     C("ES: 0.44 spawns per player per second (21.12/s), mean at least -5 %", "second.v.projectile_spawns_count", "mean", ">=", "20.064", **{"from": ROOM, "parts": es}),
     C("ES: 0.44 spawns per player per second (21.12/s), mean at most +5 %", "second.v.projectile_spawns_count", "mean", "<=", "22.176", **{"from": ROOM, "parts": es})],
    [R("effects per soldier, max", "tick.v.effects_per_soldier_max_count", "max", **{"from": ROOM}),
     R("stand-in entity changes per second, mean", "second.v.stand_in_changes_count", "mean", **{"from": ROOM}),
     R("stand-in entity spawns per second, mean", "second.v.stand_in_spawns_count", "mean", **{"from": ROOM})],
    [B("PM-0001-2/fire-stops", "a load script that stops firing after 10 minutes fails the 700 shots per second")],
    [],
    "Runs: every run of -4 to -27, -39 and -63 lists it (the 60 Hz ones are outside). The stand-ins (ADR 0004's S1: 384 effect rows, the 1 ms burn, the top-up to 2.80 ms) are named in `header.params.stand_ins`; a list cannot be checked by a stat, so the judge reports it. The portal band of +-5 % is proposed (the row names no tolerance; the shots and ES rates have +-5 %). 'Everyone in view' is per client, so it is a figure, not a pooled share. The bite run must run past 10 minutes (an S30): in a 5-minute run the defect never shows. ES's 'all 15 vehicles in view' is in the same flag (`tick.s.everyone_in_view` reads soldiers in SS, soldiers and vehicles in ES)."))

# ---------------- PM-0001-3 ----------------
PROF = {  # profile: (one-way delay ns, half jitter ns, loss as (lo, hi), rtt ns)
    "P1": (25 * MS, 5 * MS, ("0.0095", "0.0105"), 50 * MS),
    "P2": (50 * MS, 10 * MS, ("0.019", "0.021"), 100 * MS),
    "P3": (75 * MS, 15 * MS, ("0.0475", "0.0525"), 150 * MS),
    "P4": (100 * MS, 20 * MS, ("0.0475", "0.0525"), 200 * MS),
}
parts3 = []
for prof in ["P1", "P2", "P3", "P4", "B1"]:
    parts3.append(P(f"L1 {prof}", {"layer": "L1", "profile": prof}, 1))
    parts3.append(P(f"L2 {prof}", {"layer": "L2", "profile": prof, "process": "probe"}, 1))
parts3.append(P("L2 P0 (the real path, recorded)", {"layer": "L2", "profile": "P0", "process": "probe"}, 1))
parts3.append(P("MP at P2", {"machine.role": "MP", "profile": "P2"}, 1))
checks3 = []
for prof in ["P1", "P2", "P3", "P4", "B1"]:
    d, a, (llo, lhi), rtt = PROF["P2" if prof == "B1" else prof]
    pp = [f"L1 {prof}", f"L2 {prof}"]
    sfx = " (outside the burst)" if prof == "B1" else ""
    for dr in ["up", "down"]:
        dl = f"second.s.probe_{dr}_delay_ns"
        ls = f"second.s.probe_{dr}_lost"
        checks3 += [
            C(f"{prof} {dr}: 1,000,000 packets", ls, "count", ">=", 1_000_000, parts=pp),
            C(f"{prof} {dr}: loss within 5 % of the profile's{sfx}, low", ls, "share", ">=", llo, where={"op": "==", "value": 1}, parts=pp),
            C(f"{prof} {dr}: loss within 5 % of the profile's{sfx}, high", ls, "share", "<=", lhi, where={"op": "==", "value": 1}, parts=pp),
            C(f"{prof} {dr}: mean one-way delay within 5 %, low", dl, "mean", ">=", d - d // 20, parts=pp),
            C(f"{prof} {dr}: mean one-way delay within 5 %, high", dl, "mean", "<=", d + d // 20, parts=pp),
            C(f"{prof} {dr}: jitter's lower end within 5 % of the half jitter, low", dl, "min", ">=", d - a - a // 20, parts=pp),
            C(f"{prof} {dr}: jitter's lower end within 5 % of the half jitter, high", dl, "min", "<=", d - a + a // 20, parts=pp),
            C(f"{prof} {dr}: jitter's upper end within 5 % of the half jitter, low", dl, "max", ">=", d + a - a // 20, parts=pp),
            C(f"{prof} {dr}: jitter's upper end within 5 % of the half jitter, high", dl, "max", "<=", d + a + a // 20, parts=pp),
            C(f"{prof} {dr}: uniform shape, p90 within 5 % of the half jitter of d + 0.8a, low", dl, "p90", ">=", d + a * 4 // 5 - a // 20, parts=pp),
            C(f"{prof} {dr}: uniform shape, p90 within 5 % of the half jitter of d + 0.8a, high", dl, "p90", "<=", d + a * 4 // 5 + a // 20, parts=pp),
        ]
        if prof == "B1":
            checks3.append(C(f"B1 {dr}: every packet inside the 1 s burst lost", f"second.s.probe_burst_{dr}_lost", "min", "==", 1, parts=pp))
    checks3 += [
        C(f"{prof}: round trip mean within 5 %, low", "second.s.probe_rtt_ns", "mean", ">=", rtt - rtt // 20, parts=pp),
        C(f"{prof}: round trip mean within 5 %, high", "second.s.probe_rtt_ns", "mean", "<=", rtt + rtt // 20, parts=pp),
        F(f"{prof} L1: the same seed and profile give an identical packet-fate log", "fate_log_twin_equal", "==", 1, parts=[f"L1 {prof}"]),
    ]
checks3 += [
    C("MP: server-measured RTT median within 100 +- 5 ms, low", "second.v.person_rtt_ns", "p50", ">=", 95 * MS, **{"from": ROOM, "parts": ["MP at P2"]}),
    C("MP: server-measured RTT median within 100 +- 5 ms, high", "second.v.person_rtt_ns", "p50", "<=", 105 * MS, **{"from": ROOM, "parts": ["MP at P2"]}),
]
marks.append(M(
    "PM-0001-3", "The network profiles are what they say", "pass",
    ["C", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "ML*", "MP"],
    parts3, [], checks3,
    [R("P0 on L2: up delay mean", "second.s.probe_up_delay_ns", "mean", parts=["L2 P0 (the real path, recorded)"]),
     R("P0 on L2: up delay p99", "second.s.probe_up_delay_ns", "p99", parts=["L2 P0 (the real path, recorded)"]),
     R("P0 on L2: down delay mean", "second.s.probe_down_delay_ns", "mean", parts=["L2 P0 (the real path, recorded)"]),
     R("P0 on L2: down delay p99", "second.s.probe_down_delay_ns", "p99", parts=["L2 P0 (the real path, recorded)"]),
     R("L2: the path's base RTT (median of 10 s of pings)", "header.params.base_rtt_ns", "max"),
     R("MP: his own base RTT", "header.params.base_rtt_ns", "max", parts=["MP at P2"]),
     R("measured reorder share", "header.params.reorder_permille", "max")],
    [B("PM-0001-3/rtt-each-way", "the full round trip applied each way fails the mean-delay band"),
     B("PM-0001-3/loss-per-round-trip", "loss drawn once per round trip fails the per-direction loss"),
     B("PM-0001-3/normal-jitter", "a normal jitter in place of the uniform fails the jitter check"),
     B("PM-0001-3/clock-seed", "the RNG seeded from the clock fails the fate-log identity"),
     B("PM-0001-3/not-net-of-base", "on L2, the delay not net of the base RTT fails MP's 100 +- 5 ms")],
    ["R-5", "R-6", "R-9", "R-12", "R-17"],
    "Once per profile and layer before any mark under it counts, and at the start of every L2 block (each block's probe is a run that lists -3; L1 at U2). L2's count runs on the probe: a free slot port with its own ImpairSocket, 10,000 packets/s per direction, one-way delays stamped on RS's clock (`process` = probe). L2's injected delay is (profile RTT - base RTT) / 2 (R-9), base RTT = median of 10 s of pings at session start. Jitter (my reading, question): 'within 5 % of a uniform +- a' is checked as both ends of the delay range within 5 % of a from d -+ a, plus the uniform's p90 (d + 0.8a) within 5 % of a; the p90 is what catches a normal jitter whose range happens to match. B1: P2's values over the packets outside the 1 s burst (the probe writes burst packets into `probe_burst_*`), and every burst packet lost. L1's twin: a second run with role `twin`, the same seed and profile, named in `params.twin`. The MP part reads the room's RTT for the person's slot (`params.person_slot`)."))

# ---------------- PM-0001-4 ----------------
P4 = [P("S30 at P2", S30_P2, 3)]
marks.append(M(
    "PM-0001-4", "The server tick budget holds at 30 Hz", "pass", ["R2"], P4, ["V-S"],
    [C("tick p99 at most 16.67 ms", "tick.v.work_ns", "p99", "<=", 16_670_000, **{"from": ROOM}),
     C("tick max under 33.33 ms, whole run", "tick.v.work_ns", "max", "<", 33_330_000, window="all", **{"from": ROOM}),
     C("0 overruns, whole run", "tick.v.overrun", "sum", "==", 0, window="all", **{"from": ROOM}),
     C("0 dropped time, whole run", "tick.v.dropped_ns", "sum", "==", 0, window="all", **{"from": ROOM})],
    [R("tick p50", "tick.v.work_ns", "p50", **{"from": ROOM}),
     R("sim step p99 (recorded beside it: the stand-in label's input)", "tick.v.sim_ns", "p99", **{"from": ROOM}),
     R("sim step mean", "tick.v.sim_ns", "mean", **{"from": ROOM})],
    [B("PM-0001-4/B-S", "rule B-S: a per-tick busy-wait in the tick's work of (16.67 ms - measured p99 + 1.667 ms)")],
    [],
    "3 x S30 on one vCPU of RS (R2, VS2), SS at P2 on LG. Header `params.sim` = `stand-in` while the run's sim step time is below the sim core spike's measured step time for the stress scene (or that figure does not exist yet): the pass is then STAND-IN until slice 6 (key 1449-1452). Thresholds as the row writes them (16.67 ms, not 1/60 s). A p99 reads measured ticks; the max, overrun and dropped-time counts read the whole run, joins included (key 1477-1478).",
    stand_in=True))

# ---------------- PM-0001-5 ----------------
marks.append(M(
    "PM-0001-5", "Its two shares hold, each on its own", "pass", ["R2"], P4, ["V-S"],
    [C("sim step p99 at most 8.33 ms", "tick.v.sim_ns", "p99", "<=", 8_330_000, **{"from": ROOM}),
     C("server-only work p99 at most 8.33 ms", "tick.v.server_only_ns", "p99", "<=", 8_330_000, **{"from": ROOM})],
    [R("lag-compensation queries per tick, p99 (inside the step, counted server-only)", "tick.v.lagcomp_ns", "p99", **{"from": ROOM})],
    [B("PM-0001-5/B-S-sim", "rule B-S in the sim step share: busy-wait of (8.33 ms - its measured p99 + 0.833 ms)"),
     B("PM-0001-5/B-S-server-only", "rule B-S in the server-only share: busy-wait of (8.33 ms - its measured p99 + 0.833 ms)")],
    [],
    "PM-0001-4's runs. sim step = the step's elapsed time minus its lag-compensation queries; server-only = commands, lag-compensation queries, interest, encoding and the tick's sends (proposed in the row). The sim step's own parts are ADR 0004's rows, not judged here."))

# ---------------- PM-0001-6 ----------------
marks.append(M(
    "PM-0001-6", "Snapshot encoding fits", "pass", ["R2"], P4, ["V-S"],
    [C("encode p99 at most 50 us per client per snapshot", "tick.s.encode_ns", "p99", "<=", 50_000, **{"from": ROOM})],
    [R("encode mean", "tick.s.encode_ns", "mean", **{"from": ROOM}),
     R("encode p50", "tick.s.encode_ns", "p50", **{"from": ROOM}),
     R("encode max", "tick.s.encode_ns", "max", **{"from": ROOM}),
     R("bytes per snapshot per client, mean", "tick.s.snap_payload_bytes", "mean", **{"from": ROOM}),
     R("bytes per snapshot per client, p50", "tick.s.snap_payload_bytes", "p50", **{"from": ROOM}),
     R("bytes per snapshot per client, max", "tick.s.snap_payload_bytes", "max", **{"from": ROOM})],
    [B("PM-0001-6/B-S", "rule B-S in each client's encode: busy-wait of (50 us - measured p99 + 5 us)")],
    [],
    "PM-0001-4's runs; 48 x 30 = 1,440 samples per second. 48 x 50 us = 2.40 ms is the row's planning check, not a pass number."))

# ---------------- PM-0001-7 ----------------
marks.append(M(
    "PM-0001-7", "Lag compensation stays cheap and small", "pass", ["R2"], P4, ["V-S"],
    [C("rewound shot against 48 candidates p99 at most 20 us", "tick.s.rewind_ns", "p99", "<=", 20_000, **{"from": ROOM}),
     C("hit history at most 1 MB (read every minute), whole run", "event.hit_history.size_bytes", "max", "<=", 1_000_000, window="all", **{"from": ROOM}),
     C("history read at least once a minute (30 reads in an S30)", "event.hit_history.size_bytes", "count", ">=", 30, **{"from": ROOM})],
    [R("rewound shots per second, mean", "second.v.rewinds_count", "mean", **{"from": ROOM}),
     R("history depth in ticks (sized for -48's test cap plus the margin)", "header.params.history_ticks_count", "max")],
    [B("PM-0001-7/B-S", "rule B-S in the rewound-shot query: busy-wait of (20 us - measured p99 + 2 us)"),
     B("PM-0001-7/untrimmed", "a history kept for the whole match (never trimmed) fails the 1 MB; a memory bite, so over a full S30")],
    ["R-20"],
    "PM-0001-4's runs at 700 shots/s (-2 checks the rate). The history is sized for the largest cap the test uses (-48's test cap, fixed in U4 from L1's largest honest rewind at P3 and P4) plus the margin, as data, recorded; the rule cap can only shrink it, so -7 needs no re-run (plan §6.2). 700 / 30 x 20 us = 0.47 ms per tick is a planning check. MB = 10^6 B."))

# ---------------- PM-0001-8 ----------------
marks.append(M(
    "PM-0001-8", "Interest picks under the cap, in time", "pass", ["R2"], P4, ["V-S"],
    [C("region query for 48 clients p99 at most 0.2 ms per tick", "tick.v.region_query_ns", "p99", "<=", 200_000, **{"from": ROOM}),
     C("ranking and packing p99 at most 0.5 ms per tick", "tick.v.rank_pack_ns", "p99", "<=", 500_000, **{"from": ROOM}),
     C("0 snapshots over 1,067 B per interval (UDP payload + 28 B per datagram)", "tick.s.snap_wire_bytes", "max", "<=", 1067, **{"from": ROOM}),
     C("the nearest 8 enemies in every snapshot", "tick.s.nearest8_missing_count", "sum", "==", 0, **{"from": ROOM}),
     C("every relevant entity at least once in every 4 snapshots", "tick.s.relevant_starved_count", "sum", "==", 0, **{"from": ROOM})],
    [],
    [B("PM-0001-8/B-S-region", "rule B-S in the region query: busy-wait of (0.2 ms - measured p99 + 0.02 ms)"),
     B("PM-0001-8/B-S-rank-pack", "rule B-S in ranking and packing: busy-wait of (0.5 ms - measured p99 + 0.05 ms)"),
     B("PM-0001-8/rank-by-id", "an accumulator that ranks by entity id fails 'the nearest 8'")],
    [],
    "PM-0001-4's runs, SS with its fight in the centre field (R2-INT-REC4's scene). 256,000 / 8 / 30 = 1,066.7, so 'over 1,067 B' is `<= 1067`. The per-client checks come from the packer's own log. 'Rule B-S for the times' is read as one bite per timed phase."))

# ---------------- PM-0001-9 ----------------
marks.append(M(
    "PM-0001-9", "The enclosure test fits, re-flood ticks included", "pass", ["R2"], P4, ["V-S"],
    [C("pair test p99 at most 0.1 us per enemy pair", "tick.v.pair_test_per_pair_ns", "p99", "<=", 100, **{"from": ROOM}),
     C("re-flood after a portal change p99 at most 1 ms", "event.reflood.time_ns", "p99", "<=", 1_000_000, **{"from": ROOM})],
    [R("the map's area count", "header.params.area_count", "max"),
     R("re-flood ticks, share of measured ticks", "tick.v.reflood", "share", where={"op": "==", "value": 1}, **{"from": ROOM}),
     R("re-floods that fell on an encoding tick, share", "event.reflood.encoded", "share", where={"op": "==", "value": 1}, **{"from": ROOM}),
     R("enemy pairs per tick, p50", "tick.v.pair_count", "p50", **{"from": ROOM})],
    [B("PM-0001-9/B-S-pair", "rule B-S in the pair test: busy-wait per tick of 768 x (0.1 us - measured per-pair p99 + 0.01 us)"),
     B("PM-0001-9/B-S-reflood", "rule B-S in the re-flood: busy-wait of (1 ms - measured p99 + 0.1 ms) in each re-flood")],
    [],
    "PM-0001-4's runs with the doors scripted at one portal change per second. A part under 1 us is timed per tick and divided by its count (key 1500-1501): `pair_test_per_pair_ns` is that tick's pair-test time / pair count, rounded up to whole ns, so `<= 100` is exact. 768 pairs at 16v16v16 (48 x 32 / 2). 'Rule B-S' is read as one bite per time in the row (question)."))

# ---------------- PM-0001-10 ----------------
marks.append(M(
    "PM-0001-10", "The server's sync cost grows with what clients see, not faster", "pass", ["R2"],
    [P("N=48", S30_P2, 3),
     P("N=24", {"layer": "L2", "scene": "SS", "profile": "P2", "tick_hz": 30, "clients": 24}, 3),
     P("N=36", {"layer": "L2", "scene": "SS", "profile": "P2", "tick_hz": 30, "clients": 36}, 3)],
    ["V-S"],
    [F("unit cost at 48 at most 1.1 x the unit cost at 24 and at 36", "sync_cost_ratio", "<=", "1.1", parts=["N=48"])],
    [R("unit cost per part: mean server-only per tick / (clients x mean relevant entities)", figure="sync_unit_cost")],
    [B("PM-0001-10/n-squared", "an encoder that walks every client's list once more per client (work growing with N squared)")],
    [],
    "N = 48 are PM-0001-4's S30 runs; N = 24 and 36 (three teams of 8 and of 12) run 10 minutes each at P2, 3 runs per N, V-S. Proposed reading: each N = 48 run's unit cost against the mean unit cost of the 3 runs at N = 24 and, separately, at N = 36; the figure is the larger ratio, so the check runs per N = 48 run. Means, not p99s (the row). The bite run should match N = 48's select (clients 48) so the judge reads the figure."))

emit(marks)
