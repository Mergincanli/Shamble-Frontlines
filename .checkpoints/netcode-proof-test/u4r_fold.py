"""L1 of the advisor's check of the U4r builder brief (advisor-u4r-brief.md, condition L1, :416-420): fold the ruling
into u4r-brief.md (rev 3.1 -> rev 3.2).
Folded (L1's list): A1 (its three replacements, section 12 item 3's P16 line and section 0's three stamps), A2, A5, A6,
A7 and A8 as written; the brief words of U4R-Q7, Q8, Q9 (:231-249) and Q14 (:331-358) as written, in place; section
4's lane as the rulings leave it (:360-375: its bullet, three phrases of "Never" and section 12 item 3's P4 line);
section 10's "Answered" table gains twelve rows (Q1, Q2, Q4, Q6 to Q14, in that order); the fold recorded under the
title. Not folded here: A3 and A4 (their words are inside U4R-Q14's ruling); A9 (a record); U4R-Q9's plan text and
carry line and U4R-Q14's carry 74 words (the main session's L2 commit); the placeholders and values (section 12 items
1 to 3, a later script).
Every old phrase, span end, anchor and new phrase given by the ruling is read from the ruling's own lines (each item's
line range and its markers below; each line break and the next line's indent read as one space), never typed here.
The few brief anchors the ruling does not give (section 0's last stamp, the P4 line's old span, the "**Open**" label,
the R table's rows by their first two cells, the P13-P15 bullet) are the brief's own words, each asserted once, and the
words they bring (the main session's: MAIN below) are listed in the log. Each old phrase, span end or anchor must occur
once in the brief at its turn and once in rev 3.1, line breaks and the next line's indent read as one space, with no
two items' sites overlapping in rev 3.1; a miss stops the script with its name. A table row is edited in its own line;
any other paragraph, bullet or item is rewrapped within 120 characters from its first changed line to its end (a line
opening "(i) ", "(a) " or "(1) " starts a unit of its own), its earlier lines untouched. Last, the words check: the
folded brief with white space collapsed equals rev 3.1's with only these changes made, each new phrase is found once,
each replaced old phrase is gone, and no changed line passes 120 characters (table rows exempt).
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_fold.py [--apply]
Without --apply it writes nothing and prints one line per item (ruling lines to brief lines) and the unified diff."""
import difflib
import hashlib
import re
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
APPLY = "--apply" in sys.argv
BRIEF = f"{N}/u4r-brief.md"
RULING = f"{N}/advisor-u4r-brief.md"
HASHES = {
    BRIEF: "bcaef38322375a74c902f998eef079b630e9525ae98550d84f7e318596e36e2e",
    RULING: "35f281bfc11c453fb9a5054ecc6e433d4bf9c6509e668f0a28128959feb6ca37",
}
# the closed records A1's last bullet stamps into section 0, by its names
STAMPED = {
    "te-m3": (f"{N}/advisor-te-m3.md", "5ecc3e233ea22b64a8573d3c62a54f2b1a0206a726f72cfa81f10a7cf7055c48"),
    "te-m3-check": (f"{N}/advisor-te-m3-check.md", "1ffd02945dc605e63d22d8733633dda388cd3db84db9430bc36a5328b6760445"),
}


def sha(path):
    with open(path, "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()


def lines(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read().split("\n")


for p, h in list(HASHES.items()) + list(STAMPED.values()):
    got = sha(p)
    if got != h:
        sys.exit(f"{p}: sha256 {got}, expected {h}: stop")
SRC = lines(RULING)
assert len(SRC) == 464 and SRC[-1] == "", ("the ruling is not 463 lines", len(SRC))
L = lines(BRIEF)
assert len(L) == 1626 and L[-1] == "", ("the brief is not 1,625 lines", len(L))
ORIG = list(L)
OTEXT = "\n".join(ORIG)
LOG, MAIN, FLAT, NEWS, GONE = [], [], [], [], []


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


PAIR = re.compile(r'"([^"]+)" → "([^"]+)"')


def pairs(a, b, n):
    """The `"old" → "new"` pairs in the ruling's lines a..b (joined), asserted n."""
    ps = PAIR.findall(joined(a, b))
    assert len(ps) == n, (a, b, len(ps), n)
    return ps


# ---------------------------------------------------------------- editing the brief
def norm(t):
    return " ".join(t.split())


def ws(t):
    return r"\s+".join(re.escape(w) for w in t.split())


MARK = re.compile(r"^( *)(- |\d+\. )")
STRUCT = re.compile(r"^(- |\d+\. |#|\||```|\*\*|<<)")
SUB = re.compile(r"^\((?:[ivx]+|[a-h]|\d+)\) ")
RISKY = re.compile(r"^(\d+[.)](\s|$)|[-*+](\s|$)|#|>|```|~~~|\||=+\s*$|\*\*|<<|\((?:[ivx]+|[a-h]|\d+)\)(\s|$))")
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
    """The prefix, continuation indent and last line of the unit (paragraph, bullet, item, or a "(i) " part inside
    one) that line f is in, counted from line f."""
    m = MARK.match(D[f])
    prefix = m.group(0) if m else " " * indent(D[f])
    cont = " " * len(prefix)
    e = f
    while e + 1 < len(D) and D[e + 1].strip() and indent(D[e + 1]) == len(cont) \
            and not STRUCT.match(D[e + 1].lstrip(" ")) and not SUB.match(D[e + 1].lstrip(" ")):
        e += 1
    return prefix, cont, e


def once(tag, phrase, text=None, what="old phrase"):
    """The one match of `phrase` (white space read as one space) in `text` (the brief now), or a stop naming it."""
    text = "\n".join(L) if text is None else text
    ms = list(re.finditer(ws(phrase), text))
    flat = norm(text).count(norm(phrase))
    if len(ms) != 1 or flat != 1:
        sys.exit(f"{tag}: the {what} occurs {len(ms)} times in the brief ({flat} flattened): {phrase!r}")
    return ms[0]


def edit(tag, old, new, mode="replace"):
    """mode replace: old -> new; mode after: `new` inserted right after the anchor `old`. Returns how it was done."""
    global L
    text = "\n".join(L)
    m = once(tag, old, text, "old phrase" if mode == "replace" else "anchor")
    before = norm(text)
    if mode == "after":
        if not new.startswith(" "):
            new = " " + new
        cp, tail, e0 = m.end(), new, m.end()
        want = norm(text[:m.end()] + new + text[m.end():])
        FLAT.append((norm(old), norm(old + new)))
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
        assert want == before.replace(norm(old), norm(new), 1), (tag, "the flattened replacement differs")
        FLAT.append((norm(old), norm(new)))
    f = text.count("\n", 0, cp)
    last = text.count("\n", 0, max(e0 - 1, cp))
    NL = (text[:cp] + tail + text[e0:]).split("\n")
    delta = len(NL) - len(L)
    if L[f].lstrip().startswith("|"):
        assert last == f and delta == 0, (tag, "a row edit runs past its row")
        how = f"row :{f + 1} edited in its line"
    else:
        prefix, cont, e = unit_end(L, f)
        assert last <= e, (tag, "the match runs past its unit", f + 1, last + 1, e + 1)
        assert NL[f].startswith(prefix), (tag, NL[f][:20])
        blk = NL[f:e + 1 + delta]
        body = " ".join([blk[0][len(prefix):].strip()] + [x.strip() for x in blk[1:]])
        out = refill(body, prefix, cont)
        assert norm(" ".join(out)) == norm(" ".join(blk)), tag
        NL[f:e + 1 + delta] = out
        how = f"unit :{f + 1}-{e + 1} rewrapped from :{f + 1} ({e - f + 1} -> {len(out)} lines)"
    got = norm("\n".join(NL))
    assert got == want, (tag, "a word outside the edit changed")
    L = NL
    NEWS.append((tag, (old + new) if mode == "after" else new))
    if mode == "replace" and norm(old) not in norm(new):
        GONE.append((tag, old))
    return how


def span_old(tag, A, B, text):
    """The brief's text of the span from the start of `A` to the end of `B` (each found once, A before B)."""
    a = once(tag, A, text, "span start")
    b = once(tag, B, text, "span end")
    assert a.start() < b.start() and a.end() <= b.end(), (tag, "the span's ends are out of order")
    return text[a.start():b.end()]


# ---------------------------------------------------------------- the items
ITEMS = []  # dicts: tag, src (ruling lines), kind and its payload


def item(tag, src, kind, **kw):
    ITEMS.append(dict(tag=tag, src=src, kind=kind, **kw))


def R(tag, a, b, old, new):
    item(tag, (a, b), "replace", old=old, new=new)


# A1 (:78-105)
A, B, new = cut(85, 93, 'Replace the span from "', '" to "', '" with: "', LAST)
item("A1.1", (85, 93), "span", A=A, B=B, new=new)                       # section 4's TE-M3 line, with "<<P16>>"
R("A1.2", 94, 98, *cut(94, 98, 'Replace "', '" with "', LAST))          # section 4: P12's reading, TE-Q2's end
A, B, new = cut(99, 101, 'replace the span from "', '" to "', '" with "', LAST)
item("A1.3", (99, 101), "span", A=A, B=B, new=new)                      # section 12 item 5
(P16,) = cut(102, 105, 'Section 12 item 3 gains: "', '" Section 0\'s stamps gain ')
P16 = P16.replace('\\"', '"')
assert "\\" not in P16 and P16.startswith("`<<P16>>` "), P16[:20]
# the P16 line's two ends, each found once in the lines of advisor-te-m3.md it names, and its sha256 prefix
m16 = re.match(r"`<<P16>>` (\S+) :(\d+)-(\d+): .*?from \"(.+?)\" to \"(.+?)\" "
               r"\(each found once; sha256 ([0-9a-f]{8})…", P16)
assert m16 and m16.group(1) == STAMPED["te-m3"][0].split("/")[-1], P16
tj = " ".join(l.strip() for l in lines(STAMPED["te-m3"][0])[int(m16.group(2)) - 1:int(m16.group(3))])
assert tj.count(m16.group(4)) == 1 and tj.count(m16.group(5)) == 1, ("P16's ends in advisor-te-m3.md", m16.groups())
assert STAMPED["te-m3"][1].startswith(m16.group(6)), ("P16's sha256", m16.group(6))
item("A1.4", (102, 104), "bullet", anchor="   - `<<P13>>`, `<<P14>>`, `<<P15>>`:", new=P16, prefix="   - ")
stamps = cut(102, 105, "Section 0's stamps gain ", ", ", " and ", ", this file (its sha256 at the fold).")
for s in stamps[:2]:
    ms = re.fullmatch(r"`([a-z0-9-]+)` (\S+) \(([0-9a-f]{8})…\)", s)
    assert ms and ms.group(1) in STAMPED, s
    path, h = STAMPED[ms.group(1)]
    assert ms.group(2) == path.split("/")[-1] and h.startswith(ms.group(3)), (s, h[:8])
assert stamps[2] == "`u4r-check`", stamps[2]
u4r_stamp = f"{stamps[2]} {RULING.split('/')[-1]} ({sha(RULING)[:8]}…)"
last_stamp = re.search(r"(`m9` advisor-u4d2-m9\.md \([0-9a-f]{8}…\))\. All of these are in", norm(OTEXT)).group(1)
R("A1.5", 104, 105, last_stamp + ".", last_stamp + ", " + ", ".join(stamps[:2] + [u4r_stamp]) + ".")
MAIN.append("A1.5: section 0's three stamps join the list after its last (`m9`) with commas, the list's own form; "
            f"`u4r-check` is written in the stamps' form, \"{u4r_stamp}\" (the ruling: \"this file (its sha256 at the "
            "fold)\")")
MAIN.append("A1.4: the P16 line goes in section 12 item 3 as a bullet after the P13 to P15 bullet and before the P6 "
            "and P7 values bullet (the text placeholders in their numbers' order), indented as its neighbours")
# A2 (:107-131)
for k, (a, b) in enumerate([(112, 115), (116, 117), (118, 118), (119, 119), (120, 121), (122, 123), (124, 128)], 1):
    ((o, n),) = pairs(a, b, 1)
    R(f"A2.{k}", a, b, o, n)
o, x, y = cut(129, 130, 'Section 12 item 1: "', '" → the same with ', " for ", ".")
assert o.count(y) == 1 and x not in o, ("A2.8", o, x, y)
R("A2.8", 129, 130, o, o.replace(y, x))
((o, n),) = pairs(131, 131, 1)
R("A2.9", 131, 131, o, n)
# A5 to A8 (:149-167)
for k, (o, n) in enumerate(pairs(149, 151, 2), 1):
    R(f"A5.{k}", 149, 151, o, n)
((o, n),) = pairs(153, 154, 1)
R("A6", 153, 154, o, n)
for k, (o, n) in enumerate(pairs(156, 160, 2), 1):
    R(f"A7.{k}", 156, 160, o, n)
((o, n),) = pairs(162, 167, 1)
R("A8", 162, 167, o, n)
# U4R-Q7 and Q8 (:218-226)
R("Q7", 218, 220, *cut(218, 220, 'Replace "', '" with "', LAST))
o, n, row8 = cut(222, 226, 'Replace "', '" with "', '"; 7.3\'s row "', '" is removed')
R("Q8.1", 222, 224, o, n)
item("Q8.2", (225, 226), "rowdel", row=row8)
# U4R-Q9's brief words (:231-249); its plan text and carry line (:250-262) are L2's
k9 = 0
for a, b, n in ((232, 234, 2), (235, 236, 1), (237, 239, 1), (240, 241, 1), (242, 243, 2), (244, 249, 4)):
    for o, nn in pairs(a, b, n):
        k9 += 1
        R(f"Q9.{k9}", a, b, o, nn)
for row in cut(244, 249, 'the R table\'s rows → "', '", "', '", "', '", "', '"; "the warm-ups'):
    k9 += 1
    pre = re.match(r"^(\|[^|]+\|[^|]+\|)", row).group(1)
    hits = [l for l in ORIG if l.startswith(pre)]
    assert len(hits) == 1, (f"Q9.{k9}", pre, len(hits))
    R(f"Q9.{k9}", 247, 248, hits[0], row)
    MAIN.append(f"Q9.{k9}: the R table's row is found by its first two cells, {pre!r} (the ruling gives the new rows "
                "only: \"the R table's rows →\")")
# U4R-Q14's brief words (:331-358); its carry 74 words (:324-330) are L2's
A, B, n21, a22, i22, o74, n74 = cut(331, 344, 'replace the span from "', '" to "', '" with "', '" 2.2: after "',
                                    '" add "', '" 3.2\'s row 74: "', '" → "', LAST)
item("Q14.1", (331, 339), "span", A=A, B=B, new=n21)                    # 2.1's lead bullet
item("Q14.2", (339, 342), "after", anchor=a22, new=i22)                 # 2.2's press rule
R("Q14.3", 342, 344, o74, n74)                                          # 3.2's row 74
A, B, new = cut(345, 356, 'replace the span from "', '" to "', '" with "', LAST)
item("Q14.4", (345, 356), "span", A=A, B=B, new=new)                    # D13, widened
# section 4's lane (:360-375)
(A, B, nlane, n1o, n1n, n2o, n2n, n3o, n3n, p4) = cut(
    360, 375, 'Replace the bullet from "', '" to "', '" with "', '" In "Never": "', '" → "', '"; "', '" → "',
    '"; "', '" → "', '". P4\'s replacement: "', '" (both of section 12')
item("S4.1", (360, 368), "span", A=A, B=B, new=nlane)                   # the waiting bullet -> the opened bullet
R("S4.2", 368, 369, n1o, n1n)                                           # "Never": crates/server
R("S4.3", 369, 370, n2o, n2n)                                           # "Never": crates/log
R("S4.4", 370, 371, n3o, n3n)                                           # "Never": marks/
P4A = '"npt-lab or npt-bots, in check.mjs\'s deps or wasm step, or in check.test.mjs"'
P4B = "if U4R-Q6 rules (a))"
item("S4.5", (371, 374), "span", A=P4A, B=P4B, new=f'"{p4}"')          # section 12 item 3's P4 line
MAIN.append(f"S4.5: section 12 item 3's P4 line \"reworded to that one string\": the span from {P4A} to {P4B!r} (the "
            "old string and its two optional clauses) becomes the ruling's string in quotation marks")


# section 10's "Answered" table: twelve rows, in question order
def ruling_par(n):
    """The ruling's paragraph for U4R-Qn: its first and last line (to the line before the next blank line)."""
    s = [i for i, l in enumerate(SRC) if l.startswith(f"**U4R-Q{n} (")]
    assert len(s) == 1, (n, len(s))
    e = next(i for i in range(s[0], len(SRC)) if not SRC[i].strip()) - 1
    return s[0] + 1, e + 1


QORDER = (1, 2, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14)
# the main session's words: where an in-place ruling's words went (its Ruling cell), and every row's third cell
WENT = {
    7: "Its words applied in place: 7.1's -17 L1 P0 row.",
    8: "Its words applied in place: 7.1's -17 L1 B1 row; 7.3's -18 measured row removed.",
    9: "Its words applied in place: 2.1's recordings with (i) and (ii), section 5 item 4, section 6 item 9, 7.1's "
       "order and 7.3; its plan text and §U4h carry line are the main session's L2 commit.",
    14: "Its words applied in place: 2.1's lead, 2.2's press rule, 3.2's row 74 and D13; carry 74's amended words are "
        "the main session's L2 commit.",
}
WHERE = {
    1: "section 4's lane; 2.2", 2: "section 4's lane; 2.1's sizes; 2.5; 3.2's row 62; 7.1", 4: "2.4 r2; 2.2",
    6: "section 4's lane; 2.5; 7.2's bites; section 12 item 3 (P4)", 7: "7.1", 8: "7.1; 7.3",
    9: "2.1; section 5 item 4; section 6 item 9; 7.1; 7.3", 10: "2.4 r7; 7.2", 11: "2.4 r2",
    12: "8.3; 8.4; 3.2's row 40", 13: "section 4's lane; 2.6; 3.2's row 63; section 12 item 3 (P4)",
    14: "2.1; 2.2; 3.2's row 74; D13",
}
ROWS, ROWLOG = [], []
for n in QORDER:
    a, b = ruling_par(n)
    j = joined(a, b)
    m = re.match(rf"^\*\*U4R-Q{n} \((.+?)\): (.+?)\*\*(.*)$", j)
    assert m, (n, j[:60])
    letter, rest = m.group(2), m.group(3).strip()
    if n in WENT:
        st = re.search(r"\. (?=[A-Z])", rest)
        assert st, (n, "no first sentence")
        body = f"{letter} {rest[:st.start() + 1]} {WENT[n]}"
    else:
        body = f"{letter} {rest}"
    body = body.replace("|", "\\|")
    asked = [l for l in ORIG if l.startswith(f"**U4R-Q{n} (")]
    assert len(asked) == 1, (n, len(asked))
    q = re.match(rf"^\*\*(U4R-Q{n} \(.+?\))\.?\*\*", asked[0]).group(1)
    ROWS.append(f"| {q} | {body} (u4r-check :{a}-{b}) | {WHERE[n]} |")
    form = "its letter, first sentence and where its words went" if n in WENT else "its letter and its whole text"
    ROWLOG.append(f"  S10 row U4R-Q{n}: ruling :{a}-{b} ({form}); asked at brief rev 3.1 :{ORIG.index(asked[0]) + 1}")
item("S10", (176, 375), "rows", anchor="| Question | Ruling | Where the brief now stands |", rows=ROWS)
MAIN.append("S10: each Ruling cell starts with the ruled letter as the ruling's bold label gives it (after its "
            "title's \"): \", the bold marks left out), so the row says what was ruled, then the text after the label; "
            "the cite (u4r-check :a-b) ends that cell; for Q7, Q8, Q9 and Q14 the sentence \"Its words applied in "
            "place: ...\" (WENT); every third cell (WHERE)")
item("S10.open", None, "replace", old="**Open** (each self-contained;",
     new="**As asked in rev 3.1, each ruled since (the table above)** (each self-contained;")
MAIN.append("S10.open: the label \"**Open**\" above the questions becomes \"**As asked in rev 3.1, each ruled since "
            "(the table above)**\": all twelve are ruled, and the questions stay below as asked, for the record "
            "(ruling :177)")
item("title", None, "title")
item("record", None, "para")
MAIN.append("title: \"rev 3.1\" -> \"rev 3.2\"; the record paragraph after the title block's paragraph (RECORD)")

# ---------------------------------------------------------------- rev 3.1: each site once, none overlapping
SPANS = []
for it in ITEMS:
    k = it["kind"]
    if k in ("replace", "after"):
        m = once(it["tag"], it.get("old") or it.get("anchor"), OTEXT)
        s, e = m.start(), m.end()
    elif k == "span":
        a = once(it["tag"], it["A"], OTEXT, "span start")
        b = once(it["tag"], it["B"], OTEXT, "span end")
        assert a.start() < b.start(), it["tag"]
        s, e = a.start(), b.end()
    elif k == "rowdel":
        hits = [i for i, l in enumerate(ORIG) if l == it["row"]]
        assert len(hits) == 1, (it["tag"], len(hits))
        s = len("\n".join(ORIG[:hits[0]])) + 1
        e = s + len(it["row"])
    elif k in ("rows", "bullet"):
        hits = [i for i, l in enumerate(ORIG) if l.startswith(it["anchor"])]
        assert len(hits) == 1, (it["tag"], len(hits))
        s = e = len("\n".join(ORIG[:hits[0]])) + 1
    else:
        s = e = 0 if k == "title" else len("\n".join(ORIG[:2])) + 1
    it["rev31"] = (OTEXT.count("\n", 0, s) + 1, OTEXT.count("\n", 0, max(e - 1, s)) + 1)
    if k not in ("rows", "bullet", "title", "para"):
        SPANS.append((s, e, it["tag"]))
SPANS.sort()
for (a0, a1, ta), (b0, b1, tb) in zip(SPANS, SPANS[1:]):
    assert a1 <= b0, ("sites overlap in rev 3.1", ta, tb)

# ---------------------------------------------------------------- applying them, in the ruling's order
for it in ITEMS:
    tag, k = it["tag"], it["kind"]
    if k == "replace":
        how = edit(tag, it["old"], it["new"])
    elif k == "after":
        how = edit(tag, it["anchor"], it["new"], "after")
    elif k == "span":
        old = span_old(tag, it["A"], it["B"], "\n".join(L))
        how = "span " + edit(tag, old, it["new"])
    elif k == "rowdel":
        hits = [i for i, l in enumerate(L) if l == it["row"]]
        assert len(hits) == 1 and L[hits[0] - 1].startswith("|") and L[hits[0] + 1].startswith("|"), tag
        del L[hits[0]]
        FLAT.append((norm(it["row"]), ""))
        GONE.append((tag, it["row"]))
        how = f"row :{hits[0] + 1} removed"
    elif k == "rows":
        hits = [i for i, l in enumerate(L) if l == it["anchor"]]
        assert len(hits) == 1, (tag, len(hits))
        e = hits[0]
        while L[e + 1].startswith("|"):
            e += 1
        assert L[e + 1] == "", (tag, "no blank line under the table")
        FLAT.append((norm(L[e]), norm(L[e] + " " + " ".join(it["rows"]))))
        L[e + 1:e + 1] = it["rows"]
        NEWS += [(tag, r) for r in it["rows"]]
        how = f"{len(it['rows'])} rows after the table's last row :{e + 1}"
    elif k == "bullet":
        hits = [i for i, l in enumerate(L) if l.startswith(it["anchor"])]
        assert len(hits) == 1, (tag, len(hits))
        prefix, cont, e = unit_end(L, hits[0])
        assert prefix == it["prefix"] and L[e + 1].startswith(it["prefix"]), (tag, prefix, L[e + 1][:10])
        out = refill(it["new"], prefix, cont)
        FLAT.append((norm(" ".join(L[hits[0]:e + 1])), norm(" ".join(L[hits[0]:e + 1]) + " " + " ".join(out))))
        L[e + 1:e + 1] = out
        NEWS.append((tag, it["new"]))
        how = f"a new bullet of {len(out)} lines after :{e + 1} (the bullet at :{hits[0] + 1})"
    elif k == "title":
        assert L[0].startswith("# U4r builder brief:") and L[0].count("(rev 3.1,") == 1, L[0]
        new0 = L[0].replace("(rev 3.1,", "(rev 3.2,")
        FLAT.append((norm(L[0]), norm(new0)))
        NEWS.append((tag, new0))
        L[0] = new0
        how = "the title's rev 3.1 -> rev 3.2"
    elif k == "para":
        assert L[1] == "" and L[2].startswith("Rev 2 was the night shift's"), L[2][:30]
        e = next(i for i in range(2, len(L)) if not L[i].strip()) - 1
        assert L[e + 2].startswith("## Changes in rev 3.1"), L[e + 2][:30]
        RECORD = ("Rev 3.2 is the main session's fold of the advisor's check of rev 3.1 (advisor-u4r-brief.md, sha256 "
                  f"{sha(RULING)[:8]}…: APPROVE WITH CONDITIONS, L1 to L6), condition L1, by u4r_fold.py. Folded as "
                  "written: A1 (with section 12 item 3's P16 line and section 0's three new stamps), A2 and A5 to A8; "
                  "U4R-Q7's, Q8's, Q9's and Q14's brief words in place; section 4's lane as the rulings leave it, with "
                  "section 12 item 3's P4 line. Section 10's \"Answered\" table gains a row for each of the twelve "
                  "rulings (U4R-Q1, Q2, Q4 and Q6 to Q14), its third cell the main session's; the questions stay "
                  "below it as asked, relabelled. Not folded here: A3 and A4 (their words are inside U4R-Q14's "
                  "ruling); A9 (a record: no brief change); the plan and carries text (the main session's L2 commit); "
                  "the placeholders and values (section 12 items 1 to 3, next).")
        out = refill(RECORD, "", "")
        FLAT.append((norm(" ".join(L[2:e + 1])), norm(" ".join(L[2:e + 1]) + " " + RECORD)))
        L[e + 1:e + 1] = [""] + out
        NEWS.append((tag, RECORD))
        how = f"a new paragraph of {len(out)} lines after :{e + 1} (the title block's paragraph :3-{e + 1})"
    src = f"ruling :{it['src'][0]}-{it['src'][1]}" if it["src"] else "the main session's words"
    LOG.append(f"{tag}: {src} -> brief rev 3.1 :{it['rev31'][0]}-{it['rev31'][1]}; {how}")
    if k == "rows":
        LOG.extend(ROWLOG)

# ---------------------------------------------------------------- the words check
final = "\n".join(L)
flat = norm(final)
exp = norm(OTEXT)
for o, n in FLAT:
    assert exp.count(o) == 1, ("words check: an old phrase is not once at its turn", o[:60])
    exp = norm(exp.replace(o, n, 1))
assert exp == flat, "words check: the folded brief's words differ from rev 3.1's with only these changes made"
for tag, new in NEWS:
    assert flat.count(norm(new)) == 1, (tag, "the new phrase is not in the folded brief once", flat.count(norm(new)),
                                        new[:60])
for tag, old in GONE:
    assert norm(old) not in flat, (tag, "the replaced old phrase is still in the brief", old[:60])
before = set(ORIG)
long = [k + 1 for k, l in enumerate(L) if len(l) > 120 and l not in before and not l.lstrip().startswith("|")]
assert not long, ("changed lines over 120 characters", long)
assert final.count("<<P16>>") == 2 and final.count('"<<P16>>"') == 1 and final.count("`<<P16>>`") == 1, "P16"
assert L[0].count("(rev 3.2,") == 1
LOG.append(f"words check: {len(FLAT)} changes, rev 3.1's words with only these made equal the folded brief's; "
           f"{len(NEWS)} new phrases each once; {len(GONE)} replaced phrases gone; no changed line over 120 (rows "
           "exempt)")
print("\n".join(LOG))
print("the main session's words (not given word for word by the ruling):")
print("\n".join(f"- {m}" for m in MAIN))
print(f"brief sha256 before {sha(BRIEF)}; lines {len(ORIG) - 1} -> {len(L) - 1}")
sys.stdout.writelines(difflib.unified_diff([l + "\n" for l in ORIG], [l + "\n" for l in L], BRIEF, BRIEF, n=1))
if APPLY:
    with open(BRIEF, "w", encoding="utf-8") as fh:
        fh.write(final)
    print(f"applied; brief sha256 after {sha(BRIEF)}")
else:
    print("dry run: nothing written")
