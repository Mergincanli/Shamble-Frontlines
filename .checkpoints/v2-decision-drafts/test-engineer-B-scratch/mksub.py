#!/usr/bin/env python3
# usage: mksub.py "<id>" <docpath> <start> <end> <subsfile>
# subsfile: pairs of blocks separated by lines '<<<OLD' ... '>>>NEW' ... '===' ; each OLD must occur once in BEFORE.
import subprocess, sys, os
fid, doc, a, b, sf = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), sys.argv[5]
src = subprocess.run(['git','-C','/home/user/Shamble-Frontlines','show',os.environ.get('COMMIT','75615d1')+':'+doc],capture_output=True,text=True,check=True).stdout
lines = src.split('\n'); before = '\n'.join(lines[a-1:b])
if src.count(before) != 1: sys.exit('BEFORE not unique')
txt = open(sf).read(); after = before
for chunk in txt.split('\n===\n'):
    chunk = chunk.strip('\n')
    if not chunk: continue
    old, new = chunk.split('\n>>>NEW\n'); old = old.replace('<<<OLD\n','',1)
    c = after.count(old)
    if c != 1: sys.exit(f'OLD occurs {c} times: {old[:60]}')
    after = after.replace(old, new)
out = '/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/test-engineer-B-fixes-final.md'
with open(out,'a') as f: f.write(f'### FIX {fid}\nFILE: {doc}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n')
print('ok', fid)
