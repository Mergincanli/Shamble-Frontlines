#!/usr/bin/env python3
"""Build a W spec from original lines a..b (joined, indent stripped) with exact replacements.
Usage: fromorig.py <a> <b> <first_prefix> <rest_prefix> <out> <old1> <new1> [<old2> <new2> ...]"""
import sys
L = open('/home/user/Shamble-Frontlines/docs/adr/0001-multiplayer-netcode.md', encoding='utf8').read().split('\n')
a, b, fp, rp, out = int(sys.argv[1]), int(sys.argv[2]), sys.argv[3], sys.argv[4], sys.argv[5]
first = L[a - 1]
assert first.startswith(fp.replace('{sp}', ' ')), 'first prefix mismatch'
parts = [first[len(fp.replace('{sp}', ' ')):]] + [l.strip() for l in L[a:b]]
t = ' '.join(parts)
reps = sys.argv[6:]
for o, n in zip(reps[::2], reps[1::2]):
    assert t.count(o) == 1, f'{t.count(o)} x {o!r}'
    t = t.replace(o, n)
open(out, 'w', encoding='utf8').write(f'W\t{fp}\t{rp}\t{t}\n')
