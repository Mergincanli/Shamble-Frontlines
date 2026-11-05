# verifier fix pass 4: section 5/6 texts of advisor-pass4.md and the queue file's words, word for word in their targets
# (whitespace folded; list markers and quote prefixes '> ' removed). usage: python3 -I verifier-fixpass4-texts.py <a3 root>
import re, sys
root = sys.argv[1]
rd = lambda p: open(f'{root}/{p}', encoding='utf8').read()
adv = rd('.checkpoints/kit-v3-3/advisor-pass4.md').split('\n')
q = rd('.checkpoints/kit-v3-3/queue/mustafa-2026-11-05-a4-b1.md')
status, night = rd('docs/STATUS.md'), rd('.claude/NIGHT_SHIFT.md')
def norm(s):
    s = '\n'.join(re.sub(r'^\s*(> ?)+', '', l) for l in s.split('\n'))
    return re.sub(r'\s+', ' ', s).strip()
def block(a, b):  # 1-based inclusive lines of advisor-pass4.md
    return '\n'.join(adv[a - 1:b])
items = {}
r = block(200, 210); r = re.sub(r'^- ', '', r.strip()); items['R-PLAIN line (s5)'] = (r, status)
s = block(212, 220); s = s[s.index('"') + 1:s.rindex('"')]; items['fix-pass-4 summary (s5)'] = (s, status)
n = block(222, 224); n = n[n.index('** "') + 4:n.rindex('"')]; items['NIGHT_SHIFT step 7 sentence (s5)'] = (n, night)
for k, (a, b) in {'FU-P4a': (235, 239), 'FU-P4b': (240, 246), 'FU-P4c': (247, 252)}.items():
    items[k] = (re.sub(r'^\d+\. ', '', block(a, b).strip()), status)
txt = q[q.index('```text') + 8:q.rindex('```')].strip('\n').split('\n')
items['A4-FREEZE words'] = (txt[0], status)
items['B.1 addition words'] = ('\n'.join(txt[1:]), status)
ns, nn = norm(status), norm(night)
for k, (src, tgt) in items.items():
    t = ns if tgt is status else nn
    ok = norm(src) in t
    print(f"{'FOUND' if ok else 'MISSING'} x{t.count(norm(src))} {k} ({len(norm(src))} chars)")
    if not ok:
        w = norm(src).split(' ')
        for i in range(len(w), 0, -1):
            if ' '.join(w[:i]) in t: print('   matches up to word', i, 'of', len(w), ':', ' '.join(w[max(0, i - 6):i + 6])); break
