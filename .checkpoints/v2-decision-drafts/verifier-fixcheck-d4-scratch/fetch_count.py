# Count fetch-audit rows (table rows or audit lines) per verifier/skeptic file; distinct host+path where shown.
import re, glob, os
D = '/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/'
files = sorted(glob.glob(D + 'verifier-*-verdicts.md') + glob.glob(D + 'verifier-skeptic-*-results.md') + glob.glob(D + 'verifier-skeptic-6-*.md') + [D + 'verifier-completeness-findings.md', D + 'verifier-fixcheck-d3-critic.md'])
tot_rows = 0; urls = set()
for f in files:
    L = open(f).read().split('\n'); rows = 0; inside = False
    for i, l in enumerate(L):
        if re.match(r'^#+ .*[Ff]etch audit', l) or re.match(r'^\*\*Fetch audit', l):
            inside = True; continue
        if inside and re.match(r'^#+ ', l):
            inside = False
        if inside and l.startswith('|') and not re.match(r'^\|\s*:?-{3,}', l) and not re.search(r'\|\s*(id|#|time|host)\s*\|', l.lower()[:40]):
            rows += 1
            m = re.search(r'(https?://[^ |)`]+)', l)
            if m: urls.add(m.group(1).split('#')[0].rstrip('.,;'))
            else:
                c = [x.strip() for x in l.strip('|').split('|')]
                hp = [x for x in c if re.match(r'^[a-z0-9.-]+\.[a-z]{2,}$', x)]
                pp = [x for x in c if x.startswith('/')]
                if hp and pp: urls.add(hp[0] + pp[0].split(' ')[0])
    if rows: print(f'{os.path.basename(f):45s} audit rows {rows}')
    tot_rows += rows
a = [l for l in open(D + 'verifier-fixcheck-d3e-scratch/fetch-audit.txt').read().split('\n') if l.strip()]
print('d3e fetch-audit.txt lines', len(a)); tot_rows += len(a)
for l in a: urls.add(l.split('|')[2].split(' ')[0])
print('total audit rows', tot_rows, '; distinct URLs (where the row names one)', len(urls))
