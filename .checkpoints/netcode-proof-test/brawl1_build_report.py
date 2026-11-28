"""Write the BRAWL-1 builder's final report (copy 1, agent a4be43b1f756e45dd) word for word from its transcript into
brawl1-build-report.md: the text of its last SubagentHandback call, as the agent returned it. Only the heading and the
source line are the script's. The file's name ends in -report.md, so review_packet.py refuses it in any packet.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_build_report.py"""
import hashlib
import json

SRC = ("/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/tasks/"
       "a4be43b1f756e45dd.output")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-build-report.md"
with open(SRC, "rb") as fh:
    raw = fh.read()
rows = []
for line in raw.decode("utf-8").split("\n"):
    if line.strip():
        try:
            rows.append(json.loads(line))
        except json.JSONDecodeError:
            pass
calls = []
for r in rows:
    content = (r.get("message") or {}).get("content")
    if isinstance(content, list):
        for b in content:
            if isinstance(b, dict) and b.get("type") == "tool_use" and b.get("name") == "SubagentHandback":
                calls.append(b["input"])
assert calls, "no SubagentHandback call in the transcript"
text = calls[-1]["message"]
assert isinstance(text, str) and text.strip()
head = ["# BRAWL-1 builder, copy 1: its final report, word for word", "",
        f"Written by script from the agent's transcript ({SRC}, sha256 {hashlib.sha256(raw).hexdigest()}): the text of "
        f"its last SubagentHandback call ({len(calls)} in the transcript). Nothing below is the main session's.", "",
        "---", ""]
out = "\n".join(head) + text + ("" if text.endswith("\n") else "\n")
assert text in out
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(out)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {out.count(chr(10))} lines, sha256 {digest}; report {len(text)} chars")
