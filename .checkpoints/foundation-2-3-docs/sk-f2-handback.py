# Skeptic R-STATE F2: in the main transcript, find tool results for given tool_use ids
# and report whether their text mentions a refusal (read-only; console only).
# Usage: python3 -I sk-f2-handback.py <main.jsonl> <tool_use_id> [<tool_use_id> ...]
import json, re, sys

path = sys.argv[1]
ids = set(sys.argv[2:])
pat = re.compile(r"refus|hook error|guard|denied|deny|classifier|redirect", re.I)
with open(path, encoding="utf-8") as f:
    for raw in f:
        try:
            e = json.loads(raw)
        except Exception:
            continue
        msg = e.get("message") or {}
        content = msg.get("content") if isinstance(msg, dict) else None
        if not isinstance(content, list):
            continue
        for c in content:
            if not isinstance(c, dict) or c.get("type") != "tool_result":
                continue
            if c.get("tool_use_id") not in ids:
                continue
            body = c.get("content")
            if isinstance(body, list):
                body = " ".join(x.get("text", "") for x in body if isinstance(x, dict))
            body = str(body)
            print("=" * 70)
            print("ts:", e.get("timestamp"), "id:", c.get("tool_use_id"), "len:", len(body))
            hits = [(m.start(), body[max(0, m.start() - 120): m.start() + 160].replace("\n", " | ")) for m in pat.finditer(body)]
            print("hits:", len(hits))
            for pos, ctx in hits[:12]:
                print("  @", pos, ":", ctx)
