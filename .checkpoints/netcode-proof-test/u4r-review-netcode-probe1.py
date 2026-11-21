"""Read-only probe (netcode seat, U4r round 1): per r2 suite folder, the lab log's client_counts sums, run_lengths and
class_audit, and the room log's measured late_count sum and everyone_in_view share. Prints only; writes nothing."""
import json
import os
import sys

root = sys.argv[1]
for name in sorted(os.listdir(root)):
    d = os.path.join(root, name)
    lab = os.path.join(d, "lab.jsonl")
    room = os.path.join(d, "room.jsonl")
    if not (os.path.isdir(d) and os.path.exists(lab)):
        continue
    sums = {}
    rl = ca = None
    with open(lab) as f:
        for line in f:
            r = json.loads(line)
            if r.get("k") == "event" and r.get("ev") == "client_counts":
                for k, v in r["f"].items():
                    if k != "slot":
                        sums[k] = sums.get(k, 0) + v
            elif r.get("k") == "event" and r.get("ev") == "run_lengths":
                rl = r["f"]
            elif r.get("k") == "event" and r.get("ev") == "class_audit":
                ca = r["f"]
    late_meas = 0
    flags = 0
    sets = 0
    hdr = None
    first_meas = last_meas = None
    with open(room) as f:
        for line in f:
            r = json.loads(line)
            if r.get("k") == "header":
                hdr = r
            if r.get("k") == "tick" and r.get("ph") == "measured":
                late_meas += r["v"].get("late_count", 0)
                fl = r["s"].get("everyone_in_view", [])
                flags += sum(fl)
                sets += len(fl)
                first_meas = r["t"] if first_meas is None else first_meas
                last_meas = r["t"]
    cmds = sums.get("commands_count", 0)
    late = sums.get("late_count", 0)
    print(
        f"{name}: bite={hdr.get('bite') if hdr else None} tick_start={hdr['params'].get('tick_start') if hdr else None} "
        f"twrap={hdr['params'].get('ticks_below_wrap') if hdr else None} measured {first_meas}..{last_meas}; "
        f"commands {cmds} late {late} ({(100.0 * late / cmds) if cmds else 0:.3f} %) measured-late {late_meas}; "
        f"rewinds_refused {sums.get('rewinds_refused_count', 0)} stale {sums.get('stale_count', 0)} "
        f"overflow {sums.get('overflow_resyncs_count', 0)} skipped {sums.get('steps_without_command_count', 0)} "
        f"dup {sums.get('duplicate_stamps_count', 0)}; everyone {flags}/{sets}; run_lengths {rl}; "
        f"breaches {ca.get('breaches_count') if ca else None}"
    )
