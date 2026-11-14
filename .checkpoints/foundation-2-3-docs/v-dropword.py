# Verifier: in a scratch copy only, drop one word (by position: the second-to-last token of the last line) of the first
# 2026-11-13 pointer block in SPIKE_PLAN. No literal text of anyone's words is used.
#   python3 -I .checkpoints/foundation-2-3-docs/v-dropword.py <copy-root>
import sys
p = sys.argv[1] + '/docs/SPIKE_PLAN.md'
L = open(p, encoding='utf-8').read().split('\n')
s = next(i for i, l in enumerate(L) if l.strip().startswith('_Pointer (2026-11-13'))
e = next(i for i in range(s, s + 9) if L[i].rstrip().endswith('"_'))
t = L[e].split(' ')
before = len(t)
del t[-2]
L[e] = ' '.join(t)
open(p, 'w', encoding='utf-8').write('\n'.join(L))
print('dropped one word on copy line', e + 1, 'tokens', before, '->', len(t))
