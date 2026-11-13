# Round 2's fix pass: the session's chat messages (assistant text blocks) that name the advisor model or correct where
# the wrong claim stood, read from saved sessions-API pages (list_events, kinds assistant). Prints times and openings.
#   python3 -I .checkpoints/foundation-2-3-docs/fp2-tellings.py <page.txt> [<page.txt> ...]
import json, re, sys
pat = re.compile(r'Correction first|A correction to my earlier update|opening (line|message)|wrong thing at the start')
for p in sys.argv[1:]:
    raw = open(p, encoding='utf-8').read()
    d = json.loads(raw[raw.find('{'):raw.rfind('}') + 1])
    evs = d['ccr']['data']
    print(p.split('-')[-1], len(evs), 'events', evs[0]['created_at'], 'to', evs[-1]['created_at'])
    for e in evs:
        msg = ((e.get('assistant') or {}).get('internal_anthropic_catchall') or {}).get('message') or {}
        for c in msg.get('content') or []:
            if isinstance(c, dict) and c.get('type') == 'text' and pat.search(c.get('text', '')):
                print(' ', e['created_at'], '|', c['text'][:230].replace('\n', ' '))
