import re,glob
m=open('stop-answers-map.md').read()
rows=re.findall(r'^\| ([A-Z]+\d+) \|',m,re.M)
print('map rows',len(rows),'unique',len(set(rows)))
heads={}
for f in sorted(glob.glob('stop-*-fixes.md')):
    for h in re.findall(r'^### ((?:FIX|NOTE)\b.*)$',open(f).read(),re.M):
        heads.setdefault(f,[]).append(h)
cover={}
for f,hs in heads.items():
    for h in hs:
        kind=h.split()[0]
        ids=re.findall(r'\b([A-Z]{1,2}\d+)\b',h.split('—')[0].split(':')[0])
        for i in ids: cover.setdefault(i,[]).append((f.replace('stop-','').replace('-fixes.md',''),kind))
missing=[r for r in rows if r not in cover]
print('missing',missing)
extra=sorted(set(cover)-set(rows))
print('ids in blocks not in map',extra[:60])
import collections
print('kinds', collections.Counter(tuple(sorted(set(k for _,k in cover[r]))) for r in rows if r in cover))
open(S+'/map_disp.tsv','w').write('\n'.join(f"{r}\t{cover.get(r)}" for r in rows))
