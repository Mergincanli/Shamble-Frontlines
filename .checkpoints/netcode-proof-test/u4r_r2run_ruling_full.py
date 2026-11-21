"""Write the advisor's ruling on U4r's r2 record run and carry 62 (h) word for word from the workflow's journal
(wf_ec319c2d-f45) into advisor-u4r-r2run-full.md: each suite (expected, came, matches, ruling), carry 62 (h), the next
steps and "For Mustafa". Every text field is copied as the agent returned it; only the headings and the bullet labels
are the script's. Asserted: one completed result with all 14 suites of r2.txt.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_r2run_ruling_full.py"""
import hashlib
import json
import re

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_ec319c2d-f45/journal.jsonl")
R2 = "/home/user/Shamble-Frontlines/.gate/netcode-proof-test/r2-e59ece9ecafc/r2.txt"
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-u4r-r2run-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
with open(R2, encoding="utf-8") as fh:
    suites = re.findall(r"(?m)^## suite (\S+) ", fh.read())
assert sorted(s["suite"] for s in v["suites"]) == sorted(suites), ([s["suite"] for s in v["suites"]], suites)
out = ["# Advisor: the ruling on U4r's r2 record run and carry 62 (h) (workflow wf_ec319c2d-f45), 2026-11-14", "",
       "Written by script, word for word, from the workflow's journal (advisor-u4r-r2run's StructuredOutput result). "
       "The run: .gate/netcode-proof-test/r2-e59ece9ecafc/r2.txt. Its notes: advisor-u4r-r2run.md. Nothing below is the "
       "main session's.", "", "## Carry 62 (h)", "", v["carry62h"], "", "## Next steps", ""]
out += [f"{i}. {s}" for i, s in enumerate(v["next_steps"], 1)]
out += ["", "## For Mustafa", "", v["to_mustafa"], "", "## Each suite", ""]
for s in v["suites"]:
    out += [f"### {s['suite']} ({'matches' if s['matches'] else 'DOES NOT MATCH'})", ""]
    if s.get("expected"):
        out.append(f"- **Expected:** {s['expected']}")
    out += [f"- **Came:** {s['came']}", f"- **Ruling:** {s['ruling']}", ""]
text = "\n".join(out)
assert v["carry62h"] in text and v["to_mustafa"] in text and all(x in text for x in v["next_steps"])
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {len(out)} lines, sha256 {digest}; suites {len(v['suites'])}, not matching "
      f"{[s['suite'] for s in v['suites'] if not s['matches']]}; {len(v['next_steps'])} next steps")
