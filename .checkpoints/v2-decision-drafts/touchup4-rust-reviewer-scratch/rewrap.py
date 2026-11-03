# Re-wrap ADR 0004 lines A..B (1-based, inclusive) after word replacements; greedy at 120 characters, 3-space indent.
# Prints the candidate lines, their lengths and risky line starts. Usage: rewrap.py <doc> <A> <B> <variant>
import sys, re
doc, A, B, v = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
REF = sys.argv[5] if len(sys.argv) > 5 else 'a rounded exact reference'
L = open(doc, encoding='utf8').read().split('\n')
old = L[A-1:B]
assert all(l.startswith('   ') and not l.startswith('    ') for l in old)
text = ' '.join(l[3:] for l in old)
old_anchor = "as does its `f64` analogue (verifiers fixcheck-d3a and skeptic-6, 2026-11-03). A"
assert text.count(old_anchor) == 1 and text.count('equal an exact reference') == 1
adds = {
 'a': "as does its `f64` analogue (verifiers fixcheck-d3a and skeptic-6, 2026-11-03), and so does 1.99.1, ADR 0002's pin, whose source takes the same steps (verifier fixcheck-d3e, 2026-11-03). A",
 'b': "as does its `f64` analogue (verifiers fixcheck-d3a and skeptic-6, 2026-11-03); ADR 0002's pin, 1.99.1, gives 3 too, its source taking the same steps (verifier fixcheck-d3e, 2026-11-03). A",
 'd': "as does its `f64` analogue (verifiers fixcheck-d3a and skeptic-6, 2026-11-03), and so does 1.99.1, the pin, whose source takes the same steps (verifier fixcheck-d3e, 2026-11-03). A",
 'e': "as does its `f64` analogue (verifiers fixcheck-d3a and skeptic-6, 2026-11-03); the pin, 1.99.1, gives 3 too, its source taking the same steps (verifier fixcheck-d3e, 2026-11-03). A",
 'f': "as does its `f64` analogue (verifiers fixcheck-d3a and skeptic-6, 2026-11-03), and so does the pin, 1.99.1, by the same steps (verifier fixcheck-d3e, 2026-11-03). A",
 'c': "as does its `f64` analogue (verifiers fixcheck-d3a and skeptic-6, 2026-11-03), and the pinned 1.99.1 (ADR 0002 part 2) gives 3 too, since its source takes the same steps (verifier fixcheck-d3e, 2026-11-03). A",
}
new = text.replace(old_anchor, adds[v]).replace('equal an exact reference', 'equal ' + REF)
words = new.split(' '); lines = []; cur = ''
for w in words:
    cand = (cur + ' ' + w) if cur else w
    if len('   ' + cand) <= 120: cur = cand
    else: lines.append('   ' + cur); cur = w
lines.append('   ' + cur)
print(f'variant {v}: {len(old)} old lines -> {len(lines)} new lines; max len {max(map(len, lines))}')
for l in lines:
    bad = re.match(r'   ([-+*>#|]|\d+[.)])( |$)', l)
    print(f'{len(l):4d}{" RISKY" if bad else ""} {l}')
# word diff check: new words minus old words
import difflib
ow, nw = text.split(), ' '.join(l[3:] for l in lines).split()
print('word diff:', [d for d in difflib.ndiff(ow, nw) if d[0] in '+-'])
