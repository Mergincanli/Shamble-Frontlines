import re, subprocess
p='/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/test-engineer-B-fixes-final.md'
s=open(p).read()
blocks=re.findall(r'### FIX (.*?)\nFILE: (.*?)\nBEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND\n', s, re.S)
docs={}
for fid,f,b,a in blocks:
    c='72cfcc3' if '0005' in f else '75615d1'
    if f not in docs: docs[f]=subprocess.run(['git','-C','/home/user/Shamble-Frontlines','show',c+':'+f],capture_output=True,text=True).stdout
for fid,f,b,a in blocks:
    n=docs[f].count(b)
    if n!=1: print('FAIL',n,fid[:60]); continue
    docs[f]=docs[f].replace(b,a)
print(len(blocks),'blocks checked'); print(s.count('### NOTE'),'notes')
for f,t in docs.items():
    # a sentence directly under a table: a table row followed by a non-blank, non-table line
    L=t.split('\n')
    for i in range(len(L)-1):
        if L[i].startswith('|') and L[i+1].strip() and not L[i+1].startswith('|'): print('under-table',f,i+1,L[i+1][:60])
