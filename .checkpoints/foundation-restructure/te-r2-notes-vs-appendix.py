#!/usr/bin/env python3
"""test-engineer round 2: read-only. The first fenced block of each 2026-11-11 capture note vs the report appendix's
A3.3-A3.5 fenced blocks at 76544aa (independent of the docs-keeper check, which takes the appendix as its reference)."""
import subprocess
REPO = '/home/user/Shamble-Frontlines'
N = REPO + '/.checkpoints/foundation-restructure/'
def fenced(lines):
    out, buf, inside = [], [], False
    for l in lines:
        if not inside and l.startswith('```'): inside, buf = True, []; continue
        if inside and l == '```': out.append(buf); inside = False; continue
        if inside: buf.append(l)
    return out
rpt = subprocess.run(['git', 'show', '76544aa:docs/reports/2026-11-11-foundation-2-1a.md'], cwd=REPO, capture_output=True, text=True, check=True).stdout.split('\n')
blocks, cur = {}, None
for i, l in enumerate(rpt):
    if l.startswith('### A3.3') or l.startswith('### A3.4') or l.startswith('### A3.5'): cur = l[4:8]
    elif l.startswith('### '): cur = None
    if cur and l.startswith('```') and cur not in blocks:
        blocks[cur] = fenced(rpt[i:])[0]
for key, note in [('A3.3', 'mustafa-2026-11-11-f1.md'), ('A3.4', 'mustafa-2026-11-11-prompts.md'), ('A3.5', 'mustafa-2026-11-11-prompts-correction.md')]:
    nb = fenced(open(N + note, encoding='utf-8').read().split('\n'))[0]
    print(key, note, 'lines', len(nb), len(blocks.get(key, [])), 'EQUAL' if nb == blocks.get(key) else 'DIFFER')
