#!/usr/bin/env python3
"""Citation check for the decision drafts.
For each draft file given, find every research id cited (R1..R6 ids: REC, OQ, XU-OQ, XT-OQ, RR-OQ, F findings) and
every 'rN-<name>.md' line reference ('r2-netcode.md:2377-2387', 'lines 2377–2387' right after a file name and id).
Checks: (1) every cited id exists somewhere in docs/research (defined or at least present); (2) for a citation of the
form (file ..., ID ..., lines a–b) the file has those lines and the ID occurs within lines a-20..b+20 of that file.
Prints problems and counts. Usage: cite_check.py <research_dir> <draft.md>..."""
import re, sys, pathlib
R = pathlib.Path(sys.argv[1])
files = {p.name: p.read_text(encoding='utf8').split('\n') for p in R.glob('*.md')}
alltext = '\n'.join('\n'.join(v) for v in files.values())
idpat = re.compile(r'\bR[1-6]-[A-Z0-9]+-(?:XU-OQ|XT-OQ|RR-OQ|REC|OQ|F)[0-9]+\b|\bR[1-6]-RR-OQ[0-9]+\b')
# a citation group in parentheses: (r2-netcode.md, ..., R2-BUD-REC6, lines 2398–2420)
grp = re.compile(r'\(([^()]*?\b(r[1-6]-[a-z-]+\.md|GLOSSARY\.md)\b[^()]*?)\)')
linepat = re.compile(r'\blines?\s+(\d+)(?:\s*[–-]\s*(\d+))?')
colon = re.compile(r'\b(r[1-6]-[a-z-]+\.md|GLOSSARY\.md):(\d+)(?:\s*[–-]\s*(\d+))?')
total_ids = 0; missing = []; range_bad = []; groups = 0
for d in sys.argv[2:]:
    text = pathlib.Path(d).read_text(encoding='utf8')
    for m in idpat.finditer(text):
        total_ids += 1
        if m.group(0) not in alltext:
            missing.append((d, m.group(0)))
    for g in grp.finditer(text):
        body, fname = g.group(1), g.group(2)
        if fname not in files: 
            range_bad.append((d, fname, 'no such research file')); continue
        groups += 1
        ids = idpat.findall(body)
        for lm in linepat.finditer(body):
            a = int(lm.group(1)); b = int(lm.group(2) or a)
            L = files[fname]
            if b < a or b > len(L):
                range_bad.append((d, fname, f'lines {a}-{b} out of range (file has {len(L)})')); continue
            win = '\n'.join(L[max(0, a-21):min(len(L), b+20)])
            for i in ids:
                if i not in win:
                    range_bad.append((d, fname, f'{i} not within lines {a}-{b} (±20)'))
    for c in colon.finditer(text):
        fname, a = c.group(1), int(c.group(2)); b = int(c.group(3) or a)
        if fname in files and (b < a or b > len(files[fname])):
            range_bad.append((d, fname, f':{a}-{b} out of range'))
print(f'ids cited: {total_ids}; citation groups with a research file: {groups}')
print(f'ids not found anywhere in docs/research: {len(missing)}')
for x in missing[:50]: print('  MISSING', x)
print(f'line-range problems: {len(range_bad)}')
for x in range_bad[:80]: print('  RANGE', x)
sys.exit(1 if missing or range_bad else 0)
