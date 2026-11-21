import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

marks = []
BR = {"process": "browser"}
BOTS = {"process": "bots"}
ML3 = ["chrome", "edge", "firefox"]

# ---------------- PM-0001-28 ----------------
p28, judged28 = [], []
for crate in ["wtransport", "web-transport-quinn"]:
    for b in ML3:
        n = f"{crate}: ML {b}"
        p28.append(P(n, {"params.suite": "wt-attempts", "params.crate": crate, "machine.name": "ML", "params.browser": b}, 1)); judged28.append(n)
    n = f"{crate}: MAC safari"
    p28.append(P(n, {"params.suite": "wt-attempts", "params.crate": crate, "machine.role": "MAC", "params.browser": "safari"}, 1)); judged28.append(n)
    p28.append(P(f"{crate}: control, wrong certificate hash", {"role": "control", "params.suite": "wt-attempts", "params.crate": crate}, 1))
    p28.append(P(f"{crate}: ML* (rough, reported)", {"params.suite": "wt-attempts", "params.crate": crate, "machine.name": "ML*"}, 1))
ctl28 = [p["name"] for p in p28 if "control" in p["name"]]
rough28 = [p["name"] for p in p28 if "rough" in p["name"]]
marks.append(M(
    "PM-0001-28", "Each WebTransport candidate works in every browser", "pass", ["ML*", "MAC-r", "R4"],
    p28, [],
    [C("100 attempts", "event.attempt.ok", "count", ">=", 100, window="all", **{"from": BR, "parts": judged28 + ctl28}),
     C("at least 99 of 100 attempts open, exchange datagrams and one bidirectional stream", "event.attempt.ok", "sum", ">=", 99, window="all", **{"from": BR, "parts": judged28}),
     C("control: a wrong certificate hash fails 100 of 100", "event.attempt.ok", "sum", "==", 0, window="all", **{"from": BR, "parts": ctl28})],
    [R("ML*: attempts that worked", "event.attempt.ok", "sum", window="all", **{"from": BR, "parts": rough28})],
    [],
    ["R-7", "R-13"],
    "For wtransport 0.7.x and web-transport-quinn 0.13.x each (`params.crate`): a scripted page opens 100 sessions per browser and crate. 'The same local network' is the browsers' own machine (R-7): the server on loopback, the page on localhost, the server's Origin list localhost only; the runner's Safari leg runs against RS and is recorded as an internet path (`params.path`). A runner may stand as MAC here (a count row, R-13). The control is a part (role `control`), not a bite: the row asks it to fail 100 of 100, stronger than failing one check. The negotiated draft per browser is a string (`event.attempt.draft`): no stat reads it, so the verdict detail should print it (question). The ML parts wait for the named laptop."))

# ---------------- PM-0001-29 ----------------
p29 = [P(f"ML {b}", {"params.suite": "fallback", "machine.name": "ML", "params.browser": b}, 1) for b in ML3]
p29.append(P("MAC-b safari", {"params.suite": "fallback", "machine.name": "MAC-b", "params.browser": "safari"}, 1))
p29.append(P("ML* (rough, reported)", {"params.suite": "fallback", "machine.name": "ML*"}, 1))
p29.append(P("TCP-443-only network (reported)", {"params.suite": "fallback-443"}, 1))
j29 = [p["name"] for p in p29[:4]]
marks.append(M(
    "PM-0001-29", "The fallback takes over when QUIC is blocked", "pass", ["R4", "ML*", "MAC-b"],
    p29, [],
    [C("20 attempts per browser", "event.fallback.playing_ns", "count", ">=", 20, window="all", **{"from": BR, "parts": j29}),
     C("a playing WebSocket session at most 3 s after the page starts connecting", "event.fallback.playing_ns", "max", "<=", 3_000_000_000, window="all", **{"from": BR, "parts": j29})],
    [R("ML*: time to a playing session, max", "event.fallback.playing_ns", "max", window="all", **{"from": BR, "parts": ["ML* (rough, reported)"]}),
     R("TCP 443 only: attempts that reached play", "event.fallback.playing_ns", "count", window="all", **{"from": BR, "parts": ["TCP-443-only network (reported)"]})],
    [B("PM-0001-29/no-wt-timeout", "no WebTransport timeout: the attempt hangs")],
    ["R-13"],
    "RS with its UDP range closed (L2's UDP block). 20 attempts per browser (proposed), one at a time, each session closed before the next, so the per-address cap never refuses one (its setting recorded: `header.params.addr_cap`); the fallback on the TCP port the lobby hands out (OQ-0006-16 (a)). WebTransport timeout 1.5 s. A time row: Safari on a borrowed Mac only, never the runner. A hung attempt writes no `fallback` event, so the count catches it. Reported: one attempt per browser from a TCP-443-only network, for OQ-0006-16's 443 question. The ML parts wait for the named laptop."))

# ---------------- PM-0001-30 ----------------
marks.append(M(
    "PM-0001-30", "Both transports give the same server", "pass", ["R3", "C"],
    [P("RS: 3 recordings x 2 transports", {"params.suite": "both-transports", "profile": "P0", "machine.name": "RS"}, 6),
     P("C: loopback, 3 recordings x 2 transports", {"params.suite": "both-transports", "profile": "P0", "machine.role": "C"}, 6)],
    ["late-zero"],
    [F("identical server state hashes on every tick (each run against its other-transport twin)", "twin_hash_mismatch_count", "==", 0)],
    [R("input buffer depth set for P0 (ticks)", "header.params.buffer_depth_ticks_count", "max")],
    [B("PM-0001-30/wallclock", "a wall-clock read on one transport's command path")],
    [],
    "3 recordings of 5 minutes (proposed), each replayed over WebTransport and over WebSocket at P0, the input buffer deep enough for P0 on both, recorded; each run names its other-transport run in `params.twin`, so all 6 runs are measured and all 6 must keep `late-zero`. R3 (VS5), and in this container on loopback (plan §6.2). The row's second bite, 'one command of a replay delayed past its tick must make that run invalid, never a pass', expects INVALID: under the contract an invalid bite run makes the mark INVALID, so it is not in `bites`; it belongs in npt-judge's fixtures (a run with `tick.v.late_count` above 0 must come out INVALID) (question)."))

# ---------------- PM-0001-31 ----------------
marks.append(M(
    "PM-0001-31", "The fallback's stalls stay short", "pass", ["R3", "R7", "ML*"],
    [P("LG over WebSocket at P2", {"layer": "L2", "profile": "P2", "params.suite": "ws-stall", "machine.role": "LG"}, 1),
     P("ML Chrome over WebSocket at P2", {"layer": "L2", "profile": "P2", "params.suite": "ws-stall", "machine.name": "ML", "params.browser": "chrome"}, 1),
     P("ML* Chrome (rough, reported)", {"layer": "L2", "profile": "P2", "params.suite": "ws-stall", "machine.name": "ML*"}, 1)],
    [],
    [C("p99 extra snapshot delay at most 250 ms (age when decoded minus 50 ms)", "second.s.ws_extra_delay_ns", "p99", "<=", 250_000_000,
       parts=["LG over WebSocket at P2", "ML Chrome over WebSocket at P2"]),
     C("never more than one unsent snapshot per client (max, sampled every tick)", "tick.v.unsent_snaps_max_count", "max", "<=", 1, window="all", **{"from": ROOM, "parts": ["LG over WebSocket at P2", "ML Chrome over WebSocket at P2"]})],
    [R("ML*: p99 extra delay", "second.s.ws_extra_delay_ns", "p99", parts=["ML* Chrome (rough, reported)"]),
     R("stalls counted by the delay wrapper", "second.v.ws_stalls_count", "sum", **{"from": ROOM}),
     R("stall share", "second.v.ws_stall_permille", "mean", **{"from": ROOM})],
    [B("PM-0001-31/ws-queue-all", "a server that queues every snapshot over WebSocket")],
    ["R-25"],
    "L2's TCP delay (the stream wrapper in npt-server's accept path, R-25) on RS; LG (R3, VS4) and ML Chrome (R7), 10 minutes at P2, 1 run each. Extra delay is the row's proposed definition; clients write it on their server-clock estimate. The ML part waits for the named laptop."))

# ---------------- PM-0001-32 ----------------
p32 = []
for b in ML3:
    for be in ["webgpu", "webgl2"]:
        p32.append(P(f"ML {b} {be}", {"params.suite": "one-build", "machine.name": "ML", "params.browser": b, "params.backend": be}, 1))
for be in ["webgpu", "webgl2"]:
    p32.append(P(f"MAC safari {be}", {"params.suite": "one-build", "machine.role": "MAC", "params.browser": "safari", "params.backend": be}, 1))
p32.append(P("ML* (rough, reported)", {"params.suite": "one-build", "machine.name": "ML*"}, 1))
j32 = [p["name"] for p in p32 if "rough" not in p["name"]]
marks.append(M(
    "PM-0001-32", "One client build runs in all four browsers on both backends", "pass", ["R7", "ML*", "MAC-r", "MAC-b"],
    p32, [],
    [F("the same build (one hash) in every run", "one_build_hash", "==", 1, parts=j32),
     C("a scripted 60 s session", "second.v.console_errors_count", "count", ">=", 60, window="all", **{"from": BR, "parts": j32}),
     C("0 console errors", "second.v.console_errors_count", "sum", "==", 0, window="all", **{"from": BR, "parts": j32}),
     C("0 page errors", "second.v.page_errors_count", "sum", "==", 0, window="all", **{"from": BR, "parts": j32}),
     C("the feature-detection path covered", "event.feature_detect.webgpu", "count", ">=", 1, window="all", **{"from": BR, "parts": j32})],
    [R("ML*: console errors", "second.v.console_errors_count", "sum", window="all", **{"from": BR, "parts": ["ML* (rough, reported)"]})],
    [B("PM-0001-32/browser-sniff", "the datagram writer detected by browser name")],
    ["R-13"],
    "ML and MAC, 1 session per browser and backend (three.js's WebGPU renderer and its WebGL 2 path). The page's own error hook writes every console and page error; per-second counts, because an empty series fails (a 0-error session writes no error events). 'Runs unchanged' is the row's proposed reading. Safari on MAC-r if the advisor extends the runner list (R-13), else MAC-b. The ML parts wait for the named laptop."))

# ---------------- PM-0001-33 ----------------
marks.append(M(
    "PM-0001-33", "The client's RTT agrees with the server's", "pass", ["R7", "ML*"],
    [P("ML Chrome at P2", {"layer": "L2", "profile": "P2", "params.suite": "rtt", "machine.name": "ML", "params.browser": "chrome"}, 1),
     P("ML Chrome at a fixed 100 ms (recorded)", {"layer": "L2", "params.suite": "rtt-fixed-100", "machine.name": "ML", "params.browser": "chrome"}, 1),
     P("ML* Chrome (rough, reported)", {"layer": "L2", "params.suite": "rtt", "machine.name": "ML*"}, 1)],
    [],
    [F("p99 of |app-level RTT - quinn's rtt()| per second over 10 minutes at most 5 ms", "rtt_agreement_p99_ns", "<=", 5_000_000, parts=["ML Chrome at P2"])],
    [R("fixed 100 ms, no jitter or loss: the same p99", figure="rtt_agreement_p99_ns", parts=["ML Chrome at a fixed 100 ms (recorded)"]),
     R("ML*: the same p99", figure="rtt_agreement_p99_ns", parts=["ML* Chrome (rough, reported)"])],
    [B("PM-0001-33/pong-on-tick", "pongs answered from the tick thread (up to one tick period late)"),
     B("PM-0001-33/no-hold-time", "the client's pong sent without its hold time (up to one tick period added)")],
    [],
    "L2 on RS; ML Chrome; both estimates logged every second (the client's app-level RTT in the browser's log, quinn's `rtt()` in the room's), 10 minutes, 1 run per profile. The figure joins them by second on the client's server-clock estimate. The fixed 100 ms case is R5-TEST-REC4's reading, recorded. The ML parts wait for the named laptop."))

# ---------------- PM-0001-34 ----------------
MSGS = ["Input", "Snapshot", "FullState", "Resync", "Ping", "Pong", "Join", "JoinAccepted", "JoinRejected",
        "ClientSettings", "RoomEnded", "Leave", "Kick", "SessionControl"]
marks.append(M(
    "PM-0001-34", "No decoder panics or over-allocates", "pass", ["C"],
    [P(m, {"params.suite": "fuzz", "params.msg": m}, 1) for m in MSGS],
    [],
    [C("10 million random and mutated inputs", "event.fuzz_end.inputs_count", "min", ">=", 10_000_000, window="all"),
     C("0 panics", "event.fuzz_end.panics_count", "sum", "==", 0, window="all"),
     C("0 allocations above the size limits (counting allocator)", "event.fuzz_end.over_limit_allocs_count", "sum", "==", 0, window="all")],
    [R("corpus size", "event.fuzz_end.corpus_count", "max", window="all")],
    [B("PM-0001-34/unchecked-index", "an unchecked index in one decoder is found within the run")],
    [],
    "Any machine: cargo-fuzz on a dated nightly (recorded in the header), a counting allocator; 1 run per message type of the catalogue (plan §4.7's npt-proto messages and SessionControl). The decoded control messages in random sequences driven into `room` (PM-0004-60's, with this row's count) are not judged here. The fuzz harness writes one `fuzz_end` event per run."))

# ---------------- PM-0001-35 ----------------
marks.append(M(
    "PM-0001-35", "The transport stays swappable and current", "pass", ["C"],
    [P("after the swap", {"params.suite": "deps"}, 1)],
    [],
    [C("switching WebTransport crates changes no code outside the transport module (the swap's diff)", "event.swap_diff.files_outside_transport_count", "sum", "==", 0, window="all"),
     C("the diff was read", "event.swap_diff.files_outside_transport_count", "count", ">=", 1, window="all"),
     C("the sim crate depends on no transport crate (cargo tree)", "event.cargo_tree.sim_transport_deps_count", "max", "==", 0, window="all"),
     C("quinn-proto >= 0.11.18", "header.params.quinn_proto_version", "min", ">=", 11_018),
     C("rustls >= 0.23.46", "header.params.rustls_version", "min", ">=", 23_046),
     C("ring >= 0.17.12", "header.params.ring_version", "min", ">=", 17_012),
     C("tokio >= 1.54.1", "header.params.tokio_version", "min", ">=", 1_054_001),
     C("wtransport's dangerous-configuration feature off", "header.params.wt_dangerous_config", "max", "==", 0)],
    [],
    [B("PM-0001-35/quinn-in-sim", "`quinn` added to the sim crate (R5-GATE-REC3's G5 mutation)"),
     B("PM-0001-35/quinn-proto-0.11.17", "quinn-proto pinned at 0.11.17")],
    [],
    "Any machine, once, after the swap: the diff, `cargo tree`, the lockfile read by script. A version is written as one integer, major x 1,000,000 + minor x 1,000 + patch (0.11.18 = 11,018; 1.54.1 = 1,054,001), so a floor is plain data (proposed encoding; assumes minor and patch stay under 1,000)."))

# ---------------- PM-0001-36 ----------------
p36 = []
for b in ML3:
    for path in ["loopback", "eu-path"]:
        p36.append(P(f"ML {b} {path}", {"params.suite": "mds", "machine.name": "ML", "params.browser": b, "params.path": path}, 1))
for path in ["loopback", "eu-path"]:
    p36.append(P(f"MAC-b safari {path}", {"params.suite": "mds", "machine.name": "MAC-b", "params.browser": "safari", "params.path": path}, 1))
p36.append(P("ML* (rough, reported)", {"params.suite": "mds", "machine.name": "ML*"}, 1))
j36 = [p["name"] for p in p36 if "rough" not in p["name"]]
marks.append(M(
    "PM-0001-36", "Datagram size is measured per browser", "pass", ["R1", "ML*", "MAC-b"],
    p36, [],
    [C("maxDatagramSize read at start and every 10 s over 10 minutes (61 reads)", "event.mds_read.size_bytes", "count", ">=", 61, window="all", **{"from": BR, "parts": j36}),
     C("at least 1,000 B in every read", "event.mds_read.size_bytes", "min", ">=", 1000, window="all", **{"from": BR, "parts": j36})],
    [R("smallest value per browser and path", "event.mds_read.size_bytes", "min", window="all", **{"from": BR}),
     R("largest value per browser and path", "event.mds_read.size_bytes", "max", window="all", **{"from": BR})],
    [B("PM-0001-36/read-once", "the size read once, at start only")],
    ["R-7", "R-12"],
    "ML and MAC-b: to a local server (loopback, before R1: R-7) and to RS from an EU consumer connection (R1, the three browsers at once); 10 minutes per browser and path, 1 session each. A FAIL of the 1,000 B check is the row's 'otherwise' branch, a finding: the cap drops to the smallest value measured, and the size and bandwidth rows (-8, -12, -14) re-run (plan §6.2); it sets -14's cap, final only once the Mac reading exists. The ML parts wait for the named laptop; the ML* loopback legs are rough."))

# ---------------- PM-0001-37 ----------------
marks.append(M(
    "PM-0001-37", "Datagrams arrive in time at P2", "pass", ["R3"],
    [P("L2 P2, LG 48 clients", {"layer": "L2", "profile": "P2", "params.suite": "datagram-timing", "machine.role": "LG"}, 1)],
    [],
    [C("snapshots lost or late at most 3 % (late: more than 66.67 ms after send + 50 ms)", "second.s.snap_lost_or_late", "share", "<=", "0.03", where={"op": "==", "value": 1}, **{"from": BOTS})],
    [R("the research's reading: late more than 66.67 ms after sending", "second.s.snap_late_after_send", "share", where={"op": "==", "value": 1}, **{"from": BOTS})],
    [B("PM-0001-37/hold-3-intervals", "a send queue that holds each snapshot for 3 intervals")],
    [],
    "L2 on RS; LG with 48 clients, 10 minutes, 1 run. One flag per expected snapshot per client: 1 if lost, or decoded more than 66.67 ms after its expected arrival (send time + the downlink's 50 ms, on the client's server-clock estimate). 3 % = P2's 2 % loss + 1 percentage point."))

# ---------------- PM-0001-38 ----------------
marks.append(M(
    "PM-0001-38", "One-shot actions survive loss, and bulk data does not delay control", "pass", ["R3"],
    [P("L2 P3, LG", {"layer": "L2", "profile": "P3", "params.suite": "one-shots", "machine.role": "LG"}, 1)],
    [],
    [C("10,000 scripted one-shot actions", "second.v.one_shots_sent_count", "sum", ">=", 10_000, window="all", **{"from": BOTS}),
     F("0 dropped (sent by LG's clients minus received by the room)", "one_shot_dropped_count", "==", 0),
     C("10 transfers of 1 MB", "event.bulk_transfer.size_bytes", "count", ">=", 10, window="all"),
     C("each transfer 1 MB", "event.bulk_transfer.size_bytes", "min", ">=", 1_000_000, window="all"),
     F("control stream p99 message delay with a transfer minus without at most 5 ms", "bulk_control_p99_rise_ns", "<=", 5_000_000)],
    [],
    [B("PM-0001-38/bulk-on-control", "bulk data sent on the control stream")],
    [],
    "L2 on RS; LG at P3 (5 % loss), 1 run; may share PM-0001-20's presses (then that run lists both marks and its select needs `params.suite` = one-shots too, or the part's select drops it: question for the run plan). The control stream's message delays are written apart for the seconds with and without a transfer (`second.s.control_delay_bulk_ns`, `second.s.control_delay_idle_ns`); the figure is the difference of their p99s."))

# ---------------- 60 Hz rows: outside ----------------
for mid, title, bites in [
    ("PM-0001-39", "The 60 Hz margin", [B("PM-0001-39/B-S", "rule B-S, at 60 Hz")]),
    ("PM-0001-40", "The research's other 60 Hz marks", []),
    ("PM-0001-63", "The goal budget holds at the rooms per VM measured at 60 Hz",
     [B("PM-0001-63/busy-20", "a busy-wait of 20 % of the single-room p99 in every tick of one room must fail that room's row at k = 4 (PM-0006-13's bite, at 60 Hz)"),
      B("PM-0001-63/k60-1", "the cost script fed k60 = 1 must report 21 CCX23 and 1,831.77 EUR (21 x 86.49 + 8.99 + 6.49) and 'does not hold' against both provisional figures")]),
]:
    marks.append(M(mid, title, "outside", [], [], [], [], [], bites, [],
                   "Not in this change-set: the 60 Hz run, in its own change-set, replaces npt-sim through the seam and changes no other crate (plan §4.6, §6.2). Always NOT-RUN here. The bites are kept as data for that change-set."))

emit(marks)
