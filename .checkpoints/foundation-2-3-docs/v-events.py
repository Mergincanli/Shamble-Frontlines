# Verifier (read-only): parse a saved list_events page (JSON inside the harness's envelope); list assistant events'
# created_at and their text blocks (the session's own text, console only), flag texts naming the advisor model or
# the wrong-telling; print paging cursors. Never prints user events' text.
#   python3 -I .checkpoints/foundation-2-3-docs/v-events.py <page.txt> [from-iso]
import json, re, sys
raw = open(sys.argv[1], encoding='utf-8').read()
since = sys.argv[2] if len(sys.argv) > 2 else ''
a = raw.find('{')
b = raw.rfind('}')
body = json.loads(raw[a:b+1])
ccr = body.get('ccr', body)
data = ccr.get('data', [])
print('events', len(data), 'first_id', ccr.get('first_id'), 'last_id', ccr.get('last_id'), 'has_more', ccr.get('has_more'))
ts = [e.get('created_at') for e in data]
print('created_at range', min(ts) if ts else None, max(ts) if ts else None)
pat = re.compile(r'(fable|opus|wrong thing|told you|advisor model)', re.I)
for e in data:
    ca = e.get('created_at') or ''
    if ca < since:
        continue
    asst = e.get('assistant')
    if not asst:
        continue
    msg = (asst.get('internal_anthropic_catchall') or {}).get('message') or asst.get('message') or {}
    for c in msg.get('content') or []:
        if isinstance(c, dict) and c.get('type') == 'text':
            t = c.get('text', '')
            flag = bool(pat.search(t))
            print(f'TEXT {ca} len {len(t)} flag {flag} :: {t[:200]!r}')
