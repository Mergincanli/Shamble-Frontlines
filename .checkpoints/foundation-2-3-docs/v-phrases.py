# Verifier (read-only): where do given phrases occur in the main transcript? Prints line, timestamp, entry type,
# block type and tool name only (no text).
#   python3 -I .checkpoints/foundation-2-3-docs/v-phrases.py "<phrase>" ["<phrase>" ...]
import json, sys
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
phr = sys.argv[1:]
for ln, line in enumerate(open(T, encoding='utf-8'), 1):
    if not any(p in line for p in phr):
        continue
    o = json.loads(line)
    c = (o.get('message') or {}).get('content')
    where = []
    if isinstance(c, list):
        for b in c:
            if isinstance(b, dict):
                s = json.dumps(b)
                for p in phr:
                    if p in s or p in str(b.get('text', '')):
                        where.append((p[:24], b.get('type'), b.get('name')))
    elif isinstance(c, str):
        for p in phr:
            if p in c:
                where.append((p[:24], 'string-content', None))
    else:
        where.append(('?', 'attachment/other', (o.get('attachment') or {}).get('type')))
    print(ln, o.get('timestamp'), o.get('type'), 'sidechain', bool(o.get('isSidechain')), where)
