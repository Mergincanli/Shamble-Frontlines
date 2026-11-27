# test-engineer review 4b-2 round 2: plants in a SCRATCH COPY only (argv[1]), never the tree.
#   swap  : round 1's swap (sprint_time_from_full_s <-> sprint_returns_at_pct, table fields and soldier.json values)
#   group : a row moved to another group's entry of the same last step (movement.body_radius_m -> hornet.body_radius_m,
#           his Export's 0.45) and soldier.json given that value: the field name is unchanged.
#   rename: C4's pair, the renamed row in it.
#   reset : put the scratch's content back from the tree.
import os, shutil, sys
root, which = sys.argv[1], sys.argv[2]
assert root.endswith("test-engineer-4b2-scratch/r2"), root
tree = os.path.abspath(os.path.join(root, "../../.."))
table = os.path.join(root, "content/source/testlines-table.json")
soldier = os.path.join(root, "content/movement/soldier.json")
def sub(p, a, b):
    t = open(p).read(); assert t.count(a) == 1, (p, a); open(p, "w").write(t.replace(a, b))
if which == "reset":
    for rel in ("content/source/testlines-table.json", "content/movement/soldier.json"):
        shutil.copyfile(os.path.join(tree, rel), os.path.join(root, rel))
elif which == "swap":
    t = open(table).read(); a, b = '"field": "sprint_time_from_full_s"', '"field": "sprint_returns_at_pct"'
    assert t.count(a) == 1 and t.count(b) == 1
    open(table, "w").write(t.replace(a, "\0").replace(b, a).replace("\0", b))
    sub(soldier, '  "sprint_returns_at_pct": 30,\n  "sprint_time_from_full_s": 7,\n',
        '  "sprint_returns_at_pct": 7,\n  "sprint_time_from_full_s": 30,\n')
elif which == "group":
    sub(table, '"export": "movement.body_radius_m"', '"export": "hornet.body_radius_m"')
    sub(soldier, '"body_radius_m": 0.4,', '"body_radius_m": 0.45,')
elif which == "rename":
    # C4's pair: the renamed row and sprint_time_from_full_s trade fields, and soldier.json their values.
    t = open(table).read(); a, b = '"field": "sprint_time_from_full_s"', '"field": "stamina_refill_time_s"'
    assert t.count(a) == 1 and t.count(b) == 1
    open(table, "w").write(t.replace(a, "\0").replace(b, a).replace("\0", b))
    s = open(soldier).read()
    x, y = '"sprint_time_from_full_s": 7,', '"stamina_refill_time_s": 5,'
    assert s.count(x) == 1 and s.count(y) == 1
    open(soldier, "w").write(s.replace(x, '"sprint_time_from_full_s": 5,').replace(y, '"stamina_refill_time_s": 7,'))
else:
    raise SystemExit("swap | group | reset")
print(which, "done")
