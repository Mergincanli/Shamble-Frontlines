"""Append one FIX block to the output file: BEFORE copied from the doc by line range (whole lines).

Usage: python3 mkblock.py <spec.py>   (the spec sets ID, TITLE, START, END, AFTER)
Checks: BEFORE occurs exactly once in the doc; AFTER has the same line count; no AFTER line over 120 chars;
the block's range overlaps no range already written (ranges logged in ranges.txt).
"""
import sys, os
DOC = '/home/user/Shamble-Frontlines/docs/adr/0006-infrastructure-operations.md'
OUT = '/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/touchup-infra-engineer-fixes.md'
HERE = os.path.dirname(os.path.abspath(__file__))
RANGES = os.path.join(HERE, 'ranges.txt')
spec = {}
exec(open(sys.argv[1], encoding='utf-8').read(), spec)
text = open(DOC, encoding='utf-8').read()
lines = text.split('\n')
before = '\n'.join(lines[spec['START'] - 1:spec['END']])
after = spec['AFTER'].strip('\n')
n = text.count(before)
assert n == 1, ('BEFORE occurs %d times' % n)
# whole lines: must start at a line start and end at a line end
i = text.index(before)
assert i == 0 or text[i - 1] == '\n'
assert text[i + len(before)] == '\n'
nb, na = before.count('\n') + 1, after.count('\n') + 1
assert nb == na or spec.get('LINES_CHANGE'), ('line count %d -> %d' % (nb, na))
for l in after.split('\n'):
    assert len(l) <= 120, ('too long (%d): %s' % (len(l), l))
done = []
if os.path.exists(RANGES):
    done = [tuple(map(int, r.split()[:2])) for r in open(RANGES) if r.strip()]
for a, b in done:
    assert spec['END'] < a or spec['START'] > b, ('overlaps %d-%d' % (a, b))
assert before != after
block = '### FIX %s (%s; lines %d-%d on disk now)\nFILE: docs/adr/0006-infrastructure-operations.md\nBEFORE:\n%s\nAFTER:\n%s\nEND\n\n' % (
    spec['ID'], spec['TITLE'], spec['START'], spec['END'], before, after)
with open(OUT, 'a', encoding='utf-8') as f:
    f.write(block)
with open(RANGES, 'a') as f:
    f.write('%d %d %s\n' % (spec['START'], spec['END'], spec['ID']))
print('ok', spec['ID'], nb, 'lines ->', na)
