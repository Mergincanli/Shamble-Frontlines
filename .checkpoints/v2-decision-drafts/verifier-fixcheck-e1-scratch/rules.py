import subprocess,re,sys
base,head='4374a1c','HEAD'
for f in sys.argv[1:]:
    new=subprocess.run(['git','show',f'{head}:{f}'],capture_output=True,text=True).stdout.split('\n')
    diff=subprocess.run(['git','diff','-U0',base,head,'--',f],capture_output=True,text=True).stdout
    changed=set()
    for m in re.finditer(r'^@@ -\S+ \+(\d+)(?:,(\d+))? @@',diff,re.M):
        s=int(m.group(1)); n=int(m.group(2) or 1)
        changed.update(range(s,s+n))
    print(f,'changed lines',len(changed))
    for i in sorted(changed):
        L=new[i-1]
        t=L.lstrip()
        if not t.startswith('|') and len(L)>120: print(f'  LONG {i} ({len(L)}): {L[:80]}')
        if t.startswith('| PM-'):
            cells=L.strip().strip('|').split(' | ')
            # count by unescaped pipes
            n=len(re.findall(r'(?<!\\)\|',L.strip()))-1
            if n!=7: print(f'  PMCELLS {i} n={n}: {L[:60]}')
    for i in range(1,len(new)):
        prev,cur=new[i-1],new[i]
        if prev.lstrip().startswith('|') and cur.strip() and not cur.lstrip().startswith('|'):
            if (i+1) in changed or i in changed: print(f'  UNDER-TABLE {i+1}: {cur[:90]}')
