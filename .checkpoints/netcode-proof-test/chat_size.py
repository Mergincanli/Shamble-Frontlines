"""Chat size now: the context tokens of this session's last main-session assistant entries (input + cache creation +
cache read), the largest such total in the transcript, and the last drops by more than half (the compacts). Read-only.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/chat_size.py"""
import json

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
rows = []
with open(T, encoding="utf-8", errors="replace") as fh:
    for n, line in enumerate(fh, 1):
        try:
            r = json.loads(line, strict=False)
        except Exception:
            continue
        if not isinstance(r, dict) or r.get("type") != "assistant" or r.get("isSidechain"):
            continue
        u = (r.get("message") or {}).get("usage") or {}
        tot = sum(int(u.get(k) or 0) for k in ("input_tokens", "cache_creation_input_tokens", "cache_read_input_tokens"))
        if tot:
            rows.append((n, r.get("timestamp"), tot))
print(f"{len(rows)} assistant entries with usage")
for n, ts, tot in rows[-3:]:
    print(f"line {n}: {ts} context {tot:,} tokens")
peak = max(rows, key=lambda x: x[2])
print(f"peak: line {peak[0]} {peak[1]} {peak[2]:,} tokens")
drops = [(a, b) for a, b in zip(rows, rows[1:]) if b[2] < a[2] * 0.5]
for a, b in drops[-5:]:
    print(f"drop: {a[1]} {a[2]:,} -> {b[1]} {b[2]:,}")
