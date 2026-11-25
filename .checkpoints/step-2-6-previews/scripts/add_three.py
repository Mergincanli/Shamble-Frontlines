# Adds his three messages of 2026-11-25 (17:05, 18:15 and 20:35 Amsterdam) to docs/DECISIONS.md, each byte for byte
# from the session transcript, placed by its time under "Decisions, newest first". Lane 2's lines only; no existing line
# changes (checked). Run from the repo root; --write to write.
import json, hashlib, re, sys
T = '/root/.claude/projects/-home-user/a7e7f150-3195-52bd-a549-73b27426198b.jsonl'
CHAT = ('the builder chat he calls Builder V2, lane 2 of the build, session_019YNMUSYpJGi1xxCjKgr4e5, a Claude Code '
        'chat on his personal account')
MSGS = [
    dict(u='0ff798eb-597c-4f56-8981-cb2afa5a5e24', n=1094,
         h='c41b21e8a1128ff144f43f44f1c6d68497d7c78b57c8c992ce04e8d34dfde869',
         label='For Builder V2, from me, written with my main Advisor (25 November, 17:05 Amsterdam).',
         head=('his order before his go on pull request #67: main taken into lane 2\'s branch again, after the guard '
               'fix (pull request #69), by one merge commit keeping both sides\' text, the other chat\'s lines byte for '
               'byte, and keeping #69\'s removal of the "in flight" rows of #66 and #68 from STATUS; then the advisor '
               'agent\'s approval of exactly that commit, green checks, and its full commit id to him; the interim rule '
               'widened until a second guard fix is merged: git and gh always start their own command, with no other '
               'program in front of them'),
         rec=('Records, not his words: 216fb21e is the commit he thanks for (his step-4 condition in every place it is '
              'reported, after main with #68 was taken in); the merge commit his order asked for is '
              '35adfbeb04d0326aa69a06356ff121e2fecc96ea, the head his go of 18:15 Amsterdam names.')),
    dict(u='42aa43e4-5bfd-4e61-ab7d-d94f48b51c03', n=1778,
         h='e2554c4e039ca10046bd77af9f1ab9cbb44cfdbdfe0f958a7f163e2ba6bb5e07',
         label='For Builder V2, from me, written with my main Advisor (25 November, 18:15 Amsterdam).',
         head=('his go to merge pull request #67 at exactly 35adfbeb04d0326aa69a06356ff121e2fecc96ea, passed as the '
               'expected head, by a normal merge commit, which is also his go on the revised header row; after the '
               'merge, the main folder fast-forwarded to main and main\'s new commit id to him; then step 2.6\'s plan, '
               'the preview links: plan first, the advisor agent\'s review, then back to him for his go, nothing built '
               'before it, the plan holding what his WORKERS-NEXT answer asks, every click of his listed once in the '
               'order he does them for one sitting, the request to the CI Builder chat, and the day of the first '
               'preview link said honestly; his 17:05 message into the decisions file and the dated pointer in the '
               'report go into step 2.6\'s first pull request'),
         rec=('Records, not his words: #67 merged as c1da1bd24faae21794195e2384895460753a41eb (2026-11-25), with '
              '35adfbe as the expected head.')),
    dict(u='fa086681-5e34-42cd-b549-d920748365c2', n=1579,
         h='3c4f0d233f522c2253a73583bea7dcb5efb543d1aaf7b97dd129f5b475f9e3d1',
         label='For Builder V2, from Mustafa (written with the main Advisor, 25 November, 20:35 Amsterdam).',
         head=('his go on step 2.6\'s plan (revision 2, at c40f3f9), with his answers: TRIGGER (a); ACCOUNT (b), a new '
               'free Cloudflare account for previews only; LANE2-LOCKFILE (b), lane 2 owns hosting/package.json and '
               'hosting/package-lock.json, for the upload tool only, outside the game\'s install, and nothing else of '
               'his words of 18 November changes; DEPARTURES (a), all nine stand, each to the drafts\' next revision; '
               'MAIN-WALL (b), the risk accepted in writing for now because the token reaches a previews-only account, '
               'the wall going to the Guard fix chat as its next change-set, and the real token his last click: if the '
               'wall is merged by then, his acceptance ends there; if not, he is asked again before that click; his go '
               'covers the three workflow texts by their sha256; the security-auditor reads preview.yml as it stands, '
               'with the token step added after its review, in round 1; a text a review changes comes back to him with '
               'its sha256 before his sitting; the request to the CI Builder chat as planned; the wall\'s detail kept '
               'out of every further pushed file'),
         rec=('Records, not his words: TRIGGER, ACCOUNT, LANE2-LOCKFILE, DEPARTURES and MAIN-WALL are the five decisions '
              'of step 2.6\'s plan, revision 2, section 12, carried in its report; the request to the CI Builder chat is '
              'comment 7172595904 on issue #59.')),
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
    assert t.split('\n', 1)[0] == m['label'], (m['u'], t.split('\n', 1)[0])
    src = f'his label: "{m["label"].rstrip(".")}"; message {m["u"]}'
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
