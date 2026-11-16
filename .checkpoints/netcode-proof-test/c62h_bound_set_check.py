"""C62H-BOUND's set check (the build brief c62h-bound-brief.md sections 8.3 and 8.4, sha256 7936074c...; TE-CB-7 with
the advisor's amendments; the stop rules of section 10). Read only. After each set it checks every expected line of
section 8.4 on <results dir> = results/u4r/<c12>/<P>/ (runs.txt, verdict.json and, for P2, verdict-bites.json), PASS
or FAIL each, and marks each line that is one of Mustafa's stop rules ("MUSTAFA") apart from the advisor's ones; it
also counts the lines that name the new phrase or the new checks, and how many of them FAIL.
Run from the repository root:
  python3 -I .checkpoints/netcode-proof-test/c62h_bound_set_check.py P2|P3 <results dir>
  python3 -I .checkpoints/netcode-proof-test/c62h_bound_set_check.py --self-test <P2 results dir>
Shown to fail first (TE-CBB-1): once on results/u4r/fc411116c1f1/P2 as P2 (every line naming the new phrase or the new
checks must FAIL), and --self-test on P2's new verdict-bites.json with one gap planted in memory on the whole-as-lost
run (the planted-bug rule must FAIL). Exit 0 when every line passes, 1 otherwise."""
import copy
import hashlib
import json
import os
import re
import sys

M = "/home/user/Shamble-Frontlines"
N = f"{M}/.checkpoints/netcode-proof-test"
TEXTS_SHA = "3f21c9b7e5d5116db7cae7c0d8d9a4011d2ab9797c75b907f00554f0996844eb"
MARKS_AFTER = "c1935a5b996fb0aaff82b0b3c1c604255e1534f6ca7e8d2e55a9f472cd3b22a9"
OLD = {"P2": f"{M}/spikes/netcode-proof/results/u4r/fc411116c1f1/P2/runs.txt",
       "P3": f"{M}/spikes/netcode-proof/results/u4r/fd4612050cd6/P3/runs.txt"}
BLOCK_RE = r"^<!-- BEGIN (\S+) -->\n```\n(.*?)\n```\n<!-- END \1 -->$"  # c62h_bound_freeze.py's regex
PHRASE = re.compile(r"; measured snapshot datagrams (\d+), lost by the link (\d+) \(in bursts (\d+)\); measured "
                    r"client-ticks filed lost: the room sent nothing (\d+), the link lost a datagram (\d+), neither (\d+)")
WALL = re.compile(r" \(wall [0-9.]+ s\)")
WAL, LLO = "PM-0001-17/whole-as-lost", "PM-0001-17/link-lost-overstated"
rows = []


def line(name, ok, detail="", who="advisor", new=False):
    rows.append((bool(ok), who, new))
    tag = "MUSTAFA" if who == "mustafa" else "advisor"
    print(f"{'PASS' if ok else 'FAIL'}  [{tag}{', names the new' if new else ''}] {name}" + (f": {detail}" if detail else ""))


def check_names():
    v3raw = open(f"{N}/c62h-bound-texts-v3.md", "rb").read()
    if hashlib.sha256(v3raw).hexdigest() != TEXTS_SHA:
        sys.exit("texts v3's sha256 is not the frozen one")
    b = {m.group(1): m.group(2) for m in re.finditer(BLOCK_RE, v3raw.decode("utf-8"), re.M | re.S)}
    two = json.loads("[" + b["check-objects"] + "]")
    a = [c["name"] for c in two if c["figure"] == "snap_lost_unexplained_count"][0]
    w = [c["name"] for c in two if c["figure"] == "snap_whole_despite_link_loss_count"][0]
    marks = json.load(open(f"{M}/spikes/netcode-proof/marks/marks.json", encoding="utf-8"))["marks"]
    m17 = [x for x in marks if x["id"] == "PM-0001-17"][0]
    mm = [c["name"] for c in m17["checks"] if c.get("figure") == "decoded_hash_mismatch_count"]
    if len(mm) != 1:
        sys.exit("marks.json's PM-0001-17 has no single decoded_hash_mismatch_count check")
    return a, w, mm[0]


def set_parts(v):
    """setParts (c-u4r-runs.mjs:268-283): key -> 'verdict (run validity, ...)'."""
    lst = lambda x: x if isinstance(x, list) else []
    val = lambda r: "valid" if r.get("valid") is True else "invalid" if r.get("valid") is False else "valid missing"
    runs = lambda rs: ", ".join(f"{r.get('run')} {val(r)}" for r in lst(rs))
    out = {}
    for m in lst(v):
        for p in lst(m.get("parts")):
            if lst(p.get("runs")):
                out[f"{m.get('mark')} {p.get('name')}"] = f"{p.get('verdict')} ({runs(p['runs'])})"
        for b in lst(m.get("bites")):
            if lst(b.get("runs")):
                out[b.get("id")] = f"{b.get('status')} ({runs(b['runs'])})"
    return out


def run_checks(run):
    return {c.get("name"): c for c in run.get("checks", [])}


def reading(run, name):
    c = run_checks(run).get(name)
    return (c.get("value"), c.get("outcome")) if c else ("absent", "absent")


def summary_lines(path):
    text = open(path, encoding="utf-8").read().split("\n")
    i = [n for n, l in enumerate(text) if l.startswith("# one line per run")]
    out = {}
    for l in (text[i[0] + 1:] if i else []):
        if l.strip():
            out[l.split(" ", 1)[0]] = l
    return text, out


def strip(l):
    found = PHRASE.findall(l)
    ph = tuple(int(x) for x in found[0]) if len(found) == 1 else None
    return WALL.sub("", PHRASE.sub("", l, count=1), count=1), ph, len(found)


def field(fs, rx):
    for f in fs:
        m = re.fullmatch(rx, f)
        if m:
            return int(m.group(1))
    return None


def bite_rule(bites, bid, own, other, mm, label):
    """Mustafa's stop rule 3 as the advisor reads it (a conjunction): valid; own figure exactly 48 (fail); the other
    figure and the mismatches 0 (pass); gaps empty. Returns ok."""
    b = [x for x in bites if x.get("id") == bid]
    runs = b[0].get("runs", []) if len(b) == 1 else []
    if len(runs) != 1:
        line(f"{label}: one run", False, f"{len(runs)} runs", "mustafa", True)
        return False
    r = runs[0]
    rest = sorted((n, c.get("outcome")) for n, c in run_checks(r).items() if n != own)
    got = (r.get("valid"), reading(r, own), reading(r, other), reading(r, mm), r.get("gaps", []))
    want = (True, ("48", "fail"), ("0", "pass"), ("0", "pass"), [])
    ok = got == want and all(o == "pass" for _, o in rest)  # TE-CBH-1: "with every other check passing"
    line(f"{label}: valid; its own figure 48 fail; every other check pass (the other figure and the mismatches 0); "
         f"gaps []", ok, f"got valid {got[0]}, own {got[1]}, other {got[2]}, mismatches {got[3]}, gaps {got[4]}; every "
         f"other check {rest}", "mustafa", True)
    return ok


def bite_lines(lines_new, s1, judge_a):
    """The bite runs' summary lines against l1-p2-s1's (TE-CBB-3), as rows (name, ok, detail, who, new)."""
    out, s1l = [], lines_new.get("l1-p2-s1")
    for run in ("bite-whole-as-lost", "bite-link-lost-overstated"):
        l = lines_new.get(run)
        if l is None or s1 is None or s1l is None:
            out.append((f"summary {run}: compared with l1-p2-s1", False, "a line or s1's phrase is missing", "advisor",
                        True))
            continue
        st, p, nf = strip(l)
        f1 = strip(s1l)[0].split(" ", 1)[1].split("; ")
        fb = st.split(" ", 1)[1].split("; ")
        diff = [n for n in range(max(len(f1), len(fb))) if (f1[n:n + 1] or [None]) != (fb[n:n + 1] or [None])]
        if run == "bite-whole-as-lost":
            h1, hb = field(f1, r"client hashes (\d+) \(sentinels 0\)"), field(fb, r"client hashes (\d+) \(sentinels 0\)")
            i1, ib = field(f1, r"measured incomplete (\d+)"), field(fb, r"measured incomplete (\d+)")
            ok = (len(f1) == len(fb) and len(diff) == 2 and None not in (h1, hb, i1, ib) and hb == h1 - 48
                  and ib == i1 + 48 and all(fb[n].startswith(("client hashes", "measured incomplete")) for n in diff))
            out.append((f"summary {run}: differs from s1 in exactly client hashes (s1's - 48) and measured incomplete "
                        f"(s1's + 48)", ok, f"differing fields {sorted(fb[n] for n in diff if n < len(fb))}", "advisor",
                        True))
            out.append((f"summary {run}: its new phrase's S, L, B, E1 and E2 equal s1's", p is not None and nf == 1
                        and p[:5] == s1[:5], f"{p} vs s1 {s1}", "advisor", True))
            out.append((f"summary {run}: N is 48", p is not None and p[5] == 48, f"{p[5] if p else None}", "mustafa",
                        True))
        else:
            out.append((f"summary {run}: differs from s1 in no field", len(f1) == len(fb) and not diff,
                        f"differing fields {[fb[n] for n in diff if n < len(fb)]}", "advisor", True))
            out.append((f"summary {run}: its new phrase's S, L, B, E1, E2 and N (0) equal s1's", p is not None
                        and nf == 1 and p == s1 and p[5] == 0, f"{p} vs s1 {s1}", "advisor", True))
        if p is not None:
            out.append((f"summary {run}: the lab's N equals the judge's figure A", str(p[5]) == judge_a.get(run),
                        f"N {p[5]}, A {judge_a.get(run)}", "mustafa", True))
    return out


def main():
    args = sys.argv[1:]
    if len(args) != 2 or args[0] not in ("P2", "P3", "--self-test"):
        print(__doc__)
        sys.exit(2)
    a, w, mm = check_names()
    d = os.path.abspath(args[1])
    if args[0] == "--self-test":
        vb = json.load(open(os.path.join(d, "verdict-bites.json"), encoding="utf-8"))
        bites = [b for m in vb for b in m.get("bites", [])]
        print(f"self-test on {d}: the planted-bug rule and the bite lines, as the files are, then each planted")
        as_is = bite_rule(bites, WAL, a, w, mm, "as the file is: whole-as-lost")
        planted = copy.deepcopy(bites)
        [b for b in planted if b.get("id") == WAL][0]["runs"][0]["gaps"] = ["a planted gap (the set check's self-test)"]
        p = bite_rule(planted, WAL, a, w, mm, "planted: whole-as-lost with one gap")
        _, ln = summary_lines(os.path.join(d, "runs.txt"))
        s1 = strip(ln.get("l1-p2-s1", ""))[1]
        fields_of = lambda rs, run: [t for t in rs if "differs from s1" in t[0] and run in t[0]]

        def show(tag, rs):
            for t in rs:
                print(f"{'PASS' if t[1] else 'FAIL'}  ({tag}) {t[0]}: {t[2]}")
            return [t[1] for t in rs]

        def bump(l):  # TE-CBH-2: one more field changed (late N becomes N + 1)
            new, k = re.subn(r"; late (\d+);", lambda m: f"; late {int(m.group(1)) + 1};", l, count=1)
            return new if k == 1 else None

        as_lines, planted_lines = {}, {}
        for run in ("bite-whole-as-lost", "bite-link-lost-overstated"):
            as_lines[run] = show("as the file is", fields_of(bite_lines(ln, s1, {}), run))
            b = bump(ln.get(run, ""))
            if b is not None:
                planted_lines[run] = show(f"planted: {run}'s late field changed",
                                          fields_of(bite_lines(dict(ln, **{run: b}), s1, {}), run))
        ok = (as_is and not p and all(v == [True] for v in as_lines.values()) and len(planted_lines) == 2
              and all(v == [False] for v in planted_lines.values()))
        print(f"SELF-TEST {'PASS' if ok else 'FAIL'}: the planted-bug rule passes as the file is ({as_is}) and fails with "
              f"the planted gap ({not p}); each bite line's comparison passes as the file is ({as_lines}) and fails with "
              f"one more field changed ({planted_lines})")
        sys.exit(0 if ok else 1)
    P = args[0]
    seeds = [f"l1-{P.lower()}-s{k}" for k in (1, 2, 3)]
    runs_txt, lines_new = summary_lines(os.path.join(d, "runs.txt"))
    _, lines_old = summary_lines(OLD[P])
    print(f"c62h_bound_set_check {P} on {d}; old lines from {OLD[P]}")
    ms = [re.match(r"^marks\.json sha256 ([0-9a-f]{64})", l) for l in runs_txt]
    ms = [x.group(1) for x in ms if x]
    line("runs.txt's marks.json sha256 equals the built check's after", ms == [MARKS_AFTER], f"{ms}", new=True)
    v = json.load(open(os.path.join(d, "verdict.json"), encoding="utf-8"))
    sp = set_parts(v)
    runs_s = ", ".join(f"{s} valid" for s in seeds)
    for key in ("PM-0001-2 SS runs", f"PM-0001-17 L1 {P}", f"PM-0001-57 L1 {P}"):
        line(f"verdict.json: {key}: PASS ({runs_s})", sp.get(key) == f"PASS ({runs_s})", sp.get(key, "absent"))
    m17 = [x for x in v if x.get("mark") == "PM-0001-17"][0]
    part = [p for p in m17["parts"] if p.get("name") == f"L1 {P}"][0]
    judge_a = {}
    for r in part.get("runs", []):
        ra, rw, rm = reading(r, a), reading(r, w), reading(r, mm)
        judge_a[r.get("run")] = ra[0]
        line(f"verdict.json {r.get('run')}: snap_lost_unexplained_count 0 pass", ra == ("0", "pass"), f"{ra}",
             "mustafa", True)
        line(f"verdict.json {r.get('run')}: snap_whole_despite_link_loss_count 0 pass", rw == ("0", "pass"), f"{rw}",
             new=True)
        line(f"verdict.json {r.get('run')}: decoded_hash_mismatch_count 0 pass", rm == ("0", "pass"), f"{rm}")
        line(f"verdict.json {r.get('run')}: gaps empty", r.get("gaps", []) == [], f"{r.get('gaps', [])}")
    vbp = os.path.join(d, "verdict-bites.json")
    if P == "P2":
        vb = json.load(open(vbp, encoding="utf-8")) if os.path.exists(vbp) else []
        spb = set_parts(vb)
        for bid, run in ((WAL, "bite-whole-as-lost"), (LLO, "bite-link-lost-overstated")):
            line(f"verdict-bites.json: {bid}: shown ({run} valid)", spb.get(bid) == f"shown ({run} valid)",
                 spb.get(bid, "absent"), new=True)
        line("verdict-bites.json: no other bite holds a run (-57's bites are not run on P2)",
             sorted(spb) == sorted([WAL, LLO]), f"{sorted(spb)}", new=True)
        bites = [b for m in vb for b in m.get("bites", [])]
        bite_rule(bites, WAL, a, w, mm, "verdict-bites.json whole-as-lost")
        bite_rule(bites, LLO, w, a, mm, "verdict-bites.json link-lost-overstated")
        for b in bites:
            if b.get("id") in (WAL, LLO) and len(b.get("runs", [])) == 1:
                judge_a[b["runs"][0].get("run")] = reading(b["runs"][0], a)[0]
    else:
        line("P3 has no verdict-bites.json (no bites on P3)", not os.path.exists(vbp))
    ph = {}
    for k, s in enumerate(seeds):
        l, o = lines_new.get(s), lines_old.get(s)
        if l is None or o is None:
            line(f"summary {s}: the line is in runs.txt and in the old runs.txt", False, f"new {l is not None}, old "
                 f"{o is not None}")
            continue
        st, p, nf = strip(l)
        so = WALL.sub("", o, count=1)
        line(f"summary {s}: without the new phrase and the wall time, equals the old line whole", st == so)
        line(f"summary {s}: the new phrase once", nf == 1, f"{nf} times", new=True)
        if p is None:
            for t in ("E1 + E2 + N = measured incomplete", "N 0"):
                line(f"summary {s}: {t}", False, "no phrase", "mustafa" if t == "N 0" else "advisor", True)
            line(f"summary {s}: the lab's N equals the judge's figure A", False, "no phrase", "mustafa", True)
            if P == "P2":
                line(f"summary {s}: B 0", False, "no phrase", new=True)
            continue
        ph[s] = p
        S, L, B, E1, E2, NN = p
        fs = st.split(" ", 1)[1].split("; ")
        inc = field(fs, r"measured incomplete (\d+)")
        print(f"      {s}: S {S}, L {L}, B {B}; E1 {E1}, E2 {E2}, N {NN}; measured incomplete {inc}")
        line(f"summary {s}: E1 + E2 + N = measured incomplete", inc is not None and E1 + E2 + NN == inc,
             f"{E1} + {E2} + {NN} vs {inc}", new=True)
        line(f"summary {s}: N 0", NN == 0, f"{NN}", "mustafa", True)
        line(f"summary {s}: the lab's N equals the judge's figure A", str(NN) == judge_a.get(s), f"N {NN}, A "
             f"{judge_a.get(s)}", "mustafa", True)
        if P == "P2":
            line(f"summary {s}: B 0", B == 0, f"{B}", new=True)
        if P == "P3" and k == 0:
            print(f"      note: {s}'s E1 is {E1}; " + ("above 0: condition 2 is shown on a real run" if E1 > 0 else
                  "not above 0: the report says condition 2 is shown by the fixtures only (not a stop)"))
        if "(overflow resyncs: to the advisor before it counts, carry 58)" in l:
            print(f"      note: {s} carries the carry-58 flag: to the advisor before it counts")
    if P == "P2":
        for t in bite_lines(lines_new, ph.get(seeds[0]), judge_a):
            line(*t)
    nf = [r for r in rows if not r[0]]
    named = [r for r in rows if r[2]]
    print(f"c62h_bound_set_check {P}: {len(rows) - len(nf)} PASS, {len(nf)} FAIL "
          f"({sum(1 for r in nf if r[1] == 'mustafa')} on Mustafa's stop rules); lines naming the new phrase or the "
          f"new checks: {len(named)}, of which {sum(1 for r in named if not r[0])} FAIL")
    sys.exit(1 if nf else 0)


main()
