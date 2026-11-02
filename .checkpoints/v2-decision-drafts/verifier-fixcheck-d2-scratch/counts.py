#!/usr/bin/env python3
"""Count, at a git ref, per draft: OQ ids defined (bold headings **OQ-000N-..**), PM rows (| PM-000N-n |), and print
README's totals line and per-draft cells. Usage: counts.py <ref>"""
import re, subprocess, sys
ref = sys.argv[1]
show = lambda p: subprocess.run(['git','-C','/home/user/Shamble-Frontlines','show',f'{ref}:{p}'],capture_output=True,text=True).stdout
names = {'0001':'0001-multiplayer-netcode','0002':'0002-code-structure-build-toolchain','0003':'0003-rendering',
         '0004':'0004-simulation-core','0005':'0005-content-abilities-assets','0006':'0006-infrastructure-operations'}
tot_oq = tot_pm = 0
for d, n in names.items():
    t = show(f'docs/adr/{n}.md')
    oqs = sorted(set(re.findall(r'\*\*(OQ-%s-[A-Z]*\d+)[.*]' % d, t)), key=lambda x: (re.sub(r'\d+$','',x), int(re.search(r'\d+$',x).group())))
    pms = sorted(set(int(x) for x in re.findall(r'^\| PM-%s-(\d+) \|' % d, t, re.M)))
    gaps = [i for i in range(1, max(pms)+1) if i not in pms] if pms else []
    tot_oq += len(oqs); tot_pm += len(pms)
    print(d, 'OQ', len(oqs), oqs[:1], '…', oqs[-1:], '| PM', len(pms), 'max', max(pms) if pms else 0, 'gaps', gaps)
print('total OQ', tot_oq, 'PM', tot_pm)
R = show('docs/adr/README.md')
for l in R.split('\n'):
    if l.startswith('Total:') or re.match(r'^\| \[000\d\]', l): print('README>', l[-160:])
