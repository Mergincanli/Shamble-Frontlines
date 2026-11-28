"""Apply the texts of U4e's merge commit by script, as the advisor's two U4e rulings give them: advisor-u4e-r1.md (R9's
M1-M9 and T1-T6) and advisor-u4e-r2.md (Q3, which amends M5, M8 and M9; its T1, T2 block (3), T3, T5 and the new T7).
Where both rulings give a text, round 2's replaces round 1's (Q3, M8 (2)): T1, T2's block (3) (entry 80), T3, T5 and
T7 come from advisor-u4e-r2.md; T2's blocks (1), (2), (4) and (5), T4 and T6 from advisor-u4e-r1.md.

  --phase A  (M5, amended: before check.mjs's counted run on the merged tree): T1 into spikes/netcode-proof/marks/
             README.md §1, T4 into spikes/netcode-proof/marks/SERIES.md, T7 into docs/design/netcode-proof-messages.md
             (the `Resync` row's Authority cell).
  --phase B  (M8, amended: before the merge commit): T2's five blocks into u4s-carries.md, T3 into plan.md's U6 row, T6
             into u4-build-plan.md's §4.1 rows, T5 into u4e-report.md at the line ANCHOR (all in the notes folder).

Every text is read from the rulings' own code blocks, or from the exact span a ruling marks, never typed here; where a
ruling names the place (a section, an entry, a row, a line number), that is read from the ruling too and checked. Each
old text or anchor is found exactly once in its target (a line break and the next line's indent read as one space),
and each new text is found in place afterwards; no changed prose line is over 120 characters (a table line, as T3's,
T4's, T6's and T7's, is not prose: it is edited in its line); every edit is checked word for word (the target with
white space collapsed equals the old one with only that change made); carries 73-81 and C-U6-12 to C-U6-16 are
asserted absent first. The rulings' and the phase's targets' sha256 are asserted first, so a moved file stops it.

Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4e_merge_text.py --phase A|B [--apply]
Without --apply it writes nothing: it prints one log line per edit, then the unified diff of every file it would
change. With --apply in phase B, u4e-report.md must already exist and hold the line ANCHOR once (the main session
writes the report's other sections first, with the Write tool); without --apply a missing report is simulated as the
anchor line alone, so the diff shows what would be inserted."""
import bisect
import difflib
import hashlib
import os
import re
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
ARGS = sys.argv[1:]
USAGE = "usage: python3 -I .checkpoints/netcode-proof-test/u4e_merge_text.py --phase A|B [--apply]"
assert ARGS.count("--phase") == 1 and ARGS.index("--phase") + 1 < len(ARGS), USAGE
PHASE = ARGS[ARGS.index("--phase") + 1]
APPLY = "--apply" in ARGS
assert PHASE in ("A", "B") and set(ARGS) <= {"--phase", PHASE, "--apply"}, USAGE
ANCHOR = "<!-- T5: the advisor's rulings section (advisor-u4e-r2.md, T5), copied here by u4e_merge_text.py -->"

R1P, R2P = f"{N}/advisor-u4e-r1.md", f"{N}/advisor-u4e-r2.md"
README, SERIES = "spikes/netcode-proof/marks/README.md", "spikes/netcode-proof/marks/SERIES.md"
CAT = "docs/design/netcode-proof-messages.md"
CARRIES, PLAN, BPLAN, REPORT = (f"{N}/{n}" for n in ("u4s-carries.md", "plan.md", "u4-build-plan.md", "u4e-report.md"))
SHA = {
    R1P: "7c129309703969b83d0d1a4c93cba576564b7627b972bdee6a9d8e380002bf44",
    R2P: "14b0b1802ef8c14f8bf1cb6abf105ada0c427f2c96d3cff92d709ca4245bda5b",
    README: "0ad7a80f8275ddee3444d88ae37da4ffbd63134635d3103abbdd6b8dc95955f7",
    SERIES: "881d7fde98765a80428120d9d3221743b55409237ae76aa7a267c53de1dedc21",
    CAT: "35a279794a33e87f4f22bab4fadfceaf1d7bd9a93613d22dd26ec8cd8b4383fc",
    CARRIES: "cdab02404b30ac28059544b717dc638c4e1e7c7343a33028c240152ea5296c16",
    PLAN: "2bef127bcd6ccc1333ee17c1a6ea0c4c4397986db8237898a5d701265661c81a",
    BPLAN: "3f58ff0dfd624fd726d27c0efebdf551f5f97aba43943665fc100b8571ef7498",
}
TARGETS = {"A": (README, SERIES, CAT), "B": (CARRIES, PLAN, BPLAN)}


def sha256(path):
    with open(path, "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()


assert os.path.isdir(N) and os.path.exists(".git"), "run from the repo root"
for _p in (R1P, R2P) + TARGETS[PHASE]:
    _got = sha256(_p)
    assert _got == SHA[_p], f"{_p} has moved: sha256 {_got[:8]}..., the script was written for {SHA[_p][:8]}..."


def read(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read().split("\n")


R1, R2 = read(R1P), read(R2P)
NAME = {id(R1): "advisor-u4e-r1.md", id(R2): "advisor-u4e-r2.md"}


# --- the rulings: sections, their fenced blocks, their prose ---------------------------------------------------------
def section(R, heading):
    """0-based (heading line, end): the one line that starts with `heading`, up to the next heading outside a fence."""
    hits = [i for i, l in enumerate(R) if l.startswith(heading)]
    assert len(hits) == 1, (NAME[id(R)], heading, hits)
    fence = False
    for j in range(hits[0] + 1, len(R)):
        if R[j].startswith("```"):
            fence = not fence
        elif not fence and R[j].startswith("#"):
            return hits[0], j
    assert not fence, heading
    return hits[0], len(R)


def blocks(R, heading):
    """The section's fenced blocks, fences excluded: (first line, last line, lines), the line numbers 1-based."""
    s, e = section(R, heading)
    out, cur = [], None
    for i in range(s + 1, e):
        if R[i].startswith("```"):
            if cur is None:
                cur = (i + 2, [])
            else:
                out.append((cur[0], i, cur[1]))
                cur = None
        elif cur is not None:
            cur[1].append(R[i])
    assert cur is None, heading
    return out


def prose(R, a, b):
    """Lines a..b (1-based, inclusive), stripped and joined by one space: a line break and the next line's indent read
    as one space."""
    return " ".join(l.strip() for l in R[a - 1:b] if l.strip())


def between(text, start, end):
    """The exact span a ruling marks: after `start` (found once) up to the first `end` after it."""
    assert text.count(start) == 1, (start, text.count(start))
    i0 = text.index(start) + len(start)
    return text[i0:text.index(end, i0)]


def src(R, a, b, what):
    return f"{NAME[id(R)]} :{a}" + (f"-{b}" if b > a else "") + f" ({what})"


# --- the targets: finding, editing, checking -----------------------------------------------------------------------
FILES, ORIG, LOG, CHECKS, NOTES = {}, {}, [], [], []


def doc(path):
    if path not in FILES:
        FILES[path] = read(path)
        ORIG[path] = list(FILES[path])
    return FILES[path]


def find(d, old, tag):
    """The one place of `old` in d, a line break and the next line's indent read as one space (a blank line reads as a
    newline, which no old text holds): 0-based (first line, column, last line, end column), the end exclusive."""
    parts, starts, off = [], [], 0
    for l in d:
        t = l.strip() or "\n"
        starts.append(off)
        parts.append(t)
        off += len(t) + 1
    j = " ".join(parts)
    assert j.count(old) == 1, (tag, f"found {j.count(old)} times, not once", old[:80])
    o = j.index(old)

    def at(x):
        i = bisect.bisect_right(starts, x) - 1
        return i, (len(d[i]) - len(d[i].lstrip())) + (x - starts[i])

    l0, c0 = at(o)
    l1, c1 = at(o + len(old) - 1)
    assert d[l0][c0] == old[0] and d[l1][c1] == old[-1], (tag, "the place maps back wrong")
    return l0, c0, l1, c1 + 1


def collapse(text):
    return " ".join(text.split())


def edit(tag, path, a, b, new_lines, pairs, source, what):
    """Lines a..b-1 (0-based) of `path` become `new_lines`. Checked word for word: the file with white space collapsed
    equals the old one with each (old, new) pair's change made and nothing else."""
    d = doc(path)
    before = list(d)
    d[a:b] = new_lines
    exp = collapse("\n".join(before))
    for o, n in pairs:
        oc = collapse(o)
        assert exp.count(oc) == 1, (tag, "the old text, white space collapsed, is not in the file once", oc[:80])
        exp = exp.replace(oc, collapse(n))
    assert collapse("\n".join(d)) == exp, (tag, "the edit changed more than its text")
    span = f"{a + 1}-{b}" if b > a + 1 else (f"{a + 1}" if b == a + 1 else f"{a + 1} (inserted before)")
    LOG.append(f"{tag} [phase {PHASE}]: {path}:{span}, {b - a} line(s) -> {len(new_lines)}: {what}; from {source}")


RISKY = re.compile(r"^(\d+[.)](\s|$)|[-*+](\s|$)|#|>|<|```|~~~|\||=+\s*$)")
NBSP = " "


def refill(text, indent, tag):
    """`text` wrapped to 120 characters at `indent`; no line may start with a token that would begin a block."""
    assert NBSP not in text, tag
    t = text
    for _ in range(20):
        lines = textwrap.TextWrapper(width=120, initial_indent=indent, subsequent_indent=indent,
                                     break_long_words=False, break_on_hyphens=False).wrap(t)
        bad = [k for k in range(len(lines)) if RISKY.match(lines[k][len(indent):])]
        if not bad:
            lines = [l.replace(NBSP, " ") for l in lines]
            assert all(len(l) <= 120 for l in lines), (tag, "a word is longer than the line")
            return lines
        k = bad[0]
        assert k > 0, (tag, "the refilled text's first line would begin a block")
        tok = lines[k][len(indent):].split(" ")[0]
        prev = lines[k - 1].rstrip().split(" ")[-1]
        assert f"{prev} {tok}" in t, (tag, prev, tok)
        t = t.replace(f"{prev} {tok}", f"{prev}{NBSP}{tok}", 1)
    sys.exit(f"{tag}: no safe wrap")


def table_bounds(d, i):
    t0 = t1 = i
    while t0 > 0 and d[t0 - 1].startswith("|"):
        t0 -= 1
    while t1 + 1 < len(d) and d[t1 + 1].startswith("|"):
        t1 += 1
    return t0, t1


def heading_above(d, i, marks=("#",)):
    return next(d[j] for j in range(i, -1, -1) if d[j].startswith(marks))


ENTRY = re.compile(r"^(\d+)\. \*\*")


def entry_span(d, number):
    """0-based (first, last) of carry entry `number`: its first line and its four-space continuation lines."""
    starts = [i for i, l in enumerate(d) if l.startswith(f"{number}. **")]
    assert len(starts) == 1, (number, starts)
    s = e = starts[0]
    while e + 1 < len(d) and d[e + 1].startswith("    "):
        e += 1
    return s, e


# --- phase A ----------------------------------------------------------------------------------------------------------
def t1():
    tag = "T1"
    h, _ = section(R2, "### T1.")
    bl = blocks(R2, "### T1.")
    assert len(bl) == 1 and len(bl[0][2]) == 1, tag
    a, _, (new,) = bl[0]
    p = prose(R2, h + 2, a - 2)
    old = between(p, "The old text `", "` (found once")
    assert "rewrapped to the item's indent within 120" in p, tag
    sec = between(R2[h], "marks/README.md §", ", the `")
    item = between(R2[h], ", the `", "` item")
    assert new.startswith(old), (tag, "the new text keeps the old sentence first")
    d = doc(README)
    l0, _, l1, _ = find(d, old, tag)
    s = l0
    while not d[s].startswith("  - "):
        assert d[s].startswith("    ") and d[s].strip(), (tag, "not inside a list item", s + 1)
        s -= 1
    assert d[s].startswith(f"  - `{item}`"), (tag, d[s][:40])
    assert heading_above(d, s, ("## ",)).startswith(f"## {sec}. "), (tag, heading_above(d, s, ("## ",)))
    assert l0 > s, (tag, "the old text starts on the item's first line")
    e = l1
    while e + 1 < len(d) and d[e + 1].startswith("    ") and d[e + 1].strip():
        e += 1
    assert d[e + 1].startswith("  - "), (tag, "the item's end", d[e + 1][:30])
    text = " ".join(x.strip() for x in d[l0:e + 1])
    assert text.count(old) == 1, tag
    lines = refill(text.replace(old, new), "    ", tag)
    edit(tag, README, l0, e + 1, lines, [(old, new)],
         f"{src(R2, a, a, 'the block')}; {src(R2, h + 2, a - 2, 'the old text')}",
         f"§{sec}'s `{item}` item: the old sentence and the text after it, refilled at the item's indent from the line "
         f"the old text starts on to the item's end")
    CHECKS.append((README, "words", new))


def t4():
    tag = "T4"
    h, _ = section(R1, "### T4.")
    bl = blocks(R1, "### T4.")
    assert len(bl) == 1 and len(bl[0][2]) == 1, tag
    a, _, (row,) = bl[0]
    p = prose(R1, h + 2, a - 2)
    table_name = between(p, "In the table that holds `", "`, as a new line")
    before_name = between(p, "right before the row named `", "` (one such row")
    line_no = int(between(p, "(one such row, SERIES.md:", " at the branch head"))
    above_name = between(p, "in name order, and `", "` is the row above")
    assert row.startswith("| `"), tag
    new_name = row[3:row.index("` |")]
    d = doc(SERIES)
    assert new_name not in "\n".join(d), (tag, "the row exists already", new_name)
    hits = [i for i, l in enumerate(d) if l.startswith(f"| `{before_name}` |")]
    assert len(hits) == 1, (tag, hits)
    i = hits[0]
    assert i + 1 == line_no, (tag, "the row named is not at the ruling's line", i + 1, line_no)
    assert d[i - 1].startswith(f"| `{above_name}` |"), (tag, d[i - 1][:50])
    t0, t1_ = table_bounds(d, i)
    assert sum(d[k].startswith(f"| `{table_name}` |") for k in range(t0, t1_ + 1)) == 1, (tag, table_name)
    assert above_name < new_name < before_name, (tag, "the rows' name order")
    assert row.count("|") == d[i].count("|") == d[t0].count("|"), (tag, "the row's cells")
    edit(tag, SERIES, i, i, [row], [(d[i], row + "\n" + d[i])],
         f"{src(R1, a, a, 'the row')}; {src(R1, h + 2, a - 2, 'the place')}",
         f"the `{new_name}` row, right before `{before_name}` (with `{above_name}` above it), in the table that holds "
         f"`{table_name}`")
    CHECKS.append((SERIES, "lines", [d[i - 1], row, d[i + 1]]))
    head = heading_above(d, t0, ("## ",))
    m = re.match(r"^## .*\((\d+)\)$", head)
    rows = sum(1 for k in range(t0 + 2, t1_ + 2) if d[k].startswith("| `"))
    if m and int(m.group(1)) != rows:
        NOTES.append(f"NOTE (T4, not by script): SERIES.md's heading reads '{head}', and its table now holds {rows} "
                     f"rows; neither ruling changes the heading, so the script leaves it as it is")


def t7():
    tag = "T7"
    h, _ = section(R2, "### T7.")
    bl = blocks(R2, "### T7.")
    assert len(bl) == 1 and len(bl[0][2]) == 1, tag
    a, _, (new,) = bl[0]
    assert R2[h].startswith(f"### T7. {CAT}, "), (tag, R2[h][:60])
    p = prose(R2, h + 2, a - 2)
    old = between(p, "The old text `", "` (found once")
    row_name = between(p, "(found once, in the `", "` row")
    line_no = int(between(p, f"in the `{row_name}` row, :", " at ab48790"))
    cell = between(R2[h], f"the `{row_name}` row's ", " cell")
    first = between(p, 'the cell still starts "', '"')
    assert '"|"' in p and "|" not in old and "|" not in new, (tag, "no '|' added")
    d = doc(CAT)
    l0, c0, l1, c1 = find(d, old, tag)
    assert l0 == l1 and l0 + 1 == line_no and d[l0].startswith(f"| `{row_name}` |"), (tag, l0 + 1, d[l0][:20])
    t0, _ = table_bounds(d, l0)
    k = [x.strip() for x in d[t0].split("|")].index(cell)
    cells = d[l0].split("|")
    lo = len("|".join(cells[:k])) + 1
    assert lo <= c0 and c1 <= lo + len(cells[k]), (tag, f"the old text is not in the {cell} cell")
    assert cells[k].strip().startswith(first), tag
    line = d[l0][:c0] + new + d[l0][c1:]
    after = line.split("|")
    assert len(after) == len(cells) and after[k].strip().startswith(first), (tag, "the row's cells")
    edit(tag, CAT, l0, l0 + 1, [line], [(old, new)],
         f"{src(R2, a, a, 'the block')}; {src(R2, h + 2, a - 2, 'the old text')}",
         f"the `{row_name}` row's {cell} cell, in its line (a table line); the cell still starts '{first}'")
    CHECKS.append((CAT, "words", new))


# --- phase B ----------------------------------------------------------------------------------------------------------
LABEL = re.compile(r"^\*\*\((\d)\) §(U4[a-z]), (?:after entry (\d+) \(its last\)|entry (\d+) \(([a-z])\)):\*\*")


def after_last(tag, sect, n, block, source):
    """`block` (whole carry entries) inserted after entry `n`, the last of the section `## <sect> ...`."""
    d = doc(CARRIES)
    s, e = entry_span(d, n)
    assert heading_above(d, s, ("## ",)).startswith(f"## {sect} "), (tag, heading_above(d, s, ("## ",)))
    assert not d[e + 1].strip() and d[e + 2].startswith("## "), (tag, f"entry {n} is not its section's last")
    assert ENTRY.match(block[0]), (tag, block[0][:30])
    assert all(ENTRY.match(l) or (l.startswith("    ") and l.strip()) for l in block), (tag, "an entry's form")
    assert all(len(l) <= 120 for l in block), (tag, "a line over 120")
    nums = [int(ENTRY.match(l).group(1)) for l in block if ENTRY.match(l)]
    for x in nums:
        assert not any(l.startswith(f"{x}. ") for l in d), (tag, f"carry {x} exists already")
    old = "\n".join(d[s:e + 1])
    edit(tag, CARRIES, e + 1, e + 1, block, [(old, old + "\n" + "\n".join(block))], source,
         f"carr{'ies' if len(nums) > 1 else 'y'} {', '.join(map(str, nums))} after entry {n}, the last of §{sect}")
    CHECKS.append((CARRIES, "lines", d[e:e + 2 + len(block)]))
    return nums


def t2():
    h, e = section(R1, "### T2.")
    bl = blocks(R1, "### T2.")
    assert len(bl) == 5, len(bl)
    labels = [(i + 1, LABEL.match(R1[i])) for i in range(h + 1, e) if LABEL.match(R1[i])]
    assert [int(m.group(1)) for _, m in labels] == [1, 2, 3, 4, 5], [m.group(0) for _, m in labels]
    for k, ((ln, _), (a, b, _)) in enumerate(zip(labels, bl)):
        assert ln < a and (k == 4 or labels[k + 1][0] > b), ("T2: a label is not right above its block", k + 1)
    h3, _ = section(R2, "### T2, block (3).")
    m3 = re.match(r"^### T2, block \(3\)\. u4s-carries\.md §(U4[a-z]), after entry (\d+) \(its last\)", R2[h3])
    bl3 = blocks(R2, "### T2, block (3).")
    assert m3 and len(bl3) == 1, "T2 (3)"
    assert m3.groups() == labels[2][1].group(2, 3), ("T2 (3): round 2's block goes where round 1's did", m3.groups())
    assert bl[2][2][0].startswith("80. **") and bl3[0][2][0].startswith("80. **"), "T2 (3): entry 80 in both rulings"

    nums = []
    for k in (0, 3, 4):
        ln, m = labels[k]
        a, b, lines = bl[k]
        nums += after_last(f"T2 ({k + 1})", m.group(2), int(m.group(3)), lines,
                           f"{src(R1, a, b, 'the block')}; {src(R1, ln, ln, 'the place')}")
    a, b, lines = bl3[0]
    nums += after_last("T2 (3)", m3.group(1), int(m3.group(2)), lines,
                       f"{src(R2, a, b, 'the block, replacing advisor-u4e-r1.md :' + str(bl[2][0]) + '-' + str(bl[2][1]))}"
                       f"; {src(R2, h3 + 1, h3 + 1, 'the place')}")
    assert sorted(nums) == list(range(73, 82)), ("T2: the new carries are not 73-81, each once", sorted(nums))

    # (2): entry 62 (h), one span changed and one sentence inserted, refilled from the line the change starts on
    tag = "T2 (2)"
    ln, m = labels[1]
    a, b, lines = bl[1]
    assert len(lines) == 1, tag
    ins = lines[0]
    sect, n, letter = m.group(2), int(m.group(4)), m.group(5)
    p = prose(R1, ln, a - 2)
    old1 = between(p, ":** the old text `", "` (found once) becomes `")
    new1 = between(p, "` (found once) becomes `", "`; and after the old text `")
    anchor = between(p, "`; and after the old text `", "` (found once) this is inserted")
    assert p.endswith("this is inserted, before the rest of the entry:"), tag
    d = doc(CARRIES)
    s, e = entry_span(d, n)
    assert heading_above(d, s, ("## ",)).startswith(f"## {sect} "), tag
    a0, _, a1, _ = find(d, old1, tag)
    b0, _, b1, _ = find(d, anchor, tag)
    assert s < a0 <= a1 <= b0 <= b1 <= e, (tag, "the spans are not inside the entry, in order", s, a0, b1, e)
    ej = " ".join(x.strip() for x in d[s:e + 1])
    assert ej.count(f"({letter}) ") == 1 and ej.index(f"({letter}) ") < ej.index(old1) < ej.index(anchor), tag
    assert f"({chr(ord(letter) + 1)}) " not in ej, (tag, "a later item follows")
    text = " ".join(x.strip() for x in d[a0:e + 1])
    assert text.count(old1) == 1 and text.count(anchor) == 1, tag
    refilled = refill(text.replace(old1, new1).replace(anchor, anchor + " " + ins), "    ", tag)
    edit(tag, CARRIES, a0, e + 1, refilled, [(old1, new1), (anchor, anchor + " " + ins)],
         f"{src(R1, a, a, 'the inserted text')}; {src(R1, ln, a - 2, 'the old texts')}",
         f"entry {n} ({letter}): the check's window changed and the sentence inserted after the anchor, before the rest "
         f"of the entry; refilled at the entry's indent from the line the change starts on to the entry's end")
    CHECKS.append((CARRIES, "words", new1))
    CHECKS.append((CARRIES, "words", anchor + " " + ins))


def t3():
    tag = "T3"
    h, _ = section(R2, "### T3.")
    bl = blocks(R2, "### T3.")
    assert len(bl) == 1 and len(bl[0][2]) == 1, tag
    a, _, (new,) = bl[0]
    p = prose(R2, h + 2, a - 2)
    old = between(p, "The old text `", "` (found once")
    row = between(R2[h], "plan.md's ", " row")
    d = doc(PLAN)
    codes = [f"C-U6-{k}" for k in range(12, 17)]
    for c in codes:
        assert c not in "\n".join(d), (tag, f"{c} exists already")
        assert new.count(f"**{c} ") == 1, (tag, f"{c} is not in the new text once")
    l0, c0, l1, c1 = find(d, old, tag)
    assert l0 == l1 and d[l0].startswith(f"| {row} |") and c1 == len(d[l0]), (tag, "the old text is not the row's end")
    assert "C-U6-11" in d[l0], tag
    assert old.count("|") == new.count("|") == 1 and old.endswith(" |") and new.endswith(" |"), (tag, "the cells")
    edit(tag, PLAN, l0, l0 + 1, [d[l0][:c0] + new], [(old, new)],
         f"{src(R2, a, a, 'the block')}; {src(R2, h + 2, a - 2, 'the old text')}",
         f"the {row} row's end, in its line (a table line): C-U6-12 to C-U6-16 after C-U6-11")
    CHECKS.append((PLAN, "words", new))


def t6():
    h, e = section(R1, "### T6.")
    bl = blocks(R1, "### T6.")
    assert len(bl) == 6 and all(len(x[2]) == 1 for x in bl), "T6"
    sec = between(R1[h], "u4-build-plan.md §", ":")
    d = doc(BPLAN)
    sh = [i for i, l in enumerate(d) if l.startswith(f"### {sec} ")]
    assert len(sh) == 1, ("T6", sh)
    send = next(i for i in range(sh[0] + 1, len(d)) if d[i].startswith("#"))

    def label(a):
        j = a - 2
        while not R1[j - 1].startswith("- "):
            j -= 1
        return j, prose(R1, j, a - 2)

    for k in (0, 2, 4):
        (a, _, (old,)), (an, _, (new,)) = bl[k], bl[k + 1]
        lo, lp = label(a)
        ln_, lnp = label(an)
        assert lnp.startswith("- New"), ("T6", lnp)
        whole = re.match(r"^- The (npt-[a-z]+) row, old \(one line\):$", lp)
        part = re.match(r'^- The (npt-[a-z]+) row keeps its start; its end, from the words "(.+)" \(once in that row\) '
                        r"to the row's closing bar, old:$", lp)
        assert whole or part, ("T6", lp)
        crate = (whole or part).group(1)
        tag = f"T6 {crate}"
        assert old.count("|") == new.count("|"), (tag, "the cells")
        rows = [i for i in range(sh[0], send) if d[i].startswith(f"| {crate} |")]
        assert len(rows) == 1, (tag, rows)
        l0, c0, l1, c1 = find(d, old, tag)
        assert l0 == l1 == rows[0] and c1 == len(d[l0]), (tag, "the old text is not the end of the §4.1 row")
        source = f"{src(R1, an, an, 'the new text')}; {src(R1, a, a, 'the old text')}; {src(R1, lo, ln_, 'the labels')}"
        if whole:
            assert c0 == 0 and old.startswith(f"| {crate} |") and new.startswith(f"| {crate} |"), tag
            edit(tag, BPLAN, l0, l0 + 1, [new], [(old, new)], source, f"§{sec}'s {crate} row, whole (a table line)")
        else:
            words = part.group(2)
            assert d[l0].count(words) == 1 and old.startswith(words), (tag, words)
            edit(tag, BPLAN, l0, l0 + 1, [d[l0][:c0] + new], [(old, new)], source,
                 f"§{sec}'s {crate} row: its start kept, its end from '{words}' to the closing bar (a table line)")
        CHECKS.append((BPLAN, "words", new))


def t5():
    tag = "T5"
    h, _ = section(R2, "### T5.")
    bl = blocks(R2, "### T5.")
    assert len(bl) == 1, tag
    a, b, block = bl[0]
    assert block[0].startswith("## What U4e's review settled"), (tag, block[0][:40])
    assert all(len(l) <= 120 for l in block), (tag, "a line over 120")
    exists = os.path.exists(REPORT)
    assert exists or not APPLY, "u4e-report.md must exist and hold the anchor line once before --apply"
    if exists:
        d = doc(REPORT)
    else:
        FILES[REPORT], ORIG[REPORT] = [ANCHOR], [ANCHOR]
        d = FILES[REPORT]
    hits = [i for i, l in enumerate(d) if l == ANCHOR]
    assert len(hits) == 1 and "\n".join(d).count(ANCHOR) == 1, (tag, "the anchor line is not in the report once")
    k = hits[0]
    edit(tag, REPORT, k, k + 1, block, [(ANCHOR, "\n".join(block))], src(R2, a, b, "the block"),
         "the rulings section at the anchor line" + ("" if exists else " [simulated: the report is not written yet]"))
    CHECKS.append((REPORT, "lines", block))
    holes = re.findall(r"<[^<>\n]*>", "\n".join(block))
    NOTES.append(f"NOTE (T5, not by script): the section holds {len(holes)} '<...>' places the main session fills from "
                 f"M5's lines after the copy, naming each fill's log and line in the notes")
    if exists:
        NOTES.append(f"NOTE (T5): u4e-report.md's sha256 before the copy: {sha256(REPORT)[:8]}...")


if PHASE == "A":
    t1()
    t4()
    t7()
else:
    for _n in range(73, 82):
        assert not any(re.match(rf"^{_n}\. ", l) for l in doc(CARRIES)), f"carry {_n} exists already"
    for _n in range(12, 17):
        assert f"C-U6-{_n}" not in "\n".join(doc(PLAN)), f"C-U6-{_n} exists already"
    t2()
    t3()
    t6()
    t5()

# --- every new text in place; widths; the log and the diff ----------------------------------------------------------
for path, kind, text in CHECKS:
    if kind == "lines":
        assert "\n".join(text) in "\n".join(FILES[path]), (path, "a new text is not in place", text[0][:60])
    else:
        assert collapse(text) in collapse("\n".join(FILES[path])), (path, "a new text is not in place", text[:60])
SUMMARY, DIFFS = [], []
for path in FILES:
    if FILES[path] == ORIG[path]:
        continue
    diff = list(difflib.unified_diff(ORIG[path], FILES[path], f"a/{path}", f"b/{path}", n=1, lineterm=""))
    added = [l[1:] for l in diff[2:] if l.startswith("+")]
    removed = [l[1:] for l in diff[2:] if l.startswith("-")]
    long = [l[:60] for l in added if not l.startswith("|") and len(l) > 120]
    assert not long, (path, "a changed prose line is over 120 characters", long)
    before = hashlib.sha256("\n".join(ORIG[path]).encode()).hexdigest()[:8]
    after = hashlib.sha256("\n".join(FILES[path]).encode()).hexdigest()[:8]
    SUMMARY.append(f"{path}: +{len(added)} -{len(removed)} lines; sha256 {before} -> {after}; "
                   f"changed prose lines over 120: none")
    DIFFS.append("\n".join(diff))
print("\n".join(LOG))
print("\n".join(NOTES))
print("\n".join(SUMMARY))
if APPLY:
    for path in FILES:
        if FILES[path] != ORIG[path]:
            with open(path, "w", encoding="utf-8", newline="") as fh:
                fh.write("\n".join(FILES[path]))
            assert read(path) == FILES[path], (path, "the written file reads back different")
    print(f"phase {PHASE}: APPLIED ({len(LOG)} edits)")
else:
    print("\n".join(DIFFS))
    print(f"phase {PHASE}: dry run, every assert passed ({len(LOG)} edits); nothing written")
