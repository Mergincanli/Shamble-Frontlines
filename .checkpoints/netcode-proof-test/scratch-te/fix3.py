"""Fix pass 3 (idempotent): one name per quantity.
- PM-0001-58: '0 corrections larger than 1 mm (PM-0001-41's count)' reads -41's untraced-corrections count;
  a max over every correction would FAIL at P3, where corrections traced to a lost command are legitimate.
- PM-0001-19: the join hash check is the same comparison as decoded_hash_mismatch_count (-17, -58).
- PM-0001-14: maxDatagramSize is -36's event.mds_read.size_bytes, not a second name.
"""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "marks-draft.jsonl")
marks = [json.loads(l) for l in open(OUT) if l.strip()]
for m in marks:
    if m["id"] == "PM-0001-58":
        for c in m["checks"]:
            if c.get("series") == "tick.s.correction_um":
                c.clear()
                c.update({"name": "0 corrections larger than 1 mm not traced to a late or lost command (PM-0001-41's count)",
                          "series": "tick.v.untraced_corrections_count", "stat": "sum", "window": "all", "op": "==", "value": 0})
        if "traced to a lost command" not in m["notes"]:
            m["notes"] += " The correction count is PM-0001-41's: at P3 a correction traced to a lost command is allowed; one with every command in time is not."
    if m["id"] == "PM-0001-19":
        for c in m["checks"]:
            if c.get("figure") == "join_hash_mismatch_count":
                c["figure"] = "decoded_hash_mismatch_count"
    if m["id"] == "PM-0001-14":
        for r in m["report"]:
            if r.get("series") == "second.v.max_datagram_size_bytes":
                r.update({"series": "event.mds_read.size_bytes", "window": "all"})
with open(OUT + ".tmp", "w") as fh:
    for m in marks:
        fh.write(json.dumps(m, ensure_ascii=False) + "\n")
os.replace(OUT + ".tmp", OUT)
print("rewrote", len(marks))
