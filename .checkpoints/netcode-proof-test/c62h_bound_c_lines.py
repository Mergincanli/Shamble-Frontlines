"""C62H-BOUND's (c) replacement in u4r-report.md (the build brief section 9; the advisor's P3 ruling,
advisor-c62h-bound-p3-full.json, its the_c_lines and condition P2). The two sub-lines "Against the profile's loss
(C62H-REVIEW; ...)" under -17 L1 P3 (:996) and -17 L1 P2 (:1007) give way to the ruling's exact filled texts, and its
one note line goes in after :989. The three texts are copied from the ruling's the_c_lines by this script, never
retyped. Before writing, every number in the two lines is checked against the files they name (exit 1 on any
difference). After: each new line once, the diff exactly the two replaced lines plus the one added line, B1's two
lines unchanged. Refuses to run twice (the old lines must be there).
Run from the repository root:  python3 -I .checkpoints/netcode-proof-test/c62h_bound_c_lines.py"""
import difflib
import hashlib
import json
import re
import sys

M = "/home/user/Shamble-Frontlines"
N = f"{M}/.checkpoints/netcode-proof-test"
R = f"{N}/u4r-report.md"
P3 = f"{M}/spikes/netcode-proof/results/u4r/d66e23cd78f2/P3"
P2 = f"{M}/spikes/netcode-proof/results/u4r/8ee453c22882/P2"
PHRASE = re.compile(r"; measured snapshot datagrams (\d+), lost by the link (\d+) \(in bursts (\d+)\); measured "
                    r"client-ticks filed lost: the room sent nothing (\d+), the link lost a datagram (\d+), neither (\d+)")
A_NAME = "snap_lost_unexplained_count"


def stop(msg):
    print(f"c62h_bound_c_lines: STOP, nothing written: {msg}")
    sys.exit(1)


def between(text, start, end):
    i = text.index(start) + len(start)
    return text[i:text.index(end, i)]


c = json.load(open(f"{N}/advisor-c62h-bound-p3-full.json", encoding="utf-8"))["the_c_lines"]
P3_NEW = between(c, 'P3 (replacing :996): "', '"\n\nP2 (replacing :1007)')
P2_NEW = between(c, 'P2 (replacing :1007): "', '"\n\nNote line')
NOTE = between(c, 'Note line, added after :989 by the same script (ADV-CBP3-4): "', '"\n\nBefore writing')
for t in (P3_NEW, P2_NEW):
    if not t.startswith("  - Bounded by C62H-BOUND (his words of 2026-11-15, 16:35 and 20:50): "):
        stop(f"an extracted line does not start as the ruling gives it: {t[:80]!r}")
if not (NOTE.startswith("_") and NOTE.endswith("_")):
    stop("the note line is not one italic line")


def checks_of(folder, part):
    v = json.load(open(f"{folder}/verdict.json", encoding="utf-8"))
    m17 = [m for m in v if m.get("mark") == "PM-0001-17"][0]
    p = [x for x in m17["parts"] if x.get("name") == part][0]
    out = []
    for r in p["runs"]:
        cs = {x["name"]: x for x in r["checks"]}
        a = [x for n, x in cs.items() if "(0 unexplained)" in n]
        b = [x for n, x in cs.items() if "not overstated" in n]
        out.append((a[0]["value"], b[0]["value"]))
    return out


def tally_of(folder):
    rows = []
    for l in open(f"{folder}/runs.txt", encoding="utf-8").read().split("\n"):
        if l.startswith("l1-"):
            f = PHRASE.findall(l)
            inc = re.search(r"; measured incomplete (\d+);", l)
            res = re.search(r"; overflow resyncs (\d+);", l)
            if len(f) != 1 or not inc or not res:
                stop(f"a summary line without one phrase, incomplete and resyncs: {l[:60]}")
            s, lo, b, e1, e2, n = map(int, f[0])
            if e1 + e2 + n != int(inc.group(1)):
                stop(f"E1 + E2 + N is not the measured incomplete count on {l[:20]}")
            rows.append((e1, e2, n, int(res.group(1))))
    if len(rows) != 3:
        stop(f"{folder}/runs.txt has {len(rows)} measured lines, not 3")
    return rows


fmt = lambda n: f"{n:,}"
for folder, part, line, flags in ((P3, "L1 P3", P3_NEW, True), (P2, "L1 P2", P2_NEW, False)):
    ab = checks_of(folder, part)
    if ab != [("0", "0")] * 3:
        stop(f"{part}: A and B are {ab}, not 0 / 0 / 0")
    if "snap_lost_unexplained_count 0 / 0 / 0 and snap_whole_despite_link_loss_count 0 / 0 / 0" not in line:
        stop(f"{part}: the line does not carry the 0 / 0 / 0 figures")
    t = tally_of(folder)
    want = ", ".join(f"{e1} / {fmt(e2)} / {n}" for e1, e2, n, _ in t[:2]) + f" and {t[2][0]} / {fmt(t[2][1])} / {t[2][2]}"
    if f"the lab's tally E1 / E2 / N per run: {want}," not in line:
        stop(f"{part}: the files give the tally {want!r}, which the line does not carry")
    if flags and f"carry 58's {t[0][3]} and {t[2][3]} overflow resyncs" not in line:
        stop(f"{part}: the files give overflow resyncs {t[0][3]} and {t[2][3]}, which the line does not carry")
    if f"(results/u4r/{folder.split('/u4r/')[1]}/verdict.json)" not in line:
        stop(f"{part}: the line does not name its verdict.json")
vb = json.load(open(f"{P2}/verdict-bites.json", encoding="utf-8"))
b48 = []
for m in vb:
    for b in m.get("bites", []):
        if b.get("id") in ("PM-0001-17/whole-as-lost", "PM-0001-17/link-lost-overstated") and b.get("runs"):
            own = "(0 unexplained)" if b["id"].endswith("whole-as-lost") else "not overstated"
            b48 += [x["value"] for x in b["runs"][0]["checks"] if own in x["name"]]
if b48 != ["48", "48"]:
    stop(f"the planted bugs read {b48}, not 48 and 48")
for line in (P3_NEW, P2_NEW):
    if "the planted bugs 48 and 48 (P2's verdict-bites.json, results/u4r/8ee453c22882/P2/)" not in line:
        stop("a line does not carry the planted bugs 48 and 48 with P2's path")
print("the numbers in both lines equal the files they name (A, B, the tallies, the resyncs, the planted bugs)")

raw = open(R, "rb").read()
print(f"u4r-report.md before: sha256 {hashlib.sha256(raw).hexdigest()}")
lines = raw.decode("utf-8").split("\n")
old3 = [i for i, l in enumerate(lines) if l.startswith("  - Against the profile's loss (C62H-REVIEW;") and "P3: 43,295" in l]
old2 = [i for i, l in enumerate(lines) if l.startswith("  - Against the profile's loss (C62H-REVIEW;") and "P2: 17,260" in l]
if len(old3) != 1 or len(old2) != 1 or (old3[0] + 1, old2[0] + 1) != (996, 1007):
    stop(f"the old lines are at {[i + 1 for i in old3]} and {[i + 1 for i in old2]}, not once each at 996 and 1007")
if lines.count(lines[old3[0]]) != 1 or lines.count(lines[old2[0]]) != 1:
    stop("an old line is not there exactly once as a whole line")
if not lines[988].endswith('0"._'):
    stop("line 989 is not the end of the italic note")
B1 = ["- -17 L1 B1 stays with U4g's judged B1 runs.",
      "- **-17 L1 B1** stays with U4g's judged B1 runs (advisor-brawl1-sets-p0-full.md, The commit and the close, 2)."]
if [lines.count(b) for b in B1] != [1, 1]:
    stop("B1's two lines are not there once each")
new = list(lines)
new[old3[0]] = P3_NEW
new[old2[0]] = P2_NEW
new.insert(989, NOTE)
for t in (P3_NEW, P2_NEW, NOTE):
    if new.count(t) != 1:
        stop("a new line is not there exactly once")
if [new.count(b) for b in B1] != [1, 1]:
    stop("B1's lines changed")
d = [x for x in difflib.unified_diff(lines, new, lineterm="", n=0) if x[:1] in "+-" and x[:3] not in ("+++", "---")]
if sorted(x[0] for x in d) != ["+", "+", "+", "-", "-"]:
    stop(f"the diff is not two replaced lines and one added line: {len(d)} changed lines")
out = "\n".join(new).encode("utf-8")
open(R, "wb").write(out)
print(f"u4r-report.md after: sha256 {hashlib.sha256(out).hexdigest()} (the old :996 and :1007 replaced, now :997 and "
      f":1008 below the note added at :990)")
