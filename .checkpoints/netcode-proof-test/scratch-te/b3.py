import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

marks = []
LGF = {"machine.role": "LG"}
BOTS = {"process": "bots"}
BR = {"process": "browser"}
SUPER = {"process": "super"}
RSM = {"machine.role": "RS"}

# ---------------- PM-0001-19 ----------------
j19 = ["LG joins", "ML Chrome"]
marks.append(M(
    "PM-0001-19", "A client joining mid-match, or resyncing, has the whole world fast", "pass", ["R3", "R7", "ML*"],
    [P("LG joins", {"layer": "L2", "profile": "P2", "params.suite": "joins", "machine.role": "LG"}, 1),
     P("ML Chrome", {"layer": "L2", "profile": "P2", "params.suite": "joins", "machine.name": "ML", "params.browser": "chrome"}, 1),
     P("ML* Chrome (rough, reported)", {"layer": "L2", "profile": "P2", "params.suite": "joins", "machine.name": "ML*", "params.browser": "chrome"}, 1)],
    [],
    [C("every join: a complete world within 300 ms of sending Join", "event.join.world_ns", "max", "<=", 300_000_000, window="all", parts=j19),
     C("100 joins into the running S30 room", "event.join.world_ns", "count", ">=", 100, window="all", parts=["LG joins"]),
     C("the browser's joins and rejoins after Worker traps were timed", "event.join.world_ns", "count", ">=", 1, window="all", parts=["ML Chrome"]),
     F("the joined client's hash equals the server's (LG under the Arm rule)", "join_hash_mismatch_count", "==", 0, parts=j19)],
    [R("join time p50", "event.join.world_ns", "p50", window="all"),
     R("join time p99", "event.join.world_ns", "p99", window="all"),
     R("join time max", "event.join.world_ns", "max", window="all"),
     R("from the page's first connect call, p50", "event.join.from_connect_ns", "p50", window="all"),
     R("from the page's first connect call, p99", "event.join.from_connect_ns", "p99", window="all"),
     R("from the page's first connect call, max", "event.join.from_connect_ns", "max", window="all"),
     R("aarch64-only join mismatches (decide nothing)", figure="cross_arch_mismatch_count")],
    [B("PM-0001-19/fullstate-datagrams", "the first full state sent as datagrams instead of on its stream")],
    ["R-8"],
    "L2 at P2: 100 joins into a running S30 room (proposed, R3 VS4), including rejoins after a trap injected into the client's Worker (ADR 0004's S20 injects them; this row times them; R7 on ML). A plain limit. ML's Chrome (Wasm) is judged as it stands; an LG mismatch counts only once its logged bytes, decoded again on x86-64, miss the server's hash too (Arm rule); the 300 ms counts as measured. The row's 100 joins are read as LG's; the browser leg's count is not in the row (question). The ML part waits for the named laptop; ML* is reported.",
    arm_rule=True))

# ---------------- PM-0001-20 ----------------
parts20 = []
for lay, mn in [("L1", 3), ("L2", 1)]:
    for opt in ["A", "B"]:
        sel = {"layer": lay, "profile": "P3", "params.cmd_timing": opt}
        if lay == "L2":
            sel["machine.role"] = "LG"
        parts20.append(P(f"{lay} P3 option {opt}", sel, mn))
marks.append(M(
    "PM-0001-20", "Commands arrive in time and count once", "pass", ["C", "R3"],
    parts20, ["reorder"],
    [C("at least 99.9 % of commands reach the server before their tick runs", "tick.s.cmd_on_time", "share", ">=", "0.999", where={"op": "==", "value": 1}, **{"from": ROOM}),
     C("missing inputs at most 0.1 % of player-ticks", "tick.s.input_missing", "share", "<=", "0.001", where={"op": "==", "value": 1}, **{"from": ROOM}),
     C("10,000 scripted presses", "tick.v.press_count", "sum", ">=", 10_000, **{"from": ROOM}),
     C("0 presses lost", "tick.v.press_lost_count", "sum", "==", 0, **{"from": ROOM}),
     C("0 presses doubled", "tick.v.press_doubled_count", "sum", "==", 0, **{"from": ROOM})],
    [R("input buffer depth per client, median (ticks)", "second.s.buffer_depth_count", "p50", **{"from": ROOM}),
     R("input buffer depth per client, max (ticks)", "second.s.buffer_depth_count", "max", **{"from": ROOM}),
     R("measured reorder share", "header.params.reorder_permille", "max")],
    [B("PM-0001-20/redundancy-1", "a redundancy cap of 1 (no repeats) fails the 99.9 %")],
    ["R-5"],
    "48 clients, 10 minutes under P3, under each of OQ-0001-6's two command-timing options (`params.cmd_timing`; PM-0001-56 records them); L1 3 seeds, L2 1 run (R3, VS4). The server logs each command's arrival against its tick, each press counter's edges and each client's buffer depth every second. The buffer depth feeds PM-0001-49 (each buffered tick adds 33.3 ms to every honest rewind). The reorder rule applies: the presses run 'with reordering'."))

# ---------------- PM-0001-21 ----------------
SUITES = ["forged", "clamp", "fast-clock", "lag-switch"]
parts21 = []
for su in SUITES:
    for opt in ["A", "B"]:
        sel = {"layer": "L1", "params.suite": su, "params.cmd_timing": opt}
        if su == "lag-switch":
            sel["profile"] = "P1"
        parts21.append(P(f"{su}, option {opt}", sel, 3))
pp = lambda su: [f"{su}, option A", f"{su}, option B"]
marks.append(M(
    "PM-0001-21", "Forged commands and lag switches change nothing", "pass", ["C"],
    parts21, [],
    [C("forged: 1,000 commands of every kind", "event.forged_kind.sent_count", "min", ">=", 1000, window="all", parts=pp("forged")),
     C("forged: every kind of part 5's set sent (11 kinds)", "event.forged_kind.sent_count", "count", ">=", 11, window="all", parts=pp("forged")),
     C("forged: every forged command rejected and counted", "event.forged_kind.unrejected_count", "max", "==", 0, window="all", parts=pp("forged")),
     F("forged: 0 effect beyond the valid part (state hash equals the valid-parts twin's on every tick)", "twin_hash_mismatch_count", "==", 0, parts=pp("forged")),
     C("clamp: 1,000 points beyond reach per weapon kind the sim has", "event.clamp_kind.sent_count", "min", ">=", 1000, window="all", parts=pp("clamp")),
     C("clamp: never refused or counted", "event.clamp_kind.counted_count", "sum", "==", 0, window="all", parts=pp("clamp")),
     F("clamp: state hash equals the twin's whose points sit at the reach", "twin_hash_mismatch_count", "==", 0, parts=pp("clamp")),
     C("fast clock: 10 runs of 60 s", "event.fast_clock.excess_cmds_count", "count", ">=", 10, window="all", parts=pp("fast-clock")),
     C("fast clock: at most the server's ticks plus the lead bound of commands", "event.fast_clock.excess_cmds_count", "max", "<=", 0, window="all", parts=pp("fast-clock")),
     C("fast clock: moves no further than the honest client plus that many ticks of movement", "event.fast_clock.excess_move_um", "max", "<=", 0, window="all", parts=pp("fast-clock")),
     C("lag switch: 100 bursts (inputs held 200 ms under P1)", "event.lag_burst.held_ns", "count", ">=", 100, window="all", parts=pp("lag-switch")),
     C("lag switch: no tick moves further than the catch-up cap allows", "tick.v.lagswitch_excess_move_um", "max", "<=", 0, window="all", parts=pp("lag-switch"))],
    [R("lag switch: commands run per tick for that client, max", "tick.v.lagswitch_cmds_count", "max", window="all", parts=pp("lag-switch"))],
    [B(f"PM-0001-21/no-{k}", f"the {w} check removed: its forged kind gets through")
     for k, w in [("lead-bound", "lead-bound (tick not too far ahead)"), ("newer-tick", "strictly-newer-tick"),
                  ("seq-forward", "sequence-forward"), ("move-length", "move-length-at-most-1"),
                  ("press-edge", "one-press-edge-per-tick"), ("aim-bounds", "aim-inside-the-map"),
                  ("cooldown", "cooldown, ammunition and reload"), ("ack-sent", "acknowledged-snapshot-was-sent"),
                  ("trailing", "trailing-bytes-and-unknown-fields"), ("catchup-cap", "commands-per-tick (catch-up) cap"),
                  ("channel", "wrong-channel")]]
    + [B("PM-0001-21/not-older-than", "the tick check relaxed to 'not older than' lets the fast clock run the per-tick cap on every tick under OQ-0001-6 (a)"),
       B("PM-0001-21/refuse-not-clamp", "a refusal in place of the clamp counts honest far clicks as forged")],
    [],
    "L1, 3 seeds, each suite under each OQ-0001-6 option. Kinds: part 5's checks (ADR 0001:231-239) plus the wrong channel (plan §4.4), 11 in all; the beyond-reach point leaves the set (clamped) and the equal-or-older tick joins it. npt-lab writes one `forged_kind` event per kind (sent, and sent minus rejected-and-counted) and one `clamp_kind` per weapon kind (hitscan; melee and throw where built). The fast clock (1.5x for 60 s) and lag switch write their excess over the bound as signed integers (0 or less passes), with the bound's inputs beside them. A twin is the run with only valid parts (or with points at the reach), named in `params.twin`, role `twin`. Each bite run matches its suite's part (the no-* bites the forged suite; not-older-than the fast clock under option A; refuse-not-clamp the clamp suite)."))

# ---------------- PM-0001-22 ----------------
marks.append(M(
    "PM-0001-22", "A client that never acknowledges cannot grow the server", "pass", ["C"],
    [P("L1, one never-acker among 48", {"layer": "L1", "params.suite": "never-ack"}, 3)],
    [],
    [C("its event queue held at the cap: it reaches the cap and never passes it (headroom = cap - length)", "tick.v.ack_starved_headroom_count", "min", "==", 0, window="all", **{"from": ROOM}),
     C("the client resynced or dropped", "event.ack_starved.dropped", "count", ">=", 1, window="all", **{"from": ROOM})],
    [R("dropped rather than resynced", "event.ack_starved.dropped", "share", where={"op": "==", "value": 1}, window="all", **{"from": ROOM}),
     R("resident memory max", "second.v.rss_bytes", "max", window="all", **{"from": ROOM})],
    [B("PM-0001-22/unbounded", "an unbounded event queue; a queue bite, so over a full S30 of simulated time")],
    [],
    "L1, one non-acknowledging client among 48 for 10 minutes, 3 seeds. The room writes the never-acker's queue headroom (cap minus length) per tick, so an unbounded queue goes negative. The cap is a design constant written in `header.params.event_queue_cap_count` and reported."))

# ---------------- PM-0001-23 ----------------
marks.append(M(
    "PM-0001-23", "A throttled link degrades by priority", "pass", ["R3"],
    [P("L2 P2, 10 throttles", {"layer": "L2", "profile": "P2", "params.suite": "throttle"}, 1)],
    [],
    [C("at most 2 missing snapshots per second at the throttled client", "second.s.throttle_missing_snaps_count", "max", "<=", 2, **{"from": BOTS}),
     C("the local player's own state in 100 % of the snapshots sent", "tick.s.own_state_sent", "min", "==", 1, **{"from": ROOM}),
     C("full rate back within 2 s of the throttle's end", "event.throttle_end.full_rate_ns", "max", "<=", 2_000_000_000, **{"from": BOTS}),
     C("10 throttles", "event.throttle_end.full_rate_ns", "count", ">=", 10, **{"from": BOTS}),
     C("setup: the drop-tail queue holds at least 4 full datagrams", "header.params.throttle_queue_datagrams_count", "min", ">=", 4)],
    [R("the queue's depth in bytes (about 4.1 KB)", "header.params.throttle_queue_bytes", "max")],
    [B("PM-0001-23/own-state-ranked", "the own state ranked like any entity")],
    ["R-17"],
    "L2's rate limit on RS with LG: 128 kbps for 10 s, 10 throttles per run (proposed), P2 otherwise; 1 run. `second.s.throttle_missing_snaps_count` holds one value per throttled client per throttled second. The queue depths go in the run header (R-17); the setup check would better be INVALID than FAIL (question)."))

# ---------------- PM-0001-24 ----------------
marks.append(M(
    "PM-0001-24", "A bottleneck never stacks stale snapshots", "pass", ["R3"],
    [P("NewReno", {"layer": "L2", "profile": "P2", "params.suite": "bottleneck", "params.cc": "newreno"}, 1),
     P("Cubic", {"layer": "L2", "profile": "P2", "params.suite": "bottleneck", "params.cc": "cubic"}, 1)],
    ["V-S", "clean-ref"],
    [C("the newest snapshot the client holds never older than 3 intervals (100 ms)", "second.s.snap_age_ns", "max", "<=", 100_000_000, **{"from": BOTS}),
     F("memory per session flat over 10 minutes (PM-0001-15's rule, halves of the window)", "session_memory_growth_count", "==", 0),
     C("server tick p99 within 5 % of a clean run", "tick.v.work_ns", "p99", "<=", factor="1.05",
       against={"roles": ["clean"], "combine": "median"}, **{"from": ROOM}),
     C("setup: the queue holds at least 4 datagrams", "header.params.bottleneck_queue_datagrams_count", "min", ">=", 4),
     C("setup: the queue holds at least 200 ms at the bottleneck rate", "header.params.bottleneck_queue_ns", "min", ">=", 200_000_000)],
    [R("p99 snapshot age per controller (the lower one is picked)", "second.s.snap_age_ns", "p99", **{"from": BOTS})],
    [B("PM-0001-24/blocking-send", "a blocking send that queues every snapshot")],
    ["R-17"],
    "L2 on RS with LG: a bottleneck at 50 % of the snapshot stream's rate, 10 minutes per controller at P2, 1 run per controller. Age on the client from the server's send stamp and the clock estimate. The clean reference is 3 clean runs (role `clean`) in the same set whose p99s lie within 2.5 % of their median. 'Flat over 10 minutes' compares the second half's max with the first half's (the 10-minute run has no 'first and last 10 minutes': proposed). V-S is read from 'V-S's clean reference' and the tick-time number. The pick (lower p99 age) is the session's reading of the report."))

# ---------------- PM-0001-25 ----------------
flood = []
for tr in ["wt", "ws"]:
    for cap in ["raised", "on"]:
        flood.append(P(f"flood {tr}, cap {cap}", {"params.suite": "flood", "params.transport": tr, "params.addr_cap": cap}, 1))
flood.append(P("flood wt, cap on, IPv6 /64 cycler", {"params.suite": "flood", "params.transport": "wt", "params.addr_cap": "on", "params.lg_ipv6": True}, 1))
fl = [p["name"] for p in flood]
origin = []
for mach, sel, brs in [("ML", {"machine.name": "ML"}, ["chrome", "edge", "firefox"]), ("MAC", {"machine.role": "MAC"}, ["safari"])]:
    for b in brs:
        s = dict(sel); s.update({"params.suite": "origin", "params.browser": b})
        origin.append(P(f"Origin {mach} {b}", s, 1))
origin.append(P("Origin ML* (rough, reported)", {"params.suite": "origin", "machine.name": "ML*"}, 1))
og = [p["name"] for p in origin if "rough" not in p["name"]]
raised = ["flood wt, cap raised", "flood ws, cap raised"]
capon = ["flood wt, cap on", "flood ws, cap on"]
marks.append(M(
    "PM-0001-25", "Hostile input is contained on both transports", "pass", ["R4", "ML*", "MAC-r"],
    flood + origin, ["V-S", "clean-ref"],
    [C("tick p99 within 10 % of a clean run", "tick.v.work_ns", "p99", "<=", factor="1.10",
       against={"roles": ["clean"], "combine": "median"}, **{"from": ROOM, "parts": fl}),
     C("resident memory under 64 MB", "second.v.rss_bytes", "max", "<", 64_000_000, window="all", **{"from": ROOM, "parts": fl}),
     C("1,000 sessions that never send Join", "event.no_join_close.age_ns", "count", ">=", 1000, window="all", **{"from": ROOM, "parts": raised}),
     C("a session without Join closed 2 s after its TCP accept or first QUIC packet", "event.no_join_close.age_ns", "max", "<=", 2_000_000_000, window="all", **{"from": ROOM, "parts": fl}),
     C("10 slow-loris clients", "event.slow_loris_close.age_ns", "count", ">=", 10, window="all", **{"from": ROOM, "parts": fl}),
     C("each slow-loris closed by that deadline", "event.slow_loris_close.age_ns", "max", "<=", 2_000_000_000, window="all", **{"from": ROOM, "parts": fl}),
     C("handshakes in progress held at their cap (headroom = cap - in progress)", "second.v.handshake_headroom_count", "min", ">=", 0, window="all", **{"from": ROOM, "parts": fl}),
     C("cap on: the first connection over the cap from one address refused", "event.over_cap.refused", "min", "==", 1, window="all", **{"from": ROOM, "parts": capon}),
     C("cap on: an over-cap attempt was made", "event.over_cap.refused", "count", ">=", 1, window="all", **{"from": ROOM, "parts": capon}),
     C("cap on: the /64 cycler's first connection over the cap refused", "event.over_cap_v6.refused", "min", "==", 1, window="all", **{"from": ROOM, "parts": ["flood wt, cap on, IPv6 /64 cycler"]}),
     C("cap on: the cycler ran (1,000 addresses in one /64)", "event.over_cap_v6.refused", "count", ">=", 1, window="all", **{"from": ROOM, "parts": ["flood wt, cap on, IPv6 /64 cycler"]}),
     C("ws:// and a wrong Origin refused with 403, low", "event.refusal.status", "min", ">=", 403, window="all", **{"from": BR, "parts": og}),
     C("ws:// and a wrong Origin refused with 403, high", "event.refusal.status", "max", "<=", 403, window="all", **{"from": BR, "parts": og}),
     C("both probes ran (ws:// and wrong Origin)", "event.refusal.status", "count", ">=", 2, window="all", **{"from": BR, "parts": og})],
    [R("ML*: refusal statuses", "event.refusal.status", "max", window="all", **{"from": BR, "parts": ["Origin ML* (rough, reported)"]}),
     R("LG has IPv6", "header.params.lg_ipv6", "max")],
    [B("PM-0001-25/default-limits", "the library's default limits restored"),
     B("PM-0001-25/deadline-from-upgrade", "the Join deadline counted from the upgrade's end (the slow-loris stays open)"),
     B("PM-0001-25/v6-whole-address", "per-address limits keyed by the whole IPv6 address (the /64 cycler gets through)")],
    ["R-25"],
    "R4: S30's room under each flood for 10 minutes (proposed), twice per transport: the per-address cap raised to the run's session count on LG's addresses, then on; WebSocket adds oversized frames and messages. The flood: 1,000 never-Join sessions plus 10 clients sending oversize datagrams at full rate and opening streams. Origin probes from the four browsers (ML: Chrome, Edge, Firefox; Safari on MAC; a runner may stand as MAC for Origin, R-13). V-S and clean-ref concern the flood parts only: the contract's validity is per mark (question). The 2 s deadline is checked as `<= 2 s` exactly; a tolerance for the sweep's granularity is not in the row (question). The IPv6 part runs only where LG has IPv6 ('recorded if it has not'): without it the part and the v6 bite stay NOT-RUN, the record (question)."))

# ---------------- PM-0001-26 ----------------
marks.append(M(
    "PM-0001-26", "A panic in one connection ends only that connection", "pass", ["R3"],
    [P("one S30 room, 10 injections", {"layer": "L2", "scene": "SS", "params.suite": "conn-panic"}, 1)],
    [],
    [C("10 injections", "event.conn_panic.closed", "count", ">=", 10, window="all", **{"from": ROOM}),
     C("the panicking connection closes", "event.conn_panic.closed", "min", "==", 1, window="all", **{"from": ROOM}),
     C("0 of the other 47 connections drop", "event.conn_panic.others_dropped_count", "sum", "==", 0, window="all", **{"from": ROOM}),
     C("0 overruns in the following minute", "event.conn_panic.overruns_next_min_count", "sum", "==", 0, window="all", **{"from": ROOM})],
    [],
    [B("PM-0001-26/abort", "`panic = \"abort\"` in the build"),
     B("PM-0001-26/shared-mutex", "a std::sync::Mutex shared by the tick and the connection handlers, held by the handler when it panics (the poisoned lock panics the tick)")],
    [],
    "R3 (VS5): a panic injected into one connection's handler, 10 times in one S30 room (proposed). The room writes one `conn_panic` event per injection a minute after it, with the closed flag and the next minute's other-connection drops and overruns."))

# ---------------- PM-0001-59 ----------------
marks.append(M(
    "PM-0001-59", "A panic on the tick thread ends the match and the process", "pass", ["R3"],
    [P("10 S30 rooms, one tick panic each", {"layer": "L2", "scene": "SS", "params.suite": "tick-panic"}, 10)],
    [],
    [C("the process exits with a non-zero code", "event.room_exit.code", "min", ">=", 1, window="all", **{"from": SUPER}),
     C("the supervisor's stand-in logs the exit", "event.room_exit.code", "count", ">=", 1, window="all", **{"from": SUPER}),
     C("within 1 s of the panic", "event.room_exit.after_panic_ns", "max", "<=", 1_000_000_000, window="all", **{"from": SUPER}),
     F("every client gets RoomEnded(crash) or a close within that second (0 left open)", "crash_close_max_ns", "<=", 1_000_000_000),
     C("the match record written and marked crashed", "event.records_check.match_crashed", "min", "==", 1, window="all", **{"from": SUPER}),
     C("it and the crash report kept", "event.records_check.crash_report_kept", "min", "==", 1, window="all", **{"from": SUPER}),
     C("the report reached the error log", "event.records_check.error_log", "min", "==", 1, window="all", **{"from": SUPER})],
    [R("the panic's tick after warm-up", "event.tick_panic.after_warmup_ticks_count", "max", window="all", **{"from": ROOM})],
    [B("PM-0001-59/no-catch", "a tick thread whose panic leaves the process running (the loop without its top-level catch, under panic = \"unwind\")")],
    ["R-16"],
    "R3 (VS5): 10 injections into the sim step, each in its own S30 room, at a random tick within the first 5 minutes after warm-up (R-16), so 10 runs. Each room's log is cut by its own exit (`params.killed` true exempts it from the `complete` rule, or the catch writes the footer with `ended` = crash). The supervisor's stand-in writes `room_exit` (code, ns after the panic, from the crash record's monotonic stamp); the match record and crash report are read back after the last injection (`records_check`). The figure joins each LG client's close (on its server-clock estimate) with the room's panic tick. The error log is ADR 0006 part 10's endpoint once it exists: until then the stand-in's file."))

# ---------------- PM-0001-27 ----------------
p27 = []
for d in ["D1", "D2", "D3"]:
    p27.append(P(f"{d} surviving rooms", {"params.density": "8-rooms", "params.session": d, "params.killed": False}, 7))
    p27.append(P(f"{d} killed room", {"params.density": "8-rooms", "params.session": d, "params.killed": True}, 1))
surv = [f"{d} surviving rooms" for d in ["D1", "D2", "D3"]]
marks.append(M(
    "PM-0001-27", "Rooms share a VM fairly", "pass", ["R5"],
    p27, ["PM-0006-13"],
    [C("each room's tick p99 within 10 % of the single-room figure (bracket B: mean of the lone room's p99, first and last)", "tick.v.work_ns", "p99", "<=", factor="1.10",
       against={"roles": ["bracket-first", "bracket-last"], "combine": "mean"}, **{"from": ROOM}),
     F("after SIGKILL of one room, the others' p99 within 5 % of their own before it", "kill_p99_ratio", "<=", "1.05", parts=surv),
     C("0 of the other rooms' connections dropped", "second.v.disconnects_count", "sum", "==", 0, window="all", **{"from": ROOM, "parts": surv})],
    [R("each process's CPU share of one vCPU, mean", "second.v.cpu_permille", "mean", **{"from": ROOM}),
     R("each process's CPU share of one vCPU, max", "second.v.cpu_permille", "max", **{"from": ROOM}),
     R("steal per vCPU, mean", "second.v.steal_permille", "mean", **{"from": RSM}),
     R("steal per vCPU, max", "second.v.steal_permille", "max", **{"from": RSM})],
    [],
    ["R-26"],
    "PM-0006-13's setup and runs (R5, D1 to D3: three times of day): 8 room processes under the supervisor, no pinning, each SS with a snapshot client on every soldier (8 x 48 = 384 sessions). Each room is its own run, the session its set (so `against` finds the session's bracket runs). The kill comes after the 20 measured minutes; the survivors' ticks after it are phase `after`, and the figure compares their p99 with the measured p99 ('within 5 %' read as not worse by more than 5 %: proposed, question). The killed room's log has `params.killed` true. Steal is reported, never a validity condition. The row names no bite of its own; PM-0006-13's bite run (a busy-wait of 20 % of the single-room p99 in one room) could list -27 too (question). 'Every valid session passes': an INVALID session makes the mark INVALID under the contract."))

emit(marks)
