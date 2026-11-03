import re,sys
FX='/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/touchup2-game-designer-fixes.md'
SRC='/home/user/Shamble-Frontlines/docs/adr/0005-content-abilities-assets.md'
txt=open(FX,encoding='utf-8').read()
blocks=re.findall(r'^### FIX (\S+)\nFILE: (\S+)\nBEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND$',txt,re.S|re.M)
doc=open(SRC,encoding='utf-8').read()
n0=doc.count('\n')
for bid,f,b,a in blocks:
    hay='\n'+doc+'\n'; nb='\n'+b+'\n'
    c=hay.count(nb); assert c==1,(bid,c)
    doc=hay.replace(nb,'\n'+a+'\n')[1:-1]
    assert ('\n'+doc+'\n').count('\n'+a+'\n')>=1
    print(bid,'applied, delta',a.count('\n')-b.count('\n'))
print('blocks',len(blocks),'lines before',n0,'after',doc.count('\n'),'net',doc.count('\n')-n0)
open('applied/docs/adr/0005-content-abilities-assets.md','w',encoding='utf-8').write(doc)
open('base/docs/adr/0005-content-abilities-assets.md','w',encoding='utf-8').write(open(SRC,encoding='utf-8').read())
