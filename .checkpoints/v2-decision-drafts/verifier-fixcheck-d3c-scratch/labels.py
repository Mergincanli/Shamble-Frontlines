#!/usr/bin/env python3
"""Task 1c (fixcheck-d3c): labels lost inside an item between two refs. For each item (a heading, a bold item start or
a numbered step; keyed by its first 40 characters after the marker) count its labels (What, Why, Why it matters,
Research, Research ids, Need, Options, Decider, What decides it, Carries; as a '- Label:' bullet or an inline
'*Label:*' / 'Label:'), and list every (item, label) whose count fell. Usage: labels.py <old> <new>"""
import re, subprocess, sys, collections
REPO = '/home/user/Shamble-Frontlines'; old, new = sys.argv[1], sys.argv[2]
DOCS = ['docs/adr/0001-multiplayer-netcode.md', 'docs/adr/0002-code-structure-build-toolchain.md',
        'docs/adr/0003-rendering.md', 'docs/adr/0004-simulation-core.md', 'docs/adr/0005-content-abilities-assets.md',
        'docs/adr/0006-infrastructure-operations.md', 'docs/ENGINE_OVERVIEW.md', 'docs/SPIKE_PLAN.md',
        'docs/adr/README.md']
show = lambda r, p: subprocess.run(['git', '-C', REPO, 'show', f'{r}:{p}'], capture_output=True, text=True).stdout
LAB = re.compile(r'(?:^\s*- \*?|\*|(?<=[.;)] )|^\s+)(What decides it|Why it matters|Research ids|What|Why|Research|Need|Options|Decider|Carries)\*?:')
ITEM = re.compile(r'^(#{1,6} +|\s*- \*\*|\s*\d+\. )(.*)')
def count(t):
    c = collections.Counter(); key = '(top)'
    for l in t.split('\n'):
        m = ITEM.match(l)
        if m and not LAB.match(l): key = (m.group(1).strip() + ' ' + re.sub(r'\*', '', m.group(2))[:40]).strip()
        for mm in LAB.finditer(l): c[(key, mm.group(1))] += 1
    return c
tot = 0
for d in DOCS:
    a, b = count(show(old, d)), count(show(new, d))
    fell = [(k, a[k], b[k]) for k in a if b[k] < a[k]]
    # an item renamed on purpose moves its labels to a new key: show the keys that gained labels, for pairing by hand
    gained = [(k, a[k], b[k]) for k in b if b[k] > a[k]]
    print(f'=== {d}: labels {sum(a.values())} -> {sum(b.values())}; (item, label) pairs that fell: {len(fell)}')
    for k, x, y in fell: print(f'  FELL {x}->{y}  {k[1]:<16} in {k[0][:90]}')
    if fell:
        for k, x, y in gained: print(f'  gained {x}->{y}  {k[1]:<16} in {k[0][:90]}')
    tot += len(fell)
print('TOTAL (item, label) pairs that fell:', tot)
