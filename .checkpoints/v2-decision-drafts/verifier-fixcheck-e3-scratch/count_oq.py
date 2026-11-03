import re,sys,glob
tot=0;op=0
for f in sorted(glob.glob('/home/user/Shamble-Frontlines/docs/adr/000[1-6]-*.md')):
    t=open(f).read(); n=re.search(r'/000(\d)-',f).group(1)
    ents=list(re.finditer(r'^- \*\*(OQ-000'+n+r'-([A-Z]*)(\d+))\.(.*?)\*\*(.*?)(?=^- \*\*OQ-|^#|\Z)',t,re.S|re.M))
    ids=[m.group(1) for m in ents]
    dup=[i for i in set(ids) if ids.count(i)>1]
    groups={}
    for m in ents: groups.setdefault(m.group(2),[]).append(int(m.group(3)))
    gaps={g:(sorted(v)==list(range(1,max(v)+1))) for g,v in groups.items()}
    ans=[];part=[];moved=[]
    for m in ents:
        after=' '.join(m.group(5).split())[:120]
        if re.match(r'\s*Moved',after): moved.append(m.group(1))
        elif re.search(r'[Aa]nswered in part',after): part.append(m.group(1))
        elif re.search(r'\bAnswered\b',after): ans.append(m.group(1))
    o=len(ids)-len(ans)-len(moved)
    print(f.split('/')[-1], len(ids),'open',o,'answered',ans,'moved',moved,'part',part,'dup',dup,'contiguous',gaps)
    tot+=len(ids); op+=o
print('total',tot,'open',op)
