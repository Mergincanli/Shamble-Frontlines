#!/usr/bin/env python3
"""Checks on tree/ (my blocks applied) against base/ (disk copy) for ADR 0006: changed prose lines <= 120; table rows
keep their cell count; no sentence directly under a table near a change; PM rows unchanged; every quote in changed
lines found verbatim in the inputs (whitespace normalised); labels for OQ-0006-15/-20; OQ-0006 entry count."""
import difflib, re, pathlib
S = pathlib.Path(__file__).parent; P = S.parent
name = 'docs/adr/0006-infrastructure-operations.md'
a = (S / 'base' / name).read_text(encoding='utf8').split('\n'); b = (S / 'tree' / name).read_text(encoding='utf8').split('\n')
inputs = ' '.join(re.sub(r'\s+', ' ', (P / 'inputs' / f).read_text(encoding='utf8')) for f in (
    'STOP-ANSWERS-mustafa-2026-11-03.md', 'DESIGN-ANSWERS-mustafa-2026-11-03.md', 'ANSWERS-2-mustafa-2026-11-03.md'))
sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
changed = [j for op, i1, i2, j1, j2 in sm.get_opcodes() if op != 'equal' for j in range(j1, j2)]
istab = lambda l: l.lstrip().startswith('|')
print('changed lines', len(changed))
print('long prose', [(j + 1, len(b[j])) for j in changed if not istab(b[j]) and len(b[j]) > 120])
# cell counts of every table row, base vs tree, matched by row id
def cells(lines): return {l.strip().split('|')[1].strip(): l.count('|') for l in lines if istab(l) and l.strip().split('|')[1].strip()}
ca, cb = cells(a), cells(b)
print('cell count changes', [(k, ca[k], cb[k]) for k in cb if k in ca and ca[k] != cb[k]])
under = [j + 1 for j in range(1, len(b)) if istab(b[j - 1]) and b[j].strip() and not istab(b[j])]
print('sentence under a table near a change', [j for j in under if (j - 1) in changed or (j - 2) in changed])
print('PM rows', sum(1 for l in a if l.startswith('| PM-')), '->', sum(1 for l in b if l.startswith('| PM-')),
      '; PM rows changed', sum(1 for j in changed if b[j].startswith('| PM-')))
blob = re.sub(r'\s+', ' ', '\n'.join(b[j] for j in changed))
for q in re.findall(r'"([^"]{6,})"', blob):
    print('  quote', 'FOUND' if q in inputs else 'NOT in inputs', '::', q[:100])
txt = re.sub(r'\s+', ' ', '\n'.join(b))
for oq in ('OQ-0006-15', 'OQ-0006-20'):
    print(oq, 'with "answered in part" nearby:', [m.start() for m in re.finditer(oq + r'[^.;]{0,40}answered in part', txt)])
ents = re.findall(r'^- \*\*OQ-0006-(\d+)\.', '\n'.join(b), re.M)
print('OQ-0006 entries', len(ents), ents[-3:])
print('answered in part entries:', [e for e in re.findall(r'^- \*\*(OQ-0006-\d+)\.[^\n]*\n(?:[^\n]*\n){0,2}?  - (Answered[^:(]*)', '\n'.join(b), re.M)])
print('reading marks', txt.count("the advisor's reading, 2026-11-03"), '(base', re.sub(r'\s+', ' ', '\n'.join(a)).count("the advisor's reading, 2026-11-03"), ')')
