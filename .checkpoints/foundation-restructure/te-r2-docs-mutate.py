#!/usr/bin/env python3
"""test-engineer round 2, 2.1a: does the 12-block docs-keeper-check-decisions.py still bite? Same method as
te-docs-check-mutate.py (a COPY of the check, its two open() paths pointed at copies of docs/DECISIONS.md and the report
taken at 76544aa into .gate/foundation-restructure/dc/), with mutations on the three new blocks A3.3-A3.5 plus two
round-1 mutations re-run. Never touches tracked files."""
import os, subprocess, sys

REPO = '/home/user/Shamble-Frontlines'
D = REPO + '/.gate/foundation-restructure/dc'
SRC = REPO + '/.checkpoints/foundation-restructure/docs-keeper-check-decisions.py'
s = open(SRC, encoding='utf-8').read()
a = "open(REPO + '/docs/reports/2026-11-11-foundation-2-1a.md'"
b = "open(REPO + '/docs/DECISIONS.md'"
assert s.count(a) == 1 and s.count(b) == 1, 'path patterns not unique'
s = s.replace(a, "open(os.environ['TE_RPT']").replace(b, "open(os.environ['TE_DEC']").replace('import difflib,', 'import os, difflib,', 1)
open(D + '/check-copy.py', 'w', encoding='utf-8').write(s)
DEC0 = open(D + '/DECISIONS.orig.md', encoding='utf-8').read()
RPT0 = open(D + '/report.orig.md', encoding='utf-8').read()

def once(old, new, text, what):
    n = text.count(old)
    assert n == 1, '%s: pattern occurs %d times' % (what, n)
    return text.replace(old, new)
def flip(t, anchor):
    assert t.count(anchor) == 1, anchor
    i = t.index(anchor) + len(anchor) - 1
    c = t[i]; return t[:i] + ('X' if c != 'X' else 'Y') + t[i + 1:]

T35 = '> - The tally is enough as it is. No further agent for it.\n'
T34 = '> Carry on from where you are. The mutation proof keeps running.\n'
T34b = '> - Do not edit any settings file, hook or allow-list to get fewer prompts.\n'
T33 = '> - tools/smoke.mjs stays unchanged. Its wording Low becomes a follow-up for 2.3.\n'
H35 = '> My words of 2026-11-11, a correction to my words on permission prompts of 16:32 UTC'
def a35_block(t):
    i = t.index(H35); j = t.index('\n\n', i) + 1 if '\n\n' in t[i:] else len(t)
    # the block ends at the first line not starting with '>'
    k = i
    for line in t[i:].split('\n'):
        if not line.startswith('>'): break
        k += len(line) + 1
    return t[i:k]

cases = [
    ('C0 control: unmodified copies at 76544aa', DEC0, RPT0, 0),
    ('N1 one byte changed in A3.5 (DECISIONS)', flip(DEC0, T35[:-1]), RPT0, 1),
    ('N2 A3.4 last line dropped (DECISIONS)', once(T34, '', DEC0, 'n2'), RPT0, 1),
    ('N3 A3.4 two lines swapped (DECISIONS)', once(T34b, '', DEC0, 'n3a').replace(T34, T34 + T34b, 1) if False else None, RPT0, 1),
    ('N4 one byte changed in A3.3 (appendix only)', DEC0, flip(RPT0, T33[2:-1]), 1),
    ('N5 A3.5 quote removed whole from DECISIONS', None, RPT0, 1),
    ('N6 trailing space on an A3.3 line (DECISIONS)', once(T33, T33[:-1] + ' \n', DEC0, 'n6'), RPT0, 1),
    ('R1 (round-1 M1) one byte changed in an A3.1 quote', flip(DEC0, '> - Usage is not a reason to skip'), RPT0, 1),
    ('R2 (round-1 M2) one A3.1 quote line dropped', once('> - Usage is not a reason to skip a consultation. Quality comes first.\n', '', DEC0, 'r2'), RPT0, 1),
]
# N3: move the A3.4 line T34b to just before the block's last line (order changed, same bytes)
d = once(T34b, '', DEC0, 'n3a'); d = once(T34, T34b + T34, d, 'n3b')
cases[3] = (cases[3][0], d, RPT0, 1)
blk = a35_block(DEC0)
assert blk.count('\n') >= 7 and DEC0.count(blk) == 1, 'A3.5 block not found once'
cases[5] = (cases[5][0], DEC0.replace(blk, '', 1), RPT0, 1)

bad = 0
for name, dec, rpt, want in cases:
    open(D + '/DEC.md', 'w', encoding='utf-8').write(dec)
    open(D + '/RPT.md', 'w', encoding='utf-8').write(rpt)
    env = dict(os.environ, TE_DEC=D + '/DEC.md', TE_RPT=D + '/RPT.md')
    p = subprocess.run([sys.executable, '-I', D + '/check-copy.py'], env=env, capture_output=True, text=True)
    fails = [l for l in p.stdout.split('\n') if l.startswith('FAIL ')]
    crashed = p.returncode not in (0, 1)
    ok = (p.returncode == want) and not crashed
    if not ok: bad += 1
    first = (fails[0][:110] if fails else (p.stderr.strip().split('\n')[-1][:110] if crashed else '(no FAIL line)'))
    print('%s %s: exit %s (want %d); %s' % ('PASS' if ok else 'FAIL', name, p.returncode, want, first))
print('DOCS-CHECK MUTATIONS (12 blocks):', 'ALL PASS' if not bad else '%d FAILED' % bad)
