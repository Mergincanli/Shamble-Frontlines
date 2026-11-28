"""Write the advisor's ruling on U4r review round 2 word for word from the workflow's journal (wf_24167e6e-996) into
advisor-u4r-r2-full.md, in advisor-u4r-r1-full.md's form: the next steps in order, the ruling on the fix pass's two
deviations, "For Mustafa", then each finding. Every text field is copied as the agent returned it; only the headings
and the bullet labels are the script's. Asserted: one result, with every round-2 finding id of u4r-review-r2-full.md.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_ruling_r2_full.py"""
import hashlib
import json
import re

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_24167e6e-996/journal.jsonl")
N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
OUT = f"{N}/advisor-u4r-r2-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result"]
assert len(res) == 1 and isinstance(res[0], dict), len(res)
v = res[0]
with open(f"{N}/u4r-review-r2-full.md", encoding="utf-8") as fh:
    ids = re.findall(r"(?m)^### (\S+) \(", fh.read())
got = [f["id"] for f in v["findings"]]
assert sorted(got) == sorted(ids), (sorted(got), sorted(ids))
out = ["# Advisor: the ruling on U4r review round 2 (workflow wf_24167e6e-996), 2026-11-14", "",
       "Written by script, word for word, from the workflow's journal (advisor-u4r-r2's StructuredOutput result). Its "
       "notes: advisor-u4r-r2.md; the findings: u4r-review-r2-full.md. Nothing below is the main session's.", "",
       "## The main session's next steps, in order", ""]
out += [f"{i}. {s}" for i, s in enumerate(v["next_steps"], 1)] + [""]
out += ["## The fix pass's two deviations", "", v["deviations"], "", "## For Mustafa", "", v["to_mustafa"], "",
        "## Each finding", ""]
for f in v["findings"]:
    out += [f"### {f['id']} ({f['final_severity']}; {f['ruling']})", "", f"- **What happens:** {f['what_happens']}"]
    if f.get("where_it_goes"):
        out.append(f"- **Where it goes:** {f['where_it_goes']}")
    if f.get("until_then"):
        out.append(f"- **Until then:** {f['until_then']}")
    out += [f"- **Why:** {f['why']}", ""]
text = "\n".join(out)
assert v["deviations"] in text and v["to_mustafa"] in text and all(s in text for s in v["next_steps"])
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {len(out)} lines, sha256 {digest}; {len(v['next_steps'])} next steps; rulings "
      f"{[(f['id'], f['final_severity'], f['ruling']) for f in v['findings']]}")
