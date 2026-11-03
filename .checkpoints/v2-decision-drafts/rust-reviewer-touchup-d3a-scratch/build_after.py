import textwrap, sys
D = '/home/user/Shamble-Frontlines/docs/adr/0004-simulation-core.md'
lines = open(D, encoding='utf8').read().split('\n')
before = lines[261:277]            # lines 262..277 (1-based)
assert before[0].startswith('   684–692); and the') and before[-1] == '   Transcendental maths only', before[-1]
fixed = ' '.join(' '.join(before[:10]).split())   # lines 262..271, unchanged words
new = ' '.join(open('draftP.txt', encoding='utf8').read().split())
text = fixed + ' ' + new
NB = ' '
for frag in ['(3 + 2^-22) ÷ (1 + 2^-23)', 'not 2,']:
    assert text.count(frag) == 1, frag
    text = text.replace(frag, frag.replace(' ', NB))
out = textwrap.wrap(text, width=120, initial_indent='   ', subsequent_indent='   ', break_long_words=False, break_on_hyphens=False)
out = [l.replace(NB, ' ') for l in out]
for l in out: print(f'[{len(l)}] {l}')
print(len(out), 'lines (BEFORE 16)')
open('before.txt', 'w', encoding='utf8').write('\n'.join(before) + '\n')
open('after.txt', 'w', encoding='utf8').write('\n'.join(out) + '\n')
# words check: the unchanged part keeps its words in order
assert ' '.join(' '.join(out).split()).startswith(fixed)
