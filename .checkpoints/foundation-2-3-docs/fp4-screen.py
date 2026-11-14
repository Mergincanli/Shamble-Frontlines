# Step 8: a keyword screen over every recorded main-session chat text from 2026-11-13 22:40 UTC on (the transcript),
# for messages that may bear on how Mustafa was told about the advisor model. Prints line, time and the matched keys.
#   python3 -I .checkpoints/foundation-2-3-docs/fp4-screen.py
import json, re
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
KEYS = [('told you', re.compile(r'told you', re.I)), ('advisor..model', re.compile(r'advisor\W+(?:\w+\W+){0,6}model', re.I)),
        ('Opus 5.5', re.compile(r'Opus 5\.5')), ('Fable', re.compile(r'Fable'))]
n = 0; total = 0
for line in open(T, encoding='utf-8'):
    n += 1
    o = json.loads(line)
    if o.get('type') != 'assistant' or o.get('isSidechain') or o.get('timestamp', '') < '2026-11-13T22:40': continue
    for c in (o.get('message') or {}).get('content') or []:
        if isinstance(c, dict) and c.get('type') == 'text':
            total += 1
            hit = [k for k, r in KEYS if r.search(c['text'])]
            if hit: print(f'line {n} {o["timestamp"]}: {", ".join(hit)}')
print(f'{total} chat texts screened')
