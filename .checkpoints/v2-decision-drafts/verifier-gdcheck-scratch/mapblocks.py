import re,glob
removed=['  - Research: R1-ABL-REC1 (lines 2350–2352), R1-ABL-REC2 (lines 2384–2385); R1-ABL-F9.',
'  - Need: VISION_V2 §6.3 ("combos and counters can emerge").',
'  - Research: R1-ABL-REC1 (lines 2343–2345).',
'  - Need: VISION_V2 §6.5.',
"  - Need: VISION_V2 §6.3, §6.4; the rows' rules: LOOP_V2 §4–§7.",
'  - Decider: Mustafa. What decides it: a design session, then the two measurements of OQ-0005-AB3. Research ids: none.',
'  - Why it matters: a live fight has no pause. The open offer is state on the player',
'  - Decider: Mustafa, with the game-designer. What decides it: a design session, with OQ-0005-AB1. Research ids: none;',
'  - Decider: Mustafa. What decides it: a design session, then sims with mixed humans and bots. Research ids: none.']
for f in ['game-designer-fixes-final.md','rust-reviewer-fixes-final.md','game-designer-fixes-2.md']:
    src=open(f,encoding='utf8').read()
    for m in re.finditer(r'^### FIX (.+?)\n(?:```[^\n]*\n)?(?:FILE: *(\S+)\n)?BEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND\b', src, re.S|re.M):
        fid,fil,b,a=m.groups()
        for r in removed:
            if r in b:
                print(f,'|',fid[:110],'|',fil,'| removes' if r not in a else '| keeps', '|', r[:60])
