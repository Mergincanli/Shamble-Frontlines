"""Advisor condition (Retire split, item 4): the quoted phrases of every Retire field must be the same before and after
the split, apart from words moved, unquoted, into a "Keeps:" clause. Usage: retire_split_compare.py <before.md> <after.md>"""
import re, sys
def fields(path):
    g=open(path).read(); out={}
    for m in re.finditer(r'^### (.+?)\n(.*?)(?=^### |^## |\Z)', g, re.S|re.M):
        name, body = m.group(1).strip(), m.group(2)
        def f(k):
            x=re.search(r'\*\*'+re.escape(k)+r':\*\*(.*?)(?=\n- \*\*|\Z)', body, re.S); return x.group(1) if x else ''
        retire=set(re.findall(r'"([^"]+)"', f('Retire')+f('Retire (exact)')+f('Retire (in context)')))
        keeps=f('Keeps')
        out[name]=(retire, keeps)
    return out
b=fields(sys.argv[1]); a=fields(sys.argv[2]); bad=0
assert set(b)==set(a), ('entries differ', set(b)^set(a))
for name in b:
    rb,_=b[name]; ra,keeps=a[name]
    gone=rb-ra; new=ra-rb
    unexplained=[g for g in gone if g.lower() not in keeps.lower()]
    if new or unexplained:
        bad+=1; print('ENTRY', name, '| new quoted:', sorted(new), '| dropped, not in Keeps:', sorted(unexplained))
    elif gone:
        print('ok', name, '| moved to Keeps:', sorted(gone))
print('entries', len(b), 'problems', bad)
sys.exit(1 if bad else 0)
