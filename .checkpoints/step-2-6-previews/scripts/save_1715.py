# Saves Mustafa's message of 2026-11-26 (after about 17:15 UTC: MAIN-WALL on; work that stores no credential resumes;
# the token sitting after one day-session merge and one PR Advisor merge) byte for byte from this chat's transcript into
# the notes, with its uuid, time, size and sha256, for the DECISIONS entry at the next docs head.
# Usage: python3 -I save_1715.py <transcript.jsonl> <out.md> [--write]. Runs no git; writes only <out.md>.
import json, hashlib, sys
T, OUT = sys.argv[1], sys.argv[2]
LEAD = "From Mustafa: main's new rule (MAIN-WALL) is on since 26 November, about 17:15 UTC"
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
doc = (f'# Mustafa, 2026-11-26: MAIN-WALL on; work that stores no credential resumes; when the token sitting comes\n'
       f'Saved by script (scripts/save_1715.py) from this chat\'s transcript (session_019YNMUUMZBxnFZJY9gp6qxy): message\n'
       f'{u}, {ts[:19].replace("T", " ")} UTC, whole, {len(b)} bytes, sha256 {h}. Goes into docs/DECISIONS.md, word for\n'
       f'word, at the next docs head.\n\n'
       + '\n'.join(('> ' + l if l else '>') for l in t.split('\n')) + '\n')
if '--write' in sys.argv:
    open(OUT, 'w', encoding='utf-8', newline='\n').write(doc)
print('written' if '--write' in sys.argv else 'dry run')
