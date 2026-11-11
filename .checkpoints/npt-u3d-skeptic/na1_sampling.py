#!/usr/bin/env python3
"""NA1's proof (npt-u3d-skeptic, netcode-architect): on one 1,000,000-datagram leg, E (L0 = 0) and -3's P1 p99.9 can
disagree either way near the bar, with no lateness above 2a + 0.25 ms (10.25 ms). E <= 10,000 ns can PASS while -3's
p99.9 FAILs (the "reverse" R-31 rules out), and E can MISS while -3 passes below the cap.

Not run at night (the night runs no scripts). Day session, from anywhere:
    python3 -I /home/user/Shamble-Frontlines/.checkpoints/npt-u3d-skeptic/na1_sampling.py
Pure standard library; reads and writes nothing; prints one line per scenario (a minute or two in all).

Model, as the code at df8975b computes it:
- the delay D = d - a + below(word, 2a + 1): a uniform integer in [20,000,000, 30,000,000] ns at P1
  (impair.rs:121-124), drawn independently of the lateness L (the derivation's own assumption);
- E with L0 = 0 = floor(sum((L - 250,000)+) / n) (socket_tests.rs:775-782), PASS iff E <= 10,000 (:804-806);
- -3's p99.9 = the nearest-rank ceil(0.999 n)-th smallest of D + L (:759-772, :875), PASS iff it lies in
  [29,750,000, 30,250,000] (:836-848, :876).
Only datagrams with L > 250,000 ns can push D + L above 30,250,000, so the late ones are drawn explicitly and the rest
are on time (L = 0); the "full sample" lines re-check that shortcut on whole samples for three seeds.
"""
import random

N = 990_091            # delivered per direction in the dry run's rounds (selftest-20261109T035556Z.txt:33)
LO, HI = 20_000_000, 30_000_000
BAND_HI, BAND_LO = 30_250_000, 29_750_000
ALLOW = 250_000
BAR = 10_000


def rank(n, num, den):
    return min(max(-(-n * num // den), 1), n)


def e_zero(lates, n):
    return sum(max(0, l - ALLOW) for l in lates) // n


def fail_count(n):
    # p99.9 > BAND_HI iff at least n - rank + 1 delays exceed BAND_HI.
    return n - rank(n, 999, 1000) + 1


def trial(lates, n, rng):
    above = sum(1 for l in lates if rng.randint(LO, HI) + l > BAND_HI)
    return above >= fail_count(n)


def full_trial(lates, n, rng):
    delays = [rng.randint(LO, HI) + l for l in lates] + [rng.randint(LO, HI) for _ in range(n - len(lates))]
    delays.sort()
    p999 = delays[rank(n, 999, 1000) - 1]
    return not (BAND_LO <= p999 <= BAND_HI), p999


def ramp(pauses_ms, rate=9_900):
    """Lateness of the datagrams falling due inside release-path pauses of the given lengths (0..G, evenly)."""
    out = []
    for g in pauses_ms:
        k = int(rate * g / 1000)
        out += [int(g * 1_000_000 * (i + 0.5) / k) for i in range(k)]
    return out


SCENARIOS = {
    "1,980 at 5.25 ms": [5_250_000] * 1_980,
    "1,880 at 5.25 ms": [5_250_000] * 1_880,
    "ramps 21 x 10 ms": ramp([10] * 21),
    "ramps 43 x 7 ms": ramp([7] * 43),
    "2,040 at 5.25 ms (MISS side)": [5_250_000] * 2_040,
    "ramps 22 x 10 ms (MISS side)": ramp([10] * 22),
}


def main():
    print(f"n {N}; -3's p99.9 fails at >= {fail_count(N)} delays above {BAND_HI} ns; no lateness above 10 ms")
    for name, lates in SCENARIOS.items():
        e = e_zero(lates, N)
        rng = random.Random(11)
        trials = 2_000
        fails = sum(trial(lates, N, rng) for _ in range(trials))
        mean = sum(max(0.0, min(1.0, (l - ALLOW) / (HI - LO + 1))) for l in lates)
        print(f"{name}: E (L0 = 0) {e} ns, would {'PASS' if e <= BAR else 'MISS'}; expected count above "
              f"{mean:.1f}; -3 p99.9 FAIL in {fails} of {trials} seeds ({100 * fails / trials:.1f} %)")
    lates = SCENARIOS["1,980 at 5.25 ms"]
    rng = random.Random(7)
    for s in range(3):
        failed, p999 = full_trial(lates, N, rng)
        print(f"full sample check {s} (1,980 at 5.25 ms): p99.9 {p999} ns, {'FAIL' if failed else 'PASS'}")


if __name__ == "__main__":
    main()
