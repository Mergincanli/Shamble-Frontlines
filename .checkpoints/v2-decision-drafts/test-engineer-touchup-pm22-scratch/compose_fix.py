# test-engineer touch-up PM-0004-22: build FIX PM22-EUCLID from the row as it stands on disk (whole line, by script).
import sys, pathlib
DOC = pathlib.Path('/home/user/Shamble-Frontlines/docs/adr/0004-simulation-core.md')
OUT = pathlib.Path('/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/test-engineer-touchup-pm22-fixes.md')
text = DOC.read_text(encoding='utf8')
rows = [l for l in text.split('\n') if l.startswith('| PM-0004-22 |')]
assert len(rows) == 1, rows
before = rows[0]
assert text.count(before) == 1

A1 = '`sqrt` also equals a correctly rounded reference;'
INS1 = (" if part 7's `rem_euclid` or `div_euclid` is to join the allow-list (neither is on it, so this row's pass today"
        " does not wait on them), the method gets the same check, at least 10,000 fixed inputs with (3 + 2^-22,"
        " 1 + 2^-23) in `f32`, (3 + 2^-51, 1 + 2^-52) in `f64` and their sign mirrors among them, and its outputs also"
        " equal by value"
        " (`==`, which leaves a zero's sign to `to_bits`) a correctly rounded Euclidean reference, the exact quotient and"
        " remainder by integer or rational arithmetic rounded once to the type: for (a, b) and its mirrors (−a, b), (a, −b) and (−a, −b),"
        " the quotients 2, −3, −2 and 3 and the remainders 1, 2^-23, 1 and 2^-23 in `f32` and 1, 2^-52, 1 and 2^-52 in `f64` (verifiers"
        " fixcheck-d3a and skeptic-6 and the Rust and test reviews, 2026-11-03);")
A2 = 'Bites: one output moved by one unit in the last place on one target'
INS2 = ("; and, for the Euclidean clause when it runs, errors every target shares: a `div_euclid` planted as rustc"
        " 1.97.0's steps (`trunc(a / b)`, corrected by one only when `a % b < 0`: skeptic-6's disassembly, 2026-11-03),"
        " which gives 3 for (3 + 2^-22, 1 + 2^-23) where the exact quotient is 2, and one `rem_euclid` output moved by"
        " one unit in the last place on every target alike")
for a in (A1, A2):
    assert before.count(a) == 1, a
after = before.replace(A1, A1 + INS1, 1).replace(A2, A2 + INS2, 1)
assert '\n' not in after and '"' not in INS1 + INS2
cells = lambda l: l.strip().strip('|').split(' | ')
assert len(cells(before)) == 7 and len(cells(after)) == 7, (len(cells(before)), len(cells(after)))
assert after.count('|') == before.count('|')
if '--write' in sys.argv:
    block = ('### FIX PM22-EUCLID\nFILE: docs/adr/0004-simulation-core.md\nBEFORE:\n' + before + '\nAFTER:\n' + after
             + '\nEND\n\n')
    with OUT.open('a', encoding='utf8') as f:
        f.write(block)
    print('appended FIX PM22-EUCLID to', OUT)
print('cells before/after:', len(cells(before)), len(cells(after)), '; pipes:', before.count('|'), after.count('|'))
print('chars before/after:', len(before), len(after))
