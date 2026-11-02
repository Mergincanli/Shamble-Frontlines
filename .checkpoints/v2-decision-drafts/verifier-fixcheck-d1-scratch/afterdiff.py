import re,sys,pathlib,subprocess,difflib
repo='/home/user/Shamble-Frontlines'
docs={'0001':'docs/adr/0001-multiplayer-netcode.md','0002':'docs/adr/0002-code-structure-build-toolchain.md','0006':'docs/adr/0006-infrastructure-operations.md'}
head={k:subprocess.run(['git','-C',repo,'show','604a497:'+v],capture_output=True,text=True).stdout for k,v in docs.items()}
for f in sys.argv[1:]:
    src=pathlib.Path(f).read_text()
    for m in re.finditer(r'^### FIX (.+?)\n(?:```[^\n]*\n)?(?:FILE: *(\S+)\n)?BEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND\b',src,re.S|re.M):
        fid,fil,before,after=m.groups()
        k=next((k for k,v in docs.items() if fil and fil.endswith(v)),None)
        if not k or head[k].count(after)==1: continue
        hl=head[k].split('\n'); al=after.split('\n')
        # locate: line index in head maximizing matches of first and last lines
        sm=difflib.SequenceMatcher(None,hl,al,autojunk=False)
        blocks=[b for b in sm.get_matching_blocks() if b.size>0]
        if not blocks: print('NOFIND',fid); continue
        s=max(0,blocks[0].a-blocks[0].b); e=min(len(hl),blocks[-1].a+(len(al)-blocks[-1].b))
        aw=' '.join(al).split(); hw=' '.join(hl[s:e]).split()
        print(f'=== {f} :: {fid[:80]} (HEAD {k} lines {s+1}-{e})')
        sm2=difflib.SequenceMatcher(None,aw,hw,autojunk=False)
        for op,a1,a2,b1,b2 in sm2.get_opcodes():
            if op!='equal':
                print(f'  {op}: AFTER[{" ".join(aw[max(0,a1-4):a2+2])}] -> HEAD[{" ".join(hw[max(0,b1-4):b2+2])}]')
