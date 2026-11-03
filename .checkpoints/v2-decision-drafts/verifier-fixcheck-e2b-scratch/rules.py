import re,subprocess,sys
F='docs/adr/0005-content-abilities-assets.md'
head=open(F,encoding='utf-8').read().split('\n')
diff=subprocess.run(['git','diff','-U0','4374a1c','HEAD','--',F],capture_output=True,text=True).stdout
added=set()
for h in re.finditer(r'^@@ -\S+ \+(\d+)(?:,(\d+))? @@',diff,re.M):
    s=int(h.group(1)); n=int(h.group(2) or 1)
    added.update(range(s,s+n))
print('added lines',len(added))
for i in sorted(added):
    L=head[i-1]
    if not L.startswith('|') and len(L)>120: print('LONG',i,len(L))
# tables: header cell count per table; check every row; PM rows
def cells(L):
    t=L.strip()
    t=re.sub(r'`[^`]*`',lambda m:m.group(0).replace('|','\x00'),t)
    return t.count('|')-1
i=0;bad=0
while i<len(head):
    if head[i].lstrip().startswith('|') and i+1<len(head) and re.match(r'^\s*\|[\s:|-]+\|\s*$',head[i+1]):
        n=cells(head[i]); j=i+2
        while j<len(head) and head[j].lstrip().startswith('|'):
            if cells(head[j])!=n: print('CELLS',j+1,cells(head[j]),'hdr',n); bad+=1
            if head[j].startswith('| PM-') and cells(head[j])!=7: print('PM7',j+1)
            j+=1
        if j<len(head) and head[j].strip()!='' : print('UNDER-TABLE',j+1,head[j][:80])
        i=j
    else: i+=1
print('cell mismatches',bad)
