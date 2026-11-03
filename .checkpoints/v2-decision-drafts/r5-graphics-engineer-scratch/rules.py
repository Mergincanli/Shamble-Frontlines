#!/usr/bin/env python3
"""Rules on the applied tree vs base: changed lines' length (prose <= 120), PM rows' cells (7), PM total,
sentence directly under a table, quotes attributed to Mustafa found verbatim in the inputs, label greps."""
import difflib, re, pathlib, glob
S = pathlib.Path(__file__).parent
P = S.parent
inputs = ' '.join(re.sub(r'\s+', ' ', (P / 'inputs' / f).read_text(encoding='utf8')) for f in (
    'STOP-ANSWERS-mustafa-2026-11-03.md', 'DESIGN-ANSWERS-mustafa-2026-11-03.md', 'ANSWERS-2-mustafa-2026-11-03.md'))
tot = 0
for f in sorted(glob.glob(str(S / 'tree/docs/adr/000[1-6]*.md'))):
    name = pathlib.Path(f).name
    a = (S / 'base/docs/adr' / name).read_text(encoding='utf8').split('\n')
    b = pathlib.Path(f).read_text(encoding='utf8').split('\n')
    pm = [l for l in b if l.startswith('| PM-')]
    tot += len(pm)
    bad = [l[:20] for l in pm if len(l.strip().strip('|').split(' | ')) != 7]
    sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
    changed = [j for op, i1, i2, j1, j2 in sm.get_opcodes() if op != 'equal' for j in range(j1, j2)]
    long_ = [(j + 1, len(b[j])) for j in changed if not b[j].startswith('|') and len(b[j]) > 120]
    under = [j + 1 for j in range(1, len(b)) if b[j - 1].startswith('|') and b[j].strip() and not b[j].startswith('|')]
    under_changed = [j for j in under if j - 1 in changed or j - 2 in changed]
    print(f'{name[:4]}: PM rows {len(pm)} (base {sum(1 for l in a if l.startswith("| PM-"))}); bad cells {bad}; '
          f'changed lines {len(changed)}; long prose {long_}; sentence under a table near a change {under_changed}')
    # quotes in changed lines that sit beside a Mustafa attribution
    text = '\n'.join(b[j] for j in changed)
    blob = re.sub(r'\s+', ' ', text)
    for q in re.findall(r'"([^"]{8,})"', blob):
        print('   quote', 'FOUND' if q in inputs else 'not in inputs', '::', q[:90])
print('PM total', tot)
