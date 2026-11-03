#!/usr/bin/env python3
"""Print an edited copy of lines start..end of a doc on disk: pairs of (old, new) from a JSON file, each old must
occur exactly once in the selected text. Usage: row_edit.py <docs/path> <start> <end> <pairs.json>"""
import sys, json, pathlib
REPO = pathlib.Path('/home/user/Shamble-Frontlines')
path, s, e, pj = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
text = '\n'.join((REPO / path).read_text(encoding='utf8').split('\n')[s-1:e])
for old, new in json.load(open(pj, encoding='utf8')):
    n = text.count(old)
    if n != 1:
        sys.exit(f'old occurs {n} times: {old[:60]}')
    text = text.replace(old, new)
print(text)
