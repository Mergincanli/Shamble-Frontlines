#!/usr/bin/env python3
"""Regenerate one FIX block in place in the fixes file (same id, same lines). Usage: reblock.py <id> <a> <b> <spec>"""
import sys, subprocess, re, pathlib
OUT = pathlib.Path('/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/stop-netcode-architect-fixes.md')
fid, a, b, spec = sys.argv[1:5]
new = subprocess.run([sys.executable, '-I', 'mkblock.py', fid, a, b, spec, '--dry'], capture_output=True, text=True, check=True).stdout
new = new[new.index('### FIX'):]
new = new.rstrip('\n') + '\n\n'
s = OUT.read_text(encoding='utf8')
pat = re.compile(r'^### FIX ' + re.escape(fid) + r'\n.*?\nEND\n\n', re.S | re.M)
assert len(pat.findall(s)) == 1, 'block not found once'
s = pat.sub(lambda m: new, s, count=1)
OUT.write_text(s, encoding='utf8'); print('replaced', fid)
