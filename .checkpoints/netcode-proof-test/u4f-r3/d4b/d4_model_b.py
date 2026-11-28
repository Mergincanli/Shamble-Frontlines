"""D4, model B (the netcode-architect's independent model; U4f brief rev 3.1, section 11 row D4).

Read only: standard library, seeded random.Random per seed, prints its results and writes nothing.
Every modelling choice is in d4b-notes.md (C1-C14); the cites are the head's (spikes/ unchanged since 0ad9661).

Run: python3 -I <M>/.checkpoints/netcode-proof-test/u4f-r3/d4b/d4_model_b.py
"""

import random

SEC = 1_000_000_000
MS = 1_000_000
HZ = 30                       # npt_sim::TICK_HZ at 30 (the plan's rate)
P_FLOOR = SEC // HZ           # 33,333,333 ns
ONE_WAY = 50 * MS             # P2: rtt 100 ms, half each way (l1/src/lib.rs:186-188, :214-216)
HALF_JIT = 10 * MS            # P2: +-20 ms round trip, +-10 ms each way (:217-221)
LOSS_PPM = 20_000             # P2: 2 % each direction (:186-188, :935)
CONNECTS = 1_000
SEEDS = (1, 2, 3)
N_OWN = 20                    # 20 client samples in 2 s
PING_EVERY = 3                # one ping every 3 client steps at 30 Hz (100 ms)
N_ROOM = 8                    # the room's pings in the 2 s
ROOM_INTERVAL = 250 * MS      # PING_INTERVAL_NS (server/src/pump.rs:16)
K0 = 300                      # the connect's first ping 10 s into the run
BAR = 5 * MS                  # marks.json:9104-9111


def tick_ns(k):
    """Tick k's due instant: floor(k * 1e9 / hz) (server/src/schedule.rs:125-134); also the client's step grid
    (client/src/lib.rs:485-488)."""
    return k * SEC // HZ


def first_at_or_after(x):
    """The smallest k with tick_ns(k) >= x (x >= 0): ceil(hz * x / 1e9). For a Ping arriving at server time x this is
    the next tick to run (scenario.rs:476; an arrival exactly at a tick instant is stamped that tick: RQ8's I/O step
    before the tick); for an arrival at host time x after a client base it is the client step that reads it."""
    return -((-x * HZ) // SEC)


def delay(rng):
    """One direction's delay: a uniform integer ns in [one_way - a, one_way + a] (l1/src/lib.rs:846)."""
    return ONE_WAY + rng.randint(0, 2 * HALF_JIT) - HALF_JIT


def lost(rng):
    """One direction's loss draw (l1/src/lib.rs:935)."""
    return rng.randrange(1_000_000) < LOSS_PPM


def p99(values):
    """The judge's nearest-rank p99: the ceil(0.99 n)-th smallest, 1-based (judge/src/eval.rs:264-289)."""
    s = sorted(values)
    rank = (99 * len(s) + 99) // 100
    return s[rank - 1]


def median(values):
    s = sorted(values)
    return s[(len(s) + 1) // 2 - 1]


PHASES = ("i", "ii")
ARRIVALS = ("clean", "bitten")
STAMPS = ("tick", "exact", "tick-uncentred")


def put(acc, key, seed, value):
    acc.setdefault(key, {}).setdefault(seed, []).append(value)


def run_seed(seed, acc, diag):
    rng = random.Random(seed)
    for _ in range(CONNECTS):
        # C12: a fixed number of draws per connect, in a fixed order, shared by every variant.
        h = rng.randrange(0, 1_000 * SEC)          # the lab's host offset range (scenario.rs:930)
        phi = rng.randrange(0, P_FLOOR + 1)        # (ii)'s one phase per connect
        r0 = rng.randrange(0, ROOM_INTERVAL)       # the room-ping schedule's start (C8)
        own = []
        for _m in range(N_OWN):
            d_up = delay(rng)
            d_down = delay(rng)
            u = rng.randrange(0, P_FLOOR + 1)      # (i)'s per-sample phase
            lu = lost(rng)
            ld = lost(rng)
            own.append((d_up, d_down, u, lu or ld))
        room = []
        for _i in range(N_ROOM):
            d = delay(rng)
            lr = lost(rng)
            room.append((d, lr))

        s0 = tick_ns(K0) + phi                     # server time of the first ping's step
        base = s0 + h                              # the client's step 0 on its host clock

        # Own pings: host send, server arrival, host arrival (clean and bitten).
        sends = []
        for m, (d_up, d_down, u, _l) in enumerate(own):
            c_s = base + tick_ns(PING_EVERY * m)  # the step's host time (= base + m * 1e8 exactly)
            s_s = c_s - h
            s_r = s_s + d_up                       # the Ping's arrival at the room; the Pong leaves then (hold 0)
            c_r = s_r + d_down + h                 # the Pong's arrival on the host clock
            c_rb = base + tick_ns(first_at_or_after(c_r - base))  # C10: moved to the step that reads it
            sends.append((c_s, s_r, c_r, c_rb, u))

        # Per (phase, stamp, arrivals): twice the per-sample error, 2 * (est_m - h) = c_s + c_r - 2 * s_hat - 2h.
        e2 = {}
        for phase in PHASES:
            for c_s, s_r, c_r, c_rb, u in sends:
                g = u if phase == "i" else 0       # C7: (i) shifts the quantising grid per sample
                k = first_at_or_after(s_r - g)
                t_k = tick_ns(k) + g
                t_km1 = tick_ns(k - 1) + g
                two_hat = {
                    "tick": t_km1 + t_k,           # C4: the centred stamp
                    "tick-uncentred": 2 * t_k,
                    "exact": 2 * (t_km1 + ((s_r - t_km1) // 1_000) * 1_000),  # C6: to the us
                }
                for stamp in STAMPS:
                    for arr, cr in (("clean", c_r), ("bitten", c_rb)):
                        e2.setdefault((phase, stamp, arr), []).append(c_s + cr - two_hat[stamp] - 2 * h)
                if phase == "ii" and s_r == t_k:
                    diag["at_tick_instant"] += 1

        # Room pings: exact server send ns; host arrival clean and bitten.
        rooms = []
        for i, (d, _l) in enumerate(room):
            s_send = s0 + r0 + i * ROOM_INTERVAL
            c_a = s_send + d + h
            c_ab = base + tick_ns(first_at_or_after(c_a - base))
            rooms.append((s_send, c_a, c_ab))

        for loss in (False, True):
            kept = [m for m in range(N_OWN) if not (loss and own[m][3])]
            kept_room = [i for i in range(N_ROOM) if not (loss and room[i][1])]
            if loss:
                diag["own_kept"] += len(kept)
                diag["room_kept"] += len(kept_room)
            for arr in ARRIVALS:
                rtt = [(sends[m][3] if arr == "bitten" else sends[m][2]) - sends[m][0] for m in range(N_OWN)]
                if not kept:
                    diag["no_own"] += 1
                    continue
                low = kept[0]
                for m in kept:
                    if rtt[m] < rtt[low]:          # C10: first minimum in send order
                        low = m
                newest = kept[-1]
                if arr == "bitten" and not loss:
                    t = round(rtt[low] * HZ / SEC)
                    diag["rtt_ticks"][t] = diag["rtt_ticks"].get(t, 0) + 1
                for phase in PHASES:
                    for stamp in STAMPS:
                        v = e2[(phase, stamp, arr)]
                        put(acc, ("mean", stamp, phase, arr, loss), seed, sum(v[m] for m in kept) / (2 * len(kept)))
                        put(acc, ("lowest-RTT", stamp, phase, arr, loss), seed, v[low] / 2)
                        put(acc, ("newest", stamp, phase, arr, loss), seed, v[newest] / 2)
                # (e): each room ping's 2 * (est_i - h) = 2 * (c_a - s_send - h) - min RTT (C9).
                if not kept_room:
                    diag["no_room"] += 1
                    continue
                min_rtt = rtt[low]
                ei = []
                for i in kept_room:
                    s_send, c_a, c_ab = rooms[i]
                    ca = c_ab if arr == "bitten" else c_a
                    ei.append((ca - s_send, 2 * (ca - s_send - h) - min_rtt))
                e_mean = sum(x for _, x in ei) / (2 * len(ei))
                lo = ei[0]
                for pair in ei:
                    if pair[0] < lo[0]:            # the lowest one-way, first in send order
                        lo = pair
                e_low = lo[1] / 2
                for phase in PHASES:              # C8: the same under (i) and (ii)
                    put(acc, ("e/mean-of-8", "room-exact", phase, arr, loss), seed, e_mean)
                    put(acc, ("e/lowest-of-8", "room-exact", phase, arr, loss), seed, e_low)


def main():
    acc = {}
    diag = {"at_tick_instant": 0, "own_kept": 0, "room_kept": 0, "no_own": 0, "no_room": 0, "rtt_ticks": {}}
    for seed in SEEDS:
        run_seed(seed, acc, diag)

    print("D4 model B: P2 per direction 50 ms + U[-10, 10] ms (integer ns), 30 Hz, 20 own pings every 3 steps,")
    print("8 room pings every 250 ms, 1,000 connects x seeds 1, 2, 3; p99 nearest-rank of |estimate - true|; bar 5 ms.")
    print("stamps: tick = the next tick to run, centred (T_{k-1} + T_k) / 2; exact = the server's arrival ns to the us;")
    print("room-exact = the room ping's own sent_ns; tick-uncentred = T_k (sensitivity only).")
    print("phase (i) uniform per sample; (ii) one seeded phase per connect. arrivals clean = as built (datagram")
    print("arrival); bitten = moved to the client step that reads it (PM-0001-47/tick-quantised). loss = P2's 2 %.")
    print("all values in ms; 'med sgn' = the median signed error (estimate - true).")
    print()

    order = [
        ("mean", "tick", "main"), ("lowest-RTT", "tick", "main"),
        ("e/mean-of-8", "room-exact", "main"), ("e/lowest-of-8", "room-exact", "main"),
        ("mean", "exact", "addition"), ("lowest-RTT", "exact", "addition"),
        ("newest", "tick", "addition"), ("newest", "exact", "addition"),
        ("mean", "tick-uncentred", "sensitivity"), ("lowest-RTT", "tick-uncentred", "sensitivity"),
    ]
    hdr = "{:<14} {:<14} {:<3} {:<6} {:<4} {:>7} {:>7} {:>7} {:>7} {:>8} {:>7} {:<4} {}".format(
        "estimator", "stamp", "ph", "arr", "loss", "p99 s1", "p99 s2", "p99 s3", "pooled", "med sgn", "max", "pass",
        "kind")
    row = "{:<14} {:<14} {:<3} {:<6} {:<4} {:>7.3f} {:>7.3f} {:>7.3f} {:>7.3f} {:>8.3f} {:>7.3f} {:<4} {}"
    pooled_p99 = {}
    for loss in (False, True):
        print("=== loss {} ===".format("2 % each way (every row an addition)" if loss else "off (D4's parameters)"))
        print(hdr)
        for est, stamp, kind in order:
            for phase in PHASES:
                for arr in ARRIVALS:
                    key = (est, stamp, phase, arr, loss)
                    per = acc[key]
                    ps = [p99([abs(x) for x in per[s]]) for s in SEEDS]
                    allv = [x for s in SEEDS for x in per[s]]
                    pp = p99([abs(x) for x in allv])
                    pooled_p99[key] = (ps, pp)
                    mx = max(abs(x) for x in allv)
                    md = median(allv)
                    label = kind if not loss else (kind + " (loss)" if kind == "sensitivity" else "addition (loss)")
                    print(row.format(est, stamp, phase, arr, "on" if loss else "off", ps[0] / MS, ps[1] / MS,
                                     ps[2] / MS, pp / MS, md / MS, mx / MS, "yes" if pp <= BAR else "no", label))
        print()

    print("=== does the tick-quantised bite show? (clean p99 <= 5 ms and bitten p99 > 5 ms) ===")
    for loss in (False, True):
        for est, stamp, kind in order:
            if kind == "sensitivity":
                continue
            for phase in PHASES:
                cps, cpp = pooled_p99[(est, stamp, phase, "clean", loss)]
                bps, bpp = pooled_p99[(est, stamp, phase, "bitten", loss)]
                pooled = cpp <= BAR < bpp
                seeds = all(cps[j] <= BAR < bps[j] for j in range(3))
                if cpp > BAR:
                    why = "masked: clean fails"
                elif bpp <= BAR:
                    why = "no: bitten passes"
                else:
                    why = "yes"
                print("loss {:<3} {:<14} {:<14} ph {:<3} clean {:>7.3f} bitten {:>7.3f}  pooled {:<3} all seeds {:<3} ({})"
                      .format("on" if loss else "off", est, stamp, phase, cpp / MS, bpp / MS,
                              "yes" if pooled else "no", "yes" if seeds else "no", why))
    print()

    print("=== does the newest-sample bite show? (the estimator passes clean; the newest sample, same stamp, clean, fails) ===")
    for loss in (False, True):
        for est, stamp in (("mean", "tick"), ("lowest-RTT", "tick"), ("mean", "exact"), ("lowest-RTT", "exact")):
            for phase in PHASES:
                _, epp = pooled_p99[(est, stamp, phase, "clean", loss)]
                _, npp = pooled_p99[("newest", stamp, phase, "clean", loss)]
                if epp > BAR:
                    why = "masked: estimator fails"
                elif npp <= BAR:
                    why = "no: newest passes"
                else:
                    why = "yes"
                print("loss {:<3} {:<11} {:<6} ph {:<3} estimator {:>7.3f} newest {:>7.3f}  shows: {}".format(
                    "on" if loss else "off", est, stamp, phase, epp / MS, npp / MS, why))
    print()

    print("=== diagnostics ===")
    print("own samples whose Ping reached the room exactly at a tick instant (phase ii, all seeds):",
          diag["at_tick_instant"])
    tot_own = len(SEEDS) * CONNECTS * N_OWN
    tot_room = len(SEEDS) * CONNECTS * N_ROOM
    print("loss addition: own samples kept {} of {} ({:.2f} %), room pings kept {} of {} ({:.2f} %)".format(
        diag["own_kept"], tot_own, 100 * diag["own_kept"] / tot_own, diag["room_kept"], tot_room,
        100 * diag["room_kept"] / tot_room))
    print("connects with no own sample / no room ping (loss on, counted per arrivals):", diag["no_own"],
          diag["no_room"])
    print("bitten lowest RTT in ticks, per connect (loss off, all seeds):", dict(sorted(diag["rtt_ticks"].items())))


main()
