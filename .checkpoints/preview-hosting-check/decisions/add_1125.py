# Adds his message of 2026-11-25 (queued entry 0c2a5b97..., stamped 2026-11-24 23:16:11 UTC) to docs/DECISIONS.md, by
# the time in its header, from the chat's byte-for-byte copy (checked against its length and sha256). Lane 2's lines only.
import hashlib, re, sys
a = open('.checkpoints/preview-hosting-check/mustafa-2026-11-25-lane2-build.md', encoding='utf-8').read()
t = a[a.index('-----BEGIN HIS TEXT-----\n') + 25:a.index('\n-----END HIS TEXT-----')]
b = t.encode('utf-8'); h = hashlib.sha256(b).hexdigest()
assert len(b) == 1359 and h.startswith('332b0f9e4f1a'), (len(b), h)
CHAT = 'the builder chat he calls Builder V2, lane 2 of the build, session_019YNMUSYpJGi1xxCjKgr4e5, a Claude Code chat on his personal account'
head = ("lane 2 builds the game now, beside lane 1 (Builder V3, on his third account: change-set 3a merged as #65, 3b next; "
        "lane 1's CI edits now go to a separate chat, CI Builder); lane 2 reads Part 15 and STATUS again and answers in at "
        "most 12 lines what step 2.6 still needs and when his first link comes, which of its units can start now, and what "
        "of change-set 6 and M it could build before change-set 5 without lane 1's folders (his decision; nothing of it "
        "built before his yes); read and answer only; one pull request at a time per lane, every file one owner, the kit frozen")
src = ('his label: "For Builder V2 (lane 2), from me, written with my main Advisor (25 November)"; a message queued while '
       'the chat worked, entry 0c2a5b97-5c59-4e91-87ec-b3e315f2eee9')
ans = ("What it answers, in the chat's words (not his): nothing the chat asked. Its line \"The Cloudflare sitting is still to "
       "come, later today\" came after Day 2 had run (2026-11-24, from about 20:14 UTC); the chat said so and answered in "
       "its chat, changing no file for it.")
body = (f"**Taken by Mustafa, 2026-11-24, 23:16:11 UTC: {head}** ({CHAT}; {src}; whole; extracted by script from the "
        f"chat's transcript). {ans} Copied by script ({len(b)} bytes, sha256 {h}):")
out = []; cur = '- '
for w in body.split(' '):
    if len(cur) + len(w) + (0 if cur in ('- ', '  ') else 1) > 120:
        out.append(cur); cur = '  ' + w
    else:
        cur = cur + w if cur in ('- ', '  ') else cur + ' ' + w
out.append(cur)
for line in t.split('\n'):
    out.append('> ' + line if line else '>')
entry = '\n'.join(out)
P = 'docs/DECISIONS.md'
doc = open(P, encoding='utf-8').read()
start = doc.index('## Decisions, newest first\n') + len('## Decisions, newest first\n')
end = doc.index('\n## Also recorded elsewhere')
lines = doc[start:end].split('\n')
TS = re.compile(r'(20\d\d-\d\d-\d\d),? (\d\d:\d\d(?::\d\d)?)')
A = re.compile(r'^- (\*\*(Replaced.*?)?Taken by Mustafa, |_A pointer, not a decision: \*\*)')
def ets(i):
    m = TS.search(' '.join(lines[i:i+3]))
    if not m:
        d = re.search(r'20\d\d-\d\d-\d\d', ' '.join(lines[i:i+3])); return d.group(0) + 'T00:00:00'
    tt = m.group(2); tt = tt + ':00' if len(tt) == 5 else tt
    return m.group(1) + 'T' + tt
tops = [i for i, l in enumerate(lines) if A.match(l)]
me = '2026-11-24T23:16:11'
pos = next(i for i in tops if ets(i) < me)
above = max([i for i in tops if i < pos], default=None)
print('insert before:', ets(pos), '| above it:', ets(above) if above is not None else '(top)')
lines[pos:pos] = entry.split('\n')
doc2 = doc[:start] + '\n'.join(lines) + doc[end:]
old = doc.split('\n'); new = doc2.split('\n'); j = 0
for l in new:
    if j < len(old) and l == old[j]: j += 1
assert j == len(old)
assert '\n'.join(('> ' + l if l else '>') for l in t.split('\n')) in doc2
if '--write' in sys.argv: open(P, 'w', encoding='utf-8').write(doc2)
print('lines added:', len(new) - len(old), '(written)' if '--write' in sys.argv else '(dry run)')
