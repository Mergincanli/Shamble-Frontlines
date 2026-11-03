import sys, textwrap
t = ' '.join(open(sys.argv[1]).read().split())
w = int(sys.argv[2]) if len(sys.argv) > 2 else 120
# first line begins after nothing (it is a fresh line with indent)
lines = textwrap.wrap(t, width=w, initial_indent='   ', subsequent_indent='   ', break_long_words=False, break_on_hyphens=False)
for l in lines: print(f'[{len(l)}] {l}')
print(len(lines), 'lines; chars', len(t))
