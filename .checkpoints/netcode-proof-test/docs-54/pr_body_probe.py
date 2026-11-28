"""DOCS-54, read-only: find the GitHub MCP results for PRs #53 and #54 (pull_request_read, method get) in this
session's transcript, parse their JSON, and print the numbered lines of each body in a given range, with each tool
result's transcript line and tool_use id. Writes nothing. Run from the repo root:
  python3 -I .checkpoints/netcode-proof-test/docs-54/pr_body_probe.py 53 85 112
  python3 -I .checkpoints/netcode-proof-test/docs-54/pr_body_probe.py 54 18 30"""
import json
import sys

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
PR, LO, HI = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
found = []
with open(T, encoding="utf-8") as fh:
    for n, line in enumerate(fh, 1):
        if f'\\"number\\":{PR},' not in line and f'"number":{PR},' not in line:
            continue
        r = json.loads(line, strict=False)
        c = (r.get("message") or {}).get("content")
        if not isinstance(c, list):
            continue
        for b in c:
            if not (isinstance(b, dict) and b.get("type") == "tool_result"):
                continue
            cc = b.get("content")
            texts = [cc] if isinstance(cc, str) else [x.get("text") for x in cc or [] if isinstance(x, dict)]
            for t in texts:
                if isinstance(t, str) and t.startswith('{"number":%d,' % PR):
                    found.append((n, r.get("timestamp"), b.get("tool_use_id"), json.loads(t)))
print(f"PR #{PR}: {len(found)} result(s)")
for n, ts, tid, d in found[-1:]:
    body = d["body"].split("\n")
    print(f"transcript line {n} {ts} tool_use_id {tid}; merged_at {d.get('merged_at')}; body {len(body)} lines")
    for i in range(LO, min(HI, len(body)) + 1):
        print(f"{i:4d}| {body[i - 1]}")
