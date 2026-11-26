# Saves Mustafa's message of 2026-11-26, 15:05 Amsterdam (GUARD-PROMPT (a), MAIN-WALL's timing) byte for byte from this
# chat's transcript into the notes, with its uuid, time, size and sha256, for the DECISIONS entry at the next docs head.
# Usage: python3 -I save_1505.py <transcript.jsonl> <out.md> [--write]. Runs no git; writes only <out.md>.
import json, hashlib, sys
T, OUT = sys.argv[1], sys.argv[2]
LEAD = 'From Mustafa, 26 November, 15:05 Amsterdam (written with my Advisor).'
hits = []
for line in open(T, encoding='utf-8'):
    if LEAD not in line:
        continue
    o = json.loads(line)
    if o.get('type') != 'user' or o.get('isMeta') or o.get('isSidechain'):
        continue
    c = o['message']['content']
    blocks = [c] if isinstance(c, str) else [b.get('text', '') for b in c if isinstance(b, dict) and b.get('type') == 'text']
    mine = [b for b in blocks if b.startswith(LEAD)]
    if mine:
        hits.append((o['uuid'], o['timestamp'], mine, len(blocks)))
assert len(hits) == 1, [(h[0], h[1]) for h in hits]
u, ts, mine, nblocks = hits[0]
assert len(mine) == 1, len(mine)
t = mine[0]
b = t.encode('utf-8')
h = hashlib.sha256(b).hexdigest()
print(f'uuid {u}, {ts}, {len(b)} bytes, sha256 {h}; text blocks in the message: {nblocks}')
doc = (f'# Mustafa, 2026-11-26, 15:05 Amsterdam: GUARD-PROMPT (a), Builder V2 told, MAIN-WALL\'s timing\n'
       f'Saved by script (scripts/save_1505.py) from this chat\'s transcript (session_019YNMUUMZBxnFZJY9gp6qxy): message\n'
       f'{u}, {ts[:19].replace("T", " ")} UTC, whole, {len(b)} bytes, sha256 {h}. It answers GUARD-PROMPT in\n'
       f'questions-2026-11-26-new-chat.md (commit e200d51). Goes into docs/DECISIONS.md, word for word, at the next docs head.\n\n'
       + '\n'.join(('> ' + l if l else '>') for l in t.split('\n')) + '\n')
assert ('\n'.join(('> ' + l if l else '>') for l in t.split('\n'))) in doc
if '--write' in sys.argv:
    open(OUT, 'w', encoding='utf-8', newline='\n').write(doc)
print('written' if '--write' in sys.argv else 'dry run')
