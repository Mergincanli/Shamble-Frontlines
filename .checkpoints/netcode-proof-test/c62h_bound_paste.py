"""C62H-BOUND: assemble the post for Mustafa (c62h-bound-paste.md) by script. The exact texts come word for word from
c62h-bound-texts-v3.md (plain-words, check-objects, bite-object, the four SERIES rows, readme-sentence); the planned size
and the blocks "How this differs ...", "Still unbounded ...", the ADR line, the runs and the freeze come word for word
from the advisor's ruling (advisor-c62h-bound-r2-full.json: planned_size, paste_blocks). Only the headings, the short
intro and the decision entry (as the ruling's other_changes asks for it) are the main session's. Prints the paste's
sha256 and the v3 file's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/c62h_bound_paste.py"""
import hashlib
import json
import os
import re

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
V3, OUT = f"{N}/c62h-bound-texts-v3.md", f"{N}/c62h-bound-paste.md"
assert not os.path.exists(OUT), "already ran"
v3raw = open(V3, "rb").read()
v3 = v3raw.decode("utf-8")
B = {m.group(1): m.group(2) for m in re.finditer(r"^<!-- BEGIN (\S+) -->\n```\n(.*?)\n```\n<!-- END \1 -->$", v3, re.M | re.S)}
assert len(B) == 11
A = json.load(open(f"{N}/advisor-c62h-bound-r2-full.json", encoding="utf-8"))
P, S = A["paste_blocks"], A["planned_size"]
fence = lambda body, lang="": [f"```{lang}", body, "```"]
out = ["# C62H-BOUND (a): the exact wording and the planned size, for your yes", "",
       f"Assembled by script (c62h_bound_paste.py) from c62h-bound-texts-v3.md (sha256 {hashlib.sha256(v3raw).hexdigest()[:16]}…) and the "
       "advisor's ruling (advisor-c62h-bound-r2-full.json). The texts below are word for word; nothing is retyped. "
       "Nothing is built before your yes.", "",
       "## What the limit does", "", B["plain-words"], "",
       "## The exact texts (fixed at your yes)", "",
       "**marks.json, PM-0001-17: two checks, after its \"L1 P0 loses no group\" check**", "", *fence(B["check-objects"], "json"), "",
       "**marks.json, PM-0001-17: one planted bug, after whole-as-lost**", "", *fence(B["bite-object"], "json"), "",
       "**SERIES.md: two new figure rows, one new series row, and the changed `tick.s.snap_incomplete` row** (and the two "
       "header counts: Figures 36 to 38; series read only by figures 18 to 19)", ""]
for row in ("series-row-snap_lost_unexplained_count", "series-row-snap_whole_despite_link_loss_count",
            "series-row-tick.s.snap_link_lost_count", "series-row-tick.s.snap_incomplete"):
    out += [*fence(B[row]), ""]
out += ["**README.md §1, one sentence in the `decoded_hash` paragraph**", "", *fence(B["readme-sentence"]), "",
        "## The planned size", "", "| File | Lines | What |", "|---|---|---|"]
for f in S["by_file"]:
    out.append(f"| {f['file']} | {f['lines']} | {' '.join(l.lstrip('- ').strip() for l in f['what'].split(chr(10)))} |")
out += ["", S["total"], "", S["stop_line"], "", "## How it differs, what stays open, the runs", "",
        P["differs"], "", P["still_unbounded"], "", P["adr_line"], "", P["run_numbers"], "", P["freeze"], "",
        "**Decisions needed:**",
        "- **C62H-BOUND-YES: yes to the exact wording above (texts v3) and the planned size (about 650 changed lines, "
        "545 to 755; the build stops and tells you past 1,050)?**",
        "  - Why it matters: nothing is built before your yes, and the wording is fixed from then on; the judged runs "
        "cannot reword it.",
        "  - Options: (a) yes; (b) yes with changes you name; (c) no, keep only option (c)'s report lines.",
        "  - Recommendation: (a), the advisor's too.", "",
        "**For Mustafa:**", "1. Answer C62H-BOUND-YES.", ""]
text = "\n".join(out)
for k in ("plain-words", "check-objects", "bite-object", "readme-sentence"):
    assert B[k] in text
for k in ("differs", "still_unbounded", "adr_line", "run_numbers", "freeze"):
    assert P[k] in text
assert S["total"] in text and S["stop_line"] in text
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
print(f"{OUT}: {text.count(chr(10))} lines, {len(text)} chars, sha256 {hashlib.sha256(text.encode()).hexdigest()}")
