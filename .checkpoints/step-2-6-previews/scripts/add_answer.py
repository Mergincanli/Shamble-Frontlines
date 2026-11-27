# Adds his answer of 2026-11-25, 20:51:45 UTC (NEW-TEXT and PREVIEW-MISSING-BUILD-COMMENT) to docs/DECISIONS.md, each byte for byte
# from the session transcript, placed by its time under "Decisions, newest first". Lane 2's lines only; no existing line
# changes (checked). Run from the repo root; --write to write.
import json, hashlib, re, sys
T = '/root/.claude/projects/-home-user/a7e7f150-3195-52bd-a549-73b27426198b.jsonl'
CHAT = ('the builder chat he calls Builder V2, lane 2 of the build, session_019YNMUSYpJGi1xxCjKgr4e5, a Claude Code '
        'chat on his personal account')
MSGS = [
    dict(u='33a78af8-838a-4174-bfd9-23d268665740', n=28,
         h='6fbf35bfeb7ed6bddc4fdd69a05d1221372997eec8dbc83b76fca24733dbdaac',
         label=None,
         head=('his answers to the two questions from step 2.6\'s build: NEW-TEXT (a), line 50 of the approved '
               'preview.yml changes from `run: node --test hosting/` to `run: node --test hosting/preview.test.mjs`, the '
               'new text 10,577 bytes, sha256 31119c06e7d095baa3c99ed54bbc1a2c72d05b8b3cab88ffd4512dc12acd3abc, '
               'preview-close.yml and wall-test.yml unchanged; PREVIEW-MISSING-BUILD-COMMENT (a), keep the texts: the '
               'comment covers the upload\'s result and the header check, and anything that stops the build check (a '
               'missing build once CI hands one over, a refused build, a failed read of GitHub) shows only as a red run '
               'under main in the Actions tab, as the runbook says'),
         rec=('Records, not his words: the questions he answered are this chat\'s message of 2026-11-25 and its file '
              '.checkpoints/step-2-6-previews/questions-2026-11-25-build.md at e28b80891a01329f911e1b2b0557f444871e665b, '
              'carried into step 2.6\'s report; the advisor agent approved NEW-TEXT and recommended (a) on the other, '
              'and the security-auditor read the new text with no change and no findings.')),
]

def text_of(u):
    for line in open(T, encoding='utf-8'):
        if u not in line: continue
        o = json.loads(line)
        if o.get('uuid') == u and o.get('type') == 'user':
            c = o['message']['content']
            t = c if isinstance(c, str) else ''.join(b.get('text', '') for b in c if isinstance(b, dict) and b.get('type') == 'text')
            return t, o['timestamp']
    raise SystemExit(f'message {u} not found')

def wrap(body):
    out = []; cur = '- '
    for w in body.split(' '):
        if len(cur) + len(w) + (0 if cur in ('- ', '  ') else 1) > 120:
            out.append(cur); cur = '  ' + w
        else:
            cur = cur + w if cur in ('- ', '  ') else cur + ' ' + w
    out.append(cur)
    return out

P = 'docs/DECISIONS.md'
doc = open(P, encoding='utf-8').read()
orig = doc
TS = re.compile(r'(20\d\d-\d\d-\d\d),? (\d\d:\d\d(?::\d\d)?)')
A = re.compile(r'^- (\*\*(Replaced.*?)?Taken by Mustafa, |_A pointer, not a decision: \*\*)')
for m in MSGS:
    t, ts = text_of(m['u'])
    b = t.encode('utf-8'); h = hashlib.sha256(b).hexdigest()
    assert len(b) == m['n'] and h == m['h'], (m['u'], len(b), h)
    if m['label']: assert t.split('\n', 1)[0] == m['label'], (m['u'], t.split('\n', 1)[0])
    src = (f'his label: "{m["label"].rstrip(".")}"; ' if m['label'] else 'no label; ') + f'message {m["u"]}'
    body = (f"**Taken by Mustafa, {ts[:10]}, {ts[11:19]} UTC: {m['head']}** ({CHAT}; {src}, whole; extracted by "
            f"script from the chat's transcript). {m['rec']} Copied by script ({len(b)} bytes, sha256 {h}):")
    out = wrap(body) + [('> ' + l if l else '>') for l in t.split('\n')]
    start = doc.index('## Decisions, newest first\n') + len('## Decisions, newest first\n')
    end = doc.index('\n## Also recorded elsewhere')
    lines = doc[start:end].split('\n')
    def ets(i):
        mm = TS.search(' '.join(lines[i:i+3]))
        if not mm:
            d = re.search(r'20\d\d-\d\d-\d\d', ' '.join(lines[i:i+3])); return d.group(0) + 'T00:00:00'
        x = mm.group(2); x = x + ':00' if len(x) == 5 else x
        return mm.group(1) + 'T' + x
    tops = [i for i, l in enumerate(lines) if A.match(l)]
    pos = next(i for i in tops if ets(i) < ts[:19])
    print(m['u'][:8], ts[:19], '-> before an entry of', ets(pos))
    lines[pos:pos] = out
    doc = doc[:start] + '\n'.join(lines) + doc[end:]
    assert '\n'.join(('> ' + l if l else '>') for l in t.split('\n')) in doc
# Every original line kept, in order.
o_ = orig.split('\n'); n_ = doc.split('\n'); j = 0
for l in n_:
    if j < len(o_) and l == o_[j]: j += 1
assert j == len(o_), 'an original line changed'
if '--write' in sys.argv: open(P, 'w', encoding='utf-8').write(doc)
print('lines added:', len(n_) - len(o_), '(written)' if '--write' in sys.argv else '(dry run)')
