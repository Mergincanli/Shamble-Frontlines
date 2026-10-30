"""Step 3d's sample of UNREAD_SOURCES.md (advisor condition 8 of plan-amendments-2: "step 3d samples 10 rows").

Draws 10 rows of part 2 (every source not read) with a fixed seed, split over the three verifier units by the topics
in "Sought by" (r2: R2 · r14: R1 or R4 · r356: R3, R5 or R6; 3, 3 and 4 rows, no row twice). Within a unit the rows
are shuffled with the seed, and the draw takes the first row of each kind of "Needed for" cell (a hinge row, a named
id, a unit's note, no tie found) before filling up in shuffled order, so every kind of cell gets checked.
Writes unread-sample-<unit>.md. Usage: python3 unread_sample.py <commit of UNREAD_SOURCES.md>
"""
import random, re, sys

SHA = sys.argv[1]
D = '/home/user/Shamble-Frontlines/.checkpoints/v2-research'
text = open('/home/user/Shamble-Frontlines/docs/research/UNREAD_SOURCES.md').read()
part2 = text.split('\n## 2. ', 1)[1]
lines = [l for l in part2.splitlines() if l.startswith('| ')]
header, rows = lines[0], lines[1:]          # lines[1] of the table is the |---| rule, which starts with "|-"
rows = [r for r in rows if not r.startswith('|---')]


def cells(r):
    return [c.strip() for c in r.strip().strip('|').split(' | ')]


def kind(need):
    if need.startswith('a recommendation hinges on it'):
        return 'hinge'
    if need.startswith('the unit\'s note'):
        return 'note'
    if need.startswith('no tie found'):
        return 'none'
    return 'named'


UNITS = [('r2', {'R2'}, 3), ('r14', {'R1', 'R4'}, 3), ('r356', {'R3', 'R5', 'R6'}, 4)]
rng = random.Random(20261030)
used = set()
for unit, topics, quota in UNITS:
    pool = [r for r in rows if r not in used and set(re.findall(r'\bR[1-6]\b', cells(r)[1])) & topics]
    rng.shuffle(pool)
    pick = []
    for k in ('hinge', 'named', 'note', 'none'):
        for r in pool:
            if kind(cells(r)[2]) == k and r not in pick:
                pick.append(r)
                break
    for r in pool:
        if len(pick) >= quota:
            break
        if r not in pick:
            pick.append(r)
    pick = pick[:quota]
    used.update(pick)
    with open(f'{D}/unread-sample-{unit}.md', 'w') as f:
        f.write(f'# UNREAD_SOURCES.md sample for the step-3d verifier unit {unit}\n\n'
                f'Drawn by `scripts/unread_sample.py` (seed 20261030) from part 2 of docs/research/UNREAD_SOURCES.md at '
                f'commit {SHA}: {quota} rows sought by {", ".join(sorted(topics))}, the first of each kind of "Needed for" '
                f'cell first. Check each row\'s "Needed for" cell against the topic file (check 6 of your task).\n\n')
        f.write(header + '\n|---|---|---|---|---|\n' + '\n'.join(pick) + '\n')
    print(unit, len(pool), 'candidate rows;', [kind(cells(r)[2]) for r in pick])
