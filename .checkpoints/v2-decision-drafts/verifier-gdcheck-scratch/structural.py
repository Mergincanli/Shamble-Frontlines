import re,sys
def section(path):
    L=open(path,encoding='utf8').read().split('\n')
    s=next(i for i,l in enumerate(L) if l.startswith('## The ability and effect system (game-designer)'))
    e=next(i for i,l in enumerate(L) if l.startswith('## The asset pipeline (engine-architect)'))
    return [(i+1,L[i]) for i in range(s,e)]
TAG=re.compile(r'^(\s*)- (What|Why|Research|Need|When a pool is full|Not here|Added for UPGRADE-SCOPE|Why it matters|Options|Decider|Proposed|Recommendation)\b')
def struct(path):
    out=[]; item=None
    for n,l in section(path):
        if l.startswith('#'):
            item=l; out.append((n,'HEAD',l,item)); continue
        m=re.match(r'^- \*\*((AB|OQ-0005-AB)[-0-9]*)\.',l)
        if m:
            item=m.group(1); out.append((n,'ITEM',l,item)); continue
        if re.match(r'^- \*\*',l):
            item=l[:40]; out.append((n,'ITEM',l,item)); continue
        m=TAG.match(l)
        if m and item is not None:
            out.append((n,m.group(2)+'@'+str(len(m.group(1))),l,item))
    return out
a=struct(sys.argv[1]); b=struct(sys.argv[2])
print('counts', len(a), len(b))
from collections import Counter
bset=Counter(x[2] for x in b); aset=Counter(x[2] for x in a)
print('== in A not in B ==')
for n,t,l,it in a:
    if bset[l]==0: print(n,it,t,'|',l)
print('== in B not in A ==')
for n,t,l,it in b:
    if aset[l]==0: print(n,it,t,'|',l)
print('== doubled tag within one item (B) ==')
c=Counter((it,t) for n,t,l,it in b if t not in('HEAD','ITEM'))
for (it,t),k in c.items():
    if k>1: print(it,t,k,[ (n,l[:70]) for n,tt,l,i2 in b if i2==it and tt==t])
print('== doubled ITEM/HEAD lines (B) ==')
c2=Counter(l for n,t,l,it in b if t in('HEAD','ITEM'))
for l,k in c2.items():
    if k>1: print(k,l)
