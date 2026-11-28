"""The merge of main's PR #48 (main 4d21c1a, foundation 2.2: the toolchain) into the netcode branch, as Mustafa asked
on 2026-11-13 18:14 UTC (MAIN-48, history.md: "docs/STATUS.md will conflict: keep both sides and stay within its 120
lines"): the one conflict keeps both sides' changes, no line retyped but this branch's own #40 state cell; only the
conflict markers go.
- docs/STATUS.md, one conflict in "In flight", read against the merge base 2aa5e9c: the #40 row is this branch's change
  (main's equals the base's), so this branch's row is kept, its state cell brought up to date (as at the PR #49 and
  PR #51 merges); the #51 row is not kept: main dropped it (#51 merged) and this branch never changed it (as #47's at
  the PR #51 merge); the #48 row is main's change (this branch's equals the base's), so main's row is kept as main has
  it. The file stays at 120 lines.
Run in the main folder while the merge of origin/main is in progress:
  python3 -I .checkpoints/netcode-proof-test/merge_main_48.py [--apply]"""
import subprocess
import sys

APPLY = "--apply" in sys.argv
BASE = "2aa5e9ce03e9ab9ccfecd53e6aabf951abe705dd"
S = "docs/STATUS.md"


def read(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read().split("\n")


def show(rev):
    return subprocess.run(["git", "show", f"{rev}:{S}"], capture_output=True, text=True, check=True).stdout.split("\n")


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


mb = subprocess.run(["git", "merge-base", "HEAD", "origin/main"], capture_output=True, text=True, check=True)
assert mb.stdout.strip() == BASE, mb.stdout
base_rows = [l for l in show(BASE) if l.startswith("| #")]
main_rows = [l for l in show("origin/main") if l.startswith("| #")]
assert [r.split("|")[1].strip() for r in base_rows] == ["#40", "#51", "#48"], base_rows
assert [r.split("|")[1].strip() for r in main_rows] == ["#40", "#48"], main_rows

st = read(S)
cs = conflicts(st)
assert len(cs) == 1, len(cs)
s, m, e, ours, theirs = cs[0]
assert len(ours) == 3 and [r.split("|")[1].strip() for r in ours] == ["#40", "#51", "#48"], ours
assert len(theirs) == 2 and [r.split("|")[1].strip() for r in theirs] == ["#40", "#48"], theirs
# Who changed what, against the merge base.
assert ours[0] != base_rows[0] and theirs[0] == base_rows[0], "#40: only this branch changed it"
assert ours[1] == base_rows[1], "#51: this branch never changed it (main dropped it: merged)"
assert ours[2] == base_rows[2] and theirs[1] != base_rows[2], "#48: only main changed it"
old_state = ("| draft; U1-U3 done; U4 wave 1 merged (U4s, U4a, U4b, U4c, U4d, U4j) and U4d-F; wave 2: U4e in review "
             "round 2, U4d2's brief approved |")
new_state = ("| draft; U1-U3 done; U4 wave 1 (U4s, U4a, U4b, U4c, U4d, U4j), U4d-F and wave 2 (U4e, U4d2) merged; "
             "U4r's brief approved (rev 3.2), its build next |")
assert ours[0].count(old_state) == 1, "the #40 state cell"
row40 = ours[0].replace(old_state, new_state)
new = st[:s] + [row40, theirs[1]] + st[e + 1:]
assert not any(l in ("<<<<<<< HEAD", "=======", ">>>>>>> origin/main") for l in new)
n_lines = len(new) - (1 if new[-1] == "" else 0)
assert n_lines <= 120, n_lines
print(f"{S}: conflict at lines {s + 1}-{e + 1}: this branch's #40 row (state brought up to date), main's #48 row; "
      f"#51's row not kept (main dropped it, this branch never changed it); {n_lines} lines")
print("the #40 row's state cell:", new_state)
if APPLY:
    with open(S, "w", encoding="utf-8") as fh:
        fh.write("\n".join(new))
    print("applied")
else:
    print("dry run: nothing written")
