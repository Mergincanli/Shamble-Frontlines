import re, sys, glob
root = sys.argv[1]
tot = 0
for p in sorted(glob.glob(root + '/docs/adr/000[1-6]-*.md')):
    L = open(p, encoding='utf-8').read().split('\n')
    pm = [l for l in L if re.match(r'\| PM-000\d-\d+ \|', l)]
    bad = [l[:40] for l in pm if l.count('|') != 8]
    tot += len(pm)
    print(p.split('/')[-1][:8], 'PM rows', len(pm), 'not 7 cells', len(bad))
print('PM total', tot)
p = root + '/docs/adr/0004-simulation-core.md'
L = open(p, encoding='utf-8').read().split('\n')
# tables: runs of lines starting with '|'; check cell counts uniform and the line after is blank
i = 0; n = 0; glued = []; uneven = []
in_code = False
while i < len(L):
    if L[i].startswith('```'): in_code = not in_code
    if not in_code and L[i].startswith('|'):
        j = i
        while j < len(L) and L[j].startswith('|'): j += 1
        n += 1
        counts = {l.count('|') - l.count('\\|') for l in L[i:j]}
        if len(counts) > 1: uneven.append((i + 1, counts))
        if j < len(L) and L[j].strip() != '': glued.append(j + 1)
        i = j; continue
    i += 1
print('0004 tables', n, 'uneven', uneven, 'sentence directly under a table at', glued)
mig = [l for l in L if re.match(r'\| [0-8] \|', l)]
print('migration rows', len(mig), 'pipes', sorted({l.count('|') for l in mig}))
long = [(k + 1, len(l)) for k, l in enumerate(L) if len(l) > 120 and not l.startswith('|') and not l.startswith('```')]
print('prose lines over 120 in 0004:', len(long), long[:5])
