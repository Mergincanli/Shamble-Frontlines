# test-engineer review 4b-2: in a SCRATCH COPY only (argv[1]), swap two unpinned, unequal movement fields
# (sprint_time_from_full_s 7, sprint_returns_at_pct 30) in the table and in soldier.json, as plant C4 does for a
# pinned pair. Never run on the tree.
import os, sys
root = sys.argv[1]
assert root.endswith("test-engineer-4b2-scratch"), root
p = os.path.join(root, "content/source/testlines-table.json"); t = open(p).read()
a, b = '"field": "sprint_time_from_full_s"', '"field": "sprint_returns_at_pct"'
assert t.count(a) == 1 and t.count(b) == 1
open(p, "w").write(t.replace(a, "\0").replace(b, a).replace("\0", b))
p = os.path.join(root, "content/movement/soldier.json"); t = open(p).read()
x = '  "sprint_returns_at_pct": 30,\n  "sprint_time_from_full_s": 7,\n'
assert t.count(x) == 1
open(p, "w").write(t.replace(x, '  "sprint_returns_at_pct": 7,\n  "sprint_time_from_full_s": 30,\n'))
print("swapped")
