import json, sys
from datetime import datetime
# Per run: seconds from the first to the last timestamp in its transcript; the largest input+cache tokens of one turn.
T = sys.argv[1]
for l in open(sys.argv[2]):
    if not l.strip(): continue
    name, aid = l.split()[:2]
    ts = []; mx = 0
    for line in open(T + '/agent-' + aid + '.jsonl', encoding='utf-8'):
        try: ev = json.loads(line)
        except Exception: continue
        t = ev.get('timestamp')
        if t: ts.append(datetime.fromisoformat(t.replace('Z', '+00:00')))
        u = (ev.get('message') or {}).get('usage') or {}
        s = sum(u.get(k, 0) or 0 for k in ('input_tokens', 'cache_read_input_tokens', 'cache_creation_input_tokens', 'output_tokens'))
        mx = max(mx, s)
    print(name, round((max(ts) - min(ts)).total_seconds(), 1), 's; max turn tokens', mx)
