# R-WORDS scratch (read-only): an independent byte check of his words in PR #52 at the reviewed head, not through
# words.mjs. Sources: the session transcript (A1, A2 by uuid), merge commit 4d21c1a's message (A3), all read here.
# Targets: docs/DECISIONS.md and the new report at the head (git show), the pointers in SPIKE_PLAN, ADR 0002,
# DECISIONS, ROADMAP and STATUS. Prints booleans and lengths only, never the words.
#   python3 -I .checkpoints/foundation-2-3-docs/rw-bytes.py <sha>
import json, re, subprocess, sys

SHA = sys.argv[1]
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
def git(*a): return subprocess.run(['git', *a], capture_output=True, text=True, check=True).stdout
def show(p): return git('show', f'{SHA}:{p}')

msgs = {}
for line in open(T, encoding='utf-8'):
    try: o = json.loads(line)
    except Exception: continue
    if o.get('type') == 'user' and o.get('uuid') in ('b031379c-ea4c-47db-b279-54e937bf147a', 'e5fa7ca0-6494-44d0-8366-81fc612c7958'):
        msgs.setdefault(o['uuid'], []).append(o['message']['content'])
assert all(len(v) == 1 and isinstance(v[0], str) for v in msgs.values()) and len(msgs) == 2
A1 = msgs['b031379c-ea4c-47db-b279-54e937bf147a'][0]
A2 = msgs['e5fa7ca0-6494-44d0-8366-81fc612c7958'][0]
cm = git('show', '-s', '--format=%B', '4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300').split('\n')
i = [k for k, l in enumerate(cm) if 'quoted whole' in l]; assert len(i) == 1; i = i[0] + 1
while cm[i] == '': i += 1
blk = []
while i < len(cm) and (cm[i] == '>' or cm[i].startswith('> ')): blk.append(cm[i]); i += 1
A3 = '\n'.join('' if l == '>' else l[2:] for l in blk)
dl = [l for l in A2.split('\n') if len(l) > 2 and l.startswith('"') and l.endswith('"')]
print('A2 lines that start and end with a quote mark:', len(dl))
D = dl[0][1:-1]
S = re.split(r'(?<=\.) ', D)
FOUR = ' '.join(S[:4])
print('lengths: A1', len(A1), 'A2', len(A2), 'A3', len(A3), 'decision', len(D), 'sentences', len(S), 'first four', len(FOUR))
kl = [l for l in A2.split('\n') if 'KIT-FREEZE-2' in l]; print('A2 lines naming KIT-FREEZE-2:', len(kl))
KIT = kl[0]

dec = show('docs/DECISIONS.md').split('\n')
def unq(lines, ind):
    return '\n'.join('' if l == ind + '>' else l[len(ind) + 2:] for l in lines)
def take(start, ind):
    out = []; k = start
    while k < len(dec) and (dec[k] == ind + '>' or dec[k].startswith(ind + '> ')): out.append(dec[k]); k += 1
    return out, k
def after(prefix, label=None):
    h = [k for k, l in enumerate(dec) if l.startswith(prefix)]; assert len(h) == 1, prefix
    k = h[0] + 1
    while not (dec[k].startswith('>') or dec[k].startswith('  - His')): k += 1
    return k
k = after('- **Taken by Mustafa, 2026-11-13, 18:47:55 UTC')
assert dec[k].startswith('  - His decision'); q1, k = take(k + 1, '    ')
assert dec[k].startswith('  - His whole message'); q2, k = take(k + 1, '    ')
print('DECISIONS E-TASK decision quote == decision cut here:', unq(q1, '    ') == D)
print('DECISIONS E-TASK whole message == A2 (transcript):', unq(q2, '    ') == A2)
k = after('- **Taken by Mustafa, 2026-11-13, 18:29:55 UTC'); q3, _ = take(k, '')
print('DECISIONS E-HI quote == A1 (transcript):', unq(q3, '') == A1)
k = after('- **Taken by Mustafa, 2026-11-13, 16:16:47 UTC'); q4, _ = take(k, '')
print('DECISIONS E-48 quote == A3 (merge commit):', unq(q4, '') == A3)

rep = show('docs/reports/2026-11-13-foundation-2-3-docs.md')
app = rep[rep.index('## Appendix A.'):]
fences = dict((m.group(1), m.group(3)) for m in re.finditer(r'^### (A\d)\. .*\n\n(`{3,})text\n([\s\S]*?)\n\2\n', app, re.M))
print('appendix blocks:', sorted(fences), '; A1 ==', fences.get('A1') == A1, '; A2 ==', fences.get('A2') == A2, '; A3 ==', fences.get('A3') == A3)
body = rep[:rep.index('## Appendix A.')]
bq = re.findall(r'"([^"\n]+)" \(A2\)', body)
print('report body quotes cited (A2):', len(bq), '; all inside A2:', all(x in A2 for x in bq))

def joined(text): return re.sub(r'\s+', ' ', '\n'.join(re.sub(r'^[\s>]*', '', l) for l in text.split('\n')))
for p in ['docs/SPIKE_PLAN.md', 'docs/adr/0002-code-structure-build-toolchain.md', 'docs/DECISIONS.md']:
    lines = show(p).split('\n'); n = 0
    for a, l in enumerate(lines):
        if l.lstrip().startswith('_Pointer (2026-11-13'):
            b = a; t = l.strip()
            while not t.endswith('"_'): b += 1; t += ' ' + lines[b].strip()
            q = t[t.rindex(': "') + 3:-2]; n += 1
            print(f'{p}:{a + 1} pointer quote == first four sentences: {q == FOUR}; says "for foundation step 2.3 only": {"for foundation step 2.3 only" in t}')
    print(p, 'pointers of 2026-11-13:', n)
rm = joined(show('docs/ROADMAP.md'))
j = rm.index('pointer, 2026-11-13:'); seg = rm[j:rm.index('")', j) + 1]
print('ROADMAP P7 quote == first four:', seg[seg.index(': "', seg.index('DECISIONS.md')) + 3:-1] == FOUR, '; for step 2.3 only:', 'for foundation step 2.3 only' in seg, '; count:', rm.count('pointer, 2026-11-13:'))
st = joined(show('docs/STATUS.md'))
print('STATUS P8: quoted four sentences present:', ('"' + FOUR + '"') in st, '; count:', st.count(FOUR))
kq = [m for m in re.findall(r'"([^"]+)"', st) if 'KIT-FREEZE-2' in m]
print('STATUS kit quotes:', len(kq), '; inside his kit paragraph:', all(x in KIT for x in kq), '; quote is the whole paragraph after its first sentence-label:', any(KIT.endswith(x) for x in kq), '; chars left out at the paragraph end:', (len(KIT) - KIT.index(kq[0]) - len(kq[0])) if kq else None)
h10 = joined('\n'.join(dec))
m = re.search(r'Its sentence "([^"]+)" is changed for step 2\.3 only', h10)
own = [l for l in dec if l.startswith('    > I now have a second account')]
print('2026-11-10 header sentence quote inside that entry\'s own quote:', bool(m) and len(own) == 1 and m.group(1) in own[0])
