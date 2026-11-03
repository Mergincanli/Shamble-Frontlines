import sys, textwrap
src = open(sys.argv[1], encoding='utf8').read().splitlines()
a, b = int(sys.argv[2]), int(sys.argv[3])  # 1-based inclusive range of the paragraph tail to rewrap
insert_after = sys.argv[4]  # the sentence end after which the new sentence goes
new = open(sys.argv[5], encoding='utf8').read().strip()
text = ' '.join(l.strip() for l in src[a-1:b])
assert text.count(insert_after) == 1
text = text.replace(insert_after, insert_after + ' ' + new, 1)
out = textwrap.wrap(text, width=120, initial_indent='   ', subsequent_indent='   ', break_long_words=False,
                    break_on_hyphens=False)
print('\n'.join(out))
