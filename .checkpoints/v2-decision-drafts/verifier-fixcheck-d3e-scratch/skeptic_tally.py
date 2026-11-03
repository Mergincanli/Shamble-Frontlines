import re,sys
files=[f"verifier-skeptic-{i}-results.md" for i in range(1,6)]+["verifier-skeptic-6-d3a-1.md"]
for f in files:
    rows=[]
    for n,l in enumerate(open(f,encoding='utf-8'),1):
        if not l.startswith('|') or l.startswith('|---') : continue
        cells=[c.strip() for c in l.strip().strip('|').split('|')]
        if len(cells)<2: continue
        o=cells[1].lower()
        if o.startswith('upheld /') or o.startswith('outcome') or o.startswith('upheld/'): continue
        k='upheld' if o.startswith('upheld') else 'partly' if o.startswith('partly') else 'refuted' if o.startswith('refuted') else 'other:'+o[:30]
        rows.append((n,k,cells[0][:50]))
    from collections import Counter
    c=Counter(k for _,k,_ in rows)
    print(f, dict(c), len(rows))
    for r in rows: print('   ',r)
