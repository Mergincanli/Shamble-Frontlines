import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

marks = []
BR = {"process": "browser"}
ML3 = ["chrome", "edge", "firefox"]
SOAK = [P("the 8-room soak (R5 D1)", {"params.suite": "soak-8"}, 1)]

def per_browser(suite, extra_name=""):
    ps = []
    for mach, sel, brs in [("ML", {"machine.name": "ML"}, ML3), ("MAC", {"machine.role": "MAC"}, ["safari"])]:
        for b in brs:
            s = dict(sel); s.update({"params.suite": suite, "params.browser": b})
            ps.append(P(f"{mach} {b}{extra_name}", s, 1))
    return ps

# ---------------- PM-0006-20 ----------------
marks.append(M(
    "PM-0006-20", "Logging allocates nothing on the hot path", "pass", ["R2"],
    [P("logging on", {"params.suite": "logging-alloc"}, 1)],
    [],
    [F("on every tick after the warm-up, the tick thread's allocation count with logging on equals the count with it off (tick by tick)", "alloc_twin_diff_count", "==", 0),
     C("the counting allocator ran on every measured tick (10 minutes)", "tick.v.tick_allocs_count", "count", ">=", 18_000, **{"from": ROOM})],
    [R("tick-thread allocations per tick, max", "tick.v.tick_allocs_count", "max", **{"from": ROOM})],
    [B("PM-0006-20/planted-format", "a `format!` planted in a per-tick log call")],
    [],
    "RS, one room: a counting global allocator (Rust's GlobalAlloc) records allocations per tick on the tick thread; 2 runs (on, off) of 10 measured minutes, both driven by the same recorded input replay, so the sim's own allocations repeat tick for tick. The 'off' run has role `logging-off` and is named in the 'on' run's `params.twin`; the figure compares the two runs' `tick_allocs_count` at equal tick numbers."))

# ---------------- PM-0006-21 ----------------
marks.append(M(
    "PM-0006-21", "Logs keep no secrets or addresses", "pass", ["R5"],
    SOAK, [],
    [C("the grep ran over every shipped log line", "event.canary_grep.lines_count", "min", ">=", 1, window="all"),
     C("the canaries were there to find (tokens, secrets, addresses on LG; seeds on RS)", "event.canary_grep.canaries_count", "min", ">=", 1, window="all"),
     C("0 matches for any join token", "event.canary_grep.token_matches_count", "sum", "==", 0, window="all"),
     C("0 matches for any guest secret", "event.canary_grep.secret_matches_count", "sum", "==", 0, window="all"),
     C("0 matches for any client address the soak used", "event.canary_grep.address_matches_count", "sum", "==", 0, window="all"),
     C("0 matches for any match's random seed", "event.canary_grep.seed_matches_count", "sum", "==", 0, window="all"),
     C("0 matches for an IPv4 or IPv6 pattern other than the VMs' own", "event.canary_grep.ip_pattern_matches_count", "sum", "==", 0, window="all")],
    [],
    [B("PM-0006-21/planted-line", "a planted log line that prints one client's address and one token must be found")],
    ["R-21"],
    "The 30-minute soak of PM-0006-22. The spike logs each token, secret and client address it creates into a canary file on LG (never on RS); the supervisor's test build writes each match's seed into a canary file on RS's local disk outside the shipped log path; a grep script searches every log line the supervisor shipped (the tracing logs) and writes one `canary_grep` event. The results log is not a shipped log (plan §6.1, security F18), so its seed and its VM addresses (R-21) are outside the grep."))

# ---------------- PM-0006-22 ----------------
marks.append(M(
    "PM-0006-22", "Every first metric exists and moves", "pass", ["R5"],
    SOAK, [],
    [C("every metric part 9 lists was queried", "event.metric.present", "count", ">=", 1, window="all"),
     C("every metric present on MV", "event.metric.present", "min", "==", 1, window="all"),
     C("every counter the soak's script triggers has moved", "event.metric.stuck_triggered", "sum", "==", 0, window="all"),
     C("the triggers ran: one join refused per room (a bad token)", "event.soak_trigger.bad_token_refusals_count", "min", ">=", 8, window="all"),
     C("the triggers ran: one room process killed at minute 20", "event.soak_trigger.rooms_killed_count", "min", ">=", 1, window="all")],
    [R("bots on WebSocket (a quarter)", "header.params.ws_bots_count", "max")],
    [B("PM-0006-22/unregistered", "one metric left unregistered fails the presence check")],
    [],
    "After a 30-minute soak of 8 rooms at N = 48 under the supervisor (the load as PM-0006-13, P2 on LG): Prometheus queries per metric (MV's role on npt-load) write one `metric` event each (`present`; `stuck_triggered` 1 when the soak triggered it and it did not move). Counters with no trigger (per-client budget overruns, relevance churn, datagrams dropped) are checked for presence only. The kill at minute 20 cuts that room's log (`params.killed` true). The metric count part 9 lists is not in the row, so 'every metric' is the query list's own length (question)."))

# ---------------- PM-0006-23 ----------------
marks.append(M(
    "PM-0006-23", "A stall is visible at once", "pass", ["R5"],
    SOAK, [],
    [C("control: before the first stall the soak shows 0 overruns", "event.stall_scrape.overruns_before_count", "max", "==", 0, window="all"),
     C("3 stalls, 5 minutes apart, each shown", "event.stall_scrape.overrun_delta_count", "count", ">=", 3, window="all"),
     C("the first scrape after each stall: overrun counter up by exactly 1, low", "event.stall_scrape.overrun_delta_count", "min", ">=", 1, window="all"),
     C("the first scrape after each stall: overrun counter up by exactly 1, high", "event.stall_scrape.overrun_delta_count", "max", "<=", 1, window="all"),
     C("the dropped-time counter up by the stall minus the catch-up bound, within one tick period (33.33 ms)", "event.stall_scrape.dropped_err_ns", "max", "<=", 33_330_000, window="all")],
    [R("Prometheus' scrape interval", "header.params.scrape_interval_ns", "max")],
    [],
    [],
    "A deliberate 1 s stall inside one tick (longer than ADR 0004's catch-up bound, so time is dropped), injected by a test command at minute 25 of PM-0006-22's soak, 3 stalls 5 minutes apart; the scrape interval set to 15 s and recorded. MV writes one `stall_scrape` event per stall from the first scrape after it: the overrun counter's increase, and `dropped_err_ns` = |the dropped-time counter's increase - (stall - 7 tick periods)|. The row's bite is the stall itself ('the stall is the planted event'), so `bites` is empty; its control is the first check."))

# ---------------- PM-0006-24 ----------------
marks.append(M(
    "PM-0006-24", "The metrics stay small and hold no player or match id", "pass", ["R5"],
    SOAK, [],
    [C("the series count for 8 rooms on one VM under 5,000", "event.series_count.total_count", "max", "<", 5000, window="all"),
     C("the count was taken", "event.series_count.total_count", "count", ">=", 1, window="all"),
     C("every label name within {VM, room slot, mode id, transport} plus Prometheus' own target labels", "event.series_count.foreign_labels_count", "sum", "==", 0, window="all")],
    [R("series count", "event.series_count.total_count", "max", window="all")],
    [B("PM-0006-24/player-label", "a planted `player` label on one metric must fail the label check (and raise the count by the number of players)")],
    [],
    "At the end of PM-0006-22's soak: a count of every series from RS's exporter and the set of label names, by Prometheus query; one `series_count` event."))

# ---------------- PM-0006-25 ----------------
p25 = per_browser("cert-14d") + [P("ML* (rough, reported)", {"params.suite": "cert-14d", "machine.name": "ML*"}, 1)]
j25 = [p["name"] for p in p25 if "rough" not in p["name"]]
marks.append(M(
    "PM-0006-25", "The development certificate works in every browser", "pass", ["R4", "ML*", "MAC-r", "MAC-b"],
    p25, [],
    [C("20 attempts per browser", "event.attempt.ok", "count", ">=", 20, window="all", **{"from": BR, "parts": j25}),
     C("100 % connect with the pinned self-signed certificate", "event.attempt.ok", "min", "==", 1, window="all", **{"from": BR, "parts": j25}),
     C("setup: the certificate valid at most 14 days", "header.params.cert_validity_ns", "max", "<=", 1_209_600_000_000_000, parts=j25)],
    [R("ML*: attempts that worked", "event.attempt.ok", "sum", window="all", **{"from": BR, "parts": ["ML* (rough, reported)"]})],
    [],
    ["R-24"],
    "RS; ML: Chrome, Edge, Firefox; MAC: Safari (MAC-r if the advisor extends the runner list, R-13, else MAC-b). WebdriverIO opens the client page, which fetches the hash and opens a WebTransport session with `serverCertificateHashes`; success as PM-0006-15. The certificate (ECDSA P-256, made by the server at start) is the key type in the header, a string (question Q16). This row is PM-0006-27's control and names no bite. A browser that cannot pin is R-24's case. The ML parts wait for the named laptop."))

# ---------------- PM-0006-26 ----------------
marks.append(M(
    "PM-0006-26", "The certificate rotates without dropping anyone", "pass", ["R4", "ML*", "MAC-r", "MAC-b"],
    [P("3 rotations in one 30-minute session", {"params.suite": "rotation", "machine.name": "ML"}, 1),
     P("ML* (rough, reported)", {"params.suite": "rotation", "machine.name": "ML*"}, 1)],
    [],
    [C("3 rotations", "event.rotation.dropped_count", "count", ">=", 3, window="all", parts=["3 rotations in one 30-minute session"]),
     C("0 of the 48 live sessions dropped across each rotation", "event.rotation.dropped_count", "sum", "==", 0, window="all", parts=["3 rotations in one 30-minute session"]),
     C("after each rotation, one reload in each of the four browsers (12 reloads)", "event.reload.ok", "count", ">=", 12, window="all", **{"from": BR, "parts": ["3 rotations in one 30-minute session"]}),
     C("each reload connects with the new hash", "event.reload.ok", "min", "==", 1, window="all", **{"from": BR, "parts": ["3 rotations in one 30-minute session"]})],
    [R("ML*: sessions dropped", "event.rotation.dropped_count", "sum", window="all", parts=["ML* (rough, reported)"])],
    [B("PM-0006-26/closes-live", "a rotation that closes live sessions")],
    ["R-12", "R-24"],
    "48 live sessions: 44 bots on LG that pin the hash as browsers do, plus one session from each browser (ML: Chrome, Edge, Firefox; MAC: Safari). The server switches to a newly made certificate and publishes its hash; live sessions send inputs and receive snapshots for at least 5 minutes after each (proposed); 3 rotations in one 30-minute session (proposed). The room writes one `rotation` event per rotation (sessions dropped across it). The part selects the run by its named-laptop log (question Q2); it waits for the named laptop."))

# ---------------- PM-0006-27 ----------------
p27 = per_browser("cert-15d") + [P("ML* (rough, reported)", {"params.suite": "cert-15d", "machine.name": "ML*"}, 1),
                                 P("control: PM-0006-25's 14-day certificate", {"params.suite": "cert-14d"}, 1)]
j27 = [p["name"] for p in p27 if "rough" not in p["name"] and "control" not in p["name"]]
marks.append(M(
    "PM-0006-27", "A certificate that breaks the browsers' rule is refused", "pass", ["R4", "ML*", "MAC-r", "MAC-b"],
    p27, [],
    [C("5 attempts per browser", "event.attempt.ok", "count", ">=", 5, window="all", **{"from": BR, "parts": j27}),
     C("a 15-day certificate rejected: 5 of 5", "event.attempt.ok", "sum", "==", 0, window="all", **{"from": BR, "parts": j27}),
     C("setup: the certificate valid for 15 days", "header.params.cert_validity_ns", "min", ">=", 1_296_000_000_000_000, parts=j27),
     C("control: the 14-day certificate connects", "event.attempt.ok", "min", "==", 1, window="all", **{"from": BR, "parts": ["control: PM-0006-25's 14-day certificate"]})],
    [R("ML*: attempts rejected", "event.attempt.ok", "count", window="all", **{"from": BR, "parts": ["ML* (rough, reported)"]})],
    [],
    [],
    "RS started with a 15-day self-signed certificate, its hash published; WebdriverIO attempts as PM-0006-25, 5 per browser (proposed). The row names a control, not a bite: PM-0006-25's runs (which then list -27 too) are its control part. 15 days = 1,296,000 s; 14 days = 1,209,600 s. The ML parts wait for the named laptop."))

emit(marks)
