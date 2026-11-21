"""Write the advisor's close check of BRAWL-1's report section (workflow wf_631f7311-f96, agent advisor-close) word for
word from the journal into advisor-brawl1-close-full.md. Only the headings are the script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_close_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_631f7311-f96/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-close-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
keys = [("verdict", "Verdict"), ("true_to_files", "True to the files"), ("corrections", "The corrections"),
        ("close", "The close"), ("to_mustafa", "For Mustafa")]
out = ["# Advisor: the close check of BRAWL-1's report section (workflow wf_631f7311-f96), 2026-11-15", "",
       "Written by script, word for word, from the workflow's journal. Its notes: advisor-brawl1-close.md. Nothing below "
       "is the main session's.", ""]
for k, h in keys:
    out += [f"## {h}", "", v[k], ""]
text = "\n".join(out)
assert all(v[k] in text for k, _ in keys)
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    print(f"{OUT}: sha256 {hashlib.sha256(fh.read()).hexdigest()}")
