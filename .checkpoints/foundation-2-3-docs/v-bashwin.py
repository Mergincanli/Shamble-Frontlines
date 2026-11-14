# Verifier (read-only): main-session tool calls in a window: name, description, and for results a length and a short
# window around words about chat updates (the session's own outputs; console only).
#   python3 -I .checkpoints/foundation-2-3-docs/v-bashwin.py <from-iso> <until-iso>
import json, re, sys
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
a, z = sys.argv[1], sys.argv[2]
pat = re.compile(r'(told you|wrong thing|compaction|context reset|update)', re.I)
for ln, line in enumerate(open(T, encoding='utf-8'), 1):
    o = json.loads(line)
    ts = o.get('timestamp') or ''
    if o.get('isSidechain') or not (a <= ts < z):
        continue
    c = (o.get('message') or {}).get('content')
    if not isinstance(c, list):
        continue
    for b in c:
        if not isinstance(b, dict):
            continue
        if b.get('type') == 'tool_use':
            print('USE', ln, ts, b.get('name'), '::', str((b.get('input') or {}).get('description', ''))[:160])
        if b.get('type') == 'tool_result':
            cc = b.get('content')
            s = cc if isinstance(cc, str) else ' '.join(x.get('text', '') for x in cc if isinstance(x, dict)) if isinstance(cc, list) else ''
            hits = [m.start() for m in pat.finditer(s)]
            print('RES', ln, ts, 'len', len(s), 'hits', len(hits))
            for h in hits[:3]:
                print('      ...', s[max(0, h-120):h+160].replace('\n', ' '))
