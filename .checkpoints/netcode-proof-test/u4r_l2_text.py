"""L2 of the advisor's U4r brief check (advisor-u4r-brief.md, condition L2): the main session's own commit of carries
and plan text before U4r's launch.
- u4s-carries.md: carry 74's amended words (U4R-Q14); a new §U4f entry (U4R-Q4); two new §U4h entries (U4R-Q9, then
  U4R-Q12). Each new entry is the ruling's carry line as written, numbered after the file's last entry (85), so 86,
  87 and 88 ("Later rulings add entries 45 on, each marked": the file's header), and appended at its section's end.
- u4-build-plan.md: §4.15's two phrases, §5 U4r's phrase and §5 U4h's Builds addition (U4R-Q9).
Every old and new phrase is read from the ruling's own lines (each item's line range, wrapped lines joined by one
space), never retyped; the ruling is a closed record, its sha256 asserted first, and so are both files' sha256 at the
brief's section 0 stamps. Each old phrase (or anchor) must occur once in its file, line breaks read as spaces; each new
phrase must occur once after. A changed entry or bullet is rewrapped within 120 characters from the match's first line
to its end, its earlier lines untouched. Last, each file's words (white space collapsed) must equal the old file's
with only these changes made.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_l2_text.py [--apply]
Without --apply it changes nothing and prints the diff it would make."""
import difflib
import hashlib
import re
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
APPLY = "--apply" in sys.argv
RULING = f"{N}/advisor-u4r-brief.md"
CARRIES = f"{N}/u4s-carries.md"
PLAN = f"{N}/u4-build-plan.md"
RULING_SHA = "35f281bfc11c453fb9a5054ecc6e433d4bf9c6509e668f0a28128959feb6ca37"
START = {
    CARRIES: "8b7f165e85b6b7b14f65ce96412545924e4e0e342393d8a3d7b7cef9b0ab20da",
    PLAN: "01ede63dffd4a270db617b7b2870212fcebcf5f346dd71ec080a2651f84bb04b",
}


def sha(b):
    return hashlib.sha256(b).hexdigest()


def read_b(p):
    with open(p, "rb") as fh:
        return fh.read()


src_b = read_b(RULING)
assert sha(src_b) == RULING_SHA, "the ruling is not the closed record read"
SRC = src_b.decode("utf-8").split("\n")
for p, h in START.items():
    assert sha(read_b(p)) == h, (p, "not at the brief's section 0 stamp")


def joined(a, b):
    return " ".join(l.strip() for l in SRC[a - 1:b])


def collapse(t):
    return " ".join(t.split())


def ws(text):
    return r"\s+".join(re.escape(w) for w in text.split())


PAIR = re.compile(r'"([^"]+)" → "([^"]+)"')

# Carry 74 (U4R-Q14): the ruling's bullet "Carry 74, amended by this ruling", :324-330, holds one pair.
j74 = joined(324, 330)
assert j74.startswith("- **Carry 74, amended by this ruling**"), j74[:60]
p74 = PAIR.findall(j74)
assert len(p74) == 1, p74
# The plan text (U4R-Q9): the ruling's bullet "The plan text", :250-262: three pairs, one "after ... add ...", and
# the §U4h carry line.
j9 = joined(250, 262)
assert j9.startswith("- The plan text (the main session's commit"), j9[:60]
p9 = PAIR.findall(j9)
assert len(p9) == 3, p9
add9 = re.findall(r'after "([^"]+)" add "([^"]+)"', j9)
assert len(add9) == 1, add9
c9 = re.findall(r'Carry line for §U4h \(:548\): "([^"]+)"', j9)
assert len(c9) == 1, c9
# U4R-Q4's §U4f line, :202-205; U4R-Q12's §U4h line, :271-280.
j4 = joined(202, 205)
c4 = re.findall(r'§U4f \(:491\): "([^"]+)"', j4)
assert len(c4) == 1 and j4.endswith('first."'), c4
j12 = joined(271, 280)
assert j12.startswith("**U4R-Q12 (the re-run after U4h): (a), reading (i).**"), j12[:60]
c12 = re.findall(r'Carry line for §U4h \(:548\): "([^"]+)"', j12)
assert len(c12) == 1 and j12.endswith('`RttSource::Link`."'), c12
for t in (c4[0], c9[0], c12[0]):
    assert t.startswith("**U4R-Q") or t.startswith("**O1's re-run"), t[:40]

ENTRY = re.compile(r"^(\d+)\. ")
BULLET = re.compile(r"^( *)- ")


def wrapper(first, cont):
    return textwrap.TextWrapper(width=120, initial_indent=first, subsequent_indent=cont, break_long_words=False,
                                break_on_hyphens=False)


def unit_bounds(lines, i, kind):
    """The entry (carries: `NN. ` at column 0, continuation indented 4) or bullet (plan: `- ` at its indent) that holds
    line i: its first and last line, and its continuation indent."""
    s = i
    if kind == "entry":
        while not ENTRY.match(lines[s]):
            assert lines[s].startswith("    "), ("not inside an entry", i, s)
            s -= 1
            assert s >= 0, ("no entry above", i)
        cont = "    "
        e = s
        while e + 1 < len(lines) and lines[e + 1].startswith("    ") and lines[e + 1].strip():
            e += 1
    else:
        while not BULLET.match(lines[s]):
            s -= 1
            assert s >= 0 and lines[s].strip() and not lines[s].startswith("#"), ("no bullet above", i)
        cont = BULLET.match(lines[s]).group(1) + "  "
        e = s
        while e + 1 < len(lines) and lines[e + 1].strip() and not BULLET.match(lines[e + 1]) \
                and not lines[e + 1].startswith(("#", "|")):
            e += 1
    return s, e, cont


def edit(lines, old, new, kind, tag, insert_after=False):
    """Replace `old` (or, with insert_after, put `new` right after the anchor `old`) once, then rewrap the unit from
    the match's first line to its end."""
    text = "\n".join(lines)
    ms = list(re.finditer(ws(old), text))
    if len(ms) != 1:
        sys.exit(f"{tag}: the old phrase occurs {len(ms)} times: {old!r}")
    m = ms[0]
    first = text.count("\n", 0, m.start())
    last = text.count("\n", 0, m.end())
    s, e, cont = unit_bounds(lines, first, kind)
    assert e >= last, (tag, "the match runs past its unit", s, e, last)
    base = len("\n".join(lines[:first])) + (1 if first else 0)
    block = "\n".join(lines[first:e + 1])
    off, end = m.start() - base, m.end() - base
    assert block[off:end].split() == old.split(), (tag, block[off:end])
    body = block[:off] + (block[off:end] + new if insert_after else new) + block[end:]
    flat = " ".join(x.strip() for x in body.split("\n"))
    if first == s:
        head = lines[s][:len(lines[s]) - len(lines[s].lstrip())]
        marker = ENTRY.match(lines[s].lstrip()) if kind == "entry" else BULLET.match(lines[s])
        lead = (marker.group(0) if kind == "entry" else head + "- ")
        assert flat.startswith(lead.strip()), (tag, flat[:20])
        flat = flat[len(lead.strip()):].lstrip()
        out = wrapper(lead, cont).wrap(flat)
    else:
        out = wrapper(cont, cont).wrap(flat)
    lines[first:e + 1] = out
    return f"{tag}: unit :{s + 1}-{e + 1}, rewrapped from :{first + 1} ({len(out)} lines for {e + 1 - first})"


def append_entry(lines, heading, number, text, tag):
    """Add entry `number` with `text` at the end of the section under `heading` (after its last non-blank line)."""
    hs = [i for i, l in enumerate(lines) if l == heading]
    assert len(hs) == 1, (tag, heading, len(hs))
    h = hs[0]
    nxt = next(i for i in range(h + 1, len(lines)) if lines[i].startswith("## "))
    last = nxt - 1
    while not lines[last].strip():
        last -= 1
    assert last > h, (tag, "empty section")
    out = wrapper(f"{number}. ", "    ").wrap(text)
    lines[last + 1:last + 1] = out
    return f"{tag}: entry {number} after :{last + 1} under {heading!r} ({len(out)} lines)"


log = []
results = {}
# u4s-carries.md
car = read_b(CARRIES).decode("utf-8").split("\n")
car0 = list(car)
numbers = sorted(int(ENTRY.match(l).group(1)) for l in car if ENTRY.match(l))
assert numbers[-1] == 85 and len(numbers) == len(set(numbers)), numbers[-5:]
log.append(edit(car, p74[0][0], p74[0][1], "entry", "carry 74 (U4R-Q14)"))
log.append(append_entry(car, "## U4f (clock and commands)", 86, c4[0], "U4R-Q4"))
log.append(append_entry(car, "## U4h (lag compensation and aim)", 87, c9[0], "U4R-Q9"))
log.append(append_entry(car, "## U4h (lag compensation and aim)", 88, c12[0], "U4R-Q12"))
results[CARRIES] = (car0, car)
# u4-build-plan.md
plan = read_b(PLAN).decode("utf-8").split("\n")
plan0 = list(plan)
for k, (old, new) in enumerate(p9):
    log.append(edit(plan, old, new, "bullet", f"plan pair {k + 1} (U4R-Q9)"))
log.append(edit(plan, add9[0][0], add9[0][1], "bullet", "plan §5 U4h add (U4R-Q9)", insert_after=True))
results[PLAN] = (plan0, plan)

# Each new phrase once; each replaced old phrase gone; the words equal the old file's with only these changes made.
ct = "\n".join(car)
pt = "\n".join(plan)
for new, where, tag in ((p74[0][1], ct, "carry 74"), (c4[0], ct, "86"), (c9[0], ct, "87"), (c12[0], ct, "88"),
                        *[(n, pt, f"plan pair {k + 1}") for k, (_, n) in enumerate(p9)],
                        (add9[0][0] + add9[0][1], pt, "plan add")):
    assert len(re.findall(ws(new), where)) == 1, (tag, "the new phrase is not there once")
for old, where, tag in ((p74[0][0], ct, "carry 74"), *[(o, pt, f"plan pair {k + 1}") for k, (o, _) in enumerate(p9)]):
    assert not re.findall(ws(old), where), (tag, "the old phrase is still there")
exp_c = collapse("\n".join(car0)).replace(collapse(p74[0][0]), collapse(p74[0][1]))
back_c = collapse(ct)
for num, t in ((86, c4[0]), (87, c9[0]), (88, c12[0])):
    piece = f" {num}. " + collapse(t)
    assert back_c.count(piece) == 1, num
    back_c = back_c.replace(piece, "")
assert back_c == exp_c, "the carries' words differ beyond the four changes"
exp_p = collapse("\n".join(plan0))
for old, new in p9:
    assert exp_p.count(collapse(old)) == 1
    exp_p = exp_p.replace(collapse(old), collapse(new))
anchor = collapse(add9[0][0])
assert exp_p.count(anchor) == 1
exp_p = exp_p.replace(anchor, anchor + " " + collapse(add9[0][1]))
assert collapse(pt) == exp_p, "the plan's words differ beyond the four changes"
for path, (old, new) in results.items():
    before = set(old)
    long = [i + 1 for i, l in enumerate(new) if len(l) > 120 and l not in before]
    assert not long, (path, "changed lines over 120 characters", long)

print("\n".join(log))
for path, (old, new) in results.items():
    print(f"{path}: sha256 before {sha(read_b(path))[:12]}; lines {len(old)} -> {len(new)}")
    sys.stdout.writelines(difflib.unified_diff([l + "\n" for l in old], [l + "\n" for l in new], path, path, n=1))
if APPLY:
    for path, (_, new) in results.items():
        with open(path, "w", encoding="utf-8") as fh:
            fh.write("\n".join(new))
        print(f"applied {path}; sha256 after {sha(read_b(path))}; lines {len(new)}")
else:
    print("dry run: nothing written")
