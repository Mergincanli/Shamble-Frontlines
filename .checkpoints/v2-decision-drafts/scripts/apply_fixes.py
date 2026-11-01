#!/usr/bin/env python3
"""Apply an author's fix file (### FIX <id> / BEFORE: / AFTER: / END blocks) to one document, exactly.
Every BEFORE must occur exactly once in the document (checked against the text as it stands when that fix is applied);
fixes apply in order; on any failure nothing is written. Usage: apply_fixes.py <fixes.md> <doc.md> [--dry-run]"""
import re, sys, pathlib
fixes_p, doc_p = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
dry = '--dry-run' in sys.argv
src = fixes_p.read_text(encoding='utf8')
blocks = re.findall(r'^### FIX (.+?)\n(?:```[^\n]*\n)?BEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND\b', src, re.S | re.M)
doc = doc_p.read_text(encoding='utf8')
orig = doc; ok = True
for fid, before, after in blocks:
    # tolerate a fence line wrapped around BEFORE/AFTER text
    before = re.sub(r'\n```\s*$', '', before); after = re.sub(r'\n```\s*$', '', after)
    n = doc.count(before)
    if n != 1:
        print(f'FAIL {fid}: BEFORE occurs {n} times'); ok = False; continue
    doc = doc.replace(before, after, 1)
    print(f'ok   {fid}: {len(before.splitlines())} line(s) -> {len(after.splitlines())}')
print(f'{len(blocks)} fix blocks; {"all applied" if ok else "NOT applied (fix the failures first)"}')
if ok and not dry and doc != orig:
    doc_p.write_text(doc, encoding='utf8'); print('written', doc_p)
sys.exit(0 if ok else 1)
