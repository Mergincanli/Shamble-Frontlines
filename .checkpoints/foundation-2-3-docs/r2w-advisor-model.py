# R-WORDS round 2 (read-only): which model answered each advisor tool call in this session's transcript.
# Prints only timestamps, the advisor_message model and token counts; and, from the CLI's cost-state entries,
# model names with token counts. No ids, no payloads, no cost figures. Usage: python3 -I r2w-advisor-model.py
import json
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
calls = []          # main-session advisor calls, by first sighting of the tool call
seen_call = set()
usage = {}          # message id -> (first timestamp, [(model, in, out) of advisor_message iterations])
costs = []
for line in open(T, encoding='utf-8'):
    try:
        o = json.loads(line)
    except Exception:
        continue
    if o.get('type') == 'cost-state':
        mu = o.get('modelUsage') or {}
        costs.append([(m, v.get('inputTokens'), v.get('outputTokens')) for m, v in mu.items()])
        continue
    if o.get('type') != 'assistant' or o.get('isSidechain'):
        continue
    m = o.get('message') or {}
    for c in (m.get('content') or []):
        if isinstance(c, dict) and c.get('type') == 'server_tool_use' and c.get('name') == 'advisor':
            if c.get('id') not in seen_call:
                seen_call.add(c.get('id'))
                calls.append((o.get('timestamp'), m.get('id')))
    its = (m.get('usage') or {}).get('iterations') or []
    adv = [(x.get('model'), x.get('input_tokens'), x.get('output_tokens')) for x in its if x.get('type') == 'advisor_message']
    if adv and m.get('id') not in usage:
        usage[m.get('id')] = (o.get('timestamp'), adv)
models = set()
for ts, mid in calls:
    rec = usage.get(mid)
    adv = rec[1] if rec else []
    for a in adv:
        models.add(a[0])
    print('advisor call', ts, '->', adv if adv else 'NO advisor_message usage found')
print('advisor calls:', len(calls), '| with an advisor_message record:', sum(1 for _, mid in calls if mid in usage),
      '| advisor models seen:', sorted(models))
for i, c in enumerate(costs, 1):
    print('cost-state entry', i, ':', c)
