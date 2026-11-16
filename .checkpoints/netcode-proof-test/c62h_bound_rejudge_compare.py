"""C62H-BOUND's P0 re-judge comparison (the build brief c62h-bound-brief.md section 7, sha256 7936074c...; Mustafa's
C62H-BOUND-YES (a), 2026-11-15 20:50 Amsterdam, history.md:2174: "A changed judge must give the same answers on a set
it was not changed for"). Read only. It checks the earlier answers' sha256 (results/u4r/93509dfd7fd1/P0/verdict.json
e926b812... and verdict-bites.json b472784f...) and refuses on any other. Each parsed new file (<O>/verdict.json and
<O>/verdict-bites.json) must then equal the earlier one with exactly the two additions this lane makes, at their places
and in their forms: (i) -17's new bite entry {"id": "PM-0001-17/link-lost-overstated", "status": "not_run", "runs": []}
right after whole-as-lost's; (ii) exactly one -17 why line "bite PM-0001-17/link-lost-overstated: no run yet", after
whole-as-lost's in verdict.json and last in -17's why list in verdict-bites.json. It prints each part and bite that
holds a run, new beside earlier (setParts' form, c-u4r-runs.mjs:268-283), then SAME ANSWERS or every difference.
It refuses if the new bite entry appears more than once in a file. Exit 0: SAME ANSWERS; 1: a difference; 2: refused.
--self-test reads the earlier files and builds in memory: the earlier with the two additions (must give SAME ANSWERS);
one -17 L1 P0 check value changed (must print that difference); the new bite entry twice (must refuse);
verdict-bites.json without the new why line (must print a difference). It exits 0 only if all four behave so.
Run from the repository root:
  python3 -I .checkpoints/netcode-proof-test/c62h_bound_rejudge_compare.py <O>
  python3 -I .checkpoints/netcode-proof-test/c62h_bound_rejudge_compare.py --self-test"""
import copy
import hashlib
import json
import os
import sys

E = "/home/user/Shamble-Frontlines/spikes/netcode-proof/results/u4r/93509dfd7fd1/P0"
FILES = ("verdict.json", "verdict-bites.json")
EARLIER = {"verdict.json": "e926b812cbe57d81e251bfb5c6685b889a4d2859e9cac25022f6788bf74af095",
           "verdict-bites.json": "b472784f2c7208aeef38af373a8bc257bf0b3b88a18dc5dc376fb6533536942e"}
NEW_BITE = {"id": "PM-0001-17/link-lost-overstated", "status": "not_run", "runs": []}
NEW_WHY = "bite PM-0001-17/link-lost-overstated: no run yet"
WAL = "PM-0001-17/whole-as-lost"
WAL_WHY = "bite PM-0001-17/whole-as-lost: no run yet"


class Refused(Exception):
    pass


def sha(b):
    return hashlib.sha256(b).hexdigest()


def load_earlier():
    out = {}
    for f in FILES:
        b = open(os.path.join(E, f), "rb").read()
        if sha(b) != EARLIER[f]:
            raise Refused(f"the earlier {f} has sha256 {sha(b)}, not {EARLIER[f]}")
        out[f] = json.loads(b)
    return out


def mark17(v, f):
    if not isinstance(v, list):
        raise Refused(f"{f} is not a list of marks")
    m = [x for x in v if isinstance(x, dict) and x.get("mark") == "PM-0001-17"]
    if len(m) != 1:
        raise Refused(f"{f}: PM-0001-17 appears {len(m)} times")
    return m[0]


def expected(earlier, f):
    """The earlier answers with exactly this lane's two additions, at their places."""
    e = copy.deepcopy(earlier)
    m = mark17(e, f)
    ids = [b.get("id") for b in m["bites"]]
    if ids.count(WAL) != 1 or ids.count(NEW_BITE["id"]) != 0:
        raise Refused(f"the earlier {f} does not hold whole-as-lost once and link-lost-overstated never")
    m["bites"].insert(ids.index(WAL) + 1, copy.deepcopy(NEW_BITE))
    if f == "verdict.json":
        if m["why"].count(WAL_WHY) != 1:
            raise Refused("the earlier verdict.json does not hold whole-as-lost's why line once")
        m["why"].insert(m["why"].index(WAL_WHY) + 1, NEW_WHY)
    else:
        m["why"].append(NEW_WHY)
    return e


def label(x, n):
    if isinstance(x, dict):
        for k in ("mark", "name", "id", "run"):
            if isinstance(x.get(k), str):
                return f"[{x[k]}]"
    return f"[{n}]"


def diffs(a, b, path):
    """Every difference between a (the new file) and b (the expected one), one line each."""
    show = lambda x: json.dumps(x, ensure_ascii=False)[:240]
    if type(a) is not type(b):
        return [f"{path}: new {show(a)} | expected {show(b)}"]
    out = []
    if isinstance(a, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a:
                out.append(f"{path}.{k}: missing in the new file (expected {show(b[k])})")
            elif k not in b:
                out.append(f"{path}.{k}: only in the new file ({show(a[k])})")
            else:
                out += diffs(a[k], b[k], f"{path}.{k}")
    elif isinstance(a, list):
        if all(isinstance(x, str) for x in a + b):
            if a != b:
                out.append(f"{path}: new {show(a)} | expected {show(b)}")
            return out
        if len(a) != len(b):
            out.append(f"{path}: {len(a)} entries in the new file, {len(b)} expected")
        for n in range(min(len(a), len(b))):
            out += diffs(a[n], b[n], f"{path}{label(a[n], n)}")
        for n in range(len(b), len(a)):
            out.append(f"{path}{label(a[n], n)}: only in the new file ({show(a[n])})")
        for n in range(len(a), len(b)):
            out.append(f"{path}{label(b[n], n)}: missing in the new file (expected {show(b[n])})")
    elif a != b:
        out.append(f"{path}: new {show(a)} | expected {show(b)}")
    return out


def set_parts(v):
    """setParts (c-u4r-runs.mjs:268-283) in Python: (key, verdict or status with each run's validity)."""
    lst = lambda x: x if isinstance(x, list) else []
    val = lambda r: "valid" if r.get("valid") is True else "invalid" if r.get("valid") is False else "valid missing"
    runs = lambda rs: ", ".join(f"{r.get('run')} {val(r)}" for r in lst(rs))
    out = []
    for m in lst(v):
        for p in lst(m.get("parts")):
            if lst(p.get("runs")):
                out.append((f"{m.get('mark')} {p.get('name')}", f"{p.get('verdict')} ({runs(p['runs'])})"))
        for b in lst(m.get("bites")):
            if lst(b.get("runs")):
                out.append((b.get("id"), f"{b.get('status')} ({runs(b['runs'])})"))
    return out


def compare(earlier, new, quiet=False):
    """(status, differences); raises Refused."""
    for f in FILES:
        m = mark17(new[f], f)
        n = [b.get("id") for b in m.get("bites", []) if isinstance(b, dict)].count(NEW_BITE["id"])
        if n > 1:
            raise Refused(f"the new {f}: the new bite entry appears {n} times")
    allds = []
    for f in FILES:
        if not quiet:
            print(f"{f}: each part and bite that holds a run, new beside earlier")
            old = dict(set_parts(earlier[f]))
            for key, text in set_parts(new[f]):
                print(f"  {key}: new {text} | earlier {old.pop(key, 'absent')}")
            for key, text in old.items():
                print(f"  {key}: new absent | earlier {text}")
        allds += diffs(new[f], expected(earlier[f], f), f)
    return ("SAME ANSWERS" if not allds else "DIFFERENT"), allds


def self_test():
    earlier = load_earlier()
    print(f"self-test on the earlier files ({E}; sha256 as the brief gives)")
    good = {f: expected(earlier[f], f) for f in FILES}
    results = []
    s, d = compare(earlier, good, quiet=True)
    results.append(s == "SAME ANSWERS" and not d)
    print(f"case 1, the earlier answers with the two additions: {s} ({len(d)} differences); wanted SAME ANSWERS: "
          f"{'as required' if results[-1] else 'NOT AS REQUIRED'}")
    b = copy.deepcopy(good)
    part = [p for p in mark17(b["verdict.json"], "verdict.json")["parts"] if p.get("name") == "L1 P0"][0]
    chk = part["runs"][0]["checks"][0]
    chk["value"] = "1" if chk["value"] != "1" else "2"
    s, d = compare(earlier, b, quiet=True)
    results.append(s == "DIFFERENT" and len(d) == 1 and d[0].endswith(".value: new \"" + chk["value"] + "\" | expected \"0\"")
                   and "[L1 P0]" in d[0])
    print(f"case 2, one -17 L1 P0 check value changed: {s}; {d}; wanted that one difference: "
          f"{'as required' if results[-1] else 'NOT AS REQUIRED'}")
    c = copy.deepcopy(good)
    m = mark17(c["verdict.json"], "verdict.json")
    i = [x.get("id") for x in m["bites"]].index(NEW_BITE["id"])
    m["bites"].insert(i + 1, copy.deepcopy(NEW_BITE))
    try:
        s, d = compare(earlier, c, quiet=True)
        results.append(False)
        print(f"case 3, the new bite entry twice: {s}; wanted a refusal: NOT AS REQUIRED")
    except Refused as e:
        results.append(True)
        print(f"case 3, the new bite entry twice: REFUSED ({e}); wanted a refusal: as required")
    dd = copy.deepcopy(good)
    mark17(dd["verdict-bites.json"], "verdict-bites.json")["why"].remove(NEW_WHY)
    s, d = compare(earlier, dd, quiet=True)
    results.append(s == "DIFFERENT" and len(d) == 1 and "verdict-bites.json[PM-0001-17].why" in d[0])
    print(f"case 4, verdict-bites.json without the new why line: {s}; {d}; wanted that difference: "
          f"{'as required' if results[-1] else 'NOT AS REQUIRED'}")
    ok = all(results)
    print(f"SELF-TEST {'PASS' if ok else 'FAIL'}: {results.count(True)} of 4 cases as required")
    sys.exit(0 if ok else 1)


def main():
    if len(sys.argv) != 2:
        print(__doc__)
        sys.exit(2)
    try:
        if sys.argv[1] == "--self-test":
            self_test()
        o = os.path.abspath(sys.argv[1])
        earlier = load_earlier()
        new = {}
        for f in FILES:
            b = open(os.path.join(o, f), "rb").read()
            print(f"new {os.path.join(o, f)}: sha256 {sha(b)}; earlier {os.path.join(E, f)}: sha256 {EARLIER[f]}")
            new[f] = json.loads(b)
        s, d = compare(earlier, new)
    except Refused as e:
        print(f"REFUSED: {e}")
        sys.exit(2)
    for line in d:
        print(f"DIFFERENCE: {line}")
    print(s)
    sys.exit(0 if s == "SAME ANSWERS" else 1)


main()
