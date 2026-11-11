#!/usr/bin/env python3
"""docs-keeper, foundation 2.1a: dated pointer lines beside (never inside) the overtaken lines in ADR 0002 and
SPIKE_PLAN. Idempotent: starts from bb2d817's copies. His sentence is copied by script from the report's A1.1."""
import re, subprocess
REPO = '/home/user/Shamble-Frontlines'
def base(p):
    return subprocess.run(['git', '-C', REPO, 'show', 'bb2d817:' + p], capture_output=True, check=True).stdout.decode('utf-8')

rep_txt = open(REPO + '/docs/reports/2026-11-11-foundation-2-1a.md', encoding='utf-8').read()
a11 = re.search(r'^### A1\.1\. .*?\n\n```text\n(.*?)\n```$', rep_txt, re.S | re.M).group(1)
SENT = 'Steps 2.3 to 2.6 keep their order and wait for their results.'
assert a11.count(SENT) == 1
ENTRY = ('"Taken by Mustafa, 2026-11-10: the second account may plan and build foundation steps 2.1 and 2.2 now"')

def pointer(indent):
    words = ('_Pointer (2026-11-11, foundation 2.1a): for foundation steps 2.1 and 2.2, overtaken by Mustafa\'s decision '
             'of 2026-11-10 (`docs/DECISIONS.md`, the entry ' + ENTRY + '); for the rest, his words there: "' + SENT + '"_').split(' ')
    out, cur = [], indent
    for w in words:
        if len(cur) + len(w) + (0 if cur == indent else 1) > 120:
            out.append(cur); cur = indent + w
        else:
            cur = cur + ('' if cur == indent else ' ') + w
    out.append(cur)
    return out

def insert_after(lines, pred, indent, what):
    hits = [i for i, s in enumerate(lines) if pred(s)]
    assert len(hits) == 1, (what, hits)
    lines[hits[0] + 1:hits[0] + 1] = pointer(indent)
    print(what, 'after line', hits[0] + 1)

# ADR 0002, migration step 1: after the item's last line, inside item 1 (3 spaces).
p = 'docs/adr/0002-code-structure-build-toolchain.md'
L = base(p)[:-1].split('\n')
i1 = [i for i, s in enumerate(L) if s.startswith('1. **The spikes before the foundation**')]
assert len(i1) == 1
assert L[i1[0] + 3] == '   recorded in this record, OQ-0002-3 is answered, and OQ-0002-1 and OQ-0002-4 have their spike results.'
assert L[i1[0] + 4].startswith('2. **The foundation**')
insert_after(L, lambda s: s == '   recorded in this record, OQ-0002-3 is answered, and OQ-0002-1 and OQ-0002-4 have their spike results.', '   ', p + ' step 1')
open(REPO + '/' + p, 'w', encoding='utf-8').write('\n'.join(L) + '\n')

# SPIKE_PLAN: after his quoted SPIKE-ORDER (b) (never inside the quote or the mermaid fence); after the "In words"
# paragraph; after the foundation's "Depends on".
p = 'docs/SPIKE_PLAN.md'
L = base(p)[:-1].split('\n')
k = [i for i, s in enumerate(L) if s == 'three days."']
assert len(k) == 1 and L[k[0] - 5].startswith('The netcode proof test\'s place is settled: it runs before the foundation (SLICE-ORDER, condition 3).')
assert L[k[0] + 1] == '' and L[k[0] + 2] == '```mermaid'
w = [i for i, s in enumerate(L) if s == 'playable, and slice 6\'s 60 Hz margin run on the real sim, which makes the final 60 Hz call (TICK-60-SETUP).']
assert len(w) == 1 and L[w[0] + 1] == '' and any('(1) wave 1 runs one heavy spike' in L[j] for j in range(w[0] - 10, w[0]))
d = [i for i, s in enumerate(L) if s == '  is added) and branch-protection setting.']
assert len(d) == 1 and L[d[0] - 3].startswith('- **Depends on:** the drafts accepted; the boundary spike\'s and the netcode proof test\'s results')
for idx, ind, what in sorted([(k[0], '', 'SPIKE-ORDER quote'), (w[0], '', 'In words'), (d[0], '  ', 'Depends on')], reverse=True):
    L[idx + 1:idx + 1] = ([''] if ind == '' else []) + pointer(ind)  # a paragraph of its own at the top level
    print(p, what, 'after line', idx + 1)
open(REPO + '/' + p, 'w', encoding='utf-8').write('\n'.join(L) + '\n')
