#!/usr/bin/env python3
"""Compares two package-lock.json files entry by entry, for the plan's lockfile rule (section 7, item 3):
entries may be added; our own packages' entries may change; on an entry that is already there only npm's flags
(peer, dev, optional, devOptional) may change. Prints every difference; exit 1 when the rule is broken.
  python3 -I lock-compare.py <old package-lock.json> <new package-lock.json>"""
import json
import sys

FLAGS = {"peer", "dev", "optional", "devOptional"}
old = json.load(open(sys.argv[1], encoding="utf-8"))
new = json.load(open(sys.argv[2], encoding="utf-8"))
broken = []
for key in ("name", "lockfileVersion", "requires"):
    if old.get(key) != new.get(key):
        broken.append(f"top-level {key}: {old.get(key)!r} -> {new.get(key)!r}")
po, pn = old["packages"], new["packages"]
own = lambda k: k == "" or k.startswith("packages/") or k.startswith("node_modules/@shamble/")
added = sorted(k for k in pn if k not in po)
removed = sorted(k for k in po if k not in pn)
print(f"entries: {len(po)} before, {len(pn)} after; {len(added)} added, {len(removed)} removed")
for k in added:
    e = pn[k]
    flags = ",".join(f for f in sorted(FLAGS) if e.get(f)) or "-"
    print(f"  added    {k}  {e.get('version', '(link)' if e.get('link') else '?')}  flags {flags}  deps {sorted((e.get('dependencies') or {}).items())}"
          f"{'  hasInstallScript' if e.get('hasInstallScript') else ''}{'  bin ' + str(e.get('bin')) if e.get('bin') else ''}")
for k in removed:
    print(f"  removed  {k}")
    broken.append(f"removed entry {k}")
flag_only, own_changed = [], []
for k in sorted(set(po) & set(pn)):
    a, b = po[k], pn[k]
    if a == b:
        continue
    fields = sorted(f for f in set(a) | set(b) if a.get(f) != b.get(f))
    if own(k):
        own_changed.append((k, fields))
    elif set(fields) <= FLAGS:
        flag_only.append((k, {f: (a.get(f), b.get(f)) for f in fields}))
    else:
        broken.append(f"{k}: fields changed beyond flags: {fields}")
print(f"our own entries changed: {len(own_changed)}")
for k, fields in own_changed:
    print(f"  own      {k or '(root)'}  {fields}")
print(f"entries with flag-only changes: {len(flag_only)}")
for k, change in flag_only:
    print(f"  flags    {k}  {change}")
print("RULE BROKEN:" if broken else "rule kept: additions, our own entries, and flag-only changes on existing entries")
for line in broken:
    print("  " + line)
sys.exit(1 if broken else 0)
