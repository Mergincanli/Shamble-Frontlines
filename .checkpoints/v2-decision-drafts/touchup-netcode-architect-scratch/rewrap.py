#!/usr/bin/env python3
"""rewrap.py DOC START END OLD NEW [WIDTH]
Takes whole lines START..END (1-based, inclusive) of DOC, replaces OLD (must occur once in the joined text) by NEW,
greedy-rewraps with the first line's indent kept and continuation indent = indent of line START+1 (or START),
and prints JSON: before, after, line counts, max width, and whether BEFORE occurs once in DOC."""
import sys, json, re
doc, s, e, old, new = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4], sys.argv[5]
width = int(sys.argv[6]) if len(sys.argv) > 6 else 120
text = open(doc, encoding='utf-8').read()
lines = text.split('\n')
block = lines[s-1:e]
before = '\n'.join(block)
first_indent = re.match(r'\s*', block[0]).group(0)
cont_indent = re.match(r'\s*', block[1] if len(block) > 1 else block[0]).group(0)
# keep the first line's leading marker (e.g. "- ") as part of the first word sequence
joined = ' '.join(l.strip() for l in block)
assert joined.count(old) == 1, ('OLD count', joined.count(old))
joined = joined.replace(old, new)
words = joined.split(' ')
out, cur = [], first_indent
for w in words:
    cand = (cur + ' ' + w) if cur.strip() else (cur + w)
    if len(cand) > width and cur.strip():
        out.append(cur); cur = cont_indent + w
    else:
        cur = cand
out.append(cur)
after = '\n'.join(out)
print(json.dumps({'before_lines': len(block), 'after_lines': len(out), 'max_width': max(len(l) for l in out),
                  'before_once': text.count(before) == 1}, ensure_ascii=False))
open(sys.argv[0].rsplit('/',1)[0] + '/last_before.txt', 'w', encoding='utf-8').write(before)
open(sys.argv[0].rsplit('/',1)[0] + '/last_after.txt', 'w', encoding='utf-8').write(after)
