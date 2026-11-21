"""Validate marks-draft.jsonl against marks/README.md's format and the drafts commit.

Usage: python3 -I -B validate.py <worktree root at the drafts commit>
Writes names.json (every series and figure name, with the marks that read it) beside this script.
"""
import hashlib, json, os, re, sys
from collections import Counter, defaultdict

root = sys.argv[1]
HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "..", "marks-draft.jsonl")
FILES = ["docs/adr/0001-multiplayer-netcode.md", "docs/adr/0004-simulation-core.md", "docs/adr/0006-infrastructure-operations.md"]
WANT = {f"PM-0001-{i}" for i in range(1, 64)} | {"PM-0004-67"} | {f"PM-0006-{i}" for i in range(11, 28)}

# table order and hashes, from the files
order, rows = [], {}
for f in FILES:
    for n, line in enumerate(open(os.path.join(root, f), encoding="utf-8").read().split("\n"), 1):
        m = re.match(r"\| (PM-\d{4}-\d+) \|", line)
        if m and m.group(1) in WANT:
            order.append(m.group(1))
            rows[m.group(1)] = (f, hashlib.sha256(line.rstrip("\r").encode()).hexdigest())

errs, warns = [], []
def E(mid, msg): errs.append(f"{mid}: {msg}")
def W(mid, msg): warns.append(f"{mid}: {msg}")

lines = [l for l in open(SRC, encoding="utf-8").read().split("\n") if l.strip()]
marks = [json.loads(l) for l in lines]
ids = [m["id"] for m in marks]
if ids != order:
    errs.append(f"order differs: first mismatch at {next((i for i,(a,b) in enumerate(zip(ids, order)) if a!=b), min(len(ids),len(order)))}; {len(ids)} vs {len(order)}")

def no_floats(x, path, mid):
    if isinstance(x, float): E(mid, f"float at {path}")
    elif isinstance(x, dict):
        for k, v in x.items(): no_floats(v, f"{path}.{k}", mid)
    elif isinstance(x, list):
        for i, v in enumerate(x): no_floats(v, f"{path}[{i}]", mid)

VERD = {"pass", "reported", "person", "outside"}
VAL = {"V-S", "PM-0006-13", "lg-80", "padding", "late-zero", "reorder", "clean-ref"}
OPS = {"<=", "<", "==", ">=", ">"}
STATS = {"count", "sum", "min", "max", "mean", "p50", "p90", "p99", "p999", "share"}
WIN = {"measured", "all", "warmup"}
SEL = {"layer", "profile", "role", "scene", "tick_hz", "clients", "machine.name", "machine.arch", "process"}
DEC = re.compile(r"^-?\d+(\.\d+)?$")
SER = re.compile(r"^(tick|second)\.(v|s)\.[a-z0-9_]+$|^event\.[a-z0-9_]+\.[a-z0-9_]+$|^header\.params\.[a-z0-9_]+$")
UNITS = ("_ns", "_bytes", "_bits", "_permille", "_count")
CHECK_KEYS = {"name", "series", "figure", "stat", "where", "window", "op", "value", "factor", "against", "from", "parts", "pooled"}
REP_KEYS = {"name", "series", "figure", "stat", "where", "window", "from", "parts", "pooled"}
names = defaultdict(lambda: {"marks": set(), "kind": None})
uses_role = []

for m in marks:
    mid = m["id"]
    no_floats(m, "", mid)
    if mid not in rows: E(mid, "not a wanted row"); continue
    f, h = rows[mid]
    if m["draft"] != {"file": f, "row_sha256": h}: E(mid, "draft file/hash mismatch")
    if m["verdict"] not in VERD: E(mid, f"verdict {m['verdict']}")
    if m["verdict"] in ("reported", "person", "outside") and m["checks"]: E(mid, "checks on a non-pass mark")
    if m["verdict"] == "outside" and m["parts"]: E(mid, "parts on an outside mark")
    if m["verdict"] != "outside" and not m["parts"]: E(mid, "no parts")
    if m["stand_in"] != (mid == "PM-0001-4"): E(mid, "stand_in")
    if m["arm_rule"] != (mid in {"PM-0001-17", "PM-0001-19", "PM-0001-48", "PM-0001-54"}): E(mid, "arm_rule")
    for v in m["validity"]:
        if v not in VAL: E(mid, f"validity {v}")
    for r in m["readings"]:
        if not re.match(r"^R-([1-9]|1\d|2\d)$", r): E(mid, f"reading {r}")
    pn = [p["name"] for p in m["parts"]]
    if len(set(pn)) != len(pn): E(mid, "duplicate part names")
    for p in m["parts"]:
        if not isinstance(p["min"], int) or p["min"] < 1: E(mid, f"part min {p['name']}")
        for k in p["select"]:
            if k == "machine.role": uses_role.append(mid)
            elif k not in SEL and not k.startswith("params."): E(mid, f"select key {k}")
    for b in m["bites"]:
        if not b["id"].startswith(mid + "/"): E(mid, f"bite id {b['id']}")
    if len({b['id'] for b in m['bites']}) != len(m['bites']): E(mid, "duplicate bite ids")
    for kind, lst, keys in (("check", m["checks"], CHECK_KEYS), ("report", m["report"], REP_KEYS)):
        for c in lst:
            extra = set(c) - keys
            if extra: E(mid, f"{kind} {c['name']!r}: unknown keys {extra}")
            if ("series" in c) == ("figure" in c): E(mid, f"{kind} {c['name']!r}: series xor figure")
            if "series" in c:
                s = c["series"]
                if not SER.match(s): E(mid, f"series form {s}")
                if c.get("stat") not in STATS: E(mid, f"{kind} {c['name']!r}: stat {c.get('stat')}")
                if c.get("stat") == "share" and "where" not in c: E(mid, f"{kind} {c['name']!r}: share without where")
                names[s]["marks"].add(mid); names[s]["kind"] = "series"
                last = s.split(".")[-1]
                if not s.startswith("header.") and not last.endswith(UNITS):
                    W(mid, f"no unit suffix (flag?): {s}")
            else:
                names["figure:" + c["figure"]]["marks"].add(mid); names["figure:" + c["figure"]]["kind"] = "figure"
            if "window" in c and c["window"] not in WIN: E(mid, f"window {c['window']}")
            for pr in c.get("parts", []):
                if pr not in pn: E(mid, f"{kind} {c['name']!r}: unknown part {pr}")
            if "from" in c:
                for k in c["from"]:
                    if k not in SEL and k != "machine.role" and not k.startswith("params."): E(mid, f"from key {k}")
            if kind == "check":
                if c.get("op") not in OPS: E(mid, f"op {c.get('op')}")
                if "against" in c:
                    if "factor" not in c or "value" in c: E(mid, f"{c['name']!r}: against needs factor, no value")
                    if not DEC.match(c["factor"]): E(mid, "factor not decimal string")
                else:
                    v = c.get("value")
                    if not (isinstance(v, int) and not isinstance(v, bool)) and not (isinstance(v, str) and DEC.match(v)):
                        E(mid, f"{c['name']!r}: value {v!r}")
                if "where" in c:
                    w = c["where"]
                    if w.get("op") not in OPS or not isinstance(w.get("value"), (int, str)): E(mid, f"where {w}")
    if m["verdict"] == "pass" and m["verdict"] and not m["checks"]: E(mid, "pass mark without checks")
    if m["verdict"] == "pass" and not m["bites"]: W(mid, "pass mark without bites")

json.dump({k: {"kind": v["kind"], "marks": sorted(v["marks"], key=order.index)} for k, v in sorted(names.items())},
          open(os.path.join(HERE, "names.json"), "w"), indent=1)
print("marks", len(marks), "verdicts", dict(Counter(m["verdict"] for m in marks)))
print("series", sum(1 for v in names.values() if v["kind"] == "series"), "figures", sorted(k[7:] for k, v in names.items() if v["kind"] == "figure"))
print("parts selecting machine.role:", sorted(set(uses_role), key=order.index))
print("ERRORS", len(errs)); [print(" ", e) for e in errs]
print("WARNINGS", len(warns)); [print(" ", w) for w in warns]
