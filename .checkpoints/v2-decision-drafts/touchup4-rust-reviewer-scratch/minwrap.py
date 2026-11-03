# Re-wrap ADR 0004 lines A..B after replacements into exactly the same number of lines, each <= 120 characters with a
# 3-space indent and no risky Markdown start, keeping as many of the original line breaks as possible (DP).
# Usage: minwrap.py <A> <B> <candidates.py> <name> [out.txt]
import sys, re, difflib
DOC = '/home/user/Shamble-Frontlines/docs/adr/0004-simulation-core.md'
L = open(DOC, encoding='utf8').read().split('\n')
A, B = int(sys.argv[1]), int(sys.argv[2])
G = {}; exec(open(sys.argv[3], encoding='utf8').read(), G); pairs = G['CANDS'][sys.argv[4]]
old_lines = L[A-1:B]
O, obreaks = [], set()
for l in old_lines:
    O += l[3:].split(' '); obreaks.add(len(O))          # a break after token index len(O)-1
text = ' '.join(O); t = text
for o, n in pairs:
    assert t.count(o) == 1, o; t = t.replace(o, n)
N = t.split(' ')
# map original breaks to new token positions through equal blocks
nb = set()
sm = difflib.SequenceMatcher(None, O, N, autojunk=False)
for a, b, size in sm.get_matching_blocks():
    for k in range(1, size):                              # break between O[a+k-1] and O[a+k] lies inside the block
        if a + k in obreaks: nb.add(b + k)
target = len(old_lines); n = len(N)
def ok(i, j):
    s = '   ' + ' '.join(N[i:j])
    return len(s) <= 120 and not re.match(r'   ([-+*>#|]|\d+[.)])( |$)', s)
NEG = -10**9
best = [[NEG] * (target + 1) for _ in range(n + 1)]; prev = [[None] * (target + 1) for _ in range(n + 1)]
best[0][0] = 0
for i in range(n):
    for k in range(target):
        if best[i][k] == NEG: continue
        for j in range(i + 1, n + 1):
            if len('   ' + ' '.join(N[i:j])) > 120: break
            if not ok(i, j): continue
            v = best[i][k] + (1 if j in nb else 0)
            if v > best[j][k + 1]: best[j][k + 1] = v; prev[j][k + 1] = i
assert best[n][target] != NEG, 'does not fit'
cuts = []; j, k = n, target
while j: i = prev[j][k]; cuts.append((i, j)); j, k = i, k - 1
new_lines = ['   ' + ' '.join(N[i:j]) for i, j in reversed(cuts)]
same = sum(1 for a, b in zip(old_lines, new_lines) if a == b)
print(f'{len(old_lines)} -> {len(new_lines)} lines; kept breaks {best[n][target]} of {len(nb)} mappable; identical lines {same}; max {max(map(len, new_lines))}')
for a, b in zip(old_lines, new_lines): print(('  ' if a == b else '* ') + f'{len(b):4d} {b}')
print('word diff:', [d for d in difflib.ndiff(O, N) if d[0] in '+-'])
if len(sys.argv) > 5: open(sys.argv[5], 'w', encoding='utf8').write('\n'.join(new_lines) + '\n')
