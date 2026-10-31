#!/usr/bin/env python3
"""Assemble one decision draft: copy the author's notes draft to docs/adr/ verbatim, and (optionally) swap the
placeholder under '### Pass marks' for test-engineer's subsection, verbatim. Verifies the result reconstructs exactly.
Usage: assemble.py <draft_notes.md> <out docs/adr/000N-x.md> [<test-engineer-passmarks-000N.md>]"""
import sys, re, pathlib
src = pathlib.Path(sys.argv[1]).read_text(encoding='utf8')
out = pathlib.Path(sys.argv[2])
PH = re.compile(r'(^### Pass marks[ \t]*\n)(?:[ \t]*\n)*Written by test-engineer \(plan step 4\)\.[ \t]*\n', re.M)
if len(sys.argv) > 3:
    pm = pathlib.Path(sys.argv[3]).read_text(encoding='utf8')
    m = re.search(r'^### Pass marks[ \t]*\n.*?(?=^### Spike entries|\Z)', pm, re.S | re.M)
    if not m: sys.exit('no "### Pass marks" part in the pass-marks file')
    block = m.group(0).rstrip('\n') + '\n\n'
    hits = PH.findall(src)
    if len(hits) != 1: sys.exit(f'placeholder found {len(hits)} times in the draft (need exactly 1)')
    res = PH.sub(lambda _m: block, src, count=1)
    # reconstruction check: removing the inserted block and putting the placeholder back gives the source again
    i = res.index(block)
    back = res[:i] + PH.search(src).group(0) + res[i+len(block):]
    if back != src: sys.exit('reconstruction failed')
else:
    res = src
out.write_text(res, encoding='utf8')
print(f'wrote {out} ({len(res.splitlines())} lines); source {sys.argv[1]} ({len(src.splitlines())} lines)'
      + (f'; pass marks inserted ({len(block.splitlines())} lines)' if len(sys.argv) > 3 else '; verbatim copy'))
