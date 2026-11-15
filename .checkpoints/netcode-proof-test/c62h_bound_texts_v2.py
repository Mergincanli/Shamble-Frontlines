"""C62H-BOUND: the exact texts with the advisor's word-level conditions applied (advisor-c62h-bound-full.json: C1, C2's
phrase, C4, C8), taken by script, word for word, from the design (c62h-bound-design-full.json) and written to
c62h-bound-texts-v2.md: one fenced block per text between BEGIN/END markers, each block's sha256 printed.
Every replacement must match once, and every new string must appear word for word in the advisor's condition text.
C2's phrase goes into the snap_lost_unexplained_count row's Gaps sentence right after "a room tick record without it,"
(the position is the main session's choice; the advisor's next run checks it). Nothing else changes. Refuses to run
twice.  Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/c62h_bound_texts_v2.py"""
import hashlib
import json
import os

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
OUT = f"{N}/c62h-bound-texts-v2.md"
assert not os.path.exists(OUT), "already ran"
D = json.load(open(f"{N}/c62h-bound-design-full.json", encoding="utf-8"))
A = json.load(open(f"{N}/advisor-c62h-bound-full.json", encoding="utf-8"))
cond = {c.split(" ", 1)[0]: c for c in A["conditions"]}
assert set(cond) >= {"C1", "C2", "C4", "C8"}, sorted(cond)
e = D["exact_texts"]

OLD1 = "L1 P2, P3 and B1: every client-tick left incomplete is one"
NEW1 = "L1 P2, P3 and B1: every measured client-tick left incomplete is one"
OLD2 = "L1 P2, P3 and B1: no client-tick filed whole is one"
NEW2 = "L1 P2, P3 and B1: no measured client-tick filed whole is one"
for s in (NEW1, NEW2):
    assert s in cond["C1"], s
marks = e["marks_json"].replace(OLD1, NEW1) if e["marks_json"].count(OLD1) == 1 else None
assert marks and marks.count(OLD2) == 1
marks = marks.replace(OLD2, NEW2)

pw = D["plain_words"]
ADD = "in each run's measured ten minutes (not its warm-up or its last few seconds)"
assert ADD in cond["C1"] and pw.count('files as "lost" now needs') == 1
pw = pw.replace('files as "lost" now needs', f'files as "lost" {ADD} now needs')
OLD4 = "a client that files a received snapshot as lost fails the first check"
NEW4 = "a planted bug in the lab's per-client log writer that files one snapshot per client, received whole, as lost,"
assert NEW4.rstrip(",") in cond["C4"] and pw.count(OLD4) == 1
pw = pw.replace(OLD4, NEW4 + " fails the first check")

series = e["series_rows"]
OLD8 = "| -17 (its L1 P0 check; snap_lost_unexplained_count), -58, -19 (the coverage rule) |"
NEW8 = "-17 (the coverage rule, its L1 P0 check and its report; snap_lost_unexplained_count), -58, -19 (the coverage rule)"
assert NEW8 in cond["C8"] and series.count(OLD8) == 1
series = series.replace(OLD8, f"| {NEW8} |")
OLD2G = "Gaps: no room log gives `tick.s.snap_link_lost_count`, a room tick record without it, a list"
NEW2G = ("Gaps: no room log gives `tick.s.snap_link_lost_count`, a room tick record without it, a room tick record "
         "without `hs.server_hash`, a list")
assert "a room tick record without hs.server_hash" in cond["C2"] and series.count(OLD2G) == 1
series = series.replace(OLD2G, NEW2G)

readme = e["readme"]
OLDR = "On L1's P2, P3 and B1 a tick filed lost must have a reason"
assert "a measured tick filed lost" in cond["C1"] and readme.count(OLDR) == 1
readme = readme.replace(OLDR, "On L1's P2, P3 and B1 a measured tick filed lost must have a reason")

# The frozen pieces (C9): the two check objects, the bite object, the three new SERIES rows, the :320 row, README's
# sentence. Each is cut from the texts above by its own markers.
checks = marks[marks.index('inside "checks":\n') + len('inside "checks":\n'):marks.index('\nIn "bites"')]
bite = marks[marks.index("this entry follows it:\n") + len("this entry follows it:\n"):marks.index("\nNothing else in")]
rows = [l for l in series.split("\n") if l.startswith("| `")]
assert len(rows) == 4, len(rows)
sentence = [l for l in readme.split("\n") if l.startswith("On L1's P2, P3 and B1 a measured tick")]
assert len(sentence) == 1
blocks = [("plain-words", pw), ("marks-json-placement", marks), ("check-objects", checks), ("bite-object", bite),
          ("series-row-snap_lost_unexplained_count", rows[0]), ("series-row-snap_whole_despite_link_loss_count", rows[1]),
          ("series-row-tick.s.snap_link_lost_count", rows[2]), ("series-row-tick.s.snap_incomplete", rows[3]),
          ("series-placement", series), ("readme-sentence", sentence[0]), ("readme-placement", readme)]
assert rows[0].startswith("| `snap_lost_unexplained_count`") and rows[1].startswith("| `snap_whole_despite")
assert rows[2].startswith("| `tick.s.snap_link_lost_count`") and rows[3].startswith("| `tick.s.snap_incomplete`")
out = ["# C62H-BOUND: the exact texts, v2 (the design's texts with the advisor's C1, C2, C4 and C8 applied by script)",
       "", "Written by c62h_bound_texts_v2.py from c62h-bound-design-full.json and advisor-c62h-bound-full.json. Each "
       "block is word for word; nothing here is retyped. Not yet ruled: the test-engineer's read (TE-CB-1 to -10).", ""]
for name, body in blocks:
    out += [f"<!-- BEGIN {name} -->", "```", body.rstrip("\n"), "```", f"<!-- END {name} -->", ""]
    print(f"{name}: {body.count(chr(10)) + 1} lines, sha256 {hashlib.sha256(body.rstrip(chr(10)).encode()).hexdigest()}")
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write("\n".join(out))
print(f"{OUT}: {len(out)} lines")
