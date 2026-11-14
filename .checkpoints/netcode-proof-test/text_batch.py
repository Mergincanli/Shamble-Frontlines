"""Apply the advisor's text batch of 2026-11-11 (advisor-u4d-close.md: B1.2, B1.4-B1.6, B2, B3, B4) by line range.
Every block and every quoted string is read from the ruling's own lines, never retyped; anchors are checked unique.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/text_batch.py [--apply]
Without --apply it changes nothing and prints what it would do."""
import difflib
import hashlib
import re
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
APPLY = "--apply" in sys.argv
with open(f"{N}/advisor-u4d-close.md", encoding="utf-8") as fh:
    SRC = fh.read().split("\n")


def joined(a, b):
    return " ".join(l.strip() for l in SRC[a - 1:b])


def between(a, b, start, end, last=False):
    """The text between `start` and `end` in the ruling's lines a..b (1-based, inclusive), wrapped lines joined."""
    j = joined(a, b)
    assert j.count(start) == 1, (a, b, start, j.count(start))
    i0 = j.index(start) + len(start)
    i1 = j.rindex(end) if last else j.index(end, i0)
    assert i1 > i0, (a, b, start, end)
    return j[i0:i1]


def block(a, b):
    """A fenced block's lines a..b, as they are."""
    assert SRC[a - 2] == "```" and SRC[b] == "```", (a, b)
    return SRC[a - 1:b]


FILES, ORIG, LOG, NEW = {}, {}, [], []


def doc(name):
    if name not in FILES:
        with open(f"{N}/{name}", encoding="utf-8") as fh:
            FILES[name] = fh.read().split("\n")
        ORIG[name] = list(FILES[name])
    return FILES[name]


def inline(name, line, old, new, tag):
    d = doc(name)
    c = d[line - 1].count(old)
    if c != 1:
        hits = [i + 1 for i, l in enumerate(d) if old in l]
        sys.exit(f"{tag}: {name}:{line} holds the old text {c} times; lines holding it: {hits}")
    d[line - 1] = d[line - 1].replace(old, new)
    LOG.append(f"{tag}: {name}:{line}, {len(old)} chars -> {len(new)}")
    NEW.append((name, new))


def entry_span(d, number):
    starts = [i for i, l in enumerate(d) if l.startswith(f"{number}. **")]
    assert len(starts) == 1, (number, starts)
    s = e = starts[0]
    while e + 1 < len(d) and d[e + 1].startswith("    "):
        e += 1
    return s, e


def ws(text):
    return r"\s+".join(re.escape(w) for w in text.split())


WRAP = textwrap.TextWrapper(width=120, subsequent_indent="    ", break_long_words=False, break_on_hyphens=False)


def entry_sub(name, number, pattern, new, tag):
    """Replace the one match of `pattern` in entry `number` by `new`; rewrap from the match's first line to the
    entry's end (the lines before it stay as they are)."""
    d = doc(name)
    s, e = entry_span(d, number)
    text = "\n".join(d[s:e + 1])
    ms = list(re.finditer(pattern, text))
    if len(ms) != 1:
        sys.exit(f"{tag}: entry {number} matches {len(ms)} times")
    m = ms[0]
    first = text.count("\n", 0, m.start())
    lines = (text[:m.start()] + new + text[m.end():]).split("\n")
    WRAP.initial_indent = "" if first == 0 else "    "
    d[s:e + 1] = lines[:first] + WRAP.wrap(" ".join(x.strip() for x in lines[first:]))
    LOG.append(f"{tag}: {name} entry {number}, rewrapped from its line {first + 1}")
    NEW.append((name, new))


def append_after_entry(name, number, blk, tag):
    d = doc(name)
    _, e = entry_span(d, number)
    d[e + 1:e + 1] = blk
    LOG.append(f"{tag}: {name}, {len(blk)} lines after entry {number}")
    NEW.append((name, "\n".join(blk)))


C, BP, PL, RP, W1 = "u4s-carries.md", "u4-build-plan.md", "plan.md", "u4d-report.md", "advisor-u4-wave1.md"

# u4s-carries.md
entry_sub(C, 52, ws(between(140, 141, 'replace the span from "', '" to the carry')) + r"[\s\S]*\Z",
          " ".join(l.strip() for l in block(143, 149)), "B1.2 carry 52's tail")
entry_sub(C, 51, ws(between(233, 235, 'replace the span from "', '" through "')) + r"[\s\S]*?"
          + ws(between(233, 235, '" through "', '" with the block')),
          " ".join(l.strip() for l in block(237, 241)), "B2 carry 51's span")
append_after_entry(C, 51, block(250, 261) + block(266, 270) + block(275, 279) + block(287, 292) + block(297, 302),
                   "B2 carries 53-57 (U4e)")
append_after_entry(C, 49, block(356, 360) + block(157, 162) + block(376, 380), "B3/B1.2 carries 58, 59, 60 (U4r)")
entry_sub(C, 40, ws(between(322, 324, 'replace "', '" with "')), between(322, 324, '" with "', '". The VOID'),
          "F3 carry 40's text")
entry_sub(C, 40, ws(between(325, 327, 'replace "', '" with "')), between(325, 327, '" with "', '".', last=True),
          "F3 carry 40's label")
inline(C, 14, between(345, 347, ':14: replace "', '" with "'), between(345, 347, '" with "', '" (117'), "F7 line 14")
d = doc(C)
_, e13 = entry_span(d, 13)
assert e13 == 112 and d[e13].endswith('"'), (e13, d[e13][-20:])
WRAP.initial_indent = "    "
note = WRAP.wrap(between(348, 349, 'four spaces: "', '"', last=True))
d[e13 + 1:e13 + 1] = note
LOG.append(f"F7 carry 13: {C}, {len(note)} lines after its closing quote (:113)")
NEW.append((C, " ".join(note)))

# u4-build-plan.md
inline(BP, 184, between(201, 202, 'after "', '" insert'),
       between(201, 202, 'after "', '" insert') + between(201, 202, 'insert "', '".'), "B1.5 :184 (1)")
inline(BP, 184, between(203, 207, 'Replace "', '" with "'), between(203, 207, '" with "', '" (the second half'),
       "B1.5 :184 (2), F11")
inline(BP, 184, between(208, 210, 'after "', '" insert'),
       between(208, 210, 'after "', '" insert') + between(208, 210, 'insert "', '".', last=True), "B1.5 :184 (3)")
inline(BP, 185, between(211, 213, 'replace "', '" with "'), between(211, 213, '" with "', '". In its'), "B1.5 :185 (1)")
inline(BP, 185, between(211, 213, 'after "', '" insert'),
       between(211, 213, 'after "', '" insert') + between(211, 213, 'insert "', '" (F11)'), "B1.5 :185 (2), F11")
inline(BP, 192, between(333, 335, 'replace "', '" with "'), between(333, 335, '" with "', '". The table'), "F5 :192")
inline(BP, 230, between(338, 339, 'replace "', '" with "'), between(338, 339, '" with "', '"', last=True), "F6 :230")
inline(BP, 365, between(340, 341, 'replace "', '" with "'), between(340, 341, '" with "', '"', last=True), "F6 :365")
inline(BP, 247, between(315, 318, ':247, replace "', '" with "'), between(315, 318, '" with "', '". The line'),
       "F10 :247")
d = doc(BP)
assert d[492].startswith("- **Sources:** U4d's three synthetic recordings") and d[493].startswith("  since U4d2's step")
d[492:494] = block(366, 367)
LOG.append(f"F9: {BP}:493-494 replaced by the ruling's lines 366-367")
NEW.append((BP, "\n".join(block(366, 367))))

# plan.md
inline(PL, 1248, between(329, 331, 'replace "', '" with "'), between(329, 331, '" with "', '". The'), "F4 :1248")
inline(PL, 1424, between(402, 410, 'replace "', '" with: "'), between(402, 410, '" with: "', '".', last=True),
       "NA1 :1424 (R-31)")
inline(PL, 1248, between(411, 419, 'Replace "', '" with: "'), between(411, 419, '" with: "', '"', last=True),
       "NA1 :1248 (U3D-MISS's case)")
inline(PL, 1248, between(425, 429, 'after "', '" insert: "'),
       between(425, 429, 'after "', '" insert: "') + between(425, 429, '" insert: "', '"', last=True),
       "NA5 :1248 (C-U8-6)")
inline(PL, 1248, between(430, 431, 'replace "', '" with "'), between(430, 431, '" with "', '".', last=True),
       "NA5 :1248 (C-U8-4)")
inline(PL, 1248, between(436, 441, 'replace "', '" with "'), between(436, 441, '" with "', '".', last=True),
       "NA4 :1248 (run conditions)")
inline(PL, 1246, between(448, 454, 'replace "', '" with "'), between(448, 454, '" with "', '".', last=True),
       "TE2-TE4 :1246 (C-U6-8)")
inline(PL, 1246, between(382, 384, 'replace "', '" with "'), between(382, 384, '" with "', '".', last=True),
       "TUNE-1 :1246 (U6 header)")

# u4d-report.md: B1.6's bullet at the end of "Other findings, by route", and the main session's line on the answers
d = doc(RP)
h = d.index("## Other findings, by route")
nxt = next(i for i in range(h + 1, len(d)) if d[i].startswith("## "))
end = max(i for i in range(h, nxt) if d[i].strip())
d[end + 1:end + 1] = block(221, 224)
LOG.append(f"B1.6: {RP}, 4 lines after its line {end + 1}")
NEW.append((RP, "\n".join(block(221, 224))))
fa = next(i for i in range(h, nxt) if d[i].startswith("- **For the advisor:**"))
fe = fa
while d[fe + 1].startswith("  "):
    fe += 1
ans = WRAP.__class__(width=120, subsequent_indent="  ", break_long_words=False, break_on_hyphens=False).wrap(
    "- **The advisor's answers** (advisor-u4d-close.md, 2026-11-11): (A) the estimate scene's knobs are approved as "
    "stand-in tuning, with conditions (B1.1); N2-M1's fix and N2-M2's scene fix land as the main session's change "
    "U4d-F, before U4d2 pins anything (B1.2, B1.3; carry 52); (C) was a slip, and its erratum is appended to "
    "advisor-u4-wave1.md (B1.4); `Weapon`, `Scene::weapon` and `Scene::health` are approved for §4.1's npt-sim row "
    "(B1.5); TUNE-1's measured load has its own bullet in this section (B1.6).")
d[fe + 1:fe + 1] = ans
LOG.append(f"B1.5's report line: {RP}, {len(ans)} lines after 'For the advisor' (the main session's words)")

# advisor-u4-wave1.md: B1.4's erratum at the end, after a blank line
d = doc(W1)
assert d[-1] == "" and d[-2].strip()
d[-1:] = [""] + block(188, 192) + [""]
LOG.append(f"B1.4: {W1}, the erratum appended")
NEW.append((W1, "\n".join(block(188, 192))))

# checks: every new text is in its file (whitespace aside); changed prose lines fit 120 characters
for name, text in NEW:
    flat = " ".join(" ".join(FILES[name]).split())
    assert " ".join(text.split()) in flat, (name, text[:60])
for name in FILES:
    diff = list(difflib.unified_diff(ORIG[name], FILES[name], n=0, lineterm=""))[2:]
    added = [l[1:] for l in diff if l.startswith("+")]
    removed = [l[1:] for l in diff if l.startswith("-")]
    long = [l[:60] for l in added if not l.startswith("|") and len(l) > 120]
    before = hashlib.sha256("\n".join(ORIG[name]).encode()).hexdigest()[:8]
    after = hashlib.sha256("\n".join(FILES[name]).encode()).hexdigest()[:8]
    print(f"{name}: +{len(added)} -{len(removed)} lines; sha256 {before} -> {after}; over 120: {long}")
print("\n".join(LOG))
if APPLY:
    for name, d in FILES.items():
        with open(f"{N}/{name}", "w", encoding="utf-8") as fh:
            fh.write("\n".join(d))
    print("APPLIED")
else:
    print("dry run: nothing written")
