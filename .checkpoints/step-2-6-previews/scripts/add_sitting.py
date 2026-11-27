# Adds his sitting messages of 2026-11-25, 22:28:33 and 22:56:01 UTC (TOKEN-TONIGHT (b); "fixed"; watch #72) to docs/DECISIONS.md, each byte for byte
# from the session transcript, placed by its time under "Decisions, newest first". Lane 2's lines only; no existing line
# changes (checked). Run from the repo root; --write to write.
import json, hashlib, re, sys
T = '/root/.claude/projects/-home-user/a7e7f150-3195-52bd-a549-73b27426198b.jsonl'
CHAT = ('the builder chat he calls Builder V2, lane 2 of the build, session_019YNMUSYpJGi1xxCjKgr4e5, a Claude Code '
        'chat on his personal account')
MSGS = [
    dict(u='7903e9f9-6b78-4951-a72e-c82cd6c84b6d', n=941,
         h='4591957d0ff2199472a24610e54e581e123dab1f29f23960cf52505622bba38f',
         label=None,
         head=('his sitting for step 2.6, steps 5 to 7, and TOKEN-TONIGHT (b): step 5 reported done (the environment '
               '`preview` as this chat listed it: reviewers and wait timer off, the bypass box unticked, the branch rule '
               '`main`, the test secret WALL_TEST), no secret named CLOUDFLARE_ (step 6), the Actions settings done (step '
               '7); TOKEN-TONIGHT (b): the real deploy token waits until the Guard fix chat\'s protection for main is '
               'merged, so the token steps and the second half of the environment check come in a later short sitting'),
         rec=('Records, not his words: the questions and steps he answered are this chat\'s messages of 2026-11-25 '
              'evening; the re-run of the three planted jobs at 22:29 UTC then found all three running with the test '
              'value present (step 2.6\'s report); he was asked again before the token because the protection for main '
              'was not merged (main at 07d673b0).')),
    dict(u='5ff5bcf4-7ef7-42d0-9e5e-46203589ee68', n=86,
         h='de1b683b3edcec03b5c61b30ddbb34fd27deac9f77e554fd98169cd07d29bf70',
         label=None,
         head=('"fixed": the environment page\'s bypass box saved unticked, as this chat asked after the failed check; '
               'and his yes to watching pull request #72 for CI results and comments'),
         rec=('Records, not his words: the re-run at 22:56 UTC found all three planted jobs refused before any step '
              '(each "failure" with no step, no runner and no log); this chat subscribed to #72\'s activity.')),
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
