# Skeptic R-STATE F2: list rejected permission decisions in transcripts, with the refused
# call (cut), the refusal text (cut) and the same agent's next call (cut). Read-only; console only.
# Usage: python3 -I sk-f2-rejects.py <maxchars> <transcript.jsonl> [<transcript.jsonl> ...]
import json, sys

maxc = int(sys.argv[1])
for path in sys.argv[2:]:
    entries = []
    with open(path, encoding="utf-8") as f:
        for raw in f:
            try:
                entries.append(json.loads(raw))
            except Exception:
                pass
    calls = {}
    order = []
    for e in entries:
        msg = e.get("message") or {}
        c = msg.get("content") if isinstance(msg, dict) else None
        if isinstance(c, list):
            for x in c:
                if isinstance(x, dict) and x.get("type") == "tool_use":
                    calls[x.get("id")] = (e.get("timestamp"), x.get("name"), x.get("input"))
                    order.append(x.get("id"))
    print("#", path.split("/")[-1], "entries:", len(entries))
    for e in entries:
        pd = e.get("permissionDecision")
        if not (isinstance(pd, dict) and pd.get("decision") != "accept"):
            continue
        msg = e.get("message") or {}
        c = msg.get("content") if isinstance(msg, dict) else None
        for x in c if isinstance(c, list) else []:
            if not (isinstance(x, dict) and x.get("type") == "tool_result"):
                continue
            tid = x.get("tool_use_id")
            body = x.get("content")
            if isinstance(body, list):
                body = " ".join(y.get("text", "") for y in body if isinstance(y, dict))
            ts, name, inp = calls.get(tid, (None, None, None))
            nxt = None
            if tid in order:
                i = order.index(tid)
                if i + 1 < len(order):
                    nxt = calls[order[i + 1]]
            print("-" * 60)
            print("reject at", e.get("timestamp"), json.dumps(pd))
            print("  call:", name, json.dumps(inp)[:maxc])
            print("  text:", str(body)[:maxc].replace("\n", " | "))
            if nxt:
                print("  next:", nxt[0], nxt[1], json.dumps(nxt[2])[:maxc])
