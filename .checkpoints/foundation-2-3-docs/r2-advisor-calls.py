# R-STATE round 2: list every advisor-tool call in the session transcript (assistant entries holding a
# server_tool_use or tool_use named "advisor"), with the entry's timestamp and uuid. Reads only.
#   python3 -I .checkpoints/foundation-2-3-docs/r2-advisor-calls.py <transcript.jsonl>
import json, sys

path = sys.argv[1]
n = 0
for line in open(path, encoding='utf-8'):
    try:
        o = json.loads(line)
    except Exception:
        continue
    if o.get('type') != 'assistant':
        continue
    m = o.get('message') or {}
    c = m.get('content')
    if not isinstance(c, list):
        continue
    for b in c:
        if isinstance(b, dict) and b.get('type') in ('server_tool_use', 'tool_use') and b.get('name') == 'advisor':
            n += 1
            print(n, o.get('timestamp'), o.get('uuid'), b.get('type'), b.get('id'))
print('total', n)
