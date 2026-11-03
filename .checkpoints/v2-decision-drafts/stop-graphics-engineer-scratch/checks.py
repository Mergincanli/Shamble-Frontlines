import re, sys, pathlib
S = pathlib.Path(sys.argv[1]); fixes = pathlib.Path(sys.argv[2]).read_text()
blocks = re.findall(r'^### FIX (.+?)\nFILE: (\S+)\nBEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND$', fixes, re.S | re.M)
print('blocks', len(blocks))
for rel in ['docs/adr/0003-rendering.md', 'docs/adr/0005-content-abilities-assets.md']:
    b = (S / 'before' / rel).read_text().split('\n'); a = (S / 'after' / rel).read_text().split('\n')
    bset = set(b)
    long_new = [(i+1, len(l)) for i, l in enumerate(a) if len(l) > 120 and l not in bset]
    print(rel, 'lines', len(b), '->', len(a), '| new lines over 120:', long_new)
    def under_table(L):
        return [i+1 for i in range(1, len(L)) if L[i-1].startswith('|') and L[i].strip() and not L[i].startswith('|')]
    print('  text directly under a table: before', len(under_table(b)), 'after', len(under_table(a)))
    def cells(L):
        return sorted((l.count('|') for l in L if l.startswith('|')))
    print('  table rows:', len(cells(b)), '->', len(cells(a)), '| cell-count profile same:', cells(b) == cells(a))
    # every BEFORE once in the original
    for bid, fil, bef, aft in blocks:
        if rel.endswith(fil):
            n = (S / 'before' / rel).read_text().count(bef)
            if n != 1: print('  BEFORE count', bid, n)
