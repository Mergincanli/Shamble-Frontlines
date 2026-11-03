#!/usr/bin/env python3
"""Task 2 (fixcheck-d3c): dangling references at a ref, in the nine v2 decision documents.
Classes: OQ ids (with shorthand ', -6' / 'AS1 to AS5'); PM ids (with shorthand ', -40' / 'to -17'); ADR 0005's part
codes CD-n, AB-n, AP-n; OQ-SPIKE-n; 'part n' (and 'part n.x') of a named draft or the GPU section; register codes I-n,
seam S-n, gap G-n, N-n, J-n; decision codes (all-caps hyphenated) against README's decisions table, STATUS, the
research's shared ids and GLOSSARY; research ids against docs/research at 9e7db28 (with shorthand 'REC1, REC3',
'REC1–REC4'); Markdown links (file and #anchor). Prints every unresolved reference and a count per class.
Usage: refs.py <ref>"""
import re, subprocess, sys, collections, os, posixpath
REPO = '/home/user/Shamble-Frontlines'
ref = sys.argv[1] if len(sys.argv) > 1 else 'HEAD'
ADRS = {'0001': 'docs/adr/0001-multiplayer-netcode.md', '0002': 'docs/adr/0002-code-structure-build-toolchain.md',
        '0003': 'docs/adr/0003-rendering.md', '0004': 'docs/adr/0004-simulation-core.md',
        '0005': 'docs/adr/0005-content-abilities-assets.md', '0006': 'docs/adr/0006-infrastructure-operations.md'}
SYN = ['docs/ENGINE_OVERVIEW.md', 'docs/SPIKE_PLAN.md', 'docs/adr/README.md']
DOCS = list(ADRS.values()) + SYN
def show(r, p):
    x = subprocess.run(['git', '-C', REPO, 'show', f'{r}:{p}'], capture_output=True, text=True)
    return x.stdout if x.returncode == 0 else None
T = {d: show(ref, d) for d in DOCS}
# ---------- definitions
D_OQ = set(); D_PM = set(); D_PART = {}; D_SUB = collections.defaultdict(set); D_GPU = set(); D_05 = set()
for n, d in ADRS.items():
    t = T[d]
    D_OQ |= set(re.findall(r'^\s*- \*\*(OQ-%s-[A-Z]*\d+)[.*]' % n, t, re.M))
    D_PM |= set(re.findall(r'^\| *(PM-%s-\d+) *\|' % n, t, re.M))
    L = t.split('\n'); indec = False; cur = None; parts = set()
    for l in L:
        if l.startswith('## '): indec = l.startswith('## Decision'); cur = None
        if indec:
            m = re.match(r'^(\d+)\. \*\*', l)
            if m: cur = m.group(1); parts.add(cur)
            m = re.match(r'^\s+- \*\*([a-z])\. ', l)
            if m and cur: D_SUB[(n, cur)].add(m.group(1))
    D_PART[n] = parts
t5 = T[ADRS['0005']]
D_05 = set(re.findall(r'^- \*\*((?:CD|AB|AP)-\d+)\. ', t5, re.M))
g = t5.split('### What this section decides')[1].split('\n### ')[0]
D_GPU = set(re.findall(r'^(\d+)\. \*\*', g, re.M))
D_OQ |= set(re.findall(r'^- \*\*(OQ-SPIKE-\d+)\.', T['docs/SPIKE_PLAN.md'], re.M))
R = T['docs/adr/README.md']
dec = R.split('## Decisions the drafts ask for')[1].split('\n## ')[0]
D_CODE = set()
for l in dec.split('\n'):
    if l.startswith('| ') and not l.startswith('| Code') and not l.startswith('|---'):
        c = l.split('|')[1]
        D_CODE |= set(re.findall(r'\b[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)+\b', c))
REG = collections.defaultdict(set)
codes = R.split('## Codes the drafts cite')[1]
for l in codes.split('\n'):
    m = re.match(r'^\| ([IJNSG])-?(\d+) \|', l)
    if m: REG[m.group(1)].add(m.group(2))
STATUS = show(ref, 'docs/STATUS.md') or ''
LOCAL = collections.defaultdict(set)
D_README_ROW = set(re.findall(r'^\| *([A-Z][A-Z0-9]*(?:-[A-Z0-9]+)+) *\|', R, re.M))
GLOSS = show(ref, 'docs/research/GLOSSARY.md') or ''
RES = {f: show('9e7db28', f'docs/research/{f}') or '' for f in ['r1-simulation-core.md', 'r2-netcode.md',
       'r3-rendering.md', 'r4-physics-navigation.md', 'r5-toolchain-quality.md', 'r6-infrastructure.md']}
D_RID = set(); M_RID = set()
for t in RES.values():
    D_RID |= set(re.findall(r'\*\*(R[1-6]-[A-Z0-9]+-[A-Z]*\d+)[.*:)]', t))
    D_RID |= set(re.findall(r'^\| *(R[1-6]-[A-Z0-9]+-[A-Z]*\d+) *\|', t, re.M))
    M_RID |= set(re.findall(r'\b(R[1-6]-[A-Z0-9]+-[A-Z]*\d+)\b', t))
for l in open(REPO + '/.checkpoints/v2-decision-drafts/inputs/research-ids.tsv', encoding='utf-8').read().split('\n')[1:]:
    if l.strip(): D_RID.add(l.split('\t')[0])
SHARED = set(x.split('\t')[0] for x in open(REPO + '/.checkpoints/v2-decision-drafts/inputs/research-ids.tsv',
                                               encoding='utf-8').read().split('\n')[1:] if '\tSHARED\t' in x)
print(f'definitions: OQ {len(D_OQ)}, PM {len(D_PM)}, 0005 codes {len(D_05)}, GPU parts {sorted(D_GPU, key=int)}, '
      f'parts {[(n, len(p)) for n, p in D_PART.items()]}, README decision codes {len(D_CODE)}, register '
      f'{ {k: len(v) for k, v in REG.items()} }, research ids defined {len(D_RID)}')
# ---------- references
bad = collections.defaultdict(list); cnt = collections.Counter(); ok = collections.Counter()
def chk(cls, key, good, d, ln, ctx):
    cnt[cls] += 1
    if good: ok[cls] += 1
    else: bad[cls].append(f'{d.split("/")[-1]}:{ln}: {key}  <- "{ctx.strip()[:150]}"')
def lineno(t, pos): return t.count('\n', 0, pos) + 1
for d in DOCS:
    t = T[d]; own = re.search(r'/(000\d)-', d); own = own.group(1) if own else None
    flat = t
    # OQ ids and shorthand
    for m in re.finditer(r'\bOQ-(\d{4}|SPIKE)-([A-Z]*)(\d+)((?:(?:,| and| to| or|–|, and) (?:-?[A-Z]*\d+)(?![\d.]))*)', t):
        base = f'OQ-{m.group(1)}-'; pre = m.group(2)
        chk('OQ', base + pre + m.group(3), base + pre + m.group(3) in D_OQ, d, lineno(t, m.start()), t[max(0, m.start()-60):m.end()+20])
        for s in re.findall(r'(?:,| and| to| or|–|, and) (-?[A-Z]*\d+)', m.group(4)):
            s2 = s.lstrip('-'); p2 = re.match(r'[A-Z]*', s2).group(0)
            if s.startswith('-') or p2 or not pre:
                key = base + (s2 if p2 else pre + s2) if (s.startswith('-') or p2) else None
                if key: chk('OQ', key, key in D_OQ, d, lineno(t, m.start()), t[max(0, m.start()-60):m.end()+20])
    # PM ids and shorthand
    for m in re.finditer(r'\bPM-(\d{4})-(\d+)((?:(?:,| and| to| or|–|, and) -\d+)*)', t):
        base = f'PM-{m.group(1)}-'
        keys = [base + m.group(2)] + [base + s for s in re.findall(r'-(\d+)', m.group(3))]
        for k in keys: chk('PM', k, k in D_PM, d, lineno(t, m.start()), t[max(0, m.start()-40):m.end()+20])
    # ADR 0005 codes
    for m in re.finditer(r'(?<![A-Za-z0-9-])((?:CD|AB|AP)-\d+)\b(?!-)', t):
        chk('0005code', m.group(1), m.group(1) in D_05, d, lineno(t, m.start()), t[max(0, m.start()-60):m.end()+30])
    # register codes
    for m in re.finditer(r'(?<![A-Za-z0-9-])I-(\d+)\b', t):
        chk('I-n', 'I-' + m.group(1), m.group(1) in REG['I'], d, lineno(t, m.start()), t[max(0, m.start()-40):m.end()+20])
    for m in re.finditer(r'\bseams? S-?(\d+)((?:(?:,| and) S-?\d+)*)', t):
        for s in [m.group(1)] + re.findall(r'S-?(\d+)', m.group(2)):
            chk('seam', 'S' + s, s in REG['S'], d, lineno(t, m.start()), t[max(0, m.start()-40):m.end()+20])
    for m in re.finditer(r'\bgaps? G-?(\d+)((?:(?:,| and) G-?\d+)*)', t):
        for s in [m.group(1)] + re.findall(r'G-?(\d+)', m.group(2)):
            chk('gap', 'G' + s, s in REG['G'], d, lineno(t, m.start()), t[max(0, m.start()-40):m.end()+20])
    for m in re.finditer(r'(?<![A-Za-z0-9-])([NJ])-?(\d+)\b(?![-.]\d)', t):
        if m.group(1) == 'N' and not re.search(r'(shared number|\(|; |, )$', t[max(0, m.start()-16):m.start()]): continue
        chk(m.group(1) + '-n', m.group(1) + m.group(2), m.group(2) in REG[m.group(1)], d, lineno(t, m.start()), t[max(0, m.start()-50):m.end()+20])
    # research ids, with shorthand
    for m in re.finditer(r'\b(R[1-6]-[A-Z0-9]+-)([A-Z]*)(\d+)((?:(?:,| and| or|–| to|, and) (?:[A-Z]+\d+)\b(?!-))*)', t):
        base = m.group(1); keys = [base + m.group(2) + m.group(3)]
        keys += [base + s for s in re.findall(r'(?:,| and| or|–| to|, and) ([A-Z]+\d+)\b(?!-)', m.group(4))]
        for k in keys:
            chk('research', k, k in D_RID, d, lineno(t, m.start()), t[max(0, m.start()-30):m.end()+10])
    # parts
    for m in re.finditer(r"(?:(ADR (000[1-6]))(?:'s)?,? )?((?:the GPU section's|its GPU|GPU) )?\bparts? (\d+(?:\.[a-z])?(?:(?:,| and| or| to|–|, and) (?:parts? )?\d+(?:\.[a-z])?)*)", t):
        before = t[max(0, m.start()-220):m.start()]
        if re.search(r"(map's|plan|report's|appendix's|template's|brief's|checklist's|the research's|Recommendations) $", before): continue
        if m.group(3):
            n = '0005gpu'
        elif m.group(2):
            n = m.group(2)
        else:
            mm = re.findall(r'ADR (000[1-6])', before)
            n = mm[-1] if (mm and not own) else own
            if n is None: n = mm[-1] if mm else None
        items = re.findall(r'(\d+)(?:\.([a-z]))?', m.group(4))
        rng = re.findall(r'(\d+)(?: to|–)(?: parts?)? ?(\d+)', m.group(4))
        for p, sub in items:
            if n is None: chk('part', f'part {p} (no draft named)', False, d, lineno(t, m.start()), t[max(0, m.start()-80):m.end()+10]); continue
            if n == '0005gpu': good = p in D_GPU; label = f'GPU part {p}'
            else:
                good = p in D_PART[n] and (not sub or sub in D_SUB[(n, p)])
                # an ADR's bare 'part n' may be its own; accept either the named-before draft or its own
                if not good and not m.group(2) and own and own != n: good = p in D_PART[own] and (not sub or sub in D_SUB[(own, p)])
                label = f'ADR {n} part {p}' + (f'.{sub}' if sub else '')
            chk('part', label, good, d, lineno(t, m.start()), t[max(0, m.start()-80):m.end()+10])
    # decision codes: all-caps hyphenated tokens
    for m in re.finditer(r'(?<![A-Za-z0-9-`/#.])([A-Z][A-Z0-9]*(?:-[A-Z0-9]+)+)\b(?![-`/.]\w)', t):
        k = m.group(1)
        if re.match(r'^(OQ|PM|R[1-6]|ADR|CD|AB|AP|I|S|G|N|J|XT|UTF|SHA|AES|ECDSA|TLS|HTTP|P\d|L\d|DM|EU|US|CC|TCP|UDP|QUIC|MRT|GTAO|IPV\d|GLB|JSON|WASM|CSP|HSTS|CORS|COOP|COEP)(-|$)', k): continue
        pat = r'(?<![A-Za-z0-9-])' + re.escape(k) + r'(?![A-Za-z0-9-])'
        good = k in D_CODE or k in SHARED or re.search(pat, STATUS) is not None or k in D_README_ROW
        how = ''
        if not good:
            # defined in the same document: a bold start, a table first cell, or an option label
            if re.search(r'(\*\*|^\| *(\*\*)?|^\s*- \*\*|\bOption |\boption )' + re.escape(k) + r'\b', t, re.M): good = True; how = 'local'
            elif any(re.search(pat, x) for x in RES.values()): good = True; how = 'research'
        if good and how: LOCAL[how].add((d.split('/')[-1], k))
        chk('code', k, good, d, lineno(t, m.start()), t[max(0, m.start()-40):m.end()+20])
    # Markdown links
    for m in re.finditer(r'\]\(([^)\s]+)\)', t):
        target = m.group(1)
        if re.match(r'^(https?:|mailto:)', target): continue
        path, _, anchor = target.partition('#')
        tp = posixpath.normpath(posixpath.join(posixpath.dirname(d), path)) if path else d
        txt = show(ref, tp)
        if txt is None:
            chk('link', target, False, d, lineno(t, m.start()), t[max(0, m.start()-40):m.end()]); continue
        if anchor:
            slugs = set()
            for h in re.findall(r'^#{1,6} +(.*)$', txt, re.M):
                s = h.strip().lower(); s = re.sub(r'[^\w\- ]', '', s); s = s.replace(' ', '-'); slugs.add(s)
            slugs |= set(re.findall(r'<a (?:name|id)="([^"]+)"', txt))
            chk('link', target, anchor in slugs, d, lineno(t, m.start()), t[max(0, m.start()-40):m.end()])
        else: chk('link', target, True, d, lineno(t, m.start()), '')
print('codes resolved locally or in research:', {k: len(v) for k, v in LOCAL.items()})
for k, v in LOCAL.items(): print('  ', k, sorted(v))
print('per class: checked / resolved')
for c in sorted(cnt): print(f'  {c}: {cnt[c]} / {ok[c]}')
print(f'TOTAL {sum(cnt.values())} references, {sum(ok.values())} resolved, {sum(cnt.values()) - sum(ok.values())} unresolved')
for c in sorted(bad):
    print(f'\n== unresolved {c} ({len(bad[c])})')
    for b in bad[c]: print('  ' + b)
