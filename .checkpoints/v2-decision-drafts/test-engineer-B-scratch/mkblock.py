#!/usr/bin/env python3
# usage: mkblock.py "<id>" <docpath> <start> <end> <afterfile>
# Copies lines start..end (1-based, inclusive) of docpath at 75615d1 as BEFORE, checks it occurs exactly once,
# and appends a FIX block to the output file.
import subprocess, sys
fid, doc, a, b, af = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), sys.argv[5]
src = subprocess.run(['git','-C','/home/user/Shamble-Frontlines','show','75615d1:'+doc],capture_output=True,text=True,check=True).stdout
lines = src.split('\n')
before = '\n'.join(lines[a-1:b])
n = src.count(before)
if n != 1:
    sys.exit(f'BEFORE occurs {n} times')
after = open(af).read().rstrip('\n')
if '\nEND\n' in '\n'+after+'\n':
    sys.exit('AFTER contains END line')
out = '/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/test-engineer-B-fixes-final.md'
with open(out,'a') as f:
    f.write(f'### FIX {fid}\nFILE: {doc}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n')
print('ok', fid, a, b, 'before chars', len(before))
