"""Write the night-u4f scout's result (workflow wf_24144405-900, agent night-u4f-scout) word for word from the
journal into night-u4f-scout-full.md. Only the headings and the field labels are the script's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/night_u4f_scout_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_24144405-900/journal.jsonl")
N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
OUT = f"{N}/night-u4f-scout-full.md"
assert not os.path.exists(OUT), OUT
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == {"night-u4f-scout"}, sorted(res)
r = res["night-u4f-scout"]
assert isinstance(r, dict)
NOTE = "Written by script, word for word, from the workflow's journal. Nothing below is the main session's."
out = ["# The night-u4f scout: every input of U4f's brief, with line ranges (workflow wf_24144405-900), 2026-11-15", "",
       NOTE + " Its notes: night-u4f-scout.md.", "", f"- **HEAD:** {r['head_sha']}", ""]
vals = [r["head_sha"]]


def section(title, items, fields):
    out.extend([f"## {title}", ""])
    for it in items:
        out.append("- " + " · ".join(f"**{f}:** {it[f]}" for f in fields))
        vals.extend(it[f] for f in fields)
    if not items:
        out.append("(none)")
    out.append("")


section("Sections", r["sections"], ["file", "lines", "what"])
section("Carries", r["carries"], ["entry", "lines", "what"])
section("Rulings", r["rulings"], ["file", "lines", "id", "what"])
section("The card's lines", r["card_lines"], ["lines", "what"])
section("Mustafa's standing rules (pointers only)", r["mustafa_rules"], ["file", "lines", "name", "bearing"])
section("Open items", r["open_items"], ["id", "refs", "bearing"])
section("The latest brief's headings", r["brief_shape"]["headings"], ["lines", "heading"])
section("The advisor's conditions on the latest brief", r["brief_shape"]["conditions"], ["id", "lines", "required"])
d = r["disk"]
out.extend(["## Disk", "", f"- **Free (MiB):** {d['free_mib']}", f"- **Estimates:** {d['estimates']}",
            f"- **Refs:** {d['refs']}", ""])
vals.extend([str(d["free_mib"]), d["estimates"], d["refs"]])
section("The code lane", r["lane_files"], ["path", "rule", "refs"])
out.extend(["## Gaps", ""] + [f"- {g}" for g in r["gaps"]] + (["(none)"] if not r["gaps"] else []) + [""])
vals.extend(r["gaps"])
text = "\n".join(out)
assert all(v in text for v in vals)
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    print(f"{OUT}: {text.count(chr(10)) + 1} lines, sha256 {hashlib.sha256(fh.read()).hexdigest()}")
