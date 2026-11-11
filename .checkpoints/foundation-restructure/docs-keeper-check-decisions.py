#!/usr/bin/env python3
"""docs-keeper, foundation 2.1a: check docs/DECISIONS.md against bb2d817 and the report's Appendix A.
Independent of the generator: its own appendix reader, its own diff. Exit 1 on any failure."""
import difflib, re, subprocess, sys

REPO = '/home/user/Shamble-Frontlines'
fails = []
def check(ok, msg):
    print(('ok   ' if ok else 'FAIL ') + msg)
    if not ok: fails.append(msg)

def git(*a):
    return subprocess.run(['git', '-C', REPO, *a], capture_output=True, check=True).stdout.decode('utf-8')

# 1. The appendix, read line by line (no regex over the whole file).
src = {}
cur = None; inside = False; buf = []
for line in open(REPO + '/docs/reports/2026-11-11-foundation-2-1a.md', encoding='utf-8').read().split('\n'):
    if not inside and line.startswith('### A'):
        cur = line[4:].split('.', 2)
        cur = cur[0] + ('.' + cur[1] if cur[1][:1].isdigit() else '')
        continue
    if not inside and line == '```text' and cur:
        inside = True; buf = []; continue
    if inside and line == '```':
        src[cur] = buf; inside = False; cur = None; continue
    if inside: buf.append(line)
check(sorted(src) == ['A1.1', 'A1.2', 'A1.3', 'A2.1', 'A2.2', 'A2.3', 'A3.1', 'A3.2', 'A3.3', 'A4'], 'appendix: 10 blocks ' + ','.join(sorted(src)))  # A3.3 added in the fix pass (his 14:54 message)
check(src['A3.3'][0] == 'My words of 2026-11-11, for the foundation chat (PR #50, foundation step 2.1a).', 'A3.3 is his 14:54 message')
k = src['A3.1'].index('Two additions, my words of 2026-11-11:')
src['A3.1 (his labelled block)'] = src['A3.1'][k:]
del src['A3.1']
sent = 'CLAUDE.md keeps its wording until the kit unfreezes at the gate step (2.3); this clarification goes into it then.'
check(src['A3.2'][-1].endswith(' ' + sent), 'Queued sentence is the end of A3.2\'s last line, byte for byte')
src['Queued: A3.2 sentence'] = [sent]
check(len('\n'.join(src['A2.3'])) == 848, 'A2.3 is 848 characters (the chat record\'s count), got %d' % len('\n'.join(src['A2.3'])))
msg = git('show', '-s', '--format=%B', 'bb2d817').split('\n')
quoted = [l[2:] if l.startswith('> ') else '' for l in msg if l.startswith('>')]
check(quoted == src['A4'], 'A4 equals bb2d817\'s message\'s quoted lines, byte for byte')

# 2. The diff against bb2d817, line by line.
old = git('show', 'bb2d817:docs/DECISIONS.md').split('\n')
new = open(REPO + '/docs/DECISIONS.md', encoding='utf-8').read().split('\n')
def cut_italic(ls, name):
    a = [i for i, l in enumerate(ls) if l.startswith('_How it was made:')]
    z = [i for i, l in enumerate(ls) if l.endswith('Line numbers in the blocks read at their own dates._')]
    check(len(a) == 1 and len(z) == 1 and a[0] < z[0], name + ': one italic "How it was made" paragraph (lines %s-%s)' % (a[0] + 1, z[0] + 1))
    return ls[:a[0]] + ['<italic>'] + ls[z[0] + 1:], ls[a[0]:z[0] + 1]
old_c, old_it = cut_italic(old, 'bb2d817')
new_c, new_it = cut_italic(new, 'this head')
check(old_it != new_it, 'the italic paragraph is reworded (%d lines -> %d lines)' % (len(old_it), len(new_it)))
sm = difflib.SequenceMatcher(None, old_c, new_c, autojunk=False)
deleted, inserted = [], []  # inserted: list of hunks (list of lines)
for tag, i1, i2, j1, j2 in sm.get_opcodes():
    if tag in ('delete', 'replace'): deleted += old_c[i1:i2]
    if tag in ('insert', 'replace'): inserted.append(new_c[j1:j2])
check(deleted == [], 'no old line deleted or changed apart from the italic paragraph (%d)' % len(deleted))
check(len(old_c) == sum(t[2] - t[1] for t in sm.get_opcodes() if t[0] == 'equal'),
      'every other old line is unchanged and in order (%d of %d)' % (len(old_c) - 1, len(old) - len(old_it)))
inserted.append(new_it)

# 3. Classify every inserted line.
counts = {'quote': 0, 'header': 0, 'pointer': 0, 'italic': 0}
runs = []  # (indent, [stripped lines])
bad = []
HEAD_STARTS = ('- **Taken by Mustafa, 2026-11-11', '- **Taken by Mustafa, 2026-11-10', '  - The CLAUDE.md clarification',
               '  - His decision (A1.1):', '  - His follow-up in the same chat', '  - The task as he gave it there')
for hunk in inserted:
    mode = None; run = None
    for l in hunk:
        m = re.match(r'^( *)>( (.*))?$', l)
        if m:
            ind = m.group(1); body = m.group(3) if m.group(2) is not None else ''
            if m.group(2) is None and l != ind + '>': bad.append(l)
            if run is None or run[0] != ind: run = (ind, []); runs.append(run)
            run[1].append(body); counts['quote'] += 1; continue
        run = None
        s = l.lstrip()
        if s.startswith('_How it was made:'): mode = 'italic'
        elif s.startswith('_Pointer') or s.startswith('- _Pointer'): mode = 'pointer'
        elif l.startswith(HEAD_STARTS): mode = 'header'
        elif mode is None or (mode != 'italic' and not l.startswith(' ')):
            bad.append(l); continue
        counts[mode] += 1
check(not bad, 'every inserted line is a quote, header, pointer or the italic line (unclassified: %d)' % len(bad))
for b in bad: print('     unclassified:', b[:100])
print('     inserted lines by kind:', counts)

# 4. Every quote run equals exactly one source block, and every source block is used once.
used = {}
for ind, body in runs:
    hit = [name for name, lines in src.items() if lines == body]
    check(len(hit) == 1, 'quote run (indent %d, %d lines) equals one source block byte for byte: %s' % (len(ind), len(body), hit))
    for h in hit: used[h] = used.get(h, 0) + 1
check(sorted(used) == sorted(src) and all(v == 1 for v in used.values()), 'each source block inserted exactly once: ' + str(used))

# 5. Pointers sit beside quotes, never inside: no pointer line starts with ">", and each is italic.
ptr = [l for h in inserted for l in h if l.lstrip().startswith(('_Pointer', '- _Pointer'))]
check(len(ptr) == 3, 'three pointer lines in DECISIONS.md (%d)' % len(ptr))
print('RESULT:', 'PASS' if not fails else 'FAIL (%d)' % len(fails))
sys.exit(1 if fails else 0)
