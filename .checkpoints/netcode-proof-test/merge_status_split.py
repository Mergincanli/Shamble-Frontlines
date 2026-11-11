"""The merge of main's PR #49 (STATUS split into STATUS, DECISIONS and STATUS_ARCHIVE) into the netcode branch, as
Mustafa asked on 2026-11-11: this branch's additions to docs/STATUS.md move into main's three files by line ranges,
never by retyping, and STATUS's "In flight" table keeps one line for #40.
Run in the main folder while the merge of origin/main is in progress (HEAD is still this branch's tip):
  python3 -I merge_status_split.py /home/user/Shamble-Frontlines
It writes docs/STATUS.md, docs/DECISIONS.md and docs/STATUS_ARCHIVE.md, and checks that every moved line is there.

Where each of this branch's ten STATUS hunks goes (by main's own map, STATUS.md "Where STATUS's old sections went"):
- the 2026-11-06 answers block (ours 173-194) -> DECISIONS.md, "Decisions, newest first", before the 2026-11-05 entry;
- MACHINES-TIMING's note (ours 373-374, replacing base 345) -> DECISIONS.md, where that entry now lives;
- the Phase list (ours 69-71, replacing base 65-66), the follow-ups line (ours 453-455, replacing base 424-425), the
  risks entry (ours 865-873, after "## Open bugs and risks"), the one-line fix (ours 904, replacing base 865) and the
  two Health rows (ours 1061-1062, after the Health table's separator) -> STATUS_ARCHIVE.md, under the same names;
- the header's mention (ours 3), the "Also in flight" paragraph (ours 52-55) and the long In flight row (ours 119): not
  moved, superseded by STATUS.md's one #40 line, whose state cell is brought up to date."""
import subprocess
import sys

root = sys.argv[1]
BASE = "44446a5784af7c80b6560fc838fd56f19fc10d32"


def show(ref, path):
    return subprocess.run(["git", "show", f"{ref}:{path}"], cwd=root, capture_output=True, text=True,
                          check=True).stdout.split("\n")


ours = show("HEAD", "docs/STATUS.md")
base = show(BASE, "docs/STATUS.md")
st = show("origin/main", "docs/STATUS.md")
de = show("origin/main", "docs/DECISIONS.md")
ar = show("origin/main", "docs/STATUS_ARCHIVE.md")


def ln(lines, a, b):
    """Lines a to b of a file, 1-based and inclusive."""
    return lines[a - 1:b]


def find_block(doc, block, what):
    hits = [i for i in range(len(doc) - len(block) + 1) if doc[i:i + len(block)] == block]
    assert len(hits) == 1, f"{what}: found {len(hits)} times"
    return hits[0]


def replace(doc, old, new, what):
    i = find_block(doc, old, what)
    return doc[:i] + new + doc[i + len(old):]


def insert_after(doc, index, block):
    return doc[:index + 1] + block + doc[index + 1:]


answers = ln(ours, 173, 194)
assert answers[0].startswith("- **Asked of Mustafa, the netcode proof test; answered 2026-11-06**"), answers[0]
assert ours[194].startswith("- "), "the answers block does not end at line 194"

# DECISIONS.md
de = replace(de, ln(base, 345, 345), ln(ours, 373, 374), "MACHINES-TIMING's old line")
a3 = [i for i, l in enumerate(de) if l.startswith("- **Asked of Mustafa, A3; answered 2026-11-05")]
assert len(a3) == 1, a3
de = de[:a3[0]] + answers + de[a3[0]:]

# STATUS_ARCHIVE.md
ar = replace(ar, ln(base, 65, 66), ln(ours, 69, 71), "the Phase list")
ar = replace(ar, ln(base, 424, 425), ln(ours, 453, 455), "the follow-ups lines")
ar = replace(ar, ln(base, 865, 865), ln(ours, 904, 904), "the checkpoint.mjs line")
open_risks = [i for i, l in enumerate(ar) if l == "## Open bugs and risks" or l.startswith("## Open bugs and risks ")]
assert len(open_risks) == 1, open_risks
ar = insert_after(ar, open_risks[0], ln(ours, 865, 873))
health = [i for i, l in enumerate(ar) if l == "## Health (latest measurements)" or l.startswith("## Health")]
assert len(health) == 1, health
sep = next(i for i in range(health[0], len(ar)) if ar[i].startswith("|---"))
ar = insert_after(ar, sep, ln(ours, 1061, 1062))

# STATUS.md: one line for #40, its state cell brought up to date
rows = [i for i, l in enumerate(st) if l.startswith("| #40 |")]
assert len(rows) == 1, rows
old_cell = "| draft; U1-U3 done, U4 in build |"
assert st[rows[0]].count(old_cell) == 1, st[rows[0]]
st[rows[0]] = st[rows[0]].replace(old_cell, "| draft; U1-U3 done; U4 wave 1: U4s, U4b and U4a merged, U4c, U4d and "
                                            "U4j in fix or review |")

# every moved line is in its new home, in order
for block, doc, what in [(answers, de, "answers"), (ln(ours, 373, 374), de, "MACHINES-TIMING"),
                         (ln(ours, 69, 71), ar, "Phase"), (ln(ours, 453, 455), ar, "follow-ups"),
                         (ln(ours, 865, 873), ar, "risks"), (ln(ours, 904, 904), ar, "one line"),
                         (ln(ours, 1061, 1062), ar, "Health")]:
    find_block(doc, block, what + " (after the move)")

for path, doc in [("docs/STATUS.md", st), ("docs/DECISIONS.md", de), ("docs/STATUS_ARCHIVE.md", ar)]:
    with open(f"{root}/{path}", "w", encoding="utf-8") as fh:
        fh.write("\n".join(doc))
print("moved: answers 22 lines and MACHINES-TIMING 2 -> DECISIONS.md; Phase 3, follow-ups 3, risks 9, one line, "
      "Health 2 -> STATUS_ARCHIVE.md; STATUS.md's #40 row updated; every moved line found once")
