# Skeptic R-STATE F2: print the tool calls and short results in a time window (read-only).
# Usage: python3 -I sk-f2-window.py <transcript.jsonl> <from-iso> <to-iso> [maxchars]
# Prints to the console only; writes nothing.
import json, sys

path, t0, t1 = sys.argv[1], sys.argv[2], sys.argv[3]
maxc = int(sys.argv[4]) if len(sys.argv) > 4 else 300
with open(path, encoding="utf-8") as f:
    for raw in f:
        try:
            e = json.loads(raw)
        except Exception:
            continue
        ts = e.get("timestamp", "")
        if not ts or ts < t0 or ts > t1:
            continue
        msg = e.get("message") or {}
        content = msg.get("content") if isinstance(msg, dict) else None
        if isinstance(content, str):
            print(ts, e.get("type"), "text:", content[:maxc].replace("\n", " | "))
            continue
        if not isinstance(content, list):
            continue
        for c in content:
            if not isinstance(c, dict):
                continue
            k = c.get("type")
            if k == "tool_use":
                print(ts, "TOOL_USE", c.get("name"), json.dumps(c.get("input"))[:maxc])
            elif k == "tool_result":
                body = c.get("content")
                if isinstance(body, list):
                    body = " ".join(x.get("text", "") for x in body if isinstance(x, dict))
                print(ts, "RESULT", "ERR" if c.get("is_error") else "ok", str(body)[:maxc].replace("\n", " | "))
            elif k == "text":
                print(ts, "TEXT", c.get("text", "")[:maxc].replace("\n", " | "))
