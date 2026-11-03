import sys
sys.path.insert(0,'.')
from wrap import L
from pwrap import pwrap
cur=L[868:876]
orig=' '.join(l.strip() for l in cur)
PR=["⌊lifetime ÷ fire interval⌋ + 1","⌊1.4 ÷ 1.05⌋ + 1 = 2","2 × 48 = 96","N = 48","1.4 s","1.05 s","option (a)","- Live","lines 175 and 180","(⌊1.4 ÷ 1.05⌋ + 1) × 1 = 2","`pellets` 0","at least 1"]
P="times the weapon's pellets;"
M="(fuse 1.4 s, r1:1373; fire interval 1.05 s, LEGACY_TUNING.md, `WEAPONS`, lines 167–203)"
G="gives ⌊1.4 ÷ 1.05⌋ + 1 = 2 per thrower"
for s in (P,M,G): assert orig.count(s)==1, s
C={
'c1': [(P,"times its pellets, at least 1;"),(M,"(fuse 1.4 s, r1:1373; fire interval 1.05 s and `pellets` 0, LEGACY_TUNING.md, `WEAPONS`, lines 175 and 180)"),(G,"gives (⌊1.4 ÷ 1.05⌋ + 1) × 1 = 2 per thrower")],
'c1b': [(P,"times its pellets, at least 1;"),(M,"(fuse 1.4 s, r1:1373; fire interval 1.05 s and `pellets` 0, LEGACY_TUNING.md, `WEAPONS`, lines 175 and 180)")],
'c4': [(P,"times the weapon's pellets, at least 1;"),(M,"(fuse 1.4 s, r1:1373; fire interval 1.05 s, `pellets` 0: LEGACY_TUNING.md, `WEAPONS`, lines 175 and 180)"),(G,"gives (⌊1.4 ÷ 1.05⌋ + 1) × 1 = 2 per thrower")],
'c5': [(P,"times its pellets, at least 1;"),(M,"(fuse 1.4 s, r1:1373; fire interval 1.05 s, `pellets` 0: LEGACY_TUNING.md, `WEAPONS`, lines 175 and 180)"),(G,"gives (⌊1.4 ÷ 1.05⌋ + 1) × 1 = 2 per thrower")],
}
for k,reps in C.items():
    t=orig
    for a,b in reps: t=t.replace(a,b)
    w=pwrap(t,"    ","      ",[p for p in PR if p in t])
    print(k,len(w),[len(x) for x in w])
    if len(w)<=8: print('\n'.join(w)); print()
