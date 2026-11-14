# Verifier (read-only): main-session tool_use inputs in a window whose JSON mentions the advisor model or the
# wrong-telling; prints timestamp, tool name, field names and a short window of the session's own text (console only).
#   python3 -I .checkpoints/foundation-2-3-docs/v-tooltext.py <from-iso> <until-iso>
import json, re, sys
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
a, z = sys.argv[1], sys.argv[2]
pat = re.compile(r'(told you|told him|wrong thing|opus 5\.5|fable 5\.2)', re.I)
for ln, line in enumerate(open(T, encoding='utf-8'), 1):
    o = json.loads(line)
    ts = o.get('timestamp') or ''
    if o.get('type') != 'assistant' or o.get('isSidechain') or not (a <= ts < z):
        continue
    for b in (o.get('message') or {}).get('content') or []:
        if isinstance(b, dict) and b.get('type') == 'tool_use':
            inp = b.get('input') or {}
            desc = str(inp.get('description', ''))
            if pat.search(desc):
                print('DESC', ln, ts, b.get('name'), '::', desc[:300])
            s = json.dumps(inp)
            if pat.search(s) and not pat.search(desc):
                print('INPUT', ln, ts, b.get('name'), 'fields', sorted(inp.keys()), 'hits', len(pat.findall(s)))
