"""Read-only probe 2 (netcode seat, U4r round 1): B1's worst slot share, the heal records (fast within 500 ms, by
burst, with the eviction age), ES's reliable events per measured second. Prints only; writes nothing."""
import json
import os
import sys

root = sys.argv[1]


def ticks(path):
    with open(path) as f:
        for line in f:
            yield json.loads(line)


def worst(name):
    per = None
    n = 0
    for r in ticks(os.path.join(root, name, "room.jsonl")):
        if r.get("k") == "tick" and r.get("ph") == "measured":
            fl = r["s"]["everyone_in_view"]
            per = [0] * len(fl) if per is None else per
            for i, v in enumerate(fl):
                per[i] += v
            n += 1
    m = min(per)
    print(f"{name}: measured ticks {n}; worst slot {per.index(m)} holds everyone on {m} ({m / n:.6f})")


def heals(name):
    ev = {}
    for r in ticks(os.path.join(root, name, "room.jsonl")):
        if r.get("k") == "event" and r.get("ev") == "burst_eviction":
            f = r["f"]
            ev[(f["slot"], f["burst"])] = (f["evicted"], f["baseline_age"])
    fast = {}
    slow = 0
    fs = 0
    total = 0
    mx = 0
    for r in ticks(os.path.join(root, name, "lab.jsonl")):
        if r.get("k") == "event" and r.get("ev") == "burst_heal":
            f = r["f"]
            total += 1
            mx = max(mx, f["after_link_ns"])
            fs += f["full_state"]
            if f["after_link_ns"] <= 500_000_000:
                age = ev.get((f["slot"], f["burst"]), (None, None))[1]
                fast.setdefault((f["burst"], age), 0)
                fast[(f["burst"], age)] += 1
            else:
                slow += 1
    print(f"{name}: heals {total}, max {mx} ns, full_state {fs}, slow {slow}, fast by (burst, eviction age) {fast}; "
          f"evictions {len(ev)} evicted {sum(e[0] for e in ev.values())}")


def es(name):
    total = 0
    secs = 0
    shots = 0
    for r in ticks(os.path.join(root, name, "room.jsonl")):
        if r.get("k") == "second" and r.get("ph") == "measured":
            total += r["v"]["reliable_events_count"]
            shots += r["v"]["shots_count"]
            secs += 1
    print(f"{name}: measured seconds {secs}; reliable events {total} ({total / secs:.2f}/s); shots {shots} ({shots / secs:.1f}/s)")


worst("b1")
heals("b1")
heals("b1-nofallback")
heals("b1-nofallback2")
es("es")
es("p0-s1")
