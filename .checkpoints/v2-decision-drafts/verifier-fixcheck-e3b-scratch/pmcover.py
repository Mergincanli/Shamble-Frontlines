import re
t=open('/home/user/Shamble-Frontlines/docs/SPIKE_PLAN.md').read()
t=re.sub(r'\s+',' ',t)
cov={a:set() for a in range(1,7)}
# sequences: PM-000A-n then continuation items like ", -m", " to -m", " to PM-000A-m", " and -m"
for m in re.finditer(r'PM-000([1-6])-(\d+)((?:(?:,| and| to|;)? ?(?:PM-000\1-|-)\d+)*)',t):
    a=int(m.group(1)); cur=int(m.group(2)); cov[a].add(cur)
    for sep,n in re.findall(r'(,| and| to|;)? ?(?:PM-000%d-|-)(\d+)'%a,m.group(3)):
        n=int(n)
        if sep==' to':
            cov[a].update(range(cur,n+1))
        else: cov[a].add(n)
        cur=n
tot={1:63,2:78,3:47,4:75,5:46,6:32}
for a in tot:
    miss=sorted(set(range(1,tot[a]+1))-cov[a]); extra=sorted(cov[a]-set(range(1,tot[a]+1)))
    print(a,tot[a],'missing',miss,'beyond',extra)
