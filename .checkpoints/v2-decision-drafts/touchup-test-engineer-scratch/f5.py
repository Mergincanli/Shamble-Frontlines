import textwrap
doc='docs/adr/0003-rendering.md'
L=open(doc,encoding='utf8').read().split('\n'); full='\n'.join(L)
def blk(tag,a,b,old,new,ind):
    before='\n'.join(L[a-1:b]); j=' '.join(l.strip() for l in before.split('\n'))
    assert full.count(before)==1 and j.count(old)==1, tag
    lead=L[a-1][:len(L[a-1])-len(L[a-1].lstrip())]; txt=j.replace(old,new)
    w=textwrap.wrap(txt,120,subsequent_indent=' '*ind,break_long_words=False,break_on_hyphens=False)
    assert len(w)==b-a+1, (tag,len(w),b-a+1)
    print(f'### FIX F5 (fixcheck-d2, {tag})\nFILE: {doc}\nBEFORE:\n{before}\nAFTER:\n'+'\n'.join(w)+'\nEND\n')
assert L[1235].startswith('4. The two ink') and L[1292].startswith('27. One big') and L[1315].startswith('32. The busy')
blk('look step 4',1236,1240,
 "inside; the ink at the base of buildings, and where soldiers stand, is as heavy as their silhouette (5.b names this contact line as a visible difference between the families). Pick: A or B, and whether the buildings' ink is heavy enough.",
 "inside. Look at: the ink where buildings and soldiers meet the ground (5.b names this contact line as a visible difference between the families). Pick: A or B, and whether the buildings' ink and that contact line are heavy enough.",3)
blk('look step 27',1293,1296,
 "Expected: comic impact touches, used sparingly; the dotted HUD still reads. Pick:",
 "Expected: comic impact touches, used sparingly. Record: whether the dotted HUD still reads. Pick:",4)
blk('look step 32',1316,1320,
 "Expected: his own soldier, the three factions and a thrown grenade read at a glance; units, faction colours and effects stay the brightest things on screen, and the glow and the effects do not wash out the units, at day and at dusk. Record: achieved or not yet, for each mood;",
 "Expected: units, faction colours and effects are the brightest things on screen (§6.1), at day and at dusk. Record: achieved or not yet, for each mood; his notes on how his own soldier, the factions and a thrown grenade stand out, and any glow that hides units;",4)
