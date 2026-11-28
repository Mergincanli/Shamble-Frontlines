"""The merge of main's PRs #50 and #51 (main 2aa5e9c, the legacy game into legacy/) into the netcode branch, as Mustafa
asked on 2026-11-12 ("docs/DECISIONS.md and docs/STATUS.md will conflict: keep both sides, as you did for PR #49"):
each conflict keeps both sides' lines as they are, no line retyped; only the conflict markers go.
- docs/DECISIONS.md, one conflict: main's pointer (3 lines, a sub-bullet of the SPLIT (a) and SCOPE (a) entry just
  above) first, then this branch's block "Asked of Mustafa, the netcode proof test; answered 2026-11-06" (22 lines).
- docs/STATUS.md, one conflict in "In flight": this branch's #40 row, its state cell brought up to date (as at the
  PR #49 merge), then main's #51 row. #47's row is not kept: main dropped it (#47 was replaced by #50, merged), and
  this branch never changed it.
Run in the main folder while the merge of origin/main is in progress:
  python3 -I .checkpoints/netcode-proof-test/merge_main_51.py [--apply]"""
import sys

APPLY = "--apply" in sys.argv


def read(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read().split("\n")


def conflicts(lines):
    """[(start, mid, end, ours, theirs)] for each <<<<<<< HEAD / ======= / >>>>>>> origin/main block."""
    out, i = [], 0
    while i < len(lines):
        if lines[i] == "<<<<<<< HEAD":
            m = lines.index("=======", i)
            e = lines.index(">>>>>>> origin/main", m)
            assert not any(l.startswith(("<<<<<<<", "|||||||")) for l in lines[i + 1:e]), (i, e)
            out.append((i, m, e, lines[i + 1:m], lines[m + 1:e]))
            i = e
        i += 1
    return out


# DECISIONS.md
D = "docs/DECISIONS.md"
d = read(D)
cs = conflicts(d)
assert len(cs) == 1, len(cs)
s, m, e, ours, theirs = cs[0]
assert len(ours) == 22 and ours[0].startswith("- **Asked of Mustafa, the netcode proof test; answered 2026-11-06**")
assert len(theirs) == 3 and theirs[0].startswith("    - _Pointer: the foundation chat's own record of SPLIT (a)")
assert d[s - 1].startswith("    >    to legacy/ only)"), d[s - 1]
d2 = d[:s] + theirs + ours + d[e + 1:]
print(f"{D}: conflict at lines {s + 1}-{e + 1}: main's {len(theirs)} lines, then this branch's {len(ours)}")

# STATUS.md
S = "docs/STATUS.md"
st = read(S)
cs = conflicts(st)
assert len(cs) == 1, len(cs)
s2, m2, e2, ours2, theirs2 = cs[0]
assert len(ours2) == 2 and ours2[0].startswith("| #40 |") and ours2[1].startswith("| #47 |"), ours2
assert len(theirs2) == 2 and theirs2[0].startswith("| #40 |") and theirs2[1].startswith("| #51 |"), theirs2
old_state = "| draft; U1-U3 done; U4 wave 1: U4s, U4b and U4a merged, U4c, U4d and U4j in fix or review |"
new_state = ("| draft; U1-U3 done; U4 wave 1 merged (U4s, U4a, U4b, U4c, U4d, U4j) and U4d-F; wave 2: U4e in review "
             "round 2, U4d2's brief approved |")
assert ours2[0].count(old_state) == 1, "the #40 state cell"
row40 = ours2[0].replace(old_state, new_state)
st2 = st[:s2] + [row40, theirs2[1]] + st[e2 + 1:]
print(f"{S}: conflict at lines {s2 + 1}-{e2 + 1}: this branch's #40 row (state brought up to date), main's #51 row")

for path, new in ((D, d2), (S, st2)):
    assert not any(l.startswith(("<<<<<<<", "=======", ">>>>>>>")) and l in ("<<<<<<< HEAD", "=======",
                   ">>>>>>> origin/main") for l in new), path
if APPLY:
    for path, new in ((D, d2), (S, st2)):
        with open(path, "w", encoding="utf-8") as fh:
            fh.write("\n".join(new))
    print("applied")
else:
    print("dry run: nothing written")
