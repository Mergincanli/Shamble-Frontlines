"""Read-only: print every leaf that differs between two JSON files (path, old, new), and leaves present in only one.
Usage: python3 -I jdiff.py <old.json> <new.json>"""
import json
import sys


def leaves(x, p=""):
    if isinstance(x, dict):
        for k, v in x.items():
            yield from leaves(v, f"{p}.{k}")
    elif isinstance(x, list):
        for i, v in enumerate(x):
            yield from leaves(v, f"{p}[{i}]")
    else:
        yield p, x


a = dict(leaves(json.load(open(sys.argv[1], encoding="utf-8"))))
b = dict(leaves(json.load(open(sys.argv[2], encoding="utf-8"))))
n = 0
for k in sorted(set(a) | set(b)):
    if a.get(k, "<absent>") != b.get(k, "<absent>"):
        n += 1
        print(f"{k}: {str(a.get(k, '<absent>'))[:200]} -> {str(b.get(k, '<absent>'))[:200]}")
print(f"{n} leaves differ of {len(set(a) | set(b))}")
