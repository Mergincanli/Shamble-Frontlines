# Word diff of each touch-up 4 block: BEFORE vs AFTER, as word sequences (whitespace-insensitive).
import re, difflib
D = '/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/'
files = ['touchup4-engine-architect-fixes.md', 'touchup4-rust-reviewer-fixes.md', 'touchup4-test-engineer-fixes.md']
for f in files:
    t = open(D + f).read()
    for m in re.finditer(r'^### FIX (\S+)\nFILE: (\S+)\nBEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND$', t, re.S | re.M):
        bid, b, a = m.group(1), m.group(3), m.group(4)
        bw, aw = b.split(), a.split()
        sm = difflib.SequenceMatcher(a=bw, b=aw, autojunk=False)
        ops = [(tag, ' '.join(bw[i1:i2]), ' '.join(aw[j1:j2])) for tag, i1, i2, j1, j2 in sm.get_opcodes() if tag != 'equal']
        bl, al = b.split('\n'), a.split('\n')
        print(f'== {bid}: lines {len(bl)} -> {len(al)}; max len {max(map(len, bl))} -> {max(map(len, al))}; word changes {len(ops)}')
        for tag, x, y in ops:
            print(f'   {tag}: [{x}] -> [{y}]')
