"""U4d2's launch, the brief's 7.2 items 1, 2 and 8 (u4d2-brief.md rev 1.2, as the advisor's U4d2 brief check folded
it): fill P6 and P7 where they stand outside 7.2 item 3's list, re-point the plan cites of sections 0 to 7 to the plan
as it is now, re-stamp section 0, and say so in a new section-0 bullet and in the changes line.
- The plan moved at 85c08df (the check's L2 edits): 686a583c (1e8c188) -> 3f58ff0d. A cite whose lines all sit in one
  unchanged block is shifted by that block's offset (difflib over the two versions); a cite touching a rewritten block
  is mapped by its text, in MANUAL below (each key must be used once). Section 8 keeps its questions' cites as asked.
- Each changed paragraph is rewrapped within 120 characters from its first changed line to its end only if a line of
  it would pass 120; each change is checked word for word (the unit with white space collapsed equals the old one
  with only that change made). Table rows are edited in their line.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4d2_restamp.py [--apply]"""
import difflib
import hashlib
import re
import subprocess
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
BRIEF, PLAN, HIST = f"{N}/u4d2-brief.md", f"{N}/u4-build-plan.md", f"{N}/history.md"
CARD = ".checkpoints/netcode-proof-test.md"
BASE = "bfda3cd2e4733efc3d3ec9f7c6c3c99bc6c2e8c4"


def sha_b(b):
    return hashlib.sha256(b).hexdigest()


def read_b(p):
    with open(p, "rb") as fh:
        return fh.read()


def git(*a):
    return subprocess.run(["git", *a], capture_output=True, text=True, check=True).stdout


assert sha_b(read_b(BRIEF)).startswith("888566770a0d"), "the brief is not rev 1.2 as folded"
old_plan = git("show", f"1e8c188:{PLAN}")
assert sha_b(old_plan.encode()).startswith("686a583c")
new_plan_b = read_b(PLAN)
assert sha_b(new_plan_b).startswith("3f58ff0d")
for p, h in ((f"{N}/u4s-carries.md", "cdab0240"), (f"{N}/plan.md", "2bef127b"), (f"{N}/advisor-u4df.md", "842ec092"),
             (f"{N}/advisor-u4j-r2.md", "7efa9410"), (f"{N}/advisor-u4e-r1.md", "7c129309")):
    assert sha_b(read_b(p)).startswith(h), p
assert git("diff", "--stat", "28a85e7", BASE, "--", "spikes/") == "", "spikes/ moved since 28a85e7"
assert git("rev-parse", "HEAD").strip() == BASE or subprocess.run(
    ["git", "merge-base", "--is-ancestor", BASE, "HEAD"]).returncode == 0
assert subprocess.run(["git", "merge-base", "--is-ancestor", "adb1bab", BASE]).returncode == 0

old, new = old_plan.split("\n"), new_plan_b.decode().split("\n")
OPS = difflib.SequenceMatcher(a=old, b=new, autojunk=False).get_opcodes()


def shift_of(n):
    for tag, a1, a2, b1, b2 in OPS:
        if a1 < n <= a2:
            return b1 - a1 if tag == "equal" else None
    raise AssertionError(n)


# (rev 1.2's brief line, the old range) -> the new range, by the text each cite points at (both versions read in full)
MANUAL = {
    (101, "863-867"): "873-880",   # the Wave 2 bullet, whole
    (153, "863-864"): "873",       # "U4d2 owns check.mjs and check.test.mjs"
    (168, "670"): "670-671",       # "its `bites` feature forwards to npt-sim's"
    (173, "667-669"): "667-670",   # the exports, begin to "integers only"
    (199, "688-693"): "693-700",   # the third exception's bullet
    (209, "670-671"): "671",       # "A Node script loads it ... streams a recording in"
    (209, "690-691"): "695-696",   # "the test: the Node loader lists the module's exports and refuses any not on the list"
    (212, "690-691"): "695-696",
    (217, "670-671"): "671",
    (231, "672-673"): "673-674",   # HashCompare and U4r's -30 pre-check
    (248, "670-672"): "671-673",   # npt-lab's `hash` subcommand
    (273, "674-693"): "676-700",   # the check.mjs bullet with its sub-bullets
    (273, "863-864"): "873",
    (276, "675-676"): "677-680",   # the `wasm` step's sub-bullet
    (287, "675-676"): "678-679",   # "the runner refusal at check.mjs:1156-1158 stands, its note updated"
    (322, "676"): "679-680",       # "its wall time printed, budget 5 minutes"
    (328, "702"): "702",           # quotes the old words; marked as the plan at 686a583c below
    (341, "677-678"): "681-683",   # the features comparison
    (416, "688-693"): "693-700",
    (423, "692-693"): "699-700",   # plan.md §4.2's "two named exceptions" becomes three
    (502, "696-700"): "703-709",   # the Bites bullet
    (504, "670"): "670-671",       # the `bites` feature forwards
    (734, "676"): "679-680",
    (850, "701-702"): "710-712",   # the Reviewers bullet
}
used = set()
L = read_b(BRIEF).decode().split("\n")
ORIG = list(L)
S8 = [i for i, l in enumerate(L) if l.startswith("## 8. Questions for the advisor")]
assert len(S8) == 1
S8 = S8[0]
LOG = []
GROUP = re.compile(r"plan :(\d+(?:-\d+)?)((?:, :\d+(?:-\d+)?)*)")


def map_range(i, r):
    a, _, b = r.partition("-")
    a, b = int(a), int(b or a)
    shifts = {shift_of(x) for x in range(a, b + 1)}
    if len(shifts) == 1 and None not in shifts:
        s = shifts.pop()
        return f"{a + s}-{b + s}" if b != a else f"{a + s}"
    key = (i + 1, r)
    assert key in MANUAL, ("unmapped", key)
    assert key not in used, key
    used.add(key)
    return MANUAL[key]


def regroup(i, m):
    rngs = [m.group(1)] + re.findall(r":(\d+(?:-\d+)?)", m.group(2))
    out = [map_range(i, r) for r in rngs]
    return "plan :" + ", :".join(out)


# The old words a cite quotes, marked (the advisor's A1 sentence quotes the plan's :702 as it was before L2)
Q_OLD = "so plan :702's \"the step's time in CI\" is read as"
Q_NEW = ("so plan :702's \"the step's time in CI\" (the plan at 686a583c; its :711 now reads \"the step's time against "
         "its budget\") is read as")
assert sum(l.count(Q_OLD) for l in L) == 1 and Q_OLD in L[327]
L[327] = L[327].replace(Q_OLD, Q_NEW)
LOG.append("brief :328: the quoted old words marked (686a583c; :711 now)")

changed = set([327])
for i in range(S8):
    if not GROUP.search(L[i]):
        continue
    new_l = GROUP.sub(lambda m: regroup(i, m), L[i])
    if new_l != L[i]:
        LOG.append(f"brief :{i + 1}: {' | '.join(m.group(0) for m in GROUP.finditer(L[i]))} -> "
                   f"{' | '.join(m.group(0) for m in GROUP.finditer(new_l))}")
        L[i] = new_l
        changed.add(i)
assert used == set(MANUAL), ("MANUAL keys not used", set(MANUAL) - used)

# P6 and P7, filled where they stand outside 7.2 item 3's list (that list keeps the names as the record)
B7 = BASE[:7]
FILLS = [
    (93, "`<base>` (placeholder `<<P6>>`, filled by the main session at launch: the spike branch's head, which holds",
     f"`<base>` (P6, filled by the main session at launch: {BASE}, the spike branch's head with main's PRs #50 and #51 "
     "merged in, which holds"),
    (111, "**The runner:** `<<P7>>` (the main session writes it at launch: u4e-run.sh with the worktree's path and the log",
     "**The runner:** `/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4d2-run.sh` (P7, sha256 d1fd0e7e…, "
     "written by the main session at launch: u4e-run.sh with the worktree's path and the log"),
    (659, "The base: `<<P6>>` (measured, bites, self and the step count at `<base>`; at 28a85e7",
     f"The base: measured 399, bites 446, self 52 of 52, 10 steps (P6: the counts at `<base>`, {B7}, whose spikes/ is "
     "28a85e7's; at 28a85e7"),
    (803, "| yes (`<<P6>>`) |", f"| yes (P6: re-checked at launch; the same at {B7}) |"),
    (812, "1. **`<base>` and its counts (`<<P6>>`).**",
     f"1. **`<base>` and its counts (P6: {B7}; 399, 446, 52 of 52 and 10 steps, its spikes/ being 28a85e7's; done at "
     "launch).**"),
    (817, "2. **The runner (`<<P7>>`).**", "2. **The runner (P7: u4d2-run.sh, sha256 d1fd0e7e…; done at launch).**"),
]
for i, o, n in FILLS:
    assert sum(l.count(o) for l in L) == 1 and o in L[i], (i + 1, o)
    L[i] = L[i].replace(o, n)
    changed.add(i)
    LOG.append(f"brief :{i + 1}: filled ({o[:40]}…)")
assert sum(l.count("<<P6>>") for l in L) == 1 and sum(l.count("<<P7>>") for l in L) == 1  # 7.2 item 3's list only

# Section 0's stamps, re-stamped in place (7.2 item 8)
hist_b, card_b = read_b(HIST), read_b(CARD)
plan_n, hist_n, card_n = new_plan_b.count(b"\n"), hist_b.count(b"\n"), card_b.count(b"\n")
STAMPS = [("(1,079 lines, sha256 686a583c…)", f"({plan_n:,} lines, sha256 {sha_b(new_plan_b)[:8]}…)"),
          ("(1,925 lines, sha256 c62ac831…)", f"({hist_n:,} lines, sha256 {sha_b(hist_b)[:8]}…)")]
# the card's stamp wraps: "...netcode-proof-test.md (602 lines, sha256" / "  6b36da10…); all but the card ..."
ck = [i for i in range(S8) if L[i].endswith("`card` is .checkpoints/netcode-proof-test.md (602 lines, sha256")]
assert len(ck) == 1 and L[ck[0] + 1].startswith("  6b36da10…);"), ck
L[ck[0]] = L[ck[0]][:-len("(602 lines, sha256")] + f"({card_n:,} lines, sha256"
L[ck[0] + 1] = L[ck[0] + 1].replace("6b36da10…", f"{sha_b(card_b)[:8]}…", 1)
changed.update({ck[0], ck[0] + 1})
LOG.append(f"brief :{ck[0] + 1}-{ck[0] + 2}: the card's stamp -> ({card_n:,} lines, sha256 {sha_b(card_b)[:8]}…)")
for o, n in STAMPS:
    hits = [i for i in range(S8) if o in L[i]]
    assert len(hits) == 1, (o, hits)
    L[hits[0]] = L[hits[0]].replace(o, n)
    changed.add(hits[0])
    LOG.append(f"brief :{hits[0] + 1}: stamp {o} -> {n}")


# Rewrap each changed prose unit from its changed line to its end, only where a line would pass 120
def unit_end(i):
    j = i + 1
    while j < len(L) and L[j].strip() and not re.match(r"^\s*(- |\d+\. |\||#|```|\*\*)", L[j]):
        j += 1
    return j


def cont_indent(i, j):
    if j > i + 1:
        return re.match(r"^\s*", L[i + 1]).group(0)
    lead = re.match(r"^(\s*)(- |\d+\. )?", L[i])
    return lead.group(1) + " " * len(lead.group(2) or "")


for i in sorted(changed, reverse=True):
    if L[i].lstrip().startswith("|"):
        continue
    j = unit_end(i)
    if all(len(x) <= 120 for x in L[i:j]):
        continue
    first = re.match(r"^\s*", L[i]).group(0)
    rest = cont_indent(i, j)
    text = " ".join(x.strip() for x in L[i:j])
    w = textwrap.wrap(text, width=120, initial_indent=first, subsequent_indent=rest, break_long_words=False,
                      break_on_hyphens=False)
    assert " ".join(" ".join(w).split()) == " ".join(text.split())
    LOG.append(f"brief :{i + 1}-{j}: rewrapped ({j - i} -> {len(w)} lines)")
    L[i:j] = w

# The new section-0 bullet, after the bullet that ends "U4e (wip/u4e.patch, in its round-1 fix pass under e-r1) is not
# in it."; and the changes line
END0 = "  U4e (wip/u4e.patch, in its round-1 fix pass under e-r1) is not in it."
k = [i for i, l in enumerate(L) if l == END0]
assert len(k) == 1, k
BUL = ("- **Re-stamped at launch** (7.2 items 1, 2 and 8; the main session, u4d2_restamp.py, 2026-11-12): the plan cites "
       "of sections 0 to 7 read the plan as it is now (its stamp below). The advisor's U4d2 brief check's plan edits "
       "(L2, 85c08df) moved its lines below §4.11 and rewrote parts of §5 U4d2 and of the Lanes; each cite was "
       "re-pointed to the lines that now hold its text, and the one that quotes old words says so (2.4 (a)). Section "
       "8 keeps its questions' cites as asked, at the plan's 686a583c (`git show "
       "1e8c188:.checkpoints/netcode-proof-test/u4-build-plan.md`). `<base>` is " + B7 + " (main's PRs #50 and #51 "
       "merged in): its spikes/ is 28a85e7's (`git diff --stat 28a85e7 " + B7 + " -- spikes/` is empty), and its docs/ "
       "differs only by that merge, which moves no docs line this brief cites (ADR 0002's changes lie outside "
       ":256-266; ADR 0001 and 0004 are unchanged). history.md gained an entry at its end and new heading text at "
       ":280; no line this brief cites moved. carries, plan.md, df-ruling, j-r2 and e-r1 are unchanged. 7.1's rows "
       "marked \"yes\" hold at `<base>`: R2, R11, R14 and R19 read spikes/; R17 reads wip/u4e.patch as re-cut at "
       "4300944 (crates/room, crates/server and crates/client only); R20 is P6.")
bw = textwrap.wrap(BUL, width=120, subsequent_indent="  ", break_long_words=False, break_on_hyphens=False)
L[k[0] + 1:k[0] + 1] = bw
CH = "A15.4, L1-7.2.8, P1, P2, P3, P4, P5, P8; not used: block 6's 1.2 replacement (U4e has not merged)."
assert L.count(CH) == 1
ci = L.index(CH)
L[ci + 1:ci + 1] = textwrap.wrap("At launch (7.2 items 1, 2 and 8, by u4d2_restamp.py): P6 and P7 filled, the plan "
                                 "cites of sections 0 to 7 re-pointed, section 0 re-stamped (its new first bullet "
                                 "says how).", width=120, break_long_words=False, break_on_hyphens=False)
LOG.append(f"section 0: the re-stamp bullet ({len(bw)} lines) after :{k[0] + 1}; the changes line gained its launch "
           "sentence")

over =[i + 1 for i, x in enumerate(L) if len(x) > 120 and not x.lstrip().startswith("|")]
over0 = [i + 1 for i, x in enumerate(ORIG) if len(x) > 120 and not x.lstrip().startswith("|")]
assert len(over) <= len(over0), (over, over0)
print("\n".join(LOG))
d = difflib.unified_diff(ORIG, L, BRIEF, BRIEF, lineterm="", n=0)
print("\n".join(d))
data = "\n".join(L)
print(f"brief: {len(ORIG)} -> {len(L)} lines; sha256 after {sha_b(data.encode())[:12]}")
if "--apply" in sys.argv:
    with open(BRIEF, "w", encoding="utf-8") as fh:
        fh.write(data)
    print("applied")
else:
    print("dry run: nothing written")
