import re,glob
tot=0
for f in sorted(glob.glob('/home/user/Shamble-Frontlines/docs/adr/000[1-6]-*.md')):
    n=re.search(r'/000(\d)-',f).group(1)
    rows=[l for l in open(f) if re.match(r'^\| *PM-000'+n+r'-\d+ *\|',l)]
    ids=[int(re.match(r'^\| *PM-000'+n+r'-(\d+)',l).group(1)) for l in rows]
    cells=[len(re.split(r'(?<!\\)\|',l.rstrip('\n')))-2 for l in rows]
    bad=[ids[i] for i,c in enumerate(cells) if c!=7]
    print(f.split('/')[-1],len(ids),'contiguous',sorted(ids)==list(range(1,len(ids)+1)),'dups',len(ids)-len(set(ids)),'non7',bad)
    tot+=len(ids)
print(tot)
