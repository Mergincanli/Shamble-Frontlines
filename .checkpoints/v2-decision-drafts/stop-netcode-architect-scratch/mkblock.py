#!/usr/bin/env python3
"""Append one FIX block to the fixes file. BEFORE = lines a..b (1-based, inclusive) of the ORIGINAL doc on disk, copied
by script; it must occur exactly once there. AFTER comes from a spec file whose lines are units:
  W<TAB>first_prefix<TAB>rest_prefix<TAB>text   -> text wrapped at 120 columns with those prefixes
  R<TAB>raw line                                -> kept as is (table rows, headings)
Prefixes may use the literal token {sp} for one space. Usage: mkblock.py <id> <a> <b> <spec> [--dry]"""
import sys, textwrap, pathlib
DOC = pathlib.Path('/home/user/Shamble-Frontlines/docs/adr/0001-multiplayer-netcode.md')
OUT = pathlib.Path('/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/stop-netcode-architect-fixes.md')
fid, a, b, spec = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
dry = '--dry' in sys.argv
lines = DOC.read_text(encoding='utf8').split('\n')
before = '\n'.join(lines[a - 1:b])
doc = DOC.read_text(encoding='utf8')
n = doc.count(before)
assert n == 1, f'BEFORE occurs {n} times'
out = []
for u in pathlib.Path(spec).read_text(encoding='utf8').rstrip('\n').split('\n'):
    if not u.strip():
        continue
    kind, rest = u.split('\t', 1)
    if kind == 'R':
        out.append(rest)
    elif kind == 'W':
        fp, rp, text = rest.split('\t', 2)
        fp = fp.replace('{sp}', ' '); rp = rp.replace('{sp}', ' ')
        w = textwrap.TextWrapper(width=120, initial_indent=fp, subsequent_indent=rp, break_on_hyphens=False,
                                 break_long_words=False)
        import re
        prot = lambda m: m.group(0).replace(' ', '\x00')
        t = re.sub(r'\[[^\]]*\]\([^)]*\)', prot, text)
        t = re.sub(r'(?<![\w.])(\d[\d.,]*|one|two|half a) (ms|Hz|%|kbps|B|s|MB|EUR|m|µs|tick|ticks|interval|intervals|minutes)\b', prot, t)
        t = re.sub(r'\b(N|P\d) = \d+', prot, t)
        t = re.sub(r'(\d) ([−+÷×]) (\d)', prot, t)
        out += [l.replace('\x00', ' ') for l in w.wrap(t)]
    else:
        raise SystemExit('bad unit ' + kind)
after = '\n'.join(out)
for i, l in enumerate(out):
    if len(l) > 120 and not l.startswith('|'):
        print(f'WARN line {i+1} of AFTER is {len(l)} chars')
assert 'END' not in [l.strip() for l in out]
block = f'### FIX {fid}\nFILE: docs/adr/0001-multiplayer-netcode.md\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n'
if dry:
    print(block)
else:
    with OUT.open('a', encoding='utf8') as f:
        f.write(block)
    print(f'appended FIX {fid}: lines {a}-{b} ({b - a + 1}) -> {len(out)}')
