import re,sys
def strip(l): return re.sub(r'^\S+Z ','',l.rstrip('\n'))
for path in sys.argv[1:]:
    L=[strip(l) for l in open(path,encoding='utf-8',errors='replace')]
    ok={}
    has=lambda pat: any(re.search(pat,l) for l in L)
    ok['final address on release-assets.githubusercontent.com']=has(r'^final address: https://release-assets\.githubusercontent\.com/')
    ok['size: 11860397']=has(r'^size: 11860397$')
    ok['the archive: OK']=has(r'wasm-bindgen-0\.2\.130-x86_64-unknown-linux-musl\.tar\.gz: OK$')
    v=[m.group(1) for l in L for m in [re.match(r'^gh version (\d+)\.(\d+)\.(\d+)',l)] if m]
    gv=[tuple(map(int,re.match(r'^gh version (\d+)\.(\d+)\.(\d+)',l).groups())) for l in L if re.match(r'^gh version \d+\.\d+\.\d+',l)]
    ok['gh 2.93.0 or later: '+('.'.join(map(str,gv[0])) if gv else 'none')]=bool(gv) and gv[0]>=(2,93,0)
    ok['gh success line naming release 0.2.130']=has(r'Verification succeeded! wasm-bindgen-0\.2\.130-x86_64-unknown-linux-musl\.tar\.gz is present in release 0\.2\.130$')
    mem=[l for l in L if re.search(r'runner/runner\s+\d+ .* wasm-bindgen-0\.2\.130-x86_64-unknown-linux-musl/',l)]
    ok['seven members: %d'%len(mem)]=len(mem)==7
    ok['the step environment line with the new WB_TOOL']=has(r'^\s+WB_TOOL: /wasm-bindgen-0\.2\.130/wasm-bindgen$')
    want=['mode 555, owner root:root, 8170784 bytes, /wasm-bindgen-0.2.130/wasm-bindgen','mode 755, owner root:root, 4096 bytes, /wasm-bindgen-0.2.130']
    i=L.index(want[0]) if want[0] in L else -1
    four = i>=0 and L[i+1]==want[1] and re.match(r'^mode 755, owner root:root, \d+ bytes, /$',L[i+2]) is not None and L[i+3]=='/wasm-bindgen-0.2.130/wasm-bindgen: OK'
    ok['the four install lines, in order, nothing between']=bool(four)
    ok['no "exists already or could not be made" outside the script echo']=not any(('exists already or could not be made' in l) and not l.lstrip().startswith('\x1b[36;1m') for l in L)
    ok['the network is cut']=has(r'^the network is cut: no TCP connection')
    tables=[l for l in L if re.match(r'^file\s+raw\s+gzip -9\s+brotli 11$',l)]
    ok['size tables: %d'%len(tables)]=len(tables)>=2
    C=[re.sub(r'\x1b\[[0-9;]*m','',l) for l in L]
    b0=next((n for n,l in enumerate(C) if l=='== the boundary tests, on the built Wasm'),-1)
    b1=next((n for n,l in enumerate(C) if n>b0 and l.startswith('== the type checks')),-1)
    ok['the boundary tests: Tests 7 passed (7), between their echo and the type checks']=b0>=0 and b1>b0 and any(re.match(r'^\s*Tests\s+7 passed \(7\)$',l) for l in C[b0:b1])
    ch=[l for l in L if re.match(r'^Google Chrome \d+\.',l)]
    ok['Chrome version line: '+(ch[0].strip() if ch else 'none')]=bool(ch)
    ok['Running 12 tests using 1 worker']=has(r'^Running 12 tests using 1 worker$')
    ok['12 passed']=has(r'^\s+12 passed')
    ok['no failed, flaky or retry line in the browser tests']=not any(re.search(r'^\s+\d+ (failed|flaky)|retry #|Retry #',l) for l in L)
    ok['no ##[error] line']=not has(r'##\[error\]')
    from datetime import datetime
    ts=[datetime.strptime(m.group(1)[:26],'%Y-%m-%dT%H:%M:%S.%f') for l in open(path,encoding='utf-8',errors='replace') for m in [re.match(r'^\ufeff?(\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d\.\d+)Z',l)] if m]
    secs=(ts[-1]-ts[0]).total_seconds() if ts else 1e9
    ok['the job within 10 of its 15 minutes: %.0f s'%secs]=secs<=600
    print(path.split('/')[-1]); 
    for k,vv in ok.items(): print('  ','yes' if vv else 'NO ',k)
    print('   ALL' if all(ok.values()) else '   NOT ALL')
    if i>=0: print('   the install lines:',*L[i:i+4],sep='\n      ')
    w=[l for l in L if re.search(r'sim_wasm_bg\.wasm',l) and (re.match(r'^[0-9a-f]{64}',l) or 'pkg/sim_wasm_bg.wasm  ' in l)]
    print('   wasm:',*[x.strip() for x in w[:3]],sep='\n      ')
