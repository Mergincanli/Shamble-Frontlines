"""Read-only probe for MERGE-55's record: print Mustafa's message (uuid 88423842-...) as stored, and every line that
names "merge #55" in the main chain's assistant messages before it (text blocks and thinking blocks, labelled), with
their timestamps. Writes nothing. Run from the repo root:
  python3 -I .checkpoints/netcode-proof-test/docs-54/merge55_probe.py"""
import json
import re

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
UUID = "88423842-2b09-4986-bab4-20580f7d5c91"

rows = []
with open(T, encoding="utf-8", errors="replace") as fh:
    for n, line in enumerate(fh, 1):
        try:
            r = json.loads(line, strict=False)
        except Exception:
            continue
        if isinstance(r, dict):
            rows.append((n, r))
idx = [i for i, (_, r) in enumerate(rows) if r.get("uuid") == UUID]
print("matches for the uuid:", idx)
n, u = rows[idx[0]]
print("line", n, "type", u.get("type"), "timestamp", u.get("timestamp"), "sidechain", u.get("isSidechain"))
print("content:", repr((u.get("message") or {}).get("content"))[:400])
pat = re.compile(r"merge #55", re.I)
for n, r in rows[: idx[0]]:
    if r.get("type") != "assistant" or r.get("isSidechain"):
        continue
    c = (r.get("message") or {}).get("content")
    if not isinstance(c, list):
        continue
    for b in c:
        if not isinstance(b, dict):
            continue
        kind = b.get("type")
        body = b.get("text") if kind == "text" else b.get("thinking") if kind == "thinking" else None
        if not body:
            continue
        for ln in body.splitlines():
            if pat.search(ln):
                print(f"[line {n} {r.get('timestamp')} {kind}] {ln.strip()[:300]}")
