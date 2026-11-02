import re,sys,pathlib,subprocess,difflib
repo='/home/user/Shamble-Frontlines'
docs={'0001':'docs/adr/0001-multiplayer-netcode.md','0002':'docs/adr/0002-code-structure-build-toolchain.md','0006':'docs/adr/0006-infrastructure-operations.md'}
head={k:subprocess.run(['git','-C',repo,'show','604a497:'+v],capture_output=True,text=True).stdout for k,v in docs.items()}
want=sys.argv[1]
for f in sys.argv[2:]:
    src=pathlib.Path(f).read_text()
    for m in re.finditer(r'^### FIX (.+?)\n(?:```[^\n]*\n)?(?:FILE: *(\S+)\n)?BEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND\b',src,re.S|re.M):
        fid,fil,before,after=m.groups()
        if not fid.startswith(want): continue
        k=next((k for k,v in docs.items() if fil and fil.endswith(v)),None)
        if not k or head[k].count(after)==1: continue
        hl=head[k].split('\n'); al=after.split('\n')
        for a in al:
            if a in hl: continue
            best=max(range(len(hl)),key=lambda i: difflib.SequenceMatcher(None,a[:300],hl[i][:300],autojunk=False).quick_ratio())
            aw=a.split(); hw=hl[best].split()
            print(f'=== {f} :: {fid[:80]} (HEAD {k} line {best+1})')
            for op,a1,a2,b1,b2 in difflib.SequenceMatcher(None,aw,hw,autojunk=False).get_opcodes():
                if op!='equal': print(f'  {op}: AFTER[{" ".join(aw[max(0,a1-4):a2+2])}] -> HEAD[{" ".join(hw[max(0,b1-4):b2+2])}]')
