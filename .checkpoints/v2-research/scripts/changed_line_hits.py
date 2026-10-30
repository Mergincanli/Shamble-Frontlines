"""The glossary hits that sit on lines a commit range changed (for a final check of only what a fix round changed).
Usage: python3 changed_line_hits.py <base> <head> <hits.tsv made at head> <out.tsv> [R3,R5,R6]"""
import re, subprocess, sys

FILES = {'R1': 'r1-simulation-core.md', 'R2': 'r2-netcode.md', 'R3': 'r3-rendering.md',
         'R4': 'r4-physics-navigation.md', 'R5': 'r5-toolchain-quality.md', 'R6': 'r6-infrastructure.md'}
base, head, hits, out = sys.argv[1:5]
topics = sys.argv[5].split(',') if len(sys.argv) > 5 else list(FILES)
changed = {}
for t in topics:
    d = subprocess.run(['git', '-C', '/home/user/Shamble-Frontlines', 'diff', '-U0', base, head, '--',
                        'docs/research/' + FILES[t]], capture_output=True, text=True, check=True).stdout
    lines = set()
    for m in re.finditer(r'^@@ -\d+(?:,\d+)? \+(\d+)(?:,(\d+))? @@', d, re.M):
        start, n = int(m.group(1)), int(m.group(2) or 1)
        lines.update(range(start, start + n))
    changed[t] = lines
rows = [l.split('\t') for l in open(hits).read().splitlines()]
keep = [rows[0]] + [r for r in rows[1:] if r[0] in changed and int(r[1]) in changed[r[0]]]
open(out, 'w').write(''.join('\t'.join(r) + '\n' for r in keep))
print('changed lines:', {t: len(v) for t, v in changed.items()}, '; hits on them:', len(keep) - 1)
