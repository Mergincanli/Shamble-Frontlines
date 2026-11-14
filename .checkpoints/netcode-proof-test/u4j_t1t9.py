"""Apply the text commit of the advisor's U4j round-2 ruling (advisor-u4j-r2.md B7.1: T1 to T9, M8.2) by line range.
Every old and new string is the whole of one fenced block of the ruling, read from its own lines, never retyped.
Prose (marks/README.md): the named list item is joined into one string, each old string found exactly once in it and
replaced, the item refilled to 120 characters with its own indents, and its words checked equal to the old words with
only the substitutions. Single lines (SERIES.md and fixtures/README.md rows, marks.json's notes): each old string found
exactly once in the file and replaced. Run from the repo root (after the merge commit, never before it):
    python3 -I .checkpoints/netcode-proof-test/u4j_t1t9.py [--apply]
Without --apply it changes nothing and prints what it would do."""
import difflib
import hashlib
import json
import re
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
S = "spikes/netcode-proof"
APPLY = "--apply" in sys.argv
with open(f"{N}/advisor-u4j-r2.md", encoding="utf-8") as fh:
    R2 = fh.read().split("\n")


def one(a):
    """The one line of a fenced block at line a of the ruling, as it is (the fence lines checked)."""
    assert R2[a - 2].startswith("```") and R2[a] == "```", (a, R2[a - 2], R2[a])
    return R2[a - 1]


FILES, ORIG, LOG, NEW = {}, {}, [], []


def doc(name):
    if name not in FILES:
        with open(f"{S}/{name}", encoding="utf-8") as fh:
            FILES[name] = fh.read().split("\n")
        ORIG[name] = list(FILES[name])
    return FILES[name]


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
        t = t.replace(f"{prev} {tok}", f"{prev}{NBSP}{tok}", 1)
    sys.exit(f"refill: no safe wrap for {text[:60]}")


def item_sub(name, start, pairs, tag):
    """In the list item whose first line starts with `start`, each old text found exactly once and replaced; the item
    (its first line and its continuation lines, up to a blank line, a nested or sibling item, or a lesser indent)
    refilled with its own indents; its words after equal to its words before with only the substitutions."""
    d = doc(name)
    hits = [i for i, l in enumerate(d) if l.startswith(start)]
    assert len(hits) == 1, (tag, start, hits)
    s = hits[0]
    m = len(d[s]) - len(d[s].lstrip())
    assert d[s][m:].startswith("- "), (tag, d[s][:20])
    cont = " " * (m + 2)
    e = s
    while e + 1 < len(d) and d[e + 1].startswith(cont) and d[e + 1].strip() and not d[e + 1].lstrip().startswith("- "):
        e += 1
    text = " ".join([d[s][m + 2:].strip()] + [l.strip() for l in d[s + 1:e + 1]])
    new = text
    for o, n in pairs:
        c = new.count(o)
        assert c == 1, (tag, o[:50], c)
        new = new.replace(o, n)
    lines = refill(new, " " * m + "- ", cont)
    assert " ".join(l.strip() for l in lines)[2:].split() == new.split() or \
        " ".join(lines).split()[1:] == new.split(), tag
    d[s:e + 1] = lines
    LOG.append(f"{tag}: {name}:{s + 1}-{e + 1} ({e - s + 1} lines) -> {len(lines)} lines, {len(pairs)} substitution(s)")
    for _, n in pairs:
        NEW.append((name, n))


def line_sub(name, old, new, tag):
    d = doc(name)
    hits = [i for i, l in enumerate(d) if old in l]
    assert len(hits) == 1 and d[hits[0]].count(old) == 1, (tag, hits)
    d[hits[0]] = d[hits[0]].replace(old, new)
    LOG.append(f"{tag}: {name}:{hits[0] + 1}, {len(old)} chars -> {len(new)}")
    NEW.append((name, new))


RM, SE, MJ, FX = "marks/README.md", "marks/SERIES.md", "marks/marks.json", "crates/judge/fixtures/README.md"
item_sub(RM, "  - `server_hash` (", [(one(324), one(328))], "T1 README §1 server_hash")
item_sub(RM, "  - `decoded_hash` (", [(one(334), one(338)), (one(342), one(346))], "T2 README §1 decoded_hash")
item_sub(RM, "- **The WebSocket practice run's outcome record**", [(one(352), one(356))], "T3 README §2.5 practice record")
item_sub(RM, "- **`ws-practice:<tolerance>`**", [(one(363), one(367)), (one(371), one(375))], "T4 README §4 ws-practice")
line_sub(MJ, one(381), one(385), "T5 marks.json PM-0001-18 notes")
line_sub(SE, one(391), one(395), "T6 SERIES cross_arch_mismatch_count")
line_sub(SE, one(401), one(405), "T7 (i) SERIES twin_hash_mismatch_count")
line_sub(SE, one(409), one(413), "T7 (ii) SERIES twin_hash_mismatch_count")
line_sub(SE, one(420), one(424), "T8 SERIES tick.s.snap_incomplete")
line_sub(FX, one(430), one(434), "T9 fixtures README l1-aarch64-claim")

# checks: marks.json still parses, with only PM-0001-18's notes changed; every new text is in its file (whitespace
# aside); changed prose lines fit 120 characters (table rows and JSON aside)
before, after = json.loads("\n".join(ORIG[MJ])), json.loads("\n".join(FILES[MJ]))
assert json.dumps(before, sort_keys=True).replace(one(381), one(385), 1) == json.dumps(after, sort_keys=True), MJ
for name, text in NEW:
    flat = " ".join(" ".join(FILES[name]).split())
    assert " ".join(text.split()) in flat, (name, text[:60])
for name in FILES:
    diff = list(difflib.unified_diff(ORIG[name], FILES[name], n=0, lineterm=""))[2:]
    added = [l[1:] for l in diff if l.startswith("+") and not l.startswith("+++")]
    removed = [l[1:] for l in diff if l.startswith("-") and not l.startswith("---")]
    long = [l[:60] for l in added if name != MJ and not l.startswith("|") and len(l) > 120]
    b = hashlib.sha256("\n".join(ORIG[name]).encode()).hexdigest()[:8]
    a = hashlib.sha256("\n".join(FILES[name]).encode()).hexdigest()[:8]
    print(f"{name}: +{len(added)} -{len(removed)} lines; sha256 {b} -> {a}; over 120: {long}")
print("\n".join(LOG))
if APPLY:
    for name, d in FILES.items():
        with open(f"{S}/{name}", "w", encoding="utf-8") as fh:
            fh.write("\n".join(d))
    print("APPLIED")
else:
    print("dry run: nothing written")
