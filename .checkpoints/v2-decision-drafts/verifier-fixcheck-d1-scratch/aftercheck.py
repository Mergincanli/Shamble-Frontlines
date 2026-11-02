import re,sys,pathlib,subprocess,difflib
repo='/home/user/Shamble-Frontlines'
docs={'0001':'docs/adr/0001-multiplayer-netcode.md','0002':'docs/adr/0002-code-structure-build-toolchain.md','0006':'docs/adr/0006-infrastructure-operations.md'}
head={k:subprocess.run(['git','-C',repo,'show','604a497:'+v],capture_output=True,text=True).stdout for k,v in docs.items()}
base={k:subprocess.run(['git','-C',repo,'show','47eadd9:'+v],capture_output=True,text=True).stdout for k,v in docs.items()}
for f in sys.argv[1:]:
    src=pathlib.Path(f).read_text()
    blocks=[(m.group(1),m.group(2),m.group(3),m.group(4)) for m in re.finditer(r'^### FIX (.+?)\n(?:```[^\n]*\n)?(?:FILE: *(\S+)\n)?BEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND\b',src,re.S|re.M)]
    for fid,fil,before,after in blocks:
        k=next((k for k,v in docs.items() if fil and fil.endswith(v)),None)
        if not k: continue
        before=re.sub(r'\n```\s*$','',before); after=re.sub(r'\n```\s*$','',after)
        n=head[k].count(after); nb=base[k].count(before)
        if n==1: continue
        # find best matching window
        hl=head[k].split('\n'); al=after.split('\n')
        best=(0,0)
        for i in range(len(hl)):
            if al[0][:40] and al[0][:40] in hl[i]: best=(i,1);break
        print(f'{f}\t{fid[:70]}\tAFTER in HEAD x{n}\tBEFORE in base x{nb}\tfirst-line at HEAD:{best[0]+1 if best[1] else "?"}')
