"""Fix pass 2 (idempotent): names that are neither a unit nor a 0/1 flag become flags.
- event.refusal.status (HTTP status) -> event.refusal.http_403 (1 when refused with 403)
- event.room_exit.code (exit code)   -> event.room_exit.nonzero (1 when the exit code is not 0)
"""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "marks-draft.jsonl")
marks = [json.loads(l) for l in open(OUT) if l.strip()]
for m in marks:
    if m["id"] == "PM-0001-25":
        new = []
        for c in m["checks"]:
            if c.get("series") == "event.refusal.status":
                if c["name"].endswith(", high"):
                    continue  # the low/high pair becomes one flag check
                if c["name"].endswith(", low"):
                    c.update({"name": "ws:// and a wrong Origin refused with 403", "series": "event.refusal.http_403",
                              "stat": "min", "op": "==", "value": 1})
                elif c["stat"] == "count":
                    c["series"] = "event.refusal.http_403"
            new.append(c)
        m["checks"] = new
        for r in m["report"]:
            if r.get("series") == "event.refusal.status":
                r.update({"name": "ML*: refused with 403", "series": "event.refusal.http_403", "stat": "min"})
    if m["id"] == "PM-0001-59":
        for c in m["checks"]:
            if c.get("series") == "event.room_exit.code":
                c["series"] = "event.room_exit.nonzero"
                if c["stat"] == "min":
                    c.update({"op": "==", "value": 1})
with open(OUT + ".tmp", "w") as fh:
    for m in marks:
        fh.write(json.dumps(m, ensure_ascii=False) + "\n")
os.replace(OUT + ".tmp", OUT)
print("rewrote", len(marks))
