import re
R='/home/user/Shamble-Frontlines/docs/'
readme=open(R+'adr/README.md').read().splitlines()
docs={}
def doc(p):
    if p not in docs: docs[p]=open(p).read()
    return docs[p]
issues=[];n=0
for i,l in enumerate(readme,1):
    for m in re.finditer(r'\[([^\]]+)\]\(([^)#]+\.md)(#[^)]*)?\)([^\[|]*)',l):
        label,path,anchor,rest=m.groups()
        if not re.match(r'(\.\./)?0\d{3}-|\.\./SPIKE_PLAN|\.\./ENGINE',path): continue
        full=re.sub(r'[^/]+/\.\./','',R+'adr/'+path)
        t=doc(full); n+=1
        rest=rest.strip(' ;,.')
        for oq in re.findall(r'OQ-\d{4}-[A-Z]*\d+',rest):
            if not re.search(r'\*\*'+re.escape(oq)+r'\.',t): issues.append((i,path,'OQ entry missing',oq))
        for pt in re.findall(r'parts? ([0-9][0-9a-z., and–-]*)',rest):
            for p in re.findall(r'\d+',pt):
                if not re.search(r'^#+ .*\b[Pp]art '+p+r'\b|^#+ '+p+r'\.',t,re.M): issues.append((i,path,'part heading missing',p))
        for sec in re.findall(r'"([^"]+)"',rest):
            if not re.search(r'^#+ .*'+re.escape(sec),t,re.M) and sec not in t: issues.append((i,path,'section missing',sec))
        if re.search(r'\bline',rest): issues.append((i,path,'line',rest))
        if not re.search(r'OQ-|part|"|[A-Z]{2}-\d|item|section|Open questions|§',rest) : issues.append((i,path,'names nothing?',rest[:80]))
print('pointers',n)
for x in issues: print(x)
