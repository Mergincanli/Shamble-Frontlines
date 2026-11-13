"""Fill the two places T5 leaves in u4d2-report.md (advisor-u4d2-r2.md, T5: "Each `<…>` is a value the main session
fills from the named source (M5's log, the fix pass's record, the merged tree's h7 (ii) run), and the notes name the
log and line of each fill; no other word changes"), inside T5's section only, in order. Each place's text and order are
asserted; each value is read from M5's log (or the uptime record beside it) at the named line, never typed from
memory. Each bullet that holds a fill is rewrapped within 120 characters (white space only: checked). Prints the fills
with their sources.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4d2_t5_fill.py [--apply]"""
import re
import sys
import textwrap

REPORT = ".checkpoints/netcode-proof-test/u4d2-report.md"
LOG = ".gate/u4d2-merge/m5-counted.log"
UPTIME = ".gate/u4d2-merge/uptime-m5.txt"
log = open(LOG, encoding="utf-8").read().split("\n")
up = open(UPTIME, encoding="utf-8").read().split("\n")


def at(lines, n, pat):
    """The groups of `pat` in line n (1-based) of `lines`, which must match."""
    m = re.search(pat, lines[n - 1])
    assert m, (n, pat, lines[n - 1][:120])
    return m.groups()


secs = at(log, 170, r"^PASS  wasm +([\d.]+) s  native leg 1 against native leg 2: 27000 ticks compared, 0 mismatching")[0]
at(log, 175, r"^(check: all 11 steps pass)$")
uptime = int(at(up, 3, r"^(\d+)\.\d+ [\d.]+$")[0])
cmd = at(log, 177, r"^=== \d\d:\d\d:\d\d UTC run: (cargo test --locked --release -p npt-lab -- --ignored --exact "
                   r"probe::tests::h7_carry_66_the_probes_v1_datagram_hashes_are_u2s_for_p2_b1_and_p4_seed_1)"
                   r" --nocapture$")[0]
hx = r"([0-9a-f]{64})"
p2 = at(log, 199, r"^P2 seed 1: fate log [0-9a-f]{64}; v1 datagram fate log " + hx + "$")[0]
b1 = at(log, 200, r"^B1 seed 1: fate log [0-9a-f]{64}; v1 datagram fate log " + hx + "$")[0]
p4 = at(log, 201, r"^P4 seed 1: fate log [0-9a-f]{64}; v1 datagram fate log " + hx + "$")[0]
at(log, 204, r"^(test result: ok\. 1 passed; 0 failed; 0 ignored; 0 measured; \d+ filtered out)")

FILLS = [  # (the place's text, the value, its source)
    ("<the merged tree's seconds, with M5's label>",
     f"{secs} s, {round(uptime / 60)} minutes after a machine restart",
     f"m5-counted.log:170 (the wasm step's line); uptime-m5.txt:3 ({uptime} s)"),
    ("<the three v1 datagram hashes and their commands>",
     f"its three v1 datagram hashes on the merged tree, P2 {p2[:16]}…, B1 {b1[:16]}… and P4 {p4[:16]}…, each equal "
     f"to U2's by h7 (ii), `{cmd}`: 1 passed",
     "m5-counted.log:177 (the command), :199-201 (the three values), :204 (libtest's summary)"),
]
R = open(REPORT, encoding="utf-8").read().split("\n")
s = [i for i, l in enumerate(R) if l.startswith("## What U4d2's review settled")]
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
assert not re.search(r"<[^<>\s][^<>]*>", filled), "a place is left"


def bullets(ls):
    res, k = [], 0
    while k < len(ls):
        if ls[k].startswith("- "):
            m = k + 1
            while m < len(ls) and ls[m].startswith("  ") and not ls[m].startswith("  - "):
                m += 1
            res.append(" ".join(" ".join(x.strip() for x in ls[k:m]).split()))
            k = m
        else:
            k += 1
    return res


AS_COPIED = set(bullets(sec.split("\n")))
lines, res, i, touched = filled.split("\n"), [], 0, 0
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
    print(f"{k} {place} -> {v}  [{src}]")
print(f"{len(FILLS)} places filled; {touched} bullets rewrapped within 120")
if "--apply" in sys.argv:
    with open(REPORT, "w", encoding="utf-8") as fh:
        fh.write("\n".join(new))
    print("applied")
else:
    print("dry run: nothing written")
