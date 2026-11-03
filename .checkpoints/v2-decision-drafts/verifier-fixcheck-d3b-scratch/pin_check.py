import re,sys
S=sys.argv[1]
old_r=open(S+'/readme-449356c.md').read().split('\n')
new_r=open('docs/adr/README.md').read().split('\n')
old_d=open(S+'/0005-449356c.md').read().split('\n')
new_d=open('docs/adr/0005-content-abilities-assets.md').read().split('\n')
pat=re.compile(r'\[ADR 0005\]\(0005-content-abilities-assets\.md\) lines? (\d+)(?:[–-](\d+))?')
assert len(old_r)==len(new_r), (len(old_r),len(new_r))
bad=0; moved=0; same=0; n=0
for i,(o,nw) in enumerate(zip(old_r,new_r)):
    po=[m.groups() for m in pat.finditer(o)]; pn=[m.groups() for m in pat.finditer(nw)]
    if not po and not pn: continue
    if len(po)!=len(pn): print('COUNT MISMATCH', i+1); bad+=1; continue
    for (a,b),(c,d) in zip(po,pn):
        n+=1
        a,c=int(a),int(c)
        ok = old_d[a-1]==new_d[c-1]
        if b and d: ok = ok and old_d[int(b)-1]==new_d[int(d)-1]
        if c!=a: moved+=1
        else: same+=1
        ids=re.findall(r'OQ-0005-[A-Z]*\d+|AB-\d+|CD-\d+',nw.split('|')[1] if '|' in nw else nw)
        tgt=new_d[c-1]
        idok = all(x in tgt for x in ids) if ids else None
        if not ok or idok is False:
            bad+=1; print('BAD', i+1, a,'->',c, ids, repr(tgt[:120]))
# also: any other README line changed?
other=[i+1 for i,(o,nw) in enumerate(zip(old_r,new_r)) if o!=nw and not pat.search(nw)]
print('pointers',n,'moved',moved,'same',same,'bad',bad,'other changed lines',other)
