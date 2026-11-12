"""Fill the places T5 leaves in u4e-report.md (advisor-u4e-r2.md, T5: "Each `<…>` is a number the main session fills
from the merged tree's M5 lines, and the notes name the log and line of each fill; no other word changes"), inside
T5's section only, in order. Each place's text and order are asserted; each value is read from M5's log at the named
line (or, for the ring slots, computed from that line's ticks), never typed from memory. Each bullet that holds a fill
is rewrapped within 120 characters (white space only: checked). Prints the fills with their sources.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4e_t5_fill.py [--apply]"""
import re
import sys
import textwrap

REPORT = ".checkpoints/netcode-proof-test/u4e-report.md"
LOG = ".gate/u4e-merge/m5-counted.log"
PUB = ".gate/u4e-merge/pub-items.log"
log = open(LOG, encoding="utf-8").read().split("\n")


def at(n, pat):
    """The groups of `pat` in M5's log line n (1-based), which must match."""
    m = re.search(pat, log[n - 1])
    assert m, (n, pat, log[n - 1][:120])
    return m.groups()


o17 = at(149, r"^PM-0001-17/tick16: .*?(\d+) bad; client 0's orphans \[([0-9, ]+)\]; at or past 65,536 \[\]")
o58 = at(152, r"^PM-0001-58/tick16: .*?(\d+) bad; client 0's orphans \[([0-9, ]+)\]; at or past 65,536 \[\]")
n17, n58 = len(o17[1].split(",")), len(o58[1].split(","))
assert n17 == n58, (n17, n58)
clean = at(150, r"^tick16 clean: .*?; (\d+) pairs compared, 0 bad; client keys below 65,520 \[\]")
a = at(179, r"^e5a AfterSend: .*?F (\d+), B Some\((\d+)\) at age Some\((\d+)\), evicted true")
b = at(186, r"^e5a BeforeSend: .*?F (\d+), B Some\((\d+)\) at age Some\((\d+)\), evicted true")
assert a[1] == b[1], (a, b)  # the same B in both alignments
RING = 32  # npt_sim::ACK_RING_TICKS: the ring the tick16 lines show (65,504..65,535)
ss = at(207, r"carry 56 \(SS\): own-state section ([\d.]+) B mean, (\d+) B p99, (\d+) B max per snapshot .*?OwnState "
             r"record ([\d.]+) B mean, changed in ([\d.]+) of snapshots")
es = at(205, r"^carry 56 \(ES\): .*?own-state section ([\d.]+) B mean, (\d+) B p99, (\d+) B max per snapshot .*?OwnState "
             r"record ([\d.]+) B mean, changed in ([\d.]+) of snapshots")
e7 = at(157, r"^e7: (.*)$")[0]
e2 = at(154, r"^e2: (\d+) split ticks, (\d+) compared equal$")
g1 = at(187, r"^2\.5 AfterSend: .*?; (\d+) pairs, 0 differ; (\d+) events resent$")
g2 = at(188, r"^2\.5 BeforeSend: .*?; (\d+) pairs, 0 differ; (\d+) events resent$")
assert g1[0] == g2[0], (g1, g2)
pub = open(PUB, encoding="utf-8").read()
counts = re.findall(r"npt-(room|server|client): (\d+) added pub names; (\d+) of them fields .*?nor §4\.7's own: none",
                    pub)
assert len(counts) == 3, counts
c = {k: (v, f) for k, v, f in counts}


def comma(s):
    return f"{int(s):,}"


LIST = (f"re-derived from the merged tree for T6 (u4e_pub_items.py; .gate/u4e-merge/pub-items.log): every `pub` item "
        f"U4e adds is one of plan.md §4.7's own (plan.md:596-600) or in T6's §4.1 rows, each with its reason in its "
        f"row's last column: npt-room {c['room'][0]} names ({c['room'][1]} of them `ClientCounters`' fields, named as a "
        f"group), npt-server {c['server'][0]} ({c['server'][1]} `Counters`' fields), npt-client {c['client'][0]} "
        f"({c['client'][1]} `StepOut`'s fields)")
FILLS = [  # (the place's text, the value, its source)
    ("<n>", str(n17), "m5-counted.log:149 and :152 (each bite: client 0's orphans, " + str(n17) + ")"),
    ("<n>", clean[0], "m5-counted.log:150"),
    ("<363>", a[0], "m5-counted.log:179"), ("<329>", a[1], "m5-counted.log:179"), ("<34>", a[2], "m5-counted.log:179"),
    ("<362>", b[0], "m5-counted.log:186"), ("<33>", b[2], "m5-counted.log:186"),
    ("<9>", str(int(a[1]) % RING), f"m5-counted.log:179 (B {a[1]} mod ACK_RING_TICKS {RING})"),
    ("<11>", str(int(a[0]) % RING), f"m5-counted.log:179 (F {a[0]} mod {RING})"),
    ("<10>", str(int(b[0]) % RING), f"m5-counted.log:186 (F {b[0]} mod {RING})"),
    ("<mean>", ss[0], "m5-counted.log:207"), ("<p99>", ss[1], "m5-counted.log:207"),
    ("<max>", ss[2], "m5-counted.log:207"), ("<mean>", ss[3], "m5-counted.log:207"),
    ("<share>", ss[4], "m5-counted.log:207"),
    ("<mean>", es[0], "m5-counted.log:205"), ("<p99>", es[1], "m5-counted.log:205"),
    ("<max>", es[2], "m5-counted.log:205"), ("<mean>", es[3], "m5-counted.log:205"),
    ("<share>", es[4], "m5-counted.log:205"),
    ("<its print line>", e7, "m5-counted.log:157 (equal to .gate/u4e/fx-t5print.log's)"),
    ("<n>", comma(e2[0]), "m5-counted.log:154"), ("<n>", comma(e2[1]), "m5-counted.log:154"),
    ("<n>", g1[0], "m5-counted.log:187 and :188"), ("<n>", g1[1], "m5-counted.log:187"),
    ("<n>", g2[1], "m5-counted.log:188"),
    ("<the list re-derived from the merged tree for T6, each with its reason>", LIST, ".gate/u4e-merge/pub-items.log"),
]
R = open(REPORT, encoding="utf-8").read().split("\n")
s = [i for i, l in enumerate(R) if l.startswith("## What U4e's review settled")]
e = [i for i, l in enumerate(R) if l == "## Checks on the merged tree"]
assert len(s) == 1 and len(e) == 1 and s[0] < e[0], (s, e)
sec = "\n".join(R[s[0]:e[0]])
places = list(re.finditer(r"<[^<>]+>", sec))
assert [" ".join(p.group(0).split()) for p in places] == [f[0] for f in FILLS], \
    [" ".join(p.group(0).split()) for p in places]
out, last = [], 0
for p, (_, v, _) in zip(places, FILLS):
    out.append(sec[last:p.start()])
    out.append(v)
    last = p.end()
out.append(sec[last:])
filled = "".join(out)
assert not re.search(r"<[^<>\s][^<>]*>", filled.replace("<br>", "")), "a place is left"
# rewrap each bullet that holds a fill (a bullet: a line "- ..." and its "  " continuation lines); a bullet whose text
# is one of the section's bullets as copied keeps its lines


def bullets(ls):
    out, k = [], 0
    while k < len(ls):
        if ls[k].startswith("- "):
            m = k + 1
            while m < len(ls) and ls[m].startswith("  ") and not ls[m].startswith("  - "):
                m += 1
            out.append(" ".join(" ".join(x.strip() for x in ls[k:m]).split()))
            k = m
        else:
            k += 1
    return out


AS_COPIED = set(bullets(sec.split("\n")))
lines, res, i = filled.split("\n"), [], 0
touched = 0
while i < len(lines):
    if lines[i].startswith("- "):
        j = i + 1
        while j < len(lines) and lines[j].startswith("  ") and not lines[j].startswith("  - "):
            j += 1
        block = lines[i:j]
        if any(len(x) > 120 for x in block) or " ".join(" ".join(x.strip() for x in block).split()) not in AS_COPIED:
            text = " ".join(x.strip() for x in block)
            w = textwrap.wrap(text, width=120, initial_indent="", subsequent_indent="  ", break_long_words=False,
                              break_on_hyphens=False)
            assert " ".join(" ".join(w).split()) == " ".join(text.split())
            res += w
            touched += 1
        else:
            res += block
        i = j
    else:
        res.append(lines[i])
        i += 1
assert " ".join("\n".join(res).split()) == " ".join(filled.split()), "white space only"
assert all(len(x) <= 120 for x in res), [x for x in res if len(x) > 120][:2]
new = R[:s[0]] + res + R[e[0]:]
for k, (place, v, src) in enumerate(FILLS, 1):
    print(f"{k:2d} {place[:40]:42s} -> {v[:70]}{'…' if len(v) > 70 else ''}  [{src}]")
print(f"{len(FILLS)} places filled; {touched} bullets rewrapped within 120")
if "--apply" in sys.argv:
    with open(REPORT, "w", encoding="utf-8") as fh:
        fh.write("\n".join(new))
    print("applied")
else:
    print("dry run: nothing written")
