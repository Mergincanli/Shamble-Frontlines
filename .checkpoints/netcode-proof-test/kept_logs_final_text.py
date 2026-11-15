"""Write the KEPT-LOGS final text for Mustafa (the advisor's final_text_for_mustafa, workflow wf_0f69bce5-688) word for
word into kept-logs-final-text.md, after checking it equals the advisor's "BEGIN final-message" block in its notes.
Nothing is added: the file is the text alone. Run from the repo root:
python3 -I .checkpoints/netcode-proof-test/kept_logs_final_text.py"""
import hashlib
import json
import re

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_0f69bce5-688/journal.jsonl")
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
ft = res[0]["final_text_for_mustafa"].strip("\n") + "\n"
with open(f"{N}/advisor-kept-logs-rule.md", encoding="utf-8") as fh:
    m = re.search(r"<!-- BEGIN final-message -->\n(.*?)\n?<!-- END final-message -->", fh.read(), re.S)
assert m and m.group(1).strip("\n") + "\n" == ft, "the result and the notes' final-message block differ"
out = f"{N}/kept-logs-final-text.md"
with open(out, "w", encoding="utf-8") as fh:
    fh.write(ft)
with open(out, "rb") as fh:
    print(f"{out}: {ft.count(chr(10))} lines, sha256 {hashlib.sha256(fh.read()).hexdigest()}")
