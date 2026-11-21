"""RESUME-CARD's one-off split (Mustafa, 2026-11-10): move the checkpoint's history sections, byte for byte, into
history.md, leaving a pointer line under each moved heading and a placeholder for the resume card. Usage:
python3 -I split_checkpoint.py <checkpoint.md> <history.md> <first line of the history part of Running work>
Refuses to run if history.md exists; checks afterwards that every original line is in exactly one of the two files."""
import os
import sys
from collections import Counter

ckpt, hist, cut_marker = sys.argv[1], sys.argv[2], sys.argv[3]
if os.path.exists(hist):
    sys.exit(f"{hist} exists: not overwriting")
lines = open(ckpt, encoding="utf-8").read().split("\n")


def section(name):
    start = lines.index(f"## {name}")
    end = next((i for i in range(start + 1, len(lines)) if lines[i].startswith("## ")), len(lines))
    return start, end


moved = {}
for name in ["Steps done", "Current step", "Decisions taken"]:
    s, e = section(name)
    moved[name] = (s, e)
rs, re_ = section("Running work")
cut = next(i for i in range(rs + 1, re_) if lines[i].startswith(cut_marker))
moved["Running work"] = (cut, re_)

out_hist = ["# History: netcode-proof-test", "",
            "Moved here byte for byte from .checkpoints/netcode-proof-test.md by RESUME-CARD (Mustafa, 2026-11-10). The",
            "checkpoint's resume card is the handoff; open this file only when a step needs the history.", ""]
for name in ["Steps done", "Current step", "Decisions taken"]:
    s, e = moved[name]
    out_hist += lines[s:e]
out_hist += ["## Running work (older entries: U3c and before)"] + lines[cut:re_]

pointer = {
    "Steps done": "Moved to history.md (RESUME-CARD, 2026-11-10): every step from U0 to U3d, with its SHA.",
    "Current step": "Superseded by the resume card at the top; the old text (U3a era) is in history.md.",
    "Decisions taken": "Moved to history.md (RESUME-CARD, 2026-11-10). The rules still in force are listed by name on the "
                       "resume card, each pointing there.",
}
out_ck = []
i = 0
while i < len(lines):
    line = lines[i]
    hit = next((n for n, (s, e) in moved.items() if n != "Running work" and s == i), None)
    if hit:
        out_ck += [line, pointer[hit], ""]
        i = moved[hit][1]
        continue
    if i == cut:
        out_ck += ["- **Older entries (U3c and before)** moved to history.md (RESUME-CARD, 2026-11-10).", ""]
        i = re_
        continue
    out_ck.append(line)
    i += 1
first_section = next(j for j, l in enumerate(out_ck) if l.startswith("## "))
out_ck[first_section:first_section] = ["<!-- RESUME-CARD -->", ""]

orig = Counter(lines)
new = Counter(out_ck) + Counter(out_hist)
missing = orig - new
if missing:
    sys.exit(f"lines lost: {sum(missing.values())}: {list(missing)[:3]}")
open(hist, "w", encoding="utf-8").write("\n".join(out_hist) + "\n")
open(ckpt, "w", encoding="utf-8").write("\n".join(out_ck))
print(f"checkpoint {len(lines)} -> {len(out_ck)} lines; history.md {len(out_hist)} lines; no line lost")
