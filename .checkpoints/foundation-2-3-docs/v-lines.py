# Verifier (read-only): does each cited line hold "before the foundation" (one line, or wrapped across two)?
# Also lists every such hit in tracked files outside docs/reports/ (the session's notes and .gate excluded).
import subprocess, re
cites = {
 'docs/adr/0002-code-structure-build-toolchain.md': [38, 724, 1129],
 'docs/SPIKE_PLAN.md': [106, 214],
 'docs/adr/0003-simulation-core.md': [(51, 52)],
 'docs/adr/0006-netcode-transport-protocol.md': [(56, 57), 1469, 1482],
 'docs/adr/0001-v2-engine-architecture.md': [603, 1100, 1301, 1771],
 'docs/adr/0004-renderer-gpu-pipeline.md': [1245, 1251],
 'docs/adr/0005-editor-asset-pipeline.md': [92],
 'docs/adr/README.md': [72, 292],
}
files = subprocess.run(['git', 'ls-files', 'docs/adr'], capture_output=True, text=True).stdout.split()
def find(name):
    for f in files:
        if name in f:
            return f
    return name
pat = re.compile(r'before the\s+foundation', re.I)
for f0, lst in cites.items():
    f = f0 if f0 in files or not f0.startswith('docs/adr/0') else find(f0.split('/')[-1][:5])
    L = open(f, encoding='utf-8').read().split('\n')
    for c in lst:
        if isinstance(c, tuple):
            t = (L[c[0]-1].strip() + ' ' + L[c[1]-1].strip())
            ok = bool(pat.search(t)) and not pat.search(L[c[0]-1]) and not pat.search(L[c[1]-1])
            print(f, f'{c[0]}-{c[1]}', 'wrapped hit', ok)
        else:
            print(f, c, 'hit', bool(pat.search(L[c-1])), "preview-step" if "foundation's preview" in L[c-1] else '')
