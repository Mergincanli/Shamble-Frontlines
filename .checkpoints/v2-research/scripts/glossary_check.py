"""Glossary check (advisor condition 9): for every glossary term, count its marking words and its retired synonyms in
each topic file, outside quoted source text (and, with --skip-records, outside bracketed dated records and part 7).
Usage: python3 glossary_check.py [--skip-records] [--json out.json]"""
import re, sys, json, collections
R='/home/user/Shamble-Frontlines/docs/research/'
files={'R1':'r1-simulation-core.md','R2':'r2-netcode.md','R3':'r3-rendering.md','R4':'r4-physics-navigation.md','R5':'r5-toolchain-quality.md','R6':'r6-infrastructure.md'}
skip_records='--skip-records' in sys.argv
g=open(R+'GLOSSARY.md').read()
entries=[]
for m in re.finditer(r'^### (.+?)\n(.*?)(?=^### |^## |\Z)', g, re.S|re.M):
    name=m.group(1).strip(); body=m.group(2)
    def field(k):
        f=re.search(r'\*\*'+k+r':\*\*(.*?)(?=\n- \*\*|\Z)', body, re.S)
        return f.group(1) if f else ''
    marked=re.findall(r'"([^"]+)"', field('Marked by'))
    retire=re.findall(r'"([^"]+)"', field('Retire'))
    entries.append((name, marked, retire))
def clean(text):
    lines=[]
    in_part7=False
    for ln in text.split('\n'):
        if ln.startswith('## '):
            in_part7 = bool(re.match(r'## (7\. )?Verifier spot-check', ln))
        if skip_records and in_part7: continue
        lines.append(ln)
    t='\n'.join(lines)
    t=re.sub(r'"[^"\n]{0,400}"', '""', t)          # straight-quoted source text
    t=re.sub(r'“[^”\n]{0,400}”', '“”', t)          # curly-quoted source text
    if skip_records:
        t=re.sub(r'\[(?:Fix pass|Second fix pass|Verifier|Second verifier pass|Assembler\'s note|Rust review|Security review)[^\]]*\]', '[]', t, flags=re.I)
    return t
out=collections.OrderedDict(); total_retired=0
for t,f in files.items():
    txt=clean(open(R+f).read()); low=txt.lower()
    for name,marked,retire in entries:
        for ph in marked:
            n=low.count(ph.lower())
            if n: out.setdefault(name,{}).setdefault(t,{}).setdefault('marked',{})[ph]=n
        for ph in retire:
            n=low.count(ph.lower())
            if n:
                out.setdefault(name,{}).setdefault(t,{}).setdefault('retired',{})[ph]=n; total_retired+=n
    hdr=txt.split('\n## ',1)[0]
    out.setdefault('_header_links_glossary',{})[t]=('GLOSSARY.md' in hdr)
print(f'terms {len(entries)}; retired-synonym hits outside quotes{" and dated records" if skip_records else ""}: {total_retired}')
for name,v in out.items():
    if name.startswith('_'): continue
    r={t:d.get('retired') for t,d in v.items() if d.get('retired')}
    if r: print(' RETIRED', name, r)
print(' header links GLOSSARY.md:', out['_header_links_glossary'])
if '--json' in sys.argv:
    json.dump(out, open(sys.argv[sys.argv.index('--json')+1],'w'), indent=1)
