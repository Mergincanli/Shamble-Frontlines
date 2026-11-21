"""Find when the C62H-BOUND report reached Mustafa's chat: the timestamp of the assistant message that holds
'C62H-BOUND is finished and checked. Every lost snapshot tick' in this session's transcript. Read-only; writes nothing.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/report_sent_time.py"""
import json

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
HEAD = "C62H-BOUND is finished and checked. Every lost snapshot tick"
hits = []
with open(T, encoding="utf-8", errors="replace") as fh:
    for n, line in enumerate(fh, 1):
        try:
            r = json.loads(line, strict=False)
        except Exception:
            continue
        if not isinstance(r, dict) or r.get("type") != "assistant":
            continue
        content = (r.get("message") or {}).get("content")
        if not isinstance(content, list):
            continue
        for b in content:
            if isinstance(b, dict) and b.get("type") == "text" and HEAD in b.get("text", ""):
                hits.append((n, r.get("timestamp"), r.get("uuid"), len(b["text"])))
for h in hits:
    print(f"line {h[0]}: {h[1]} uuid {h[2]} text {h[3]} chars")
print(f"{len(hits)} hit(s)")
