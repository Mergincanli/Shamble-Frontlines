# read-only helper: extract a source text span and print it rewrapped (no file edits)
import sys, textwrap
src, a, b, first, rest, width = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4], sys.argv[5], int(sys.argv[6])
lines = open(src, encoding='utf-8').read().split('\n')[a-1:b]
text = ' '.join(l.strip() for l in lines)
for pre in sys.argv[7:]:
    if text.startswith(pre): text = text[len(pre):]
print('\n'.join(textwrap.wrap(text, width=width, initial_indent=first, subsequent_indent=rest, break_long_words=False, break_on_hyphens=False)))
