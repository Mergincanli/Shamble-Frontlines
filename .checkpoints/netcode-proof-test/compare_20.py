"""Read-only: compare -20's re-run on U4f-F1's merge with 350677f's kept run, as the advisor's condition 5 asks
(advisor-u4f-f1-plan.md:119-140; advisor-u4f-f1-r1.md:191-205): for each run, (1) the summary line in runs.txt with
its "(wall ...)" and the header lines' binary sha256s set aside, (2) every room log tick record's world hash `h`,
tick by tick, (3) the sum of tick.v.press_lost_count, press_count and press_doubled_count.
Usage: python3 -I compare_20.py <old results dir> <new results dir> <old runs dir> <new runs dir>"""
import json
import os
import re
import sys

old_res, new_res, old_runs, new_runs = sys.argv[1:5]
RUNS = [("measured", r) for r in ("l1-20-a-s1", "l1-20-a-s2", "l1-20-a-s3", "l1-20-b-s1", "l1-20-b-s2", "l1-20-b-s3")]
RUNS.append(("bites", "bite-redundancy-1"))


def summary_lines(res):
    out = {}
    for line in open(os.path.join(res, "runs.txt"), encoding="utf-8"):
        m = re.match(r"^(l1-20-[ab]-s\d|bite-redundancy-1) ", line)
        if m:
            out[m.group(1)] = re.sub(r" \(wall [0-9.]+ s\)\s*$", "", line.rstrip("\n"))
    return out


def ticks(path):
    hs, lost, seen, doubled = {}, 0, 0, 0
    for line in open(path, encoding="utf-8"):
        if not line.startswith('{"k":"tick"'):
            continue
        r = json.loads(line)
        hs[r["t"]] = r.get("h")
        v = r.get("v", {})
        lost += v.get("press_lost_count", 0)
        seen += v.get("press_count", 0)
        doubled += v.get("press_doubled_count", 0)
    return hs, lost, seen, doubled


old_s, new_s = summary_lines(old_res), summary_lines(new_res)
for phase, run in RUNS:
    same_line = old_s.get(run) == new_s.get(run)
    ho, lo, so, do = ticks(os.path.join(old_runs, phase, run, "room.jsonl"))
    hn, ln, sn, dn = ticks(os.path.join(new_runs, phase, run, "room.jsonl"))
    common = sorted(set(ho) & set(hn))
    diff = [t for t in common if ho[t] != hn[t]]
    print(f"{run}: summary line (wall aside) {'EQUAL' if same_line else 'DIFFERS'}; ticks old {len(ho)} new {len(hn)}, "
          f"common {len(common)}, h mismatches {len(diff)}{(' first at ' + str(diff[0])) if diff else ''}; "
          f"presses seen {so} -> {sn}, lost {lo} -> {ln}, doubled {do} -> {dn}")
    if not same_line:
        print("   old:", old_s.get(run, "(none)")[:600])
        print("   new:", new_s.get(run, "(none)")[:600])
