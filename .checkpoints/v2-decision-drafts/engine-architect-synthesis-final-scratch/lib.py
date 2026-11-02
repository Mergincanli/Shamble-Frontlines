"""Helper for the synthesis final fix round: each fix()/note() call appends one block to the output file at once.
BEFORE is copied from the 604a497 snapshot by line range (whole lines) and must occur exactly once there and in the
applied copy as it stands (so two blocks never touch the same text). Blocks whose id is already in the output are
skipped (restart-safe). The applied copies live in applied/ (a preview of the docs after every block so far)."""
import os, re, json
ROOT = '/home/user/Shamble-Frontlines'
D = ROOT + '/.checkpoints/v2-decision-drafts'
S = D + '/engine-architect-synthesis-final-scratch'
OUT = D + '/engine-architect-synthesis-fixes-final.md'
NOTES = D + '/engine-architect-synthesis-final-notes.md'
P = {'docs/ENGINE_OVERVIEW.md': 'ENGINE_OVERVIEW.md', 'docs/SPIKE_PLAN.md': 'SPIKE_PLAN.md', 'docs/adr/README.md': 'README.md'}
os.makedirs(S + '/applied', exist_ok=True)

def snap(path):
    return open(f'{S}/snap/{P[path]}', encoding='utf8').read()

def _ap(path):
    return f'{S}/applied/{P[path]}'

def applied(path):
    p = _ap(path)
    return open(p, encoding='utf8').read() if os.path.exists(p) else snap(path)

def existing():
    if not os.path.exists(OUT): return set()
    return set(m.group(2).strip() for m in re.finditer(r'^### (FIX|NOTE) (.+)$', open(OUT, encoding='utf8').read(), re.M))

def lines(path, a, b=None):
    L = snap(path).split('\n')
    return '\n'.join(L[a - 1:(b or a)])

def find(path, needle):
    """1-based line numbers in the snapshot whose text contains needle."""
    return [i + 1 for i, l in enumerate(snap(path).split('\n')) if needle in l]

def fix(fid, path, a, b=None, after=None, subs=None):
    if fid in existing():
        print('skip (already written)', fid); return
    before = lines(path, a, b)
    assert snap(path).count(before) == 1, ('BEFORE not unique in snapshot', fid, snap(path).count(before))
    cur = applied(path)
    assert cur.count(before) == 1, ('BEFORE not unique in applied copy (overlap?)', fid, cur.count(before))
    if subs is not None:
        after = before
        for old, new in subs:
            assert after.count(old) == 1, ('sub not unique', fid, old, after.count(old))
            after = after.replace(old, new)
    assert after is not None and after != before, ('no change', fid)
    block = f'### FIX {fid}\nFILE: {path}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n'
    with open(OUT, 'a', encoding='utf8') as f: f.write('\n' + block)
    open(_ap(path), 'w', encoding='utf8').write(cur.replace(before, after, 1))
    print(f'FIX {fid}: {path} {a}-{b or a} -> {len(after.splitlines())} line(s)')

def note(fid, body):
    if fid in existing():
        print('skip (already written)', fid); return
    with open(OUT, 'a', encoding='utf8') as f: f.write(f'\n### NOTE {fid}\n{body.strip()}\nEND\n')
    print('NOTE', fid)

def log(line):
    with open(NOTES, 'a', encoding='utf8') as f: f.write('- ' + line.strip() + '\n')

def rebuild():
    """Replay every FIX block in OUT onto the snapshots, in order, into applied/ (asserting each BEFORE once)."""
    src = open(OUT, encoding='utf8').read()
    docs = {p: snap(p) for p in P}
    for m in re.finditer(r'^### FIX (.+?)\nFILE: *(\S+)\nBEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND\b', src, re.S | re.M):
        fid, fil, b, a = m.groups()
        assert docs[fil].count(b) == 1, ('replay', fid, docs[fil].count(b))
        docs[fil] = docs[fil].replace(b, a, 1)
    for p, t in docs.items(): open(_ap(p), 'w', encoding='utf8').write(t)
    print('rebuilt applied/ from', OUT)

def unfix(fid):
    """Withdraw a block written in this run (before anything depends on it), then rebuild applied/."""
    src = open(OUT, encoding='utf8').read()
    pat = re.compile(r'\n### (FIX|NOTE) ' + re.escape(fid) + r'\n.*?\nEND\n', re.S)
    assert len(pat.findall(src)) == 1, ('unfix', fid)
    open(OUT, 'w', encoding='utf8').write(pat.sub('', src, 1)); rebuild()
