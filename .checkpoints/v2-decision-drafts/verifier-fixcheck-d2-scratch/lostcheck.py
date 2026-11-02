#!/usr/bin/env python3
"""List structural lines (headings, bold item starts, numbered steps, table rows' ids) that a FIX block's BEFORE has
and its AFTER drops, and that are absent from the doc at the ref. Usage: lostcheck.py <ref> <docs,...> <fixfile>..."""
import re, subprocess, sys
ref, docs = sys.argv[1], sys.argv[2].split(',')
text = {d: subprocess.run(['git','-C','/home/user/Shamble-Frontlines','show',f'{ref}:{d}'],capture_output=True,text=True).stdout for d in docs}
struct = re.compile(r'^\s*(#{2,4} |- \*\*|\d+\. |\| *[A-Z][A-Za-z0-9-]+ *\|)')
for ff in sys.argv[3:]:
    L = open(ff, encoding='utf-8').read().split('\n'); i = 0; title = ''
    while i < len(L):
        if L[i].startswith('### '): title = L[i][4:]
        if L[i].startswith('FILE:'):
            f = L[i][5:].strip(); i += 1; mode = None; b = []; a = []
            while i < len(L) and L[i] != 'END':
                if L[i] == 'BEFORE:': mode = 'b'
                elif L[i] == 'AFTER:': mode = 'a'
                elif mode == 'b': b.append(L[i])
                elif mode == 'a': a.append(L[i])
                i += 1
            if f in text:
                aj = ' '.join(x.strip() for x in a)
                for x in b:
                    if struct.match(x):
                        key = struct.match(x).group(0).strip()
                        # the start of the line (first 40 chars) must survive somewhere
                        head = x.strip()[:40]
                        if head not in aj and head not in text[f]:
                            print(f"{ff.split('/')[-1]} :: {title[:70]} :: {f.split('/')[-1]} :: lost: {x.strip()[:110]}")
        i += 1
