import re,sys
inp=open('/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/inputs/STOP-ANSWERS-mustafa-2026-11-03.md').read()
norm=lambda s:re.sub(r'\s+',' ',s)
inpn=norm(inp)
for f in sys.argv[1:]:
    t=open(f).read()
    lines=t.split('\n')
    tn=norm(t)
    # find quotes with line numbers: search across normalized text, map back approx
    for m in re.finditer(r'"([^"]{8,}?)"',t,re.S):
        q=norm(m.group(1))
        ln=t[:m.start()].count('\n')+1
        ok=q in inpn
        if not ok and q.rstrip('.') in inpn: ok='ok-stripdot'
        print(f"{f.split('/')[-1]}:{ln}\t{ok}\t{q[:110]}")
