import re, sys
root, fixes = sys.argv[1], open(sys.argv[2], encoding='utf8').read()
n = 0
for m in re.finditer(r'^### FIX (.*?)\nFILE: (.*?)\nBEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND$', fixes, re.S | re.M):
    fid, path, before, after = m.groups()
    p = f'{root}/{path}'
    t = open(p, encoding='utf8').read()
    c = t.count(before)
    if c != 1:
        print('NOT UNIQUE/ABSENT', c, fid)
        continue
    open(p, 'w', encoding='utf8').write(t.replace(before, after))
    n += 1
print('applied', n)
