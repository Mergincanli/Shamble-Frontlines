#!/usr/bin/env python3
"""Index of R1–R6 findings: defined finding ids, and which ones each topic's verifier section checked, with verdicts.
Output TSV: finding_id, topic, defined_line, checked (yes/no), verdicts seen (in the verifier section's table rows)."""
import re, sys, pathlib, collections
root = pathlib.Path(sys.argv[1])
fid = re.compile(r'\b(R[1-6]-[A-Z0-9]+-F[0-9]+)\b')
defpat = re.compile(r'^\s*(?:[-*]\s+)?\*\*(R[1-6]-[A-Z0-9]+-F[0-9]+)\.')
out = []
for f in sorted(root.glob('r[1-6]-*.md')):
    lines = f.read_text(encoding='utf8').split('\n')
    start = next((i for i, l in enumerate(lines) if re.match(r'^## (?:\d+\. )?Verifier spot-check', l)), None)
    end = next((i for i, l in enumerate(lines) if i > (start or 0) and re.match(r'^## (?:\d+\. )?Sources', l)), len(lines))
    defined = {}
    for n, l in enumerate(lines, 1):
        m = defpat.match(l)
        if m and m.group(1) not in defined:
            defined[m.group(1)] = n
    checked = collections.defaultdict(set)
    if start is not None:
        for l in lines[start:end]:
            if not l.startswith('|'):
                continue
            ids = fid.findall(l.split('|')[1] if l.count('|') > 2 else l)
            low = l.lower()
            v = [w for w in ('confirmed', 'wrong', 'unsupported', 'partly') if w in low]
            for i in ids:
                checked[i].update(v or ['(no verdict word)'])
    for i, n in sorted(defined.items(), key=lambda x: x[1]):
        out.append((i, f.name, n, 'yes' if i in checked else 'no', ','.join(sorted(checked.get(i, [])))))
    extra = sorted(set(checked) - set(defined))
    sys.stderr.write(f'{f.name}: {len(defined)} findings defined; {len(set(checked) & set(defined))} checked in the verifier section; ids in the section not defined here: {len(extra)}\n')
print('finding\tfile\tline\tchecked\tverdicts')
for r in out:
    print('\t'.join(map(str, r)))
