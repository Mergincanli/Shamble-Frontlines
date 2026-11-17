# Change-set 0, the verifier's finding 5: the research files cite repository lines as read on main c94c81b; this
# change-set's own edits to SPIKE_PLAN and ADR 0005 moved some of them. Each cited line is mapped from c94c81b to the
# working tree with a line diff and rewritten. Run from the repository root: python3 -I .checkpoints/cs0-model-rules/renumber-cites.py [--write]
import difflib, re, subprocess, sys
FILES = {'0005': 'docs/adr/0005-content-abilities-assets.md', 'SP': 'docs/SPIKE_PLAN.md', '0003': 'docs/adr/0003-rendering.md',
         '0002': 'docs/adr/0002-code-structure-build-toolchain.md'}
maps = {}
for key, path in FILES.items():
    old = subprocess.run(['git', 'show', 'c94c81b:' + path], capture_output=True, text=True).stdout.split('\n')
    new = open(path, encoding='utf-8').read().split('\n')
    m = {}
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, old, new, autojunk=False).get_opcodes():
        for k in range(i1, i2):
            m[k + 1] = (j1 + (k - i1) + 1) if tag == 'equal' else (min(j1 + (k - i1), max(j2 - 1, j1)) + 1)
    maps[key] = m
changed = []
def fix_run(key, run):  # "1777-1781, 1582-1583": every number of at least three digits is a line of that file
    def one(mo):
        n = int(mo.group(0)); nn = maps[key].get(n, n)
        if nn != n: changed.append((key, n, nn))
        return str(nn)
    return re.sub(r'\d{3,4}', one, run)
PAT = re.compile(r'(?P<key>0005|SP|0003|0002):(?P<run>\d{3,4}(?:[-–]\d{3,4})?(?:, \d{3,4}(?:[-–]\d{3,4})?)*)')
LONG = re.compile(r'(?P<path>`docs/(?:adr/0005-content-abilities-assets|SPIKE_PLAN)\.md`):(?P<run>\d{3,4}(?:[-–]\d{3,4})?)')
for f in ['docs/research/asset-export-rules.md', 'docs/research/soldier-joint-list.md']:
    t = open(f, encoding='utf-8').read()
    t2 = PAT.sub(lambda mo: mo.group('key') + ':' + fix_run(mo.group('key'), mo.group('run')), t)
    t2 = LONG.sub(lambda mo: mo.group('path') + ':' + fix_run('0005' if '0005' in mo.group('path') else 'SP', mo.group('run')), t2)
    if '--write' in sys.argv and t2 != t: open(f, 'w', encoding='utf-8').write(t2)
    print(f, 'changed' if t2 != t else 'unchanged')
for c in sorted(set(changed)): print(' ', c[0], c[1], '->', c[2])
