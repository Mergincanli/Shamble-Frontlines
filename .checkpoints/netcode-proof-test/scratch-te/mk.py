"""Helpers for the marks draft. Each block script builds its marks and calls emit(marks).

emit() appends to ../marks-draft.jsonl only the marks not yet in it, in order, so a rerun is idempotent
and a restart continues after the last complete line.
"""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "marks-draft.jsonl")
HASHES = json.load(open(os.path.join(HERE, "row-hashes.json")))

def C(name, series=None, stat=None, op=None, value=None, **kw):
    """A check. value: int in base units, or a decimal string."""
    c = {"name": name}
    if series is not None:
        c["series"] = series
    if stat is not None:
        c["stat"] = stat
    for k in ("where", "window"):
        if k in kw:
            c[k] = kw.pop(k)
    c["op"] = op
    if value is not None:
        c["value"] = value
    for k in ("factor", "against", "figure", "from", "parts", "pooled"):
        if k in kw:
            c[k] = kw.pop(k)
    if kw:
        raise SystemExit(f"unknown check fields {kw}")
    return c

def F(name, figure, op, value, **kw):
    c = {"name": name, "figure": figure, "op": op, "value": value}
    c.update(kw)
    return c

def R(name, series=None, stat=None, figure=None, **kw):
    r = {"name": name}
    if figure:
        r["figure"] = figure
    else:
        r["series"] = series
        r["stat"] = stat
    r.update(kw)
    return r

def P(name, select, mn):
    return {"name": name, "select": select, "min": mn}

def B(bid, what):
    return {"id": bid, "what": what}

def M(mid, title, verdict, where, parts, validity, checks, report, bites, readings, notes,
      stand_in=False, arm_rule=False):
    h = HASHES[mid]
    return {"id": mid, "title": title,
            "draft": {"file": h["file"], "row_sha256": h["row_sha256"]},
            "verdict": verdict, "where": where, "parts": parts, "validity": validity,
            "checks": checks, "report": report, "bites": bites,
            "stand_in": stand_in, "arm_rule": arm_rule, "readings": readings, "notes": notes}

def emit(marks):
    have = set()
    if os.path.exists(OUT):
        for line in open(OUT):
            if line.strip():
                have.add(json.loads(line)["id"])
    n = 0
    with open(OUT, "a") as fh:
        for m in marks:
            if m["id"] in have:
                continue
            fh.write(json.dumps(m, ensure_ascii=False) + "\n")
            n += 1
    print(f"appended {n}, skipped {len(marks) - n}")

# Common selects and parts
S30_P2 = {"layer": "L2", "scene": "SS", "profile": "P2", "tick_hz": 30, "clients": 48, "params.soak": "S30"}
ROOM = {"process": "room"}
ALL_VMS = ["R1", "R2", "R3", "R4", "R5", "R6", "R7"]
MS = 1_000_000  # ns per ms
