"""Apply the merge commit's text of the advisor's U4j round-2 ruling (advisor-u4j-r2.md: B5, B7.2 to B7.5) by line
range. Every block and quoted string is read from the rulings' own lines (advisor-u4j-r2.md; advisor-u4j.md for R3
and R5's quote), never retyped; anchors are checked unique; a prose paragraph is refilled to 120 characters and its
words are checked equal to the old words with only the substitutions.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4j_merge_text.py [--apply]
Without --apply it changes nothing and prints what it would do."""
import difflib
import hashlib
import re
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
APPLY = "--apply" in sys.argv


def read(name):
    with open(f"{N}/{name}", encoding="utf-8") as fh:
        return fh.read().split("\n")


R2 = read("advisor-u4j-r2.md")
R1 = read("advisor-u4j.md")


def rng(src, a, b):
    """Lines a..b (1-based, inclusive), as they are."""
    return src[a - 1:b]


def one(src, a):
    """The one line of a fenced block at line a, as it is (the fence lines checked)."""
    assert src[a - 2].startswith("```") and src[a] == "```", (a, src[a - 2], src[a])
    return src[a - 1]


def joined(src, a, b):
    return " ".join(l.strip() for l in src[a - 1:b])


def between(src, a, b, start, end):
    j = joined(src, a, b)
    assert j.count(start) == 1, (a, b, start)
    i0 = j.index(start) + len(start)
    return j[i0:j.index(end, i0)]


FILES, ORIG, LOG, NEW = {}, {}, [], []


def doc(name):
    if name not in FILES:
        FILES[name] = read(name)
        ORIG[name] = list(FILES[name])
    return FILES[name]


def entry_span(d, number):
    starts = [i for i, l in enumerate(d) if l.startswith(f"{number}. **")]
    assert len(starts) == 1, (number, starts)
    s = e = starts[0]
    while e + 1 < len(d) and d[e + 1].startswith("    "):
        e += 1
    return s, e


def section_of(d, i):
    return next(d[j] for j in range(i, -1, -1) if d[j].startswith("## "))


def entry_lines(a, b, number):
    """Entry `number` of the ruling, lines a..b, as they are: its first line numbered, the rest indented four
    spaces, a blank line on each side."""
    lines = rng(R2, a, b)
    assert lines[0].startswith(f"{number}. **"), (number, lines[0][:20])
    assert all(l.startswith("    ") for l in lines[1:]), number
    assert R2[a - 2] == "" and R2[b] == "", number
    return lines


RISKY = re.compile(r"^(\d+[.)](\s|$)|[-*+](\s|$)|#|>|```|~~~|\||=+\s*$)")
NBSP = " "


def refill(text, first, cont):
    """`text` wrapped to 120 characters; no continuation line may start with a token that would begin a block."""
    t = text
    for _ in range(20):
        w = textwrap.TextWrapper(width=120, initial_indent=first, subsequent_indent=cont, break_long_words=False,
                                 break_on_hyphens=False)
        lines = w.wrap(t)
        bad = [k for k in range(1, len(lines)) if RISKY.match(lines[k][len(cont):])]
        if not bad:
            return [l.replace(NBSP, " ") for l in lines]
        k = bad[0]
        tok = lines[k][len(cont):].split(" ")[0]
        prev = lines[k - 1].rstrip().split(" ")[-1]
        pair = f"{prev} {tok}"
        assert t.count(pair) >= 1, pair
        t = t.replace(pair, f"{prev}{NBSP}{tok}", 1)
    sys.exit(f"refill: no safe wrap for {text[:60]}")


def para_sub(name, heading, start, end, pairs, tag):
    """In the paragraph right under `heading` (from `start` to `end`), each old text found exactly once and replaced;
    the paragraph refilled; its words after equal to its words before with only the substitutions."""
    d = doc(name)
    h = [i for i, l in enumerate(d) if l == heading]
    assert len(h) == 1, (tag, heading, h)
    s = h[0] + 1
    assert d[s].startswith(start), (tag, d[s][:40])
    e = s
    while e + 1 < len(d) and d[e + 1].strip() and not d[e + 1].startswith(("#", "- ", "|")):
        e += 1
    text = " ".join(l.strip() for l in d[s:e + 1])
    assert text.endswith(end), (tag, text[-40:])
    new = text
    for o, n in pairs:
        c = new.count(o)
        assert c == 1, (tag, o[:50], c)
        new = new.replace(o, n)
    lines = refill(new, "", "")
    assert " ".join(lines).split() == new.split(), tag
    d[s:e + 1] = lines
    LOG.append(f"{tag}: {name}:{s + 1}-{e + 1} ({e - s + 1} lines) -> {len(lines)} lines, {len(pairs)} substitution(s)")
    for _, n in pairs:
        NEW.append((name, n))


C, BP, PL, UB = "u4s-carries.md", "u4-build-plan.md", "plan.md", "u4j-build.md"

# B7.2: u4s-carries.md, entries 61 to 65
d = doc(C)
for n in range(61, 66):
    assert not any(l.startswith(f"{n}. **") for l in d), f"entry {n} is taken"
e61 = entry_lines(447, 465, 61)
e62 = entry_lines(469, 490, 62)
e63 = entry_lines(492, 497, 63)
e64 = entry_lines(501, 526, 64)
e65 = entry_lines(530, 536, 65)
head64 = between(R2, 499, 499, 'after §U4r: "', '":**')
assert head64.startswith("## The next change to crates/judge"), head64
_, e = entry_span(d, 57)
assert section_of(d, e).startswith("## U4e"), section_of(d, e)
d[e + 1:e + 1] = e61
LOG.append(f"B7.2 entry 61: {C}, {len(e61)} lines after entry 57 (§U4e), from advisor-u4j-r2.md :447-465")
_, e = entry_span(d, 60)
assert section_of(d, e).startswith("## U4r") and d[e + 1] == "" and d[e + 2].startswith("## U4f"), d[e + 2][:20]
add = e62 + e63 + ["", head64] + e64
d[e + 1:e + 1] = add
LOG.append(f"B7.2 entries 62, 63: {C}, after entry 60 (§U4r), from :469-490 and :492-497; then the new section "
           f"'{head64}' (:499) with entry 64 (:501-526) before §U4f")
_, e = entry_span(d, 52)
assert section_of(d, e).startswith("## U4d2"), section_of(d, e)
d[e + 1:e + 1] = e65
LOG.append(f"B7.2 entry 65: {C}, {len(e65)} lines after entry 52 (§U4d2), from :530-536")
for blk in (e61, e62, e63, e64, e65):
    NEW.append((C, "\n".join(blk)))
NEW.append((C, head64))

# B7.3: u4-build-plan.md §4.1, U4j's public items (the merged list was re-derived by diff and equals the ruling's)
d = doc(BP)
old_row, new_row, judge_row = one(R2, 548), one(R2, 552), one(R2, 556)
hits = [i for i, l in enumerate(d) if l == old_row]
assert len(hits) == 1, hits
d[hits[0]:hits[0] + 1] = [new_row, judge_row]
LOG.append(f"B7.3: {BP}:{hits[0] + 1}, the npt-log row replaced (:552) and the npt-judge row added after it (:556)")
NEW.append((BP, new_row))
NEW.append((BP, judge_row))

# B7.5: u4-build-plan.md §10's preamble (the orphan reading, its quote copied from advisor-u4j.md :538-540) and §11
j = joined(R1, 538, 540)
quote = j[j.index("PM-0001-17's"):j.index("Stricter.") + len("Stricter.")]
b577 = one(R2, 577)
assert b577.count(quote) == 1, "the ruling's block does not hold R5's words exactly"
k = b577.index(quote)
new10 = b577[:k] + quote + b577[k + len(quote):]
para_sub(BP, "## 10. Questions for the advisor", "Plain words first", "new: Q23 to Q26.", [(one(R2, 573), new10)],
         "B7.5 §10's preamble")
para_sub(BP, "## 11. For Mustafa", "Nothing now.", "(plan.md:430-438).",
         [(one(R2, 582), one(R2, 586)), (one(R2, 590), one(R2, 594)), (one(R2, 598), one(R2, 602))], "B7.5 §11")

# B7.4: plan.md's U6 row, C-U6-9 to C-U6-11 after C-U6-8's last words, inside the cell
d = doc(PL)
anchor = between(R2, 564, 565, 'ends with "', '", the block')
blk = one(R2, 562)
hits = [i for i, l in enumerate(d) if l.startswith("| U6 |")]
assert len(hits) == 1, hits
row = d[hits[0]]
assert row.count(anchor) == 1 and row.endswith(anchor + " |"), row[-80:]
d[hits[0]] = row[:-len(" |")] + blk + " |"
LOG.append(f"B7.4: {PL}:{hits[0] + 1}, C-U6-9 to C-U6-11 (advisor-u4j-r2.md :562) after '...{anchor[-30:]}'")
NEW.append((PL, blk))

# B5: u4j-build.md, "## Readings, as ruled", copied by line range
d = doc(UB)
assert not any(l.startswith("## Readings, as ruled") for l in d)
assert d[-1] == "" and d[-2].strip(), "u4j-build.md's end"
r3 = rng(R1, 472, 506)
assert r3[0].startswith("## R3. ") and R1[506] == "" and R1[507].startswith("## R4"), r3[0]
b42, b43, n2 = rng(R2, 207, 218), rng(R2, 220, 231), rng(R2, 174, 177)
assert b42[0].startswith("### B4.2") and R2[218] == "" and b43[0].startswith("### B4.3") and R2[231] == ""
assert n2[0].startswith("- **NC-U4J-R2-N2**") and R2[177].startswith("- **NC-U4J-R2-N3")
note = textwrap.wrap(
    "Copied by script (u4j_merge_text.py), by line range, as the advisor's U4j round-2 ruling asks (advisor-u4j-r2.md, "
    "B5; the main session's words in this paragraph only): advisor-u4j.md :472-506 (R3, whole; its heading one level "
    "down), then advisor-u4j-r2.md :207-218 (B4.2), :220-231 (B4.3) and :174-177 (B3's NC-U4J-R2-N2 line). Line "
    "numbers inside the copied text are the rulings' own.", width=120)
sec = ["", "## Readings, as ruled"] + note + [""] + ["#" + r3[0]] + r3[1:] + [""] + b42 + [""] + b43 + [""] + n2
d[-1:] = sec + [""]
LOG.append(f"B5: {UB}, '## Readings, as ruled' appended: R3 (advisor-u4j.md :472-506), B4.2 (:207-218), B4.3 "
           f"(:220-231), NC-U4J-R2-N2 (:174-177)")
for blk in (r3[1:], b42, b43, n2):
    NEW.append((UB, "\n".join(blk)))

# checks: every new text is in its file (whitespace aside); changed prose lines fit 120 characters
for name, text in NEW:
    flat = " ".join(" ".join(FILES[name]).split())
    assert " ".join(text.split()) in flat, (name, text[:60])
for name in FILES:
    diff = list(difflib.unified_diff(ORIG[name], FILES[name], n=0, lineterm=""))[2:]
    added = [l[1:] for l in diff if l.startswith("+") and not l.startswith("+++")]
    removed = [l[1:] for l in diff if l.startswith("-") and not l.startswith("---")]
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
