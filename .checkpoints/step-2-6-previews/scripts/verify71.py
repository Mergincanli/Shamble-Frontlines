# Checks a resolved merge: every line either side added (against the merge base) is in the result as often as that
# side added it, every line either side removed is gone as that side left it, and nothing else is new.
import subprocess, collections, sys
BASE, OURS, THEIRS = 'c1da1bd24faae21794195e2384895460753a41eb', '4fda80a4038e8e1656e89a82fd56ff8ce33396bd', '07d673b0ee509d36ce1b976ed8dfae002c7d7b4a'
def show(ref, p):
    r = subprocess.run(['git', 'show', f'{ref}:{p}'], capture_output=True, text=True); return r.stdout.split('\n') if r.returncode == 0 else []
ok = True
for p in sys.argv[1:]:
    b, o, t = (collections.Counter(show(x, p)) for x in (BASE, OURS, THEIRS))
    r = collections.Counter(open(p, encoding='utf-8').read().split('\n'))
    bad = []
    for line in set(b) | set(o) | set(t) | set(r):
        # three-way expectation per distinct line: base count plus each side's change
        exp = b[line] + (o[line] - b[line]) + (t[line] - b[line])
        if exp < 0: exp = 0
        if line == '' : continue
        if r[line] != exp: bad.append((r[line], exp, b[line], o[line], t[line], line[:110]))
    print(p, 'OK' if not bad else f'{len(bad)} lines differ from the three-way count')
    for x in bad[:12]: print('   result', x[0], 'expected', x[1], '(base', x[2], 'ours', x[3], 'theirs', x[4], ')', x[5])
    ok &= not bad
print('ALL OK' if ok else 'CHECK')
