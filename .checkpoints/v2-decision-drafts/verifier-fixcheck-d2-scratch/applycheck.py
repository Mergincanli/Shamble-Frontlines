#!/usr/bin/env python3
"""For every FIX block (FILE/BEFORE/AFTER/END) in the given fix files that targets one of the given docs,
check at a git ref: AFTER present (whitespace-normalized), BEFORE absent unless contained in AFTER.
Usage: applycheck.py <ref> <doc1,doc2,...> <fixfile>..."""
import re, subprocess, sys
ref, docs = sys.argv[1], sys.argv[2].split(',')
norm = lambda s: re.sub(r'\s+', ' ', s).strip()
text = {d: norm(subprocess.run(['git','-C','/home/user/Shamble-Frontlines','show',f'{ref}:{d}'],capture_output=True,text=True).stdout) for d in docs}
for ff in sys.argv[3:]:
    lines = open(ff, encoding='utf-8').read().split('\n')
    i = 0; title = None
    while i < len(lines):
        l = lines[i]
        if l.startswith('### '): title = l[4:]
        if l.startswith('FILE:'):
            f = l[5:].strip(); i += 1; mode = None; b = []; a = []
            while i < len(lines) and lines[i] != 'END':
                if lines[i] == 'BEFORE:': mode = 'b'
                elif lines[i] == 'AFTER:': mode = 'a'
                elif mode == 'b': b.append(lines[i])
                elif mode == 'a': a.append(lines[i])
                i += 1
            if f in text:
                B, A, T = norm('\n'.join(b)), norm('\n'.join(a)), text[f]
                ok_a = A in T
                ok_b = (B not in T) or (B in A)
                if not (ok_a and ok_b):
                    print(f"{ff.split('/')[-1]} :: {title[:90]} :: {f.split('/')[-1]} :: AFTER present={ok_a} BEFORE gone={ok_b}")
        i += 1
