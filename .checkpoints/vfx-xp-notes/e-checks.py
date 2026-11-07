# vfx-xp-notes: read-only checks E1-E4 (plan revision 1 §6; the advisor's conditions 1 and 6).
# Run from the worktree root: python3 -I .checkpoints/vfx-xp-notes/e-checks.py [baseRef]. Exit 0 = PASS.
import difflib, re, subprocess, sys
BASE = sys.argv[1] if len(sys.argv) > 1 else '4c6db5b'
fails = []
def show(spec): return subprocess.run(['git', 'show', spec], capture_output=True, text=True, check=True).stdout
def read(p): return open(p, encoding='utf-8').read()
def joined(t):
    t = re.sub(r'`\[(DECIDED|DEFAULT|OPEN|LATER|IDEA)\]` ', '', t)
    t = re.sub(r'\n[ \t]*(?:[-*] |\d+\. )?', ' ', t)
    return re.sub(r' +', ' ', t)
def check(c, label):
    print(('PASS ' if c else 'FAIL ') + label)
    if not c: fails.append(label)

words = read('.checkpoints/vfx-xp-notes/mustafa-words.md').split('```text\n')
vfx = words[1].split('\n```')[0].split('\n')
xp = words[2].split('\n```')[0].split('\n')
ed, vi = read('docs/EDITOR_V2.md'), read('docs/VISION_V2.md')
jed, jvi = joined(ed), joined(vi)

print('== E1: his words byte for byte')
for line in vfx[1:]:
    check(jed.count(line[2:]) == 1, 'VFX line once in EDITOR_V2: ' + line[2:45])
check('a VFX editor' in vfx[0] and jed.count("**A VFX editor**, as a future tool, after Editor v2's map features") == 1,
      'VFX opening: "A VFX editor" (one capital letter), then his words, once')
for piece in ["as a future tool, after Editor v2's map features", 'Decided principles, design later']:
    check(piece in vfx[0] and jed.count(piece) == 1, 'VFX opening piece once: ' + piece)
check('n progression in private (rented) matches' in xp[0] and jvi.count('n progression in private (rented) matches') == 1,
      'XP header words once in VISION_V2 (one capital letter)')
check(jvi.count(xp[1][len('- [DECIDED] '):]) == 1, 'XP rule once in VISION_V2')
check(jvi.count(xp[2][2:]) == 1, 'XP open line once in VISION_V2, "Open," kept')

print('== E2: nothing else changed')
for path, allowed in [('docs/EDITOR_V2.md', 2), ('docs/VISION_V2.md', 2)]:
    old = show(f'{BASE}:{path}').split('\n'); new = read(path).split('\n')
    ops = [o for o in difflib.SequenceMatcher(a=old, b=new, autojunk=False).get_opcodes() if o[0] != 'equal']
    print('  ', path, [(o[0], o[1] + 1, o[2], o[3] + 1, o[4]) for o in ops])
    ok = len(ops) == allowed and all(o[0] == 'insert' or (o[0] == 'replace' and o[1] == 6 and o[2] == 7) for o in ops)
    check(ok, f'{path}: only {allowed} block(s): inserts, and for VISION_V2 its header line :7 rewritten')
check([l for l in show(f'{BASE}:docs/EDITOR_V2.md').split('\n') if l.startswith('#')] ==
      [l for l in ed.split('\n') if l.startswith('#')][:-1], 'EDITOR_V2 headings unchanged, §7 added last')
rd = subprocess.run(['git', 'diff', '--numstat', BASE, '--', 'docs/adr/README.md'], capture_output=True, text=True).stdout.split()
check(rd[:2] == ['5', '5'], 'README: five changed lines (the five clauses)')

print('== E3: tag counts')
for path, exp in [('docs/EDITOR_V2.md', {'DECIDED': 10, 'LATER': 1}), ('docs/VISION_V2.md', {'DECIDED': 1, 'OPEN': 1})]:
    old, new = show(f'{BASE}:{path}'), read(path)
    for t in ['DECIDED', 'DEFAULT', 'OPEN', 'LATER', 'IDEA']:
        a, b = old.count(f'`[{t}]`'), new.count(f'`[{t}]`')
        check(b - a == exp.get(t, 0), f'{path} [{t}] {a} -> {b} (expected {exp.get(t, 0):+d})')

print('== E4: line citations into EDITOR_V2 and VISION_V2 (to fix or record)')
out = subprocess.run(['git', 'grep', '-n', '-E', r'(EDITOR|VISION)_V2(\.md)?:[0-9]', '--', 'docs', '.claude', 'CLAUDE.md'],
                     capture_output=True, text=True).stdout.strip().split('\n')
for c in out:
    if c and not c.startswith('docs/reports/'): print('   ', c[:150])
print('RESULT:', 'PASS' if not fails else f'FAIL ({len(fails)})')
sys.exit(1 if fails else 0)
