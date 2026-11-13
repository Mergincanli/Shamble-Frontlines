# Skeptic R-STATE F2: list error tool results in a transcript (read-only).
# Usage: python3 -I sk-f2-errors.py <transcript.jsonl> [maxchars] [until-iso]
# Prints: timestamp, tool name, the call's input (cut), the error text (cut).
# Prints to the console only; writes nothing.
import json, sys

path = sys.argv[1]
maxc = int(sys.argv[2]) if len(sys.argv) > 2 else 400
until = sys.argv[3] if len(sys.argv) > 3 else None

calls = {}
rows = []
n_lines = 0
n_err = 0
with open(path, encoding="utf-8") as f:
    for raw in f:
        n_lines += 1
        try:
            e = json.loads(raw)
        except Exception:
            continue
        ts = e.get("timestamp", "")
        msg = e.get("message") or {}
        content = msg.get("content") if isinstance(msg, dict) else None
        if isinstance(content, list):
            for c in content:
                if not isinstance(c, dict):
                    continue
                if c.get("type") == "tool_use":
                    calls[c.get("id")] = (c.get("name"), c.get("input"), ts)
                elif c.get("type") == "tool_result" and c.get("is_error"):
                    n_err += 1
                    body = c.get("content")
                    if isinstance(body, list):
                        body = " ".join(
                            x.get("text", "") for x in body if isinstance(x, dict)
                        )
                    rows.append((ts, c.get("tool_use_id"), str(body)))
        # hook / system entries that mention a block or a refusal
        t = e.get("type")
        if t in ("system", "attachment"):
            s = json.dumps(e)[:2000]
            low = s.lower()
            if any(k in low for k in ("block", "refus", "denied", "deny", "classifier")):
                rows.append((ts, "(entry type " + str(t) + ")", s))

print("lines:", n_lines, "error tool_results:", n_err)
for ts, tid, body in rows:
    if until and ts and ts > until:
        continue
    name, inp, cts = calls.get(tid, (None, None, None))
    print("=" * 70)
    print("ts:", ts, "| tool:", name, "| call ts:", cts)
    if inp is not None:
        print("input:", json.dumps(inp)[:maxc])
    print("error:", body[:maxc])
