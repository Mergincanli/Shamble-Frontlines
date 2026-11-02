import sys
sys.path.insert(0,'.')
from wrap import L, wrap
orig=' '.join(l.strip() for l in L[868:876])
assert orig.startswith('- Live projectiles')
cands={
'A': orig.replace("times the weapon's pellets;", "times the weapon's pellets (1 where `pellets` is 0, as for legacy's Molotov);"),
'B': orig.replace("times the weapon's pellets;", "times the weapon's pellets (1 where `pellets` is 0);").replace("(fuse 1.4 s, r1:1373; fire interval 1.05 s,", "(fuse 1.4 s, r1:1373; fire interval 1.05 s and `pellets` 0,"),
'C': orig.replace("times the weapon's pellets;", "times the weapon's pellets (1 where `pellets` is 0, as for the Molotov);"),
'D': orig.replace("times the weapon's pellets;", "times the weapon's pellets (1 where legacy's `pellets` is 0, as for the Molotov);"),
}
for k,t in cands.items():
    w=wrap(t,'    ','      ')
    print(k,len(w),[len(x) for x in w])
    if len(w)==8:
        print('\n'.join(w))
