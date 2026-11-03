# Apply every touch-up FIX block to 449356c and compare with HEAD for every file the diff touches.
import sys, subprocess, os
root = sys.argv[1]; os.chdir(root)
cp = '.checkpoints/v2-decision-drafts/'
files = ['touchup-engine-architect-0002-fixes.md','touchup-netcode-architect-fixes.md','touchup-infra-engineer-fixes.md',
         'touchup2-engine-architect-synthesis-fixes.md','touchup-test-engineer-fixes.md','touchup2-game-designer-fixes.md',
         'rust-reviewer-touchup-d3a-fixes.md','test-engineer-touchup-pm22-fixes.md','touchup3-engine-architect-repin-fixes.md']
targets = subprocess.run(['git','diff','--name-only','449356c','HEAD','--','docs/adr','docs/SPIKE_PLAN.md','docs/ENGINE_OVERVIEW.md'],capture_output=True,text=True).stdout.split()
docs = {t: subprocess.run(['git','show','449356c:'+t],capture_output=True,text=True).stdout for t in targets}
blocks=[]
for f in files:
    lines = open(cp+f).read().split('\n'); i=0
    while i < len(lines):
        if lines[i].startswith('### FIX'):
            name=lines[i]; fl=lines[i+1].split('FILE:')[1].strip()
            assert lines[i+2]=='BEFORE:', (f,name,lines[i+2])
            j=i+3; b=[]
            while lines[j]!='AFTER:': b.append(lines[j]); j+=1
            j+=1; a=[]
            while lines[j]!='END': a.append(lines[j]); j+=1
            blocks.append((f,name,fl,b,a)); i=j
        i+=1
bad=0
for f,name,fl,b,a in blocks:
    if fl not in docs: print('NOT IN DIFF', f, name, fl); bad+=1; continue
    bt='\n'.join(b)+'\n'; at='\n'.join(a)+'\n'
    n=docs[fl].count(bt)
    if n!=1: print('BEFORE COUNT', n, f, name); bad+=1
    docs[fl]=docs[fl].replace(bt,at,1)
print('blocks', len(blocks), 'problems', bad)
from collections import Counter
print(Counter(f for f,*_ in blocks))
for t in targets:
    head=open(t).read()
    if head==docs[t]: print(t, 'identical to HEAD')
    else:
        print(t, 'DIFFERS')
        open(cp+'verifier-fixcheck-d3-critic-scratch/rebuilt-'+t.split('/')[-1],'w').write(docs[t])
