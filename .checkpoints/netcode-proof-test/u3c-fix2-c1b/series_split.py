"""U3c C1b: move SERIES.md's 70 per-second split rows into a run-level `event.ws_split` table (the advisor's (b)),
add the two violations rows, and reword "this second" to the run. Idempotent: refuses if the new heading exists.

Usage: python3 -I series_split.py <path to marks/SERIES.md>
"""
import re
import sys

path = sys.argv[1]
text = open(path, encoding="utf-8").read()
HEAD = "### The per-release split over the run (`event.ws_split`)"
if HEAD in text:
    sys.exit("already moved")

lines = text.split("\n")
pat = re.compile(r"^\| `second\.v\.ws_late_(down|up)_(woken|clock|hop|work|other)")
moved = [l for l in lines if pat.match(l)]
assert len(moved) == 70, len(moved)
kept = [l for l in lines if not pat.match(l)]

WRITER = "npt-net (DelayStream; the owning process's writer: npt-server on RS)"


def reword(l: str) -> str:
    l = l.replace("| `second.v.", "| `event.ws_split.", 1)
    l = l.replace("x0 for this second", "x0 over the run")
    l = l.replace("over the second, x0 its median", "over the run, x0 its median")
    l = l.replace(" class this second (", " class over the run, pooled over every connection (")
    l = l.replace(
        "a down record released there is written at the pump's next pass",
        "a down record released there is written by the pump's next pass, which the pump wakes for at once (R1)",
    )
    assert "second" not in l.split("|", 2)[2], l
    return l


out_rows = []
for d, where in (("down", "server to client (RS's TCP sends; model end 0)"), ("up", "client to server (the client's TCP; model end 1)")):
    rows = [reword(l) for l in moved if l.startswith(f"| `second.v.ws_late_{d}_")]
    assert len(rows) == 35, (d, len(rows))
    out_rows += rows
    out_rows.append(
        f"| `event.ws_split.ws_late_{d}_split_violations_count` | count | {WRITER} | {where}, the release lateness's "
        "per-release split (**The per-release split** above): releases whose recorded parts did not sum to their L "
        "(checked per release and counted; 0 while the stamps chain, as npt-net's identity test shows) |"
    )

table = [
    HEAD,
    "",
    "| Name | Unit | Written by (crate) | Meaning |",
    "|---|---|---|---|",
    *out_rows,
    "",
]
anchor = "### The `ws_summary` event's own fields (`event.ws_summary`)"
i = kept.index(anchor)
kept[i:i] = table
open(path, "w", encoding="utf-8").write("\n".join(kept))
print(f"moved {len(moved)} rows, added 2; table before line {i + 1}")
