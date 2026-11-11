# Verifier (final, 2.1a): each fenced block of the report's Appendix A equals its source note block, byte for byte.
# Read-only. Own parser (independent of report-appendix.py and docs-keeper-check-decisions.py).
import subprocess, sys
REPO = '/home/user/Shamble-Frontlines'
N = REPO + '/.checkpoints/foundation-restructure/'

def blocks(path):
    out, inside, buf = [], False, []
    with open(path, 'rb') as fh:
        data = fh.read().decode('utf-8')
    for line in data.split('\n'):
        if not inside and line == '```text':
            inside, buf = True, []; continue
        if inside and line == '```':
            out.append('\n'.join(buf)); inside = False; continue
        if inside: buf.append(line)
    if inside: raise SystemExit('unclosed fence in ' + path)
    return out

rep = blocks(REPO + '/docs/reports/2026-11-11-foundation-2-1a.md')
src = (blocks(N + 'mustafa-2026-11-10.md') + blocks(N + 'mustafa-2026-11-10-answers.md') + blocks(N + 'mustafa-2026-11-11.md')
       + blocks(N + 'mustafa-2026-11-11-f1.md') + blocks(N + 'mustafa-2026-11-11-prompts.md')
       + blocks(N + 'mustafa-2026-11-11-prompts-correction.md'))
msg = subprocess.run(['git', 'show', '-s', '--format=%B', 'bb2d817'], cwd=REPO, capture_output=True, check=True).stdout.decode('utf-8')
a4 = '\n'.join(l[2:] if l.startswith('> ') else '' for l in msg.split('\n') if l.startswith('>'))
src.append(a4)
names = ['A1.1', 'A1.2', 'A1.3', 'A2.1', 'A2.2', 'A2.3', 'A3.1', 'A3.2', 'A3.3', 'A3.4', 'A3.5', 'A4']
print('report blocks', len(rep), 'source blocks', len(src))
bad = 0
for i, n in enumerate(names):
    r = rep[i] if i < len(rep) else None
    s = src[i] if i < len(src) else None
    ok = r is not None and r == s
    bad += not ok
    print(('ok  ' if ok else 'FAIL'), n, 'chars', len(r or ''), len(s or ''))
if len(rep) != 12 or len(src) != 12: bad += 1
print('RESULT', 'PASS' if bad == 0 else 'FAIL (%d)' % bad)
sys.exit(1 if bad else 0)
