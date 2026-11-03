#!/usr/bin/env python3
"""Task 1b (fixcheck-d3c): structural lines that occur twice where they should occur once, at a ref, in the nine
documents: (1) headings with the same text; (2) bold item ids defined twice (AB-n, CD-n, OQ-..., PM-..., etc.);
(3) table first cells repeated inside one table, and PM ids in more than one row of a document; (4) numbered lists
whose numbers repeat, skip or do not start at 1; (5) a label (Need, What, Why, Research) twice in one top-level item.
Usage: doubled.py <ref>"""
import re, subprocess, sys, collections
REPO = '/home/user/Shamble-Frontlines'
DOCS = ['docs/adr/0001-multiplayer-netcode.md', 'docs/adr/0002-code-structure-build-toolchain.md',
        'docs/adr/0003-rendering.md', 'docs/adr/0004-simulation-core.md', 'docs/adr/0005-content-abilities-assets.md',
        'docs/adr/0006-infrastructure-operations.md', 'docs/ENGINE_OVERVIEW.md', 'docs/SPIKE_PLAN.md',
        'docs/adr/README.md']
ref = sys.argv[1]
show = lambda p: subprocess.run(['git', '-C', REPO, 'show', f'{ref}:{p}'], capture_output=True, text=True).stdout
ind = lambda l: len(l) - len(l.lstrip(' '))
ID = re.compile(r'^(OQ-\d{4}-[A-Z]*\d+|OQ-SPIKE-\d+|PM-\d{4}-\d+|[A-Z]{1,4}-\d+[a-z]?|[A-Z][0-9]+[a-z]?|Part \d+)\b')
total = 0
for d in DOCS:
    L = show(d).split('\n'); out = []
    # (1) headings
    hs = collections.defaultdict(list)
    for i, l in enumerate(L, 1):
        m = re.match(r'^(#{1,6}) +(.*)$', l)
        if m: hs[m.group(1) + ' ' + m.group(2).strip()].append(i)
    for h, ls in hs.items():
        if len(ls) > 1: out.append(f'(1) heading x{len(ls)} at lines {ls}: {h[:90]}')
    # (2) bold item ids
    ids = collections.defaultdict(list)
    for i, l in enumerate(L, 1):
        m = re.match(r'^\s*(?:- |\d+\. )?\*\*(.*)', l)
        if m:
            mm = ID.match(m.group(1))
            if mm: ids[(ind(l), mm.group(1))].append(i)
    for (k, idv), ls in ids.items():
        if len(ls) > 1: out.append(f'(2) bold item id {idv} (indent {k}) x{len(ls)} at lines {ls}')
    # (3) tables
    tbl = []; pm = collections.defaultdict(list)
    def flush(t):
        c = collections.defaultdict(list)
        for i, cell in t: c[cell].append(i)
        for cell, ls in c.items():
            if len(ls) > 1 and cell: out.append(f'(3) table first cell x{len(ls)} in one table at lines {ls}: |{cell[:70]}|')
    for i, l in enumerate(L, 1):
        if l.startswith('|'):
            cell = l.split('|')[1].strip()
            if re.fullmatch(r':?-{3,}:?', cell): continue
            tbl.append((i, cell))
            m = re.match(r'^\| *(PM-\d{4}-\d+) *\|', l)
            if m: pm[m.group(1)].append(i)
        else:
            if tbl: flush(tbl); tbl = []
    if tbl: flush(tbl)
    for p, ls in pm.items():
        if len(ls) > 1: out.append(f'(3) PM row {p} x{len(ls)} at lines {ls}')
    # (4) numbered lists: numbers grouped by their parent (the nearest heading for indent 0, else the nearest less
    # indented non-blank line); within a group each number must be the previous + 1, or 1 (a new list)
    groups = collections.defaultdict(list); heading = 0
    for i, l in enumerate(L, 1):
        if l.startswith('#'): heading = i
        m = re.match(r'^(\s*)(\d+)\. ', l)
        if not m: continue
        k = ind(l); par = heading
        if k > 0:
            for j in range(i - 2, -1, -1):
                if L[j].strip() and ind(L[j]) < k: par = j + 1; break
        groups[(par, k)].append((i, int(m.group(2))))
    for (par, k), seq in groups.items():
        for (i0, n0), (i1, n1) in zip(seq, seq[1:]):
            if n1 == n0 + 1 or n1 == 1: continue
            out.append(f'(4) numbered list (parent line {par}, indent {k}): {n0} at line {i0} then {n1} at line {i1}')
        if seq[0][1] not in (0, 1): out.append(f'(4) numbered list (parent line {par}, indent {k}) starts at {seq[0][1]} (line {seq[0][0]})')
    # (5) a label twice in one item: the owner of a bullet label ('  - Need:') is the nearest preceding list item
    # less indented; the owner of an inline label ('*Need:*') is the list item whose text holds it
    BUL = re.compile(r'^(\s*)(- |\d+\. )')
    own = collections.defaultdict(list); heading = 0
    for i, l in enumerate(L, 1):
        if l.startswith('#'): heading = i
        for m in re.finditer(r'(^\s*- )?\*?(Need|What|Why|Research)(?::\*|\*?:)', l):
            if not (m.group(1) or l[max(0, m.start(2) - 1)] == '*'): continue
            k = ind(l); owner = heading
            if m.group(1):  # a bullet label
                for j in range(i - 2, heading - 1, -1):
                    if BUL.match(L[j]) and ind(L[j]) < k: owner = j + 1; break
            else:
                for j in range(i - 1, heading - 1, -1):
                    if BUL.match(L[j]) and (j == i - 1 or ind(L[j]) < k): owner = j + 1; break
                    if L[j].strip() == '' or L[j].startswith('|'): break
            own[(owner, m.group(2))].append(i)
    for (owner, k), ls in sorted(own.items()):
        if len(ls) > 1: out.append(f'(5) label {k} x{len(ls)} in the item at line {owner}: lines {ls}')
    print(f'=== {d} ({len(L)} lines): {len(out)} item(s)')
    for o in out: print('  ' + o)
    total += len(out)
print('TOTAL', total)
