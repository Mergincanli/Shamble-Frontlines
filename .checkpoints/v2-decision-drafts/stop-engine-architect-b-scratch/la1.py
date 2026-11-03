import sys
R='/home/user/Shamble-Frontlines/docs/adr/README.md'
L=open(R,encoding='utf8').read().split('\n')
M='; LATER: the design session after the netcode proof test'
special={77:"; LATER, keeping its own timing: decided in the look proof test (the advisor's reading, 2026-11-03)",
 125:"; LATER, keeping its own timing: before the foundation's client-host token, and until then the preview hosting check runs under ADR 0006 part 13's item B9 interim rule, every deploy waiting for Mustafa's approval (the advisor's reading, 2026-11-03)",
 130:"; LATER, keeping its own timing: if the first model comes before the design session, it goes to Mustafa then (the advisor's reading, 2026-11-03)"}
a,b=int(sys.argv[1]),int(sys.argv[2])
out=[]
for n in range(a,b+1):
    cells=L[n-1].split(' | ')
    # the decider cell is the second last cell
    assert len(cells)==4, (n,len(cells))
    cells[2]=cells[2]+special.get(n,M)
    out.append(' | '.join(cells))
print('\n'.join(out))
