#!/usr/bin/env python3
"""Every quoted span in the AFTER texts of the fixes file: found verbatim (whitespace normalized) in one of the three
inputs files of 2026-11-03, or listed as 'not in inputs' for a manual reading (titles, labels, other docs' words)."""
import re, pathlib
B = pathlib.Path('/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/')
inp = ' '.join(re.sub(r'\s+', ' ', (B / 'inputs' / f).read_text(encoding='utf8')) for f in
               ['STOP-ANSWERS-mustafa-2026-11-03.md', 'DESIGN-ANSWERS-mustafa-2026-11-03.md', 'ANSWERS-2-mustafa-2026-11-03.md', 'HETZNER-CHECK-mustafa-2026-11-03.md', 'HETZNER-CORRECTION-mustafa-2026-11-03.md', 'HETZNER-CORRECTION-2-mustafa-2026-11-03.md', 'HETZNER-ANSWERS-2-mustafa-2026-11-03.md', 'LG-COUNT-mustafa-2026-11-03.md', 'ANSWERS-3-mustafa-2026-11-03.md'])
src = (B / 'r6b-engine-architect-b-fixes.md').read_text(encoding='utf8')
afters = re.findall(r'^### FIX (\S+)\n.*?\nAFTER:\n(.*?)\nEND$', src, re.S | re.M)
befores = dict(re.findall(r'^### FIX (\S+)\n.*?\nBEFORE:\n(.*?)\nAFTER:', src, re.S | re.M))
found = miss = 0
for fid, a in afters:
    a = re.sub(r'\s+', ' ', a)
    qs = re.findall(r'“([^”]*)”', a)
    rest = re.sub(r'“[^”]*”', ' ', a)
    qs += re.findall(r'"([^"]+)"', rest)
    old = re.sub(r'\s+', ' ', befores[fid])
    for q in qs:
        if q.strip() in inp:
            found += 1
        elif q in old:
            pass  # unchanged text from the doc
        else:
            miss += 1; print(f'{fid}: not in inputs (new in AFTER): "{q}"')
print(f'quotes found in inputs: {found}; new quotes not in inputs: {miss}')
