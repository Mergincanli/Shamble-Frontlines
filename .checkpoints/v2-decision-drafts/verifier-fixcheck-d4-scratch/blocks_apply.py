# Apply touch-up 4's FIX blocks to the docs at 882cfa6 in commit order; compare to fd8c6f8 byte for byte.
import subprocess, re, sys, os
R = '/home/user/Shamble-Frontlines'
D = R + '/.checkpoints/v2-decision-drafts/'
files = ['touchup4-engine-architect-fixes.md', 'touchup4-rust-reviewer-fixes.md', 'touchup4-test-engineer-fixes.md']
def show(rev, path):
    return subprocess.run(['git', '-C', R, 'show', f'{rev}:{path}'], capture_output=True, text=True, check=True).stdout
blocks = []
for f in files:
    t = open(D + f).read()
    for m in re.finditer(r'^### FIX (\S+)\nFILE: (\S+)\nBEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND$', t, re.S | re.M):
        blocks.append((f, m.group(1), m.group(2), m.group(3) + '\n', m.group(4) + '\n'))
print('blocks', len(blocks))
docs = {}
for f, bid, path, b, a in blocks:
    if path not in docs:
        docs[path] = show('882cfa6', path)
    base = show('882cfa6', path)
    head = show('fd8c6f8', path)
    nb = docs[path].count(b)
    print(f'{f[8:20]:12s} {bid:14s} {path:45s} before_in_base={base.count(b)} before_in_cur={nb} after_in_head={head.count(a)} before_in_head={head.count(b)}')
    if nb == 1:
        docs[path] = docs[path].replace(b, a)
for path, txt in docs.items():
    head = show('fd8c6f8', path)
    print(path, 'applied == fd8c6f8:', txt == head)
changed = subprocess.run(['git', '-C', R, 'diff', '--name-only', '882cfa6', 'fd8c6f8', '--', 'docs/adr', 'docs/SPIKE_PLAN.md', 'docs/ENGINE_OVERVIEW.md'], capture_output=True, text=True).stdout.split()
print('changed docs:', changed)
print('changed but no block:', [c for c in changed if c not in docs])
