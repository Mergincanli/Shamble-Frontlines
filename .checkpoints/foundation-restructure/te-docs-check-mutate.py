#!/usr/bin/env python3
"""test-engineer review, 2.1a: does docs-keeper-check-decisions.py bite? Runs a COPY of it on COPIES of
docs/DECISIONS.md and the report (taken at the pin into .gate/foundation-restructure/dc/), one mutation at a time.
Never touches the tracked files. Prints one line per mutation: expected, got, PASS/FAIL."""
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

L_YES = '> - Usage is not a reason to skip a consultation. Quality comes first.\n'
TWO = '> Two additions, my words of 2026-11-11:\n'
def two_block(t):
    i = t.index(TWO); j = t.index('\n\n', i) + 1
    return t[i:j]
def rewrap_split(t):
    i = t.index('> - My yes covers'); j = t.index('\n', i); line = t[i:j]
    k = line.index(' ', 60)
    return t[:i] + line[:k] + '\n> ' + line[k + 1:] + t[j:]
def rewrap_join(t):
    old = '>   1. before you commit to a plan or an approach;\n>   2. when'
    return once(old, '>   1. before you commit to a plan or an approach; 2. when', t, 'join')
def flip(t, anchor):
    i = t.index(anchor) + len(anchor) - 1
    c = t[i]; return t[:i] + ('X' if c != 'X' else 'Y') + t[i + 1:]

cases = [
    ('C0 control: unmodified copies', DEC0, RPT0, 0),
    ('M1 one byte changed in a quote (DECISIONS)', flip(DEC0, '> - My yes covers'), RPT0, 1),
    ('M2 one quote line dropped', once(L_YES, '', DEC0, 'drop'), RPT0, 1),
    ('M3 a quote line re-wrapped into two', rewrap_split(DEC0), RPT0, 1),
    ('M4 two quote lines joined into one', rewrap_join(DEC0), RPT0, 1),
    ('M5 a block inserted twice (blank line between)', DEC0.replace(two_block(DEC0), two_block(DEC0) + '\n' + two_block(DEC0), 1), RPT0, 1),
    ('M6 a block inserted twice (adjacent)', DEC0.replace(two_block(DEC0), two_block(DEC0) + two_block(DEC0), 1), RPT0, 1),
    ('M7 trailing space on a quote line', once(L_YES, L_YES[:-1] + ' \n', DEC0, 'ws'), RPT0, 1),
    ('M8 one byte changed in the appendix only', DEC0, flip(RPT0, '- Usage is not a reason to skip'), 1),
    ('M9 one byte changed in an old (bb2d817) line', None, RPT0, 1),
    ('M10 same byte changed in DECISIONS and appendix (A3.x block; expect PASS: appendix is the reference)',
     flip(DEC0, '> - Usage is not a reason to skip'), flip(RPT0, '- Usage is not a reason to skip'), 0),
]
# M9: flip a byte on a line that exists at bb2d817 and outside the italic paragraph
old_lines = subprocess.run(['git', 'show', 'bb2d817:docs/DECISIONS.md'], cwd=REPO, capture_output=True, check=True).stdout.decode().split('\n')
cand = next(l for l in old_lines[20:] if l.startswith('> ') and len(l) > 40 and DEC0.count(l + '\n') == 1)
cases[9] = (cases[9][0], DEC0.replace(cand, cand[:-1] + ('X' if cand[-1] != 'X' else 'Y'), 1), RPT0, 1)

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
print('DOCS-CHECK MUTATIONS:', 'ALL PASS' if not bad else '%d FAILED' % bad)
