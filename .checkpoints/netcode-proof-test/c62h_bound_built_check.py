"""C62H-BOUND's built check (the build brief c62h-bound-brief.md section 2, sha256 7936074c...; the advisor's C9;
TE-CB-3). Read only. On <root>/spikes/netcode-proof/marks/ it:
- re-checks texts v3 and the freeze (each block's sha256 as the freeze gives it) and prints the paste's sha256;
- shows each frozen block once, whole, in its file: check-objects and bite-object in marks.json; the four row blocks
  in SERIES.md, each a whole line, with the two header lines `(38)` and `(19)`; readme-sentence in README.md with
  whitespace collapsed (the reflow may break it across lines);
- shows the three files' sha256 equal the brief's expected values, and prints them;
- runs `git diff --word-diff=porcelain <LANE_BASE> -- spikes/netcode-proof/marks/README.md` in <root> and shows its
  only added words are the sentence's, in order, with nothing removed;
- prints marks.json's sha256 "after", which each set's runs.txt must print (c-u4r-runs.mjs:475);
- prints PASS or FAIL per check and exits 1 on any FAIL.
Run from the repository root:
  python3 -I .checkpoints/netcode-proof-test/c62h_bound_built_check.py <root>"""
import hashlib
import os
import re
import subprocess
import sys

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
LANE_BASE = "044fab188419e8859dbca699d851563c44c078c9"
TEXTS_SHA = "3f21c9b7e5d5116db7cae7c0d8d9a4011d2ab9797c75b907f00554f0996844eb"
FREEZE_SHA = "9cb8be172eaa73608ecee5b46e2dc5cfd44e8c2b3a26ca7a74bffb5ef4e82950"
PASTE_SHA = "cfac7502b4a4ba9ea058e31fa7bc16fa324b3fa62a88ecdc5117e2614a57469a"
AFTER = {"marks.json": "c1935a5b996fb0aaff82b0b3c1c604255e1534f6ca7e8d2e55a9f472cd3b22a9",
         "SERIES.md": "6898e119109a794a7779ad0395301e48a461848b60cfdc16431ec64faad87460",
         "README.md": "f3c3c804c4e4d078ca07461f956eb60bc96e74eb4ed507bf28f652383a5742fe"}
BLOCK_RE = r"^<!-- BEGIN (\S+) -->\n```\n(.*?)\n```\n<!-- END \1 -->$"  # c62h_bound_freeze.py's regex
ROWS = ["series-row-tick.s.snap_link_lost_count", "series-row-snap_lost_unexplained_count",
        "series-row-snap_whole_despite_link_loss_count", "series-row-tick.s.snap_incomplete"]
HEADERS = ["## Figures (38): computations npt-judge carries in code", "## Series read only by figures (19)"]
results = []


def sha(b):
    return hashlib.sha256(b).hexdigest()


def check(name, ok, detail=""):
    results.append(bool(ok))
    print(f"{'PASS' if ok else 'FAIL'}  {name}" + (f" ({detail})" if detail else ""))


def main():
    if len(sys.argv) != 2:
        print(__doc__)
        sys.exit(2)
    root = os.path.abspath(sys.argv[1])
    d = os.path.join(root, "spikes", "netcode-proof", "marks")
    head = subprocess.run(["git", "rev-parse", "HEAD"], cwd=root, capture_output=True, text=True, check=True).stdout
    print(f"c62h_bound_built_check on {d}; HEAD {head.strip()}; LANE_BASE {LANE_BASE}")
    v3raw = open(f"{N}/c62h-bound-texts-v3.md", "rb").read()
    fz = open(f"{N}/c62h-bound-freeze.txt", "rb").read()
    paste = open(f"{N}/c62h-bound-paste.md", "rb").read()
    check("texts v3 sha256 as the brief gives", sha(v3raw) == TEXTS_SHA, sha(v3raw))
    check("freeze sha256 as the brief gives", sha(fz) == FREEZE_SHA, sha(fz))
    want = {}
    for line in fz.decode("utf-8").split("\n"):
        if line and not line.startswith("#"):
            name, _, h = line.rpartition(" ")
            want[name] = h
    check("the paste sha256 as posted and as the freeze records", sha(paste) == PASTE_SHA
          and want.get("c62h-bound-paste.md (as posted)") == PASTE_SHA, sha(paste))
    print(f"the paste: c62h-bound-paste.md sha256 {sha(paste)}")
    blocks = {m.group(1): m.group(2) for m in re.finditer(BLOCK_RE, v3raw.decode("utf-8"), re.M | re.S)}
    names = [n for n in want if n not in ("c62h-bound-paste.md (as posted)", "c62h-bound-texts-v3.md")]
    check("the freeze names 8 blocks", len(names) == 8, str(len(names)))
    for n in names:
        got = sha(blocks[n].encode("utf-8")) if n in blocks else "missing"
        check(f"freeze: block {n} sha256 as the freeze gives", got == want[n], got)
    marks = open(os.path.join(d, "marks.json"), "rb").read()
    series = open(os.path.join(d, "SERIES.md"), "rb").read()
    readme = open(os.path.join(d, "README.md"), "rb").read()
    mt, sl, rt = marks.decode("utf-8"), series.decode("utf-8").split("\n"), readme.decode("utf-8")
    for n in ("check-objects", "bite-object"):
        c = mt.count(blocks[n])
        check(f"marks.json holds {n} once, whole", c == 1, f"{c} times")
    for n in ROWS:
        c = sl.count(blocks[n])
        check(f"SERIES.md holds {n} once, as a whole line", c == 1, f"{c} times")
    for h in HEADERS:
        c = sl.count(h)
        check(f"SERIES.md holds the header line '{h}' once", c == 1, f"{c} times")
    s = " ".join(blocks["readme-sentence"].split())
    c = " ".join(rt.split()).count(s)
    check("README.md holds readme-sentence once (whitespace collapsed)", c == 1, f"{c} times")
    for f, b in (("marks.json", marks), ("SERIES.md", series), ("README.md", readme)):
        check(f"{f} sha256 equals the brief's expected {AFTER[f]}", sha(b) == AFTER[f], sha(b))
    wd = subprocess.run(["git", "diff", "--word-diff=porcelain", LANE_BASE, "--",
                         "spikes/netcode-proof/marks/README.md"], cwd=root, capture_output=True, text=True,
                        check=True).stdout
    added, removed, hunk = [], [], False
    for line in wd.split("\n"):
        if line.startswith("@@"):
            hunk = True
        elif hunk and line.startswith("+"):
            added.append(line[1:])
        elif hunk and line.startswith("-"):
            removed.append(line[1:])
    aw = " ".join(added).split()
    check("README.md's word diff from LANE_BASE adds only the sentence's words, in order, and removes nothing",
          aw == blocks["readme-sentence"].split() and not removed,
          f"{len(aw)} words added in {len(added)} runs, {len(removed)} runs removed")
    print(f"marks.json sha256 after: {sha(marks)}")
    nf = results.count(False)
    print(f"c62h_bound_built_check: {results.count(True)} PASS, {nf} FAIL")
    sys.exit(1 if nf else 0)


main()
