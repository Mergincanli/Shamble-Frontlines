import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

marks = []
BR = {"process": "browser"}
CL = {"process": "client"}  # the client endpoint's log: npt-lab on L1, the Worker on ML
def mlp(name, extra):
    s = {"layer": "L2", "machine.name": "ML", "params.browser": "chrome"}; s.update(extra); return P(name, s, 1)
def roughp(name, extra):
    s = {"layer": "L2", "machine.name": "ML*"}; s.update(extra); return P(name, s, 1)

# ---------------- PM-0001-41 ----------------
p41 = [P(f"L1 CO {p}", {"layer": "L1", "scene": "CO", "profile": p}, 3) for p in ["P0", "P1", "P2", "P3", "P4"]]
p41 += [mlp("ML Chrome CO P2", {"scene": "CO", "profile": "P2"}), roughp("ML* CO P2 (rough, reported)", {"scene": "CO", "profile": "P2"})]
j41 = [p["name"] for p in p41 if "rough" not in p["name"]]
marks.append(M(
    "PM-0001-41", "Prediction is exact when commands arrive in time", "pass", ["C", "R7", "ML*"],
    p41, [],
    [C("0 corrections above 1 mm not traced to a late or lost command (so 0 above 1 mm while every command is in time, and 100 % of the others traced)", "tick.v.untraced_corrections_count", "sum", "==", 0, **{"parts": j41}),
     C("the replay check ran on every tick", "tick.v.untraced_corrections_count", "count", ">=", 18_000, **{"parts": j41})],
    [R("corrections above 1 mm, all", "tick.v.corrections_count", "sum"),
     R("ML*: untraced corrections", "tick.v.untraced_corrections_count", "sum", parts=["ML* CO P2 (rough, reported)"])],
    [B("PM-0001-41/server-speed-1pct", "the server's run speed 1 % above the client's"),
     B("PM-0001-41/server-quant-differs", "the server quantising its own copy differently from what it sends")],
    [],
    "CO under P0 to P4, 10 minutes per profile (18,000 ticks); L1 (the client endpoint's Rust code, headless) 3 seeds per profile, ML Chrome over L2 1 run at P2. The client counts corrections from part 4's replay check and logs each command's arrival; a correction is traced when a command it replays arrived late or was lost. Per-tick counts, so a clean run is a non-empty series of zeros. The ML part waits for the named laptop."))

# ---------------- PM-0001-42 ----------------
p42 = []
for p in ["P2", "P3", "P4"]:
    for opt in ["A", "B"]:
        p42.append(P(f"L1 brawl {p} option {opt}", {"layer": "L1", "scene": "SS", "profile": p, "params.cmd_timing": opt, "params.suite": "brawl"}, 3))
p42 += [mlp("ML Chrome brawl P3 with LG's 47", {"scene": "SS", "profile": "P3", "params.suite": "brawl"}),
        roughp("ML* brawl P3 (rough, reported)", {"scene": "SS", "profile": "P3", "params.suite": "brawl"})]
p3_42 = ["L1 brawl P3 option A", "L1 brawl P3 option B", "ML Chrome brawl P3 with LG's 47"]
rep42 = [R(f"{p['name']}: snapshots with a correction above 5 cm", "second.s.snap_corr_over_5cm", "share", where={"op": "==", "value": 1}, parts=[p["name"]])
         for p in p42 if "P3" not in p["name"]]
marks.append(M(
    "PM-0001-42", "Corrections stay small in a crowd", "pass", ["C", "R7", "ML*"],
    p42, [],
    [C("under P3: corrections above 5 cm in fewer than 1 % of snapshots", "second.s.snap_corr_over_5cm", "share", "<", "0.01", where={"op": "==", "value": 1}, parts=p3_42),
     C("the drawn correction never moves the player more than 5 cm in one frame below the 2 m snap", "second.s.frame_correction_um", "max", "<=", 50_000, **{"from": BR, "parts": ["ML Chrome brawl P3 with LG's 47"]})],
    rep42 + [R("ML*: frames with the largest drawn correction", "second.s.frame_correction_um", "max", **{"from": BR, "parts": ["ML* brawl P3 (rough, reported)"]})],
    [B("PM-0001-42/smoothing-off", "smoothing off (a correction applied in one frame)")],
    [],
    "The local player in SS's fight, a brawl of 48, 10 minutes (proposed): ML Chrome over L2 with LG's 47 clients (1 run), L1 for the snapshot share (3 seeds). Plan §6.2 runs it under both OQ-0001-6 options at P2 to P4 (netcode NC2-F4): the row's number is at P3, so P3 is judged and P2 and P4 are reported (question). The soldier collision rule is named in the run's record (`header.params.collision_rule`): a pass holds for that rule only, and the row runs again when the rule changes. Snaps (2 m and above) are outside the frame check. The ML part waits for the named laptop."))

# ---------------- PM-0001-60 ----------------
p60 = [P("L1 P2", {"layer": "L1", "profile": "P2", "params.suite": "own-change"}, 3),
       mlp("ML Chrome P2", {"profile": "P2", "params.suite": "own-change"}),
       roughp("ML* P2 (rough, reported)", {"profile": "P2", "params.suite": "own-change"})]
j60 = ["L1 P2", "ML Chrome P2"]
marks.append(M(
    "PM-0001-60", "A server-side change to own movement or fire timing is predicted from the next snapshot on", "pass", ["C", "R7", "ML*"],
    p60, [],
    [C("100 changes of each kind (move speed, fire interval)", "event.own_change.corrections_count", "count", ">=", 200, window="all", parts=j60),
     C("at most one correction per change", "event.own_change.corrections_count", "max", "<=", 1, window="all", parts=j60),
     C("that correction at the first snapshot that carries the change", "event.own_change.misplaced_count", "sum", "==", 0, window="all", parts=j60),
     C("0 corrections above 1 mm after it while every command is in time", "event.own_change.later_over_1mm_count", "sum", "==", 0, window="all", parts=j60)],
    [R("the first correction's size, p50", "event.own_change.first_size_um", "p50", window="all"),
     R("the first correction's size, max", "event.own_change.first_size_um", "max", window="all"),
     R("time the client predicted without the change, p50", "event.own_change.blind_ns", "p50", window="all")],
    [B("PM-0001-60/own-state-missing", "the own-state section without the changed value (a correction on every snapshot while the change lasts)")],
    [],
    "At P2: 100 changes of each kind at random ticks (proposed: +-10 % of move speed, +-10 % of the fire interval, on a stand-in attribute of the throwaway sim, as ADR 0005's effects would set it); L1 3 seeds, ML Chrome 1 run. The client writes one `own_change` event per change, closed when the change ends. The report sets the first correction's size beside the change times the blind time (about one round trip plus the buffer wait). The ML part waits for the named laptop."))

# ---------------- PM-0001-43 ----------------
marks.append(M(
    "PM-0001-43", "Own movement and own shots feel the same at any latency", "pass", ["R7", "ML*"],
    [P("ML", {"params.suite": "input-latency", "machine.name": "ML"}, 4),
     P("RD", {"params.suite": "input-latency", "machine.role": "RD"}, 4),
     P("ML* (rough, reported)", {"params.suite": "input-latency", "machine.name": "ML*"}, 4)],
    [],
    [F("p99 at P4 minus p99 at P0 at most 33.33 ms, for own movement and for the muzzle flash", "input_latency_p99_rise_ns", "<=", 33_330_000, parts=["ML", "RD"]),
     F("the same for the medians", "input_latency_p50_rise_ns", "<=", 33_330_000, parts=["ML", "RD"]),
     C("the isolation flag on (crossOriginIsolated in the Worker)", "event.worker.cross_origin_isolated", "min", "==", 1, window="all", **{"from": BR, "parts": ["ML", "RD"]}),
     C("200 scripted inputs per profile and kind", "event.input_latency.to_screen_ns", "count", ">=", 200, window="all", **{"from": BR, "parts": ["ML", "RD"]})],
    [R("absolute input-to-screen at P0, p50 (per kind)", "event.input_latency.to_screen_ns", "p50", window="all", **{"from": BR}),
     R("absolute input-to-screen at P0, p99 (per kind)", "event.input_latency.to_screen_ns", "p99", window="all", **{"from": BR}),
     R("ML*: the p99 rise", figure="input_latency_p99_rise_ns", parts=["ML* (rough, reported)"])],
    [B("PM-0001-43/snapshots-only", "own movement drawn from snapshots only (no prediction)")],
    [],
    "ML and RD: Chrome on WebGPU, presented frames on a 60 Hz display, through `/iso/` (the isolation flag on); 200 scripted inputs per profile and kind sent through the Chrome DevTools Protocol; latency from the input event's time stamp to the first frame that draws its effect. 1 run per machine, profile (P0, P4) and kind (move, flash): 4 runs per machine. Judged on named machines only: RD has no stand-in, so this mark waits for RD (plan §6.3). 'The same' read on p99 and median (the row's proposal). The figures take the larger rise of the two kinds. The absolute P0 numbers are reported (spike item 5; OQ-0001-15)."))

# ---------------- PM-0001-44 ----------------
p44 = []
for b in ["chrome", "firefox"]:
    for d in [6, 7]:
        p44.append(P(f"ML {b} depth {d}", {"params.suite": "replay", "machine.name": "ML", "params.browser": b, "params.replay_depth": d}, 3))
j44 = [p["name"] for p in p44]
p44.append(P("ML* (rough, reported)", {"params.suite": "replay", "machine.name": "ML*"}, 1))
for p in ["P0", "P1", "P2", "P3", "P4"]:
    p44.append(P(f"real depth {p} (PM-0001-41's L1 runs, reported)", {"layer": "L1", "scene": "CO", "profile": p}, 1))
rep44 = [R(f"real replay depth {p}, median", "tick.v.replay_depth_count", "p50", parts=[f"real depth {p} (PM-0001-41's L1 runs, reported)"]) for p in ["P0", "P1", "P2", "P3", "P4"]]
rep44 += [R(f"real replay depth {p}, max", "tick.v.replay_depth_count", "max", parts=[f"real depth {p} (PM-0001-41's L1 runs, reported)"]) for p in ["P0", "P1", "P2", "P3", "P4"]]
rep44 += [R("replay time forced at P3's and P4's measured maximum depths, p99", "second.s.replay_forced_ns", "p99", **{"from": BR}),
          R("ML*: replay p99", "second.s.replay_ns", "p99", **{"from": BR, "parts": ["ML* (rough, reported)"]})]
marks.append(M(
    "PM-0001-44", "The replay fits on the minimum laptop", "pass", ["R7", "ML*"],
    p44, [],
    [C("reset and replay over the forced depth (6 or 7 ticks) at most 1 ms p99", "second.s.replay_ns", "p99", "<=", 1_000_000, **{"from": BR, "parts": j44}),
     C("18,000 replays per run", "second.s.replay_ns", "count", ">=", 18_000, **{"from": BR, "parts": j44}),
     C("the isolation flag on", "event.worker.cross_origin_isolated", "min", "==", 1, window="all", **{"from": BR, "parts": j44})],
    rep44,
    [B("PM-0001-44/B-S", "rule B-S applied to the replay: a busy-wait in it of (1 ms - measured p99 + 0.1 ms)")],
    [],
    "ML: Chrome and Firefox (two Wasm engines), in Wasm in the Worker; SS's fight at P2 for 10 minutes, the replay forced on every snapshot with exactly 6 ticks, then in separate runs with exactly 7; `performance.now()` around reset and replay, isolation flag on (the `/iso/` path); 3 runs per depth (18,000 replays each, p99 = the 17,820th). 7 ticks is part 4's deepest replay at P2 with a 3-tick buffer. The 12-tick 60 Hz form is PM-0001-40's (outside). The real depths are reported from PM-0001-41's L1 runs, which then list -44 too. The vehicle case only if the sim has a piloted vehicle. Judged on the named laptop only (R8); ML* is rough."))

# ---------------- PM-0001-45 ----------------
p45 = []
for p in ["P2", "P3", "B1"]:
    p45.append(mlp(f"ML Chrome {p}", {"profile": p, "params.suite": "extrapolation"}))
    p45.append(P(f"L1 {p}", {"layer": "L1", "profile": p, "params.suite": "extrapolation"}, 3))
p45.append(roughp("ML* (rough, reported)", {"params.suite": "extrapolation"}))
marks.append(M(
    "PM-0001-45", "Remote soldiers are drawn without guessing", "pass", ["C", "R7", "ML*"],
    p45, [],
    [C("P2: extrapolation in fewer than 1 % of rendered frames", "second.s.frame_extrapolated", "share", "<", "0.01", where={"op": "==", "value": 1}, **{"from": BR, "parts": ["ML Chrome P2"]}),
     C("P3: fewer than 3 % of rendered frames", "second.s.frame_extrapolated", "share", "<", "0.03", where={"op": "==", "value": 1}, **{"from": BR, "parts": ["ML Chrome P3"]}),
     C("B1: 0 frames extrapolated beyond one snapshot interval", "second.s.frame_extrap_over_interval", "sum", "==", 0, **{"from": BR, "parts": ["ML Chrome B1"]}),
     C("L1 P2: fewer than 1 % of blocks built by extrapolation", "tick.v.block_extrapolated", "share", "<", "0.01", where={"op": "==", "value": 1}, parts=["L1 P2"]),
     C("L1 P3: fewer than 3 % of blocks", "tick.v.block_extrapolated", "share", "<", "0.03", where={"op": "==", "value": 1}, parts=["L1 P3"]),
     C("L1 B1: 0 blocks extrapolated beyond one interval", "tick.v.block_extrap_over_interval", "sum", "==", 0, parts=["L1 B1"])],
    [R("ML*: extrapolated frames", "second.s.frame_extrapolated", "share", where={"op": "==", "value": 1}, **{"from": BR, "parts": ["ML* (rough, reported)"]})],
    [B("PM-0001-45/buffer-one-short", "R5-GATE-REC3's G11 mutation: a snapshot buffer one slot too short for P2")],
    [],
    "ML Chrome over L2, 1 run per profile, 10 minutes; L1 3 seeds for the per-tick count. The Worker flags each block built by extrapolation and the page counts frames drawn from flagged blocks; on L1 the client endpoint's blocks are counted per tick (L1 has no frames). The ML parts wait for the named laptop."))

# ---------------- PM-0001-46 ----------------
marks.append(M(
    "PM-0001-46", "The delay adapts, and respawns never slide", "pass", ["R7", "ML*"],
    [mlp("ML Chrome", {"params.suite": "delay-adapt"}), roughp("ML* (rough, reported)", {"params.suite": "delay-adapt"})],
    [],
    [C("20 changes each way between P1 and P3", "event.delay_settle.after_change_ns", "count", ">=", 40, window="all", parts=["ML Chrome"]),
     C("the delay settles within 5 s of a profile change", "event.delay_settle.after_change_ns", "max", "<=", 5_000_000_000, window="all", parts=["ML Chrome"]),
     C("the delay at least 2 intervals", "second.s.interp_delay_ns", "min", ">=", 66_666_666, parts=["ML Chrome"]),
     C("the delay at most 5 intervals", "second.s.interp_delay_ns", "max", "<=", 166_666_667, parts=["ML Chrome"]),
     C("an announced change of more than one interval per second clamped", "second.v.delay_step_abs_ns", "max", "<=", 33_333_334, parts=["ML Chrome"]),
     C("1,000 scripted respawns and teleports", "event.respawn.slide_frames_count", "count", ">=", 1000, window="all", **{"from": BR, "parts": ["ML Chrome"]}),
     C("0 visible slides", "event.respawn.slide_frames_count", "sum", "==", 0, window="all", **{"from": BR, "parts": ["ML Chrome"]})],
    [R("ML*: settle time max", "event.delay_settle.after_change_ns", "max", window="all", parts=["ML* (rough, reported)"])],
    [B("PM-0001-46/respawn-interpolated", "respawns interpolated like movement")],
    [],
    "ML Chrome over L2, 1 run: 20 changes each way between P1 and P3 (proposed), 1,000 respawns and teleports. 'Settled' (proposed): within +- half an interval of its final value, the mean of the next 30 s, and staying there; the Worker writes one `delay_settle` event per change. A slide: a rendered frame drawing the entity on the line between its old and new place, more than 0.1 m from both (the page counts them per respawn). 2 to 5 intervals at 30 Hz are 66.67 to 166.67 ms (rounded outward to whole ns). The ML part waits for the named laptop."))

# ---------------- PM-0001-61 ----------------
marks.append(M(
    "PM-0001-61", "Remote events play with the poses they belong to", "pass", ["C", "R7", "ML*"],
    [P("L1 P2", {"layer": "L1", "profile": "P2", "params.suite": "events"}, 3),
     mlp("ML Chrome P2 with LG's 47", {"profile": "P2", "params.suite": "events"}),
     roughp("ML* (rough, reported)", {"profile": "P2", "params.suite": "events"})],
    [],
    [C("L1: every remote spatial event in the first block whose delayed time reaches its tick (0 exceptions)", "tick.v.remote_event_misplaced_count", "sum", "==", 0, parts=["L1 P2"]),
     C("own, predicted and owner events in the block written at the tick that emitted or received them", "tick.v.own_event_misplaced_count", "sum", "==", 0, parts=["L1 P2", "ML Chrome P2 with LG's 47"]),
     C("ML: an event never plays after the first frame that draws its entity at or past its tick", "event.remote_event.lead_ns", "min", ">=", 0, window="all", **{"from": BR, "parts": ["ML Chrome P2 with LG's 47"]}),
     C("ML: at most 50.00 ms before it (one tick + one refresh at 60 Hz)", "event.remote_event.lead_ns", "max", "<=", 50_000_000, window="all", **{"from": BR, "parts": ["ML Chrome P2 with LG's 47"]})],
    [R("lead p50", "event.remote_event.lead_ns", "p50", window="all", **{"from": BR}),
     R("lead p99", "event.remote_event.lead_ns", "p99", window="all", **{"from": BR}),
     R("lead max", "event.remote_event.lead_ns", "max", window="all", **{"from": BR})],
    [B("PM-0001-61/on-arrival", "remote events released on arrival, into the newest block (at P2 they play one interpolation delay early, 100 to 166.7 ms)")],
    [],
    "SS's fight at P2, 10 minutes (proposed). The client endpoint logs each event's tick and the block it went into with that block's delayed time; the page logs per frame its drawn time and each event it plays (a test-build log); `lead_ns` = the first frame drawing the entity at or past the event's tick minus the first frame playing the event (positive: the event played first). Remote spatial events: a remote soldier's action start or cosmetic shot, a hit on or the death of a remote entity, a blast, a structure kill. The ML part waits for the named laptop."))

# ---------------- PM-0001-47 ----------------
marks.append(M(
    "PM-0001-47", "The client's clock is right", "pass", ["C"],
    [P("L1 P2, 1,000 connects", {"layer": "L1", "profile": "P2", "params.suite": "clock"}, 3)],
    [],
    [C("1,000 connects with random true offsets", "event.connect_clock.offset_err_ns", "count", ">=", 1000, window="all"),
     C("offset error at most 5 ms after 2 s of samples (p99 of |estimate - true|)", "event.connect_clock.offset_err_ns", "p99", "<=", 5_000_000, window="all")],
    [R("offset error max", "event.connect_clock.offset_err_ns", "max", window="all")],
    [B("PM-0001-47/newest-sample", "an estimator that takes the newest raw sample"),
     B("PM-0001-47/tick-quantised", "receive times quantised to the tick (each sample timed when the tick reads it, not at its message event)")],
    [],
    "L1 (the client endpoint's Rust code): each connect's estimate compared at 2 s with the simulator's true clock; 1,000 connects (proposed count), 3 seeds. `offset_err_ns` is the absolute error."))

# ---------------- PM-0001-48 ----------------
p48 = []
for lay, mn in [("L1", 3), ("L2", 1)]:
    for p in ["P0", "P1", "P2", "P3"]:
        for opt in ["A", "B"]:
            s = {"layer": lay, "scene": "TC", "profile": p, "params.cmd_timing": opt, "params.cap": "test"}
            if lay == "L2":
                s["machine.role"] = "LG"
            p48.append(P(f"{lay} {p} option {opt}", s, mn))
upto2 = [p["name"] for p in p48 if p["name"].split()[1] in ("P0", "P1", "P2")]
at3 = [p["name"] for p in p48 if p["name"].split()[1] == "P3"]
marks.append(M(
    "PM-0001-48", "Hits register as the shooter saw them", "pass", ["C", "R3"],
    p48, [],
    [F("up to P2: at least 99 % of view hits register (Arm rule on L2)", "view_hit_registered_share", ">=", "0.99", parts=upto2),
     F("P3: at least 95 %", "view_hit_registered_share", ">=", "0.95", parts=at3),
     F("false hits (a view miss counted as a hit) below 0.5 % of view misses", "false_hit_share", "<", "0.005"),
     C("2,000 shots per profile", "event.shot.view_time_replaced", "count", ">=", 2000, window="all", **{"from": ROOM}),
     C("setup: the test cap above every honest rewind (0 shots clipped)", "event.shot.clipped", "sum", "==", 0, window="all", **{"from": ROOM})],
    [R("share of honest shots whose view time was replaced by the server's estimate", "event.shot.view_time_replaced", "share", where={"op": "==", "value": 1}, window="all", **{"from": ROOM}),
     R("the test cap (ticks): the run's largest measured rewind, rounded up", "header.params.rewind_cap_ticks_count", "max"),
     R("aarch64-only shot mismatches (decide nothing)", figure="cross_arch_mismatch_count")],
    [B("PM-0001-48/no-interp-delay", "a rewind that leaves out the interpolation delay (at 9 m/s, 100 ms is about 0.9 m)"),
     B("PM-0001-48/no-buffer-wait", "an estimate without the buffer wait (with a buffer of 2 ticks or more, honest view times are replaced)")],
    ["R-8", "R-20"],
    "TC: a scripted shooter at the sprinting target (9 m/s, random turns) under P0 to P3, 2,000 shots per profile (proposed); L1 3 seeds and L2 (LG's shooter, R3 VS5) 1 run per profile, each under both OQ-0001-6 options (plan §6.2), with the test cap (`params.cap` = test) above every honest rewind. View hit: the ray hits the target's volume in the state the Worker published for the command's view time. Arm rule on L2: LG's shooter logs per shot the ray and the target's interpolated pose; a shot that counts against the mark counts only once the view test, replayed on x86-64 (npt-lab), agrees; an aarch64-only difference is reported. Hence figures, which join the shooter's view test with the server's registration per shot. The false-hit denominator (view misses) is my reading (question). After these runs this container picks the OQ-0001-6 option by its rule and the advisor confirms; the rule cap is PM-0001-49's.",
    arm_rule=True))

emit(marks)
