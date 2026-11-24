# Extracts Mustafa's messages to this chat (Builder V2, lane 2) byte for byte from the session transcript, by message
# id or by timestamp, checks each against its saved file where one exists, and writes messages.json beside this script.
import json, hashlib, sys, os
T = '/root/.claude/projects/-home-user/a7e7f150-3195-52bd-a549-73b27426198b.jsonl'
HERE = os.path.dirname(os.path.abspath(__file__))
SAVED = os.path.join(HERE, '..')
WANT = [  # (key, uuid or None, timestamp prefix or None, saved file or None)
 ('task',        None, '2026-11-15T15:33:3', 'mustafa-2026-11-15-task.md'),
 ('notes',       None, '2026-11-15T16:03:0', 'mustafa-2026-11-15-notes.md'),
 ('go',          'c7a8e93b-1b32-4886-8330-e14ada13e8a7', None, 'mustafa-2026-11-15-go.md'),
 ('yes',         None, '2026-11-16T08:46:1', 'mustafa-2026-11-16-yes.md'),
 ('rename',      '4f8140d3-1bb5-4a6a-b5a4-b7b2a9f73a86', None, None),
 ('check-yes',   'a0f3eeda-ce06-4801-ba28-f739b853c52d', None, 'mustafa-2026-11-18-check-yes.md'),
 ('token-window','8b31c945-0c40-4a8b-8b62-185f409e0a0e', None, None),
 ('token-scope', 'd9550773-1c5b-4a8b-8b62-185f409e0a0e', None, None),
 ('reviewer-b',  'cd546f27-bed3-4291-97d0-174478c77df5', None, 'mustafa-2026-11-18-reviewer-b.md'),
 ('browsers',    'aff8acbf-a617-472c-a56a-b58822623ae1', None, 'mustafa-2026-11-18-browsers.md'),
 ('lane2',       '81327c70-528e-4e03-bc98-e8a5d163fbd5', None, 'mustafa-2026-11-18-lane2.md'),
 ('answers',     '8ccb05f4-fbaa-4495-90b3-6595ea781e91', None, 'mustafa-2026-11-19-answers.md'),
 ('ads',         '88036370-e3fe-47f4-a3b4-fa570aeacd0a', None, 'mustafa-2026-11-19-ads.md'),
 ('day2',        '15af9685-0caf-45cf-a404-14f591e2e9af', None, 'mustafa-2026-11-24-day2.md'),
]
def text_of(o):
    c = o.get('message', {}).get('content')
    if isinstance(c, str): return c
    return ''.join(b.get('text', '') for b in (c or []) if isinstance(b, dict) and b.get('type') == 'text')
users = []
with open(T, encoding='utf-8') as f:
    for line in f:
        try: o = json.loads(line)
        except Exception: continue
        if o.get('type') == 'user' and not o.get('isMeta') and text_of(o).strip():
            users.append(o)
out = {}
for key, uuid, tsp, saved in WANT:
    cand = [o for o in users if (uuid and o.get('uuid', '').startswith(uuid[:8])) or (tsp and o.get('timestamp', '').startswith(tsp))]
    cand = [o for o in cand if not text_of(o).startswith('<') and 'tool_result' not in json.dumps(o.get('message', {}).get('content', ''))[:200]]
    if len(cand) != 1:
        print(key, 'candidates:', [(o.get('uuid'), o.get('timestamp'), text_of(o)[:50]) for o in cand]); continue
    o = cand[0]; t = text_of(o); b = t.encode('utf-8')
    rec = {'uuid': o.get('uuid'), 'timestamp': o.get('timestamp'), 'bytes': len(b), 'sha256': hashlib.sha256(b).hexdigest(), 'text': t}
    if saved:
        sb = open(os.path.join(SAVED, saved), 'rb').read()
        rec['saved_equal'] = (sb == b)
    out[key] = rec
    print(f"{key:13s} {rec['uuid']} {rec['timestamp']} {rec['bytes']:5d} {rec['sha256'][:12]} saved_equal={rec.get('saved_equal')}")
json.dump(out, open(os.path.join(HERE, 'messages.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
