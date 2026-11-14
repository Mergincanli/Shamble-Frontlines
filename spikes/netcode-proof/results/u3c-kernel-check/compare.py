# U3c's kernel cross-check: applies run.sh's written rule (its header) to the kernel runs and the model's runs alike.
#   python3 -I compare.py summary <kernel-runN.json>   one kernel run's numbers and its standing (VOID, INVALID)
#   python3 -I compare.py rtt <out-dir>                the model's RTT: the mean client tcpi_rtt over the runs, in ns
#   python3 -I compare.py judge <out-dir>              the verdict (verdict.json beside it)
#   python3 -I compare.py may-run <folder>             exits 0 if the rerun rule lets a new run start, else 4
import glob
import json
import math
import os
import sys

GAP = 33_333_333
HALF_GAP = GAP // 2
SLOP = 2_000_000
JITTER = 1_000_000
P50_WITHIN = 2_000_000
LEVEL = 0.99


def p50(values):
    s = sorted(values)
    return s[(len(s) + 1) // 2 - 1]


def episodes(n, base, late, threshold):
    """Lengths (delay - base) of the first message of each maximal run of messages more than `threshold` above base.
    `late` maps a message index to its delay, for every message more than 1 ms above base (the rest are below any
    threshold used here)."""
    out = []
    prev = False
    for k in range(n):
        d = late.get(k)
        above = d is not None and d - base > threshold
        if above and not prev:
            out.append(d - base)
        prev = above
    return out


def classify(eps, rtt):
    bound = rtt + GAP + SLOP + JITTER
    rack = [e for e in eps if e <= bound]
    return rack, len(eps) - len(rack)


def kernel_run(path):
    r = json.load(open(path))
    delays = r["delay_ns"]
    got = [d for d in delays if d is not None]
    base = p50(got)
    late = {k: d for k, d in enumerate(delays) if d is not None and d > base + 1_000_000}
    lat = sorted(abs(x) for x in r["send_late_ns"])
    return r, base, late, lat[min(len(lat) - 1, math.ceil(0.99 * len(lat)) - 1)]


def deltas(r):
    """The system-wide retransmission counters' growth over the run (recorded, not judged)."""
    before, after = r.get("kernel_counters_before", {}), r.get("kernel_counters_after", {})
    return {k: (after[k] - before[k]) if after.get(k) is not None and before.get(k) is not None else None for k in after}


def standing(r, eps, lat99):
    """A kernel run's standing before any judgement (run.sh's header): INVALID, VOID or None (judged), with why."""
    if r["delivered"] != r["messages"]:
        return "INVALID", f"delivered {r['delivered']} of {r['messages']}"
    if lat99 > JITTER:
        return "INVALID", f"send lateness p99 {lat99} ns > the 1 ms allowance"
    retrans = r["tcp_info_client"]["total_retrans"]
    if retrans == 0:
        return "VOID", "the kernel retransmitted nothing (tcpi_total_retrans 0): no loss to compare"
    if len(eps) > retrans:
        return "VOID", f"{len(eps)} judged episodes > {retrans} retransmissions (the noise guard)"
    return None, f"{len(eps)} judged episodes <= {retrans} retransmissions"


def model_runs(path):
    runs = []
    for line in open(path):
        if line.startswith("KCMODEL "):
            m = json.loads(line[len("KCMODEL "):])
            m["late_map"] = {k: d for k, d in m["late"]}
            runs.append(m)
    return runs


def band(n, p, level):
    tail = (1 - level) / 2
    pmf = [math.comb(n, k) * p**k * (1 - p) ** (n - k) for k in range(n + 1)]
    lo, below = 0, 0.0
    for k, x in enumerate(pmf):
        if below + x > tail:
            lo = k
            break
        below += x
    hi, above = n, 0.0
    for k in range(n, -1, -1):
        if above + pmf[k] > tail:
            hi = k
            break
        above += pmf[k]
    return lo, hi


def kernel_rtt(out):
    rtts = [json.load(open(p))["tcp_info_client"]["rtt_us"] for p in sorted(glob.glob(os.path.join(out, "kernel-run*.json")))]
    return round(1000 * sum(rtts) / len(rtts))


def may_run(folder):
    """The rerun rule (run.sh's RERUNS): None if a new run may start, else why not."""
    void = 0
    for d in sorted(glob.glob(os.path.join(folder, "out-*"))):
        if os.path.exists(os.path.join(d, "ABORTED")):
            continue
        v = os.path.join(d, "verdict.json")
        if not os.path.exists(v):
            return f"{d} has no verdict: take it to the advisor, or write why in {d}/ABORTED to set it aside"
        kind = json.load(open(v)).get("verdict")
        if kind == "PASS":
            return f"{d}: PASS, final"
        if kind == "MISS":
            return f"{d}: MISS, never re-run: to the advisor before U3c closes"
        void += 1
        if void >= 2:
            return f"{d}: a second VOID or INVALID verdict: to the advisor"
    if void == 1:
        print("compare.py: the one rerun after a VOID or INVALID verdict: run it on a quiet container")
    return None


def main():
    cmd = sys.argv[1]
    if cmd == "may-run":
        why = may_run(sys.argv[2])
        if why:
            print(f"compare.py: refusing a new run: {why}", file=sys.stderr)
            sys.exit(4)
        return
    if cmd == "summary":
        r, base, late, lat99 = kernel_run(sys.argv[2])
        eps = episodes(r["messages"], base, late, HALF_GAP)
        kind, why = standing(r, eps, lat99)
        print(
            f"{os.path.basename(sys.argv[2])}: delivered {r['delivered']} of {r['messages']}; base {base} ns; "
            f"{len(eps)} stall episodes; send lateness p99 {lat99} ns; cc {r['cc_client']}/{r['cc_server']} "
            f"{r['cc_errors']}; tcp_info {r['tcp_info_client']}; kernel counters' growth {deltas(r)}; "
            f"{kind or 'judged'}: {why}"
        )
        return
    out = sys.argv[2]
    if cmd == "rtt":
        print(kernel_rtt(out))
        return
    assert cmd == "judge", cmd
    rtt = kernel_rtt(out)
    problems, kinds, runs_seen = [], set(), []
    k_eps, k_eps_gap = [], 0
    for path in sorted(glob.glob(os.path.join(out, "kernel-run*.json"))):
        r, base, late, lat99 = kernel_run(path)
        name = os.path.basename(path)
        eps = episodes(r["messages"], base, late, HALF_GAP)
        kind, why = standing(r, eps, lat99)
        runs_seen.append(
            {
                "run": name,
                "standing": kind or "judged",
                "why": why,
                "episodes": len(eps),
                "episodes_one_gap": len(episodes(r["messages"], base, late, GAP)),
                "tcpi_total_retrans": r["tcp_info_client"]["total_retrans"],
                "kernel_counters_growth": deltas(r),
            }
        )
        if kind:
            kinds.add(kind)
            problems.append(f"{name}: {kind}: {why}")
        if r["cc_client"] != "cubic" or r["cc_server"] != "cubic":
            problems.append(f"{name}: cubic not set (client {r['cc_client']}, server {r['cc_server']}): recorded")
        k_eps += eps
        k_eps_gap += len(episodes(r["messages"], base, late, GAP))
    m_eps, m_eps_gap, m_rep = [], 0, [0, 0, 0]
    runs = model_runs(os.path.join(out, "model.txt"))
    if not runs:
        kinds.add("INVALID")
        problems.append("model.txt holds no KCMODEL line: INVALID")
    for m in runs:
        m_eps += episodes(m["n"], m["p50_ns"], m["late_map"], HALF_GAP)
        m_eps_gap += len(episodes(m["n"], m["p50_ns"], m["late_map"], GAP))
        m_rep = [a + b for a, b in zip(m_rep, m["repairs"])]
    k_rack, k_other = classify(k_eps, rtt)
    m_rack, m_other = classify(m_eps, rtt)
    verdict = {
        "rtt_ns": rtt,
        "kernel_runs": runs_seen,
        "kernel_episodes": len(k_eps),
        "model_episodes": len(m_eps),
        "problems": problems,
    }
    lines = [f"loopback RTT (mean client tcpi_rtt) {rtt} ns; RACK bound L <= {rtt + GAP + SLOP + JITTER} ns"]
    lines += [f"{x['run']}: {x['standing']} ({x['why']}); kernel counters' growth {x['kernel_counters_growth']}" for x in runs_seen]
    ok = True
    if k_rack and m_rack:
        kp, mp = p50(k_rack), p50(m_rack)
        within = abs(kp - mp) <= P50_WITHIN
        verdict.update(kernel_rack_p50_ns=kp, model_rack_p50_ns=mp, rack_pass=within)
        lines.append(
            f"(1) RACK-path stall p50: kernel {kp / 1e6:.3f} ms ({len(k_rack)} episodes), model {mp / 1e6:.3f} ms "
            f"({len(m_rack)}): {'PASS' if within else 'MISS'} (within 2 ms)"
        )
        ok &= within
    else:
        lines.append(f"(1) no RACK-path episodes (kernel {len(k_rack)}, model {len(m_rack)}): MISS")
        ok = False
    if m_eps:
        p = m_other / len(m_eps)
        lo, hi = band(len(k_eps), p, LEVEL)
        inside = lo <= k_other <= hi
        verdict.update(kernel_other=k_other, model_other_share=p, band=[lo, hi], other_pass=inside)
        lines.append(
            f"(2) probe-or-timeout share: kernel {k_other} of {len(k_eps)}, model {m_other} of {len(m_eps)} "
            f"(p {p:.4f}); 99 % band {lo}-{hi}: {'PASS' if inside else 'MISS'}"
        )
        ok &= inside
    lines.append(
        f"reported, not judged: episodes at a one-gap threshold kernel {k_eps_gap} (vs {len(k_eps)}), model "
        f"{m_eps_gap} (vs {len(m_eps)}); the model's own repairs rack/probe/timeout {m_rep}"
    )
    lines += [f"problem: {p}" for p in problems]
    if "INVALID" in kinds:
        kind = "INVALID"
    elif "VOID" in kinds:
        kind = "VOID"
    else:
        kind = "PASS" if ok else "MISS"
    follow = {
        "PASS": "final",
        "MISS": "never re-run: to the advisor before U3c closes",
        "VOID": "not judged; one rerun on a quiet container, a second VOID or INVALID goes to the advisor",
        "INVALID": "not judged; one rerun on a quiet container, a second VOID or INVALID goes to the advisor",
    }[kind]
    lines.append(f"VERDICT: {kind} ({follow})")
    verdict["verdict"] = kind
    verdict["pass"] = kind == "PASS"
    json.dump(verdict, open(os.path.join(out, "verdict.json"), "w"), indent=1)
    print("\n".join(lines))


main()
