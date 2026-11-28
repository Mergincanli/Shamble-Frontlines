"""Write the advisor's second ruling on U4r's judged sets, on P0 (workflow wf_d49add99-31f), word for word from the
journal into advisor-u4r-sets-r2-full.md: each ruling (Q1 to Q3) with its why, each finding, the next steps and "For
Mustafa". Every text field is copied as the agent returned it; only the headings and labels are the script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_sets_r2_ruling_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_d49add99-31f/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-u4r-sets-r2-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
out = ["# Advisor: the second ruling on U4r's judged sets, on P0 (workflow wf_d49add99-31f), 2026-11-14", "",
       "Written by script, word for word, from the workflow's journal (advisor-u4r-sets-r2's StructuredOutput result). "
       "Its notes: advisor-u4r-sets-r2.md. Nothing below is the main session's.", "", "## Rulings", ""]
for r in v["rulings"]:
    out += [f"### {r['q']}", "", f"**Ruling:** {r['ruling']}", "", f"**Why:** {r['why']}", ""]
out += ["## Findings", ""]
for f in v["findings"]:
    out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **Wrong:** {f['wrong']}",
            f"- **Fix:** {f['fix']}", ""]
if not v["findings"]:
    out += ["None.", ""]
out += ["## Next steps", ""] + [f"{i}. {s}" for i, s in enumerate(v["next_steps"], 1)]
out += ["", "## For Mustafa", "", v["to_mustafa"], ""]
text = "\n".join(out)
assert all(r["ruling"] in text and r["why"] in text for r in v["rulings"]) and v["to_mustafa"] in text
assert all(f["fix"] in text and f["wrong"] in text for f in v["findings"]) and all(s in text for s in v["next_steps"])
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {len(out)} lines, sha256 {digest}; {len(v['rulings'])} rulings; findings "
      f"{[(f['id'], f['severity']) for f in v['findings']]}; {len(v['next_steps'])} next steps")
