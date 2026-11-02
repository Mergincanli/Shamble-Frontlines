import sys
sys.path.insert(0,'.')
from wrap import L
from pwrap import pwrap
cur=L[868:876]
orig=' '.join(l.strip() for l in cur)
PR=["⌊lifetime ÷ fire interval⌋ + 1","⌊1.4 ÷ 1.05⌋ + 1 = 2","2 × 48 = 96","N = 48","1.4 s","1.05 s","option (a)","- Live"]
w0=pwrap(orig,'    ','      ',PR)
print('orig identical:', w0==cur)
for a,b in zip(w0,cur):
    if a!=b: print('NEW',a); print('OLD',b)
G=orig.replace("times the weapon's pellets;","times max(1, the weapon's pellets);")
w=pwrap(G,'    ','      ',PR+["max(1, the weapon's pellets)"])
print(len(w),[len(x) for x in w]); print('\n'.join(w))
