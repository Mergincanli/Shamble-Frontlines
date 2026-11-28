"""L1 of the advisor's check of the U4d2 builder brief (advisor-u4d2-brief.md, block 11): fold the ruling into
u4d2-brief.md (rev 1.1 -> rev 1.2), then fill the placeholders P1 to P5 and P8 by 7.2 item 3's rule as folded.
Folded (L1's list): block 4's A1 to A10 (A7 as block 9 corrects it; A1 carrying A12's seat), block 5's words for
U4D2-Q1 to Q12 with section 8's table, the header and 7.2 items 4 and 9, block 6's A11 and its 1.2 sentence for the
builder, block 8's A14, block 10's A15, and L1's three "read X for Y" corrections. Not folded: the "Plan, ..." items
and A13's §4.11 (applied to the plan at 85c08df), the card item, and block 6's 1.2 replacement (only if U4e merged
before launch; it has not). P6 and P7 are the main session's and stay.
Every old phrase, new phrase and inserted text is read from the ruling's own lines (each item's line range and its
markers below; wrapped lines joined with one space), never typed here; each placeholder's anchors and line ranges are
read from the brief's own 7.2 item 3 as folded. Each old phrase or anchor must occur once in the brief at its turn
(line breaks and the next line's indent read as one space; A1's ordered pair: (d) after (a)), and the anchors must not
overlap in rev 1.1. A table row is edited in its line; any other paragraph or bullet is rewrapped from its first
changed line to its end within 120 characters, its earlier lines untouched; each edit is checked word for word.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4d2_fold.py [--apply]
Without --apply it writes nothing and prints one log line per edit and the unified diff."""
import difflib
import hashlib
import re
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
APPLY = "--apply" in sys.argv
BRIEF = f"{N}/u4d2-brief.md"
RULING = f"{N}/advisor-u4d2-brief.md"
HASHES = {
    BRIEF: "e6bcefc841dd963dd4ce9c4bd72b3dc62efa5bf11aea2eb4aa06c3c51b9268fa",
    RULING: "155f3376b47c05011bb18b3c1881242d528a7bd743d28cb2a0b58cc6921379b3",
}
# the placeholders' sources: (path, sha256, section 0's name for it)
SOURCES = {
    "carries": (f"{N}/u4s-carries.md", "cdab02404b30ac28059544b717dc638c4e1e7c7343a33028c240152ea5296c16"),
    "history": (f"{N}/history.md", "c62ac831ff2ebd5e93023fe2b65a90a614fa24443e864fa704ba0475d654b55e"),
    "df-ruling": (f"{N}/advisor-u4df.md", "842ec092a4b473d6d91be2147c7b5dc39cd8fb435dac2ff0d58c88b3ce791151"),
    "j-r2": (f"{N}/advisor-u4j-r2.md", "7efa9410323f09236c2f8f3ca09b91045b2e9217d64bd1864c98786e85deead1"),
}


def sha(path):
    with open(path, "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()


def lines(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read().split("\n")


for p, h in HASHES.items():
    got = sha(p)
    if got != h:
        sys.exit(f"{p}: sha256 {got}, expected {h}: stop")
SRC = lines(RULING)
L = lines(BRIEF)
ORIG = list(L)
LOG, NEWS, SPANS = [], [], []


# ---------------------------------------------------------------- reading the ruling
def joined(a, b):
    return " ".join(l.strip() for l in SRC[a - 1:b])


LAST = None  # as the final marker: up to the range's last double quote


def cut(a, b, *marks):
    """The pieces between consecutive markers in the ruling's lines a..b (joined); the first marker must occur once;
    each later marker is the first after the previous one; LAST as the final marker means the range's last quote."""
    j = joined(a, b)
    assert j.count(marks[0]) == 1, (a, b, marks[0], j.count(marks[0]))
    pos = j.index(marks[0]) + len(marks[0])
    out = []
    for mk in marks[1:]:
        if mk is LAST:
            end = j.rindex('"')
            assert end >= pos and j[end + 1:].strip() in ("", "."), (a, b, j[end:])
        else:
            end = j.find(mk, pos)
            assert end >= pos, (a, b, mk)
        out.append(j[pos:end])
        if mk is not LAST:
            pos = end + len(mk)
    return out


def fenced(a, b):
    assert SRC[a - 2] == "```" and SRC[b] == "```", (a, b)
    return SRC[a - 1:b]


# ---------------------------------------------------------------- editing the brief
def norm(t):
    return " ".join(t.split())


def ws(t):
    return r"\s+".join(re.escape(w) for w in t.split())


MARK = re.compile(r"^( *)(- |\d+\. )")
STRUCT = re.compile(r"^(- |\d+\. |#|\||```|\*\*|<<)")
RISKY = re.compile(r"^(\d+[.)](\s|$)|[-*+](\s|$)|#|>|```|~~~|\||=+\s*$|\*\*|<<)")
NBSP = " "


def refill(t, first, cont):
    for _ in range(40):
        w = textwrap.TextWrapper(width=120, initial_indent=first, subsequent_indent=cont, break_long_words=False,
                                 break_on_hyphens=False)
        out = w.wrap(t)
        bad = [k for k in range(1, len(out)) if RISKY.match(out[k][len(cont):])]
        if not bad:
            return [l.replace(NBSP, " ") for l in out]
        k = bad[0]
        tok = out[k][len(cont):].split(" ")[0]
        prev = out[k - 1].rstrip().split(" ")[-1]
        assert f"{prev} {tok}" in t, (prev, tok)
        t = t.replace(f"{prev} {tok}", f"{prev}{NBSP}{tok}", 1)
    sys.exit(f"refill: no safe wrap for {t[:60]!r}")


def indent(l):
    return len(l) - len(l.lstrip(" "))


def unit_end(D, f):
    """The prefix, continuation indent and last line of the unit (paragraph, bullet or item) that line f is in,
    counted from line f."""
    m = MARK.match(D[f])
    prefix = m.group(0) if m else " " * indent(D[f])
    cont = " " * len(prefix)
    e = f
    while e + 1 < len(D) and D[e + 1].strip() and indent(D[e + 1]) == len(cont) \
            and not STRUCT.match(D[e + 1].lstrip(" ")):
        e += 1
    return prefix, cont, e


def region():
    """7.2 item 3's list (the record of the placeholders' names): its first and last line."""
    s = [i for i, l in enumerate(L) if l.startswith("3. **The placeholders, by script")]
    assert len(s) == 1, s
    e = next(i for i in range(s[0] + 1, len(L)) if L[i].startswith("4. **"))
    return s[0], e - 1


def edit(tag, old, new, mode="replace", outside=None):
    """mode replace: old -> new; mode after: `new` inserted right after the anchor `old`. `outside`: count only the
    matches outside that line range (7.2 item 3's list)."""
    global L
    text = "\n".join(L)
    ms = list(re.finditer(ws(old), text))
    if outside:
        a = len("\n".join(L[:outside[0]]))
        b = len("\n".join(L[:outside[1] + 1]))
        ms = [m for m in ms if not (a <= m.start() < b)]
    if len(ms) != 1:
        sys.exit(f"{tag}: the {'old phrase' if mode == 'replace' else 'anchor'} occurs {len(ms)} times in the brief"
                 f"{' outside 7.2 item 3' if outside else ''}: {old!r}")
    m = ms[0]
    before = norm(text)
    if not outside:
        assert before.count(norm(old)) == 1, (tag, "flattened", before.count(norm(old)))
    if mode == "after":
        if not new.startswith(" "):
            new = " " + new
        cp, tail, e0 = m.end(), new, m.end()
        want = norm(text[:m.end()] + new + text[m.end():])
    else:
        ow, nw = old.split(), new.split()
        k = 0
        while k < min(len(ow), len(nw)) and ow[k] == nw[k]:
            k += 1
        if k:
            mt = text[m.start():m.end()]
            wm = list(re.finditer(r"\S+", mt))
            cp = m.start() + wm[k - 1].end()
            wn = list(re.finditer(r"\S+", new))
            tail = new[wn[k - 1].end():]
        else:
            cp, tail = m.start(), new
        e0 = m.end()
        want = norm(text[:m.start()] + new + text[m.end():])
        if not outside:
            assert want == before.replace(norm(old), norm(new), 1), (tag, "the flattened replacement differs")
    f = text.count("\n", 0, cp)
    last = text.count("\n", 0, max(e0 - 1, cp))
    NL = (text[:cp] + tail + text[e0:]).split("\n")
    delta = len(NL) - len(L)
    if L[f].lstrip().startswith("|"):
        assert last == f and delta == 0, (tag, "a row edit runs past its row")
        how = f"row :{f + 1}"
    else:
        prefix, cont, e = unit_end(L, f)
        assert last <= e, (tag, "the match runs past its unit", f + 1, last + 1, e + 1)
        assert NL[f].startswith(prefix), (tag, NL[f][:20])
        blk = NL[f:e + 1 + delta]
        body = " ".join([blk[0][len(prefix):].strip()] + [x.strip() for x in blk[1:]])
        out = refill(body, prefix, cont)
        assert norm(" ".join(out)) == norm(" ".join(blk)), tag
        NL[f:e + 1 + delta] = out
        how = f"unit :{f + 1}-{e + 1}, rewrapped from :{f + 1} ({e - f + 1} -> {len(out)} lines)"
    got = norm("\n".join(NL))
    assert got == want, (tag, "a word outside the edit changed")
    L = NL
    LOG.append(f"{tag}: {mode} once; {how}; {len(old)} chars {'anchor' if mode == 'after' else '->'} {len(new)}")
    NEWS.append((tag, new))


def span(tag, old):
    """Where an item's old phrase or anchor sits in rev 1.1 (for the overlap check)."""
    ms = list(re.finditer(ws(old), "\n".join(ORIG)))
    assert len(ms) == 1, (tag, "in rev 1.1", len(ms))
    SPANS.append((ms[0].start(), ms[0].end(), tag))


# ---------------------------------------------------------------- L1: the items
ITEMS = []  # (tag, mode, old, new)


def R(tag, a, b, end=LAST, start='replace "', mid='" with "'):
    old, new = cut(a, b, start, mid, end)
    ITEMS.append((tag, "replace", old, new))
    return old, new


def A(tag, a, b, mid, end=LAST):
    anchor, ins = cut(a, b, 'after "', mid, end)
    ITEMS.append((tag, "after", anchor, ins))
    return anchor, ins


# block 4
R("A1.1", 159, 163)                                   # 2.4 (a), Timing
R("A1.2", 164, 165)                                   # 2.4 (a) item 6, std-sin
R("A1.3", 166, 168)                                   # section 5 item 4
R("A1.4", 169, 172, end='". (Edited')                 # 7.2 item 7, with A12's seat; after A1.1 (the ordered pair)
R("A2.1", 186, 191)                                   # 7.2 item 2
o1, n1, o2, n2 = cut(192, 196, '1.4: replace "', '" with "', '"; and replace "', '" with "', LAST)
ITEMS += [("A2.2", "replace", o1, n1), ("A2.3", "replace", o2, n2)]   # 1.4, the two halves
A("A2.4", 197, 202, '" add: "')                       # 2.4 (a), on a FAIL
R("A2.5", 203, 204)                                   # 2.7's lead
R("A2.6", 205, 205)                                   # 2.7's h2 row
R("A2.7", 206, 207)                                   # section 4 rule 2
A3_OLD, A3_NEW = R("A3", 212, 217)                    # 7.2 item 3's P4 bullet
R("A4.1", 224, 224)                                   # 1.3
R("A4.2", 225, 225)                                   # 1.4
R("A4.3", 226, 226)                                   # 1.5
R("A4.4", 227, 228)                                   # 2.7's lead
R("A4.5", 229, 230)                                   # section 5 item 3
R("A4.6", 231, 236)                                   # 7.2 item 3's lead
A("A5", 242, 244, '" insert "')                       # 2.4 (b)'s self-test
R("A6.1", 250, 253)                                   # h7 (ii)
R("A6.2", 254, 256)                                   # 2.3's carry 66 bullet
a7_old, a7_new = cut(262, 264, 'replace "', '" with "', LAST)
x7, y7 = cut(618, 620, 'read "', '" in place of "', '". Everything')   # block 9's erratum: read x7 for y7
assert a7_new.count(y7) == 1, ("A7", "block 9's old words are not in A7's new text once")
ITEMS.append(("A7", "replace", a7_old, a7_new.replace(y7, x7)))      # 7.2 item 8
R("A8", 267, 269)                                     # 2.4 (a) item 6
R("A9", 272, 274)                                     # 2.4 (d)
A("A10", 280, 283, '" add "')                         # section 4's expected failures
# block 5
R("Q1", 292, 294)                                     # section 0
R("Q3", 310, 312)                                     # 2.1
R("Q4", 320, 323)                                     # 2.4 (c) (ii)
R("Q5.1", 336, 337)                                   # 2.4 (a) item 6
R("Q5.2", 338, 339)                                   # 2.7's std-sin row
q9_anchor, q9_ins = cut(367, 370, 'after "', '" add "', LAST)      # 2.5: a new bullet after proto's
assert q9_ins.startswith("\\n- "), q9_ins[:6]
ITEMS.append(("Q9.1", "bullet", q9_anchor, q9_ins[2:]))
R("Q9.2", 371, 372)                                   # 2.4 (e)
R("Q9.3", 373, 375)                                   # section 4's files
R("Q9.4", 376, 377)                                   # 3.1's carry 41 note
R("Q9.5", 378, 380)                                   # return item 9
(s8_head,) = cut(417, 417, 'after the heading "', '" insert:')
ITEMS.append(("S8", "table", s8_head, fenced(419, 434)))           # section 8's ruled table
t4_old, t4_new = cut(436, 440, 'replace "', '" with "', LAST)      # 7.2 item 4, with L1's two corrections
x1, y1, x2, y2, x3, y3 = cut(688, 698, 'replacement for 7.2 item 4, "', '" for "', '" and "', '" for "',
                             '"; and, in 7.2 item 8, "', '" for "', '". Each old')
for x, y in ((x1, y1), (x2, y2)):
    assert t4_new.count(y) == 1, ("7.2.4", y)
    t4_new = t4_new.replace(y, x)
ITEMS.append(("7.2.4", "replace", t4_old, t4_new))
R("7.2.9", 441, 442)                                  # 7.2 item 9
R("header", 443, 445)                                 # the header's Next
# block 6
R("A11", 478, 480, end='". (Checked')                 # section 4 rule 2
A("1.2", 494, 498, '" add "', end='" (The anchor')    # 1.2's sentence for the builder (U4e merging while it builds)
# block 8
A("A14", 608, 611, '" insert "', end='" (The anchor') # section 4's files bullet
# block 10
R("A15.1", 648, 659)                                  # 2.4 (c) (ii)
R("A15.2", 660, 662)                                  # 2.4 (c)'s self-test controls
R("A15.3", 663, 665)                                  # 2.7's flags row
R("A15.4", 666, 668)                                  # R24
# L1's correction of 7.2 item 8 itself: read x3 for y3
ITEMS.append(("L1-7.2.8", "replace", y3, x3))

# the anchors in rev 1.1: each once, none overlapping
for tag, mode, old, new in ITEMS:
    if mode == "table":
        assert ORIG.count(old) == 1, (tag, ORIG.count(old))
        continue
    span(tag, old)
SPANS.sort()
for (a0, a1, ta), (b0, b1, tb) in zip(SPANS, SPANS[1:]):
    assert a1 <= b0, ("anchors overlap", ta, tb)

for tag, mode, old, new in ITEMS:
    if mode in ("replace", "after"):
        edit(tag, old, new, mode)
    elif mode == "bullet":
        text = "\n".join(L)
        ms = list(re.finditer(ws(old), text))
        if len(ms) != 1:
            sys.exit(f"{tag}: the anchor occurs {len(ms)} times in the brief: {old!r}")
        f = text.count("\n", 0, ms[0].start())
        assert text[ms[0].start() - 1] == "\n", (tag, "the anchor does not start its line")
        prefix, cont, e = unit_end(L, f)
        last = text.count("\n", 0, ms[0].end() - 1)
        assert last == e and L[e].endswith(old.split()[-1]), (tag, "the anchor is not its bullet whole")
        mk = MARK.match(new)
        assert mk and mk.group(0) == prefix, (tag, new[:4])
        out = refill(new[len(prefix):], prefix, cont)
        L[e + 1:e + 1] = out
        LOG.append(f"{tag}: a new bullet of {len(out)} lines after :{e + 1} (the bullet ending the anchor)")
        NEWS.append((tag, new))
    elif mode == "table":
        hits = [i for i, l in enumerate(L) if l == old]
        assert len(hits) == 1, (tag, len(hits))
        h = hits[0]
        assert L[h + 1] == "", (tag, "no blank line under the heading")
        add = [""]
        for l in new:
            add += [l] if (not l or l.startswith("|")) else refill(l, "", "")
        L[h + 1:h + 1] = add
        LOG.append(f"{tag}: {len(add)} lines after the heading :{h + 1} (the ruling's block :419-434)")
        NEWS += [(tag, l) for l in new if l]

# ---------------------------------------------------------------- 7.2 item 3: P1 to P5 and P8
rs, re_ = region()
REG = " ".join(l.strip() for l in L[rs:re_ + 1])


def inreg(start, end, within=None, after=None):
    t = REG if within is None else within
    i0 = t.index(after) if after else 0
    i = t.index(start, i0) + len(start)
    j = t.index(end, i)
    return t[i:j]


def bullet(name, nxt):
    return inreg(f"`<<{name}>>`", f"`<<{nxt}>>`")


# section 0's stamps of the four sources (lines and sha256 prefix)
S0 = norm(" ".join(L[[i for i, l in enumerate(L) if l.startswith("- `plan` is ")][0]:][:8]))
for key, (path, h) in SOURCES.items():
    m = re.search(rf"`{re.escape(key)}` is {re.escape(path.split('/')[-1])} \(([\d,]+) lines, sha256 ([0-9a-f]{{8}})…",
                  S0)
    assert m, (key, "no stamp in section 0")
    got = sha(path)
    assert got == h and h.startswith(m.group(2)), (key, got, m.group(2))
    with open(path, encoding="utf-8") as fh:
        assert fh.read().count("\n") == int(m.group(1).replace(",", "")), (key, "line count")
    LOG.append(f"source {key}: {path.split('/')[-1]} sha256 {got[:12]} (section 0: {m.group(2)}…, {m.group(1)} lines)")

FILLS = []  # (tag, kind, copy)

# P1: carries, by its heading, to the line before the next "## "
b1 = bullet("P1", "P2")
h1 = inreg('from the line beginning "', '"', b1)
h2 = inreg('to the line before the next line beginning "', '"', b1)
nums = re.findall(r"\d+", inreg("at line start are exactly ", ".", b1))
C = lines(SOURCES["carries"][0])
st = [i for i, l in enumerate(C) if l.startswith(h1)]
assert len(st) == 1, ("P1", len(st))
en = next(i for i in range(st[0] + 1, len(C)) if C[i].startswith(h2))
p1 = C[st[0]:en]
got_nums = [re.match(r"(\d+)\. ", l).group(1) for l in p1 if re.match(r"\d+\. ", l)]
assert got_nums == nums, ("P1", got_nums, nums)
FILLS.append(("P1", "block", p1, f"carries :{st[0] + 1}-{en} (entries {', '.join(got_nums)})"))

# P2: history, the line by its first words and its one continuation line; the line before it asserted
b2 = bullet("P2", "P3")
p2_first = inreg('the line beginning "', '"', b2)
p2_end = inreg('(ending "', '")', b2)
p2_prev = inreg('the line before it is "', '".', b2)
H = lines(SOURCES["history"][0])
st = [i for i, l in enumerate(H) if l.startswith(p2_first)]
assert len(st) == 1, ("P2", len(st))
i = st[0]
assert H[i - 1] == p2_prev, ("P2", H[i - 1])
assert indent(H[i + 1]) > indent(H[i]) and not MARK.match(H[i + 1]) and H[i + 2].lstrip().startswith("- "), "P2"
src2 = norm(H[i] + " " + H[i + 1])
assert src2.endswith(p2_end), ("P2", src2[-30:])
mk = MARK.match(H[i])
assert mk and src2.startswith(mk.group(2)), "P2"
p2 = src2[len(mk.group(2)):]        # the list marker is history's markup, not a word: the copy starts at the words
assert norm(p2_first).startswith(mk.group(2).strip()) and p2.startswith(norm(p2_first)[len(mk.group(2)):]), "P2"
FILLS.append(("P2", "inline", p2, f"history :{i + 1}-{i + 2} (its list marker left out)"))


def ranged(name, nxt, key, lead_first=True):
    b = bullet(name, nxt)
    m = re.search(r" :(\d+)-(\d+): ", b)
    a, z = int(m.group(1)), int(m.group(2))
    first = inreg('from "', '" to "', b)
    D = lines(SOURCES[key][0])
    j = " ".join(l.strip() for l in D[a - 1:z])
    assert j.count(first) == 1, (name, first)
    return b, a, z, j, first


# P3: df-ruling :462-464, from "Write files with the" to "whole with Write."
b3, a, z, j, first = ranged("P3", "P4", "df-ruling")
last = inreg('" to "', '".', b3)
assert j.count(last) == 1, ("P3", last)
p3 = j[j.index(first):j.index(last) + len(last)]
FILLS.append(("P3", "inline", p3, f"df-ruling :{a}-{z}"))

# P4: df-ruling :465-468, from "a failure in <crate>" to "and go on.", the two `<crate>` replacements (A3)
b4, a, z, j, first = ranged("P4", "P5", "df-ruling")
last = inreg('" to "', '", with', b4)
assert j.count(last) == 1, ("P4", last)
p4 = j[j.index(first):j.index(last) + len(last)]
tok = inreg("with the first `", "` replaced by \"", b4)
rep1 = inreg('` replaced by "', '" and "', b4)
tok2 = inreg('" and "', '" by "', b4, after="` replaced by \"")
rep2 = inreg('" by "', '" (the T4', b4, after="` replaced by \"")
assert all(x in A3_NEW for x in (tok, rep1, tok2, rep2)), "P4: the bullet's words are not A3's"
assert p4.count(tok) == 2 and p4.count(tok2) == 1 and tok2.startswith(tok), ("P4", p4.count(tok), p4.count(tok2))
p4 = p4.replace(tok2, rep2, 1)
assert p4.count(tok) == 1
p4 = p4.replace(tok, rep1, 1)
assert tok not in p4, "P4: a <crate> left"
FILLS.append(("P4", "inline", p4, f"df-ruling :{a}-{z}, two replacements"))

# P5: j-r2 :168-171, from "a FAIL in check.mjs's test step" to "until it is explained."
b5, a, z, j, first = ranged("P5", "P8", "j-r2")
last = inreg('" to "', '".', b5)
assert j.count(last) == 1, ("P5", last)
p5 = j[j.index(first):j.index(last) + len(last)]
FILLS.append(("P5", "inline", p5, f"j-r2 :{a}-{z}"))

# P8: j-r2 :298, the words after "From now on: " to the line's end
b8 = bullet("P8", "P6")
ln = int(re.search(r" :(\d+): ", b8).group(1))
lead = inreg('the words after "', '" to the line', b8)
D = lines(SOURCES["j-r2"][0])
assert D[ln - 1].count(lead) == 1 and sum(l.count(lead) for l in D) == 1, ("P8", lead)
p8 = D[ln - 1][D[ln - 1].index(lead) + len(lead):].strip()
FILLS.append(("P8", "inline", p8, f"j-r2 :{ln}"))

for tag, kind, copy, where in FILLS:
    rs, re_ = region()
    if kind == "block":
        hits = [k for k, l in enumerate(L) if l.startswith(f"<<{tag}:") and not rs <= k <= re_]
        assert len(hits) == 1, (tag, len(hits))
        s = hits[0]
        e = next(k for k in range(s, len(L)) if L[k].endswith(">>"))
        assert L[s - 1] == "```" and L[e + 1] == "```", (tag, "not in its fence")
        L[s:e + 1] = copy
        assert norm(" ".join(L[s:s + len(copy)])) == norm(" ".join(copy)), tag
        LOG.append(f"{tag}: lines :{s + 1}-{e + 1} of the brief's fence replaced by {where}, {len(copy)} lines as they are")
    else:
        edit(tag, f"`<<{tag}>>`", f'"{copy}"', outside=(rs, re_))
        LOG[-1] += f"; from {where}"
    rs, re_ = region()
    left = [k + 1 for k, l in enumerate(L) if (f"<<{tag}>>" in l or f"<<{tag}:" in l) and not rs <= k <= re_]
    assert not left, (tag, "the placeholder is still at", left)
    assert f"`<<{tag}>>`" in " ".join(L[rs:re_ + 1]), (tag, "7.2 item 3's record lost the name")
    flat = norm("\n".join(L))
    assert flat.count(norm(" ".join(copy) if kind == "block" else copy)) == 1, (tag, "the copy is not in once")

# ---------------------------------------------------------------- the title and the changes line
assert L[0].startswith("# U4d2 builder brief:") and L[0].count("(rev 1.1,") == 1, L[0]
L[0] = L[0].replace("(rev 1.1,", "(rev 1.2,")
tags = [t for t, _, _, _ in ITEMS] + [t for t, _, _, _ in FILLS]
note = ("Changes in rev 1.2 (the main session's fold of advisor-u4d2-brief.md, condition L1, by u4d2_fold.py): "
        + ", ".join(tags) + "; not used: block 6's 1.2 replacement (U4e has not merged).")
assert L[1] == ""
L[1:1] = refill(note, "", "")
LOG.append(f"title: rev 1.2; the changes line under it ({len(tags)} tags)")

# ---------------------------------------------------------------- checks
flat = norm("\n".join(L))
for tag, new in NEWS:
    if tag.startswith("P"):
        continue
    assert norm(new) in flat, (tag, "the new phrase is not in the brief", new[:60])
for tag, kind, copy, where in FILLS:
    assert norm(" ".join(copy) if kind == "block" else copy) in flat, tag
before = set(ORIG)
long = [k + 1 for k, l in enumerate(L) if len(l) > 120 and l not in before and not l.lstrip().startswith("|")]
assert not long, ("changed lines over 120 characters", long)
assert not re.search(r"<<P[123458]\b", "\n".join(L[:region()[0]] + L[region()[1] + 1:])), "a placeholder left"
print("\n".join(LOG))
text = "\n".join(L)
print(f"brief sha256 before {sha(BRIEF)}; lines {len(ORIG)} -> {len(L)}")
sys.stdout.writelines(difflib.unified_diff([l + "\n" for l in ORIG], [l + "\n" for l in L], BRIEF, BRIEF, n=1))
if APPLY:
    with open(BRIEF, "w", encoding="utf-8") as fh:
        fh.write(text)
    print(f"applied; brief sha256 after {sha(BRIEF)}")
else:
    print("dry run: nothing written")
