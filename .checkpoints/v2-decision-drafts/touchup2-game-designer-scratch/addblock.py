import sys
P='/home/user/Shamble-Frontlines/docs/adr/0005-content-abilities-assets.md'
OUT='/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/touchup2-game-designer-fixes.md'
DOCREL='docs/adr/0005-content-abilities-assets.md'
L=open(P,encoding='utf-8').read().split('\n')
def before(a,b):
    """lines a..b (1-based, inclusive), copied from disk"""
    seg=L[a-1:b]
    text='\n'.join(seg)
    full='\n'.join(L)
    # whole-line occurrence count
    n=0; i=0
    hay='\n'+full+'\n'; needle='\n'+text+'\n'
    start=0
    while True:
        k=hay.find(needle,start)
        if k<0: break
        n+=1; start=k+1
    assert n==1, ('BEFORE occurs', n)
    return seg
def add(kind_id, seg, after, check_len=120):
    for x in after:
        assert len(x)<=check_len, (len(x), x)
    with open(OUT,'a',encoding='utf-8') as f:
        f.write('\n### FIX '+kind_id+'\nFILE: '+DOCREL+'\nBEFORE:\n'+'\n'.join(seg)+'\nAFTER:\n'+'\n'.join(after)+'\nEND\n')
    print('appended', kind_id, 'lines', len(seg), '->', len(after), 'delta', len(after)-len(seg))
