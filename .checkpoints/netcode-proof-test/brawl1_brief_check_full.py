"""Write the advisor's conformance check of BRAWL-1's builder brief (workflow wf_12d3f66b-ca1) word for word from the
journal into advisor-brawl1-brief-full.md: the verdict, its summary, each edit, each new decision and "For Mustafa".
Every text field is copied as the agent returned it; only the headings and labels are the script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_brief_check_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_12d3f66b-ca1/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-brief-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
out = ["# Advisor: the conformance check of BRAWL-1's builder brief (workflow wf_12d3f66b-ca1), 2026-11-15", "",
       "Written by script, word for word, from the workflow's journal (advisor-brawl1-brief's StructuredOutput result). "
       "The brief: brawl1-brief.md. Its notes: advisor-brawl1-brief.md. Nothing below is the main session's.", "",
       f"## Verdict: {v['verdict']}", "", v["summary"], "", "## Edits", ""]
for i, e in enumerate(v["edits"], 1):
    out += [f"{i}. **Where:** {e['where']}", f"   - **Gap:** {e['gap']}", f"   - **Words:** {e['words']}"]
if not v["edits"]:
    out += ["None."]
out += ["", "## New decisions", ""]
for i, d in enumerate(v["new_decisions"], 1):
    out += [f"{i}. **Where:** {d['where']}", f"   - **What:** {d['what']}", f"   - **Ruling:** {d['ruling']}"]
if not v["new_decisions"]:
    out += ["None."]
out += ["", "## For Mustafa", "", v["to_mustafa"], ""]
text = "\n".join(out)
assert v["summary"] in text and v["to_mustafa"] in text
assert all(e["words"] in text and e["gap"] in text for e in v["edits"])
assert all(d["ruling"] in text for d in v["new_decisions"])
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {len(out)} lines, sha256 {digest}; verdict {v['verdict']}; {len(v['edits'])} edits; "
      f"{len(v['new_decisions'])} new decisions")
