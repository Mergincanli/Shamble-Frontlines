#!/usr/bin/env python3
"""List every recommendation and open-question id DEFINED in docs/research/r1..r6 (a line '- **<ID>. Title'),
plus the shared decision questions ('- **CODE (Decider). ...'). Output TSV: id, kind, file, line, title."""
import re, sys, pathlib
root = pathlib.Path(sys.argv[1])
pat = re.compile(r'^\s*(?:[-*]\s+)?\*\*(R[1-6]-[A-Z0-9]+-(?:XU-OQ|XT-OQ|REC|OQ)[0-9]+)\.?\s*(.*)')
shared = re.compile(r'^\s*[-*]\s+\*\*([A-Z][A-Z0-9]+-[A-Z0-9]+) \(([^)]+)\)\.\s*(.*)')
rows, seen = [], {}
for f in sorted(root.glob('r[1-6]-*.md')):
    for n, line in enumerate(f.read_text(encoding='utf8').split('\n'), 1):
        m = pat.match(line)
        if m:
            i = m.group(1)
            if i[1] != f.name[1]:  # a definition lives in its own topic file; a bold id of another topic is a reference
                continue
            kind = 'XU' if '-XU-OQ' in i else 'XT' if '-XT-OQ' in i else 'RR' if '-RR-' in i else 'REC' if '-REC' in i else 'OQ'
            title = re.sub(r'\*\*.*', '', m.group(2)).strip()[:90]
            key = (i, f.name)
            if key in seen:  # a second definition line in the same file: keep the first, note it
                continue
            seen[key] = n
            rows.append((i, kind, f.name, n, title))
            continue
        s = shared.match(line)
        if s and s.group(1) in ('MODES-3','PREVIEW-WALL','BASELINE-WHEN','SEC-SPREAD','ABL-PREDICT','PREV-DELETE'):
            key = (s.group(1), f.name)
            if key not in seen:
                seen[key] = n
                rows.append((s.group(1), 'SHARED', f.name, n, s.group(3)[:90]))
out = sys.stdout
out.write('id\tkind\tfile\tline\ttitle\n')
for r in rows:
    out.write('\t'.join(str(x) for x in r) + '\n')
import collections
c = collections.Counter((r[1]) for r in rows)
sys.stderr.write(f'{len(rows)} rows; by kind: {dict(c)}; unique ids: {len(set(r[0] for r in rows))}\n')
dups = collections.Counter(r[0] for r in rows if r[1] != 'SHARED')
d2 = [k for k, v in dups.items() if v > 1]
sys.stderr.write(f'ids defined in more than one file: {len(d2)} {d2[:10]}\n')
