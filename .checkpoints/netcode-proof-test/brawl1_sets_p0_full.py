"""Write the advisor's ruling on BRAWL-1's P0 re-run set (workflow wf_abd08482-f78) word for word from the journal into
advisor-brawl1-sets-p0-full.md. Every text field is copied as the agent returned it; only the headings are the script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_sets_p0_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_abd08482-f78/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-sets-p0-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
keys = [("summary", "Summary"), ("condition15", "Condition 15 for P0"), ("tick16", "The tick16 bite"),
        ("commit_and_close", "The commit and the close"), ("report_words", "The report words"),
        ("to_mustafa", "For Mustafa")]
out = ["# Advisor: the ruling on BRAWL-1's P0 re-run set (workflow wf_abd08482-f78), 2026-11-15", "",
       "Written by script, word for word, from the workflow's journal. Its notes: advisor-brawl1-sets-p0.md. Nothing "
       "below is the main session's.", ""]
for k, h in keys:
    out += [f"## {h}", "", v[k], ""]
text = "\n".join(out)
assert all(v[k] in text for k, _ in keys)
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    print(f"{OUT}: {text.count(chr(10)) + 1} lines, sha256 {hashlib.sha256(fh.read()).hexdigest()}")
