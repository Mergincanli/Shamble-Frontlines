# Copies R7's builder (a715be2) with ONLY its input and output paths changed. Usage (from the repo root):
#   python3 -I .checkpoints/foundation/patch-builder.py <orig.py> <out.py> <cabana.json> <tri.json> <outdir>
import sys
src, dst, cab, tri, outdir = sys.argv[1:6]
s = open(src, encoding='utf-8').read()
subs = [
    ("'.checkpoints/r7-legacy-inventory/tuning/cabana-1.json'", repr(cab)),
    ("'.checkpoints/r7-legacy-inventory/tuning/tri-1.json'", repr(tri)),
    ("open('docs/legacy-tuning.json', 'w')", f"open({outdir + '/legacy-tuning.json'!r}, 'w')"),
    ("open('docs/legacy-tuning.json', 'a')", f"open({outdir + '/legacy-tuning.json'!r}, 'a')"),
    ("open('docs/LEGACY_TUNING.md', 'w')", f"open({outdir + '/LEGACY_TUNING.md'!r}, 'w')"),
]
for old, new in subs:
    n = s.count(old)
    if n != 1:
        raise SystemExit(f'expected exactly one {old!r}, found {n}')
    s = s.replace(old, new)
open(dst, 'w', encoding='utf-8').write(s)
print('patched 5 sites ->', dst)
