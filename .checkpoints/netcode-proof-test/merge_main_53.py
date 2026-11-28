"""The merge of main 53272a1 (PR #52's docs pass at 4460ef5 and PR #53, the kept-logs rule) into the netcode branch, after
Mustafa's KL-MERGE (a) of 2026-11-15 20:50 Amsterdam ("After the merge this branch takes main in", history.md:2163-2175),
done as at MAIN-48 (merge_main_48.py): docs/STATUS.md's one conflict keeps both sides, no line retyped but the #40 state
cell; only the conflict markers go; the file stays within 120 lines.
- docs/STATUS.md, one conflict in "In flight", read against the merge base 4d21c1a: both sides changed only the #40 row's
  state cell (cells 0-2 and 4 equal on both sides), so one #40 row is kept with its state brought up to date (as at the
  PR #49, #51 and #48 merges); the #48 row is not kept: main dropped it (#48 merged) and this branch never changed it
  (as #51's at the MAIN-48 merge); the #52 row is main's change, kept as main has it.
Run in the main folder while the merge of origin/main is in progress:
  python3 -I .checkpoints/netcode-proof-test/merge_main_53.py [--apply]"""
import subprocess
import sys

APPLY = "--apply" in sys.argv
BASE = "4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300"
MAIN = "53272a11024dcec8349e67bd8c70a8db337e85bb"
S = "docs/STATUS.md"


def show(rev):
    return subprocess.run(["git", "show", f"{rev}:{S}"], capture_output=True, text=True, check=True).stdout.split("\n")


def cells(row):
    return [c.strip() for c in row.split("|")[1:-1]]


mb = subprocess.run(["git", "merge-base", "HEAD", "origin/main"], capture_output=True, text=True, check=True)
om = subprocess.run(["git", "rev-parse", "origin/main"], capture_output=True, text=True, check=True)
assert mb.stdout.strip() == BASE and om.stdout.strip() == MAIN, (mb.stdout, om.stdout)
base = {cells(l)[0]: l for l in show(BASE) if l.startswith("| #")}
assert list(base) == ["#40", "#48"], list(base)
with open(S, encoding="utf-8") as fh:
    st = fh.read().split("\n")
s = st.index("<<<<<<< HEAD")
m = st.index("=======", s)
e = st.index(">>>>>>> origin/main", m)
assert st.count("<<<<<<< HEAD") == 1, "one conflict only"
ours, theirs = st[s + 1:m], st[m + 1:e]
assert [cells(r)[0] for r in ours] == ["#40", "#48"] and [cells(r)[0] for r in theirs] == ["#40", "#52"], (ours, theirs)
o40, t40 = cells(ours[0]), cells(theirs[0])
assert o40[:3] == t40[:3] and o40[4] == t40[4] and o40[3] != t40[3], "both changed only the #40 state cell"
assert ours[1] == base["#48"], "#48: this branch never changed it (main dropped it: merged)"
assert ours[0] != base["#40"] and theirs[0] != base["#40"]
new_state = ("draft; U1-U3 done; U4: U4r merged (71ee13d), its judged runs closed, and BRAWL-1 merged (b30314a); "
             "C62H-BOUND approved by Mustafa, its build next; main 53272a1 merged in (its checkpoint, 2026-11-15)")
assert theirs[0].count(f"| {t40[3]} |") == 1
row40 = theirs[0].replace(f"| {t40[3]} |", f"| {new_state} |")
new = st[:s] + [row40, theirs[1]] + st[e + 1:]
assert not any(l in ("<<<<<<< HEAD", "=======", ">>>>>>> origin/main") for l in new)
n_lines = len(new) - (1 if new[-1] == "" else 0)
assert n_lines <= 120, n_lines
print(f"{S}: conflict at lines {s + 1}-{e + 1}: one #40 row (main's, state cell brought up to date), main's #52 row; "
      f"#48's row not kept (main dropped it, this branch never changed it); {n_lines} lines")
print("the #40 row's state cell:", new_state)
if APPLY:
    with open(S, "w", encoding="utf-8") as fh:
        fh.write("\n".join(new))
    print("applied")
else:
    print("dry run: nothing written")
