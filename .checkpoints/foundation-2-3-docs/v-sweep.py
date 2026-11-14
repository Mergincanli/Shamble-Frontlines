# Verifier (read-only): every "before the foundation" hit (same line or wrapped) in tracked files outside docs/reports/
# and .checkpoints/; prints file:line and whether it is the preview-step class or inside a pointer.
import subprocess, re
files = [f for f in subprocess.run(['git', 'ls-files'], capture_output=True, text=True).stdout.split()
         if f.endswith('.md') and not f.startswith('docs/reports/') and not f.startswith('.checkpoints/')]
pat = re.compile(r'before the\s+foundation', re.I)
for f in files:
    try:
        L = open(f, encoding='utf-8').read().split('\n')
    except Exception:
        continue
    for i, l in enumerate(L):
        nxt = L[i+1].strip() if i + 1 < len(L) else ''
        same = bool(pat.search(l))
        wrapped = (not same) and bool(re.search(r'before the\s*$', l.rstrip(), re.I)) and nxt.lower().startswith('foundation')
        wrapped = wrapped or ((not same) and bool(re.search(r'before\s*$', l.rstrip(), re.I)) and nxt.lower().startswith('the foundation'))
        if same or wrapped:
            seg = (l + ' ' + nxt)
            cls = 'preview-step' if re.search(r"before the\s+foundation's\s+preview", seg, re.I) else ''
            ptr = 'pointer' if '_Pointer' in l or 'pointer, 20' in l else ''
            print(f'{f}:{i+1}{"-"+str(i+2) if wrapped else ""}', cls, ptr)
