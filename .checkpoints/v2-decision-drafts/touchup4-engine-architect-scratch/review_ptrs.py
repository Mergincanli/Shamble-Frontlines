#!/usr/bin/env python3
"""Every "the X review('s) finding/question n" pointer in the nine docs: does review-<X>.md (step 10) hold row n / Qn?
Usage: review_ptrs.py <repo root>"""
import re, glob, sys, os
root = sys.argv[1]; ck = root + '/.checkpoints/v2-decision-drafts/'
name = {'art': 'art-director', 'engine-architect': 'engine-architect', 'game-designer': 'game-designer', 'graphics': 'graphics-engineer',
        'infra': 'infra-engineer', 'netcode': 'netcode-architect', 'perf': 'perf-auditor', 'Rust': 'rust-reviewer',
        'security': 'security-auditor', 'test': 'test-engineer', 'design': 'game-designer'}
files = sorted(glob.glob(root + '/docs/adr/000[1-6]-*.md')) + [root + '/docs/adr/README.md', root + '/docs/SPIKE_PLAN.md', root + '/docs/ENGINE_OVERVIEW.md']
ok = bad = other = 0
for f in files:
    T = open(f, encoding='utf8').read().replace('\n', ' ')
    for m in re.finditer(r'(\S+) review(?:\'s|s\'|,)? ?,? (finding|findings|question|Q)s? ?(\d+)', T):
        who, kind, n = m.group(1), m.group(2), m.group(3)
        if who not in name: other += 1; print('OTHER', os.path.basename(f), m.group(0)); continue
        R = open(ck + 'review-' + name[who] + '.md', encoding='utf8').read()
        hit = re.search(r'^\| ' + n + r' \|', R, re.M) if kind.startswith('finding') else re.search(r'\*\*Q' + n + r'\b', R)
        if hit: ok += 1
        else: bad += 1; print('MISS', os.path.basename(f), m.group(0))
print(f'pointers to the ten step-10 reviews: {ok} found, {bad} missing; other reviews: {other}')
