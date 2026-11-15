"""Write the advisor's review of KL-CHANGE (workflow wf_d7fbca53-76d, agent advisor-kl-change) word for word from the
journal into advisor-kl-change-full.md. Only the headings and the field labels are the script's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/kl_change_review_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_d7fbca53-76d/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-kl-change-full.md"
assert not os.path.exists(OUT), OUT
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == {"advisor-kl-change"}, sorted(res)
a = res["advisor-kl-change"]
assert isinstance(a, dict)
out = ["# Advisor: the review of KL-CHANGE, pack's second walk (workflow wf_d7fbca53-76d), 2026-11-15", "",
       "Written by script, word for word, from the workflow's journal. Nothing below is the main session's. Its notes: "
       "advisor-kl-change.md.", "", f"- **Verdict:** {a['verdict']}", "", "## Summary", "", a["summary"], "",
       "## Against his words", ""]
vals = [a["verdict"], a["summary"]]
for m in a["meets_his_words"]:
    out.append(f"- **{m['point']}** · **met:** {'yes' if m['met'] else 'NOT YET'} · {m['evidence']}")
    vals += [m["point"], m["evidence"]]
out += ["", "## Findings", ""]
for f in a["findings"]:
    out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **What:** {f['what']}",
            f"- **Fix:** {f['fix']}", ""]
    vals += [f[x] for x in ("id", "severity", "where", "what", "fix")]
if not a["findings"]:
    out += ["(none)", ""]
out += ["## Conditions", ""] + [f"{i}. {c}" for i, c in enumerate(a["conditions"], 1)]
if not a["conditions"]:
    out.append("(none)")
vals += a["conditions"]
out += ["", "## Build plan changes", "", a["build_plan_changes"], "", "## To Mustafa", "", a["to_mustafa"], ""]
vals += [a["build_plan_changes"], a["to_mustafa"]]
text = "\n".join(out)
assert all(v in text for v in vals)
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    print(f"{OUT}: {text.count(chr(10)) + 1} lines, sha256 {hashlib.sha256(fh.read()).hexdigest()}")
