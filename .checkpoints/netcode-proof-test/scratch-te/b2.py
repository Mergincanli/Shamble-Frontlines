import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

marks = []
P4 = [P("S30 at P2", S30_P2, 3)]
RSM = {"machine.role": "RS"}
BR = {"process": "browser"}

# ---------------- PM-0001-11 ----------------
marks.append(M(
    "PM-0001-11", "The transport runtime fits beside the tick", "pass", ["R2"], P4, ["V-S"],
    [C("transport runtime threads at most 1 vCPU (p99 of 1-second windows)", "second.v.io_cpu_permille", "p99", "<=", 1000, **{"from": ROOM}),
     C("send call p99 at most 1 ms", "second.s.send_call_ns", "p99", "<=", 1_000_000, **{"from": ROOM})],
    [R("transport runtime CPU, mean", "second.v.io_cpu_permille", "mean", **{"from": ROOM}),
     R("input datagrams received per second, mean", "second.v.inputs_received_count", "mean", **{"from": ROOM}),
     R("snapshot datagrams sent per second, mean", "second.v.snap_datagrams_sent_count", "mean", **{"from": ROOM})],
    [B("PM-0001-11/B-S-send", "rule B-S on the send call: busy-wait in each send_datagram of (1 ms - measured p99 + 0.1 ms)")],
    [],
    "PM-0001-4's runs; the runtime's threads (the three vCPUs outside the tick's) read every second from /proc; 1000 permille = one vCPU. 48 sessions x 30 input datagrams and 30 snapshots per second each: the input count is V-S's clause; the two rates are reported."))

# ---------------- PM-0001-12 ----------------
ES_P2 = {"layer": "L2", "scene": "ES", "profile": "P2", "tick_hz": 30, "clients": 48}
marks.append(M(
    "PM-0001-12", "The downstream fits per client", "pass", ["R2"],
    [P("ES at P2", ES_P2, 3), P("SS at P2", S30_P2, 3)],
    ["V-S"],
    [F("ES: each client's mean at most 160 kbps", "down_client_mean_max_bits", "<=", 160_000, parts=["ES at P2"]),
     C("ES: p99 of 1-second windows at most 256 kbps", "second.s.down_bits", "p99", "<=", 256_000, **{"from": ROOM, "parts": ["ES at P2"]}),
     C("SS: every 1-second window at most 256 kbps", "second.s.down_bits", "max", "<=", 256_000, **{"from": ROOM, "parts": ["SS at P2"]}),
     C("SS: 0 datagrams above the cap rule (1,000 B until -36 measures)", "tick.s.snap_datagram_bytes", "max", "<=", 1000, **{"from": ROOM, "parts": ["SS at P2"]})],
    [R("SS: mean per client", "second.s.down_bits", "mean", **{"from": ROOM, "parts": ["SS at P2"]}),
     R("SS: R2's formula at the run's own counts", figure="r2_down_formula_bits", parts=["SS at P2"]),
     R("ES: mean per client, pooled", "second.s.down_bits", "mean", **{"from": ROOM, "parts": ["ES at P2"]}),
     R("the IPv6 figure, if LG reaches RS over IPv6", figure="down_mean_ipv6_bits")],
    [B("PM-0001-12/B-N-ES", "rule B-N in ES: padding per snapshot of the measured margin + 10 % of the mark"),
     B("PM-0001-12/B-N-SS", "rule B-N in SS: padding per snapshot of the measured margin + 10 % of the mark")],
    [],
    "ES at P2 for 10 minutes, 3 runs; SS at P2 with everyone in view (PM-0001-2), PM-0001-4's runs. Counted per client at the server's send side, UDP payload + 28 B per datagram; kbps = 1,000 bit/s. Reading (question): 'mean at most 160 kbps' per client is the largest client mean (a figure); the p99 pools every client's 1-second windows. The cap value is 1,000 B until PM-0001-36 sets it; a later cap under 1,000 B changes this value and re-runs the row (plan §6.2). R2's formula uses each client's set, shots, spawns and events per second (lines 1764-1766), as PM-0006-17 does. The bite run should match its scene's part."))

# ---------------- PM-0001-13 ----------------
SS_P4 = {"layer": "L2", "scene": "SS", "profile": "P4", "tick_hz": 30, "clients": 48}
rep13 = []
for pn in ["P2", "P4 (report only)"]:
    rep13 += [
        R(f"{pn}: p99 of 1-second windows", "second.s.up_bits", "p99", **{"from": ROOM, "parts": [pn]}),
        R(f"{pn}: largest input datagram", "second.s.input_datagram_bytes", "max", **{"from": ROOM, "parts": [pn]}),
        R(f"{pn}: commands per datagram, median", "second.s.input_cmds_count", "p50", **{"from": ROOM, "parts": [pn]}),
        R(f"{pn}: commands per datagram, max", "second.s.input_cmds_count", "max", **{"from": ROOM, "parts": [pn]}),
        R(f"{pn}: command size whole (U1), mean", "second.s.cmd_whole_bytes", "mean", **{"from": ROOM, "parts": [pn]}),
        R(f"{pn}: command size as sent, delta-coded (U2), mean", "second.s.cmd_sent_bytes", "mean", **{"from": ROOM, "parts": [pn]}),
    ]
rep13.append(R("P4: each client's mean", figure="up_client_mean_max_bits", parts=["P4 (report only)"]))
marks.append(M(
    "PM-0001-13", "The upstream fits per client", "pass", ["R2"],
    [P("P2", S30_P2, 3), P("P4 (report only)", SS_P4, 1)],
    ["V-S"],
    [F("each client's mean at most 48 kbps (IPv4, UDP payload + 28 B)", "up_client_mean_max_bits", "<=", 48_000, parts=["P2"])],
    rep13,
    [B("PM-0001-13/B-N", "rule B-N: padding per input datagram of the measured margin + 10 % of the mark")],
    [],
    "P2 in PM-0001-4's runs (receive side at RS); P4 for 10 minutes for the report, where the redundancy cap of 8 is reached (1 run: the row names no count). The mean is the proposed reading of the row; per client it is the largest client mean (question). 3 to 6 commands per datagram at P2 is the draft's estimate, reported, not judged. The P4 part has no check: it needs a report entry that takes `parts` (question)."))

# ---------------- PM-0001-14 ----------------
judged14 = ["S30 runs", "ML Chrome", "ML Edge", "ML Firefox", "MAC Safari"]
br14 = ["ML Chrome", "ML Edge", "ML Firefox", "MAC Safari"]
marks.append(M(
    "PM-0001-14", "No datagram is ever too big", "pass", ["R1", "R2", "R4", "R7", "MAC-r", "ML*"],
    [P("S30 runs", {"params.soak": "S30"}, 3),
     P("ML Chrome", {"machine.name": "ML", "params.browser": "chrome"}, 1),
     P("ML Edge", {"machine.name": "ML", "params.browser": "edge"}, 1),
     P("ML Firefox", {"machine.name": "ML", "params.browser": "firefox"}, 1),
     P("MAC Safari", {"machine.role": "MAC", "params.browser": "safari"}, 1),
     P("ML* (rough, reported)", {"machine.name": "ML*"}, 1)],
    [],
    [C("0 snapshot datagrams above the cap (1,000 B until -36 measures)", "tick.s.snap_datagram_bytes", "max", "<=", 1000, **{"from": ROOM, "parts": judged14}),
     C("0 sends refused as above the peer's maximum, server side", "tick.v.datagram_too_large_count", "sum", "==", 0, window="all", **{"from": ROOM, "parts": judged14}),
     C("0 datagrams above the browser's reported maximum, browser side", "second.v.recv_over_max_count", "sum", "==", 0, window="all", **{"from": BR, "parts": br14}),
     C("0 fragmented IP packets (packet capture on RS)", "event.capture.fragments_count", "sum", "==", 0, window="all", **{"from": RSM, "parts": judged14}),
     C("the capture ran", "event.capture.fragments_count", "count", ">=", 1, window="all", **{"from": RSM, "parts": judged14})],
    [R("each browser's smallest maxDatagramSize", "second.v.max_datagram_size_bytes", "min", **{"from": BR}),
     R("ML*: largest snapshot datagram", "tick.s.snap_datagram_bytes", "max", **{"from": ROOM, "parts": ["ML* (rough, reported)"]}),
     R("ML*: datagrams above the browser's maximum", "second.v.recv_over_max_count", "sum", **{"from": BR, "parts": ["ML* (rough, reported)"]})],
    [B("PM-0001-14/unsplit-group", "an entity group that is never split sends a datagram above the cap")],
    ["R-13"],
    "Every S30 (R2) and every browser session (R1, R4, R7; MAC-r): the cap is min(the smallest maxDatagramSize of PM-0001-36, 1,150 B), 1,000 B until measured; this value changes when -36 reads lower. A runner may stand as MAC for this yes-or-no row (key, R-13). The ML parts wait for the named laptop (R8); ML* is a rough check, reported only, so no check names its part. The capture is a summary event written once per run, read with `window: all`."))

# ---------------- PM-0001-15 ----------------
marks.append(M(
    "PM-0001-15", "A room's memory stays bounded and flat", "pass", ["R2"], P4, ["V-S"],
    [C("resident memory peak of 1-second samples at most 64 MB, start and after 20 minutes included", "second.v.rss_bytes", "max", "<=", 64_000_000, window="all", **{"from": ROOM}),
     F("slope after warm-up within 1 MB per 10 minutes, high", "rss_slope_bytes_per_10min", "<=", 1_000_000),
     F("slope after warm-up within 1 MB per 10 minutes, low", "rss_slope_bytes_per_10min", ">=", -1_000_000),
     F("every per-client queue flat: max of the last 10 minutes at most the max of the first 10", "queue_growth_count", "==", 0)],
    [R("resident memory, max", "second.v.rss_bytes", "max", window="all", **{"from": ROOM}),
     R("resident memory at the start (first warm-up second)", "second.v.rss_bytes", "min", window="warmup", **{"from": ROOM})],
    [B("PM-0001-15/leak", "R5-TEST-REC5's planted leak (one buffer kept per snapshot), flagged within the run; a memory bite, so over a full S30")],
    [],
    "S30 at P2, PM-0001-4's runs; resident memory read every second; queue lengths (outbound, unacknowledged events) per client from the server's counters. The 64 MB at start and after 20 minutes are inside the 1-second peak. MB = 10^6 B. The slope method (least squares over the measured seconds) is proposed; 'flat' is the row's proposed reading. 'Resident memory at the start' reads the first warm-up second as the minimum of the warm-up window, an approximation (all 48 joined)."))

# ---------------- PM-0001-16 ----------------
marks.append(M(
    "PM-0001-16", "A room starts fast", "pass", ALL_VMS,
    [P("every room start", {}, 1)],
    [],
    [C("every start, process spawn to Open, under 2 s", "event.room_start.spawn_to_open_ns", "max", "<", 2_000_000_000, window="all"),
     C("at least 20 starts over the spike", "event.room_start.spawn_to_open_ns", "count", ">=", 20, window="all", pooled=True)],
    [R("spawn to Open, p50", "event.room_start.spawn_to_open_ns", "p50", window="all"),
     R("spawn to Open, max", "event.room_start.spawn_to_open_ns", "max", window="all")],
    [B("PM-0001-16/double-load", "a start that loads its mode file twice over a 1 s delay")],
    [],
    "Every room start in every run, at least 20 (proposed), first counted in R1; timed by the supervisor's stand-in, which writes one `room_start` event per start. A plain limit. The 20 is pooled over every run that lists the mark."))

# ---------------- PM-0001-17 ----------------
parts17 = [P(f"L1 {p}", {"layer": "L1", "profile": p}, 3) for p in ["P0", "P2", "P3", "B1"]]
parts17 += [P(f"L2 {p}", {"layer": "L2", "profile": p, "machine.role": "LG"}, 1) for p in ["P0", "P2", "P3", "B1"]]
parts17.append(P("every S30", {"params.soak": "S30"}, 3))
marks.append(M(
    "PM-0001-17", "Every decoded snapshot is the server's state", "pass", ["C", "R3"],
    parts17, [],
    [F("100 % of decoded snapshots rebuild the server's quantised state (0 mismatches; on L2 under the Arm rule)", "decoded_hash_mismatch_count", "==", 0),
     C("setup: the S30 starts 1,800 ticks below 65,536, so it crosses the 16-bit wrap", "header.params.tick_start", "max", "==", 63_736, parts=["every S30"])],
    [R("aarch64-only mismatches (a cross-architecture finding; decides nothing)", figure="cross_arch_mismatch_count"),
     R("decoded snapshots compared", figure="decoded_hash_compared_count")],
    [B("PM-0001-17/tick16", "a 16-bit tick in the snapshot header (fails at the wrap: the bite run must cross it)"),
     B("PM-0001-17/client-rounding", "a client quantiser that rounds differently from the server's")],
    ["R-5", "R-8"],
    "L1: 48 headless clients, 3 seeds per profile, 10 minutes each under P0, P2, P3 and B1; L2: LG's 48 clients, 1 run per profile (R3, VS4). The server logs each client's quantised hash per tick, the client logs its decoded hash; each LG client logs every datagram and stream byte it receives (kept on LG). Arm rule: an L2 mismatch counts only once those bytes, decoded again on x86-64 (npt-lab, in this container or on RS: R-8), miss the server's hash too; a mismatch only on aarch64 is reported. The figure fails, never passes, on 0 compared snapshots.",
    arm_rule=True))

# ---------------- PM-0001-57 ----------------
marks.append(M(
    "PM-0001-57", "Decoded values keep the protocol's precision", "pass", ["C"],
    [P(f"L1 {p}", {"layer": "L1", "profile": p}, 3) for p in ["P0", "P2", "P3", "B1"]],
    [],
    [C("positions: max error at most 1 cm", "tick.s.pos_err_um", "max", "<=", 10_000, window="all"),
     C("yaw: max error at most 0.01 degree", "tick.s.yaw_err_microdeg", "max", "<=", 10_000, window="all"),
     C("pitch: max error at most one 0.18 degree step", "tick.s.pitch_err_microdeg", "max", "<=", 180_000, window="all")],
    [R("positions: error p99", "tick.s.pos_err_um", "p99", window="all")],
    [B("PM-0001-57/pos-4cm", "positions quantised to 4 cm (errors up to 2 cm)"),
     B("PM-0001-57/yaw-12bit", "yaw in 12 bits (0.088 degree steps)"),
     B("PM-0001-57/pitch-8bit", "pitch in 8 bits (0.70 degree steps)")],
    [],
    "PM-0001-17's L1 runs: the server logs each sent entity's position, yaw and pitch before quantising, the client the values it decodes; npt-lab writes each value's error against the server's. Units `_um` and `_microdeg` are not in README §1's list (question). 0.18 degree is the row's literal; the 10-bit field's exact step depends on the pitch range (180 / 1024 = 0.176 degree if it spans 180 degrees)."))

# ---------------- PM-0001-58 ----------------
marks.append(M(
    "PM-0001-58", "Every counter survives its wrap", "pass", ["C"],
    [P("L1 P0", {"layer": "L1", "profile": "P0", "params.suite": "wrap"}, 3),
     P("L1 P3", {"layer": "L1", "profile": "P3", "params.suite": "wrap"}, 3)],
    [],
    [F("100 % of decoded snapshots rebuild the server's state on every tick", "decoded_hash_mismatch_count", "==", 0),
     C("0 commands refused", "tick.v.cmd_refused_count", "sum", "==", 0, window="all"),
     C("0 commands lost", "tick.v.cmd_lost_count", "sum", "==", 0, window="all"),
     C("0 commands doubled", "tick.v.cmd_doubled_count", "sum", "==", 0, window="all"),
     C("0 presses lost", "tick.v.press_lost_count", "sum", "==", 0, window="all"),
     C("0 presses doubled", "tick.v.press_doubled_count", "sum", "==", 0, window="all"),
     C("0 corrections larger than 1 mm", "tick.s.correction_um", "max", "<=", 1000, window="all"),
     C("setup: the server tick starts 30 ticks below a multiple of 65,536", "header.params.ticks_below_wrap", "max", "==", 30),
     C("every 16-bit or narrower counter wrapped in the run (server tick's low 16 bits, command sequence, press counters, tick offsets, event numbers, acknowledgement numbers)", "event.counter_wrap.tick_count", "count", ">=", 6, window="all"),
     C("the run lasts 10 s (300 ticks)", "tick.v.cmd_lost_count", "count", ">=", 300, window="all")],
    [R("measured reorder share", "header.params.reorder_permille", "max")],
    [B("PM-0001-58/lt-cmd-seq", "the command sequence number compared with a plain < instead of its wrap-aware comparison"),
     B("PM-0001-58/lt-press", "the press counters compared with a plain <"),
     B("PM-0001-58/lt-tick-offset", "the snapshot's tick offsets compared with a plain <"),
     B("PM-0001-58/lt-event-seq", "the event numbers compared with a plain <"),
     B("PM-0001-58/lt-ack", "the acknowledgement numbers compared with a plain <"),
     B("PM-0001-58/tick16", "the header tick narrowed to 16 bits (PM-0001-17's bite, caught here within seconds)")],
    ["R-5"],
    "A deterministic L1 run (48 headless clients): the tick starts 30 below a multiple of 65,536 and every counter a few steps below its wrap (within the first 2 s), then 10 s; at P0 and P3, 3 seeds each. The reorder rule (key 1471-1474) applies to the P3 parts only ('so reordering crosses the wrap'): validity is per mark in the contract, so it is not listed here (question); P3's reorder share is reported. The whole run is read (`window: all`): the wraps fall in the first seconds. The proof test's harness now, the gate's network step (G11) later."))

# ---------------- PM-0001-18 ----------------
marks.append(M(
    "PM-0001-18", "A loss burst heals", "pass", ["C", "R3"],
    [P("L1 B1", {"layer": "L1", "profile": "B1"}, 3), P("L2 B1", {"layer": "L2", "profile": "B1", "machine.role": "LG"}, 1)],
    [],
    [C("every client decodes deltas again within 500 ms of the link coming back", "event.burst_heal.after_link_ns", "max", "<=", 500_000_000),
     C("every client after every burst: 48 x 10 heal records", "event.burst_heal.after_link_ns", "count", ">=", 480)],
    [R("heal time p50", "event.burst_heal.after_link_ns", "p50"),
     R("heal time p99", "event.burst_heal.after_link_ns", "p99")],
    [B("PM-0001-18/no-fallback", "a baseline that never falls back (deltas only against the last acknowledged snapshot)")],
    ["R-5", "R-12"],
    "B1 with 48 clients, 10 bursts per run (proposed); L1 3 seeds, L2 1 run (R3, VS4). A plain limit per client and burst. Each client writes one `burst_heal` event per burst, when it first decodes a delta after the link returns; a client that never heals writes none, so the count catches it. On L2 the time uses the client's server-clock estimate, whose error bound is PM-0001-47's."))

emit(marks)
