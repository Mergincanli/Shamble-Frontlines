import sys
sys.path.insert(0,'.')
from wrap import L
from pwrap import pwrap
cur=L[868:876]
orig=' '.join(l.strip() for l in cur)
PR=["⌊lifetime ÷ fire interval⌋ + 1","⌊1.4 ÷ 1.05⌋ + 1 = 2","2 × 48 = 96","N = 48","1.4 s","1.05 s","option (a)","- Live"]
for rep in ["times max(1, its pellets);","times max(1, the weapon's `pellets`);","times the weapon's pellets, or 1 if it has none;","times its pellets (1 where `pellets` is 0);","times max(1, pellets);","times the weapon's pellets (at least 1);"]:
    G=orig.replace("times the weapon's pellets;",rep)
    pr=PR+[rep[6:-1]] if 'max' in rep else PR
    w=pwrap(G,'    ','      ',pr)
    print(rep, len(w),[len(x) for x in w])
    if len(w)==8: print('\n'.join(w))
