# B.2's read-only checks E1-E4 (plan revision 1 §5, the advisor's conditions 1, 3, 8 and 9).
# Run from the worktree root: python3 -I .checkpoints/b2-community-servers/e-checks.py [baseRef]
# It reads files and git objects only; it writes nothing. Exit 0 = PASS, 1 = a check failed.
import difflib, re, subprocess, sys

BASE = sys.argv[1] if len(sys.argv) > 1 else 'ee736ec'
fails = []

def git_show(ref_path):
    return subprocess.run(['git', 'show', ref_path], capture_output=True, text=True, check=True).stdout

def read(path):
    with open(path, encoding='utf-8') as f:
        return f.read()

def joined(text):
    # Join wrapped lines with one space and drop list markers and indentation, as E1 did in B3 and B.1.
    t = re.sub(r'\n[ \t]*(?:[-*] |\d+\. )?', ' ', text)
    return re.sub(r' +', ' ', t)

def check(cond, label):
    print(('PASS ' if cond else 'FAIL ') + label)
    if not cond:
        fails.append(label)

old = git_show(f'{BASE}:docs/VISION_V2.md')
new = read('docs/VISION_V2.md')
loop = read('docs/LOOP_V2.md')
words_b2 = git_show('4f268da:.checkpoints/kit-v3-3/queue/mustafa-2026-11-03-list.md').split('\n')[11]
walk = git_show('c5d86cb:.checkpoints/b3-design-walk/mustafa-2026-11-04.md')
appendix = read('docs/reports/2026-11-01-v2-decision-drafts-appendix.md').split('\n')[2217]
jn, jloop = joined(new), joined(loop)

print('== E1: his words byte for byte')
b2_pieces = [
    'Community servers (later)',
    'bigger or longer matches, community growth, a safety net if the game grows faster than our funds, possible '
    'revenue such as renting out official private servers',
    'progression counts on official servers only',
    "Counter-Strike and Rust community servers, Battlefield's rented servers, Minecraft Realms",
]
for p in b2_pieces:
    check(p in words_b2, f'B.2 piece is in his line of 2026-11-03 (4f268da queue file, line 12): {p[:50]}')
    check(jn.count(p) == 1, f'B.2 piece found exactly once in VISION_V2: {p[:50]}')
factions = ('Factions: the counter triangle and tier one stay shared; the differences grow with tech (tier-two '
            'specialists and vehicles are unique).')
check(factions in walk, 'Factions sentence is in his walk of 2026-11-04 (c5d86cb words file)')
check(factions in jloop, 'Factions sentence matches LOOP_V2 §5 (joined)')
check(jn.count(factions) == 1, 'Factions sentence found exactly once in VISION_V2 (C3: his sentence only)')
b3q2 = ("Account progression gives class variants and cosmetics only. New classes come through the team's tech in "
        'the match, so nobody is ever missing a counter.')
check(b3q2 in walk, 'B3-Q2 (a) sentence is in his walk answers (c5d86cb words file)')
check(b3q2 in jloop, 'B3-Q2 (a) sentence matches LOOP_V2 §8 (joined)')
check(jn.count(b3q2) == 1, 'B3-Q2 (a) sentence found exactly once in VISION_V2')
door = "keep the door open, don't design it"
check(door in appendix, 'COMMUNITY-SERVERS quote matches the decision-drafts appendix line 2218')
check(jn.count(door) == 1, 'COMMUNITY-SERVERS quote found exactly once in VISION_V2')
check('infantry shared, tanks per faction' not in new, 'the stale LOOP_V2 quote is gone from VISION_V2')
check('drafts keep' not in jn and 'keep possible' not in jn, 'C1: the note does not say the drafts keep anything')

print('== E2: nothing else changed in VISION_V2')
ol, nl = old.split('\n'), new.split('\n')
ops = [op for op in difflib.SequenceMatcher(a=ol, b=nl, autojunk=False).get_opcodes() if op[0] != 'equal']
allowed = []
for tag, i1, i2, j1, j2 in ops:
    o, n = ol[i1:i2], nl[j1:j2]
    print(f'  {tag} old {i1 + 1}-{i2} -> new {j1 + 1}-{j2}')
    if tag == 'insert' and i1 == 5 and n[0].startswith('Updated on 2026-11-06'):
        allowed.append('header line')
    elif tag == 'replace' and o and o[0].lstrip().startswith('- `[OPEN]` How much is faction-specific?'):
        allowed.append('pillar 5 replaced')
    elif tag == 'replace' and len(o) == 1 and joined('\n'.join(n)).startswith(o[0]):
        # One old line kept whole as the start of its new bullet (a pointer appended); maybe a block after it.
        rest = joined('\n'.join(n))[len(o[0]):]
        allowed.append('pointer on old line ' + str(i1 + 1) + (' + note' if 'Community servers (later)' in rest else ''))
    elif tag == 'insert' and any('Community servers (later)' in x for x in n):
        allowed.append('the note')
    elif tag == 'insert' and n[0].startswith('  (') and ol[i1 - 1].startswith('- '):
        # A pointer that starts on a new continuation line of the old bullet just above it (its words unchanged).
        allowed.append('pointer on old line ' + str(i1))
    else:
        allowed.append(None)
check(None not in allowed, 'every change is one of: header line, pillar 5, a pointer on an old line, the note')
print('  classified:', allowed)
check(len(ops) == 5, 'exactly five change blocks (header, pillar 5, line 59 + the note, line 86)')
heads = lambda t: [l for l in t.split('\n') if l.startswith('#')]
check(heads(old) == heads(new), 'every heading unchanged and in order')
for ln in (59, 86):
    check(ol[ln - 1] in new, f'old line {ln} kept whole in the new file')

print('== E3: tag counts')
expected = {'DECIDED': 2, 'DEFAULT': 0, 'OPEN': -1, 'LATER': 1, 'IDEA': 0}
for t, d in expected.items():
    a, b = old.count(f'`[{t}]`'), new.count(f'`[{t}]`')
    check(b - a == d, f'[{t}] {a} -> {b} (expected {d:+d})')

print('== E4: links and citations')
secs = re.findall(r'^## (\d+)\.', new, re.M)
check(secs == [str(i) for i in range(1, 8)], 'sections 1-7 in order')
s6 = new.split('## 6.')[1].split('## 7.')[0]
check(re.findall(r'^(\d+)\. ', s6, re.M) == [str(i) for i in range(1, 11)], '§6 items 1-10 in order')
s7 = new.split('## 7.')[1]
check('**Faction asymmetry:**' in s7, '§7 still holds the faction question (README OQ-0005-CD5 owner, ADR 0005:686)')
readme = read('docs/adr/README.md')
check('the faction design session (VISION_V2 §7, faction asymmetry; Mustafa)' in readme, 'CD5 owner cell intact')
check('**OQ-0006-21.' in read('docs/adr/0006-infrastructure-operations.md'), 'ADR 0006 holds OQ-0006-21')
row = [l for l in readme.split('\n') if l.startswith('| OQ-0006-21 ')][0]
check('No option is weighed and no timing is set. His note in VISION_V2 §5' in row, 'C8: clause appended, old text kept')
rd = subprocess.run(['git', 'diff', '--numstat', BASE, '--', 'docs/adr/README.md'], capture_output=True,
                    text=True).stdout.split()
check(rd[:2] == ['1', '1'], 'C8: README has one changed line')
cites = subprocess.run(['git', 'grep', '-n', '-E', r'VISION_V2(\.md)?:[0-9]', '--', 'docs', '.claude', 'CLAUDE.md'],
                       capture_output=True, text=True).stdout.strip().split('\n')
cites = [c for c in cites if c]
outside = [c for c in cites if not c.startswith(('docs/STATUS.md:', 'docs/ROADMAP.md:'))]
print('  line-number citations into VISION_V2 outside STATUS and ROADMAP (C9):')
for c in outside:
    print('   ', c[:150])
check(all(c.startswith('docs/research/r1-simulation-core.md:') for c in outside),
      'C9: the only ones outside STATUS and ROADMAP are the research record\'s (pre-existing, left unedited)')

print('RESULT:', 'PASS' if not fails else f'FAIL ({len(fails)})')
sys.exit(1 if fails else 0)
