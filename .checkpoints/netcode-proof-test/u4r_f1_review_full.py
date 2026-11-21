"""Write infra-engineer's review of U4r-F1 (workflow wf_9ca6c622-b4f) word for word from the journal into
u4r-f1-review-full.md: the verdict, its summary and each finding. Every text field is copied as the agent returned it;
only the headings and the bullet labels are the script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_f1_review_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_9ca6c622-b4f/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-f1-review-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
out = ["# U4r-F1's review: infra-engineer, its one round (workflow wf_9ca6c622-b4f), 2026-11-14", "",
       "Written by script, word for word, from the workflow's journal. The diff reviewed: the two run scripts, uncommitted "
       "in the main folder (125 insertions, 48 deletions). Its notes: u4r-f1-review-infra.md. Nothing below is the main "
       "session's.", "", f"## Verdict: {v['verdict']}", "", v["summary"], "", "## Findings", ""]
for f in v["findings"]:
    out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **Wrong:** {f['wrong']}",
            f"- **Evidence:** {f['evidence']}", f"- **Fix:** {f['fix']}", ""]
text = "\n".join(out)
assert v["summary"] in text and all(f["fix"] in text for f in v["findings"])
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {len(out)} lines, sha256 {digest}; verdict {v['verdict']}; findings "
      f"{[(f['id'], f['severity']) for f in v['findings']]}")
