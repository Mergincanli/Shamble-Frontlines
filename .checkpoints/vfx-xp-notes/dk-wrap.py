# Rewrap lines A..B (1-based, inclusive) of a file at 120 characters; first line keeps its indent, the rest use IND.
import sys, textwrap
p, a, b, ind = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4] if len(sys.argv) > 4 else ''
L = open(p, encoding='utf-8').read().split('\n')
first = L[a-1]; lead = first[:len(first) - len(first.lstrip())]
text = ' '.join(x.strip() for x in L[a-1:b])
out = textwrap.wrap(text, 120, initial_indent=lead, subsequent_indent=ind, break_on_hyphens=False, break_long_words=False)
L[a-1:b] = out
open(p, 'w', encoding='utf-8').write('\n'.join(L))
print('\n'.join(out))
