#!/usr/bin/env python3
"""Task 1 (fixcheck-d3c): structural lines lost or changed between two refs, in the nine v2 decision documents,
each attributed to the fix block(s) whose BEFORE holds it; plus doubled structural lines at the new ref.
Structural lines: headings; bold item starts (a line whose first non-space text, after an optional '- ' or 'n. ', is
'**'); numbered steps; table rows (first cell); and, for the doubling check, sub-item labels ('  - Need:' etc.).
Usage: structdiff.py <oldref> <newref>   (run from anywhere; reads git objects; writes to stdout)"""
import re, subprocess, sys, glob, difflib, collections, os
REPO = '/home/user/Shamble-Frontlines'
FIXDIR = REPO + '/.checkpoints/v2-decision-drafts'
DOCS = ['docs/adr/0001-multiplayer-netcode.md', 'docs/adr/0002-code-structure-build-toolchain.md',
        'docs/adr/0003-rendering.md', 'docs/adr/0004-simulation-core.md', 'docs/adr/0005-content-abilities-assets.md',
        'docs/adr/0006-infrastructure-operations.md', 'docs/ENGINE_OVERVIEW.md', 'docs/SPIKE_PLAN.md',
        'docs/adr/README.md']
old, new = sys.argv[1], sys.argv[2]
def show(ref, p):
    r = subprocess.run(['git', '-C', REPO, 'show', f'{ref}:{p}'], capture_output=True, text=True)
    return r.stdout
H = re.compile(r'^(#{1,6}) +(.*)$')
B = re.compile(r'^(\s*)(?:- |\d+\. )?\*\*(.*)$')
N = re.compile(r'^(\s*)(\d+)\. +(.*)$')
T = re.compile(r'^\|([^|]*)\|')
LBL = re.compile(r'^(\s+)- (What|Why|Research|Need|Options|Proposed|Owner|Pass|Who|How|Cost|Risk)\b[^:]{0,20}:')
def struct(text):
    out = []  # (kind, key, lineno, line)
    for i, l in enumerate(text.split('\n'), 1):
        m = H.match(l)
        if m: out.append(('H', m.group(1) + ' ' + m.group(2).strip(), i, l)); continue
        if T.match(l):
            c = T.match(l).group(1).strip()
            if re.fullmatch(r':?-{3,}:?', c): continue
            out.append(('T', '|' + c[:60] + '|', i, l)); continue
        m = N.match(l)
        if m: out.append(('N', m.group(1) + m.group(2) + '. ' + m.group(3)[:32], i, l)); continue
        m = B.match(l)
        if m:
            body = m.group(2); body = body.split('**')[0] if '**' in body else body
            out.append(('B', m.group(1) + '**' + body[:32], i, l)); continue
    return out
def blocks():
    res = []
    for ff in sorted(glob.glob(FIXDIR + '/*fixes*.md')):
        src = open(ff, encoding='utf-8').read()
        for m in re.finditer(r'^### FIX (.+?)\n(?:```[^\n]*\n)?(?:FILE: *(\S+)\n)?BEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND\b',
                             src, re.S | re.M):
            res.append((os.path.basename(ff), m.group(1), (m.group(2) or '').split('Shamble-Frontlines/')[-1].lstrip('/'),
                        m.group(3).split('\n'), m.group(4).split('\n'), src[:m.start()].count('\n') + 1))
    return res
BL = blocks()
def find_blocks(doc, line):
    s = line.strip(); hits = []
    for ff, fid, fil, bef, aft, ln in BL:
        if fil and fil != doc: continue
        if any(x.strip() == s for x in bef):
            kept = any(x.strip() == s for x in aft)
            hits.append((ff, ln, fid[:90], 'AFTER keeps it' if kept else 'AFTER drops it'))
    return hits
tot_old = tot_lost = tot_changed = 0
for d in DOCS:
    a, b = struct(show(old, d)), struct(show(new, d))
    tot_old += len(a)
    ka, kb = [x[0] + x[1] for x in a], [x[0] + x[1] for x in b]
    sm = difflib.SequenceMatcher(None, ka, kb, autojunk=False)
    print(f'\n=== {d}: {len(a)} structural lines at {old}, {len(b)} at {new}')
    for op, i1, i2, j1, j2 in sm.get_opcodes():
        if op == 'equal': continue
        if op == 'insert':
            for x in b[j1:j2]: print(f'  ADDED  {new}:{x[2]}: {x[3].strip()[:120]}')
            continue
        for x in a[i1:i2]:
            # does the same key still exist somewhere at new (moved)?
            moved = (x[0] + x[1]) in kb
            tag = 'MOVED ' if moved else ('CHANGED' if op == 'replace' else 'LOST  ')
            if not moved:
                if op == 'replace': tot_changed += 1
                else: tot_lost += 1
            print(f'  {tag} {old}:{x[2]}: {x[3].strip()[:120]}')
            if not moved:
                for h in find_blocks(d, x[3]): print(f'           <- {h[0]}:{h[1]} FIX {h[2]} [{h[3]}]')
        if op == 'replace':
            for x in b[j1:j2]: print(f'    now  {new}:{x[2]}: {x[3].strip()[:120]}')
print(f'\nTOTAL structural lines at {old}: {tot_old}; without the same key at {new}: lost {tot_lost}, changed {tot_changed}')
