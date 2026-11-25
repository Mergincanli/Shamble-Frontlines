# Resolves the merge of main 07d673b0 (#71, lane 1's ci-after-3a) into step 2.6's branch (4fda80a): DECISIONS and
# REPO_MAP only. Both sides' text kept; lane 1's lines byte for byte. Run inside the merge worktree; --write to write.
import re, sys

def blocks(lines):
    """Yields (start, mid, end) indices of each conflict block."""
    i = 0
    while i < len(lines):
        if lines[i].startswith('<<<<<<< '):
            mid = next(j for j in range(i, len(lines)) if lines[j] == '=======')
            end = next(j for j in range(mid, len(lines)) if lines[j].startswith('>>>>>>> '))
            yield i, mid, end
            i = end + 1
        else:
            i += 1

def entries(lines):
    """Splits DECISIONS lines into entries, each starting at a top-level '- **' line."""
    out = []
    for l in lines:
        if l.startswith('- **') or not out:
            out.append([l])
        else:
            out[-1].append(l)
    return out

TS = re.compile(r'Taken by Mustafa, (20\d\d-\d\d-\d\d), (\d\d:\d\d:\d\d) UTC')
out = {}

# DECISIONS: one block; ours (HEAD) and theirs (origin/main) are whole entries; ordered by time, newest first.
p = 'docs/DECISIONS.md'
L = open(p, encoding='utf-8').read().split('\n')
bs = list(blocks(L))
assert len(bs) == 1, bs
s, m, e = bs[0]
ours, theirs = entries(L[s + 1:m]), entries(L[m + 1:e])
for x in ours + theirs:
    assert TS.search(x[0]), x[0][:100]
keyed = [(TS.search(x[0]).groups(), 'ours', x) for x in ours] + [(TS.search(x[0]).groups(), 'theirs', x) for x in theirs]
keyed.sort(key=lambda k: k[0], reverse=True)
print('DECISIONS order:', [(f'{k[0][1]}', k[1]) for k in keyed])
merged = [l for k in keyed for l in k[2]]
out[p] = L[:s] + merged + L[e + 1:]

# REPO_MAP: two blocks; theirs changed a row (byte for byte), ours kept that row as the base had it and added rows
# after it. Result: theirs' row, then ours' added rows.
p = 'docs/REPO_MAP.md'
L = open(p, encoding='utf-8').read().split('\n')
res = []
last = 0
for s, m, e in blocks(L):
    ours_b, theirs_b = L[s + 1:m], L[m + 1:e]
    assert len(theirs_b) == 1, theirs_b
    key = theirs_b[0].split('|')[1].strip()
    assert ours_b[0].split('|')[1].strip() == key, (ours_b[0][:60], key)
    res += L[last:s] + theirs_b + ours_b[1:]
    last = e + 1
    print('REPO_MAP block at', s + 1, ': theirs\'', key, '+ ours\'', [x.split('|')[1].strip() for x in ours_b[1:]])
res += L[last:]
out[p] = res

for p, lines in out.items():
    assert not any(l.startswith(('<<<<<<< ', '>>>>>>> ')) or l == '=======' for l in lines), p
    if '--write' in sys.argv:
        open(p, 'w', encoding='utf-8').write('\n'.join(lines))
print('written' if '--write' in sys.argv else 'dry run')
