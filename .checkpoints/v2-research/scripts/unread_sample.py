"""Step 3d's sample of UNREAD_SOURCES.md (advisor condition 8 of plan-amendments-2: "step 3d samples 10 rows").

Draws 10 rows of part 2 (every source not read) with a fixed seed, split over the three verifier units by the topics
in "Sought by" (r2: R2 · r14: R1 or R4 · r356: R3, R5 or R6; 3, 3 and 4 rows, no row twice). Within a unit the rows
are shuffled with the seed, and the draw takes the first row of each kind of "Needed for" cell (a hinge row, a named
id, a unit's note, no tie found) before filling up in shuffled order, so every kind of cell gets checked.
Writes unread-sample.md (all 10 rows). Usage: python3 unread_sample.py <commit of UNREAD_SOURCES.md>
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
drawn = []
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
    drawn += [(unit, r) for r in pick]
    print(unit, len(pool), 'candidate rows;', [kind(cells(r)[2]) for r in pick])
with open(f'{D}/unread-sample.md', 'w') as f:   # one file: the R1+R4 unit, which runs last, checks all 10 rows
    f.write(f'# UNREAD_SOURCES.md sample for step 3d (10 rows, all six topics)\n\n'
            f'Drawn by `scripts/unread_sample.py` (seed 20261030) from part 2 of docs/research/UNREAD_SOURCES.md at commit '
            f'{SHA}: 3 rows sought by R2, 3 by R1 or R4, 4 by R3, R5 or R6, the first of each kind of "Needed for" cell '
            f'first. Check each row\'s "Needed for" cell against the topic file (check 6 of your task).\n\n')
    f.write('| Drawn for | ' + header[2:] + '\n|---|---|---|---|---|---|\n' + '\n'.join(f'| {u} {r}' for u, r in drawn) + '\n')
