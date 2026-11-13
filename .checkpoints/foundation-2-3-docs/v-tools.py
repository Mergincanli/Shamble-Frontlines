# Verifier (read-only): in a time window of the main transcript, count tool_use names, list system entries'
# subtypes and any assistant entry flagged as an API error; prints names, timestamps and counts only.
#   python3 -I .checkpoints/foundation-2-3-docs/v-tools.py <from-iso> <until-iso>
import json, sys, collections
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
a, z = sys.argv[1], sys.argv[2]
names = collections.Counter()
for ln, line in enumerate(open(T, encoding='utf-8'), 1):
    o = json.loads(line)
    ts = o.get('timestamp') or ''
    if not (a <= ts < z):
        continue
    t = o.get('type')
    if t == 'assistant':
        if o.get('isApiErrorMessage') or o.get('error'):
            print('API-ERROR', ln, ts, o.get('error'))
        for b in (o.get('message') or {}).get('content') or []:
            if isinstance(b, dict) and b.get('type') in ('tool_use', 'server_tool_use'):
                names[b.get('name')] += 1
    if t == 'system':
        print('SYSTEM', ln, ts, o.get('subtype'), 'level', o.get('level'))
print('tool_use names:', dict(names))
