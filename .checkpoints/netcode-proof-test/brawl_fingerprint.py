"""BRAWL-1's fingerprint on a judged run, print only (advisor-u4r-sets-r1-full.md, Q1 (3) and next step 4). For each
run folder given, it reads room.jsonl's measured ticks (tick.s.everyone_in_view, one 0/1 per slot) and prints:
- the onset: the first measured tick on which fewer than all slots hold everyone in view, after a stretch where all do;
- the slot with the least share, and how many measured ticks after the onset it holds everyone in view;
- the other slots' count of 1 flags after the onset, folded on (t - 63,736) mod 360 into 12 bins of 30 ticks (the
  door's 12 s cycle at 30 Hz), with the mean count per bin, so a square wave (about 180 ticks near 0, then a climb)
  shows plainly;
- the least per-slot share (the judge's figure, recomputed).
The rest of the fingerprint (overflow resyncs 0; very large units held in at least 0.95 of sets) comes from the run's
summary line in the set's kept log.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl_fingerprint.py <run folder>..."""
import json
import sys

TICK0 = 63_736
for folder in sys.argv[1:]:
    ticks = []
    with open(f"{folder}/room.jsonl", encoding="utf-8") as fh:
        for line in fh:
            if '"everyone_in_view"' not in line:
                continue
            r = json.loads(line)
            if r.get("k") == "tick" and r.get("ph") == "measured":
                ticks.append((r["t"], r["s"]["everyone_in_view"]))
    n = len(ticks[0][1])
    shares = [sum(f[s] for _, f in ticks) for s in range(n)]
    least = min(range(n), key=lambda s: shares[s])
    onset = next((t for t, f in ticks if sum(f) < n), None)
    print(f"== {folder.rstrip('/').split('/')[-1]}: {len(ticks)} measured ticks ({ticks[0][0]}-{ticks[-1][0]}), {n} slots; "
          f"least share slot {least}: {shares[least]}/{len(ticks)} = {shares[least] / len(ticks):.6f}")
    if onset is None:
        print("   no onset: every slot holds everyone in view on every measured tick")
        continue
    after = [(t, f) for t, f in ticks if t >= onset]
    held = sum(f[least] for _, f in after)
    print(f"   onset tick {onset} ({(onset - TICK0) / 30:.1f} s after tick 63,736); every slot held it on all "
          f"{sum(1 for t, _ in ticks if t < onset)} measured ticks before; slot {least} holds it on {held} of the "
          f"{len(after)} ticks from the onset")
    bins = [[] for _ in range(12)]
    for t, f in after:
        others = sum(f) - f[least]
        bins[((t - TICK0) % 360) // 30].append(others)
    means = [sum(b) / len(b) if b else float("nan") for b in bins]
    print("   others' count after the onset, folded on (t - 63,736) mod 360, 30-tick bins: " +
          " ".join(f"{i * 30}:{m:.1f}" for i, m in enumerate(means)))
