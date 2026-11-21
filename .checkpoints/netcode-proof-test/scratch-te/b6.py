import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

marks = []
BR = {"process": "browser"}
def mlp(name, extra):
    s = {"layer": "L2", "machine.name": "ML", "params.browser": "chrome"}; s.update(extra); return P(name, s, 1)
def roughp(name, extra):
    s = {"layer": "L2", "machine.name": "ML*"}; s.update(extra); return P(name, s, 1)

# ---------------- PM-0001-49 ----------------
p49 = []
for lay, mn in [("L1", 3), ("L2", 1)]:
    for p in ["P0", "P1", "P2", "P3", "P4"]:
        for opt in ["A", "B"]:
            s = {"layer": lay, "scene": "TC", "profile": p, "params.cmd_timing": opt, "params.cap": "test"}
            if lay == "L2":
                s["machine.role"] = "LG"
            p49.append(P(f"test cap {lay} {p} option {opt}", s, mn))
for lay, mn in [("L1", 3), ("L2", 1)]:
    for p in ["P0", "P1", "P2", "P3", "P4"]:
        s = {"layer": lay, "scene": "TC", "profile": p, "params.cap": "rule"}
        if lay == "L2":
            s["machine.role"] = "LG"
        p49.append(P(f"rule cap {lay} {p}", s, mn))
rule = [p["name"] for p in p49 if p["name"].startswith("rule")]
marks.append(M(
    "PM-0001-49", "The rewind cap holds; what it clips is known", "pass", ["C", "R3", "R7"],
    p49, [],
    [C("100 % of shots resolved with a rewind at most the cap, honest and cheat clients", "event.shot.over_cap", "sum", "==", 0, window="all", **{"from": ROOM}),
     C("the cheat clients of PM-0001-50 among the shooters, the 150 ms pong delayer included", "event.shot.cheat", "sum", ">=", 1, window="all", **{"from": ROOM, "parts": rule}),
     F("the rule cap is the picked option's honest-rewind p99 at P2, rounded up to whole ticks", "rule_cap_set_by_rule", "==", 1, parts=rule)],
    [R("the rule cap (ticks)", "header.params.rewind_cap_ticks_count", "max"),
     R("the run's largest rewind", "event.shot.rewind_ns", "max", window="all", **{"from": ROOM}),
     R("share of shots the cap clips", "event.shot.clipped", "share", where={"op": "==", "value": 1}, window="all", **{"from": ROOM}),
     R("registration with the cap (PM-0001-48's measure)", figure="view_hit_registered_share"),
     R("honest rewind, median", "event.shot.honest_rewind_ns", "p50", window="all", **{"from": ROOM}),
     R("honest rewind, p99", "event.shot.honest_rewind_ns", "p99", window="all", **{"from": ROOM}),
     R("honest rewind, max", "event.shot.honest_rewind_ns", "max", window="all", **{"from": ROOM}),
     R("its part: RTT, p50", "event.shot.rtt_ns", "p50", window="all", **{"from": ROOM}),
     R("its part: the announced delay, p50", "event.shot.delay_ns", "p50", window="all", **{"from": ROOM}),
     R("its part: the buffer wait, p50", "event.shot.buffer_wait_ns", "p50", window="all", **{"from": ROOM}),
     R("its part: the blend fraction, p50", "event.shot.blend_ns", "p50", window="all", **{"from": ROOM}),
     R("its part: the drawn frame to the sending tick, p50", "event.shot.frame_wait_ns", "p50", window="all", **{"from": ROOM})],
    [B("PM-0001-49/cap-estimate-only", "the cap applied to the estimate only, not to the final rewind")],
    ["R-20", "R-28"],
    "PM-0001-48's shots: the honest rewind read from its test-cap runs (both options; P4 added here), then rule-cap runs with the picked option (`params.cap` = rule; -49's rule-cap run opens R7), with PM-0001-50's cheat clients among the shooters; L1 3 seeds, L2 1 run per profile. The cap's rule is OQ-0001-5's, answered by Mustafa (2026-11-03): the honest rewind's p99 at P2, rounded up to whole ticks; it feeds checklist steps 7 and 10 and is read-only on the overlay (R-28). Honest rewind = how far behind the server's present the client's view is when its command runs; each shot's parts are logged. Reports are per part (question). The row's second 'bite', a client delaying every pong by 150 ms, must still meet the cap on 100 % of its shots: it is a case that must pass, not a defect that must fail, so it rides in the rule-cap runs' cheat set (`event.shot.cheat`), not in `bites` (question)."))

# ---------------- PM-0001-50 ----------------
marks.append(M(
    "PM-0001-50", "A cheat cannot buy rewind", "pass", ["C"],
    [P("L1 P0", {"layer": "L1", "profile": "P0", "params.suite": "rewind-cheat"}, 3),
     P("L1 P1", {"layer": "L1", "profile": "P1", "params.suite": "rewind-cheat"}, 3),
     P("L1 WebSocket path (recorded)", {"layer": "L1", "params.suite": "rewind-cheat", "params.transport": "ws"}, 1)],
    [],
    [C("1,000 shots of the tolerance-edge cheat", "event.edge_cheat_shot.gain_ns", "count", ">=", 1000, window="all", parts=["L1 P0", "L1 P1"]),
     C("the tolerance-edge cheat gains at most 33.33 ms of rewind over an honest client at the same RTT", "event.edge_cheat_shot.gain_ns", "max", "<=", 33_330_000, window="all", parts=["L1 P0", "L1 P1"]),
     C("1,000 shots of the pong delayer", "event.pong_cheat_shot.gain_ns", "count", ">=", 1000, window="all", parts=["L1 P0", "L1 P1"]),
     C("the pong delayer gains at most the tolerance with the transport's own RTT", "event.pong_cheat_shot.gain_ns", "max", "<=", 33_330_000, window="all", parts=["L1 P0", "L1 P1"]),
     C("its logged gap between the transport RTT and its ping RTT at least 145 ms in every 1-second sample", "second.v.pong_cheat_gap_ns", "min", ">=", 145_000_000, window="all", parts=["L1 P0", "L1 P1"])],
    [R("WebSocket path: the pong delayer's extra rewind, max", "event.pong_cheat_shot.gain_ns", "max", window="all", parts=["L1 WebSocket path (recorded)"])],
    [B("PM-0001-50/no-tolerance-check", "the view time accepted without the tolerance check")],
    [],
    "L1, 1,000 shots per profile and client, 3 seeds. npt-lab pairs each cheat shot with an honest twin shot at the same RTT and writes the gain (cheat rewind minus honest rewind). L1's link RTT stands in for quinn's `rtt()`. 145 ms = 150 ms less PM-0001-33's 5 ms honest bound (proposed). On the WebSocket path, where the RTT is the lowest recent ping sample, the extra rewind is recorded (part 8's accepted risk, held to the cap by PM-0001-49); the profile of that leg is not in the row."))

# ---------------- PM-0001-51 ----------------
p51 = [P(f"L1 TC {p}", {"layer": "L1", "scene": "TC", "profile": p, "params.suite": "aim-height"}, 3) for p in ["P0", "P1", "P2", "P3"]]
p51 += [mlp("ML Chrome TC P2", {"scene": "TC", "profile": "P2", "params.suite": "aim-height"}),
        roughp("ML* (rough, reported)", {"scene": "TC", "profile": "P2", "params.suite": "aim-height"})]
j51 = [p["name"] for p in p51 if "rough" not in p["name"]]
ch51 = []
for h, lab in [("ground", "on the ground"), ("h2_5m", "at 2.5 m"), ("h4m", "at 4 m")]:
    ch51 += [C(f"1,000 shots {lab}", f"event.static_shot.err_{h}_um", "count", ">=", 1000, window="all", parts=j51),
             C(f"{lab}: the server's hit point within 5 cm of the client-picked point in at least 99.9 % of shots", f"event.static_shot.err_{h}_um", "share", ">=", "0.999", where={"op": "<=", "value": 50_000}, window="all", parts=j51)]
ch51 += [C("1,000 encoded commands checked", "event.aim_encoded.size_bits", "count", ">=", 1000, window="all", parts=j51),
         C("the aim at most 80 bits per command before delta coding", "event.aim_encoded.size_bits", "max", "<=", 80, window="all", parts=j51)]
marks.append(M(
    "PM-0001-51", "Aiming at height hits the clicked point", "pass", ["C", "R7", "ML*"],
    p51, [], ch51,
    [R("hit-point error p99, at 4 m", "event.static_shot.err_h4m_um", "p99", window="all"),
     R("the aim's field widths (bits, from the codec)", "header.params.aim_bits", "max")],
    [B("PM-0001-51/parallel-ray-10cm", "the server's ray built parallel to the client's (a direction) with a planted 10 cm fire-origin error")],
    [],
    "TC's static targets on the ground, at 2.5 m and at 4 m, P0 to P3; 1,000 shots per height and profile (proposed: one miss allowed per 1,000); L1 3 seeds, ML Chrome over L2 at P2 1 run. Each shot's event carries the error under its height's field, so every height has its own share. The aim's bits are read from the codec's field widths and checked on 1,000 encoded commands. The ML part waits for the named laptop."))

# ---------------- PM-0001-52 ----------------
marks.append(M(
    "PM-0001-52", "Hit feedback waits for the server", "pass", ["R1", "R4", "R7", "ML*", "MAC-r", "MP"],
    [P("ML Chrome, 1,000 scripted hits", {"layer": "L2", "machine.name": "ML", "params.browser": "chrome", "params.suite": "feedback"}, 1),
     P("every ML session", {"machine.name": "ML", "process": "browser"}, 1),
     P("every MAC session", {"machine.role": "MAC", "process": "browser"}, 1),
     P("MP: the checklist session", {"machine.role": "MP", "params.suite": "checklist"}, 1),
     P("ML* sessions (rough, reported)", {"machine.name": "ML*", "process": "browser"}, 1)],
    [],
    [C("0 damage numbers or kills on another soldier shown before the server's event for it", "second.v.early_feedback_count", "sum", "==", 0, window="all",
       **{"from": BR, "parts": ["ML Chrome, 1,000 scripted hits", "every ML session", "every MAC session", "MP: the checklist session"]}),
     C("1,000 scripted hits shown", "second.v.feedback_shown_count", "sum", ">=", 1000, window="all", **{"from": BR, "parts": ["ML Chrome, 1,000 scripted hits"]})],
    [R("ML*: early feedback", "second.v.early_feedback_count", "sum", window="all", **{"from": BR, "parts": ["ML* sessions (rough, reported)"]})],
    [B("PM-0001-52/predicted-damage", "damage shown on the client's own predicted hit")],
    [],
    "Every browser session of the spike lists -52, plus 1,000 scripted hits (proposed) on ML Chrome over L2 and Mustafa's checklist session on MP. The page logs each shown number and kill with its event id and the time the Worker received that event; `early_feedback_count` counts those shown before it. Per-second counts, so a clean session is a non-empty series of zeros. MP's page writes this log; his judgment stays -53's. The ML parts wait for the named laptop."))

# ---------------- PM-0001-62 ----------------
marks.append(M(
    "PM-0001-62", "Own shots play their cues once, however often they are replayed", "pass", ["C", "R7", "ML*"],
    [P("L1 P2, forced replays", {"layer": "L1", "profile": "P2", "params.suite": "cues"}, 3),
     mlp("ML Chrome P2, forced replays", {"profile": "P2", "params.suite": "cues"}),
     roughp("ML* (rough, reported)", {"profile": "P2", "params.suite": "cues"})],
    [],
    [C("1,000 scripted shots, flash and sound each (2,000 cue keys)", "event.own_cue.plays_count", "count", ">=", 2000, window="all", parts=["L1 P2, forced replays", "ML Chrome P2, forced replays"]),
     C("every own shot's flash and sound at least once (0 missing)", "event.own_cue.plays_count", "min", ">=", 1, window="all", parts=["L1 P2, forced replays", "ML Chrome P2, forced replays"]),
     C("and at most once (0 doubled)", "event.own_cue.plays_count", "max", "<=", 1, window="all", parts=["L1 P2, forced replays", "ML Chrome P2, forced replays"]),
     C("the server sends the shooter 0 cosmetic events for its own shots", "second.v.own_cosmetic_sent_count", "sum", "==", 0, window="all", **{"from": ROOM, "parts": ["L1 P2, forced replays", "ML Chrome P2, forced replays"]})],
    [R("ML*: doubled cues", "event.own_cue.plays_count", "max", window="all", parts=["ML* (rough, reported)"])],
    [B("PM-0001-62/cues-from-replay", "cues written from the replay (each replayed shot plays again)"),
     B("PM-0001-62/server-echo", "the server echoing the owner's shots")],
    [],
    "At P2 with replays forced as in PM-0001-44; 1,000 scripted shots (R1's count); L1 3 seeds, ML Chrome 1 run. One `own_cue` event per cue key (entity, weapon, command sequence) and cue kind, written when the shot's window closes, with how often it played: on L1 the client endpoint's emitted cues, on ML the page's played cues. The server's send log counts the shooter's own cosmetic events. The ML part waits for the named laptop."))

# ---------------- PM-0001-53 ----------------
marks.append(M(
    "PM-0001-53", "Mustafa's verdict: prediction, reconciliation and lag-compensated hits look correct at P2", "person", ["R7", "MP"],
    [P("Mustafa's checklist session (MP, Brave on Windows, P2)", {"machine.role": "MP", "profile": "P2", "params.suite": "checklist"}, 1)],
    [], [], [], [], ["R-28"],
    "Judged by Mustafa: one session on MP in Brave (a separate profile on his PC), at P2 (§11), beside the measurable marks -41 to -52 and -60 to -62. His session writes one `person` event (who = Mustafa, mark = PM-0001-53, verdict = pass only if every step of the netcode checklist is 'achieved', steps = each step with 'achieved' or 'not yet'), through the record form, under a per-run capability. A step marked 'not yet' goes back to the draft and is shown again. The cap on the overlay is read-only (R-28)."))

# ---------------- PM-0001-54 ----------------
marks.append(M(
    "PM-0001-54", "Client-side projectile flight holds, or E3b carries the kinds that still desync", "pass", ["C", "R3"],
    [P("L1", {"layer": "L1", "params.suite": "throws"}, 3),
     P("L2 LG", {"layer": "L2", "params.suite": "throws", "machine.role": "LG"}, 1)],
    [],
    [C("1,000 throws per projectile kind", "event.throw_kind.throws_count", "min", ">=", 1000, window="all"),
     F("0 desyncs of client-side flight against the server (Arm rule on L2)", "throw_desync_count", "==", 0)],
    [R("aarch64-only desyncs (decide nothing)", figure="cross_arch_mismatch_count")],
    [B("PM-0001-54/step-1-60", "a client trajectory step of 1/60 s against the server's 1/30 s")],
    ["R-8", "R-11"],
    "Conditional: only if live projectiles are in the load; R-11 rules that the stand-ins meet the condition (300 live projectiles through one shared trajectory function). 1,000 scripted throws per kind, each landing compared with the server's; L1 3 seeds, L2 LG. Arm rule: an LG throw that differs is fed again (start position, velocity, fuse) to the trajectory function on x86-64 (npt-lab) and counts only if it differs there too. A desync is first a defect of the shared trajectory function (PM-0004-73), fixed there and the row run again; only a kind that still desyncs rides in every snapshot (E3b for that kind alone), its live count x 5.5 B per snapshot shown against the 1,067 B cap and PM-0001-12 run again with it: that consequence is the session's, not the judge's.",
    arm_rule=True))

# ---------------- PM-0001-55 ----------------
marks.append(M(
    "PM-0001-55", "A soldier beside a moving vehicle is not corrected much", "pass", ["C"],
    [P("L1 P2, beside a moving big vehicle", {"layer": "L1", "profile": "P2", "params.suite": "beside-vehicle"}, 3)],
    [],
    [C("corrections larger than 5 cm in fewer than 2 % of snapshots", "second.s.snap_corr_over_5cm", "share", "<", "0.02", where={"op": "==", "value": 1})],
    [],
    [B("PM-0001-55/vehicle-snap", "a vehicle collision that snaps")],
    ["R-11", "R-12"],
    "Conditional: only if the sim moves vehicles (R-11: the stand-ins meet it). The local soldier walking beside a moving big vehicle for 10 minutes under P2, 3 seeds. The same per-snapshot flag as PM-0001-42."))

# ---------------- PM-0001-56 ----------------
marks.append(M(
    "PM-0001-56", "The proof test's records are complete", "pass", ALL_VMS + ["C"],
    [P("the record", {"params.suite": "record"}, 1)],
    [],
    [C("every listed item read once and written (10 items)", "event.record_item.has_value", "count", ">=", 10, window="all"),
     C("every item has a number or a value", "event.record_item.has_value", "min", "==", 1, window="all")],
    [],
    [],
    ["R-10"],
    "The record step reads each item once and writes one `record_item` event per item (`item` names it; `has_value` 1 when it has a number or value). The items: (1) both crates' results on PM-0001-11, -25, -28 and -35, and their pick with its reason (R2-TRN-OQ5); (2) each browser's negotiated draft and its behaviour with UDP blocked; (3) the real record sizes per entity kind and the command size; (4) NewReno against Cubic (-24) and each browser's received snapshot rate and age at P2 to P4; (5) the smoothness of a remote soldier sprinting straight and turning with linear interpolation (the standard deviation of its per-frame displacement as a share of the mean) beside Mustafa's step 5; (6) OQ-0001-6's two options, each with -42's correction share, -48's registration and -49's honest rewind at P2 to P4; (7) the absolute input-to-screen delay at P0 (-43); (8) the replay depth per profile (-44); (9) Brave's transport and version on MP; (10) R-10's cover item: both cover variants recorded on L1 (plan §6.2). Item (1) needs -11 and -25 run with both crates, which their own rows do not ask: the run plans must (question). No bite: the row names none."))

# ---------------- PM-0004-67 ----------------
stalls = ["stall runs"]
marks.append(M(
    "PM-0004-67", "The server loop keeps time (S17)", "pass", ["R2"],
    [P("jitter: S30 at P2", S30_P2, 3),
     P("stall runs", {"layer": "L2", "scene": "SS", "profile": "P2", "tick_hz": 30, "params.suite": "stalls"}, 3)],
    ["V-S"],
    [C("tick start jitter (actual minus scheduled start) p99 at most 2 ms", "tick.v.start_jitter_ns", "p99", "<=", 2_000_000, **{"from": ROOM, "parts": ["jitter: S30 at P2"]}),
     C("over 36,000 ticks", "tick.v.start_jitter_ns", "count", ">=", 36_000, **{"from": ROOM, "parts": ["jitter: S30 at P2"]}),
     C("never more than 7 ticks in one wake", "tick.v.wake_ticks_count", "max", "<=", 7, window="all", **{"from": ROOM}),
     C("a 200 ms stall injected", "event.stall_200ms.recover_ns", "count", ">=", 1, window="all", **{"from": ROOM, "parts": stalls}),
     C("after a 200 ms stall, back on schedule within 1 s (no ticks owed, every later start within 2 ms)", "event.stall_200ms.recover_ns", "max", "<=", 1_000_000_000, window="all", **{"from": ROOM, "parts": stalls}),
     C("a 1 s stall injected", "event.stall_1s.dropped_ns", "count", ">=", 1, window="all", **{"from": ROOM, "parts": stalls}),
     C("after a 1 s stall, the dropped time in the dropped-time counter", "event.stall_1s.dropped_ns", "min", ">=", 1, window="all", **{"from": ROOM, "parts": stalls}),
     C("overrun and dropped-time counters in the metrics output", "event.scrape.loop_counters_present", "min", "==", 1, window="all", **{"from": {"process": "load"}, "parts": stalls})],
    [R("jitter p50", "tick.v.start_jitter_ns", "p50", **{"from": ROOM}),
     R("jitter max", "tick.v.start_jitter_ns", "max", **{"from": ROOM}),
     R("1 s stall: dropped time against stall minus the catch-up bound (766.67 ms)", "event.stall_1s.dropped_ns", "max", window="all", **{"from": ROOM})],
    [B("PM-0004-67/timer-period", "a loop that counts ticks from its timer period instead of elapsed time"),
     B("PM-0004-67/uncapped-catchup", "an uncapped catch-up (more than 7 ticks in one wake)"),
     B("PM-0004-67/uncounted-drop", "a drop that is not counted in the dropped-time counter")],
    ["R-12", "R-15"],
    "RS (Linux), the proof test's room under its 30 Hz baseline load; start times from the loop's monotonic clock. The jitter is read from the first 36,000 measured ticks of each S30 (PM-0001-4's runs): the contract's `measured` window reads all 54,000, so a first-N window is needed (question). The 200 ms and 1 s stalls (a planted sleep on the tick thread at a fixed tick) run in 3 runs of their own, far apart in each, so no recovery falls inside the jitter count (R-15). 7 = floor(0.25 s x 30). The room writes one event per stall: `recover_ns` (from the stall's end until no tick is owed and every later start is within 2 ms) and `dropped_ns` (the counter's increase). The dropped-time check is the row's proposed 'in the counter' (above 0); PM-0006-23 holds the 33.33 ms bound. `due_ticks` and these bites are unit-tested in this container on a fake clock before any VM hour (plan §4.6); those tests are not this mark's runs."))

emit(marks)
