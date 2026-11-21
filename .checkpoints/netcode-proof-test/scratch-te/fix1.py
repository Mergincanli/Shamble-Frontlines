"""Fix pass 1 on marks-draft.jsonl (rewrites the file in place, same order; idempotent).

- PM-0001-27: add the times-of-day figure check (ADR 0006's key, the advisor's reading of 2026-11-06).
- PM-0001-27, PM-0006-13: a room process's CPU share of one vCPU is `second.v.proc_cpu_permille`;
  `second.v.cpu_permille` stays LG's machine CPU, which V-S and lg-80 read.
"""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "marks-draft.jsonl")
marks = [json.loads(l) for l in open(OUT) if l.strip()]
TOD = {"name": "the 3 sessions at least 6 hours apart, one between 18:00 and 23:00 Amsterdam (from the results' UTC times)",
       "figure": "times_of_day_rule", "op": "==", "value": 1}
for m in marks:
    if m["id"] in ("PM-0001-27", "PM-0006-13"):
        for r in m["report"]:
            if r.get("series") == "second.v.cpu_permille" and r.get("from") == {"process": "room"}:
                r["series"] = "second.v.proc_cpu_permille"
    if m["id"] == "PM-0001-27" and not any(c.get("figure") == "times_of_day_rule" for c in m["checks"]):
        m["checks"].append(TOD)
tmp = OUT + ".tmp"
with open(tmp, "w") as fh:
    for m in marks:
        fh.write(json.dumps(m, ensure_ascii=False) + "\n")
os.replace(tmp, OUT)
print("rewrote", len(marks))
