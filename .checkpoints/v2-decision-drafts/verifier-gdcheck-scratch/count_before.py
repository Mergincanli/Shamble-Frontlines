import re,sys,pathlib
src=pathlib.Path(sys.argv[1]).read_text()
doc=pathlib.Path(sys.argv[2]).read_text()
lines=doc.split('\n')
for m in re.finditer(r'^### FIX (.+?)\n(?:```[^\n]*\n)?(?:FILE: *(\S+)\n)?BEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND\b', src, re.S|re.M):
    fid,fil,b,a=m.groups()
    n=doc.count(b); idx=doc.find(b); ln=doc[:idx].count('\n')+1
    print(fid, 'count', n, 'starts at line', ln, 'ends', ln+len(b.split('\n'))-1, 'after-in-orig', doc.count(a))
