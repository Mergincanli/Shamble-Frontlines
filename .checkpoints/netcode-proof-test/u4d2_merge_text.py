"""Apply the texts of U4d2's merge by script, as the advisor's two U4d2 rulings give them: advisor-u4d2-r1.md (Block 9's
M1-M9 and its texts T1-T5) and advisor-u4d2-r2.md (Block 5, which amends M5, M8 and M9; Block 3, which keeps T1 and T4
and gives T2, T3 and T5 whole). As Block 5's amended M8 rules: T1 and T4 are copied from advisor-u4d2-r1.md, unchanged;
T2, T3 and T5 from advisor-u4d2-r2.md (round 1's T2, T3 and T5 are not used).

  --phase A  (M5: before check.mjs's counted run on the merged tree): T4 into spikes/netcode-proof/marks/SERIES.md, the
             header-params table, after `fate_log_sha256`'s row.
  --phase B  (M8, amended: at the merge commit): T1 into u4-build-plan.md's §4.1 rows (npt-lab's and npt-hash-wasm's),
             T2 into u4s-carries.md §U4r after entry 77 (entries 82 to 85), T3 into plan.md's U7 row (C-U7-2), T5 into
             u4d2-report.md at the line ANCHOR (all in the notes folder).

Every text is read from the rulings' own code blocks (the fenced block, or T1's two, under the heading "### Tn."), never
typed here; where a ruling names the place (a section, an entry, a row, a heading, a line number), that is read from the
ruling too and checked. Each old text or anchor is found exactly once in its target (a line break and the next line's
indent read as one space); each new text is found in place afterwards, equal byte for byte to its code block as the
ruling file holds it; no changed prose line is over 120 characters (a table line, as T1's, T3's and T4's, is not prose:
it is edited in its line); every edit is checked word for word (the target with white space collapsed equals the old
one with only that change made); and each file's unified diff removes and adds exactly the ruled lines, nothing else.
Carries 82-85, C-U7-2, T1's new rows and T4's row are asserted absent first. The rulings' and the phase's targets'
sha256 are asserted first, so a moved file stops it.

Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4d2_merge_text.py --phase A|B [--apply]
Without --apply it writes nothing: it prints one log line per edit, then a summary line per file, then the unified diff
of every file it would change. With --apply in phase B, u4d2-report.md must already exist and hold the line ANCHOR once
(the main session writes the report's other sections first, with the Write tool); without --apply a missing report is
simulated as the anchor line alone, so the diff shows what would be inserted. T5's `<...>` places are copied as they
are; the main session fills them later from M5's lines (a separate step)."""
import bisect
import difflib
import hashlib
import os
import re
import sys

N = ".checkpoints/netcode-proof-test"
ARGS = sys.argv[1:]
USAGE = "usage: python3 -I .checkpoints/netcode-proof-test/u4d2_merge_text.py --phase A|B [--apply]"
assert ARGS.count("--phase") == 1 and ARGS.index("--phase") + 1 < len(ARGS), USAGE
PHASE = ARGS[ARGS.index("--phase") + 1]
APPLY = "--apply" in ARGS
assert PHASE in ("A", "B") and set(ARGS) <= {"--phase", PHASE, "--apply"}, USAGE
ANCHOR = "<!-- T5: the advisor's rulings section, copied by u4d2_merge_text.py -->"

R1P, R2P = f"{N}/advisor-u4d2-r1.md", f"{N}/advisor-u4d2-r2.md"
SERIES = "spikes/netcode-proof/marks/SERIES.md"
CARRIES, PLAN, BPLAN, REPORT = (f"{N}/{n}" for n in ("u4s-carries.md", "plan.md", "u4-build-plan.md", "u4d2-report.md"))
SHA = {
    R1P: "9978b43b763958f9066b708a05319dfd5b03fa61dbc1b920b0375465298d9b6f",
    R2P: "e776f0ba7c4894ef72f05fb12ebfa1c955a61bd6018b200404f010f5b310f8d4",
    SERIES: "f118a8defb872f0e3e76a9238fab4ffdfd4f5c86eb818c45e9c591ae0a34d21e",
    BPLAN: "52dd31860efc6c58708845efbe09ebff056bb8cb27696c2521fc56775d9bd039",
    CARRIES: "efdd7b6bd358378fcf5e5e447d8b7b20db01fdda5a348a24f3b622c0e7b17c59",
    PLAN: "aac32c3d280211b029b0ea7b67e0568e89c1503707e29f96e0989ecdfc5a7ba8",
}
TARGETS = {"A": (SERIES,), "B": (BPLAN, CARRIES, PLAN)}


def raw_bytes(path):
    with open(path, "rb") as fh:
        return fh.read()


def sha256(path):
    return hashlib.sha256(raw_bytes(path)).hexdigest()


assert os.path.isdir(N) and os.path.exists(".git"), "run from the repo root"
for _p in (R1P, R2P) + TARGETS[PHASE]:
    _got = sha256(_p)
    assert _got == SHA[_p], f"{_p} has moved: sha256 {_got[:8]}..., the script was written for {SHA[_p][:8]}..."
    assert b"\r" not in raw_bytes(_p), (_p, "a carriage return: the line-by-line read would not keep the file's bytes")


def read(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read().split("\n")


R1, R2 = read(R1P), read(R2P)
NAME = {id(R1): "advisor-u4d2-r1.md", id(R2): "advisor-u4d2-r2.md"}
RAW = {id(R1): raw_bytes(R1P).split(b"\n"), id(R2): raw_bytes(R2P).split(b"\n")}


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


def block_bytes(R, a, b, lines):
    """The ruling file's own bytes of lines a..b (1-based, inclusive): what the copy is compared with, byte for byte."""
    out = b"\n".join(RAW[id(R)][a - 1:b])
    assert out == "\n".join(lines).encode("utf-8"), (NAME[id(R)], a, b, "the block's bytes and its lines differ")
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


def says(R, heading, phrase):
    """The ruling's section `heading`, white space collapsed, holds `phrase` once: the rule a step rests on."""
    s, e = section(R, heading)
    text = collapse("\n".join(R[s:e]))
    assert text.count(phrase) == 1, (NAME[id(R)], heading, "does not say once", phrase)


def src(R, a, b, what):
    return f"{NAME[id(R)]} :{a}" + (f"-{b}" if b > a else "") + f" ({what})"


# --- which ruling gives which text (advisor-u4d2-r2.md, Blocks 3 and 5, and its texts' first line) --------------------
def collapse(text):
    return " ".join(text.split())


for _h in ("### T1.", "### T4."):
    assert not any(l.startswith(_h) for l in R2), f"advisor-u4d2-r2.md gives {_h[4:]}: round 1's would not be the text"
says(R2, "### Block 3.",
     "**T1** (u4-build-plan.md §4.1's two rows) stays as round 1 wrote it (advisor-u4d2-r1.md, T1)")
says(R2, "### Block 3.", "**T4** (SERIES.md's row) stays as written there.")
says(R2, "### Block 5.",
     "T1 and T4 are copied from advisor-u4d2-r1.md, unchanged; T2, T3 and T5 from this file")
says(R2, "## The texts",
     "T1 and T4: advisor-u4d2-r1.md's code blocks, unchanged (Block 3). T2, T3 and T5 below replace round 1's.")


# --- the targets: finding, editing, checking -----------------------------------------------------------------------
FILES, ORIG, LOG, NOTES, EXPECT, BYTES, WIDE = {}, {}, [], [], {}, [], {}


def doc(path):
    if path not in FILES:
        FILES[path] = read(path)
        ORIG[path] = list(FILES[path])
        EXPECT[path] = []
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


def edit(tag, path, a, b, new_lines, pairs, removed, added, source, what):
    """Lines a..b-1 (0-based) of `path` become `new_lines`. Checked word for word: the file with white space collapsed
    equals the old one with each (old, new) pair's change made and nothing else. `removed` and `added` are the ruled
    lines (built from the ruling's texts): the lines replaced must be `removed`, the lines put in must be `added`, and
    the file's diff must remove and add exactly these."""
    d = doc(path)
    before = list(d)
    assert before[a:b] == removed and new_lines == added, (tag, "the edit's lines are not the ruled ones")
    d[a:b] = new_lines
    exp = collapse("\n".join(before))
    for o, n in pairs:
        oc = collapse(o)
        assert exp.count(oc) == 1, (tag, "the old text, white space collapsed, is not in the file once", oc[:80])
        exp = exp.replace(oc, collapse(n))
    assert collapse("\n".join(d)) == exp, (tag, "the edit changed more than its text")
    delta = len(new_lines) - (b - a)
    for p, _, ad in EXPECT[path]:
        inside = max(a, p) < min(b, p + len(ad)) or (a == b and p < a < p + len(ad))
        assert not inside, (tag, "two edits overlap", p + 1)
    EXPECT[path] = [(p + delta if p >= b else p, r, ad) for p, r, ad in EXPECT[path]]
    for k, x in enumerate(BYTES):
        if x[1] == path and x[3] >= b:
            BYTES[k] = x[:3] + (x[3] + delta,) + x[4:]
    EXPECT[path].append((a, removed, added))
    span = f"{a + 1}-{b}" if b > a + 1 else (f"{a + 1}" if b == a + 1 else f"{a + 1} (inserted before)")
    LOG.append(f"{tag} [phase {PHASE}]: {path}:{span}, {b - a} line(s) -> {len(new_lines)}: {what}; from {source}")


def byte_check(tag, path, raw, line, prefix):
    """Record a byte-for-byte check: the code block's bytes `raw` stand once in the file, at 0-based line `line`, after
    the kept start `prefix` of that line (b"" for whole lines), and end their line."""
    BYTES.append((tag, path, raw, line, prefix))


def run_byte_checks(content, when):
    for tag, path, raw, line, prefix in BYTES:
        final = content(path)
        assert final.count(raw) == 1, (tag, when, f"the code block's bytes stand {final.count(raw)} times in {path}")
        pos = final.index(raw)
        end = pos + len(raw)
        start = pos - len(prefix)
        assert end == len(final) or final[end:end + 1] == b"\n", (tag, when, "the copied text does not end its line")
        assert start >= 0 and final[start:pos] == prefix, (tag, when, "the line's kept start differs")
        assert start == 0 or final[start - 1:start] == b"\n", (tag, when, "the copied text does not start its line")
        at = final[:start].count(b"\n")
        assert at == line, (tag, when, "the copied text is not at its line", at + 1, line + 1)


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


def entries(block):
    """The carry entries of a block: {number: its lines}, in order."""
    out, cur = {}, None
    for l in block:
        m = ENTRY.match(l)
        if m:
            cur = int(m.group(1))
            assert cur not in out, ("an entry twice", cur)
            out[cur] = [l]
        else:
            assert cur is not None and l.startswith("    ") and l.strip(), ("an entry's form", l[:40])
            out[cur].append(l)
    return out


# --- phase A ----------------------------------------------------------------------------------------------------------
def t4():
    tag = "T4"
    h, _ = section(R1, "### T4.")
    bl = blocks(R1, "### T4.")
    assert len(bl) == 1 and len(bl[0][2]) == 1, tag
    a, b, (row,) = bl[0]
    assert R1[h].startswith("### T4. marks/SERIES.md, the header-params table, after `"), (tag, R1[h][:60])
    assert SERIES.endswith("/marks/SERIES.md"), tag
    p = prose(R1, h + 2, a - 2)
    assert p.startswith("After the line `") and p.endswith(" at the head), insert:"), (tag, p[:40], p[-40:])
    old = between(p, "After the line `", "` (found once")
    line_no = int(between(p, "` (found once, :", " at the head)"))
    name = between(R1[h], "after `", "`'s row")
    assert old.startswith(f"| `{name}` |"), (tag, old[:40])
    assert row.startswith("| `") and "` |" in row, tag
    new_name = row[3:row.index("` |")]
    d = doc(SERIES)
    assert new_name not in "\n".join(d), (tag, "the row exists already", new_name)
    assert row not in d, (tag, "the row exists already")
    l0, c0, l1, c1 = find(d, old, tag)
    assert "\n".join(d).count(old) == 1 and sum(l == old for l in d) == 1, (tag, "the old line is not there once")
    assert l0 == l1 and c0 == 0 and c1 == len(d[l0]), (tag, "the old text is not one whole line")
    assert l0 + 1 == line_no, (tag, "the line is not at the ruling's line", l0 + 1, line_no)
    t0, _ = table_bounds(d, l0)
    head = heading_above(d, t0, ("## ",))
    assert head.startswith("## Header params "), (tag, "not the header-params table", head)
    assert row.count("|") == d[l0].count("|") == d[t0].count("|"), (tag, "the row's cells")
    i = l0 + 1
    edit(tag, SERIES, i, i, [row], [(old, old + "\n" + row)], [], [row],
         f"{src(R1, a, a, 'the row')}; {src(R1, h + 2, a - 2, 'the place')}",
         f"the `{new_name}` row, right after `{name}`'s row (SERIES.md:{line_no}), in the header-params table "
         f"('{head[3:40]}...'); a table line")
    byte_check(tag, SERIES, block_bytes(R1, a, b, [row]), i, b"")
    m = re.match(r"^## .*\((\d+)\)$", head)
    if m:
        NOTES.append(f"NOTE (T4, not by script): SERIES.md's heading reads '{head}'; neither ruling changes it")


# --- phase B ----------------------------------------------------------------------------------------------------------
T1_LABEL = re.compile(r"^\(([ab])\) The row `(.+)` becomes:$")


def t1():
    h, e = section(R1, "### T1.")
    bl = blocks(R1, "### T1.")
    assert len(bl) == 2 and all(len(x[2]) == 1 for x in bl), ("T1: two one-line blocks", len(bl))
    assert R1[h].endswith(" (the merge commit)"), "T1"
    sec = between(R1[h], "### T1. u4-build-plan.md §", ":")
    labels = [i for i in range(h + 1, e) if T1_LABEL.match(R1[i])]
    assert len(labels) == 2, ("T1: the labels (a) and (b)", labels)
    d = doc(BPLAN)
    sh = [i for i, l in enumerate(d) if l.startswith(f"### {sec} ")]
    assert len(sh) == 1, ("T1", sh)
    send = next(i for i in range(sh[0] + 1, len(d)) if d[i].startswith("#"))
    for k, (letter, crate) in enumerate((("a", "npt-lab"), ("b", "npt-hash-wasm"))):
        tag = f"T1 ({letter})"
        a, b, (new,) = bl[k]
        lab = a - 3
        assert labels[k] == lab and R1[lab + 1] == "```", (tag, "the label is not right above its block")
        m = T1_LABEL.match(R1[lab])
        assert m.group(1) == letter, tag
        old = m.group(2)
        assert f"{crate}'s row" in R1[h] and crate in old.split("|")[1 if letter == "a" else 2], (tag, crate)
        assert new not in d, (tag, "the new row exists already")
        l0, c0, l1, c1 = find(d, old, tag)
        assert "\n".join(d).count(old) == 1 and sum(l == old for l in d) == 1, (tag, "the old row is not there once")
        assert l0 == l1 and c0 == 0 and c1 == len(d[l0]), (tag, "the old text is not one whole row")
        assert sh[0] < l0 < send, (tag, f"the row is not in §{sec}")
        t0, _ = table_bounds(d, l0)
        assert old.count("|") == new.count("|") == d[t0].count("|"), (tag, "the row's cells")
        assert new.split("|")[1] == old.split("|")[1], (tag, "the row's first cell is kept")
        edit(tag, BPLAN, l0, l0 + 1, [new], [(old, new)], [old], [new],
             f"{src(R1, a, a, 'the new row')}; {src(R1, lab + 1, lab + 1, 'the old row')}",
             f"§{sec}'s {crate} row, whole (a table line)")
        byte_check(tag, BPLAN, block_bytes(R1, a, b, [new]), l0, b"")


T2_HEAD = re.compile(r'^### T2\. u4s-carries\.md §(U4[a-z]), after entry (\d+) \(its last\), above "(## [^"]+)" '
                     r"\(the merge commit\)$")


def t2():
    tag = "T2"
    h, _ = section(R2, "### T2.")
    m = T2_HEAD.match(R2[h])
    bl = blocks(R2, "### T2.")
    assert m and len(bl) == 1, (tag, R2[h][:60], len(bl))
    sect, n, above = m.group(1), int(m.group(2)), m.group(3)
    a, b, block = bl[0]
    got = entries(block)
    nums = list(got)
    assert nums == [82, 83, 84, 85], (tag, "the block's entries are not 82 to 85, in order", nums)
    says(R2, "### Block 5.", "u4s-carries.md gains entries 82 to 85.")
    says(R2, "### Block 3.", "82 as round 1 wrote it;")
    (_, _, r1block), = blocks(R1, "### T2.")
    assert entries(r1block)[82] == got[82], (tag, "entry 82 is not as round 1 wrote it")
    assert all(len(l) <= 120 for l in block), (tag, "a line over 120")
    d = doc(CARRIES)
    for x in nums:
        assert not any(l.startswith(f"{x}. ") for l in d), (tag, f"carry {x} exists already")
    s, e = entry_span(d, n)
    assert heading_above(d, s, ("## ",)).startswith(f"## {sect} "), (tag, heading_above(d, s, ("## ",)))
    assert sum(l == above for l in d) == 1, (tag, "the heading named is not in the file once", above)
    assert not d[e + 1].strip() and d[e + 2] == above, (tag, f"entry {n} is not right above '{above[:40]}'")
    old = "\n".join(d[s:e + 1])
    find(d, collapse(old), tag)
    edit(tag, CARRIES, e + 1, e + 1, block, [(old, old + "\n" + "\n".join(block))], [], block,
         f"{src(R2, a, b, 'the block')}; {src(R2, h + 1, h + 1, 'the place')}",
         f"carries {', '.join(map(str, nums))} after entry {n}, the last of §{sect}, above '{above[:44]}...'")
    byte_check(tag, CARRIES, block_bytes(R2, a, b, block), e + 1, b"")


def t3():
    tag = "T3"
    h, _ = section(R2, "### T3.")
    bl = blocks(R2, "### T3.")
    assert len(bl) == 1 and len(bl[0][2]) == 1, tag
    a, b, (new,) = bl[0]
    p = prose(R2, h + 2, a - 2)
    old = between(p, "The text `", "` (found once")
    row = between(R2[h], "plan.md's ", " row")
    code = between(R2[h], " row: ", " (the merge commit)")
    line_no = int(between(p, f"at the end of the {row} row's last cell, plan.md:", " at the head)"))
    assert '(no "|" added inside the cell)' in p and "on the row's one line" in p, tag
    d = doc(PLAN)
    assert code not in "\n".join(d), (tag, f"{code} exists already")
    assert new.count(f"**{code}**") == 1, (tag, f"{code} is not in the new text once")
    l0, c0, l1, c1 = find(d, old, tag)
    assert "\n".join(d).count(old) == 1, (tag, "the old text is not there once")
    assert l0 == l1 and l0 + 1 == line_no and d[l0].startswith(f"| {row} |"), (tag, l0 + 1, d[l0][:20])
    assert c1 == len(d[l0]), (tag, "the old text is not the row's end")
    assert "**C-U7-1**" in d[l0][:c0], (tag, "C-U7-2 follows C-U7-1 in the cell")
    assert old.count("|") == new.count("|") == 1 and old.endswith(" |") and new.endswith(" |"), (tag, "the cells")
    prefix = d[l0][:c0]
    line = prefix + new
    assert line.count("|") == d[l0].count("|"), (tag, "a '|' added")
    edit(tag, PLAN, l0, l0 + 1, [line], [(old, new)], [prefix + old], [line],
         f"{src(R2, a, a, 'the block')}; {src(R2, h + 2, a - 2, 'the old text')}",
         f"the {row} row's end, in its line (a table line): {code} after C-U7-1")
    byte_check(tag, PLAN, block_bytes(R2, a, b, [new]), l0, prefix.encode("utf-8"))


def t5():
    tag = "T5"
    h, _ = section(R2, "### T5.")
    bl = blocks(R2, "### T5.")
    assert len(bl) == 1, tag
    a, b, block = bl[0]
    assert block[0] == ("## What U4d2's review settled (the advisor's U4d2 rulings: round 1, advisor-u4d2-r1.md; "
                        "round 2, advisor-u4d2-r2.md)"), (tag, block[0][:40])
    assert all(len(l) <= 120 for l in block), (tag, "a line over 120")
    assert prose(R2, h + 2, a - 2).startswith("Copied by script as it is."), tag
    holes = re.findall(r"<[^<>\n]*>", "\n".join(block))
    says(R2, "### Block 3.", "Two placeholders stay for the main session to fill from the named source")
    assert len(holes) == 2, (tag, "not round 2's two placeholders", holes)
    exists = os.path.exists(REPORT)
    assert exists or not APPLY, "u4d2-report.md must exist and hold the anchor line once before --apply"
    if exists:
        assert b"\r" not in raw_bytes(REPORT), (tag, "a carriage return in the report")
        d = doc(REPORT)
        assert block[0] not in d, (tag, "the rulings section is in the report already")
    else:
        FILES[REPORT], ORIG[REPORT], EXPECT[REPORT] = [ANCHOR], [ANCHOR], []
        d = FILES[REPORT]
    hits = [i for i, l in enumerate(d) if l == ANCHOR]
    assert len(hits) == 1 and "\n".join(d).count(ANCHOR) == 1, (tag, "the anchor line is not in the report once")
    k = hits[0]
    edit(tag, REPORT, k, k + 1, block, [(ANCHOR, "\n".join(block))], [ANCHOR], block, src(R2, a, b, "the block"),
         "the rulings section at the anchor line" + ("" if exists else " [simulated: the report is not written yet]"))
    byte_check(tag, REPORT, block_bytes(R2, a, b, block), k, b"")
    NOTES.append(f"NOTE (T5, not by script): the section holds {len(holes)} '<...>' places, copied as they are: "
                 + "; ".join(holes) + ". The main session fills them from M5's lines after the copy, naming each "
                 "fill's log and line in the notes")
    if exists:
        NOTES.append(f"NOTE (T5): u4d2-report.md's sha256 before the copy: {sha256(REPORT)}")
    else:
        NOTES.append("NOTE (T5): u4d2-report.md does not exist yet: T5 is shown against a stand-in that holds the "
                     "anchor line alone; --apply refuses until the report holds that line once")


if PHASE == "A":
    t4()
else:
    t1()
    t2()
    t3()
    t5()

# --- every new text in place, byte for byte; the diff is the ruled lines; widths; the log ---------------------------
run_byte_checks(lambda p: "\n".join(FILES[p]).encode("utf-8"), "before writing")
SUMMARY, DIFFS = [], []
for path in FILES:
    if FILES[path] == ORIG[path]:
        continue
    diff = list(difflib.unified_diff(ORIG[path], FILES[path], f"a/{path}", f"b/{path}", n=1, lineterm=""))
    body = [l for l in diff[2:] if not l.startswith("@@")]
    added = [l[1:] for l in body if l.startswith("+")]
    removed = [l[1:] for l in body if l.startswith("-")]
    ruled = sorted(EXPECT[path], key=lambda x: x[0])
    assert removed == [l for _, r, _ in ruled for l in r], (path, "the diff removes other lines than the ruled ones")
    assert added == [l for _, _, ad in ruled for l in ad], (path, "the diff adds other lines than the ruled ones")
    long = [l[:60] for l in added if not l.startswith("|") and len(l) > 120]
    assert not long, (path, "a changed prose line is over 120 characters", long)
    tables = sum(1 for l in added if l.startswith("|"))
    before = hashlib.sha256("\n".join(ORIG[path]).encode()).hexdigest()
    after = hashlib.sha256("\n".join(FILES[path]).encode()).hexdigest()
    tags = ", ".join(t for t, p, *_ in BYTES if p == path)
    SUMMARY.append(f"{path}: -{len(removed)} +{len(added)} lines ({tags}); sha256 {before[:8]}... -> {after[:8]}... "
                   f"(after: {after}); the diff's removed and added lines are exactly the ruled ones; each copied text "
                   f"equals its code block byte for byte, once, in place; changed prose lines over 120: none "
                   f"({tables} table line(s) among the added, not prose)")
    DIFFS.append("\n".join(diff))
print("\n".join(LOG + NOTES + SUMMARY))
if APPLY:
    for path in FILES:
        if FILES[path] != ORIG[path]:
            with open(path, "w", encoding="utf-8", newline="") as fh:
                fh.write("\n".join(FILES[path]))
            assert read(path) == FILES[path], (path, "the written file reads back different")
    run_byte_checks(raw_bytes, "read back after writing")
    print(f"phase {PHASE}: APPLIED ({len(LOG)} edits); each copied text read back byte for byte")
else:
    print("\n".join(DIFFS))
    print(f"phase {PHASE}: dry run, every assert passed ({len(LOG)} edits); nothing written")
