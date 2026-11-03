#!/usr/bin/env python3
"""Task 4b (fixcheck-d3c): tables in the nine documents at a ref.
(1) No sentence directly under a table: the line after a table's last row must be blank, a heading or the end of the
    file (GFM folds a plain line straight after a table into the table as a row). Other block starts ('- ', 'n. ', '>',
    '```') end a GFM table too; they are listed separately.
(2) Every PM row has exactly 7 cells, split as GFM does (on every '|' not escaped with a backslash, code spans included),
    and every table's rows have as many cells as its header (a short or long row in any table).
Usage: tables.py <ref>"""
import re, subprocess, sys, collections
REPO = '/home/user/Shamble-Frontlines'; ref = sys.argv[1] if len(sys.argv) > 1 else 'HEAD'
DOCS = ['docs/adr/0001-multiplayer-netcode.md', 'docs/adr/0002-code-structure-build-toolchain.md',
        'docs/adr/0003-rendering.md', 'docs/adr/0004-simulation-core.md', 'docs/adr/0005-content-abilities-assets.md',
        'docs/adr/0006-infrastructure-operations.md', 'docs/ENGINE_OVERVIEW.md', 'docs/SPIKE_PLAN.md',
        'docs/adr/README.md']
show = lambda p: open('/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/touchup4-engine-architect-scratch/tree/' + p, encoding='utf8').read()
def cells(l):
    s = l.strip()
    if s.startswith('|'): s = s[1:]
    if s.endswith('|') and not s.endswith('\\|'): s = s[:-1]
    return re.split(r'(?<!\\)\|', s)
glued = []; blockafter = []; pmbad = []; widthbad = []; tables = 0; pmrows = 0; rows = 0
for d in DOCS:
    L = show(d).split('\n'); i = 0; inside_fence = False
    while i < len(L):
        if L[i].startswith('```'): inside_fence = not inside_fence
        if not inside_fence and L[i].startswith('|') and i + 1 < len(L) and re.match(r'^\|\s*:?-{3,}', L[i + 1]):
            tables += 1; start = i; ncol = len(cells(L[i])); j = i
            while j < len(L) and L[j].startswith('|'):
                c = cells(L[j]); rows += 1
                if re.match(r'^\| *PM-\d{4}-\d+ *\|', L[j]):
                    pmrows += 1
                    if len(c) != 7: pmbad.append(f'{d.split("/")[-1]}:{j + 1}: {len(c)} cells: {L[j][:90]}')
                if len(c) != ncol and not re.match(r'^\|\s*:?-{3,}', L[j]):
                    widthbad.append(f'{d.split("/")[-1]}:{j + 1}: {len(c)} cells, header {ncol} (table at line {start + 1}): {L[j][:80]}')
                j += 1
            nxt = L[j] if j < len(L) else ''
            if nxt.strip() and not nxt.startswith('#'):
                tgt = blockafter if re.match(r'^\s*(- |\* |\d+\. |>|```)', nxt) else glued
                tgt.append(f'{d.split("/")[-1]}:{j + 1}: after the table at line {start + 1}: {nxt[:100]}')
            i = j; continue
        i += 1
print(f'tables {tables}, rows {rows}, PM rows {pmrows}')
print(f'(1) sentences directly under a table: {len(glued)}'); [print('   ' + x) for x in glued]
print(f'    other blocks directly under a table (they end a GFM table): {len(blockafter)}'); [print('   ' + x) for x in blockafter]
print(f'(2) PM rows without exactly 7 cells: {len(pmbad)}'); [print('   ' + x) for x in pmbad]
print(f'    rows whose cell count differs from their header (any table): {len(widthbad)}'); [print('   ' + x) for x in widthbad]
