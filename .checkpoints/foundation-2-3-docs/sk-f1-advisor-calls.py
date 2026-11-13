# Skeptic R-STATE F1: list advisor tool calls (server_tool_use named advisor) in a transcript, with timestamps.
# Prints only timestamps, uuids, entry types and tool names: never payloads.
import json, sys
path = sys.argv[1]
n = 0
names = {}
with open(path, 'r', encoding='utf-8') as f:
    for ln, line in enumerate(f, 1):
        line = line.strip()
        if not line:
            continue
        try:
            o = json.loads(line)
        except Exception:
            continue
        msg = o.get('message') or {}
        content = msg.get('content') if isinstance(msg, dict) else None
        if not isinstance(content, list):
            continue
        for c in content:
            if not isinstance(c, dict):
                continue
            t = c.get('type')
            if t in ('server_tool_use', 'advisor_tool_result') or (t == 'tool_use' and c.get('name') == 'advisor'):
                nm = c.get('name')
                names[(t, nm)] = names.get((t, nm), 0) + 1
                if nm == 'advisor' or t == 'advisor_tool_result':
                    n += 1 if t != 'advisor_tool_result' else 0
                    print(ln, o.get('timestamp'), o.get('uuid'), o.get('isSidechain'), t, nm)
            elif isinstance(t, str) and 'advisor' in t:
                names[(t, None)] = names.get((t, None), 0) + 1
                print(ln, o.get('timestamp'), o.get('uuid'), o.get('isSidechain'), t)
print('advisor server_tool_use count:', n)
print('types seen:', names)
