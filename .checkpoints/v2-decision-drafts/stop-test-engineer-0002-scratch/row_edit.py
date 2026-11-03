#!/usr/bin/env python3
"""Print line <n> of the doc ON DISK with exact substring replacements (each must occur exactly once in the line).
Usage: row_edit.py <docs/path> <n> <old1> <new1> [<old2> <new2> ...]"""
import sys, pathlib
REPO = pathlib.Path('/home/user/Shamble-Frontlines')
path, n = sys.argv[1], int(sys.argv[2])
line = (REPO / path).read_text(encoding='utf8').split('\n')[n-1]
pairs = sys.argv[3:]
for old, new in zip(pairs[0::2], pairs[1::2]):
    c = line.count(old)
    if c != 1: sys.exit(f'{old!r} occurs {c} times')
    line = line.replace(old, new)
print(line)
