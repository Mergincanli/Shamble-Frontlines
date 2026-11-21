"""Write the advisor's check of the night plan's fix pass (workflow wf_6df5140c-eeb, agent advisor-night-plan-fixes) word
for word from the journal into advisor-night-plan-fixes-full.md. Only the headings and the field labels are the
script's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/night_plan_fixes_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_6df5140c-eeb/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-night-plan-fixes-full.md"
assert not os.path.exists(OUT), OUT
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == {"advisor-night-plan-fixes"}, sorted(res)
a = res["advisor-night-plan-fixes"]
assert isinstance(a, dict)
out = ["# Advisor: the check of the night plan's fix pass, 5afc772..7812127 (workflow wf_6df5140c-eeb), 2026-11-15", "",
       "Written by script, word for word, from the workflow's journal. Nothing below is the main session's. Its notes: "
       "advisor-night-plan-fixes.md.", "", f"- **Verdict:** {a['verdict']}", "", "## Summary", "", a["summary"], "",
       "## Conditions", ""]
vals = [a["verdict"], a["summary"]]
for c in a["conditions_met"]:
    out.append(f"- **{c['id']}** · **met:** {'yes' if c['met'] else 'NO'} · {c['evidence']}")
    vals += [c["id"], c["evidence"]]
out += ["", "## Other changes", ""]
for c in a["other_changes"]:
    out.append(f"- **{c['change']}** · **right:** {'yes' if c['ok'] else 'NO'} · {c['note']}")
    vals += [c["change"], c["note"]]
out += ["", "## Findings", ""]
for f in a["findings"]:
    out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **What:** {f['what']}",
            f"- **Fix:** {f['fix']}", ""]
    vals += [f[x] for x in ("id", "severity", "where", "what", "fix")]
if not a["findings"]:
    out += ["(none)", ""]
out += ["## To Mustafa", "", a["to_mustafa"], ""]
vals.append(a["to_mustafa"])
text = "\n".join(out)
assert all(v in text for v in vals)
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    print(f"{OUT}: {text.count(chr(10)) + 1} lines, sha256 {hashlib.sha256(fh.read()).hexdigest()}")
