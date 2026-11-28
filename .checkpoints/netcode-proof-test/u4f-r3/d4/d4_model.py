"""D4 of the U4f builder brief (u4f-brief.md section 11, row D4; section 2.1; U4F-Q6; NA-4): a read-only model of
ClockSync's offset error at P2 after 2 s of samples, against PM-0001-47's bar, p99 of |estimate - true| at most 5 ms
(marks.json:9104-9111). The main session's model; the independent second model is u4f-r3/d4b/d4_model_b.py, written
without reading this one. It writes nothing. Run from the repo root:
  python3 -I .checkpoints/netcode-proof-test/u4f-r3/d4/d4_model.py              (prints the results)
  python3 -I .checkpoints/netcode-proof-test/u4f-r3/d4/d4_model.py --check F    (re-computes them and compares with
                                                                               the fenced block saved in file F)
The model, as D4 fixes it (time in ms; the true offset delta = server time - host time, seeded per connect):
- 1,000 connects x seeds 1, 2, 3; per direction a one-way delay of 50 ms + U[-10, 10] ms (P2: l1/src/lib.rs:186-188,
  :214-221); hold 0 (the lab runs the room's I/O at every arrival: lab/src/scenario.rs:1231-1235, :655).
- 20 client samples in 2 s: a Ping at every 3rd client step; the steps fall every P = 1000/30 ms of the host clock
  (client/src/lib.rs:485-488), which runs at the server's rate; the first step at a seeded instant inside the first
  tick (lab/src/scenario.rs:931).
- The room answers each Ping at its arrival T with the next tick to run (lab/src/scenario.rs:476, :480;
  server/src/pump.rs:458-467), read by the client as that tick's due instant: ceil(T / P) * P.
- Phase (i): each sample's stamp error uniform in [0, P), independent per sample (the idealised random phase);
  phase (ii): the connect's seeded step phase, every send a whole number of tick periods from it, so the stamp error is
  set by that phase and the up delay's jitter only (SC-1).
- Arrivals (1) as built: the Pong's datagram arrival; (2) bitten, PM-0001-47/tick-quantised: each arrival moved to
  the client's next step (the step's time in place of the arrival).
- Estimators: each sample's offset is S - (t0 + t1) / 2, less half a tick for the stamp's expected bias (a stamp is
  the next tick's instant, on average half a tick after the arrival); (a) the mean of the 20; (b) the lowest-RTT
  sample's; (e) the 8 room Pings of the 2 s (server/src/pump.rs:16, :387-404: every 250 ms, each carrying the server's
  exact send time; its schedule's phase seeded) less half the lowest own-ping RTT: mean of (sent + half RTT - arrival).
Additions, each marked [add]: the mean and lowest-RTT estimators with an exact sub-tick stamp (U4F-Q6 (b)'s
reference: S = T, no half-tick correction); the newest sample alone (PM-0001-47/newest-sample's estimator); (e) with
half the mean own-ping RTT; and a second pass with P2's 2 % loss per direction (l1/src/lib.rs:187), which D4's fixed
parameters leave out (a lost Ping, Pong or room Ping is no sample). p99 is the nearest rank: the ceil(0.99 n)-th
smallest |error|. Both passes and both phase models use the same draws.
"""
import hashlib
import math
import random
import sys

P = 1000.0 / 30.0
ONE_WAY = 50.0
JIT = 10.0
SAMPLES = 20
PING_EVERY = 3
ROOM_PINGS = 8
ROOM_PERIOD = 250.0
CONNECTS = 1000
SEEDS = (1, 2, 3)
LOSS = 0.02
BAR = 5.0
EPS = 1e-9

ORDER = [
    ("(a) mean", "(i)"), ("(a) mean", "(ii)"),
    ("(b) lowest-RTT", "(i)"), ("(b) lowest-RTT", "(ii)"),
    ("(e) room pings, half lowest RTT", "-"),
    ("[add] exact stamp, mean", "-"), ("[add] exact stamp, lowest-RTT", "-"),
    ("[add] newest sample", "(i)"), ("[add] newest sample", "(ii)"),
    ("[add] (e) room pings, half mean RTT", "-"),
]
ARRIVALS = ("as built", "bitten")


def next_step(t, base):
    """The host time of the client's first step at or after host time t; its steps fall at base + k * P."""
    return base + math.ceil((t - base) / P - EPS) * P


def stamp(t_arr):
    """The server time the client reads from the room's Pong: the next tick to run, at or after the arrival."""
    return math.ceil(t_arr / P - EPS) * P


def connect(rng, loss):
    """One connect's draws; returns {(estimator, phase, arrivals): estimate - true offset}."""
    delta = rng.uniform(-1.0e6, 1.0e6)
    base_srv = rng.uniform(0.0, P) + P * rng.randrange(1000, 100000)
    base = base_srv - delta
    samples = []
    for j in range(SAMPLES):
        s0 = base_srv + PING_EVERY * j * P
        d_up = ONE_WAY + rng.uniform(-JIT, JIT)
        d_dn = ONE_WAY + rng.uniform(-JIT, JIT)
        q_i = rng.uniform(0.0, P)
        lost_up, lost_dn = rng.random(), rng.random()
        if loss and (lost_up < LOSS or lost_dn < LOSS):
            continue
        t_arr = s0 + d_up
        t1 = t_arr + d_dn - delta
        samples.append((s0 - delta, t1, next_step(t1, base), t_arr, t_arr + q_i, stamp(t_arr)))
    psi = rng.uniform(0.0, ROOM_PERIOD)
    room = []
    for k in range(ROOM_PINGS):
        sent = base_srv + psi + ROOM_PERIOD * k
        d_dn = ONE_WAY + rng.uniform(-JIT, JIT)
        lost = rng.random()
        if loss and lost < LOSS:
            continue
        a = sent + d_dn - delta
        room.append((sent, a, next_step(a, base)))
    out = {}
    for arr, col in (("as built", 1), ("bitten", 2)):
        rtts = [s[col] - s[0] for s in samples]
        lo = min(range(len(samples)), key=lambda i: rtts[i])
        for phase, scol in (("(i)", 4), ("(ii)", 5)):
            offs = [s[scol] - (s[0] + s[col]) / 2.0 - P / 2.0 for s in samples]
            out[("(a) mean", phase, arr)] = sum(offs) / len(offs) - delta
            out[("(b) lowest-RTT", phase, arr)] = offs[lo] - delta
            out[("[add] newest sample", phase, arr)] = offs[-1] - delta
        exact = [s[3] - (s[0] + s[col]) / 2.0 for s in samples]
        out[("[add] exact stamp, mean", "-", arr)] = sum(exact) / len(exact) - delta
        out[("[add] exact stamp, lowest-RTT", "-", arr)] = exact[lo] - delta
        rcol = 1 if arr == "as built" else 2
        for name, half in (("(e) room pings, half lowest RTT", rtts[lo] / 2.0),
                           ("[add] (e) room pings, half mean RTT", sum(rtts) / len(rtts) / 2.0)):
            out[(name, "-", arr)] = sum(r[0] + half - r[rcol] for r in room) / len(room) - delta
    return out


def p99(values):
    v = sorted(abs(x) for x in values)
    return v[math.ceil(0.99 * len(v)) - 1]


def self_checks():
    """A few hand cases, so a sign or rounding slip stops the model before it prints."""
    assert abs(next_step(10.0 + 2 * P, 10.0) - (10.0 + 2 * P)) < 1e-6
    assert abs(next_step(10.0 + 2 * P + 0.1, 10.0) - (10.0 + 3 * P)) < 1e-6
    assert abs(stamp(3 * P) - 3 * P) < 1e-6 and abs(stamp(3 * P + 0.1) - 4 * P) < 1e-6
    # Equal delays and an exact stamp: the sample's offset is the true one.
    delta, s0, d = 123.4, 1000.0, 50.0
    t0, t1, t_arr = s0 - delta, s0 + 2 * d - delta, s0 + d
    assert abs((t_arr - (t0 + t1) / 2.0) - delta) < 1e-9
    # The quantised stamp less half a tick is within half a tick of the truth.
    for frac in (0.0, 0.25, 0.5, 0.99):
        t_arr = 100 * P + frac * P
        err = stamp(t_arr) - t_arr - P / 2.0
        assert -P / 2.0 - 1e-9 <= err <= P / 2.0 + 1e-9


def run():
    self_checks()
    me = open(__file__, "rb").read()
    lines = [f"D4 model (the main session's), script sha256 {hashlib.sha256(me).hexdigest()[:16]}, "
             f"Python {sys.version.split()[0]}",
             f"P = {P:.3f} ms; one-way 50 ms + U[-10, 10] ms each direction; {SAMPLES} pings (every {PING_EVERY} "
             f"steps); {ROOM_PINGS} room pings (every {ROOM_PERIOD:.0f} ms); {CONNECTS} connects x seeds "
             f"{SEEDS[0]}-{SEEDS[-1]}; bar p99 <= {BAR:.1f} ms"]
    shows = {}
    for loss in (False, True):
        lines.append("")
        lines.append("pass: " + ("P2's 2 % loss per direction ([add], a sensitivity pass)" if loss
                                  else "no loss (D4's fixed parameters)"))
        lines.append(f"{'estimator':38} {'phase':5} {'arrivals':8} | {'p99 s1':>7} {'p99 s2':>7} {'p99 s3':>7} | "
                     f"{'pooled':>7} {'mean':>7} {'max':>7} | <= 5 ms")
        per = {}
        for seed in SEEDS:
            rng = random.Random(seed)
            for _ in range(CONNECTS):
                for key, err in connect(rng, loss).items():
                    per.setdefault(key, {}).setdefault(seed, []).append(err)
        for name, phase in ORDER:
            for arr in ARRIVALS:
                by_seed = per[(name, phase, arr)]
                pooled = [e for s in SEEDS for e in by_seed[s]]
                pp = p99(pooled)
                lines.append(f"{name:38} {phase:5} {arr:8} | "
                             + " ".join(f"{p99(by_seed[s]):7.3f}" for s in SEEDS)
                             + f" | {pp:7.3f} {sum(pooled) / len(pooled):7.3f} "
                             f"{max(abs(e) for e in pooled):7.3f} | {'yes' if pp <= BAR else 'NO'}")
                if not loss:
                    shows[(name, phase, arr)] = pp
    lines.append("")
    lines.append("PM-0001-47/tick-quantised, no-loss pass: the bite shows where the clean p99 is at most 5 ms and the "
                 "bitten p99 above it")
    for name, phase in ORDER:
        clean, bitten = shows[(name, phase, "as built")], shows[(name, phase, "bitten")]
        verdict = ("shows" if clean <= BAR < bitten else
                   "does not show (the clean run already fails)" if clean > BAR else
                   "does not show (the bitten run passes)")
        lines.append(f"  {name:38} {phase:5}: clean {clean:7.3f}, bitten {bitten:7.3f}: {verdict}")
    return "\n".join(lines) + "\n"


def main():
    text = run()
    if len(sys.argv) == 3 and sys.argv[1] == "--check":
        saved = open(sys.argv[2], encoding="utf-8").read()
        start = saved.index("```text\n") + len("```text\n")
        block = saved[start:saved.index("```", start)]
        if block == text:
            print("MATCH: the saved output equals a fresh run, byte for byte")
        else:
            a, b = block.splitlines(), text.splitlines()
            first = next((i for i in range(min(len(a), len(b))) if a[i] != b[i]), min(len(a), len(b)))
            print(f"MISMATCH at line {first + 1}:\n saved: {a[first] if first < len(a) else '<end>'}\n fresh: "
                  f"{b[first] if first < len(b) else '<end>'}")
            sys.exit(1)
    else:
        sys.stdout.write(text)


main()
