import difflib, re, sys
ROOT='/home/user/Shamble-Frontlines/'
docs=['docs/adr/README.md','docs/SPIKE_PLAN.md','docs/ENGINE_OVERVIEW.md']
base = sys.argv[1] if len(sys.argv)>1 else 'tree/'
def cells(l):
    s=l.strip()
    s=re.sub(r'`[^`]*`', lambda m: m.group(0).replace('|','¦'), s)
    return len(s.strip('|').split('|'))
tot=0
for d in docs:
    o=open(ROOT+d,encoding='utf8').read().split('\n'); n=open(base+d,encoding='utf8').read().split('\n')
    sm=difflib.SequenceMatcher(None,o,n,autojunk=False)
    new=set()
    for tag,i1,i2,j1,j2 in sm.get_opcodes():
        if tag in('replace','insert'): new.update(range(j1,j2))
    long=[(j+1,len(n[j])) for j in sorted(new) if len(n[j])>120 and not n[j].lstrip().startswith('|')]
    # table cell counts: header row = first row of each table block
    bad=[]; glued=[]
    j=0
    while j<len(n):
        if n[j].startswith('|'):
            k=j
            while k<len(n) and n[k].startswith('|'): k+=1
            hc=cells(n[j])
            for t in range(j,k):
                if t in new and cells(n[t])!=hc: bad.append((t+1,cells(n[t]),hc))
            if k<len(n) and n[k].strip()!='' and not n[k].startswith('#'):
                glued.append(k+1)
            j=k
        else: j+=1
    # straight quotes per changed line block (odd count)
    odd=[j+1 for j in sorted(new) if n[j].count('"')%2 and n[j].startswith('|')]
    print(f'{d}: changed/new lines {len(new)}; prose lines >120: {long}; table rows with a wrong cell count: {bad}; text glued under a table: {glued}; table rows with odd straight quotes: {odd}')
    tot+=len(long)+len(bad)+len(glued)
print('problems', tot)
