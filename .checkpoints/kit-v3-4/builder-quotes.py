# Read-only: every quote of Mustafa's words placed by the docs parts (F, D) is found byte for byte in his source
# (joined text) and in each target file (joined text). Run from the worktree root.
import re, sys
def joined(path, lo=None, hi=None, strip_quote=False):
    lines = open(path, encoding='utf-8').read().split('\n')
    if lo: lines = lines[lo-1:hi]
    out = []
    for l in lines:
        l = l.strip()
        if strip_quote and l.startswith('>'): l = l[1:].strip()
        out.append(l)
    return ' '.join(out)
ans = joined('.checkpoints/kit-v3-4/mustafa-2026-11-07-answers.md', 9, 9)
a4 = joined('docs/STATUS.md', *[None, None], strip_quote=True)
# his A4 words: locate the A4 block in STATUS by its first quoted line
st = open('docs/STATUS.md', encoding='utf-8').read().split('\n')
i = next(n for n, l in enumerate(st) if '> A4, a small kit change-set after A3' in l)
a4 = joined('docs/STATUS.md', i+1, i+16, strip_quote=True)
XP = ('all account progression (the account level and Service points), never the in-match weapon XP: amps reset every '
      'match, so there\'s nothing to farm, and cutting them would only make practice with bots feel worse.')
checks = [
 (ans, XP, ['docs/VISION_V2.md', 'docs/STATUS.md']),
 (ans, 'XP-TERM: (a), ' + XP, ['docs/STATUS.md']),
 (ans, 'all account progression (the account level and Service points), never the in-match weapon XP',
  ['docs/adr/README.md', 'docs/STATUS.md']),
 (ans, 'LOOK: (a).', ['docs/STATUS.md']),
 (a4, 'fast checks on save (format, type checks for Rust and TypeScript), the full gate in CI', ['docs/SPIKE_PLAN.md']),
 (a4, 'next to the fast checks on save', ['docs/SPIKE_PLAN.md']),
 (a4, 'next to the server metrics already planned', ['docs/adr/0006-infrastructure-operations.md']),
 (a4, "from the first playtests on, error reports and telemetry from players' browsers, next to the server metrics "
      "already planned", ['docs/adr/0006-infrastructure-operations.md']),
 (a4, "from the first playtests on, error reports and telemetry from players' browsers", ['docs/adr/README.md']),
 (a4, 'one-click rollback of client deploys and of server versions',
  ['docs/adr/README.md', 'docs/adr/0006-infrastructure-operations.md']),
 (a4, 'first playtests', ['docs/adr/README.md', 'docs/adr/0006-infrastructure-operations.md']),
]
fails = 0
for src, q, targets in checks:
    ok = q in src
    for t in targets:
        tj = joined(t)
        n = tj.count('"' + q + '"') + tj.count('"' + q[:-1] + '"' if q.endswith('.') else '\0')
        ok_t = q in tj
        print(('PASS' if ok and ok_t else 'FAIL'), 'src' if ok else 'SRC-MISSING', t, 'quoted x%d' % tj.count(q), '|', q[:60])
        fails += not (ok and ok_t)
# every double-quoted string on added lines that claims him must be in a source: list added quotes for a manual look
print('FAILS', fails); sys.exit(1 if fails else 0)
