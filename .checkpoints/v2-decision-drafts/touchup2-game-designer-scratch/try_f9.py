import sys
sys.path.insert(0,'.')
from wrap import L, wrap
cur=L[868:876]
orig=' '.join(l.strip() for l in cur)
w0=wrap(orig,'    ','      ')
print('orig rewrap identical:', w0==cur)
P="times the weapon's pellets;"
M="(fuse 1.4 s, r1:1373; fire interval 1.05 s, LEGACY_TUNING.md"
G="gives ⌊1.4 ÷ 1.05⌋ + 1 = 2 per thrower"
cands={
'G': orig.replace(P,"times max(1, the weapon's pellets);"),
'G2': orig.replace(P,"times max(1, the weapon's pellets);").replace(G,"gives (⌊1.4 ÷ 1.05⌋ + 1) × 1 = 2 per thrower"),
'G3': orig.replace(P,"times max(1, the weapon's pellets);").replace(G,"gives (⌊1.4 ÷ 1.05⌋ + 1) × 1 = 2 per thrower").replace(M,"(fuse 1.4 s, r1:1373; fire interval 1.05 s and `pellets` 0, LEGACY_TUNING.md"),
'H': orig.replace(P,"times the weapon's pellets (1 where `pellets` is 0);"),
'H2': orig.replace(P,"times the weapon's pellets (1 where `pellets` is 0);").replace(G,"gives (⌊1.4 ÷ 1.05⌋ + 1) × 1 = 2 per thrower"),
'H3': orig.replace(P,"times the weapon's pellets (1 where `pellets` is 0);").replace(M,"(fuse 1.4 s, r1:1373; fire interval 1.05 s, `pellets` 0, LEGACY_TUNING.md"),
'I': orig.replace(P,"times the weapon's pellets (at least 1);"),
}
for k,t in cands.items():
    w=wrap(t,'    ','      ')
    print(k,len(w),[len(x) for x in w])
    if len(w)==8:
        print('\n'.join(w)); print()
