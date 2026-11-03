import sys, subprocess, re, os
root = sys.argv[1]
os.chdir(root)
cp = '.checkpoints/v2-decision-drafts/'
files = ['touchup-netcode-architect-fixes.md','touchup-infra-engineer-fixes.md','touchup-engine-architect-0002-fixes.md','touchup-test-engineer-fixes.md']
targets = ['docs/adr/0001-multiplayer-netcode.md','docs/adr/0002-code-structure-build-toolchain.md','docs/adr/0006-infrastructure-operations.md']
docs = {t: subprocess.run(['git','show','449356c:'+t],capture_output=True,text=True).stdout for t in targets}
blocks=[]
for f in files:
    lines = open(cp+f).read().split('\n')
    i=0
    while i < len(lines):
        if lines[i].startswith('### FIX'):
            name=lines[i]; fl=lines[i+1].split('FILE:')[1].strip()
            assert lines[i+2]=='BEFORE:', (f,name)
            j=i+3; b=[]
            while lines[j]!='AFTER:': b.append(lines[j]); j+=1
            j+=1; a=[]
            while lines[j]!='END': a.append(lines[j]); j+=1
            blocks.append((f,name,fl,b,a)); i=j
        i+=1
for f,name,fl,b,a in blocks:
    if fl not in docs: continue
    bt='\n'.join(b)+'\n'; at='\n'.join(a)+'\n'
    n=docs[fl].count(bt)
    print(f, name, fl.split('/')[-1], 'occurs', n, 'lines', len(b),'->',len(a))
    docs[fl]=docs[fl].replace(bt,at,1)
for t in targets:
    head=open(t).read()
    print(t, 'identical to HEAD' if head==docs[t] else 'DIFFERS')
