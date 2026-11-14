# Verifier (read-only): for given transcript line numbers holding tool_use blocks, print the matching tool_result's
# timestamp, length and any saved-file path it names (no payload text).
#   python3 -I .checkpoints/foundation-2-3-docs/v-results.py <line> [<line> ...]
import json, re, sys
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
want = set(int(x) for x in sys.argv[1:])
lines = open(T, encoding='utf-8').read().split('\n')
ids = {}
for n in want:
    o = json.loads(lines[n-1])
    for b in (o.get('message') or {}).get('content') or []:
        if isinstance(b, dict) and b.get('type') == 'tool_use':
            ids[b['id']] = n
for ln, line in enumerate(lines, 1):
    if not line:
        continue
    o = json.loads(line)
    c = (o.get('message') or {}).get('content')
    if not isinstance(c, list):
        continue
    for b in c:
        if isinstance(b, dict) and b.get('type') == 'tool_result' and b.get('tool_use_id') in ids:
            cc = b.get('content')
            s = cc if isinstance(cc, str) else json.dumps(cc)
            paths = re.findall(r'/root/\.claude/[^\s"\\]+\.txt', s)
            print('use line', ids[b['tool_use_id']], '-> result line', ln, o.get('timestamp'), 'len', len(s), 'is_error', b.get('is_error'), 'paths', paths[:2])
