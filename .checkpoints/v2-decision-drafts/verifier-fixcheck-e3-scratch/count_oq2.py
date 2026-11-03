import re,glob
tot=op=0
for f in sorted(glob.glob('/home/user/Shamble-Frontlines/docs/adr/000[1-6]-*.md')):
    t=open(f).read(); n=re.search(r'/000(\d)-',f).group(1)
    ents=list(re.finditer(r'^- \*\*(OQ-000'+n+r'-[A-Z]*\d+)\.(.*?)(?=^- \*\*OQ-|^#|\Z)',t,re.S|re.M))
    ans=[];part=[];moved=[]
    for m in ents:
        body=' '.join(m.group(2).split())
        if re.search(r'\*\*\s*Moved',body): moved.append(m.group(1)); continue
        if re.search(r'[Aa]nswered in part',body): part.append(m.group(1)); continue
        if re.search(r'\bAnswered\b|\banswered by\b',body): ans.append(m.group(1))
    o=len(ents)-len(ans)-len(moved); tot+=len(ents); op+=o
    print(f.split('/')[-1],len(ents),'open',o,'ans',ans,'moved',moved,'part',part)
print(tot,op)
