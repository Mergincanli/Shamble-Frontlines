"""Fold the advisor's check of the U4e brief (advisor-u4e-brief.md, about 20:43 UTC 2026-11-11: APPROVE on L1-L4) into
the brief (rev 3.2 -> rev 3.3; its condition L1), and make the plan and carries text of its condition L2 (u4-build-plan.md's
four edits; u4s-carries.md's entry 66). Every old phrase, new phrase, inserted text and new row is read from the ruling's
own lines by line range (whitespace joined), never retyped. Each old phrase or anchor must occur exactly once in the
flattened text of the file's prose blocks outside fenced blocks ("snapshot.rs:29-30" three times, as ruled); the block
it is in is edited and refilled to 120 characters with its own indents, its words checked equal to the old words with
only the change. 3.1's fenced copy of the carries is never touched, and is checked equal to carries :147-247 after.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4e_fold3.py [--apply]"""
import difflib
import re
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
APPLY = "--apply" in sys.argv


def read(name):
    with open(f"{N}/{name}", encoding="utf-8") as fh:
        return fh.read().split("\n")


R = read("advisor-u4e-brief.md")
FILES = {"u4e-brief.md": read("u4e-brief.md"), "u4-build-plan.md": read("u4-build-plan.md"),
         "u4s-carries.md": read("u4s-carries.md")}
ORIG = {k: list(v) for k, v in FILES.items()}
LOG, NEW = [], []


def joined(a, b):
    return " ".join(l.strip() for l in R[a - 1:b])


def between(a, b, start, end, last=False):
    """The text after `start` and before `end` in the ruling's lines a..b (wrapped lines joined); `last`: up to the
    last `end` in those lines."""
    j = joined(a, b)
    assert j.count(start) >= 1, (a, b, start)
    i0 = j.index(start) + len(start)
    i1 = j.rindex(end) if last else j.index(end, i0)
    assert i1 > i0, (a, b, start, end)
    return j[i0:i1]


def pair(a, b, start='replace "', mid='" with "'):
    """(old, new) of a 'replace "old" with "new"' bullet in lines a..b: new runs to the bullet's last quote."""
    j = joined(a, b)
    i0 = j.index(start) + len(start)
    i1 = j.index(mid, i0)
    return j[i0:i1], j[i1 + len(mid):j.rindex('"')]


NEXT = re.compile(r"^ *(- |\d+\. |#|\||```)")


def blocks(L):
    out, i, n = [], 0, len(L)
    while i < n:
        l = L[i]
        if not l.strip():
            i += 1
            continue
        if l.startswith("```"):
            j = i + 1
            while j < n and not L[j].startswith("```"):
                j += 1
            out.append((i, j, "fence", "", ""))
            i = j + 1
            continue
        if l.startswith("#"):
            out.append((i, i, "heading", "", ""))
            i += 1
            continue
        if l.lstrip().startswith("|"):
            out.append((i, i, "row", "", ""))
            i += 1
            continue
        m = re.match(r"^( *)(- |\d+\. )", l)
        if m:
            ind, mk = m.group(1), m.group(2)
            cont = " " * (len(ind) + len(mk))
            j = i
            while j + 1 < n and L[j + 1].strip() and L[j + 1].startswith(cont) and not NEXT.match(L[j + 1]):
                j += 1
            out.append((i, j, "item", ind + mk, cont))
            i = j + 1
            continue
        ind = " " * (len(l) - len(l.lstrip()))
        j = i
        while j + 1 < n and L[j + 1].strip() and not NEXT.match(L[j + 1]):
            j += 1
        out.append((i, j, "para", ind, ind))
        i = j + 1
    return out


def text(L, blk):
    s, e, kind, first, cont = blk
    if kind in ("row", "heading"):
        return L[s]
    return " ".join([L[s][len(first):].strip()] + [l.strip() for l in L[s + 1:e + 1]])


RISKY = re.compile(r"^(\d+[.)](\s|$)|[-*+](\s|$)|#|>|```|~~~|\||=+\s*$)")
NBSP = " "


def refill(t, first, cont):
    for _ in range(30):
        w = textwrap.TextWrapper(width=120, initial_indent=first, subsequent_indent=cont, break_long_words=False,
                                 break_on_hyphens=False)
        lines = w.wrap(t)
        bad = [k for k in range(1, len(lines)) if RISKY.match(lines[k][len(cont):])]
        if not bad:
            return [l.replace(NBSP, " ") for l in lines]
        k = bad[0]
        tok = lines[k][len(cont):].split(" ")[0]
        prev = lines[k - 1].rstrip().split(" ")[-1]
        t = t.replace(f"{prev} {tok}", f"{prev}{NBSP}{tok}", 1)
    sys.exit(f"refill: no safe wrap for {t[:60]}")


def put(L, blk, new_text, tag):
    s, e, kind, first, cont = blk
    if kind in ("row", "heading"):
        L[s] = new_text
        return
    lines = refill(new_text, first, cont)
    got = " ".join(l.strip() for l in lines)
    want = (first.strip() + " " + new_text).strip() if first.strip() else new_text
    assert got.split() == want.split(), tag
    L[s:e + 1] = lines


def edit(name, old, new, tag, count=1, mode="replace"):
    """mode replace: old -> new; mode after: insert `new` right after the anchor `old`."""
    L = FILES[name]
    total = 0
    while True:
        hits = [b for b in blocks(L) if b[2] != "fence" and old in text(L, b)]
        if not hits:
            break
        b = hits[0]
        t = text(L, b)
        total += t.count(old)
        t2 = t.replace(old, new) if mode == "replace" else t.replace(old, old + new)
        put(L, b, t2, tag)
        if mode == "after" or old in new:
            break
    assert total == count, (tag, total, count)
    if mode == "after":
        assert sum(text(L, b).count(old + new) for b in blocks(L) if b[2] != "fence") == 1, tag
    LOG.append(f"{tag}: {name}, {total} match(es)")
    NEW.append((name, new))


def add_after_block(name, start, new_lines, tag):
    L = FILES[name]
    hits = [b for b in blocks(L) if b[2] != "fence" and L[b[0]].startswith(start)]
    assert len(hits) == 1, (tag, start, len(hits))
    e = hits[0][1]
    L[e + 1:e + 1] = new_lines
    LOG.append(f"{tag}: {name}, {len(new_lines)} line(s) after :{e + 1}")
    NEW.append((name, " ".join(new_lines)))


def append_to_block(name, start, add, tag):
    L = FILES[name]
    hits = [b for b in blocks(L) if b[2] != "fence" and L[b[0]].startswith(start)]
    assert len(hits) == 1, (tag, start, len(hits))
    put(L, hits[0], text(L, hits[0]) + add, tag)
    LOG.append(f"{tag}: {name}, appended to the block at :{hits[0][0] + 1}")
    NEW.append((name, add))


B, P, C = "u4e-brief.md", "u4-build-plan.md", "u4s-carries.md"

# --- L1: the brief, rev 3.3 ---
edit(B, "# U4e builder brief: the room and the codec path (rev 3.2)",
     "# U4e builder brief: the room and the codec path (rev 3.3)", "title")
note = textwrap.wrap(
    "**The main session's fold of the advisor's check (2026-11-11 about 21:20 UTC; rev 3.3):** the advisor checked rev "
    "3.2 (advisor-u4e-brief.md: APPROVE on its conditions L1 to L4). By its L1 the main session folded, by script "
    "(u4e_fold3.py, each phrase read from the ruling's lines), its findings A1 to A7 and its words for U4E-Q12 (c), "
    "U4E-Q13 (a) and U4E-Q14 (b); 3.1 is untouched. The plan's four edits and carry 66 are the main session's own "
    "commit (L2); the carry-51 commit comes before launch (L3).", width=120)
add_after_block(B, "**The main session's fold at U4j's merge", [""] + note, "the rev 3.3 fold note")
# A1: carry 57's e12 bullet ends with ";", and carry 61's bullet follows it
L = FILES[B]
b57 = [b for b in blocks(L) if b[2] == "item" and L[b[0]].startswith("  - carry 57: a never-acking client")]
assert len(b57) == 1
t57 = text(L, b57[0])
assert t57.endswith(".")
put(L, b57[0], t57[:-1] + ";", "A1, carry 57's end")
j = joined(95, 100)
c61 = j[j.index('"') + 1:j.rindex('"')]
assert c61.startswith("  - carry 61: "), c61[:20]
add_after_block(B, "  - carry 57: a never-acking client", refill(c61[4:], "  - ", "    "), "A1, carry 61's e12 bullet")
edit(B, *pair(107, 111, 'replace "', '" with: "'), "A2, e7's split clause")
edit(B, *pair(119, 123, 'replace "', '" with: "'), "A3 (i), 2.6's owner-to-all row")
edit(B, between(124, 126, 'after "', '" insert: "'), between(124, 126, 'insert: "', '"', last=True),
     "A3 (ii), e12's carry 53", mode="after")
edit(B, between(134, 138, 'after "', '" add: "'), " " + between(134, 138, 'add: "', '"', last=True),
     "A4, 2.1's Encoder bullet", mode="after")
edit(B, between(143, 146, 'after "', '" add: "'), between(143, 146, 'add: "', '"', last=True), "A5, 2.8",
     mode="after")
edit(B, between(149, 150, 'replace "', '"'), between(149, 150, 'with "', '"'), "A6 (i), snapshot.rs's cite", count=3)
edit(B, *pair(151, 152), "A6 (ii), section 1.4")
edit(B, *pair(153, 155), "A6 (iii), the provenance paragraph")
edit(B, *pair(159, 162), "A7 (i), section 4 rule 5")
edit(B, *pair(163, 165), "A7 (ii), 7.2 item 1")
# U4E-Q14 (b), the brief
edit(B, *pair(186, 188), "Q14 e5's lead")
edit(B, *pair(189, 194), "Q14 e5a (i)")
edit(B, *pair(195, 200), "Q14 e5a (ii)")
edit(B, *pair(201, 202), "Q14 2.5")
edit(B, *pair(203, 206), "Q14 2.5's gap bullet")
# U4E-Q12 (c) and U4E-Q13 (a), the brief
append_to_block(B, "- **`EventQueue`** (§4.7 :330-333)", " " + between(227, 230, 'add: "', '"', last=True),
                "Q12 2.1's EventQueue bullet")
edit(B, *pair(231, 232), "Q12 and Q13, 2.1's npt_room::BITES bullet")
add_after_block(B, "| PM-0001-62/server-echo |", [between(233, 238, 'add: "', '"', last=True)], "Q12 2.6's new row")
edit(B, *pair(239, 241), "Q12 2.6's bullet")
edit(B, *pair(254, 254), "Q13 2.6's owner-to-all row")
edit(B, *pair(255, 262), "Q13 2.6's bullet")
# section 8 and 7.2
j = joined(267, 270)
r1 = j[j.index('add "') + 5:j.index('", "')]
r2 = j[j.index('", "') + 4:j.index('" and "')]
r3 = j[j.index('" and "') + 7:j.rindex('"')]
assert all(r.startswith("| U4E-Q1") and r.endswith("|") for r in (r1, r2, r3)), (r1[:12], r2[:12], r3[:12])
add_after_block(B, "| U4E-Q11 |", [r1, r2, r3], "section 8's three ruled rows")
edit(B, *pair(271, 273), "section 8's Open heading")
edit(B, *pair(274, 275), "7.2 item 4")

# --- L2: the plan's four edits, and carry 66 ---
edit(P, *pair(208, 210), "plan §4.14's -18/no-fallback row (Q14)")
edit(P, *pair(211, 212), "plan §5 U4e Tests (Q14)")
edit(P, *pair(213, 214), "plan §5 U4e -18's bite (Q14)")
edit(P, *pair(242, 244), "plan §5 U4e Bites line (Q12)")
L = FILES[C]
assert not any(l.startswith("66. **") for l in L)
assert R[296] == "```" and R[308] == "```" and R[297].startswith("66. **"), (R[296], R[308], R[297][:10])
e66 = R[297:308]
s65 = [i for i, l in enumerate(L) if l.startswith("65. **")]
assert len(s65) == 1
e = s65[0]
while L[e + 1].startswith("    "):
    e += 1
sec = next(L[k] for k in range(e, -1, -1) if L[k].startswith("## "))
assert sec.startswith("## U4d2 or the main session"), sec
L[e + 1:e + 1] = e66
LOG.append(f"L2 carry 66: {C}, {len(e66)} lines after entry 65 (§U4d2 or the main session), from the ruling :298-308")
NEW.append((C, "\n".join(e66)))

# checks
with open(f"{N}/u4s-carries.md", encoding="utf-8") as fh:
    CARRIES = fh.read().split("\n")
L = FILES[B]
h = L.index("### 3.1 Word for word")
o = next(i for i in range(h, len(L)) if L[i] == "```")
c = next(i for i in range(o + 1, len(L)) if L[i] == "```")
assert L[o + 1:c] == CARRIES[146:247], "3.1 changed"
for name, t in NEW:
    flat = " ".join(" ".join(FILES[name]).split())
    assert " ".join(t.split()) in flat, (name, t[:60])
for name in FILES:
    diff = list(difflib.unified_diff(ORIG[name], FILES[name], n=0, lineterm=""))[2:]
    added = [l[1:] for l in diff if l.startswith("+") and not l.startswith("+++")]
    removed = [l for l in diff if l.startswith("-") and not l.startswith("---")]
    long = [l[:50] for l in added if not l.lstrip().startswith("|") and len(l) > 120]
    print(f"{name}: +{len(added)} -{len(removed)} lines; {len(ORIG[name])} -> {len(FILES[name])}; over 120: {long}")
print("\n".join(LOG))
if APPLY:
    for name, L in FILES.items():
        with open(f"{N}/{name}", "w", encoding="utf-8") as fh:
            fh.write("\n".join(L))
    print("APPLIED")
else:
    print("dry run: nothing written")
