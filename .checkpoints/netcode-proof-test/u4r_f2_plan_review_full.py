"""Write the advisor's plan review of U4r-F2 (workflow wf_4b7d75ec-24c) word for word from the journal into
advisor-u4r-f2-plan-full.md: the verdict, the conditions, the answers to Q1-Q5, the findings, the size and "For
Mustafa". Every text field is copied as the agent returned it; only the headings and labels are the script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_f2_plan_review_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_4b7d75ec-24c/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-u4r-f2-plan-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
out = ["# Advisor: the plan review of U4r-F2 (workflow wf_4b7d75ec-24c), 2026-11-14", "",
       "Written by script, word for word, from the workflow's journal (advisor-u4r-f2-plan's StructuredOutput result). The "
       "plan: u4r-f2-plan.md. Its notes: advisor-u4r-f2-plan.md. Nothing below is the main session's.", "",
       f"## Verdict: {v['verdict']}", "", "## Conditions", ""]
out += [f"{i}. {c}" for i, c in enumerate(v["conditions"], 1)] + ["", "## The choices (the plan's section 8)", ""]
for c in v["choices"]:
    out += [f"- **{c['q']}** {c['answer']} **Why:** {c['why']}"]
out += ["", "## Findings", ""]
for f in v["findings"]:
    out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **Wrong:** {f['wrong']}",
            f"- **Fix:** {f['fix']}", ""]
if not v["findings"]:
    out += ["None.", ""]
out += ["## Size", "", v["size"], "", "## For Mustafa", "", v["to_mustafa"], ""]
text = "\n".join(out)
assert all(c in text for c in v["conditions"]) and v["to_mustafa"] in text and v["size"] in text
assert all(c["answer"] in text and c["why"] in text for c in v["choices"])
assert all(f["fix"] in text and f["wrong"] in text for f in v["findings"])
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {len(out)} lines, sha256 {digest}; verdict {v['verdict']}; {len(v['conditions'])} conditions; "
      f"{len(v['choices'])} choices; findings {[(f['id'], f['severity']) for f in v['findings']]}")
