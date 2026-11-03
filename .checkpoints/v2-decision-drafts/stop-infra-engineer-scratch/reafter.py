#!/usr/bin/env python3
"""Replace the AFTER of one existing FIX block in place. Usage: reafter.py <id> <spec>"""
import sys, re, pathlib
OUT = pathlib.Path('/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/stop-infra-engineer-fixes.md')
fid, spec = sys.argv[1], sys.argv[2]
s = OUT.read_text(encoding='utf8')
pat = re.compile(r'(^### FIX ' + re.escape(fid) + r'\n.*?\nAFTER:\n)(.*?)(\nEND\n)', re.S | re.M)
ms = list(pat.finditer(s)); assert len(ms) == 1, len(ms)
new = pathlib.Path(spec).read_text(encoding='utf8').rstrip('\n')
al = new.split('\n')
assert not [l for l in al if len(l) > 120 and not l.lstrip().startswith('|')]
assert not any(l.startswith('END') for l in al)
m = ms[0]
OUT.write_text(s[:m.start(2)] + new + s[m.end(2):], encoding='utf8')
print('replaced AFTER of', fid, len(al), 'lines')
