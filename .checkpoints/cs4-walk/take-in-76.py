#!/usr/bin/env python3
"""Builder V4, the take-in of main 2671595 under pull request #75: docs/STATUS.md by script.

Mustafa, 2026-11-26, 18:16:35 UTC: "In those four files keep both sides. Where a line belongs to another chat, main's
text stays unchanged. STATUS ends at 120 lines or fewer: main is at 120 now, so fold your own lines by script."

From the worktree's root, during the merge:
    python3 .checkpoints/cs4-walk/take-in-76.py <main's STATUS.md> <the branch's STATUS.md as at 2e28217>
The two inputs are written by `git show origin/main:docs/STATUS.md` and `git show 2e28217:docs/STATUS.md`.
It writes docs/STATUS.md: main's file, line for line, with lane 1's own lines put in place of lane 1's older ones
(each replaced line of main's is one lane 1 wrote; the script stops if any of them is not in the archive byte for
byte), and it appends to docs/STATUS_ARCHIVE.md the long form of lane 1's lines that are shortened here.
"""
import sys

main = open(sys.argv[1]).read().split("\n")
mine = open(sys.argv[2]).read().split("\n")
archive_path = "docs/STATUS_ARCHIVE.md"
archive = open(archive_path).read()

def block(lines, first, after):
    """The lines from the one that starts with `first` up to (not with) the one that starts with `after`."""
    a = next(i for i, l in enumerate(lines) if l.startswith(first))
    b = next(i for i in range(a + 1, len(lines)) if lines[i].startswith(after))
    return a, b

out = list(main)
replaced = []

def put(first, after, new):
    a, b = block(out, first, after)
    replaced.extend(out[a:b])
    out[a:b] = new

# 1. In flight: lane 1's row, as the branch has it, after main's rows.
row = next(l for l in mine if l.startswith("| #75 (draft) | change-set 4b-1"))
a, b = block(out, "| none yet | MAIN-WALL", "## Next")
assert out[a + 1] == ""
out.insert(a + 1, row)

# 2. "Next", item 2 (lane 1's): the branch's two lines, with the pull request's number.
a, b = block(mine, "2. v2: the netcode proof test (#40)", "3. Lane 2 (Builder V2)")
item = [l.replace("**4b-1 reviewed**", "**4b-1 reviewed (#75)**") for l in mine[a:b]]
assert len(item) == 2 and "(#75)" in item[1]
put("2. v2: the netcode proof test (#40)", "3. Lane 2 (Builder V2)", item)

# 3. "Waiting on Mustafa": lane 1's two bullets from 4a's report and on the release count give way to the branch's
#    two, each on one line here (their long form goes to the archive below).
waiting = [
    "- 4b-1 (#75), reviewed (two rounds, no Critical or High; the verifier and the advisor agent: its report): his go;"
    " the builder's SIGHT-FLAG, DUR-1.",
    "- Later, from 4a's report: MOVE-RULES, NO-SOURCE, RESUME-FLASH, TUNE-HOME, SPRINT-DELAY-4; `docs/UNITS.md` §0's"
    " two lines (his unit walk); DIFF-LIST at 4b-2's data step.",
]
put("- From 4a's report (#70, merged 2026-11-26, 2483e9a)", "- His accounts, his words", waiting)

# 4. Follow-ups: his VISION-RULE line, word for word as the branch has it, and change-set 4b-1's line with
#    4B-AIM-REACH's sentence word for word; its list of ids is shortened to a count (the archive's block has each).
a, b = block(mine, "- **VISION-RULE** (his point 4c", "- **From change-set 4b-1**")
vision = mine[a:b]
assert len(vision) == 4
a, b = block(mine, "- **From change-set 4b-1**", "## Open risks")
from_4b1 = [l for l in mine[a:b] if l]
cut = next(i for i, l in enumerate(from_4b1) if "No numbered change-set holds it yet. Also " in l)
head, tail = from_4b1[cut].split(" Also ", 1)
ids = (tail + " " + " ".join(l.strip() for l in from_4b1[cut + 1:])).split(". Closed at merge:")[0]
count = len([i for i in ids.replace("(the CI Builder)", "").split(",") if i.strip()])
short = from_4b1[:cut] + [head + f" And {count} more ids there (4B-CI-CONTENT is the CI Builder's)."]
a, b = block(out, "- **Merged pull requests' bullets**", "## Open risks")
assert out[b - 1] == ""
out[b - 1:b - 1] = vision + short

# 5. "Open risks", the v2 bullet (lane 1's change-sets 3a, 3b, 4a and 4b-1): the branch's.
a, b = block(mine, "- **v2:** the decision drafts' risks", "- **Preview hosting (lane 2):**")
put("- **v2:** the decision drafts' risks", "- **Preview hosting (lane 2):**", mine[a:b])

# Every line of main's that gave way is lane 1's own and is in the archive already, byte for byte, but for the v2
# bullet's lines, which the branch's bullet rewrites (4A-S3-READ closed) and which are appended below.
missing = [l for l in replaced if l and l + "\n" not in archive]
fold = [l for l in mine if l.startswith("_Last updated: 2026-11-26 (change-set 4b-1's docs pass, second part")]
a, b = block(mine, "- 4b-1, reviewed (two rounds", "- His to give:")
fold += mine[a:b] + from_4b1[cut:]
note = (
    "\n_Moved from STATUS (2026-11-26, change-set 4b-1's take-in of main 2671595 under pull request #75; his word of"
    " 18:16:35 UTC: \"fold your own lines by script\"): lane 1's own lines as they stood on the branch at 2e28217, byte"
    " for byte, where STATUS now holds a shorter form (its top line, its two \"Waiting on Mustafa\" bullets, the list of"
    " ids of \"From change-set 4b-1\"); and main's lines of lane 1's that the branch's lines replace and that were not"
    " in this archive yet:_\n\n"
)
marker = "\n## Targets\n"
assert archive.count(marker) == 1
archive = archive.replace(marker, note + "\n".join(fold + missing) + "\n" + marker)
open(archive_path, "w").write(archive)
open("docs/STATUS.md", "w").write("\n".join(out))
print(f"STATUS: {len(main) - 1} lines on main, {len(out) - 1} now; {len([l for l in replaced if l])} of main's lines gave way,"
      f" {len(missing)} of them newly archived; {len(fold)} of the branch's lines folded; {count} ids counted")
for l in missing: print("  newly archived:", l[:110])
