#!/usr/bin/env python3
"""Append one FIX block to r5-engine-architect-b-fixes.md. BEFORE = lines s..e (1-based, inclusive) of the doc ON DISK
(the repository's docs, never the scratch tree). AFTER = those lines with the (old, new) pairs of a JSON file applied,
each old occurring exactly once in the joined text.
Modes: row  = table rows, no wrapping; every AFTER row keeps the BEFORE row's cell count (extra AFTER rows allowed via
              a pair whose new text adds '\n| ...' rows);
       wrap = prose lines: continuation indents stripped, pairs applied, rewrapped at 120 with the span's own indents.
Checks: BEFORE occurs exactly once in the doc; AFTER prose lines <= 120; table cell counts. Prints the AFTER.
Usage: mk.py <id> <docs/path> <s> <e> <row|wrap> <pairs.json> [--print-only]"""
import sys, json, pathlib, textwrap
REPO = pathlib.Path('/home/user/Shamble-Frontlines')
OUT = REPO / '.checkpoints/v2-decision-drafts/r5-engine-architect-b-fixes.md'
fid, path, s, e, mode, pj = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), sys.argv[5], sys.argv[6]
printonly = '--print-only' in sys.argv
doc = (REPO / path).read_text(encoding='utf8')
lines = doc.split('\n')
span = lines[s - 1:e]
before = '\n'.join(span)
n = doc.count(before)
if n != 1:
    sys.exit(f'BEFORE occurs {n} times in {path}: not written')
pairs = json.load(open(pj, encoding='utf8'))


def cells(l):
    return l.strip().count('|') - 1


if mode == 'row':
    text = before
    for old, new in pairs:
        k = text.count(old)
        if k != 1:
            sys.exit(f'old occurs {k} times: {old[:70]}')
        text = text.replace(old, new)
    after = text
    want = {cells(l) for l in span if l.startswith('|')}
    got = [cells(l) for l in after.split('\n') if l.startswith('|')]
    if len(want) != 1 or any(g not in want for g in got):
        sys.exit(f'cell counts differ: before {want}, after {got}')
elif mode == 'wrap':
    first = span[0]
    ind0 = first[:len(first) - len(first.lstrip(' '))]
    if len(span) > 1:
        ind1 = span[1][:len(span[1]) - len(span[1].lstrip(' '))]
    else:
        ind1 = ind0 + ('  ' if first.lstrip(' ').startswith('- ') else '')
    text = ' '.join([first.strip()] + [l.strip() for l in span[1:]])
    for old, new in pairs:
        k = text.count(old)
        if k != 1:
            sys.exit(f'old occurs {k} times: {old[:70]}')
        text = text.replace(old, new)
    after = textwrap.fill(text, width=120, initial_indent=ind0, subsequent_indent=ind1, break_long_words=False,
                          break_on_hyphens=False)
else:
    sys.exit('mode must be row or wrap')
long_ = [len(l) for l in after.split('\n') if len(l) > 120 and not l.startswith('|')]
if long_:
    sys.exit(f'AFTER has prose line(s) over 120: {long_}')
print(after)
if not printonly:
    with OUT.open('a', encoding='utf8') as f:
        f.write(f'### FIX {fid}\nFILE: {path}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n')
    print(f'--- written FIX {fid}: {path}:{s}-{e} ({e - s + 1} line(s) -> {len(after.splitlines())})')
