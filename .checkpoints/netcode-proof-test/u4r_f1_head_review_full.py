"""Write the advisor's review of U4r-F1's pushed head f1f131b (workflow wf_6bfb2c19-ce1) word for word from the journal
into advisor-u4r-f1-head-full.md: the verdict, the head, each check, the header-sentence decision, each finding, the next
steps and "For Mustafa". Every text field is copied as the agent returned it; only the headings and labels are the
script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_f1_head_review_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_6bfb2c19-ce1/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-u4r-f1-head-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
out = ["# Advisor: the review of U4r-F1's pushed head (workflow wf_6bfb2c19-ce1), 2026-11-14", "",
       "Written by script, word for word, from the workflow's journal (advisor-u4r-f1-head's StructuredOutput result). "
       "Its notes: advisor-u4r-f1-head.md. Nothing below is the main session's.", "",
       f"## Verdict: {v['verdict']}", "", f"**Head:** {v['head']}", "", "## Checks", ""]
out += [f"{i}. {c}" for i, c in enumerate(v["checks"], 1)]
out += ["", "## INF-F1-1's optional header sentence", "", v["header_sentence"], "", "## Findings", ""]
for f in v["findings"]:
    out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **Wrong:** {f['wrong']}",
            f"- **Fix:** {f['fix']}", ""]
out += ["## Next steps", ""] + [f"{i}. {s}" for i, s in enumerate(v["next_steps"], 1)]
out += ["", "## For Mustafa", "", v["to_mustafa"], ""]
text = "\n".join(out)
assert all(c in text for c in v["checks"]) and v["header_sentence"] in text and v["to_mustafa"] in text
assert all(f["fix"] in text and f["wrong"] in text for f in v["findings"]) and all(s in text for s in v["next_steps"])
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {len(out)} lines, sha256 {digest}; verdict {v['verdict']}; {len(v['checks'])} checks; findings "
      f"{[(f['id'], f['severity']) for f in v['findings']]}; {len(v['next_steps'])} next steps")
