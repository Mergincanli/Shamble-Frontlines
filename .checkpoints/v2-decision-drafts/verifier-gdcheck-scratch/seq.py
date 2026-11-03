import re,sys
exec(open('structural.py').read().split("a=struct(")[0])
TAG2=re.compile(r'^(\s*)- (What|Why|Research|Need|When a pool is full|Not here|Added for UPGRADE-SCOPE|Why it matters|Options[^:]*|Decider)[:( ]')
def seq(path):
    d={}; order=[]; item=None
    for n,l in section(path):
        m=re.match(r'^- \*\*((AB|OQ-0005-AB)[-0-9]*)\.',l)
        if l.startswith('#'): item=None; continue
        if m: item=m.group(1); d[item]=[]; order.append(item); continue
        if re.match(r'^- \*\*',l): item=None; continue
        m=TAG2.match(l)
        if m and item: d[item].append(('  '*(len(m.group(1))//2-1))+m.group(2).split(' (')[0])
    return order,d
oa,da=seq(sys.argv[1]); ob,db=seq(sys.argv[2])
print('A items',oa); print('B items',ob)
for it in ob:
    if da.get(it)!=db[it]: print(it,'\n  A:',da.get(it),'\n  B:',db[it])
    else: print(it,'same:',db[it])
