import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

marks = []
BR = {"process": "browser"}
RSM = {"machine.role": "RS"}
ML3 = ["chrome", "edge", "firefox"]

# ---------------- PM-0006-11 ----------------
marks.append(M(
    "PM-0006-11", "The reference server exists and is the machine every server row ran on", "pass", ALL_VMS,
    [P("every RS session", {"machine.name": "RS"}, 1)],
    [],
    [F("RS named: type, location, the day's address, CPU model, vCPU count, memory, kernel, threads per core and the SMT sibling map", "machine_named", "==", 1),
     C("the CPU model read at the session's start", "event.cpu_read.changed", "count", ">=", 1, window="all", **{"from": RSM}),
     C("where a core carries 2 threads, the transport runtime kept off the tick thread's sibling (pinned rows)", "event.affinity.sibling_free", "min", "==", 1, window="all", **{"from": ROOM})],
    [R("CPU model changes within the run's sessions (named in the results)", "event.cpu_read.changed", "sum", window="all", **{"from": RSM})],
    [B("PM-0006-11/no-smt-map", "a header without the SMT map")],
    ["R-12", "R-21"],
    "Every run: the CCX23 created in fsn1 on the run day by Mustafa, in a separate Hetzner project, deleted after it; a script reads /proc/cpuinfo and the plan at each session start and writes `cpu_read` (`changed` 1 when the model differs from the run's first read). The address is the VM's day address (R-21: recorded in the results). The density runs (-27, PM-0006-13, -14) are unpinned and list this mark only for the header (their room logs then carry no `affinity` events; the sibling check needs a part that excludes them, or the density runs list -11 from RS's log only: question)."))

# ---------------- PM-0006-12 ----------------
marks.append(M(
    "PM-0006-12", "The round trip to the test servers' location", "person", ["R1", "MP", "ML*"],
    [P("MP: 100 echoes to LG's CAX21 (operations checklist step 1)", {"machine.role": "MP", "params.suite": "ping"}, 1),
     P("ML: 100 echoes", {"machine.role": "ML", "params.suite": "ping"}, 1)],
    [], [],
    [R("mean round trip", "event.ping.mean_ns", "max", window="all"),
     R("max round trip", "event.ping.max_ns", "max", window="all"),
     R("loss", "event.ping.lost_count", "max", window="all")],
    [], [],
    "Reported, no pass number; decides nothing, and a surprise goes to Mustafa. `ping -n 100 <address>` from ML and from MP to one of the run's CAX21s in fsn1 (LG's, created on the run day, ICMP allowed on it only, never on RS), 1 run per machine. Each session pastes Windows' `ping` output into the results through the record form, which writes one `ping` event (`mean_ns`, `max_ns`, `lost_count` out of 100) and, from Mustafa, a `person` event (who = Mustafa, mark = PM-0006-12, verdict = pass once he has read it beside VISION_V2 §4's 'about 100 ms', fail for a surprise, steps = the pasted output). The ML leg is a reading, so any ML (the stand-in included) serves; `machine.role` in a select is question Q1."))

# ---------------- PM-0006-13 ----------------
d13 = [P(f"{d}: 8 rooms", {"params.density": "8-rooms", "params.session": d}, 8) for d in ["D1", "D2", "D3"]]
marks.append(M(
    "PM-0006-13", "Rooms per VM: 8 rooms at N = 48 on RS", "pass", ["R5"],
    d13, ["PM-0006-13"],
    [C("each room's tick p99 at most 1.10 x the single-room figure (bracket B)", "tick.v.work_ns", "p99", "<=", factor="1.10",
       against={"roles": ["bracket-first", "bracket-last"], "combine": "mean"}, **{"from": ROOM}),
     F("the 3 sessions at least 6 hours apart, one between 18:00 and 23:00 Amsterdam (from the results' UTC times)", "times_of_day_rule", "==", 1)],
    [R("each room process's CPU share of one vCPU, mean", "second.v.cpu_permille", "mean", **{"from": ROOM}),
     R("each room process's CPU share of one vCPU, max", "second.v.cpu_permille", "max", **{"from": ROOM}),
     R("steal per vCPU per second, mean", "second.v.steal_permille", "mean", **{"from": RSM}),
     R("steal per vCPU per second, max", "second.v.steal_permille", "max", **{"from": RSM})],
    [B("PM-0006-13/busy-20", "a busy-wait of 20 % of the single-room p99 planted in every tick of one room must fail that room's row")],
    ["R-19", "R-26"],
    "R5, D1 to D3: 8 room processes under the supervisor, no CPU pinning (recorded), 384 bot clients on LG (as many CAX21s as the calibration shows), bracket B (one room alone 20 minutes first and last; the single-room figure is the mean of their p99s; the session valid only if they lie within 5 %). Each room is its own run, the session its set. Validity is bracket B's and LG's 80 % rule, not V-S; steal is reported, never a condition. PM-0001-27 is this set of runs. The sessions of one N may share one VM set kept up through a day, rooms stopped between them (R-26); the results say which shared a host. The times-of-day rule needs a UTC start time in the header (question)."))

# ---------------- PM-0006-14 ----------------
p14 = [P(f"N={n}", {"params.density": "count", "clients": n}, 1) for n in [48, 36, 18]]
marks.append(M(
    "PM-0006-14", "Rooms per VM counted, the figure that replaces A-2", "reported", ["R5", "R6"],
    p14, ["PM-0006-13", "padding"], [],
    [R("rooms per VM k: the lowest count of the 3 sessions ('a lower bound, provisional until slice 6')", figure="rooms_per_vm_count"),
     R("each room's padded step time p99", "tick.v.padded_step_ns", "p99", **{"from": ROOM}),
     R("the 3-sessions rule (>= 6 h apart, one 18:00-23:00 Amsterdam)", figure="times_of_day_rule")],
    [],
    ["R-19", "R-26"],
    "Reported, no pass number and no bite; npt-judge's fixtures prove that short padding comes out INVALID (the advisor's reading, 2026-11-06). On RS at N = 48, 36 and 18: per N, one session with bracket B: k = 4, then 6, 8 and up by 2 until a k fails, then the odd k between the last pass and the first fail; if k = 4 fails, 3, 2, 1; then the count and count + 1 at the other 2 times of day. 'Meets R2-ROOM-REC2 marks 1 and 2' at 30 Hz: tick p99 at most 16.67 ms, max under 33.33 ms, 0 overruns, and p99 within 10 % of the single-room figure. The stand-in's sim step is padded by a busy-wait to 8.33 ms per tick (`params.padding_target_ns` = 8,330,000), or replays the sim core spike's step times (DENSITY-FIGURE: Mustafa, 2026-11-06, option (a)). A run plan with every session's VM-hours comes first (it feeds PM-0006-18). Each k step is a set of k room runs with `params.k`; the figure reads the search from the parts' runs. PM-0001-63 is this rule at 60 Hz (outside)."))

# ---------------- PM-0006-15 ----------------
p15, j15 = [], []
for mach, sel, brs in [("ML", {"machine.name": "ML"}, ML3), ("MAC", {"machine.role": "MAC"}, ["safari"])]:
    for b in brs:
        for tr in ["wt", "ws"]:
            s = dict(sel); s.update({"params.suite": "firewall", "params.browser": b, "params.transport": tr})
            n = f"{mach} {b} {tr}"
            p15.append(P(n, s, 1)); j15.append(n)
p15.append(P("closed-port probe from ML", {"params.suite": "fw-probe", "machine.name": "ML"}, 1))
p15.append(P("rule lists (pasted from the console)", {"params.suite": "rule-list"}, 1))
p15.append(P("ML* (rough, reported)", {"params.suite": "firewall", "machine.name": "ML*"}, 1))
marks.append(M(
    "PM-0006-15", "The firewall lets both game transports in and nothing else", "pass", ["R4", "ML*", "MAC-r"],
    p15, [],
    [C("20 attempts per browser and transport", "event.attempt.ok", "count", ">=", 20, window="all", **{"from": BR, "parts": j15}),
     C("20 of 20 reach a room on RS (session ready and one echo)", "event.attempt.ok", "min", "==", 1, window="all", **{"from": BR, "parts": j15}),
     C("the wss certificate accepted with no warning", "event.wss_cert.accepted", "min", "==", 1, window="all", **{"from": BR, "parts": [n for n in j15 if n.endswith("ws")]}),
     C("a TCP connect to the closed port times out (never 'refused')", "event.fw_probe.tcp_timed_out", "min", "==", 1, window="all", parts=["closed-port probe from ML"]),
     C("a UDP probe to the closed port gets no reply", "event.fw_probe.udp_no_reply", "min", "==", 1, window="all", parts=["closed-port probe from ML"]),
     C("a rule list read for every VM role", "event.rule_list.diff_count", "count", ">=", 2, window="all", parts=["rule lists (pasted from the console)"]),
     C("each role's rule list equals part 5's list, every source limited as listed", "event.rule_list.diff_count", "sum", "==", 0, window="all", parts=["rule lists (pasted from the console)"])],
    [R("ML*: attempts that worked", "event.attempt.ok", "sum", window="all", **{"from": BR, "parts": ["ML* (rough, reported)"]})],
    [B("PM-0006-15/probed-port-rule", "a rule added for the probed TCP port makes the probe connect (the negative test can fail)"),
     B("PM-0006-15/ssh-open", "an SSH rule opened to every source fails the list comparison")],
    ["R-1", "R-18"],
    "R4: WebdriverIO opens the client page with the transport forced by a URL flag; success = session ready + one datagram or message echoed; 20 attempts per browser and transport (ML: Chrome, Edge, Firefox; MAC: Safari, the runner allowed for this count row). The probe: one TCP connection and one UDP datagram to one closed port of each kind, with a test listener on each. Mustafa reads each role's rule list from Hetzner's console (no token) and pastes it; a compare script writes one `rule_list` event per role with the count of rules differing from part 5's list (ops/firewall/, R-1's ports). Which certificate the wss endpoint presented is a string (question Q16). The ML parts wait for the named laptop."))

# ---------------- PM-0006-16 ----------------
marks.append(M(
    "PM-0006-16", "The metrics path works with only the rule it needs", "pass", ["R5", "ML*"],
    [P("the scrape path (R5 D1)", {"params.suite": "soak-8"}, 1),
     P("probe from ML", {"params.suite": "metrics-probe", "machine.name": "ML"}, 1),
     P("probe from ML* (rough, reported)", {"params.suite": "metrics-probe", "machine.name": "ML*"}, 1)],
    [],
    [C("every metric of PM-0006-22 reaches Prometheus on MV", "event.metric.present", "min", "==", 1, window="all", parts=["the scrape path (R5 D1)"]),
     C("the exporter's port drops every source but MV: ML's probe gets no answer", "event.metrics_probe.answered", "max", "==", 0, window="all", parts=["probe from ML"]),
     C("the probe ran", "event.metrics_probe.answered", "count", ">=", 1, window="all", parts=["probe from ML"])],
    [R("ML*: probe answered", "event.metrics_probe.answered", "max", window="all", parts=["probe from ML* (rough, reported)"])],
    [B("PM-0006-16/rule-open", "the metrics rule opened to every source makes ML's probe connect")],
    ["R-3"],
    "Part 9 picks the scrape (R-3): Prometheus on MV (LG's CAX21) scrapes RS's exporter, bound to RS's public IPv4; the rule names that CAX21's address of the day as its only source, set on the run day. The push half (no inbound rule) does not apply. The metric presence is PM-0006-22's query, read here too. The judged probe waits for the named laptop (plan §6.3); ML* is rough."))

# ---------------- PM-0006-17 ----------------
marks.append(M(
    "PM-0006-17", "Egress per 24v24 match-hour matches R2's estimate", "pass", ["R2"],
    [P("one room-hour, 48 bots all in view", {"params.suite": "egress", "clients": 48, "params.interest": "off"}, 1),
     P("interest on (reported)", {"params.suite": "egress", "clients": 48, "params.interest": "on"}, 1)],
    [],
    [F("interface bytes sent in one room-hour at least 0.8 x R2's E3 formula at the run's counts", "egress_vs_r2_ratio", ">=", "0.8", parts=["one room-hour, 48 bots all in view"]),
     F("at most 1.2 x", "egress_vs_r2_ratio", "<=", "1.2", parts=["one room-hour, 48 bots all in view"]),
     F("the room's bytes-sent metric within 5 % of the interface counter, low", "egress_metric_vs_interface_ratio", ">=", "0.95", parts=["one room-hour, 48 bots all in view"]),
     F("high", "egress_metric_vs_interface_ratio", "<=", "1.05", parts=["one room-hour, 48 bots all in view"])],
    [R("R2's formula at the run's counts (the arithmetic shown)", figure="r2_egress_formula_bytes"),
     R("interface bytes sent in the room-hour", figure="egress_interface_bytes"),
     R("interest on: interface bytes sent", figure="egress_interface_bytes", parts=["interest on (reported)"])],
    [B("PM-0006-17/double-send", "snapshots sent twice per tick (about 2 x the reference)")],
    [],
    "RS with LG, one room alone: 48 bot clients over IPv4, all in view, three 20-minute matches back to back, joins and full states included; P2 on LG; 1 room-hour per setting. The interface's transmitted-byte counter is read at the start and end (`event.iface.tx_bytes`); the room's metric at the same moments (`event.room_metric.sent_bytes`). R2's formula (r2 lines 1764-1766) with the run's P, V, shots and projectile spawns; at R2's own counts it is 3.156 GB (band 2.52 to 3.79 GB). The interest-on figure is reported, since the sim has the interest filter (0006:1372-1373)."))

# ---------------- PM-0006-18 ----------------
marks.append(M(
    "PM-0006-18", "The spike's own hosting bill", "person", ["MP"],
    [P("Mustafa reads the project's costs (operations checklist step 2)", {"machine.role": "MP", "params.suite": "bill"}, 1)],
    [], [], [], [], [],
    "Judged by Mustafa after the last session: the spike's Hetzner project's costs, excluding VAT, for every VM-hour and traffic block the proof test created (LG's calibrated CAX21s counted), against each run's cap (OQ-0006-20: one cap per run, from its planned hours). His session writes one `person` event (who = Mustafa, mark = PM-0006-18, verdict = pass if every run's cost is under its cap, steps = each run's cost beside its cap, as he read them). The planning check before any VM (each run plan's hours x 0.1386 EUR for a CCX23 and 0.0176 EUR for a CAX21, the arithmetic shown) lives in the run plans, not in a results log. Money has no unit suffix in README §1, so no cost is a series here (question)."))

# ---------------- PM-0006-19 ----------------
marks.append(M(
    "PM-0006-19", "Logging costs the tick almost nothing", "pass", ["R2"],
    [P("logging on (3 runs)", {"params.suite": "logging-cost"}, 3)],
    [],
    [F("pooled p99 of the 3 'on' runs at most 1.02 x the pooled p99 of the 3 'off' runs (54,000 ticks each)", "logging_cost_ratio", "<=", "1.02", parts=["logging on (3 runs)"])],
    [R("pooled p99, logging on", "tick.v.work_ns", "p99", pooled=True, **{"from": ROOM})],
    [B("PM-0006-19/busy-50us", "a planted 50 us busy-wait in each logged event")],
    [],
    "RS with LG; one room alone, one vCPU; 6 runs of 10 measured minutes, each a fresh process, alternating off, on, off, on, off, on; one recorded input replay drives all six. The 'off' runs have role `logging-off` in the same set. The pooled comparison is a figure: `against` combines per-run statistics, not pooled ones (question). The row's validity, 'each off run's p99 within 1 % of the pooled off p99', is no rule of README §4 (closest: `clean-ref`, 2.5 % of the median): a rule `logging-off-1pct` is needed (question); an invalid set is neither pass nor fail."))

emit(marks)
