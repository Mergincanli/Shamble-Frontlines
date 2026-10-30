import re, json, subprocess, collections, glob, os, sys
REPO='/home/user/Shamble-Frontlines'
R=REPO+'/docs/research/'
C=REPO+'/.checkpoints/v2-research/'
files={'R1':'r1-simulation-core.md','R2':'r2-netcode.md','R3':'r3-rendering.md','R4':'r4-physics-navigation.md','R5':'r5-toolchain-quality.md','R6':'r6-infrastructure.md'}
SHA=subprocess.run(['git','-C',REPO,'rev-parse','--short','HEAD'],capture_output=True,text=True).stdout.strip()
URL=re.compile(r'https?://[^\s|)`>\]"]+')
def norm(u):
    u=u.rstrip('.,;:')
    u=re.sub(r'^https?://','',u); u=re.sub(r'^www\.','',u); return u.rstrip('/')
def esc(x): return re.sub(r'\s+',' ',x.replace('|','\\|')).strip()
QUESTIONS={'R1':{'ecs':'ECS or plain data','tick':'a fixed tick rate','determinism':'identical results between Wasm and native','boundary-worker':'the Rust ↔ TypeScript boundary; the simulation in a Web Worker','abilities':'a data-driven ability and effect system','data-hotreload':'data formats and hot reload of tuning'},
 'R2':{'sync':'snapshots, prediction, interpolation and lag compensation','budgets':'tick rate and bandwidth budgets','interest':'interest management for a top-down camera','transport':'the WebTransport server stack and the WebSocket fallback','rooms':'rooms and private matches'},
 'R3':{'wgpu':'three.js WebGPURenderer with TSL','gl2':'the WebGL 2 fallback','look':'candidate techniques for a comic look','char':'characters (skinning, instancing, LOD)','budget':'budgets for the minimum and recommended hardware','measure':'how we measure it','audio':'spatial audio with Web Audio'},
 'R4':{'phys':'Rapier 3D or custom','nav':'navigation meshes in Rust','floors':'bots moving between floors','los':'line of sight across heights','obst':'obstacles placed mid-match'},
 'R5':{'layout':'repository layout','tslint':'strict TypeScript and lint rules','rustlint':'clippy and rustfmt','test':'tests','gate':'the gate v2','ci':'CI with Rust caching; preview links per PR'},
 'R6':{'host':'hosting for previews and EU game servers','cost':'costs','obs':'logs, metrics and crash reports','tel':'balance telemetry','sec':'security baseline (deferred)'}}
DRAFTS={'R3':'graphics-engineer-r3-r3-{q}-draft.md','R4':'engine-architect-r4-r4-{q}-draft.md','R5':'test-engineer-r5-{q}-draft.md','R6':'infra-engineer-r6-r6-{q}-draft.md'}
def question_for(t, urls, unit, phrase=''):
    if t in ('R1','R2'):
        us=[u.strip() for u in unit.split(',') if u.strip()]
        return '; '.join(QUESTIONS[t].get(u,u) for u in us)
    found=[]
    for q,name in QUESTIONS[t].items():
        if t=='R5' and q=='ci': fn=C+'infra-engineer-r5-ci-draft.md'
        else: fn=C+DRAFTS[t].format(q=q)
        if not os.path.exists(fn): continue
        txt=norm_text=open(fn).read()
        if urls:
            if any(norm(u) in re.sub(r'https?://(www\.)?','',txt) for u in urls): found.append(name)
        elif phrase and phrase in re.sub(r'\s+',' ',txt): found.append(name)
    return '; '.join(found)
# 1. hinge tables
hinge=[]; hinge_norm=collections.defaultdict(list)
for t,f in files.items():
    sec=None; hdr=None
    for ln in open(R+f).read().split('\n'):
        if ln.startswith('#'): sec=ln; hdr=None; continue
        if sec and 'hinge on a source not read' in sec and ln.startswith('|'):
            cells=[c.strip() for c in ln.strip().strip('|').split('|')]
            if hdr is None: hdr=[c.lower() for c in cells]; continue
            if set(''.join(cells))<=set('-: '): continue
            d=dict(zip(hdr,cells))
            rec=d.get('recommendation','')
            src=d.get('source not read') or d.get('unread source') or d.get('source') or ''
            why=d.get('reason') or d.get('why not read') or d.get('reason not read') or ''
            need=d.get('what hinges on it') or d.get('tied by') or ''
            if not need:
                m=re.match(r'(\S+)\s*\((.*)\)\s*$', rec); need=m.group(2) if m else ''
            hinge.append((t,rec,need,src,why))
            for u in URL.findall(src): hinge_norm[norm(u)].append((t,rec.split(' ')[0]))
# 2. not-read tables
rows=[]; tables=0
for t,f in files.items():
    sec=None; hdr=None
    for i,ln in enumerate(open(R+f).read().split('\n')):
        if ln.startswith('#'):
            if re.search(r'[Ss]ources not read', ln): tables+=1
            sec=ln; hdr=None; continue
        if sec and re.search(r'[Ss]ources not read', sec) and ln.startswith('|'):
            cells=[c.strip() for c in ln.strip().strip('|').split('|')]
            if hdr is None: hdr=[c.lower() for c in cells]; continue
            if set(''.join(cells))<=set('-: '): continue
            d=dict(zip(hdr,cells))
            src=d.get('source') or d.get('url') or cells[1 if hdr[0]=='#' else 0]
            reason=next((v for k,v in d.items() if k.startswith('reason') or k.startswith('why')),'')
            unit=d.get('unit') or d.get('units') or ''
            urls=[u.rstrip('.,;:') for u in URL.findall(src)]
            rows.append(dict(topic=t,src=src,reason=reason,unit=unit,urls=urls,line=i+1,group=sec.strip('# ').strip()))
def cites(t,u):
    s=open(R+files[t]).read(); s2=re.sub(r'https?://(www\.)?','',s)
    out=set(); key=norm(u)
    for m in re.finditer(re.escape(key), s2):
        pre=s2[max(0,m.start()-3000):m.start()]
        # skip hits inside the sources tables
        line=s2[s2.rfind('\n',0,m.start())+1:m.start()]
        if line.startswith('|'): continue
        ids=re.findall(r'\*\*('+t+r'-[A-Z0-9]+-(?:F|REC|OQ)\d+)\.', pre)
        if ids: out.add(ids[-1])
    return out
def note(u):
    key=norm(u)[:90]
    res=subprocess.run(['grep','-rn','-F',key,'--include=*.md',C],capture_output=True,text=True).stdout.split('\n')
    best=None
    for o in res:
        if not o or '-draft.md:' in o: continue
        path,_,rest=o.partition(':'); ln,_,txt=rest.partition(':')
        txt=re.sub(r'\s+',' ',txt.strip())
        if txt.startswith('|') or len(txt)<len(key)+25: continue
        if best is None or len(txt)<len(best[2]): best=(os.path.basename(path),ln,txt)
    return best
merged=collections.OrderedDict()
for r in rows:
    k='URL:'+norm(r['urls'][0]) if r['urls'] else 'TEXT:'+r['topic']+':'+r['src'][:100]
    m=merged.setdefault(k,dict(src=r['src'],topics=[],reasons=[],cited=set(),norms=set(norm(u) for u in r['urls']),lead=False))
    ph=re.sub(r'\s+',' ',re.sub(r'[`*]','',r['src'])).strip()[:28]
    q=question_for(r['topic'], r['urls'], r['unit'], ph if not r['urls'] else '')
    m['topics'].append(r['topic']+(f' ({q})' if q else ' (question not recorded)'))
    m['reasons'].append(r['topic']+': '+r['reason'])
    for u in r['urls']: m['cited']|=cites(r['topic'],u); m['norms'].add(norm(u))
    if re.search(r'search (lead|result)', r['reason'], re.I): m['lead']=True
out=[]
for k,m in merged.items():
    need=[]
    hits=sorted(set(f"{t} {rid}" for n in m['norms'] for (t,rid) in hinge_norm.get(n,[])))
    if hits: need.append('a recommendation hinges on it (part 1: '+', '.join(hits)+')')
    if m['cited']: need.append('named by '+', '.join(sorted(m['cited'])))
    if not need:
        nb=next((note(n) for n in sorted(m['norms']) if note(n)), None) if m['norms'] else None
        if nb: need.append(f'the unit\'s note ({nb[0]}:{nb[1]} at {SHA}): "'+nb[2][:230]+('…' if len(nb[2])>230 else '')+'"')
        else: need.append('no tie found in the file or notes'+('; the topic calls it a search lead' if m['lead'] else ''))
    elif m['lead']: need.append('the topic calls it a search lead')
    out.append((m['src'],'; '.join(dict.fromkeys(m['topics'])),'; '.join(need),' / '.join(m['reasons'])))
# check: every normalized url once in part 2
seen=collections.Counter(n for m in merged.values() for n in m['norms'])
allnorm=set(norm(u) for r in rows for u in r['urls'])
dups=[n for n in allnorm if sum(1 for m in merged.values() if n in m['norms'])!=1]
L=['# Sources not read (R1–R6)','',
"Every source the six research topics of v2 phase 0 tried or found and did not read, and what each was needed for,",
"for the Advisor's search for other copies (the Advisor, 2026-10-30). Built on 2026-10-30 by the main session, with a",
f"script, from each topic's \"Sources not read\" tables (8) and \"Recommendations that hinge on a source not read\" tables (6),",
f"and the units' notes in `.checkpoints/v2-research/` at commit {SHA} (that folder is deleted when the change-set",
"closes; the commit keeps it). It lists and judges nothing, and claims nothing about what the sources say. No",
"recommendation rests on a source listed here: where one mattered, its topic turned the point into an open question that",
"says what the source would settle. Addresses are matched ignoring `http`/`https`, `www.` and a trailing slash.",'',
f"Counts: {len(rows)} not-read rows in the six topics ({', '.join(f'{t} {c}' for t,c in collections.Counter(r['topic'] for r in rows).items())}), merged into {len(merged)} sources ({len(allnorm)} distinct addresses, each listed once in part 2); {len(hinge)} rows in the hinge tables.",'',
'## 1. Sources a recommendation hinges on','',
"Each topic's own hinge table, row for row: the recommendation, what hinges on the source (or the open question that ties",
"them), the source as the topic names it, and why it was not read.",'',
'| Topic | Recommendation | What hinges on it | Source not read | Why not read | Other copy (the Advisor) |','|---|---|---|---|---|---|']
for t,rec,need,src,why in hinge: L.append(f'| {t} | {esc(rec)} | {esc(need)} | {esc(src)} | {esc(why)} | |')
L+=['','## 2. Every source not read','',
"One row per source, merged across the topics. \"Needed for\" gives, in this order: the part 1 rows that name it; the",
"findings, recommendations or open questions that name it; the unit's own note on it (file and line, at the commit",
"above); or \"no tie found in the file or notes\". \"Search lead\" appears only where the topic's own words say so.",'',
'| Source | Sought by (topic and brief question) | Needed for | Why not read (each topic\'s words) | Other copy (the Advisor) |','|---|---|---|---|---|']
for src,top,need,why in out: L.append(f'| {esc(src)} | {esc(top)} | {esc(need)} | {esc(why)} | |')
open(R+'UNREAD_SOURCES.md','w').write('\n'.join(L)+'\n')
rep=dict(sha=SHA, tables=tables, rows=len(rows), merged=len(merged), distinct_urls=len(allnorm), hinge_rows=len(hinge), dup_or_missing=dups,
         by_need=collections.Counter(('hinge' if 'hinges' in n else 'named' if 'named by' in n else 'note' if 'note (' in n else 'none') for _,_,n,_ in out))
print(json.dumps(rep, default=str))
json.dump(rep, open('/tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/sp3/unread-check.json','w'), default=str, indent=1)
