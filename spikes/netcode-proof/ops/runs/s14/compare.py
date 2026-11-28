# S14's comparison (u8-brief.md rev 1.1 section 1 Block 4; plan.md:1256 under TCP-FREEZE; the advisor's section 2 ruling
# on the burst knob), a copy of results/u3c-kernel-check/compare.py adapted (never imported from results/). It applies
# one written rule to the kernel's runs (npt-s14's output folder, copied here after R1) and the model's KCMODEL lines
# alike. The model side runs in this container after R1 (R1.md, "S14's model side").
#   python3 -I compare.py schedule        the burst schedule as NPT_KC_BURSTS reads it (equal to probe.py's)
#   python3 -I compare.py env <out>       the model run's environment: NPT_KC_RTT_NS (the measured tcpi_rtt), ...
#   python3 -I compare.py summary <out>   each kernel run's numbers and standing
#   python3 -I compare.py judge <out>     the verdict, with verdict.json written beside model.txt
# ---- Fixed in writing before any run -----------------------------------------------------------------------------
# DELAY, STALL and REPAIR PATH: as results/u3c-kernel-check/run.sh:19-36. A message's delay is its in-order arrival
#   minus its send stamp (the model: its in-order release minus its write). BASE is the run's median delay over every
#   message (nearest rank). An episode is a maximal run of consecutive messages each more than half a send gap
#   (16,666,666 ns) above BASE; its length L is its first message's delay minus BASE. RACK path if L <= RTT + gap +
#   2 ms + 1 ms (RTT: the mean client tcpi_rtt over the kernel runs), else probe-or-timeout. Stalls are counted over
#   the steady window only: messages written before 599 s (index k with k x gap < 599 s), before the first burst.
# SAMPLE (advisor-u8-stopb.md section 4, NC-SB-4): 600 s steady, 34 bursts every 10 s from 600 s, a 20 s tail (950 s,
#   28,500 messages a run), three runs: about 1,000 steady episodes pooled (17,970 steady messages a run at 2 % loss),
#   so the stall p99 is about the 11th largest; 102 recoveries, so the burst p99 is the 2nd largest.
# BURST RECOVERY (advisor-u3q.md:130-133's (e) definition): per burst, the time from the link's return to the in-order
#   delivery of the first message written at or after the return. Kernel: the return is the burst's measured end
#   (bursts-runN.json `actual`: the data direction's window, the other direction's in `actual_reverse`, NC-SB-17;
#   from the run's first message), the message the first whose send stamp is at or after
#   it. Model: the return is the scheduled end, the message index ceil((start + len) / gap), its delay from the line's
#   `delay_ns` (every message) when present, else its `late` entry, else BASE (recorded as approximate).
# A KERNEL RUN IS INVALID if it did not deliver every message; if its send lateness p99 > 1 ms; if its tcpi_rtt is
#   outside 99-120 ms (netem's 100 ms round trip not in place: U3c's C12 (f), read against netem's 100 ms); if its
#   bursts file does not hold the 34 scheduled bursts, or a burst started more than 20 ms off schedule or lasted more
#   than 20 ms more or less than 1 s, or the steady window's retransmissions were not read. VOID if it retransmitted
#   nothing, or its steady episodes outnumber the retransmissions of the steady window (U3c's noise guard, read at
#   599 s by the client, so the bursts' retransmissions cannot hide it: NC-SB-12).
# JUDGED (S14, plan.md:1256): each half compares the kernel's pooled values (3 runs) with the model's (seeds pooled):
#   p50 and p99 (nearest rank) each within 10 % of the model's (|kernel - model| <= 0.10 x model).
#   Stall half: the steady episodes' L. Burst half: the recoveries; NOT-RUN when the model's lines carry no `bursts`
#   equal to the schedule (MOVED's NPT_KC_BURSTS rider not in): R1's report then says the burst half is NOT-RUN and
#   why, and it comes to the advisor before -18's WebSocket part counts.
# VERDICT: INVALID if any run is INVALID or the model gave no line; else VOID if any run is VOID; else MISS if either
#   half misses, PASS if both pass, PARTIAL if the stall half passes and the burst half is NOT-RUN. A MISS means netem
#   wins (TCP-FREEZE): the WebSocket rows are judged on VM runs with netem in place of DelayStream's model, the model
#   is not repaired, and R1's report records the comparison and the switch (NETEM-BAR (a) applies to those rows).
import glob
import json
import math
import os
import sys

GAP = 33_333_333
HALF_GAP = GAP // 2
SLOP = 2_000_000
JITTER = 1_000_000
STEADY_END_NS = 599_000_000_000
BURSTS = [((600 + 10 * k) * 1_000_000_000, 1_000_000_000) for k in range(34)]
MESSAGES = 950 * 1_000_000_000 // GAP
WITHIN = 0.10
RTT_LO, RTT_HI = 99_000_000, 120_000_000
BURST_SLACK_NS = 20_000_000
RERUN = ("not judged; re-run the same day only if it ends 5 minutes before the bites' start (r1.sh's s14 block), "
         "else on R2's RS, a new folder, R1's kept; to the advisor (a MISS is never re-run)")


def rank(values, q):
    """Nearest-rank quantile (q in (0, 1])."""
    s = sorted(values)
    return s[max(1, math.ceil(q * len(s))) - 1]


def episodes(delay_of, n_steady, base):
    """L of the first message of each maximal run of steady messages more than half a gap above base."""
    out, prev = [], False
    for k in range(n_steady):
        d = delay_of(k)
        above = d is not None and d - base > HALF_GAP
        if above and not prev:
            out.append(d - base)
        prev = above
    return out


def n_steady():
    return math.ceil(STEADY_END_NS / GAP)


def kernel_runs(out):
    runs = []
    for spath in sorted(glob.glob(os.path.join(out, "server-run*.json"))):
        tag = spath[spath.rindex("-run") + 4:-5]
        s = json.load(open(spath))
        c = json.load(open(os.path.join(out, f"client-run{tag}.json")))
        bpath = os.path.join(out, f"bursts-run{tag}.json")
        b = json.load(open(bpath)) if os.path.exists(bpath) else None
        runs.append((f"run{tag}", s, c, b))
    return runs


def kernel_numbers(s, c, b):
    delays = s["delay_ns"]
    got = [d for d in delays if d is not None]
    base = rank(got, 0.5) if got else 0
    eps = episodes(lambda k: delays[k], min(n_steady(), len(delays)), base)
    lat = sorted(abs(x) for x in c["send_late_ns"])
    lat99 = rank(lat, 0.99) if lat else 0
    rec = []
    if b:
        first = c["first_due_ns"]
        for start, length in b["actual"]:
            ret = first + start + length
            k0 = next((k for k, t in enumerate(s["sent_ns"]) if t is not None and t >= ret), None)
            if k0 is not None and delays[k0] is not None:
                rec.append(s["sent_ns"][k0] + delays[k0] - ret)
    return base, eps, lat99, rec


def standing(s, c, b, eps, lat99):
    if s["delivered"] != s["messages"]:
        return "INVALID", f"delivered {s['delivered']} of {s['messages']}"
    if lat99 > JITTER:
        return "INVALID", f"send lateness p99 {lat99} ns > 1 ms"
    rtt = 1000 * c["tcp_info_client"]["rtt_us"]
    if not RTT_LO <= rtt <= RTT_HI:
        return "INVALID", f"tcpi_rtt {rtt} ns outside 99-120 ms: netem's 100 ms round trip not in place"
    if not b or len(b.get("actual", [])) != len(BURSTS):
        return "INVALID", f"the bursts file does not hold the {len(BURSTS)} scheduled bursts"
    for (s0, l0), (s1, l1) in zip(BURSTS, b["actual"]):
        if abs(s1 - s0) > BURST_SLACK_NS or abs(l1 - l0) > BURST_SLACK_NS:
            return "INVALID", f"a burst ran off schedule ({s1}, {l1}) against ({s0}, {l0})"
    retrans = c["tcp_info_client"]["total_retrans"]
    steady = c["tcp_info_client"].get("total_retrans_steady")
    if steady is None:
        return "INVALID", "the steady window's retransmissions were not read"
    if retrans == 0:
        return "VOID", "no retransmission (tcpi_total_retrans 0)"
    # NC-SB-12: the guard reads the steady window's own retransmissions, so the bursts' cannot hide receiver noise.
    if len(eps) > steady:
        return "VOID", f"{len(eps)} steady episodes > {steady} retransmissions in the steady window (the noise guard)"
    return None, f"{len(eps)} steady episodes <= {steady} retransmissions in the steady window"


def model_lines(path):
    out = []
    if os.path.exists(path):
        for line in open(path):
            if line.startswith("KCMODEL "):
                out.append(json.loads(line[len("KCMODEL "):]))
    return out


def model_numbers(m):
    late = {k: d for k, d in m["late"]}
    full = m.get("delay_ns")
    base = m["p50_ns"]

    def delay_of(k):
        if full is not None:
            return full[k]
        return late.get(k, base)

    eps = episodes(delay_of, min(n_steady(), m["n"]), base)
    rec, approx = [], 0
    for start, length in m.get("bursts") or []:
        ret = start + length
        k0 = -(-ret // GAP)
        if k0 >= m["n"]:
            continue
        if full is None and k0 not in late:
            approx += 1
        rec.append(k0 * GAP + delay_of(k0) - ret)
    return eps, rec, approx


def kernel_rtt(out):
    rtts = [c["tcp_info_client"]["rtt_us"] for _, _, c, _ in kernel_runs(out)]
    return round(1000 * sum(rtts) / len(rtts))


def half(kernel, model):
    """One half's comparison: p50 and p99 within 10 % of the model's."""
    if not kernel or not model:
        return "MISS", f"no values (kernel {len(kernel)}, model {len(model)})", {}
    k50, k99, m50, m99 = rank(kernel, 0.5), rank(kernel, 0.99), rank(model, 0.5), rank(model, 0.99)
    ok50, ok99 = abs(k50 - m50) <= WITHIN * m50, abs(k99 - m99) <= WITHIN * m99
    nums = {"kernel_p50_ns": k50, "kernel_p99_ns": k99, "model_p50_ns": m50, "model_p99_ns": m99,
            "kernel_n": len(kernel), "model_n": len(model)}
    text = (f"p50 kernel {k50 / 1e6:.3f} ms vs model {m50 / 1e6:.3f} ms ({'within' if ok50 else 'NOT within'} 10 %); "
            f"p99 kernel {k99 / 1e6:.3f} ms vs model {m99 / 1e6:.3f} ms ({'within' if ok99 else 'NOT within'} 10 %)")
    return ("PASS" if ok50 and ok99 else "MISS"), text, nums


def judge(out):
    rtt = kernel_rtt(out)
    bound = rtt + GAP + SLOP + JITTER
    kinds, lines, runs_seen = set(), [], []
    k_eps, k_rec = [], []
    for name, s, c, b in kernel_runs(out):
        base, eps, lat99, rec = kernel_numbers(s, c, b)
        kind, why = standing(s, c, b, eps, lat99)
        if kind:
            kinds.add(kind)
        if c["cc_client"] != "cubic" or s["cc_server"] != "cubic":
            why += f"; cubic not set (client {c['cc_client']}, server {s['cc_server']}): recorded"
        runs_seen.append({"run": name, "standing": kind or "judged", "why": why, "base_ns": base,
                          "steady_episodes": len(eps), "recoveries": len(rec),
                          "tcpi_rtt_minus_100ms_ns": 1000 * c["tcp_info_client"]["rtt_us"] - 100_000_000})
        lines.append(f"{name}: {kind or 'judged'} ({why}); tcpi_rtt - 100 ms = "
                     f"{1000 * c['tcp_info_client']['rtt_us'] - 100_000_000} ns")
        k_eps += eps
        k_rec += rec
    models = model_lines(os.path.join(out, "model.txt"))
    if not models:
        kinds.add("INVALID")
        lines.append("model.txt holds no KCMODEL line: INVALID")
    m_eps, m_rec, approx, with_bursts = [], [], 0, True
    for m in models:
        if [list(x) for x in m.get("bursts") or []] != [list(x) for x in BURSTS]:
            with_bursts = False
        eps, rec, a = model_numbers(m)
        m_eps += eps
        m_rec += rec
        approx += a
    stall, stall_text, stall_nums = half(k_eps, m_eps)
    if models and with_bursts:
        burst, burst_text, burst_nums = half(k_rec, m_rec)
    else:
        burst, burst_nums = "NOT-RUN", {}
        burst_text = ("the model's lines carry no burst schedule equal to probe.py's (MOVED's NPT_KC_BURSTS rider not "
                      "in): R1's report says the burst half is NOT-RUN and why; to the advisor before -18's WebSocket "
                      "part counts")
    k_rack = sum(1 for e in k_eps if e <= bound)
    m_rack = sum(1 for e in m_eps if e <= bound)
    if "INVALID" in kinds:
        verdict = "INVALID"
    elif "VOID" in kinds:
        verdict = "VOID"
    elif "MISS" in (stall, burst):
        verdict = "MISS"
    elif burst == "NOT-RUN":
        verdict = "PARTIAL"
    else:
        verdict = "PASS"
    follow = {
        "PASS": "the model agrees with the kernel at zero jitter: DelayStream's model stands for the WebSocket rows",
        "PARTIAL": "the stalls agree; the burst half is NOT-RUN (above)",
        "MISS": ("netem wins (TCP-FREEZE): the WebSocket rows are judged on VM runs with netem in place of "
                 "DelayStream's model; the model is not repaired; R1's report records the comparison and the switch"),
        "VOID": RERUN,
        "INVALID": RERUN,
    }[verdict]
    lines = [f"tcpi_rtt (mean over runs) {rtt} ns; RACK bound L <= {bound} ns"] + lines
    lines.append(f"stall half: {stall}: {stall_text}")
    lines.append(f"burst half: {burst}: {burst_text}")
    lines.append(f"reported, not judged: RACK-path share kernel {k_rack} of {len(k_eps)}, model {m_rack} of "
                 f"{len(m_eps)}; model recoveries approximated from BASE: {approx}")
    lines.append(f"VERDICT: {verdict} ({follow})")
    result = {"verdict": verdict, "rtt_ns": rtt, "kernel_runs": runs_seen, "stall_half": stall,
              "stall": stall_nums, "burst_half": burst, "burst": burst_nums, "model_lines": len(models),
              "model_recoveries_approximate": approx}
    json.dump(result, open(os.path.join(out, "verdict.json"), "w"), indent=1)
    print("\n".join(lines))


def main():
    cmd = sys.argv[1] if len(sys.argv) > 1 else ""
    if cmd == "schedule" and len(sys.argv) == 2:
        print(",".join(f"{s}:{l}" for s, l in BURSTS))
    elif cmd == "env" and len(sys.argv) == 3:
        print(f"NPT_KC_RTT_NS={kernel_rtt(sys.argv[2])} NPT_KC_MESSAGES={MESSAGES} NPT_KC_RUNS=100 "
              f"NPT_KC_BURSTS={','.join(f'{s}:{l}' for s, l in BURSTS)}")
    elif cmd == "summary" and len(sys.argv) == 3:
        for name, s, c, b in kernel_runs(sys.argv[2]):
            base, eps, lat99, rec = kernel_numbers(s, c, b)
            kind, why = standing(s, c, b, eps, lat99)
            print(f"{name}: delivered {s['delivered']} of {s['messages']}; base {base} ns; {len(eps)} steady episodes; "
                  f"{len(rec)} recoveries; send lateness p99 {lat99} ns; tcp_info {c['tcp_info_client']}; "
                  f"{kind or 'judged'}: {why}")
    elif cmd == "judge" and len(sys.argv) == 3:
        judge(sys.argv[2])
    else:
        print("usage: compare.py schedule | env <out> | summary <out> | judge <out>", file=sys.stderr)
        sys.exit(2)


main()
