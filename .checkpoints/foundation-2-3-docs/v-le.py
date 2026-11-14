# Verifier (read-only): the main session's list_events / get_event calls (inputs only) with timestamps.
import json
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
for ln, line in enumerate(open(T, encoding='utf-8'), 1):
    o = json.loads(line)
    if o.get('type') != 'assistant' or o.get('isSidechain'):
        continue
    for b in (o.get('message') or {}).get('content') or []:
        if isinstance(b, dict) and b.get('type') == 'tool_use' and ('list_events' in b.get('name', '') or 'get_event' in b.get('name', '')):
            print(ln, o.get('timestamp'), b.get('name'), json.dumps(b.get('input')))
