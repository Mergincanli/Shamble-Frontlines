#!/usr/bin/env python3
"""Task 2b (fixcheck-d3c): 'step n' references to the drafts' Brave checklists, at a ref: every cited step must exist
in the named checklist. Prints each reference with the step's first words, for a read of the meaning.
Usage: steps.py <ref>"""
import re, subprocess, sys
REPO = '/home/user/Shamble-Frontlines'; ref = sys.argv[1] if len(sys.argv) > 1 else 'HEAD'
ADRS = {'0001': 'docs/adr/0001-multiplayer-netcode.md', '0002': 'docs/adr/0002-code-structure-build-toolchain.md',
        '0003': 'docs/adr/0003-rendering.md', '0004': 'docs/adr/0004-simulation-core.md',
        '0005': 'docs/adr/0005-content-abilities-assets.md', '0006': 'docs/adr/0006-infrastructure-operations.md'}
DOCS = list(ADRS.values()) + ['docs/ENGINE_OVERVIEW.md', 'docs/SPIKE_PLAN.md', 'docs/adr/README.md']
show = lambda p: subprocess.run(['git', '-C', REPO, 'show', f'{ref}:{p}'], capture_output=True, text=True).stdout
CL = {}  # (adr, name) -> {n: text}
for n, d in ADRS.items():
    L = show(d).split('\n'); cur = None
    for l in L:
        m = re.match(r'^#### The (.+?) checklist', l)
        if m: cur = (n, m.group(1).lower()); CL[cur] = {}; continue
        if l.startswith('#'): cur = None; continue
        if cur:
            mm = re.match(r'^(\d+)\. (.*)', l)
            if mm: CL[cur][int(mm.group(1))] = mm.group(2)[:70]
names = {k[1] for k in CL}
print('checklists:', {f'{a} {b}': len(v) for (a, b), v in CL.items()})
bad = 0; tot = 0
for d in DOCS:
    t = show(d); own = re.search(r'/(000\d)-', d); own = own.group(1) if own else None
    flat = re.sub(r'\s*\n\s*', ' ', t)
    for m in re.finditer(r"\b(?:the )?(" + '|'.join(re.escape(x) for x in sorted(names, key=len, reverse=True)) +
                         r")(?: checklist)?(?:'s)?,? \(?(?:its )?steps? (\d+)((?:(?:,| and| to|–|, and) \d+)*)", flat, re.I):
        name = m.group(1).lower()
        pre = flat[max(0, m.start() - 40):m.start()]
        adr = re.findall(r'ADR (000\d)', pre); adr = adr[-1] if adr else own
        cands = [k for k in CL if k[1] == name and (adr is None or k[0] == adr)] or [k for k in CL if k[1] == name]
        nums = [int(m.group(2))] + [int(x) for x in re.findall(r'\d+', m.group(3))]
        rng = re.search(r'(\d+)(?: to|–)(\d+)', m.group(2) + m.group(3))
        for k in nums:
            tot += 1; key = cands[0] if cands else None
            ok = key is not None and k in CL[key]
            if not ok: bad += 1
            print(f"{'ok ' if ok else 'BAD'} {d.split('/')[-1]}: '{m.group(0)[:60]}' -> {key} step {k}: "
                  f"{CL[key].get(k, '(none)')[:60] if key else '(no checklist)'}")
print(f'TOTAL step references {tot}, unresolved {bad}')
# migration-plan steps: 'migration step n', 'migration plan, step n', 'the draft's migration step n'
MIG = {}
for n, d in ADRS.items():
    t = show(d); sec = t.split('\n## Migration plan')[1].split('\n## ')[0] if '\n## Migration plan' in t else ''
    MIG[n] = {int(x.group(1)): x.group(2)[:60] for x in re.finditer(r'^(\d+)\. (.*)', sec, re.M)}
print('migration plans:', {k: (min(v) if v else None, max(v) if v else None) for k, v in MIG.items()})
mb = mt = 0
for d in DOCS:
    t = show(d); own = re.search(r'/(000\d)-', d); own = own.group(1) if own else None
    flat = re.sub(r'\s*\n\s*', ' ', t)
    for m in re.finditer(r"migration (?:plan,? )?steps? (\d+)((?:(?:,| and| to|–) \d+)*)", flat):
        pre = flat[max(0, m.start() - 60):m.start()]
        adr = re.findall(r'ADR (000\d)', pre); adr = adr[-1] if adr else own
        for k in [int(m.group(1))] + [int(x) for x in re.findall(r'\d+', m.group(2))]:
            mt += 1; ok = adr in MIG and k in MIG[adr]
            if not ok: mb += 1
            print(f"{'ok ' if ok else 'BAD'} {d.split('/')[-1]}: '{flat[max(0,m.start()-30):m.end()][:80]}' -> ADR {adr} migration step {k}: {MIG.get(adr, {}).get(k, '(none)')[:55]}")
print(f'TOTAL migration-step references {mt}, unresolved {mb}')
