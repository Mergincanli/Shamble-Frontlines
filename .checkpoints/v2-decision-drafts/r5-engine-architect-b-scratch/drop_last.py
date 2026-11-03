#!/usr/bin/env python3
"""Drop the last complete block (### FIX/NOTE ... END) from the fixes file, if its id matches argv[1]."""
import sys, re, pathlib
p = pathlib.Path('/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/r5-engine-architect-b-fixes.md')
t = p.read_text(encoding='utf8')
i = t.rfind('\n### ')
head, last = t[:i + 1], t[i + 1:]
m = re.match(r'### (FIX|NOTE) (\S+)', last)
if not m or m.group(2) != sys.argv[1]:
    sys.exit(f'last block is {m.group(2) if m else None}, not {sys.argv[1]}')
p.write_text(head, encoding='utf8'); print('dropped', sys.argv[1])
