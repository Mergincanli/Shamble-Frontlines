# Read-only model (prints only): at L1 P3 (l1/src/lib.rs:190-192, :214-221; 1 % adjacent swaps :146, :173),
# how far the client's newest snapshot tick moves per client step, and so how many stamps the rev 3.1 rule
# "at most 2 a step; a target more than 2 ahead leaves the stamps between unused" (u4f-brief.md 2.2 :302)
# leaves unused under U4F-Q7 (a) (target = newest + L), against -20's 0.1 % missing (marks.json:4724-4737).
# Seeded, no I/O beyond print.
import random

P = 1000.0 / 30.0          # tick period, ms
ONE_WAY = 75.0             # ms, P3 half RTT
HALF_JIT = 15.0            # ms, P3 half jitter
LOSS = 0.05                # each way
SWAP = 0.01                # adjacent pairs swapped (PART_13)

def run(seed, ticks):
    r = random.Random(seed)
    arr = []
    for n in range(ticks):
        if r.random() < LOSS:
            arr.append(None)
        else:
            arr.append(n * P + ONE_WAY + r.uniform(-HALF_JIT, HALF_JIT))
    # the reorder step: swap adjacent datagrams' arrival times with probability SWAP
    for n in range(ticks - 1):
        if arr[n] is not None and arr[n + 1] is not None and r.random() < SWAP:
            arr[n], arr[n + 1] = arr[n + 1], arr[n]
    phase = r.uniform(0, P)
    # newest at each step time: process arrivals in time order
    events = sorted((a, n) for n, a in enumerate(arr) if a is not None)
    newest = -1
    i = 0
    jumps = {}
    skipped = 0
    backlog_max = 0
    backlog = 0
    steps = 0
    k = 0
    prev_newest = None
    while True:
        t = phase + k * P + ONE_WAY + 3 * P  # start once traffic flows
        if t > (ticks - 10) * P:
            break
        while i < len(events) and events[i][0] <= t:
            newest = max(newest, events[i][1])
            i += 1
        if prev_newest is not None:
            j = newest - prev_newest
            jumps[j] = jumps.get(j, 0) + 1
            if j > 2:
                skipped += j - 2
            # catch-up reading: owed stamps carried, at most 2 a step
            backlog += j
            give = min(2, backlog)
            backlog -= give
            backlog_max = max(backlog_max, backlog)
            steps += 1
        prev_newest = newest
        k += 1
    return steps, jumps, skipped, backlog_max

tot_steps = 0
tot_skip = 0
tot_j = {}
bmax = 0
for seed in (1, 2, 3):
    s, j, sk, bm = run(seed, 300_000)
    tot_steps += s
    tot_skip += sk
    bmax = max(bmax, bm)
    for a, b in j.items():
        tot_j[a] = tot_j.get(a, 0) + b
print("steps", tot_steps)
for a in sorted(tot_j):
    print("jump", a, tot_j[a], "share %.5f" % (tot_j[a] / tot_steps))
print("skip reading: stamps unused", tot_skip, "share of ticks %.5f" % (tot_skip / tot_steps))
print("catch-up reading: largest backlog (stamps owed after a step)", bmax)
