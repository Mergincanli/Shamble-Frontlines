import re,sys,pathlib,subprocess
repo='/home/user/Shamble-Frontlines'
docpath=sys.argv[1]; files=sys.argv[2:]
doc=subprocess.run(['git','-C',repo,'show','47eadd9:'+docpath],capture_output=True,text=True).stdout
for f in files:
    src=pathlib.Path(f).read_text()
    for m in re.finditer(r'^### FIX (.+?)\n(?:```[^\n]*\n)?(?:FILE: *(\S+)\n)?BEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND\b',src,re.S|re.M):
        fid,fil,before,after=m.groups()
        if not fil or not fil.endswith(docpath): continue
        before=re.sub(r'\n```\s*$','',before); after=re.sub(r'\n```\s*$','',after)
        n=doc.count(before)
        if n!=1: print(f'# FAIL {f} {fid[:60]} x{n}',file=sys.stderr); continue
        doc=doc.replace(before,after,1)
sys.stdout.write(doc)
