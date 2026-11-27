# U2 (run ONLY after Mustafa's yes on NEW-TEXT): places the three workflow texts from the notes into .github/workflows/,
# byte for byte, and checks each sha256 before and after. Run from the repo root. --dry-run checks the sources only.
# The guard asks Mustafa before this write (a .github/ edit in a day session).
import hashlib, sys, pathlib
T = pathlib.Path('.checkpoints/step-2-6-previews/texts')
W = pathlib.Path('.github/workflows')
PLAN = [
    ('preview-proposed.yml', 'preview.yml', '31119c06e7d095baa3c99ed54bbc1a2c72d05b8b3cab88ffd4512dc12acd3abc'),
    ('preview-close.yml', 'preview-close.yml', 'c2f1fa90511db0f558d4e9287d64e189877f8cbc03eb27e6efbe08787fe20e66'),
    ('wall-test.yml', 'wall-test.yml', '3af067068060971beba8444346fdb836b7779a41403ac09d25e43027e8b18507'),
]
sha = lambda b: hashlib.sha256(b).hexdigest()
for src, dst, want in PLAN:
    b = (T / src).read_bytes()
    assert sha(b) == want, (src, sha(b))
    assert not (W / dst).exists(), f'{W / dst} exists already'
    print('source ok', src, len(b), want[:12])
if '--dry-run' in sys.argv:
    sys.exit(0)
for src, dst, want in PLAN:
    (W / dst).write_bytes((T / src).read_bytes())
for src, dst, want in PLAN:
    got = sha((W / dst).read_bytes())
    assert got == want, (dst, got)
    print('placed', W / dst, got)
