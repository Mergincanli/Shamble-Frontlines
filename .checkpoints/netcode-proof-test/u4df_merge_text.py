"""Apply the merge commit's text of the advisor's U4d-F ruling (advisor-u4df.md: T1 and T2; R5's M8) by script.
Every block is read from the ruling's own code blocks, never retyped; each anchor is checked unique; each new text is
found in place afterwards; no inserted line is over 120 characters; carries 67-72 must not exist before.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4df_merge_text.py [--apply]
Without --apply it changes nothing and prints what it would do. With --apply, u4df-report.md must already exist and
hold the line ANCHOR once (the main session writes the report's other sections first, with the Write tool)."""
import os
import sys

N = ".checkpoints/netcode-proof-test"
APPLY = "--apply" in sys.argv
ANCHOR = "<!-- T1 (1): the advisor's measurement section (advisor-u4df.md, T1), copied here by u4df_merge_text.py -->"


def read(name):
    with open(f"{N}/{name}", encoding="utf-8") as fh:
        return fh.read().split("\n")


RULING = read("advisor-u4df.md")


def blocks_under(heading):
    """The fenced blocks (fences excluded) of the ruling's section whose heading line starts with `heading`, up to
    the next heading outside a fence."""
    starts = [i for i, l in enumerate(RULING) if l.startswith(heading)]
    assert len(starts) == 1, (heading, starts)
    out, cur, fence = [], None, False
    for line in RULING[starts[0] + 1:]:
        if line.startswith("```"):
            if fence:
                out.append(cur)
                cur = None
            else:
                cur = []
            fence = not fence
            continue
        if fence:
            cur.append(line)
        elif line.startswith("#"):
            break
    assert not fence, heading
    return out


T1 = blocks_under("### T1.")
T2 = blocks_under("### T2.")
assert len(T1) == 2 and len(T2) == 5, (len(T1), len(T2))
assert T1[0][0].startswith("## The estimate scene after U4d-F"), T1[0][0]
assert T1[1][0].startswith("- **Fixed by U4d-F**"), T1[1][0]
assert T2[0][0].startswith("script, run by run (ADR 0001:1480). Since U4d-F"), T2[0][0]
assert T2[1][0].startswith("68. **NF-L1, "), T2[1][0]
assert T2[2][0].startswith("69. **NF-L1's U4r half"), T2[2][0]
assert T2[3][0].startswith("70. **NF-L2, "), T2[3][0]
assert T2[4][0].startswith("## Any later change to npt-sim's synthetic recordings"), T2[4][0]
LOG = []


def entry_span(d, number):
    """0-based (first, last) of carry entry `number`: its first line and its four-space continuation lines."""
    starts = [i for i, l in enumerate(d) if l.startswith(f"{number}. **")]
    assert len(starts) == 1, (number, starts)
    s = e = starts[0]
    while e + 1 < len(d) and d[e + 1].startswith("    "):
        e += 1
    return s, e


def insert_after(d, at, lines, what):
    d[at + 1:at + 1] = lines
    LOG.append(f"{what}: {len(lines)} lines inserted after line {at + 1}")


# A. u4df-report.md: the measurement section at the anchor.
REPORT = f"{N}/u4df-report.md"
report = read("u4df-report.md") if os.path.exists(REPORT) else (None if APPLY else [ANCHOR])
assert report is not None, "u4df-report.md must exist with the anchor before --apply"
assert report.count(ANCHOR) == 1, report.count(ANCHOR)
k = report.index(ANCHOR)
report[k:k + 1] = T1[0]
LOG.append(f"u4df-report.md: T1 (1), {len(T1[0])} lines, at the anchor (line {k + 1})"
           + ("" if os.path.exists(REPORT) else " [simulated: the report is not written yet]"))

# B. u4d-report.md: the pointer bullet after the Routing bullet of "The merge condition".
u4d = read("u4d-report.md")
sec = [i for i, l in enumerate(u4d) if l == "## The merge condition (N2-M1, the netcode-architect's round 2)"]
assert len(sec) == 1, sec
end = next(i for i in range(sec[0] + 1, len(u4d)) if u4d[i].startswith("## "))
routing = [i for i in range(sec[0], end) if u4d[i].startswith("- **Routing:**")]
assert len(routing) == 1, routing
r = routing[0]
while u4d[r + 1].startswith("  "):
    r += 1
assert not u4d[r + 1].strip() and u4d[r + 2].startswith("## "), (u4d[r + 1], u4d[r + 2])
insert_after(u4d, r, T1[1], "u4d-report.md: T1 (2), after the Routing bullet")

# C. u4s-carries.md: carry 59's sentence, entries 68-70 at their sections' ends, the new section with 67, 71, 72.
car = read("u4s-carries.md")
for n in range(67, 73):
    assert not any(l.startswith(f"{n}. **") for l in car), f"carry {n} exists already"
OLD59 = "    script, run by run (ADR 0001:1480).\""
s59, e59 = entry_span(car, 59)
hits = [i for i in range(s59, e59 + 1) if car[i] == OLD59]
assert len(hits) == 1 and sum(l == OLD59 for l in car) == 1, hits
car[hits[0]:hits[0] + 1] = ["    " + T2[0][0]] + T2[0][1:]
LOG.append(f"u4s-carries.md: carry 59's sentence (T2 (1)), line {hits[0] + 1} -> {len(T2[0])} lines")
for number, block, heading_next, what in [
        (61, T2[1], "## U4r (the L1 runner and its judged runs)", "T2 (2), carry 68 after 61 (§U4e)"),
        (63, T2[2], "## The next change to crates/judge (whoever makes it; its first step)",
         "T2 (3), carry 69 after 63 (§U4r)"),
        (47, T2[3], "## U4h (lag compensation and aim)", "T2 (4), carry 70 after 47 (§U4g)")]:
    s, e = entry_span(car, number)
    assert not car[e + 1].strip() and car[e + 2] == heading_next, (number, car[e + 1], car[e + 2])
    insert_after(car, e, block, f"u4s-carries.md: {what}")
s64, e64 = entry_span(car, 64)
assert not car[e64 + 1].strip() and car[e64 + 2] == "## U4f (clock and commands)", (car[e64 + 1], car[e64 + 2])
insert_after(car, e64, [""] + T2[4], "u4s-carries.md: T2 (5), the new section (carries 67, 71, 72) after 64")

# D. Checks: widths, and every block found in place.
for block in T1 + T2:
    for line in block:
        assert len(line) <= 120, (len(line), line)
assert len("    " + T2[0][0]) <= 120, len("    " + T2[0][0])
for name, d, blocks in [("u4df-report.md", report, [T1[0]]), ("u4d-report.md", u4d, [T1[1]]),
                        ("u4s-carries.md", car, T2[1:])]:
    text = "\n".join(d)
    for block in blocks:
        assert "\n".join(block) in text, (name, block[0])
assert "\n".join(["    " + T2[0][0]] + T2[0][1:]) in "\n".join(car)
print("\n".join(LOG))
if APPLY:
    for name, d in [("u4df-report.md", report), ("u4d-report.md", u4d), ("u4s-carries.md", car)]:
        with open(f"{N}/{name}", "w", encoding="utf-8") as fh:
            fh.write("\n".join(d))
    print("applied")
else:
    print("dry run: every anchor found once; nothing written")
