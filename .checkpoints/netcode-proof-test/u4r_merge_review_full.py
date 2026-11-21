"""Write the advisor's review of the pushed head 71ee13d (U4r's merge into the spike branch) word for word from the
workflow's journal (wf_ed59da85-818, resumed after the 14:14 cut) into advisor-u4r-merge-full.md: the verdict and head,
the checks, the findings, the next steps and "For Mustafa". Every text field is copied as the agent returned it; only
the headings and the bullet labels are the script's. Asserted: one completed result.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_merge_review_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_ed59da85-818/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-u4r-merge-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
out = ["# Advisor: the review of the pushed head 71ee13d, U4r's merge into the spike branch (workflow wf_ed59da85-818), "
       "2026-11-14", "",
       "Written by script, word for word, from the workflow's journal (advisor-u4r-merge's StructuredOutput result; the "
       "run was cut by the spend limit at 14:14 and resumed at 15:32). Its notes: advisor-u4r-merge.md. Nothing below is "
       "the main session's.", "", f"## Verdict: {v['verdict']}", "", v["head"], "", "## Checks", ""]
out += [f"- {c}" for c in v["checks"]] + ["", "## Findings", ""]
if not v["findings"]:
    out += ["None.", ""]
for f in v["findings"]:
    out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **Wrong:** {f['wrong']}",
            f"- **Fix:** {f['fix']}", ""]
out += ["## Next steps", ""] + [f"{i}. {s}" for i, s in enumerate(v["next_steps"], 1)]
out += ["", "## For Mustafa", "", v["to_mustafa"], ""]
text = "\n".join(out)
assert v["head"] in text and v["to_mustafa"] in text and all(s in text for s in v["next_steps"] + v["checks"])
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {len(out)} lines, sha256 {digest}; verdict {v['verdict']}; findings "
      f"{[(f['id'], f['severity']) for f in v['findings']]}; {len(v['next_steps'])} next steps")
