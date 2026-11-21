"""D4's two models side by side: how far a p99 over 3,000 connects moves by chance alone, so the gaps between the main
session's model (d4_model.py) and the independent model B (u4f-r3/d4b/d4_model_b.py) can be read as noise or not.
It runs d4_model.py's own functions (its source loaded and executed without its last line, main(); the file is only
read) on 99 more seeds (101 to 199, 1,000 connects each, no loss): the p99 over all 99,000 connects, and the spread of
the p99 over the 33 disjoint groups of 3,000 (3 seeds each, as D4 pools them). Writes nothing. Run from the repo root:
  python3 -I .checkpoints/netcode-proof-test/u4f-r3/d4/d4_noise.py"""
import hashlib
import json
import math
import random
import statistics

PATH = ".checkpoints/netcode-proof-test/u4f-r3/d4/d4_model.py"
B = ".checkpoints/netcode-proof-test/u4f-r3/d4b-r31-full.json"
src = open(PATH, encoding="utf-8").read()
assert src.rstrip().endswith("\nmain()"), "d4_model.py's last line changed"
ns = {"__file__": PATH, "__name__": "d4_model_loaded"}
exec(compile(src.rstrip()[: -len("main()")], PATH, "exec"), ns)
connect, p99 = ns["connect"], ns["p99"]

# The main model's pooled p99s (seeds 1-3, no loss), from its saved output u4f-r3/d4/d4-output.md.
MINE = {
    ("(a) mean", "(i)", "as built"): 6.063, ("(a) mean", "(i)", "bitten"): 14.560,
    ("(a) mean", "(ii)", "as built"): 8.685, ("(a) mean", "(ii)", "bitten"): 17.264,
    ("(b) lowest-RTT", "(i)", "as built"): 18.235, ("(b) lowest-RTT", "(i)", "bitten"): 24.324,
    ("(b) lowest-RTT", "(ii)", "as built"): 18.569, ("(b) lowest-RTT", "(ii)", "bitten"): 23.752,
    ("(e) room pings, half lowest RTT", "-", "as built"): 12.847,
    ("(e) room pings, half lowest RTT", "-", "bitten"): 25.403,
    ("[add] exact stamp, mean", "-", "as built"): 2.321, ("[add] exact stamp, mean", "-", "bitten"): 11.807,
    ("[add] exact stamp, lowest-RTT", "-", "as built"): 4.940,
    ("[add] exact stamp, lowest-RTT", "-", "bitten"): 9.895,
}
# Model B's pooled p99s for the same rows (its result's bite_shows lines, which quote clean and bitten pooled values).
B_VALUES = {
    ("(a) mean", "(i)", "as built"): 6.134, ("(a) mean", "(i)", "bitten"): 14.276,
    ("(a) mean", "(ii)", "as built"): 8.831, ("(a) mean", "(ii)", "bitten"): 17.822,
    ("(b) lowest-RTT", "(i)", "as built"): 18.391, ("(b) lowest-RTT", "(i)", "bitten"): 23.997,
    ("(b) lowest-RTT", "(ii)", "as built"): 18.281, ("(b) lowest-RTT", "(ii)", "bitten"): 23.811,
    ("(e) room pings, half lowest RTT", "-", "as built"): 12.712,
    ("(e) room pings, half lowest RTT", "-", "bitten"): 25.833,
    ("[add] exact stamp, mean", "-", "as built"): 2.312, ("[add] exact stamp, mean", "-", "bitten"): 12.194,
    ("[add] exact stamp, lowest-RTT", "-", "as built"): 4.567,
    ("[add] exact stamp, lowest-RTT", "-", "bitten"): 9.922,
}
bite_text = " ".join(json.load(open(B, encoding="utf-8"))["bite_shows"])
for v in B_VALUES.values():
    assert f"{v:.3f}" in bite_text, f"{v} not in model B's result"

errs = {k: [] for k in MINE}
for seed in range(101, 200):
    rng = random.Random(seed)
    for _ in range(1000):
        out = connect(rng, False)
        for k in MINE:
            errs[k].append(out[k])
print(f"d4_model.py sha256 {hashlib.sha256(src.encode()).hexdigest()[:16]}; seeds 101-199 x 1,000 connects, no loss")
print(f"{'row':58} | {'p99 all':>7} {'sd 3k':>6} | {'main':>7} {'B':>7} {'gap':>6} {'gap/sd':>6}")
for k in MINE:
    groups = [p99(errs[k][i:i + 3000]) for i in range(0, 99000, 3000)]
    sd = statistics.stdev(groups)
    gap = MINE[k] - B_VALUES[k]
    print(f"{' / '.join(k):58} | {p99(errs[k]):7.3f} {sd:6.3f} | {MINE[k]:7.3f} {B_VALUES[k]:7.3f} {gap:6.3f} "
          f"{gap / (sd * math.sqrt(2)):6.2f}")
print("gap/sd = (main - B) / (sd x sqrt 2): the gap in units of the spread of a difference of two independent "
      "3,000-connect p99s; within about +-2.5 reads as chance.")
