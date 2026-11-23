import re,subprocess,json,sys
T=open('.checkpoints/ci-wasm/report-template.md',encoding='utf-8').read()
fill=json.load(open('.checkpoints/ci-wasm/report-fill.json',encoding='utf-8'))
diff=subprocess.run(['git','diff','origin/main','HEAD','--','.github/workflows/toolchain.yml'],capture_output=True,text=True,check=True).stdout.rstrip('\n')
N=open('.checkpoints/ci-wasm/infra-engineer-review-1.md',encoding='utf-8').read().split('\n')
a=next(i for i,l in enumerate(N) if l.startswith('### THE TABLE'))
b=next(i for i,l in enumerate(N) if i>a and l.rstrip().endswith('FIX-WRITE (a) on b4081e48...03ec).'))
table='\n'.join(N[a+1:b+1])
L=[re.sub(r'\x1b\[[0-9;]*m','',re.sub(r'^﻿?\S+Z ','',l.rstrip('\n'))) for l in open('.gate/ci-wasm/logs/push-137974047720.log',encoding='utf-8',errors='replace')]
def rng(pat_from,pat_to,inc_to=True):
    i=next(k for k,l in enumerate(L) if re.search(pat_from,l)); j=next(k for k,l in enumerate(L) if k>=i and re.search(pat_to,l))
    return L[i:j+1 if inc_to else j]
parts=[]
k=next(i for i,l in enumerate(L) if l=='Image: ubuntu-24.04'); parts+= L[k:k+2]
parts+= ['[...]']+rng(r'^final address: ',r'tar\.gz: OK$')
parts+= ['[...]']+rng(r'^gh version ',r'Verification succeeded!')
parts+= ['[...]']+rng(r'^drwxr-xr-x runner/runner',r'^/wasm-bindgen-0\.2\.130/wasm-bindgen: OK$')
parts+= ['[...]']+rng(r'^control: connected to ',r'^the network is cut: ')
parts+= ['[...]']+rng(r'^== the Wasm built here',r'^== the browser tests')
parts+= rng(r'^Google Chrome ',r'^\s+12 passed')
out=[]
for l in parts:
    l=re.sub(r'\?[A-Za-z0-9%&=._~+/-]{20,}','?(query left out)',l)
    out.append(l if len(l)<=400 else '[one readings line of %d characters left out]'%len(l))
quotes='\n'.join(out)
T=T.replace('{{WORKFLOW_DIFF}}',diff).replace('{{RULE_TABLE}}',table).replace('{{CLEAN_RUN_QUOTES}}',quotes)
for k,v in fill.items(): T=T.replace('{{'+k+'}}',v)
left=re.findall(r'\{\{[A-Z0-9_]+\}\}',T)
import textwrap
out2=[]; fence=False
for l in T.split('\n'):
    if l.startswith('```'): fence=not fence
    if fence or l.startswith('|') or len(l)<=120: out2.append(l); continue
    lead=re.match(r'\s*',l).group(0); m=re.match(r'\s*(- |\d+\. )',l)
    out2.extend(textwrap.wrap(l,width=118,subsequent_indent=lead+(' '*len(m.group(1)) if m else ''),break_long_words=False,break_on_hyphens=False))
T='\n'.join(out2)
open('docs/reports/2026-11-19-ci-wasm.md','w',encoding='utf-8').write(T)
print('written; lines',T.count('\n'),'| placeholders left:',left)
