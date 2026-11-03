import sys, re
def check(path, orig):
    L = open(path).read().split('\n'); O = open(orig).read().split('\n')
    bad = 0
    # tables: header cell count per table; no text line directly under a table
    i = 0
    while i < len(L):
        if L[i].lstrip().startswith('|') and i+1 < len(L) and re.match(r'^\s*\|[-| :]+\|\s*$', L[i+1]):
            n = L[i].count('|') - L[i].count('\\|')
            j = i
            while j < len(L) and L[j].lstrip().startswith('|'):
                c = L[j].count('|') - L[j].count('\\|')
                if c != n: print(path, j+1, 'cells', c, 'vs', n); bad += 1
                j += 1
            if j < len(L) and L[j].strip() != '': print(path, j+1, 'text directly under a table'); bad += 1
            i = j
        else: i += 1
    so = set(O)
    for k, l in enumerate(L):
        if l not in so and not l.lstrip().startswith('|') and len(l) > 120:
            print(path, k+1, 'long', len(l)); bad += 1
    pm = lambda X: len([x for x in X if re.match(r'^\| PM-\d{4}-\d+ \|', x)])
    oq = lambda X: len(re.findall(r'OQ-\d{4}-[A-Z]*\d+', '\n'.join(X)))
    print(path.split('/')[-1], 'PM rows', pm(O), '->', pm(L), '| changed new lines', len([l for l in L if l not in so]), '| problems', bad)
for p in sys.argv[1:]:
    check('tree/' + p, '/home/user/Shamble-Frontlines/' + p)
